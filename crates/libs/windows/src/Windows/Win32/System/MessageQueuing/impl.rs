#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQApplication_Impl: Sized + super::Com::IDispatch_Impl {
    fn MachineIdOfMachineName(&self, machinename: &super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQApplication_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQApplication_Impl, const OFFSET: isize>() -> IMSMQApplication_Vtbl {
        unsafe extern "system" fn MachineIdOfMachineName<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, machinename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrguid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MachineIdOfMachineName(::core::mem::transmute(&machinename)) {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrguid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(), MachineIdOfMachineName: MachineIdOfMachineName::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQApplication as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQApplication2_Impl: Sized + super::Com::IDispatch_Impl + IMSMQApplication_Impl {
    fn RegisterCertificate(&self, flags: *const super::Com::VARIANT, externalcertificate: *const super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn MachineNameOfMachineId(&self, bstrguid: &super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn MSMQVersionMajor(&self) -> ::windows::core::Result<i16>;
    fn MSMQVersionMinor(&self) -> ::windows::core::Result<i16>;
    fn MSMQVersionBuild(&self) -> ::windows::core::Result<i16>;
    fn IsDsEnabled(&self) -> ::windows::core::Result<i16>;
    fn Properties(&self) -> ::windows::core::Result<super::Com::IDispatch>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQApplication2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQApplication2_Impl, const OFFSET: isize>() -> IMSMQApplication2_Vtbl {
        unsafe extern "system" fn RegisterCertificate<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQApplication2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: *const super::Com::VARIANT, externalcertificate: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RegisterCertificate(::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&externalcertificate)).into()
        }
        unsafe extern "system" fn MachineNameOfMachineId<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQApplication2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrguid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrmachinename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MachineNameOfMachineId(::core::mem::transmute(&bstrguid)) {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrmachinename = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MSMQVersionMajor<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQApplication2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psmsmqversionmajor: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MSMQVersionMajor() {
                ::core::result::Result::Ok(ok__) => {
                    *psmsmqversionmajor = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MSMQVersionMinor<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQApplication2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psmsmqversionminor: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MSMQVersionMinor() {
                ::core::result::Result::Ok(ok__) => {
                    *psmsmqversionminor = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MSMQVersionBuild<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQApplication2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psmsmqversionbuild: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MSMQVersionBuild() {
                ::core::result::Result::Ok(ok__) => {
                    *psmsmqversionbuild = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsDsEnabled<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQApplication2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfisdsenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsDsEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *pfisdsenabled = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQApplication2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcolproperties = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IMSMQApplication_Vtbl::new::<Identity, Impl, OFFSET>(),
            RegisterCertificate: RegisterCertificate::<Identity, Impl, OFFSET>,
            MachineNameOfMachineId: MachineNameOfMachineId::<Identity, Impl, OFFSET>,
            MSMQVersionMajor: MSMQVersionMajor::<Identity, Impl, OFFSET>,
            MSMQVersionMinor: MSMQVersionMinor::<Identity, Impl, OFFSET>,
            MSMQVersionBuild: MSMQVersionBuild::<Identity, Impl, OFFSET>,
            IsDsEnabled: IsDsEnabled::<Identity, Impl, OFFSET>,
            Properties: Properties::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQApplication2 as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IMSMQApplication as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQApplication3_Impl: Sized + super::Com::IDispatch_Impl + IMSMQApplication_Impl + IMSMQApplication2_Impl {
    fn ActiveQueues(&self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn PrivateQueues(&self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn DirectoryServiceServer(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn IsConnected(&self) -> ::windows::core::Result<i16>;
    fn BytesInAllQueues(&self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SetMachine(&self, bstrmachine: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Machine(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Connect(&self) -> ::windows::core::Result<()>;
    fn Disconnect(&self) -> ::windows::core::Result<()>;
    fn Tidy(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQApplication3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQApplication3_Impl, const OFFSET: isize>() -> IMSMQApplication3_Vtbl {
        unsafe extern "system" fn ActiveQueues<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQApplication3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvactivequeues: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ActiveQueues() {
                ::core::result::Result::Ok(ok__) => {
                    *pvactivequeues = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrivateQueues<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQApplication3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvprivatequeues: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PrivateQueues() {
                ::core::result::Result::Ok(ok__) => {
                    *pvprivatequeues = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DirectoryServiceServer<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQApplication3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdirectoryserviceserver: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DirectoryServiceServer() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrdirectoryserviceserver = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsConnected<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQApplication3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfisconnected: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsConnected() {
                ::core::result::Result::Ok(ok__) => {
                    *pfisconnected = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BytesInAllQueues<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQApplication3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvbytesinallqueues: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).BytesInAllQueues() {
                ::core::result::Result::Ok(ok__) => {
                    *pvbytesinallqueues = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMachine<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQApplication3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrmachine: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetMachine(::core::mem::transmute(&bstrmachine)).into()
        }
        unsafe extern "system" fn Machine<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQApplication3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrmachine: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Machine() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrmachine = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Connect<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQApplication3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Connect().into()
        }
        unsafe extern "system" fn Disconnect<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQApplication3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Disconnect().into()
        }
        unsafe extern "system" fn Tidy<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQApplication3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Tidy().into()
        }
        Self {
            base: IMSMQApplication2_Vtbl::new::<Identity, Impl, OFFSET>(),
            ActiveQueues: ActiveQueues::<Identity, Impl, OFFSET>,
            PrivateQueues: PrivateQueues::<Identity, Impl, OFFSET>,
            DirectoryServiceServer: DirectoryServiceServer::<Identity, Impl, OFFSET>,
            IsConnected: IsConnected::<Identity, Impl, OFFSET>,
            BytesInAllQueues: BytesInAllQueues::<Identity, Impl, OFFSET>,
            SetMachine: SetMachine::<Identity, Impl, OFFSET>,
            Machine: Machine::<Identity, Impl, OFFSET>,
            Connect: Connect::<Identity, Impl, OFFSET>,
            Disconnect: Disconnect::<Identity, Impl, OFFSET>,
            Tidy: Tidy::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQApplication3 as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IMSMQApplication as ::windows::core::Interface>::IID || iid == &<IMSMQApplication2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQCollection_Impl: Sized + super::Com::IDispatch_Impl {
    fn Item(&self, index: *const super::Com::VARIANT) -> ::windows::core::Result<super::Com::VARIANT>;
    fn Count(&self) -> ::windows::core::Result<i32>;
    fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQCollection_Impl, const OFFSET: isize>() -> IMSMQCollection_Vtbl {
        unsafe extern "system" fn Item<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: *const super::Com::VARIANT, pvarret: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvarret = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *pcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *ppunk = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Item: Item::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQCollection as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQCoordinatedTransactionDispenser_Impl: Sized + super::Com::IDispatch_Impl {
    fn BeginTransaction(&self) -> ::windows::core::Result<IMSMQTransaction>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQCoordinatedTransactionDispenser_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQCoordinatedTransactionDispenser_Impl, const OFFSET: isize>() -> IMSMQCoordinatedTransactionDispenser_Vtbl {
        unsafe extern "system" fn BeginTransaction<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQCoordinatedTransactionDispenser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptransaction: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).BeginTransaction() {
                ::core::result::Result::Ok(ok__) => {
                    *ptransaction = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(), BeginTransaction: BeginTransaction::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQCoordinatedTransactionDispenser as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQCoordinatedTransactionDispenser2_Impl: Sized + super::Com::IDispatch_Impl {
    fn BeginTransaction(&self) -> ::windows::core::Result<IMSMQTransaction2>;
    fn Properties(&self) -> ::windows::core::Result<super::Com::IDispatch>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQCoordinatedTransactionDispenser2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQCoordinatedTransactionDispenser2_Impl, const OFFSET: isize>() -> IMSMQCoordinatedTransactionDispenser2_Vtbl {
        unsafe extern "system" fn BeginTransaction<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQCoordinatedTransactionDispenser2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptransaction: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).BeginTransaction() {
                ::core::result::Result::Ok(ok__) => {
                    *ptransaction = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQCoordinatedTransactionDispenser2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcolproperties = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            BeginTransaction: BeginTransaction::<Identity, Impl, OFFSET>,
            Properties: Properties::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQCoordinatedTransactionDispenser2 as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQCoordinatedTransactionDispenser3_Impl: Sized + super::Com::IDispatch_Impl {
    fn BeginTransaction(&self) -> ::windows::core::Result<IMSMQTransaction3>;
    fn Properties(&self) -> ::windows::core::Result<super::Com::IDispatch>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQCoordinatedTransactionDispenser3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQCoordinatedTransactionDispenser3_Impl, const OFFSET: isize>() -> IMSMQCoordinatedTransactionDispenser3_Vtbl {
        unsafe extern "system" fn BeginTransaction<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQCoordinatedTransactionDispenser3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptransaction: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).BeginTransaction() {
                ::core::result::Result::Ok(ok__) => {
                    *ptransaction = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQCoordinatedTransactionDispenser3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcolproperties = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            BeginTransaction: BeginTransaction::<Identity, Impl, OFFSET>,
            Properties: Properties::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQCoordinatedTransactionDispenser3 as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQDestination_Impl: Sized + super::Com::IDispatch_Impl {
    fn Open(&self) -> ::windows::core::Result<()>;
    fn Close(&self) -> ::windows::core::Result<()>;
    fn IsOpen(&self) -> ::windows::core::Result<i16>;
    fn IADs(&self) -> ::windows::core::Result<super::Com::IDispatch>;
    fn putref_IADs(&self, piads: &::core::option::Option<super::Com::IDispatch>) -> ::windows::core::Result<()>;
    fn ADsPath(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetADsPath(&self, bstradspath: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn PathName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetPathName(&self, bstrpathname: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn FormatName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetFormatName(&self, bstrformatname: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Destinations(&self) -> ::windows::core::Result<super::Com::IDispatch>;
    fn putref_Destinations(&self, pdestinations: &::core::option::Option<super::Com::IDispatch>) -> ::windows::core::Result<()>;
    fn Properties(&self) -> ::windows::core::Result<super::Com::IDispatch>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQDestination_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQDestination_Impl, const OFFSET: isize>() -> IMSMQDestination_Vtbl {
        unsafe extern "system" fn Open<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQDestination_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Open().into()
        }
        unsafe extern "system" fn Close<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQDestination_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Close().into()
        }
        unsafe extern "system" fn IsOpen<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQDestination_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfisopen: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsOpen() {
                ::core::result::Result::Ok(ok__) => {
                    *pfisopen = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IADs<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQDestination_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppiads: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IADs() {
                ::core::result::Result::Ok(ok__) => {
                    *ppiads = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_IADs<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQDestination_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piads: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).putref_IADs(::core::mem::transmute(&piads)).into()
        }
        unsafe extern "system" fn ADsPath<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQDestination_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstradspath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ADsPath() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstradspath = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetADsPath<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQDestination_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstradspath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetADsPath(::core::mem::transmute(&bstradspath)).into()
        }
        unsafe extern "system" fn PathName<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQDestination_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrpathname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PathName() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrpathname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPathName<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQDestination_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpathname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPathName(::core::mem::transmute(&bstrpathname)).into()
        }
        unsafe extern "system" fn FormatName<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQDestination_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrformatname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).FormatName() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrformatname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFormatName<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQDestination_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrformatname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetFormatName(::core::mem::transmute(&bstrformatname)).into()
        }
        unsafe extern "system" fn Destinations<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQDestination_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdestinations: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Destinations() {
                ::core::result::Result::Ok(ok__) => {
                    *ppdestinations = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_Destinations<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQDestination_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdestinations: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).putref_Destinations(::core::mem::transmute(&pdestinations)).into()
        }
        unsafe extern "system" fn Properties<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQDestination_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcolproperties = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Open: Open::<Identity, Impl, OFFSET>,
            Close: Close::<Identity, Impl, OFFSET>,
            IsOpen: IsOpen::<Identity, Impl, OFFSET>,
            IADs: IADs::<Identity, Impl, OFFSET>,
            putref_IADs: putref_IADs::<Identity, Impl, OFFSET>,
            ADsPath: ADsPath::<Identity, Impl, OFFSET>,
            SetADsPath: SetADsPath::<Identity, Impl, OFFSET>,
            PathName: PathName::<Identity, Impl, OFFSET>,
            SetPathName: SetPathName::<Identity, Impl, OFFSET>,
            FormatName: FormatName::<Identity, Impl, OFFSET>,
            SetFormatName: SetFormatName::<Identity, Impl, OFFSET>,
            Destinations: Destinations::<Identity, Impl, OFFSET>,
            putref_Destinations: putref_Destinations::<Identity, Impl, OFFSET>,
            Properties: Properties::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQDestination as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQEvent_Impl: Sized + super::Com::IDispatch_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQEvent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQEvent_Impl, const OFFSET: isize>() -> IMSMQEvent_Vtbl {
        Self { base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQEvent as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQEvent2_Impl: Sized + super::Com::IDispatch_Impl + IMSMQEvent_Impl {
    fn Properties(&self) -> ::windows::core::Result<super::Com::IDispatch>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQEvent2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQEvent2_Impl, const OFFSET: isize>() -> IMSMQEvent2_Vtbl {
        unsafe extern "system" fn Properties<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQEvent2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcolproperties = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: IMSMQEvent_Vtbl::new::<Identity, Impl, OFFSET>(), Properties: Properties::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQEvent2 as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IMSMQEvent as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQEvent3_Impl: Sized + super::Com::IDispatch_Impl + IMSMQEvent_Impl + IMSMQEvent2_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQEvent3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQEvent3_Impl, const OFFSET: isize>() -> IMSMQEvent3_Vtbl {
        Self { base: IMSMQEvent2_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQEvent3 as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IMSMQEvent as ::windows::core::Interface>::IID || iid == &<IMSMQEvent2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQManagement_Impl: Sized + super::Com::IDispatch_Impl {
    fn Init(&self, machine: *const super::Com::VARIANT, pathname: *const super::Com::VARIANT, formatname: *const super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn FormatName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Machine(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn MessageCount(&self) -> ::windows::core::Result<i32>;
    fn ForeignStatus(&self) -> ::windows::core::Result<i32>;
    fn QueueType(&self) -> ::windows::core::Result<i32>;
    fn IsLocal(&self) -> ::windows::core::Result<i16>;
    fn TransactionalStatus(&self) -> ::windows::core::Result<i32>;
    fn BytesInQueue(&self) -> ::windows::core::Result<super::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQManagement_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQManagement_Impl, const OFFSET: isize>() -> IMSMQManagement_Vtbl {
        unsafe extern "system" fn Init<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQManagement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, machine: *const super::Com::VARIANT, pathname: *const super::Com::VARIANT, formatname: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Init(::core::mem::transmute_copy(&machine), ::core::mem::transmute_copy(&pathname), ::core::mem::transmute_copy(&formatname)).into()
        }
        unsafe extern "system" fn FormatName<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQManagement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrformatname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).FormatName() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrformatname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Machine<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQManagement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrmachine: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Machine() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrmachine = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MessageCount<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQManagement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmessagecount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MessageCount() {
                ::core::result::Result::Ok(ok__) => {
                    *plmessagecount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ForeignStatus<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQManagement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plforeignstatus: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ForeignStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *plforeignstatus = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueueType<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQManagement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plqueuetype: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).QueueType() {
                ::core::result::Result::Ok(ok__) => {
                    *plqueuetype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsLocal<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQManagement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfislocal: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsLocal() {
                ::core::result::Result::Ok(ok__) => {
                    *pfislocal = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransactionalStatus<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQManagement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pltransactionalstatus: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).TransactionalStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *pltransactionalstatus = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BytesInQueue<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQManagement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvbytesinqueue: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).BytesInQueue() {
                ::core::result::Result::Ok(ok__) => {
                    *pvbytesinqueue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Init: Init::<Identity, Impl, OFFSET>,
            FormatName: FormatName::<Identity, Impl, OFFSET>,
            Machine: Machine::<Identity, Impl, OFFSET>,
            MessageCount: MessageCount::<Identity, Impl, OFFSET>,
            ForeignStatus: ForeignStatus::<Identity, Impl, OFFSET>,
            QueueType: QueueType::<Identity, Impl, OFFSET>,
            IsLocal: IsLocal::<Identity, Impl, OFFSET>,
            TransactionalStatus: TransactionalStatus::<Identity, Impl, OFFSET>,
            BytesInQueue: BytesInQueue::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQManagement as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQMessage_Impl: Sized + super::Com::IDispatch_Impl {
    fn Class(&self) -> ::windows::core::Result<i32>;
    fn PrivLevel(&self) -> ::windows::core::Result<i32>;
    fn SetPrivLevel(&self, lprivlevel: i32) -> ::windows::core::Result<()>;
    fn AuthLevel(&self) -> ::windows::core::Result<i32>;
    fn SetAuthLevel(&self, lauthlevel: i32) -> ::windows::core::Result<()>;
    fn IsAuthenticated(&self) -> ::windows::core::Result<i16>;
    fn Delivery(&self) -> ::windows::core::Result<i32>;
    fn SetDelivery(&self, ldelivery: i32) -> ::windows::core::Result<()>;
    fn Trace(&self) -> ::windows::core::Result<i32>;
    fn SetTrace(&self, ltrace: i32) -> ::windows::core::Result<()>;
    fn Priority(&self) -> ::windows::core::Result<i32>;
    fn SetPriority(&self, lpriority: i32) -> ::windows::core::Result<()>;
    fn Journal(&self) -> ::windows::core::Result<i32>;
    fn SetJournal(&self, ljournal: i32) -> ::windows::core::Result<()>;
    fn ResponseQueueInfo(&self) -> ::windows::core::Result<IMSMQQueueInfo>;
    fn putref_ResponseQueueInfo(&self, pqinforesponse: &::core::option::Option<IMSMQQueueInfo>) -> ::windows::core::Result<()>;
    fn AppSpecific(&self) -> ::windows::core::Result<i32>;
    fn SetAppSpecific(&self, lappspecific: i32) -> ::windows::core::Result<()>;
    fn SourceMachineGuid(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn BodyLength(&self) -> ::windows::core::Result<i32>;
    fn Body(&self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SetBody(&self, varbody: &super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn AdminQueueInfo(&self) -> ::windows::core::Result<IMSMQQueueInfo>;
    fn putref_AdminQueueInfo(&self, pqinfoadmin: &::core::option::Option<IMSMQQueueInfo>) -> ::windows::core::Result<()>;
    fn Id(&self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn CorrelationId(&self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SetCorrelationId(&self, varmsgid: &super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Ack(&self) -> ::windows::core::Result<i32>;
    fn SetAck(&self, lack: i32) -> ::windows::core::Result<()>;
    fn Label(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetLabel(&self, bstrlabel: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn MaxTimeToReachQueue(&self) -> ::windows::core::Result<i32>;
    fn SetMaxTimeToReachQueue(&self, lmaxtimetoreachqueue: i32) -> ::windows::core::Result<()>;
    fn MaxTimeToReceive(&self) -> ::windows::core::Result<i32>;
    fn SetMaxTimeToReceive(&self, lmaxtimetoreceive: i32) -> ::windows::core::Result<()>;
    fn HashAlgorithm(&self) -> ::windows::core::Result<i32>;
    fn SetHashAlgorithm(&self, lhashalg: i32) -> ::windows::core::Result<()>;
    fn EncryptAlgorithm(&self) -> ::windows::core::Result<i32>;
    fn SetEncryptAlgorithm(&self, lencryptalg: i32) -> ::windows::core::Result<()>;
    fn SentTime(&self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn ArrivedTime(&self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn DestinationQueueInfo(&self) -> ::windows::core::Result<IMSMQQueueInfo>;
    fn SenderCertificate(&self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SetSenderCertificate(&self, varsendercert: &super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn SenderId(&self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SenderIdType(&self) -> ::windows::core::Result<i32>;
    fn SetSenderIdType(&self, lsenderidtype: i32) -> ::windows::core::Result<()>;
    fn Send(&self, destinationqueue: &::core::option::Option<IMSMQQueue>, transaction: *const super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn AttachCurrentSecurityContext(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQMessage_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage_Impl, const OFFSET: isize>() -> IMSMQMessage_Vtbl {
        unsafe extern "system" fn Class<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plclass: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Class() {
                ::core::result::Result::Ok(ok__) => {
                    *plclass = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrivLevel<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plprivlevel: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PrivLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *plprivlevel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrivLevel<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lprivlevel: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPrivLevel(::core::mem::transmute_copy(&lprivlevel)).into()
        }
        unsafe extern "system" fn AuthLevel<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plauthlevel: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AuthLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *plauthlevel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAuthLevel<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lauthlevel: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAuthLevel(::core::mem::transmute_copy(&lauthlevel)).into()
        }
        unsafe extern "system" fn IsAuthenticated<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisauthenticated: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsAuthenticated() {
                ::core::result::Result::Ok(ok__) => {
                    *pisauthenticated = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delivery<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pldelivery: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Delivery() {
                ::core::result::Result::Ok(ok__) => {
                    *pldelivery = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDelivery<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ldelivery: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDelivery(::core::mem::transmute_copy(&ldelivery)).into()
        }
        unsafe extern "system" fn Trace<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pltrace: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Trace() {
                ::core::result::Result::Ok(ok__) => {
                    *pltrace = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTrace<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ltrace: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetTrace(::core::mem::transmute_copy(&ltrace)).into()
        }
        unsafe extern "system" fn Priority<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plpriority: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Priority() {
                ::core::result::Result::Ok(ok__) => {
                    *plpriority = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPriority<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpriority: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPriority(::core::mem::transmute_copy(&lpriority)).into()
        }
        unsafe extern "system" fn Journal<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pljournal: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Journal() {
                ::core::result::Result::Ok(ok__) => {
                    *pljournal = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetJournal<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ljournal: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetJournal(::core::mem::transmute_copy(&ljournal)).into()
        }
        unsafe extern "system" fn ResponseQueueInfo<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinforesponse: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ResponseQueueInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *ppqinforesponse = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_ResponseQueueInfo<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqinforesponse: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).putref_ResponseQueueInfo(::core::mem::transmute(&pqinforesponse)).into()
        }
        unsafe extern "system" fn AppSpecific<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plappspecific: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AppSpecific() {
                ::core::result::Result::Ok(ok__) => {
                    *plappspecific = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAppSpecific<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lappspecific: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAppSpecific(::core::mem::transmute_copy(&lappspecific)).into()
        }
        unsafe extern "system" fn SourceMachineGuid<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrguidsrcmachine: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SourceMachineGuid() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrguidsrcmachine = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BodyLength<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcbbody: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).BodyLength() {
                ::core::result::Result::Ok(ok__) => {
                    *pcbbody = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Body<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarbody: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Body() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarbody = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBody<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varbody: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetBody(::core::mem::transmute(&varbody)).into()
        }
        unsafe extern "system" fn AdminQueueInfo<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinfoadmin: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AdminQueueInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *ppqinfoadmin = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_AdminQueueInfo<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqinfoadmin: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).putref_AdminQueueInfo(::core::mem::transmute(&pqinfoadmin)).into()
        }
        unsafe extern "system" fn Id<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarmsgid: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarmsgid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CorrelationId<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarmsgid: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CorrelationId() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarmsgid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCorrelationId<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varmsgid: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCorrelationId(::core::mem::transmute(&varmsgid)).into()
        }
        unsafe extern "system" fn Ack<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plack: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Ack() {
                ::core::result::Result::Ok(ok__) => {
                    *plack = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAck<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lack: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAck(::core::mem::transmute_copy(&lack)).into()
        }
        unsafe extern "system" fn Label<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrlabel: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Label() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrlabel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLabel<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrlabel: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetLabel(::core::mem::transmute(&bstrlabel)).into()
        }
        unsafe extern "system" fn MaxTimeToReachQueue<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmaxtimetoreachqueue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MaxTimeToReachQueue() {
                ::core::result::Result::Ok(ok__) => {
                    *plmaxtimetoreachqueue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxTimeToReachQueue<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmaxtimetoreachqueue: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetMaxTimeToReachQueue(::core::mem::transmute_copy(&lmaxtimetoreachqueue)).into()
        }
        unsafe extern "system" fn MaxTimeToReceive<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmaxtimetoreceive: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MaxTimeToReceive() {
                ::core::result::Result::Ok(ok__) => {
                    *plmaxtimetoreceive = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxTimeToReceive<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmaxtimetoreceive: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetMaxTimeToReceive(::core::mem::transmute_copy(&lmaxtimetoreceive)).into()
        }
        unsafe extern "system" fn HashAlgorithm<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plhashalg: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).HashAlgorithm() {
                ::core::result::Result::Ok(ok__) => {
                    *plhashalg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHashAlgorithm<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lhashalg: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetHashAlgorithm(::core::mem::transmute_copy(&lhashalg)).into()
        }
        unsafe extern "system" fn EncryptAlgorithm<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plencryptalg: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EncryptAlgorithm() {
                ::core::result::Result::Ok(ok__) => {
                    *plencryptalg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEncryptAlgorithm<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lencryptalg: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetEncryptAlgorithm(::core::mem::transmute_copy(&lencryptalg)).into()
        }
        unsafe extern "system" fn SentTime<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarsenttime: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SentTime() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarsenttime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ArrivedTime<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plarrivedtime: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ArrivedTime() {
                ::core::result::Result::Ok(ok__) => {
                    *plarrivedtime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DestinationQueueInfo<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinfodest: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DestinationQueueInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *ppqinfodest = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SenderCertificate<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarsendercert: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SenderCertificate() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarsendercert = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSenderCertificate<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varsendercert: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSenderCertificate(::core::mem::transmute(&varsendercert)).into()
        }
        unsafe extern "system" fn SenderId<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarsenderid: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SenderId() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarsenderid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SenderIdType<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plsenderidtype: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SenderIdType() {
                ::core::result::Result::Ok(ok__) => {
                    *plsenderidtype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSenderIdType<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lsenderidtype: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSenderIdType(::core::mem::transmute_copy(&lsenderidtype)).into()
        }
        unsafe extern "system" fn Send<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, destinationqueue: ::windows::core::RawPtr, transaction: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Send(::core::mem::transmute(&destinationqueue), ::core::mem::transmute_copy(&transaction)).into()
        }
        unsafe extern "system" fn AttachCurrentSecurityContext<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AttachCurrentSecurityContext().into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Class: Class::<Identity, Impl, OFFSET>,
            PrivLevel: PrivLevel::<Identity, Impl, OFFSET>,
            SetPrivLevel: SetPrivLevel::<Identity, Impl, OFFSET>,
            AuthLevel: AuthLevel::<Identity, Impl, OFFSET>,
            SetAuthLevel: SetAuthLevel::<Identity, Impl, OFFSET>,
            IsAuthenticated: IsAuthenticated::<Identity, Impl, OFFSET>,
            Delivery: Delivery::<Identity, Impl, OFFSET>,
            SetDelivery: SetDelivery::<Identity, Impl, OFFSET>,
            Trace: Trace::<Identity, Impl, OFFSET>,
            SetTrace: SetTrace::<Identity, Impl, OFFSET>,
            Priority: Priority::<Identity, Impl, OFFSET>,
            SetPriority: SetPriority::<Identity, Impl, OFFSET>,
            Journal: Journal::<Identity, Impl, OFFSET>,
            SetJournal: SetJournal::<Identity, Impl, OFFSET>,
            ResponseQueueInfo: ResponseQueueInfo::<Identity, Impl, OFFSET>,
            putref_ResponseQueueInfo: putref_ResponseQueueInfo::<Identity, Impl, OFFSET>,
            AppSpecific: AppSpecific::<Identity, Impl, OFFSET>,
            SetAppSpecific: SetAppSpecific::<Identity, Impl, OFFSET>,
            SourceMachineGuid: SourceMachineGuid::<Identity, Impl, OFFSET>,
            BodyLength: BodyLength::<Identity, Impl, OFFSET>,
            Body: Body::<Identity, Impl, OFFSET>,
            SetBody: SetBody::<Identity, Impl, OFFSET>,
            AdminQueueInfo: AdminQueueInfo::<Identity, Impl, OFFSET>,
            putref_AdminQueueInfo: putref_AdminQueueInfo::<Identity, Impl, OFFSET>,
            Id: Id::<Identity, Impl, OFFSET>,
            CorrelationId: CorrelationId::<Identity, Impl, OFFSET>,
            SetCorrelationId: SetCorrelationId::<Identity, Impl, OFFSET>,
            Ack: Ack::<Identity, Impl, OFFSET>,
            SetAck: SetAck::<Identity, Impl, OFFSET>,
            Label: Label::<Identity, Impl, OFFSET>,
            SetLabel: SetLabel::<Identity, Impl, OFFSET>,
            MaxTimeToReachQueue: MaxTimeToReachQueue::<Identity, Impl, OFFSET>,
            SetMaxTimeToReachQueue: SetMaxTimeToReachQueue::<Identity, Impl, OFFSET>,
            MaxTimeToReceive: MaxTimeToReceive::<Identity, Impl, OFFSET>,
            SetMaxTimeToReceive: SetMaxTimeToReceive::<Identity, Impl, OFFSET>,
            HashAlgorithm: HashAlgorithm::<Identity, Impl, OFFSET>,
            SetHashAlgorithm: SetHashAlgorithm::<Identity, Impl, OFFSET>,
            EncryptAlgorithm: EncryptAlgorithm::<Identity, Impl, OFFSET>,
            SetEncryptAlgorithm: SetEncryptAlgorithm::<Identity, Impl, OFFSET>,
            SentTime: SentTime::<Identity, Impl, OFFSET>,
            ArrivedTime: ArrivedTime::<Identity, Impl, OFFSET>,
            DestinationQueueInfo: DestinationQueueInfo::<Identity, Impl, OFFSET>,
            SenderCertificate: SenderCertificate::<Identity, Impl, OFFSET>,
            SetSenderCertificate: SetSenderCertificate::<Identity, Impl, OFFSET>,
            SenderId: SenderId::<Identity, Impl, OFFSET>,
            SenderIdType: SenderIdType::<Identity, Impl, OFFSET>,
            SetSenderIdType: SetSenderIdType::<Identity, Impl, OFFSET>,
            Send: Send::<Identity, Impl, OFFSET>,
            AttachCurrentSecurityContext: AttachCurrentSecurityContext::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQMessage as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQMessage2_Impl: Sized + super::Com::IDispatch_Impl {
    fn Class(&self) -> ::windows::core::Result<i32>;
    fn PrivLevel(&self) -> ::windows::core::Result<i32>;
    fn SetPrivLevel(&self, lprivlevel: i32) -> ::windows::core::Result<()>;
    fn AuthLevel(&self) -> ::windows::core::Result<i32>;
    fn SetAuthLevel(&self, lauthlevel: i32) -> ::windows::core::Result<()>;
    fn IsAuthenticated(&self) -> ::windows::core::Result<i16>;
    fn Delivery(&self) -> ::windows::core::Result<i32>;
    fn SetDelivery(&self, ldelivery: i32) -> ::windows::core::Result<()>;
    fn Trace(&self) -> ::windows::core::Result<i32>;
    fn SetTrace(&self, ltrace: i32) -> ::windows::core::Result<()>;
    fn Priority(&self) -> ::windows::core::Result<i32>;
    fn SetPriority(&self, lpriority: i32) -> ::windows::core::Result<()>;
    fn Journal(&self) -> ::windows::core::Result<i32>;
    fn SetJournal(&self, ljournal: i32) -> ::windows::core::Result<()>;
    fn ResponseQueueInfo_v1(&self) -> ::windows::core::Result<IMSMQQueueInfo>;
    fn putref_ResponseQueueInfo_v1(&self, pqinforesponse: &::core::option::Option<IMSMQQueueInfo>) -> ::windows::core::Result<()>;
    fn AppSpecific(&self) -> ::windows::core::Result<i32>;
    fn SetAppSpecific(&self, lappspecific: i32) -> ::windows::core::Result<()>;
    fn SourceMachineGuid(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn BodyLength(&self) -> ::windows::core::Result<i32>;
    fn Body(&self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SetBody(&self, varbody: &super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn AdminQueueInfo_v1(&self) -> ::windows::core::Result<IMSMQQueueInfo>;
    fn putref_AdminQueueInfo_v1(&self, pqinfoadmin: &::core::option::Option<IMSMQQueueInfo>) -> ::windows::core::Result<()>;
    fn Id(&self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn CorrelationId(&self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SetCorrelationId(&self, varmsgid: &super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Ack(&self) -> ::windows::core::Result<i32>;
    fn SetAck(&self, lack: i32) -> ::windows::core::Result<()>;
    fn Label(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetLabel(&self, bstrlabel: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn MaxTimeToReachQueue(&self) -> ::windows::core::Result<i32>;
    fn SetMaxTimeToReachQueue(&self, lmaxtimetoreachqueue: i32) -> ::windows::core::Result<()>;
    fn MaxTimeToReceive(&self) -> ::windows::core::Result<i32>;
    fn SetMaxTimeToReceive(&self, lmaxtimetoreceive: i32) -> ::windows::core::Result<()>;
    fn HashAlgorithm(&self) -> ::windows::core::Result<i32>;
    fn SetHashAlgorithm(&self, lhashalg: i32) -> ::windows::core::Result<()>;
    fn EncryptAlgorithm(&self) -> ::windows::core::Result<i32>;
    fn SetEncryptAlgorithm(&self, lencryptalg: i32) -> ::windows::core::Result<()>;
    fn SentTime(&self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn ArrivedTime(&self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn DestinationQueueInfo(&self) -> ::windows::core::Result<IMSMQQueueInfo2>;
    fn SenderCertificate(&self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SetSenderCertificate(&self, varsendercert: &super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn SenderId(&self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SenderIdType(&self) -> ::windows::core::Result<i32>;
    fn SetSenderIdType(&self, lsenderidtype: i32) -> ::windows::core::Result<()>;
    fn Send(&self, destinationqueue: &::core::option::Option<IMSMQQueue2>, transaction: *const super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn AttachCurrentSecurityContext(&self) -> ::windows::core::Result<()>;
    fn SenderVersion(&self) -> ::windows::core::Result<i32>;
    fn Extension(&self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SetExtension(&self, varextension: &super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn ConnectorTypeGuid(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetConnectorTypeGuid(&self, bstrguidconnectortype: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn TransactionStatusQueueInfo(&self) -> ::windows::core::Result<IMSMQQueueInfo2>;
    fn DestinationSymmetricKey(&self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SetDestinationSymmetricKey(&self, vardestsymmkey: &super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Signature(&self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SetSignature(&self, varsignature: &super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn AuthenticationProviderType(&self) -> ::windows::core::Result<i32>;
    fn SetAuthenticationProviderType(&self, lauthprovtype: i32) -> ::windows::core::Result<()>;
    fn AuthenticationProviderName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetAuthenticationProviderName(&self, bstrauthprovname: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetSenderId(&self, varsenderid: &super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn MsgClass(&self) -> ::windows::core::Result<i32>;
    fn SetMsgClass(&self, lmsgclass: i32) -> ::windows::core::Result<()>;
    fn Properties(&self) -> ::windows::core::Result<super::Com::IDispatch>;
    fn TransactionId(&self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn IsFirstInTransaction(&self) -> ::windows::core::Result<i16>;
    fn IsLastInTransaction(&self) -> ::windows::core::Result<i16>;
    fn ResponseQueueInfo(&self) -> ::windows::core::Result<IMSMQQueueInfo2>;
    fn putref_ResponseQueueInfo(&self, pqinforesponse: &::core::option::Option<IMSMQQueueInfo2>) -> ::windows::core::Result<()>;
    fn AdminQueueInfo(&self) -> ::windows::core::Result<IMSMQQueueInfo2>;
    fn putref_AdminQueueInfo(&self, pqinfoadmin: &::core::option::Option<IMSMQQueueInfo2>) -> ::windows::core::Result<()>;
    fn ReceivedAuthenticationLevel(&self) -> ::windows::core::Result<i16>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQMessage2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage2_Impl, const OFFSET: isize>() -> IMSMQMessage2_Vtbl {
        unsafe extern "system" fn Class<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plclass: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Class() {
                ::core::result::Result::Ok(ok__) => {
                    *plclass = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrivLevel<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plprivlevel: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PrivLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *plprivlevel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrivLevel<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lprivlevel: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPrivLevel(::core::mem::transmute_copy(&lprivlevel)).into()
        }
        unsafe extern "system" fn AuthLevel<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plauthlevel: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AuthLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *plauthlevel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAuthLevel<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lauthlevel: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAuthLevel(::core::mem::transmute_copy(&lauthlevel)).into()
        }
        unsafe extern "system" fn IsAuthenticated<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisauthenticated: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsAuthenticated() {
                ::core::result::Result::Ok(ok__) => {
                    *pisauthenticated = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delivery<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pldelivery: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Delivery() {
                ::core::result::Result::Ok(ok__) => {
                    *pldelivery = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDelivery<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ldelivery: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDelivery(::core::mem::transmute_copy(&ldelivery)).into()
        }
        unsafe extern "system" fn Trace<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pltrace: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Trace() {
                ::core::result::Result::Ok(ok__) => {
                    *pltrace = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTrace<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ltrace: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetTrace(::core::mem::transmute_copy(&ltrace)).into()
        }
        unsafe extern "system" fn Priority<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plpriority: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Priority() {
                ::core::result::Result::Ok(ok__) => {
                    *plpriority = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPriority<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpriority: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPriority(::core::mem::transmute_copy(&lpriority)).into()
        }
        unsafe extern "system" fn Journal<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pljournal: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Journal() {
                ::core::result::Result::Ok(ok__) => {
                    *pljournal = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetJournal<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ljournal: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetJournal(::core::mem::transmute_copy(&ljournal)).into()
        }
        unsafe extern "system" fn ResponseQueueInfo_v1<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinforesponse: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ResponseQueueInfo_v1() {
                ::core::result::Result::Ok(ok__) => {
                    *ppqinforesponse = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_ResponseQueueInfo_v1<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqinforesponse: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).putref_ResponseQueueInfo_v1(::core::mem::transmute(&pqinforesponse)).into()
        }
        unsafe extern "system" fn AppSpecific<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plappspecific: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AppSpecific() {
                ::core::result::Result::Ok(ok__) => {
                    *plappspecific = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAppSpecific<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lappspecific: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAppSpecific(::core::mem::transmute_copy(&lappspecific)).into()
        }
        unsafe extern "system" fn SourceMachineGuid<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrguidsrcmachine: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SourceMachineGuid() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrguidsrcmachine = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BodyLength<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcbbody: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).BodyLength() {
                ::core::result::Result::Ok(ok__) => {
                    *pcbbody = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Body<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarbody: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Body() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarbody = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBody<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varbody: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetBody(::core::mem::transmute(&varbody)).into()
        }
        unsafe extern "system" fn AdminQueueInfo_v1<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinfoadmin: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AdminQueueInfo_v1() {
                ::core::result::Result::Ok(ok__) => {
                    *ppqinfoadmin = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_AdminQueueInfo_v1<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqinfoadmin: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).putref_AdminQueueInfo_v1(::core::mem::transmute(&pqinfoadmin)).into()
        }
        unsafe extern "system" fn Id<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarmsgid: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarmsgid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CorrelationId<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarmsgid: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CorrelationId() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarmsgid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCorrelationId<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varmsgid: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCorrelationId(::core::mem::transmute(&varmsgid)).into()
        }
        unsafe extern "system" fn Ack<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plack: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Ack() {
                ::core::result::Result::Ok(ok__) => {
                    *plack = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAck<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lack: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAck(::core::mem::transmute_copy(&lack)).into()
        }
        unsafe extern "system" fn Label<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrlabel: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Label() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrlabel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLabel<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrlabel: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetLabel(::core::mem::transmute(&bstrlabel)).into()
        }
        unsafe extern "system" fn MaxTimeToReachQueue<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmaxtimetoreachqueue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MaxTimeToReachQueue() {
                ::core::result::Result::Ok(ok__) => {
                    *plmaxtimetoreachqueue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxTimeToReachQueue<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmaxtimetoreachqueue: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetMaxTimeToReachQueue(::core::mem::transmute_copy(&lmaxtimetoreachqueue)).into()
        }
        unsafe extern "system" fn MaxTimeToReceive<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmaxtimetoreceive: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MaxTimeToReceive() {
                ::core::result::Result::Ok(ok__) => {
                    *plmaxtimetoreceive = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxTimeToReceive<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmaxtimetoreceive: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetMaxTimeToReceive(::core::mem::transmute_copy(&lmaxtimetoreceive)).into()
        }
        unsafe extern "system" fn HashAlgorithm<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plhashalg: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).HashAlgorithm() {
                ::core::result::Result::Ok(ok__) => {
                    *plhashalg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHashAlgorithm<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lhashalg: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetHashAlgorithm(::core::mem::transmute_copy(&lhashalg)).into()
        }
        unsafe extern "system" fn EncryptAlgorithm<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plencryptalg: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EncryptAlgorithm() {
                ::core::result::Result::Ok(ok__) => {
                    *plencryptalg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEncryptAlgorithm<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lencryptalg: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetEncryptAlgorithm(::core::mem::transmute_copy(&lencryptalg)).into()
        }
        unsafe extern "system" fn SentTime<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarsenttime: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SentTime() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarsenttime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ArrivedTime<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plarrivedtime: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ArrivedTime() {
                ::core::result::Result::Ok(ok__) => {
                    *plarrivedtime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DestinationQueueInfo<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinfodest: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DestinationQueueInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *ppqinfodest = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SenderCertificate<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarsendercert: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SenderCertificate() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarsendercert = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSenderCertificate<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varsendercert: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSenderCertificate(::core::mem::transmute(&varsendercert)).into()
        }
        unsafe extern "system" fn SenderId<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarsenderid: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SenderId() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarsenderid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SenderIdType<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plsenderidtype: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SenderIdType() {
                ::core::result::Result::Ok(ok__) => {
                    *plsenderidtype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSenderIdType<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lsenderidtype: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSenderIdType(::core::mem::transmute_copy(&lsenderidtype)).into()
        }
        unsafe extern "system" fn Send<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, destinationqueue: ::windows::core::RawPtr, transaction: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Send(::core::mem::transmute(&destinationqueue), ::core::mem::transmute_copy(&transaction)).into()
        }
        unsafe extern "system" fn AttachCurrentSecurityContext<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AttachCurrentSecurityContext().into()
        }
        unsafe extern "system" fn SenderVersion<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plsenderversion: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SenderVersion() {
                ::core::result::Result::Ok(ok__) => {
                    *plsenderversion = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Extension<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarextension: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Extension() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarextension = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExtension<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varextension: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetExtension(::core::mem::transmute(&varextension)).into()
        }
        unsafe extern "system" fn ConnectorTypeGuid<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrguidconnectortype: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ConnectorTypeGuid() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrguidconnectortype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetConnectorTypeGuid<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrguidconnectortype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetConnectorTypeGuid(::core::mem::transmute(&bstrguidconnectortype)).into()
        }
        unsafe extern "system" fn TransactionStatusQueueInfo<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinfoxactstatus: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).TransactionStatusQueueInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *ppqinfoxactstatus = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DestinationSymmetricKey<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvardestsymmkey: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DestinationSymmetricKey() {
                ::core::result::Result::Ok(ok__) => {
                    *pvardestsymmkey = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDestinationSymmetricKey<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vardestsymmkey: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDestinationSymmetricKey(::core::mem::transmute(&vardestsymmkey)).into()
        }
        unsafe extern "system" fn Signature<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarsignature: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Signature() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarsignature = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSignature<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varsignature: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSignature(::core::mem::transmute(&varsignature)).into()
        }
        unsafe extern "system" fn AuthenticationProviderType<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plauthprovtype: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AuthenticationProviderType() {
                ::core::result::Result::Ok(ok__) => {
                    *plauthprovtype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAuthenticationProviderType<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lauthprovtype: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAuthenticationProviderType(::core::mem::transmute_copy(&lauthprovtype)).into()
        }
        unsafe extern "system" fn AuthenticationProviderName<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrauthprovname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AuthenticationProviderName() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrauthprovname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAuthenticationProviderName<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrauthprovname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAuthenticationProviderName(::core::mem::transmute(&bstrauthprovname)).into()
        }
        unsafe extern "system" fn SetSenderId<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varsenderid: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSenderId(::core::mem::transmute(&varsenderid)).into()
        }
        unsafe extern "system" fn MsgClass<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmsgclass: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MsgClass() {
                ::core::result::Result::Ok(ok__) => {
                    *plmsgclass = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMsgClass<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmsgclass: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetMsgClass(::core::mem::transmute_copy(&lmsgclass)).into()
        }
        unsafe extern "system" fn Properties<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcolproperties = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransactionId<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarxactid: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).TransactionId() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarxactid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsFirstInTransaction<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisfirstinxact: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsFirstInTransaction() {
                ::core::result::Result::Ok(ok__) => {
                    *pisfirstinxact = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsLastInTransaction<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pislastinxact: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsLastInTransaction() {
                ::core::result::Result::Ok(ok__) => {
                    *pislastinxact = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResponseQueueInfo<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinforesponse: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ResponseQueueInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *ppqinforesponse = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_ResponseQueueInfo<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqinforesponse: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).putref_ResponseQueueInfo(::core::mem::transmute(&pqinforesponse)).into()
        }
        unsafe extern "system" fn AdminQueueInfo<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinfoadmin: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AdminQueueInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *ppqinfoadmin = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_AdminQueueInfo<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqinfoadmin: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).putref_AdminQueueInfo(::core::mem::transmute(&pqinfoadmin)).into()
        }
        unsafe extern "system" fn ReceivedAuthenticationLevel<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psreceivedauthenticationlevel: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ReceivedAuthenticationLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *psreceivedauthenticationlevel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Class: Class::<Identity, Impl, OFFSET>,
            PrivLevel: PrivLevel::<Identity, Impl, OFFSET>,
            SetPrivLevel: SetPrivLevel::<Identity, Impl, OFFSET>,
            AuthLevel: AuthLevel::<Identity, Impl, OFFSET>,
            SetAuthLevel: SetAuthLevel::<Identity, Impl, OFFSET>,
            IsAuthenticated: IsAuthenticated::<Identity, Impl, OFFSET>,
            Delivery: Delivery::<Identity, Impl, OFFSET>,
            SetDelivery: SetDelivery::<Identity, Impl, OFFSET>,
            Trace: Trace::<Identity, Impl, OFFSET>,
            SetTrace: SetTrace::<Identity, Impl, OFFSET>,
            Priority: Priority::<Identity, Impl, OFFSET>,
            SetPriority: SetPriority::<Identity, Impl, OFFSET>,
            Journal: Journal::<Identity, Impl, OFFSET>,
            SetJournal: SetJournal::<Identity, Impl, OFFSET>,
            ResponseQueueInfo_v1: ResponseQueueInfo_v1::<Identity, Impl, OFFSET>,
            putref_ResponseQueueInfo_v1: putref_ResponseQueueInfo_v1::<Identity, Impl, OFFSET>,
            AppSpecific: AppSpecific::<Identity, Impl, OFFSET>,
            SetAppSpecific: SetAppSpecific::<Identity, Impl, OFFSET>,
            SourceMachineGuid: SourceMachineGuid::<Identity, Impl, OFFSET>,
            BodyLength: BodyLength::<Identity, Impl, OFFSET>,
            Body: Body::<Identity, Impl, OFFSET>,
            SetBody: SetBody::<Identity, Impl, OFFSET>,
            AdminQueueInfo_v1: AdminQueueInfo_v1::<Identity, Impl, OFFSET>,
            putref_AdminQueueInfo_v1: putref_AdminQueueInfo_v1::<Identity, Impl, OFFSET>,
            Id: Id::<Identity, Impl, OFFSET>,
            CorrelationId: CorrelationId::<Identity, Impl, OFFSET>,
            SetCorrelationId: SetCorrelationId::<Identity, Impl, OFFSET>,
            Ack: Ack::<Identity, Impl, OFFSET>,
            SetAck: SetAck::<Identity, Impl, OFFSET>,
            Label: Label::<Identity, Impl, OFFSET>,
            SetLabel: SetLabel::<Identity, Impl, OFFSET>,
            MaxTimeToReachQueue: MaxTimeToReachQueue::<Identity, Impl, OFFSET>,
            SetMaxTimeToReachQueue: SetMaxTimeToReachQueue::<Identity, Impl, OFFSET>,
            MaxTimeToReceive: MaxTimeToReceive::<Identity, Impl, OFFSET>,
            SetMaxTimeToReceive: SetMaxTimeToReceive::<Identity, Impl, OFFSET>,
            HashAlgorithm: HashAlgorithm::<Identity, Impl, OFFSET>,
            SetHashAlgorithm: SetHashAlgorithm::<Identity, Impl, OFFSET>,
            EncryptAlgorithm: EncryptAlgorithm::<Identity, Impl, OFFSET>,
            SetEncryptAlgorithm: SetEncryptAlgorithm::<Identity, Impl, OFFSET>,
            SentTime: SentTime::<Identity, Impl, OFFSET>,
            ArrivedTime: ArrivedTime::<Identity, Impl, OFFSET>,
            DestinationQueueInfo: DestinationQueueInfo::<Identity, Impl, OFFSET>,
            SenderCertificate: SenderCertificate::<Identity, Impl, OFFSET>,
            SetSenderCertificate: SetSenderCertificate::<Identity, Impl, OFFSET>,
            SenderId: SenderId::<Identity, Impl, OFFSET>,
            SenderIdType: SenderIdType::<Identity, Impl, OFFSET>,
            SetSenderIdType: SetSenderIdType::<Identity, Impl, OFFSET>,
            Send: Send::<Identity, Impl, OFFSET>,
            AttachCurrentSecurityContext: AttachCurrentSecurityContext::<Identity, Impl, OFFSET>,
            SenderVersion: SenderVersion::<Identity, Impl, OFFSET>,
            Extension: Extension::<Identity, Impl, OFFSET>,
            SetExtension: SetExtension::<Identity, Impl, OFFSET>,
            ConnectorTypeGuid: ConnectorTypeGuid::<Identity, Impl, OFFSET>,
            SetConnectorTypeGuid: SetConnectorTypeGuid::<Identity, Impl, OFFSET>,
            TransactionStatusQueueInfo: TransactionStatusQueueInfo::<Identity, Impl, OFFSET>,
            DestinationSymmetricKey: DestinationSymmetricKey::<Identity, Impl, OFFSET>,
            SetDestinationSymmetricKey: SetDestinationSymmetricKey::<Identity, Impl, OFFSET>,
            Signature: Signature::<Identity, Impl, OFFSET>,
            SetSignature: SetSignature::<Identity, Impl, OFFSET>,
            AuthenticationProviderType: AuthenticationProviderType::<Identity, Impl, OFFSET>,
            SetAuthenticationProviderType: SetAuthenticationProviderType::<Identity, Impl, OFFSET>,
            AuthenticationProviderName: AuthenticationProviderName::<Identity, Impl, OFFSET>,
            SetAuthenticationProviderName: SetAuthenticationProviderName::<Identity, Impl, OFFSET>,
            SetSenderId: SetSenderId::<Identity, Impl, OFFSET>,
            MsgClass: MsgClass::<Identity, Impl, OFFSET>,
            SetMsgClass: SetMsgClass::<Identity, Impl, OFFSET>,
            Properties: Properties::<Identity, Impl, OFFSET>,
            TransactionId: TransactionId::<Identity, Impl, OFFSET>,
            IsFirstInTransaction: IsFirstInTransaction::<Identity, Impl, OFFSET>,
            IsLastInTransaction: IsLastInTransaction::<Identity, Impl, OFFSET>,
            ResponseQueueInfo: ResponseQueueInfo::<Identity, Impl, OFFSET>,
            putref_ResponseQueueInfo: putref_ResponseQueueInfo::<Identity, Impl, OFFSET>,
            AdminQueueInfo: AdminQueueInfo::<Identity, Impl, OFFSET>,
            putref_AdminQueueInfo: putref_AdminQueueInfo::<Identity, Impl, OFFSET>,
            ReceivedAuthenticationLevel: ReceivedAuthenticationLevel::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQMessage2 as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQMessage3_Impl: Sized + super::Com::IDispatch_Impl {
    fn Class(&self) -> ::windows::core::Result<i32>;
    fn PrivLevel(&self) -> ::windows::core::Result<i32>;
    fn SetPrivLevel(&self, lprivlevel: i32) -> ::windows::core::Result<()>;
    fn AuthLevel(&self) -> ::windows::core::Result<i32>;
    fn SetAuthLevel(&self, lauthlevel: i32) -> ::windows::core::Result<()>;
    fn IsAuthenticated(&self) -> ::windows::core::Result<i16>;
    fn Delivery(&self) -> ::windows::core::Result<i32>;
    fn SetDelivery(&self, ldelivery: i32) -> ::windows::core::Result<()>;
    fn Trace(&self) -> ::windows::core::Result<i32>;
    fn SetTrace(&self, ltrace: i32) -> ::windows::core::Result<()>;
    fn Priority(&self) -> ::windows::core::Result<i32>;
    fn SetPriority(&self, lpriority: i32) -> ::windows::core::Result<()>;
    fn Journal(&self) -> ::windows::core::Result<i32>;
    fn SetJournal(&self, ljournal: i32) -> ::windows::core::Result<()>;
    fn ResponseQueueInfo_v1(&self) -> ::windows::core::Result<IMSMQQueueInfo>;
    fn putref_ResponseQueueInfo_v1(&self, pqinforesponse: &::core::option::Option<IMSMQQueueInfo>) -> ::windows::core::Result<()>;
    fn AppSpecific(&self) -> ::windows::core::Result<i32>;
    fn SetAppSpecific(&self, lappspecific: i32) -> ::windows::core::Result<()>;
    fn SourceMachineGuid(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn BodyLength(&self) -> ::windows::core::Result<i32>;
    fn Body(&self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SetBody(&self, varbody: &super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn AdminQueueInfo_v1(&self) -> ::windows::core::Result<IMSMQQueueInfo>;
    fn putref_AdminQueueInfo_v1(&self, pqinfoadmin: &::core::option::Option<IMSMQQueueInfo>) -> ::windows::core::Result<()>;
    fn Id(&self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn CorrelationId(&self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SetCorrelationId(&self, varmsgid: &super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Ack(&self) -> ::windows::core::Result<i32>;
    fn SetAck(&self, lack: i32) -> ::windows::core::Result<()>;
    fn Label(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetLabel(&self, bstrlabel: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn MaxTimeToReachQueue(&self) -> ::windows::core::Result<i32>;
    fn SetMaxTimeToReachQueue(&self, lmaxtimetoreachqueue: i32) -> ::windows::core::Result<()>;
    fn MaxTimeToReceive(&self) -> ::windows::core::Result<i32>;
    fn SetMaxTimeToReceive(&self, lmaxtimetoreceive: i32) -> ::windows::core::Result<()>;
    fn HashAlgorithm(&self) -> ::windows::core::Result<i32>;
    fn SetHashAlgorithm(&self, lhashalg: i32) -> ::windows::core::Result<()>;
    fn EncryptAlgorithm(&self) -> ::windows::core::Result<i32>;
    fn SetEncryptAlgorithm(&self, lencryptalg: i32) -> ::windows::core::Result<()>;
    fn SentTime(&self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn ArrivedTime(&self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn DestinationQueueInfo(&self) -> ::windows::core::Result<IMSMQQueueInfo3>;
    fn SenderCertificate(&self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SetSenderCertificate(&self, varsendercert: &super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn SenderId(&self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SenderIdType(&self) -> ::windows::core::Result<i32>;
    fn SetSenderIdType(&self, lsenderidtype: i32) -> ::windows::core::Result<()>;
    fn Send(&self, destinationqueue: &::core::option::Option<super::Com::IDispatch>, transaction: *const super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn AttachCurrentSecurityContext(&self) -> ::windows::core::Result<()>;
    fn SenderVersion(&self) -> ::windows::core::Result<i32>;
    fn Extension(&self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SetExtension(&self, varextension: &super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn ConnectorTypeGuid(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetConnectorTypeGuid(&self, bstrguidconnectortype: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn TransactionStatusQueueInfo(&self) -> ::windows::core::Result<IMSMQQueueInfo3>;
    fn DestinationSymmetricKey(&self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SetDestinationSymmetricKey(&self, vardestsymmkey: &super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Signature(&self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SetSignature(&self, varsignature: &super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn AuthenticationProviderType(&self) -> ::windows::core::Result<i32>;
    fn SetAuthenticationProviderType(&self, lauthprovtype: i32) -> ::windows::core::Result<()>;
    fn AuthenticationProviderName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetAuthenticationProviderName(&self, bstrauthprovname: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetSenderId(&self, varsenderid: &super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn MsgClass(&self) -> ::windows::core::Result<i32>;
    fn SetMsgClass(&self, lmsgclass: i32) -> ::windows::core::Result<()>;
    fn Properties(&self) -> ::windows::core::Result<super::Com::IDispatch>;
    fn TransactionId(&self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn IsFirstInTransaction(&self) -> ::windows::core::Result<i16>;
    fn IsLastInTransaction(&self) -> ::windows::core::Result<i16>;
    fn ResponseQueueInfo_v2(&self) -> ::windows::core::Result<IMSMQQueueInfo2>;
    fn putref_ResponseQueueInfo_v2(&self, pqinforesponse: &::core::option::Option<IMSMQQueueInfo2>) -> ::windows::core::Result<()>;
    fn AdminQueueInfo_v2(&self) -> ::windows::core::Result<IMSMQQueueInfo2>;
    fn putref_AdminQueueInfo_v2(&self, pqinfoadmin: &::core::option::Option<IMSMQQueueInfo2>) -> ::windows::core::Result<()>;
    fn ReceivedAuthenticationLevel(&self) -> ::windows::core::Result<i16>;
    fn ResponseQueueInfo(&self) -> ::windows::core::Result<IMSMQQueueInfo3>;
    fn putref_ResponseQueueInfo(&self, pqinforesponse: &::core::option::Option<IMSMQQueueInfo3>) -> ::windows::core::Result<()>;
    fn AdminQueueInfo(&self) -> ::windows::core::Result<IMSMQQueueInfo3>;
    fn putref_AdminQueueInfo(&self, pqinfoadmin: &::core::option::Option<IMSMQQueueInfo3>) -> ::windows::core::Result<()>;
    fn ResponseDestination(&self) -> ::windows::core::Result<super::Com::IDispatch>;
    fn putref_ResponseDestination(&self, pdestresponse: &::core::option::Option<super::Com::IDispatch>) -> ::windows::core::Result<()>;
    fn Destination(&self) -> ::windows::core::Result<super::Com::IDispatch>;
    fn LookupId(&self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn IsAuthenticated2(&self) -> ::windows::core::Result<i16>;
    fn IsFirstInTransaction2(&self) -> ::windows::core::Result<i16>;
    fn IsLastInTransaction2(&self) -> ::windows::core::Result<i16>;
    fn AttachCurrentSecurityContext2(&self) -> ::windows::core::Result<()>;
    fn SoapEnvelope(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn CompoundMessage(&self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SetSoapHeader(&self, bstrsoapheader: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetSoapBody(&self, bstrsoapbody: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQMessage3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage3_Impl, const OFFSET: isize>() -> IMSMQMessage3_Vtbl {
        unsafe extern "system" fn Class<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plclass: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Class() {
                ::core::result::Result::Ok(ok__) => {
                    *plclass = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrivLevel<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plprivlevel: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PrivLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *plprivlevel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrivLevel<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lprivlevel: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPrivLevel(::core::mem::transmute_copy(&lprivlevel)).into()
        }
        unsafe extern "system" fn AuthLevel<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plauthlevel: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AuthLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *plauthlevel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAuthLevel<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lauthlevel: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAuthLevel(::core::mem::transmute_copy(&lauthlevel)).into()
        }
        unsafe extern "system" fn IsAuthenticated<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisauthenticated: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsAuthenticated() {
                ::core::result::Result::Ok(ok__) => {
                    *pisauthenticated = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delivery<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pldelivery: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Delivery() {
                ::core::result::Result::Ok(ok__) => {
                    *pldelivery = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDelivery<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ldelivery: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDelivery(::core::mem::transmute_copy(&ldelivery)).into()
        }
        unsafe extern "system" fn Trace<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pltrace: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Trace() {
                ::core::result::Result::Ok(ok__) => {
                    *pltrace = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTrace<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ltrace: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetTrace(::core::mem::transmute_copy(&ltrace)).into()
        }
        unsafe extern "system" fn Priority<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plpriority: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Priority() {
                ::core::result::Result::Ok(ok__) => {
                    *plpriority = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPriority<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpriority: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPriority(::core::mem::transmute_copy(&lpriority)).into()
        }
        unsafe extern "system" fn Journal<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pljournal: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Journal() {
                ::core::result::Result::Ok(ok__) => {
                    *pljournal = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetJournal<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ljournal: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetJournal(::core::mem::transmute_copy(&ljournal)).into()
        }
        unsafe extern "system" fn ResponseQueueInfo_v1<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinforesponse: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ResponseQueueInfo_v1() {
                ::core::result::Result::Ok(ok__) => {
                    *ppqinforesponse = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_ResponseQueueInfo_v1<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqinforesponse: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).putref_ResponseQueueInfo_v1(::core::mem::transmute(&pqinforesponse)).into()
        }
        unsafe extern "system" fn AppSpecific<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plappspecific: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AppSpecific() {
                ::core::result::Result::Ok(ok__) => {
                    *plappspecific = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAppSpecific<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lappspecific: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAppSpecific(::core::mem::transmute_copy(&lappspecific)).into()
        }
        unsafe extern "system" fn SourceMachineGuid<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrguidsrcmachine: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SourceMachineGuid() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrguidsrcmachine = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BodyLength<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcbbody: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).BodyLength() {
                ::core::result::Result::Ok(ok__) => {
                    *pcbbody = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Body<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarbody: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Body() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarbody = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBody<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varbody: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetBody(::core::mem::transmute(&varbody)).into()
        }
        unsafe extern "system" fn AdminQueueInfo_v1<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinfoadmin: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AdminQueueInfo_v1() {
                ::core::result::Result::Ok(ok__) => {
                    *ppqinfoadmin = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_AdminQueueInfo_v1<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqinfoadmin: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).putref_AdminQueueInfo_v1(::core::mem::transmute(&pqinfoadmin)).into()
        }
        unsafe extern "system" fn Id<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarmsgid: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarmsgid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CorrelationId<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarmsgid: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CorrelationId() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarmsgid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCorrelationId<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varmsgid: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCorrelationId(::core::mem::transmute(&varmsgid)).into()
        }
        unsafe extern "system" fn Ack<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plack: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Ack() {
                ::core::result::Result::Ok(ok__) => {
                    *plack = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAck<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lack: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAck(::core::mem::transmute_copy(&lack)).into()
        }
        unsafe extern "system" fn Label<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrlabel: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Label() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrlabel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLabel<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrlabel: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetLabel(::core::mem::transmute(&bstrlabel)).into()
        }
        unsafe extern "system" fn MaxTimeToReachQueue<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmaxtimetoreachqueue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MaxTimeToReachQueue() {
                ::core::result::Result::Ok(ok__) => {
                    *plmaxtimetoreachqueue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxTimeToReachQueue<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmaxtimetoreachqueue: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetMaxTimeToReachQueue(::core::mem::transmute_copy(&lmaxtimetoreachqueue)).into()
        }
        unsafe extern "system" fn MaxTimeToReceive<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmaxtimetoreceive: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MaxTimeToReceive() {
                ::core::result::Result::Ok(ok__) => {
                    *plmaxtimetoreceive = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxTimeToReceive<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmaxtimetoreceive: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetMaxTimeToReceive(::core::mem::transmute_copy(&lmaxtimetoreceive)).into()
        }
        unsafe extern "system" fn HashAlgorithm<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plhashalg: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).HashAlgorithm() {
                ::core::result::Result::Ok(ok__) => {
                    *plhashalg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHashAlgorithm<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lhashalg: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetHashAlgorithm(::core::mem::transmute_copy(&lhashalg)).into()
        }
        unsafe extern "system" fn EncryptAlgorithm<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plencryptalg: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EncryptAlgorithm() {
                ::core::result::Result::Ok(ok__) => {
                    *plencryptalg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEncryptAlgorithm<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lencryptalg: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetEncryptAlgorithm(::core::mem::transmute_copy(&lencryptalg)).into()
        }
        unsafe extern "system" fn SentTime<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarsenttime: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SentTime() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarsenttime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ArrivedTime<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plarrivedtime: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ArrivedTime() {
                ::core::result::Result::Ok(ok__) => {
                    *plarrivedtime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DestinationQueueInfo<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinfodest: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DestinationQueueInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *ppqinfodest = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SenderCertificate<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarsendercert: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SenderCertificate() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarsendercert = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSenderCertificate<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varsendercert: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSenderCertificate(::core::mem::transmute(&varsendercert)).into()
        }
        unsafe extern "system" fn SenderId<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarsenderid: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SenderId() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarsenderid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SenderIdType<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plsenderidtype: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SenderIdType() {
                ::core::result::Result::Ok(ok__) => {
                    *plsenderidtype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSenderIdType<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lsenderidtype: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSenderIdType(::core::mem::transmute_copy(&lsenderidtype)).into()
        }
        unsafe extern "system" fn Send<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, destinationqueue: ::windows::core::RawPtr, transaction: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Send(::core::mem::transmute(&destinationqueue), ::core::mem::transmute_copy(&transaction)).into()
        }
        unsafe extern "system" fn AttachCurrentSecurityContext<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AttachCurrentSecurityContext().into()
        }
        unsafe extern "system" fn SenderVersion<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plsenderversion: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SenderVersion() {
                ::core::result::Result::Ok(ok__) => {
                    *plsenderversion = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Extension<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarextension: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Extension() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarextension = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExtension<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varextension: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetExtension(::core::mem::transmute(&varextension)).into()
        }
        unsafe extern "system" fn ConnectorTypeGuid<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrguidconnectortype: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ConnectorTypeGuid() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrguidconnectortype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetConnectorTypeGuid<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrguidconnectortype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetConnectorTypeGuid(::core::mem::transmute(&bstrguidconnectortype)).into()
        }
        unsafe extern "system" fn TransactionStatusQueueInfo<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinfoxactstatus: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).TransactionStatusQueueInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *ppqinfoxactstatus = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DestinationSymmetricKey<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvardestsymmkey: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DestinationSymmetricKey() {
                ::core::result::Result::Ok(ok__) => {
                    *pvardestsymmkey = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDestinationSymmetricKey<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vardestsymmkey: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDestinationSymmetricKey(::core::mem::transmute(&vardestsymmkey)).into()
        }
        unsafe extern "system" fn Signature<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarsignature: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Signature() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarsignature = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSignature<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varsignature: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSignature(::core::mem::transmute(&varsignature)).into()
        }
        unsafe extern "system" fn AuthenticationProviderType<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plauthprovtype: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AuthenticationProviderType() {
                ::core::result::Result::Ok(ok__) => {
                    *plauthprovtype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAuthenticationProviderType<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lauthprovtype: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAuthenticationProviderType(::core::mem::transmute_copy(&lauthprovtype)).into()
        }
        unsafe extern "system" fn AuthenticationProviderName<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrauthprovname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AuthenticationProviderName() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrauthprovname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAuthenticationProviderName<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrauthprovname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAuthenticationProviderName(::core::mem::transmute(&bstrauthprovname)).into()
        }
        unsafe extern "system" fn SetSenderId<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varsenderid: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSenderId(::core::mem::transmute(&varsenderid)).into()
        }
        unsafe extern "system" fn MsgClass<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmsgclass: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MsgClass() {
                ::core::result::Result::Ok(ok__) => {
                    *plmsgclass = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMsgClass<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmsgclass: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetMsgClass(::core::mem::transmute_copy(&lmsgclass)).into()
        }
        unsafe extern "system" fn Properties<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcolproperties = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransactionId<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarxactid: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).TransactionId() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarxactid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsFirstInTransaction<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisfirstinxact: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsFirstInTransaction() {
                ::core::result::Result::Ok(ok__) => {
                    *pisfirstinxact = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsLastInTransaction<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pislastinxact: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsLastInTransaction() {
                ::core::result::Result::Ok(ok__) => {
                    *pislastinxact = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResponseQueueInfo_v2<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinforesponse: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ResponseQueueInfo_v2() {
                ::core::result::Result::Ok(ok__) => {
                    *ppqinforesponse = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_ResponseQueueInfo_v2<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqinforesponse: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).putref_ResponseQueueInfo_v2(::core::mem::transmute(&pqinforesponse)).into()
        }
        unsafe extern "system" fn AdminQueueInfo_v2<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinfoadmin: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AdminQueueInfo_v2() {
                ::core::result::Result::Ok(ok__) => {
                    *ppqinfoadmin = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_AdminQueueInfo_v2<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqinfoadmin: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).putref_AdminQueueInfo_v2(::core::mem::transmute(&pqinfoadmin)).into()
        }
        unsafe extern "system" fn ReceivedAuthenticationLevel<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psreceivedauthenticationlevel: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ReceivedAuthenticationLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *psreceivedauthenticationlevel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResponseQueueInfo<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinforesponse: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ResponseQueueInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *ppqinforesponse = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_ResponseQueueInfo<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqinforesponse: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).putref_ResponseQueueInfo(::core::mem::transmute(&pqinforesponse)).into()
        }
        unsafe extern "system" fn AdminQueueInfo<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinfoadmin: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AdminQueueInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *ppqinfoadmin = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_AdminQueueInfo<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqinfoadmin: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).putref_AdminQueueInfo(::core::mem::transmute(&pqinfoadmin)).into()
        }
        unsafe extern "system" fn ResponseDestination<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdestresponse: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ResponseDestination() {
                ::core::result::Result::Ok(ok__) => {
                    *ppdestresponse = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_ResponseDestination<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdestresponse: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).putref_ResponseDestination(::core::mem::transmute(&pdestresponse)).into()
        }
        unsafe extern "system" fn Destination<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdestdestination: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Destination() {
                ::core::result::Result::Ok(ok__) => {
                    *ppdestdestination = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LookupId<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarlookupid: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LookupId() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarlookupid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsAuthenticated2<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisauthenticated: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsAuthenticated2() {
                ::core::result::Result::Ok(ok__) => {
                    *pisauthenticated = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsFirstInTransaction2<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisfirstinxact: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsFirstInTransaction2() {
                ::core::result::Result::Ok(ok__) => {
                    *pisfirstinxact = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsLastInTransaction2<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pislastinxact: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsLastInTransaction2() {
                ::core::result::Result::Ok(ok__) => {
                    *pislastinxact = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AttachCurrentSecurityContext2<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AttachCurrentSecurityContext2().into()
        }
        unsafe extern "system" fn SoapEnvelope<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrsoapenvelope: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SoapEnvelope() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrsoapenvelope = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CompoundMessage<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarcompoundmessage: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CompoundMessage() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarcompoundmessage = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSoapHeader<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsoapheader: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSoapHeader(::core::mem::transmute(&bstrsoapheader)).into()
        }
        unsafe extern "system" fn SetSoapBody<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsoapbody: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSoapBody(::core::mem::transmute(&bstrsoapbody)).into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Class: Class::<Identity, Impl, OFFSET>,
            PrivLevel: PrivLevel::<Identity, Impl, OFFSET>,
            SetPrivLevel: SetPrivLevel::<Identity, Impl, OFFSET>,
            AuthLevel: AuthLevel::<Identity, Impl, OFFSET>,
            SetAuthLevel: SetAuthLevel::<Identity, Impl, OFFSET>,
            IsAuthenticated: IsAuthenticated::<Identity, Impl, OFFSET>,
            Delivery: Delivery::<Identity, Impl, OFFSET>,
            SetDelivery: SetDelivery::<Identity, Impl, OFFSET>,
            Trace: Trace::<Identity, Impl, OFFSET>,
            SetTrace: SetTrace::<Identity, Impl, OFFSET>,
            Priority: Priority::<Identity, Impl, OFFSET>,
            SetPriority: SetPriority::<Identity, Impl, OFFSET>,
            Journal: Journal::<Identity, Impl, OFFSET>,
            SetJournal: SetJournal::<Identity, Impl, OFFSET>,
            ResponseQueueInfo_v1: ResponseQueueInfo_v1::<Identity, Impl, OFFSET>,
            putref_ResponseQueueInfo_v1: putref_ResponseQueueInfo_v1::<Identity, Impl, OFFSET>,
            AppSpecific: AppSpecific::<Identity, Impl, OFFSET>,
            SetAppSpecific: SetAppSpecific::<Identity, Impl, OFFSET>,
            SourceMachineGuid: SourceMachineGuid::<Identity, Impl, OFFSET>,
            BodyLength: BodyLength::<Identity, Impl, OFFSET>,
            Body: Body::<Identity, Impl, OFFSET>,
            SetBody: SetBody::<Identity, Impl, OFFSET>,
            AdminQueueInfo_v1: AdminQueueInfo_v1::<Identity, Impl, OFFSET>,
            putref_AdminQueueInfo_v1: putref_AdminQueueInfo_v1::<Identity, Impl, OFFSET>,
            Id: Id::<Identity, Impl, OFFSET>,
            CorrelationId: CorrelationId::<Identity, Impl, OFFSET>,
            SetCorrelationId: SetCorrelationId::<Identity, Impl, OFFSET>,
            Ack: Ack::<Identity, Impl, OFFSET>,
            SetAck: SetAck::<Identity, Impl, OFFSET>,
            Label: Label::<Identity, Impl, OFFSET>,
            SetLabel: SetLabel::<Identity, Impl, OFFSET>,
            MaxTimeToReachQueue: MaxTimeToReachQueue::<Identity, Impl, OFFSET>,
            SetMaxTimeToReachQueue: SetMaxTimeToReachQueue::<Identity, Impl, OFFSET>,
            MaxTimeToReceive: MaxTimeToReceive::<Identity, Impl, OFFSET>,
            SetMaxTimeToReceive: SetMaxTimeToReceive::<Identity, Impl, OFFSET>,
            HashAlgorithm: HashAlgorithm::<Identity, Impl, OFFSET>,
            SetHashAlgorithm: SetHashAlgorithm::<Identity, Impl, OFFSET>,
            EncryptAlgorithm: EncryptAlgorithm::<Identity, Impl, OFFSET>,
            SetEncryptAlgorithm: SetEncryptAlgorithm::<Identity, Impl, OFFSET>,
            SentTime: SentTime::<Identity, Impl, OFFSET>,
            ArrivedTime: ArrivedTime::<Identity, Impl, OFFSET>,
            DestinationQueueInfo: DestinationQueueInfo::<Identity, Impl, OFFSET>,
            SenderCertificate: SenderCertificate::<Identity, Impl, OFFSET>,
            SetSenderCertificate: SetSenderCertificate::<Identity, Impl, OFFSET>,
            SenderId: SenderId::<Identity, Impl, OFFSET>,
            SenderIdType: SenderIdType::<Identity, Impl, OFFSET>,
            SetSenderIdType: SetSenderIdType::<Identity, Impl, OFFSET>,
            Send: Send::<Identity, Impl, OFFSET>,
            AttachCurrentSecurityContext: AttachCurrentSecurityContext::<Identity, Impl, OFFSET>,
            SenderVersion: SenderVersion::<Identity, Impl, OFFSET>,
            Extension: Extension::<Identity, Impl, OFFSET>,
            SetExtension: SetExtension::<Identity, Impl, OFFSET>,
            ConnectorTypeGuid: ConnectorTypeGuid::<Identity, Impl, OFFSET>,
            SetConnectorTypeGuid: SetConnectorTypeGuid::<Identity, Impl, OFFSET>,
            TransactionStatusQueueInfo: TransactionStatusQueueInfo::<Identity, Impl, OFFSET>,
            DestinationSymmetricKey: DestinationSymmetricKey::<Identity, Impl, OFFSET>,
            SetDestinationSymmetricKey: SetDestinationSymmetricKey::<Identity, Impl, OFFSET>,
            Signature: Signature::<Identity, Impl, OFFSET>,
            SetSignature: SetSignature::<Identity, Impl, OFFSET>,
            AuthenticationProviderType: AuthenticationProviderType::<Identity, Impl, OFFSET>,
            SetAuthenticationProviderType: SetAuthenticationProviderType::<Identity, Impl, OFFSET>,
            AuthenticationProviderName: AuthenticationProviderName::<Identity, Impl, OFFSET>,
            SetAuthenticationProviderName: SetAuthenticationProviderName::<Identity, Impl, OFFSET>,
            SetSenderId: SetSenderId::<Identity, Impl, OFFSET>,
            MsgClass: MsgClass::<Identity, Impl, OFFSET>,
            SetMsgClass: SetMsgClass::<Identity, Impl, OFFSET>,
            Properties: Properties::<Identity, Impl, OFFSET>,
            TransactionId: TransactionId::<Identity, Impl, OFFSET>,
            IsFirstInTransaction: IsFirstInTransaction::<Identity, Impl, OFFSET>,
            IsLastInTransaction: IsLastInTransaction::<Identity, Impl, OFFSET>,
            ResponseQueueInfo_v2: ResponseQueueInfo_v2::<Identity, Impl, OFFSET>,
            putref_ResponseQueueInfo_v2: putref_ResponseQueueInfo_v2::<Identity, Impl, OFFSET>,
            AdminQueueInfo_v2: AdminQueueInfo_v2::<Identity, Impl, OFFSET>,
            putref_AdminQueueInfo_v2: putref_AdminQueueInfo_v2::<Identity, Impl, OFFSET>,
            ReceivedAuthenticationLevel: ReceivedAuthenticationLevel::<Identity, Impl, OFFSET>,
            ResponseQueueInfo: ResponseQueueInfo::<Identity, Impl, OFFSET>,
            putref_ResponseQueueInfo: putref_ResponseQueueInfo::<Identity, Impl, OFFSET>,
            AdminQueueInfo: AdminQueueInfo::<Identity, Impl, OFFSET>,
            putref_AdminQueueInfo: putref_AdminQueueInfo::<Identity, Impl, OFFSET>,
            ResponseDestination: ResponseDestination::<Identity, Impl, OFFSET>,
            putref_ResponseDestination: putref_ResponseDestination::<Identity, Impl, OFFSET>,
            Destination: Destination::<Identity, Impl, OFFSET>,
            LookupId: LookupId::<Identity, Impl, OFFSET>,
            IsAuthenticated2: IsAuthenticated2::<Identity, Impl, OFFSET>,
            IsFirstInTransaction2: IsFirstInTransaction2::<Identity, Impl, OFFSET>,
            IsLastInTransaction2: IsLastInTransaction2::<Identity, Impl, OFFSET>,
            AttachCurrentSecurityContext2: AttachCurrentSecurityContext2::<Identity, Impl, OFFSET>,
            SoapEnvelope: SoapEnvelope::<Identity, Impl, OFFSET>,
            CompoundMessage: CompoundMessage::<Identity, Impl, OFFSET>,
            SetSoapHeader: SetSoapHeader::<Identity, Impl, OFFSET>,
            SetSoapBody: SetSoapBody::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQMessage3 as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQMessage4_Impl: Sized + super::Com::IDispatch_Impl {
    fn Class(&self) -> ::windows::core::Result<i32>;
    fn PrivLevel(&self) -> ::windows::core::Result<i32>;
    fn SetPrivLevel(&self, lprivlevel: i32) -> ::windows::core::Result<()>;
    fn AuthLevel(&self) -> ::windows::core::Result<i32>;
    fn SetAuthLevel(&self, lauthlevel: i32) -> ::windows::core::Result<()>;
    fn IsAuthenticated(&self) -> ::windows::core::Result<i16>;
    fn Delivery(&self) -> ::windows::core::Result<i32>;
    fn SetDelivery(&self, ldelivery: i32) -> ::windows::core::Result<()>;
    fn Trace(&self) -> ::windows::core::Result<i32>;
    fn SetTrace(&self, ltrace: i32) -> ::windows::core::Result<()>;
    fn Priority(&self) -> ::windows::core::Result<i32>;
    fn SetPriority(&self, lpriority: i32) -> ::windows::core::Result<()>;
    fn Journal(&self) -> ::windows::core::Result<i32>;
    fn SetJournal(&self, ljournal: i32) -> ::windows::core::Result<()>;
    fn ResponseQueueInfo_v1(&self) -> ::windows::core::Result<IMSMQQueueInfo>;
    fn putref_ResponseQueueInfo_v1(&self, pqinforesponse: &::core::option::Option<IMSMQQueueInfo>) -> ::windows::core::Result<()>;
    fn AppSpecific(&self) -> ::windows::core::Result<i32>;
    fn SetAppSpecific(&self, lappspecific: i32) -> ::windows::core::Result<()>;
    fn SourceMachineGuid(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn BodyLength(&self) -> ::windows::core::Result<i32>;
    fn Body(&self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SetBody(&self, varbody: &super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn AdminQueueInfo_v1(&self) -> ::windows::core::Result<IMSMQQueueInfo>;
    fn putref_AdminQueueInfo_v1(&self, pqinfoadmin: &::core::option::Option<IMSMQQueueInfo>) -> ::windows::core::Result<()>;
    fn Id(&self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn CorrelationId(&self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SetCorrelationId(&self, varmsgid: &super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Ack(&self) -> ::windows::core::Result<i32>;
    fn SetAck(&self, lack: i32) -> ::windows::core::Result<()>;
    fn Label(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetLabel(&self, bstrlabel: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn MaxTimeToReachQueue(&self) -> ::windows::core::Result<i32>;
    fn SetMaxTimeToReachQueue(&self, lmaxtimetoreachqueue: i32) -> ::windows::core::Result<()>;
    fn MaxTimeToReceive(&self) -> ::windows::core::Result<i32>;
    fn SetMaxTimeToReceive(&self, lmaxtimetoreceive: i32) -> ::windows::core::Result<()>;
    fn HashAlgorithm(&self) -> ::windows::core::Result<i32>;
    fn SetHashAlgorithm(&self, lhashalg: i32) -> ::windows::core::Result<()>;
    fn EncryptAlgorithm(&self) -> ::windows::core::Result<i32>;
    fn SetEncryptAlgorithm(&self, lencryptalg: i32) -> ::windows::core::Result<()>;
    fn SentTime(&self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn ArrivedTime(&self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn DestinationQueueInfo(&self) -> ::windows::core::Result<IMSMQQueueInfo4>;
    fn SenderCertificate(&self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SetSenderCertificate(&self, varsendercert: &super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn SenderId(&self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SenderIdType(&self) -> ::windows::core::Result<i32>;
    fn SetSenderIdType(&self, lsenderidtype: i32) -> ::windows::core::Result<()>;
    fn Send(&self, destinationqueue: &::core::option::Option<super::Com::IDispatch>, transaction: *const super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn AttachCurrentSecurityContext(&self) -> ::windows::core::Result<()>;
    fn SenderVersion(&self) -> ::windows::core::Result<i32>;
    fn Extension(&self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SetExtension(&self, varextension: &super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn ConnectorTypeGuid(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetConnectorTypeGuid(&self, bstrguidconnectortype: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn TransactionStatusQueueInfo(&self) -> ::windows::core::Result<IMSMQQueueInfo4>;
    fn DestinationSymmetricKey(&self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SetDestinationSymmetricKey(&self, vardestsymmkey: &super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Signature(&self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SetSignature(&self, varsignature: &super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn AuthenticationProviderType(&self) -> ::windows::core::Result<i32>;
    fn SetAuthenticationProviderType(&self, lauthprovtype: i32) -> ::windows::core::Result<()>;
    fn AuthenticationProviderName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetAuthenticationProviderName(&self, bstrauthprovname: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetSenderId(&self, varsenderid: &super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn MsgClass(&self) -> ::windows::core::Result<i32>;
    fn SetMsgClass(&self, lmsgclass: i32) -> ::windows::core::Result<()>;
    fn Properties(&self) -> ::windows::core::Result<super::Com::IDispatch>;
    fn TransactionId(&self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn IsFirstInTransaction(&self) -> ::windows::core::Result<i16>;
    fn IsLastInTransaction(&self) -> ::windows::core::Result<i16>;
    fn ResponseQueueInfo_v2(&self) -> ::windows::core::Result<IMSMQQueueInfo2>;
    fn putref_ResponseQueueInfo_v2(&self, pqinforesponse: &::core::option::Option<IMSMQQueueInfo2>) -> ::windows::core::Result<()>;
    fn AdminQueueInfo_v2(&self) -> ::windows::core::Result<IMSMQQueueInfo2>;
    fn putref_AdminQueueInfo_v2(&self, pqinfoadmin: &::core::option::Option<IMSMQQueueInfo2>) -> ::windows::core::Result<()>;
    fn ReceivedAuthenticationLevel(&self) -> ::windows::core::Result<i16>;
    fn ResponseQueueInfo(&self) -> ::windows::core::Result<IMSMQQueueInfo4>;
    fn putref_ResponseQueueInfo(&self, pqinforesponse: &::core::option::Option<IMSMQQueueInfo4>) -> ::windows::core::Result<()>;
    fn AdminQueueInfo(&self) -> ::windows::core::Result<IMSMQQueueInfo4>;
    fn putref_AdminQueueInfo(&self, pqinfoadmin: &::core::option::Option<IMSMQQueueInfo4>) -> ::windows::core::Result<()>;
    fn ResponseDestination(&self) -> ::windows::core::Result<super::Com::IDispatch>;
    fn putref_ResponseDestination(&self, pdestresponse: &::core::option::Option<super::Com::IDispatch>) -> ::windows::core::Result<()>;
    fn Destination(&self) -> ::windows::core::Result<super::Com::IDispatch>;
    fn LookupId(&self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn IsAuthenticated2(&self) -> ::windows::core::Result<i16>;
    fn IsFirstInTransaction2(&self) -> ::windows::core::Result<i16>;
    fn IsLastInTransaction2(&self) -> ::windows::core::Result<i16>;
    fn AttachCurrentSecurityContext2(&self) -> ::windows::core::Result<()>;
    fn SoapEnvelope(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn CompoundMessage(&self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SetSoapHeader(&self, bstrsoapheader: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetSoapBody(&self, bstrsoapbody: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQMessage4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage4_Impl, const OFFSET: isize>() -> IMSMQMessage4_Vtbl {
        unsafe extern "system" fn Class<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plclass: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Class() {
                ::core::result::Result::Ok(ok__) => {
                    *plclass = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrivLevel<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plprivlevel: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PrivLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *plprivlevel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrivLevel<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lprivlevel: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPrivLevel(::core::mem::transmute_copy(&lprivlevel)).into()
        }
        unsafe extern "system" fn AuthLevel<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plauthlevel: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AuthLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *plauthlevel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAuthLevel<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lauthlevel: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAuthLevel(::core::mem::transmute_copy(&lauthlevel)).into()
        }
        unsafe extern "system" fn IsAuthenticated<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisauthenticated: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsAuthenticated() {
                ::core::result::Result::Ok(ok__) => {
                    *pisauthenticated = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delivery<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pldelivery: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Delivery() {
                ::core::result::Result::Ok(ok__) => {
                    *pldelivery = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDelivery<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ldelivery: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDelivery(::core::mem::transmute_copy(&ldelivery)).into()
        }
        unsafe extern "system" fn Trace<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pltrace: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Trace() {
                ::core::result::Result::Ok(ok__) => {
                    *pltrace = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTrace<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ltrace: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetTrace(::core::mem::transmute_copy(&ltrace)).into()
        }
        unsafe extern "system" fn Priority<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plpriority: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Priority() {
                ::core::result::Result::Ok(ok__) => {
                    *plpriority = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPriority<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpriority: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPriority(::core::mem::transmute_copy(&lpriority)).into()
        }
        unsafe extern "system" fn Journal<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pljournal: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Journal() {
                ::core::result::Result::Ok(ok__) => {
                    *pljournal = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetJournal<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ljournal: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetJournal(::core::mem::transmute_copy(&ljournal)).into()
        }
        unsafe extern "system" fn ResponseQueueInfo_v1<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinforesponse: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ResponseQueueInfo_v1() {
                ::core::result::Result::Ok(ok__) => {
                    *ppqinforesponse = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_ResponseQueueInfo_v1<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqinforesponse: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).putref_ResponseQueueInfo_v1(::core::mem::transmute(&pqinforesponse)).into()
        }
        unsafe extern "system" fn AppSpecific<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plappspecific: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AppSpecific() {
                ::core::result::Result::Ok(ok__) => {
                    *plappspecific = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAppSpecific<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lappspecific: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAppSpecific(::core::mem::transmute_copy(&lappspecific)).into()
        }
        unsafe extern "system" fn SourceMachineGuid<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrguidsrcmachine: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SourceMachineGuid() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrguidsrcmachine = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BodyLength<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcbbody: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).BodyLength() {
                ::core::result::Result::Ok(ok__) => {
                    *pcbbody = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Body<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarbody: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Body() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarbody = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBody<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varbody: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetBody(::core::mem::transmute(&varbody)).into()
        }
        unsafe extern "system" fn AdminQueueInfo_v1<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinfoadmin: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AdminQueueInfo_v1() {
                ::core::result::Result::Ok(ok__) => {
                    *ppqinfoadmin = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_AdminQueueInfo_v1<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqinfoadmin: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).putref_AdminQueueInfo_v1(::core::mem::transmute(&pqinfoadmin)).into()
        }
        unsafe extern "system" fn Id<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarmsgid: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarmsgid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CorrelationId<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarmsgid: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CorrelationId() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarmsgid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCorrelationId<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varmsgid: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCorrelationId(::core::mem::transmute(&varmsgid)).into()
        }
        unsafe extern "system" fn Ack<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plack: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Ack() {
                ::core::result::Result::Ok(ok__) => {
                    *plack = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAck<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lack: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAck(::core::mem::transmute_copy(&lack)).into()
        }
        unsafe extern "system" fn Label<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrlabel: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Label() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrlabel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLabel<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrlabel: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetLabel(::core::mem::transmute(&bstrlabel)).into()
        }
        unsafe extern "system" fn MaxTimeToReachQueue<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmaxtimetoreachqueue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MaxTimeToReachQueue() {
                ::core::result::Result::Ok(ok__) => {
                    *plmaxtimetoreachqueue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxTimeToReachQueue<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmaxtimetoreachqueue: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetMaxTimeToReachQueue(::core::mem::transmute_copy(&lmaxtimetoreachqueue)).into()
        }
        unsafe extern "system" fn MaxTimeToReceive<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmaxtimetoreceive: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MaxTimeToReceive() {
                ::core::result::Result::Ok(ok__) => {
                    *plmaxtimetoreceive = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxTimeToReceive<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmaxtimetoreceive: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetMaxTimeToReceive(::core::mem::transmute_copy(&lmaxtimetoreceive)).into()
        }
        unsafe extern "system" fn HashAlgorithm<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plhashalg: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).HashAlgorithm() {
                ::core::result::Result::Ok(ok__) => {
                    *plhashalg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHashAlgorithm<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lhashalg: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetHashAlgorithm(::core::mem::transmute_copy(&lhashalg)).into()
        }
        unsafe extern "system" fn EncryptAlgorithm<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plencryptalg: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EncryptAlgorithm() {
                ::core::result::Result::Ok(ok__) => {
                    *plencryptalg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEncryptAlgorithm<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lencryptalg: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetEncryptAlgorithm(::core::mem::transmute_copy(&lencryptalg)).into()
        }
        unsafe extern "system" fn SentTime<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarsenttime: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SentTime() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarsenttime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ArrivedTime<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plarrivedtime: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ArrivedTime() {
                ::core::result::Result::Ok(ok__) => {
                    *plarrivedtime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DestinationQueueInfo<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinfodest: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DestinationQueueInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *ppqinfodest = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SenderCertificate<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarsendercert: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SenderCertificate() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarsendercert = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSenderCertificate<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varsendercert: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSenderCertificate(::core::mem::transmute(&varsendercert)).into()
        }
        unsafe extern "system" fn SenderId<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarsenderid: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SenderId() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarsenderid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SenderIdType<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plsenderidtype: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SenderIdType() {
                ::core::result::Result::Ok(ok__) => {
                    *plsenderidtype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSenderIdType<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lsenderidtype: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSenderIdType(::core::mem::transmute_copy(&lsenderidtype)).into()
        }
        unsafe extern "system" fn Send<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, destinationqueue: ::windows::core::RawPtr, transaction: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Send(::core::mem::transmute(&destinationqueue), ::core::mem::transmute_copy(&transaction)).into()
        }
        unsafe extern "system" fn AttachCurrentSecurityContext<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AttachCurrentSecurityContext().into()
        }
        unsafe extern "system" fn SenderVersion<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plsenderversion: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SenderVersion() {
                ::core::result::Result::Ok(ok__) => {
                    *plsenderversion = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Extension<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarextension: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Extension() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarextension = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExtension<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varextension: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetExtension(::core::mem::transmute(&varextension)).into()
        }
        unsafe extern "system" fn ConnectorTypeGuid<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrguidconnectortype: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ConnectorTypeGuid() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrguidconnectortype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetConnectorTypeGuid<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrguidconnectortype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetConnectorTypeGuid(::core::mem::transmute(&bstrguidconnectortype)).into()
        }
        unsafe extern "system" fn TransactionStatusQueueInfo<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinfoxactstatus: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).TransactionStatusQueueInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *ppqinfoxactstatus = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DestinationSymmetricKey<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvardestsymmkey: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DestinationSymmetricKey() {
                ::core::result::Result::Ok(ok__) => {
                    *pvardestsymmkey = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDestinationSymmetricKey<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vardestsymmkey: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDestinationSymmetricKey(::core::mem::transmute(&vardestsymmkey)).into()
        }
        unsafe extern "system" fn Signature<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarsignature: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Signature() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarsignature = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSignature<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varsignature: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSignature(::core::mem::transmute(&varsignature)).into()
        }
        unsafe extern "system" fn AuthenticationProviderType<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plauthprovtype: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AuthenticationProviderType() {
                ::core::result::Result::Ok(ok__) => {
                    *plauthprovtype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAuthenticationProviderType<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lauthprovtype: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAuthenticationProviderType(::core::mem::transmute_copy(&lauthprovtype)).into()
        }
        unsafe extern "system" fn AuthenticationProviderName<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrauthprovname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AuthenticationProviderName() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrauthprovname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAuthenticationProviderName<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrauthprovname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAuthenticationProviderName(::core::mem::transmute(&bstrauthprovname)).into()
        }
        unsafe extern "system" fn SetSenderId<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varsenderid: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSenderId(::core::mem::transmute(&varsenderid)).into()
        }
        unsafe extern "system" fn MsgClass<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmsgclass: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MsgClass() {
                ::core::result::Result::Ok(ok__) => {
                    *plmsgclass = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMsgClass<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmsgclass: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetMsgClass(::core::mem::transmute_copy(&lmsgclass)).into()
        }
        unsafe extern "system" fn Properties<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcolproperties = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransactionId<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarxactid: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).TransactionId() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarxactid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsFirstInTransaction<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisfirstinxact: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsFirstInTransaction() {
                ::core::result::Result::Ok(ok__) => {
                    *pisfirstinxact = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsLastInTransaction<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pislastinxact: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsLastInTransaction() {
                ::core::result::Result::Ok(ok__) => {
                    *pislastinxact = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResponseQueueInfo_v2<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinforesponse: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ResponseQueueInfo_v2() {
                ::core::result::Result::Ok(ok__) => {
                    *ppqinforesponse = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_ResponseQueueInfo_v2<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqinforesponse: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).putref_ResponseQueueInfo_v2(::core::mem::transmute(&pqinforesponse)).into()
        }
        unsafe extern "system" fn AdminQueueInfo_v2<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinfoadmin: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AdminQueueInfo_v2() {
                ::core::result::Result::Ok(ok__) => {
                    *ppqinfoadmin = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_AdminQueueInfo_v2<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqinfoadmin: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).putref_AdminQueueInfo_v2(::core::mem::transmute(&pqinfoadmin)).into()
        }
        unsafe extern "system" fn ReceivedAuthenticationLevel<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psreceivedauthenticationlevel: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ReceivedAuthenticationLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *psreceivedauthenticationlevel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResponseQueueInfo<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinforesponse: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ResponseQueueInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *ppqinforesponse = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_ResponseQueueInfo<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqinforesponse: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).putref_ResponseQueueInfo(::core::mem::transmute(&pqinforesponse)).into()
        }
        unsafe extern "system" fn AdminQueueInfo<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinfoadmin: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AdminQueueInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *ppqinfoadmin = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_AdminQueueInfo<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqinfoadmin: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).putref_AdminQueueInfo(::core::mem::transmute(&pqinfoadmin)).into()
        }
        unsafe extern "system" fn ResponseDestination<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdestresponse: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ResponseDestination() {
                ::core::result::Result::Ok(ok__) => {
                    *ppdestresponse = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_ResponseDestination<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdestresponse: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).putref_ResponseDestination(::core::mem::transmute(&pdestresponse)).into()
        }
        unsafe extern "system" fn Destination<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdestdestination: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Destination() {
                ::core::result::Result::Ok(ok__) => {
                    *ppdestdestination = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LookupId<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarlookupid: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LookupId() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarlookupid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsAuthenticated2<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisauthenticated: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsAuthenticated2() {
                ::core::result::Result::Ok(ok__) => {
                    *pisauthenticated = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsFirstInTransaction2<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisfirstinxact: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsFirstInTransaction2() {
                ::core::result::Result::Ok(ok__) => {
                    *pisfirstinxact = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsLastInTransaction2<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pislastinxact: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsLastInTransaction2() {
                ::core::result::Result::Ok(ok__) => {
                    *pislastinxact = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AttachCurrentSecurityContext2<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AttachCurrentSecurityContext2().into()
        }
        unsafe extern "system" fn SoapEnvelope<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrsoapenvelope: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SoapEnvelope() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrsoapenvelope = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CompoundMessage<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarcompoundmessage: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CompoundMessage() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarcompoundmessage = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSoapHeader<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsoapheader: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSoapHeader(::core::mem::transmute(&bstrsoapheader)).into()
        }
        unsafe extern "system" fn SetSoapBody<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsoapbody: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSoapBody(::core::mem::transmute(&bstrsoapbody)).into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Class: Class::<Identity, Impl, OFFSET>,
            PrivLevel: PrivLevel::<Identity, Impl, OFFSET>,
            SetPrivLevel: SetPrivLevel::<Identity, Impl, OFFSET>,
            AuthLevel: AuthLevel::<Identity, Impl, OFFSET>,
            SetAuthLevel: SetAuthLevel::<Identity, Impl, OFFSET>,
            IsAuthenticated: IsAuthenticated::<Identity, Impl, OFFSET>,
            Delivery: Delivery::<Identity, Impl, OFFSET>,
            SetDelivery: SetDelivery::<Identity, Impl, OFFSET>,
            Trace: Trace::<Identity, Impl, OFFSET>,
            SetTrace: SetTrace::<Identity, Impl, OFFSET>,
            Priority: Priority::<Identity, Impl, OFFSET>,
            SetPriority: SetPriority::<Identity, Impl, OFFSET>,
            Journal: Journal::<Identity, Impl, OFFSET>,
            SetJournal: SetJournal::<Identity, Impl, OFFSET>,
            ResponseQueueInfo_v1: ResponseQueueInfo_v1::<Identity, Impl, OFFSET>,
            putref_ResponseQueueInfo_v1: putref_ResponseQueueInfo_v1::<Identity, Impl, OFFSET>,
            AppSpecific: AppSpecific::<Identity, Impl, OFFSET>,
            SetAppSpecific: SetAppSpecific::<Identity, Impl, OFFSET>,
            SourceMachineGuid: SourceMachineGuid::<Identity, Impl, OFFSET>,
            BodyLength: BodyLength::<Identity, Impl, OFFSET>,
            Body: Body::<Identity, Impl, OFFSET>,
            SetBody: SetBody::<Identity, Impl, OFFSET>,
            AdminQueueInfo_v1: AdminQueueInfo_v1::<Identity, Impl, OFFSET>,
            putref_AdminQueueInfo_v1: putref_AdminQueueInfo_v1::<Identity, Impl, OFFSET>,
            Id: Id::<Identity, Impl, OFFSET>,
            CorrelationId: CorrelationId::<Identity, Impl, OFFSET>,
            SetCorrelationId: SetCorrelationId::<Identity, Impl, OFFSET>,
            Ack: Ack::<Identity, Impl, OFFSET>,
            SetAck: SetAck::<Identity, Impl, OFFSET>,
            Label: Label::<Identity, Impl, OFFSET>,
            SetLabel: SetLabel::<Identity, Impl, OFFSET>,
            MaxTimeToReachQueue: MaxTimeToReachQueue::<Identity, Impl, OFFSET>,
            SetMaxTimeToReachQueue: SetMaxTimeToReachQueue::<Identity, Impl, OFFSET>,
            MaxTimeToReceive: MaxTimeToReceive::<Identity, Impl, OFFSET>,
            SetMaxTimeToReceive: SetMaxTimeToReceive::<Identity, Impl, OFFSET>,
            HashAlgorithm: HashAlgorithm::<Identity, Impl, OFFSET>,
            SetHashAlgorithm: SetHashAlgorithm::<Identity, Impl, OFFSET>,
            EncryptAlgorithm: EncryptAlgorithm::<Identity, Impl, OFFSET>,
            SetEncryptAlgorithm: SetEncryptAlgorithm::<Identity, Impl, OFFSET>,
            SentTime: SentTime::<Identity, Impl, OFFSET>,
            ArrivedTime: ArrivedTime::<Identity, Impl, OFFSET>,
            DestinationQueueInfo: DestinationQueueInfo::<Identity, Impl, OFFSET>,
            SenderCertificate: SenderCertificate::<Identity, Impl, OFFSET>,
            SetSenderCertificate: SetSenderCertificate::<Identity, Impl, OFFSET>,
            SenderId: SenderId::<Identity, Impl, OFFSET>,
            SenderIdType: SenderIdType::<Identity, Impl, OFFSET>,
            SetSenderIdType: SetSenderIdType::<Identity, Impl, OFFSET>,
            Send: Send::<Identity, Impl, OFFSET>,
            AttachCurrentSecurityContext: AttachCurrentSecurityContext::<Identity, Impl, OFFSET>,
            SenderVersion: SenderVersion::<Identity, Impl, OFFSET>,
            Extension: Extension::<Identity, Impl, OFFSET>,
            SetExtension: SetExtension::<Identity, Impl, OFFSET>,
            ConnectorTypeGuid: ConnectorTypeGuid::<Identity, Impl, OFFSET>,
            SetConnectorTypeGuid: SetConnectorTypeGuid::<Identity, Impl, OFFSET>,
            TransactionStatusQueueInfo: TransactionStatusQueueInfo::<Identity, Impl, OFFSET>,
            DestinationSymmetricKey: DestinationSymmetricKey::<Identity, Impl, OFFSET>,
            SetDestinationSymmetricKey: SetDestinationSymmetricKey::<Identity, Impl, OFFSET>,
            Signature: Signature::<Identity, Impl, OFFSET>,
            SetSignature: SetSignature::<Identity, Impl, OFFSET>,
            AuthenticationProviderType: AuthenticationProviderType::<Identity, Impl, OFFSET>,
            SetAuthenticationProviderType: SetAuthenticationProviderType::<Identity, Impl, OFFSET>,
            AuthenticationProviderName: AuthenticationProviderName::<Identity, Impl, OFFSET>,
            SetAuthenticationProviderName: SetAuthenticationProviderName::<Identity, Impl, OFFSET>,
            SetSenderId: SetSenderId::<Identity, Impl, OFFSET>,
            MsgClass: MsgClass::<Identity, Impl, OFFSET>,
            SetMsgClass: SetMsgClass::<Identity, Impl, OFFSET>,
            Properties: Properties::<Identity, Impl, OFFSET>,
            TransactionId: TransactionId::<Identity, Impl, OFFSET>,
            IsFirstInTransaction: IsFirstInTransaction::<Identity, Impl, OFFSET>,
            IsLastInTransaction: IsLastInTransaction::<Identity, Impl, OFFSET>,
            ResponseQueueInfo_v2: ResponseQueueInfo_v2::<Identity, Impl, OFFSET>,
            putref_ResponseQueueInfo_v2: putref_ResponseQueueInfo_v2::<Identity, Impl, OFFSET>,
            AdminQueueInfo_v2: AdminQueueInfo_v2::<Identity, Impl, OFFSET>,
            putref_AdminQueueInfo_v2: putref_AdminQueueInfo_v2::<Identity, Impl, OFFSET>,
            ReceivedAuthenticationLevel: ReceivedAuthenticationLevel::<Identity, Impl, OFFSET>,
            ResponseQueueInfo: ResponseQueueInfo::<Identity, Impl, OFFSET>,
            putref_ResponseQueueInfo: putref_ResponseQueueInfo::<Identity, Impl, OFFSET>,
            AdminQueueInfo: AdminQueueInfo::<Identity, Impl, OFFSET>,
            putref_AdminQueueInfo: putref_AdminQueueInfo::<Identity, Impl, OFFSET>,
            ResponseDestination: ResponseDestination::<Identity, Impl, OFFSET>,
            putref_ResponseDestination: putref_ResponseDestination::<Identity, Impl, OFFSET>,
            Destination: Destination::<Identity, Impl, OFFSET>,
            LookupId: LookupId::<Identity, Impl, OFFSET>,
            IsAuthenticated2: IsAuthenticated2::<Identity, Impl, OFFSET>,
            IsFirstInTransaction2: IsFirstInTransaction2::<Identity, Impl, OFFSET>,
            IsLastInTransaction2: IsLastInTransaction2::<Identity, Impl, OFFSET>,
            AttachCurrentSecurityContext2: AttachCurrentSecurityContext2::<Identity, Impl, OFFSET>,
            SoapEnvelope: SoapEnvelope::<Identity, Impl, OFFSET>,
            CompoundMessage: CompoundMessage::<Identity, Impl, OFFSET>,
            SetSoapHeader: SetSoapHeader::<Identity, Impl, OFFSET>,
            SetSoapBody: SetSoapBody::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQMessage4 as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQOutgoingQueueManagement_Impl: Sized + super::Com::IDispatch_Impl + IMSMQManagement_Impl {
    fn State(&self) -> ::windows::core::Result<i32>;
    fn NextHops(&self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn EodGetSendInfo(&self) -> ::windows::core::Result<IMSMQCollection>;
    fn Resume(&self) -> ::windows::core::Result<()>;
    fn Pause(&self) -> ::windows::core::Result<()>;
    fn EodResend(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQOutgoingQueueManagement_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQOutgoingQueueManagement_Impl, const OFFSET: isize>() -> IMSMQOutgoingQueueManagement_Vtbl {
        unsafe extern "system" fn State<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQOutgoingQueueManagement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plstate: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).State() {
                ::core::result::Result::Ok(ok__) => {
                    *plstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NextHops<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQOutgoingQueueManagement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvnexthops: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).NextHops() {
                ::core::result::Result::Ok(ok__) => {
                    *pvnexthops = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EodGetSendInfo<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQOutgoingQueueManagement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EodGetSendInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcollection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Resume<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQOutgoingQueueManagement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Resume().into()
        }
        unsafe extern "system" fn Pause<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQOutgoingQueueManagement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Pause().into()
        }
        unsafe extern "system" fn EodResend<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQOutgoingQueueManagement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EodResend().into()
        }
        Self {
            base: IMSMQManagement_Vtbl::new::<Identity, Impl, OFFSET>(),
            State: State::<Identity, Impl, OFFSET>,
            NextHops: NextHops::<Identity, Impl, OFFSET>,
            EodGetSendInfo: EodGetSendInfo::<Identity, Impl, OFFSET>,
            Resume: Resume::<Identity, Impl, OFFSET>,
            Pause: Pause::<Identity, Impl, OFFSET>,
            EodResend: EodResend::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQOutgoingQueueManagement as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IMSMQManagement as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQPrivateDestination_Impl: Sized + super::Com::IDispatch_Impl {
    fn Handle(&self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SetHandle(&self, varhandle: &super::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQPrivateDestination_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQPrivateDestination_Impl, const OFFSET: isize>() -> IMSMQPrivateDestination_Vtbl {
        unsafe extern "system" fn Handle<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQPrivateDestination_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarhandle: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Handle() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarhandle = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHandle<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQPrivateDestination_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varhandle: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetHandle(::core::mem::transmute(&varhandle)).into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Handle: Handle::<Identity, Impl, OFFSET>,
            SetHandle: SetHandle::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQPrivateDestination as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQPrivateEvent_Impl: Sized + super::Com::IDispatch_Impl {
    fn Hwnd(&self) -> ::windows::core::Result<i32>;
    fn FireArrivedEvent(&self, pq: &::core::option::Option<IMSMQQueue>, msgcursor: i32) -> ::windows::core::Result<()>;
    fn FireArrivedErrorEvent(&self, pq: &::core::option::Option<IMSMQQueue>, hrstatus: ::windows::core::HRESULT, msgcursor: i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQPrivateEvent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQPrivateEvent_Impl, const OFFSET: isize>() -> IMSMQPrivateEvent_Vtbl {
        unsafe extern "system" fn Hwnd<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQPrivateEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phwnd: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Hwnd() {
                ::core::result::Result::Ok(ok__) => {
                    *phwnd = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FireArrivedEvent<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQPrivateEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pq: ::windows::core::RawPtr, msgcursor: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).FireArrivedEvent(::core::mem::transmute(&pq), ::core::mem::transmute_copy(&msgcursor)).into()
        }
        unsafe extern "system" fn FireArrivedErrorEvent<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQPrivateEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pq: ::windows::core::RawPtr, hrstatus: ::windows::core::HRESULT, msgcursor: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).FireArrivedErrorEvent(::core::mem::transmute(&pq), ::core::mem::transmute_copy(&hrstatus), ::core::mem::transmute_copy(&msgcursor)).into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Hwnd: Hwnd::<Identity, Impl, OFFSET>,
            FireArrivedEvent: FireArrivedEvent::<Identity, Impl, OFFSET>,
            FireArrivedErrorEvent: FireArrivedErrorEvent::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQPrivateEvent as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQQuery_Impl: Sized + super::Com::IDispatch_Impl {
    fn LookupQueue(&self, queueguid: *const super::Com::VARIANT, servicetypeguid: *const super::Com::VARIANT, label: *const super::Com::VARIANT, createtime: *const super::Com::VARIANT, modifytime: *const super::Com::VARIANT, relservicetype: *const super::Com::VARIANT, rellabel: *const super::Com::VARIANT, relcreatetime: *const super::Com::VARIANT, relmodifytime: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQQueueInfos>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQQuery_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQuery_Impl, const OFFSET: isize>() -> IMSMQQuery_Vtbl {
        unsafe extern "system" fn LookupQueue<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQuery_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, queueguid: *const super::Com::VARIANT, servicetypeguid: *const super::Com::VARIANT, label: *const super::Com::VARIANT, createtime: *const super::Com::VARIANT, modifytime: *const super::Com::VARIANT, relservicetype: *const super::Com::VARIANT, rellabel: *const super::Com::VARIANT, relcreatetime: *const super::Com::VARIANT, relmodifytime: *const super::Com::VARIANT, ppqinfos: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LookupQueue(::core::mem::transmute_copy(&queueguid), ::core::mem::transmute_copy(&servicetypeguid), ::core::mem::transmute_copy(&label), ::core::mem::transmute_copy(&createtime), ::core::mem::transmute_copy(&modifytime), ::core::mem::transmute_copy(&relservicetype), ::core::mem::transmute_copy(&rellabel), ::core::mem::transmute_copy(&relcreatetime), ::core::mem::transmute_copy(&relmodifytime)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppqinfos = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(), LookupQueue: LookupQueue::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQQuery as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQQuery2_Impl: Sized + super::Com::IDispatch_Impl {
    fn LookupQueue(&self, queueguid: *const super::Com::VARIANT, servicetypeguid: *const super::Com::VARIANT, label: *const super::Com::VARIANT, createtime: *const super::Com::VARIANT, modifytime: *const super::Com::VARIANT, relservicetype: *const super::Com::VARIANT, rellabel: *const super::Com::VARIANT, relcreatetime: *const super::Com::VARIANT, relmodifytime: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQQueueInfos2>;
    fn Properties(&self) -> ::windows::core::Result<super::Com::IDispatch>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQQuery2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQuery2_Impl, const OFFSET: isize>() -> IMSMQQuery2_Vtbl {
        unsafe extern "system" fn LookupQueue<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQuery2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, queueguid: *const super::Com::VARIANT, servicetypeguid: *const super::Com::VARIANT, label: *const super::Com::VARIANT, createtime: *const super::Com::VARIANT, modifytime: *const super::Com::VARIANT, relservicetype: *const super::Com::VARIANT, rellabel: *const super::Com::VARIANT, relcreatetime: *const super::Com::VARIANT, relmodifytime: *const super::Com::VARIANT, ppqinfos: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LookupQueue(::core::mem::transmute_copy(&queueguid), ::core::mem::transmute_copy(&servicetypeguid), ::core::mem::transmute_copy(&label), ::core::mem::transmute_copy(&createtime), ::core::mem::transmute_copy(&modifytime), ::core::mem::transmute_copy(&relservicetype), ::core::mem::transmute_copy(&rellabel), ::core::mem::transmute_copy(&relcreatetime), ::core::mem::transmute_copy(&relmodifytime)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppqinfos = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQuery2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcolproperties = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            LookupQueue: LookupQueue::<Identity, Impl, OFFSET>,
            Properties: Properties::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQQuery2 as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQQuery3_Impl: Sized + super::Com::IDispatch_Impl {
    fn LookupQueue_v2(&self, queueguid: *const super::Com::VARIANT, servicetypeguid: *const super::Com::VARIANT, label: *const super::Com::VARIANT, createtime: *const super::Com::VARIANT, modifytime: *const super::Com::VARIANT, relservicetype: *const super::Com::VARIANT, rellabel: *const super::Com::VARIANT, relcreatetime: *const super::Com::VARIANT, relmodifytime: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQQueueInfos3>;
    fn Properties(&self) -> ::windows::core::Result<super::Com::IDispatch>;
    fn LookupQueue(&self, queueguid: *const super::Com::VARIANT, servicetypeguid: *const super::Com::VARIANT, label: *const super::Com::VARIANT, createtime: *const super::Com::VARIANT, modifytime: *const super::Com::VARIANT, relservicetype: *const super::Com::VARIANT, rellabel: *const super::Com::VARIANT, relcreatetime: *const super::Com::VARIANT, relmodifytime: *const super::Com::VARIANT, multicastaddress: *const super::Com::VARIANT, relmulticastaddress: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQQueueInfos3>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQQuery3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQuery3_Impl, const OFFSET: isize>() -> IMSMQQuery3_Vtbl {
        unsafe extern "system" fn LookupQueue_v2<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQuery3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, queueguid: *const super::Com::VARIANT, servicetypeguid: *const super::Com::VARIANT, label: *const super::Com::VARIANT, createtime: *const super::Com::VARIANT, modifytime: *const super::Com::VARIANT, relservicetype: *const super::Com::VARIANT, rellabel: *const super::Com::VARIANT, relcreatetime: *const super::Com::VARIANT, relmodifytime: *const super::Com::VARIANT, ppqinfos: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LookupQueue_v2(::core::mem::transmute_copy(&queueguid), ::core::mem::transmute_copy(&servicetypeguid), ::core::mem::transmute_copy(&label), ::core::mem::transmute_copy(&createtime), ::core::mem::transmute_copy(&modifytime), ::core::mem::transmute_copy(&relservicetype), ::core::mem::transmute_copy(&rellabel), ::core::mem::transmute_copy(&relcreatetime), ::core::mem::transmute_copy(&relmodifytime)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppqinfos = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQuery3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcolproperties = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LookupQueue<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQuery3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, queueguid: *const super::Com::VARIANT, servicetypeguid: *const super::Com::VARIANT, label: *const super::Com::VARIANT, createtime: *const super::Com::VARIANT, modifytime: *const super::Com::VARIANT, relservicetype: *const super::Com::VARIANT, rellabel: *const super::Com::VARIANT, relcreatetime: *const super::Com::VARIANT, relmodifytime: *const super::Com::VARIANT, multicastaddress: *const super::Com::VARIANT, relmulticastaddress: *const super::Com::VARIANT, ppqinfos: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
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
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            LookupQueue_v2: LookupQueue_v2::<Identity, Impl, OFFSET>,
            Properties: Properties::<Identity, Impl, OFFSET>,
            LookupQueue: LookupQueue::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQQuery3 as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQQuery4_Impl: Sized + super::Com::IDispatch_Impl {
    fn LookupQueue_v2(&self, queueguid: *const super::Com::VARIANT, servicetypeguid: *const super::Com::VARIANT, label: *const super::Com::VARIANT, createtime: *const super::Com::VARIANT, modifytime: *const super::Com::VARIANT, relservicetype: *const super::Com::VARIANT, rellabel: *const super::Com::VARIANT, relcreatetime: *const super::Com::VARIANT, relmodifytime: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQQueueInfos4>;
    fn Properties(&self) -> ::windows::core::Result<super::Com::IDispatch>;
    fn LookupQueue(&self, queueguid: *const super::Com::VARIANT, servicetypeguid: *const super::Com::VARIANT, label: *const super::Com::VARIANT, createtime: *const super::Com::VARIANT, modifytime: *const super::Com::VARIANT, relservicetype: *const super::Com::VARIANT, rellabel: *const super::Com::VARIANT, relcreatetime: *const super::Com::VARIANT, relmodifytime: *const super::Com::VARIANT, multicastaddress: *const super::Com::VARIANT, relmulticastaddress: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQQueueInfos4>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQQuery4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQuery4_Impl, const OFFSET: isize>() -> IMSMQQuery4_Vtbl {
        unsafe extern "system" fn LookupQueue_v2<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQuery4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, queueguid: *const super::Com::VARIANT, servicetypeguid: *const super::Com::VARIANT, label: *const super::Com::VARIANT, createtime: *const super::Com::VARIANT, modifytime: *const super::Com::VARIANT, relservicetype: *const super::Com::VARIANT, rellabel: *const super::Com::VARIANT, relcreatetime: *const super::Com::VARIANT, relmodifytime: *const super::Com::VARIANT, ppqinfos: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LookupQueue_v2(::core::mem::transmute_copy(&queueguid), ::core::mem::transmute_copy(&servicetypeguid), ::core::mem::transmute_copy(&label), ::core::mem::transmute_copy(&createtime), ::core::mem::transmute_copy(&modifytime), ::core::mem::transmute_copy(&relservicetype), ::core::mem::transmute_copy(&rellabel), ::core::mem::transmute_copy(&relcreatetime), ::core::mem::transmute_copy(&relmodifytime)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppqinfos = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQuery4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcolproperties = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LookupQueue<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQuery4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, queueguid: *const super::Com::VARIANT, servicetypeguid: *const super::Com::VARIANT, label: *const super::Com::VARIANT, createtime: *const super::Com::VARIANT, modifytime: *const super::Com::VARIANT, relservicetype: *const super::Com::VARIANT, rellabel: *const super::Com::VARIANT, relcreatetime: *const super::Com::VARIANT, relmodifytime: *const super::Com::VARIANT, multicastaddress: *const super::Com::VARIANT, relmulticastaddress: *const super::Com::VARIANT, ppqinfos: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
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
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            LookupQueue_v2: LookupQueue_v2::<Identity, Impl, OFFSET>,
            Properties: Properties::<Identity, Impl, OFFSET>,
            LookupQueue: LookupQueue::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQQuery4 as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQQueue_Impl: Sized + super::Com::IDispatch_Impl {
    fn Access(&self) -> ::windows::core::Result<i32>;
    fn ShareMode(&self) -> ::windows::core::Result<i32>;
    fn QueueInfo(&self) -> ::windows::core::Result<IMSMQQueueInfo>;
    fn Handle(&self) -> ::windows::core::Result<i32>;
    fn IsOpen(&self) -> ::windows::core::Result<i16>;
    fn Close(&self) -> ::windows::core::Result<()>;
    fn Receive(&self, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage>;
    fn Peek(&self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage>;
    fn EnableNotification(&self, event: &::core::option::Option<IMSMQEvent>, cursor: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Reset(&self) -> ::windows::core::Result<()>;
    fn ReceiveCurrent(&self, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage>;
    fn PeekNext(&self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage>;
    fn PeekCurrent(&self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQQueue_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueue_Impl, const OFFSET: isize>() -> IMSMQQueue_Vtbl {
        unsafe extern "system" fn Access<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, placcess: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Access() {
                ::core::result::Result::Ok(ok__) => {
                    *placcess = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShareMode<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plsharemode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ShareMode() {
                ::core::result::Result::Ok(ok__) => {
                    *plsharemode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueueInfo<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).QueueInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *ppqinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Handle<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plhandle: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Handle() {
                ::core::result::Result::Ok(ok__) => {
                    *plhandle = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsOpen<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisopen: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsOpen() {
                ::core::result::Result::Ok(ok__) => {
                    *pisopen = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Close<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Close().into()
        }
        unsafe extern "system" fn Receive<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Receive(::core::mem::transmute_copy(&transaction), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Peek<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Peek(::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnableNotification<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, event: ::windows::core::RawPtr, cursor: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EnableNotification(::core::mem::transmute(&event), ::core::mem::transmute_copy(&cursor), ::core::mem::transmute_copy(&receivetimeout)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn ReceiveCurrent<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ReceiveCurrent(::core::mem::transmute_copy(&transaction), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeekNext<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PeekNext(::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeekCurrent<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PeekCurrent(::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Access: Access::<Identity, Impl, OFFSET>,
            ShareMode: ShareMode::<Identity, Impl, OFFSET>,
            QueueInfo: QueueInfo::<Identity, Impl, OFFSET>,
            Handle: Handle::<Identity, Impl, OFFSET>,
            IsOpen: IsOpen::<Identity, Impl, OFFSET>,
            Close: Close::<Identity, Impl, OFFSET>,
            Receive: Receive::<Identity, Impl, OFFSET>,
            Peek: Peek::<Identity, Impl, OFFSET>,
            EnableNotification: EnableNotification::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            ReceiveCurrent: ReceiveCurrent::<Identity, Impl, OFFSET>,
            PeekNext: PeekNext::<Identity, Impl, OFFSET>,
            PeekCurrent: PeekCurrent::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQQueue as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQQueue2_Impl: Sized + super::Com::IDispatch_Impl {
    fn Access(&self) -> ::windows::core::Result<i32>;
    fn ShareMode(&self) -> ::windows::core::Result<i32>;
    fn QueueInfo(&self) -> ::windows::core::Result<IMSMQQueueInfo2>;
    fn Handle(&self) -> ::windows::core::Result<i32>;
    fn IsOpen(&self) -> ::windows::core::Result<i16>;
    fn Close(&self) -> ::windows::core::Result<()>;
    fn Receive_v1(&self, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage>;
    fn Peek_v1(&self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage>;
    fn EnableNotification(&self, event: &::core::option::Option<IMSMQEvent2>, cursor: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Reset(&self) -> ::windows::core::Result<()>;
    fn ReceiveCurrent_v1(&self, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage>;
    fn PeekNext_v1(&self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage>;
    fn PeekCurrent_v1(&self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage>;
    fn Receive(&self, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage2>;
    fn Peek(&self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage2>;
    fn ReceiveCurrent(&self, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage2>;
    fn PeekNext(&self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage2>;
    fn PeekCurrent(&self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage2>;
    fn Properties(&self) -> ::windows::core::Result<super::Com::IDispatch>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQQueue2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueue2_Impl, const OFFSET: isize>() -> IMSMQQueue2_Vtbl {
        unsafe extern "system" fn Access<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueue2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, placcess: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Access() {
                ::core::result::Result::Ok(ok__) => {
                    *placcess = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShareMode<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueue2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plsharemode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ShareMode() {
                ::core::result::Result::Ok(ok__) => {
                    *plsharemode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueueInfo<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueue2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).QueueInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *ppqinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Handle<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueue2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plhandle: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Handle() {
                ::core::result::Result::Ok(ok__) => {
                    *plhandle = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsOpen<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueue2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisopen: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsOpen() {
                ::core::result::Result::Ok(ok__) => {
                    *pisopen = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Close<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueue2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Close().into()
        }
        unsafe extern "system" fn Receive_v1<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueue2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Receive_v1(::core::mem::transmute_copy(&transaction), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Peek_v1<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueue2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Peek_v1(::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnableNotification<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueue2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, event: ::windows::core::RawPtr, cursor: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EnableNotification(::core::mem::transmute(&event), ::core::mem::transmute_copy(&cursor), ::core::mem::transmute_copy(&receivetimeout)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueue2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn ReceiveCurrent_v1<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueue2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ReceiveCurrent_v1(::core::mem::transmute_copy(&transaction), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeekNext_v1<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueue2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PeekNext_v1(::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeekCurrent_v1<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueue2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PeekCurrent_v1(::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Receive<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueue2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Receive(::core::mem::transmute_copy(&transaction), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Peek<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueue2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Peek(::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReceiveCurrent<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueue2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ReceiveCurrent(::core::mem::transmute_copy(&transaction), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeekNext<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueue2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PeekNext(::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeekCurrent<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueue2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PeekCurrent(::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueue2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcolproperties = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Access: Access::<Identity, Impl, OFFSET>,
            ShareMode: ShareMode::<Identity, Impl, OFFSET>,
            QueueInfo: QueueInfo::<Identity, Impl, OFFSET>,
            Handle: Handle::<Identity, Impl, OFFSET>,
            IsOpen: IsOpen::<Identity, Impl, OFFSET>,
            Close: Close::<Identity, Impl, OFFSET>,
            Receive_v1: Receive_v1::<Identity, Impl, OFFSET>,
            Peek_v1: Peek_v1::<Identity, Impl, OFFSET>,
            EnableNotification: EnableNotification::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            ReceiveCurrent_v1: ReceiveCurrent_v1::<Identity, Impl, OFFSET>,
            PeekNext_v1: PeekNext_v1::<Identity, Impl, OFFSET>,
            PeekCurrent_v1: PeekCurrent_v1::<Identity, Impl, OFFSET>,
            Receive: Receive::<Identity, Impl, OFFSET>,
            Peek: Peek::<Identity, Impl, OFFSET>,
            ReceiveCurrent: ReceiveCurrent::<Identity, Impl, OFFSET>,
            PeekNext: PeekNext::<Identity, Impl, OFFSET>,
            PeekCurrent: PeekCurrent::<Identity, Impl, OFFSET>,
            Properties: Properties::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQQueue2 as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQQueue3_Impl: Sized + super::Com::IDispatch_Impl {
    fn Access(&self) -> ::windows::core::Result<i32>;
    fn ShareMode(&self) -> ::windows::core::Result<i32>;
    fn QueueInfo(&self) -> ::windows::core::Result<IMSMQQueueInfo3>;
    fn Handle(&self) -> ::windows::core::Result<i32>;
    fn IsOpen(&self) -> ::windows::core::Result<i16>;
    fn Close(&self) -> ::windows::core::Result<()>;
    fn Receive_v1(&self, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage>;
    fn Peek_v1(&self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage>;
    fn EnableNotification(&self, event: &::core::option::Option<IMSMQEvent3>, cursor: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Reset(&self) -> ::windows::core::Result<()>;
    fn ReceiveCurrent_v1(&self, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage>;
    fn PeekNext_v1(&self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage>;
    fn PeekCurrent_v1(&self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage>;
    fn Receive(&self, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage3>;
    fn Peek(&self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage3>;
    fn ReceiveCurrent(&self, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage3>;
    fn PeekNext(&self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage3>;
    fn PeekCurrent(&self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage3>;
    fn Properties(&self) -> ::windows::core::Result<super::Com::IDispatch>;
    fn Handle2(&self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn ReceiveByLookupId(&self, lookupid: &super::Com::VARIANT, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage3>;
    fn ReceiveNextByLookupId(&self, lookupid: &super::Com::VARIANT, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage3>;
    fn ReceivePreviousByLookupId(&self, lookupid: &super::Com::VARIANT, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage3>;
    fn ReceiveFirstByLookupId(&self, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage3>;
    fn ReceiveLastByLookupId(&self, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage3>;
    fn PeekByLookupId(&self, lookupid: &super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage3>;
    fn PeekNextByLookupId(&self, lookupid: &super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage3>;
    fn PeekPreviousByLookupId(&self, lookupid: &super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage3>;
    fn PeekFirstByLookupId(&self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage3>;
    fn PeekLastByLookupId(&self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage3>;
    fn Purge(&self) -> ::windows::core::Result<()>;
    fn IsOpen2(&self) -> ::windows::core::Result<i16>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQQueue3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueue3_Impl, const OFFSET: isize>() -> IMSMQQueue3_Vtbl {
        unsafe extern "system" fn Access<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueue3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, placcess: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Access() {
                ::core::result::Result::Ok(ok__) => {
                    *placcess = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShareMode<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueue3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plsharemode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ShareMode() {
                ::core::result::Result::Ok(ok__) => {
                    *plsharemode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueueInfo<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueue3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).QueueInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *ppqinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Handle<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueue3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plhandle: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Handle() {
                ::core::result::Result::Ok(ok__) => {
                    *plhandle = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsOpen<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueue3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisopen: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsOpen() {
                ::core::result::Result::Ok(ok__) => {
                    *pisopen = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Close<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueue3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Close().into()
        }
        unsafe extern "system" fn Receive_v1<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueue3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Receive_v1(::core::mem::transmute_copy(&transaction), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Peek_v1<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueue3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Peek_v1(::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnableNotification<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueue3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, event: ::windows::core::RawPtr, cursor: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EnableNotification(::core::mem::transmute(&event), ::core::mem::transmute_copy(&cursor), ::core::mem::transmute_copy(&receivetimeout)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueue3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn ReceiveCurrent_v1<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueue3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ReceiveCurrent_v1(::core::mem::transmute_copy(&transaction), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeekNext_v1<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueue3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PeekNext_v1(::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeekCurrent_v1<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueue3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PeekCurrent_v1(::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Receive<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueue3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Receive(::core::mem::transmute_copy(&transaction), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Peek<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueue3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Peek(::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReceiveCurrent<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueue3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ReceiveCurrent(::core::mem::transmute_copy(&transaction), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeekNext<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueue3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PeekNext(::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeekCurrent<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueue3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PeekCurrent(::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueue3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcolproperties = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Handle2<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueue3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarhandle: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Handle2() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarhandle = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReceiveByLookupId<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueue3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lookupid: ::core::mem::ManuallyDrop<super::Com::VARIANT>, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ReceiveByLookupId(::core::mem::transmute(&lookupid), ::core::mem::transmute_copy(&transaction), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReceiveNextByLookupId<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueue3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lookupid: ::core::mem::ManuallyDrop<super::Com::VARIANT>, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ReceiveNextByLookupId(::core::mem::transmute(&lookupid), ::core::mem::transmute_copy(&transaction), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReceivePreviousByLookupId<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueue3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lookupid: ::core::mem::ManuallyDrop<super::Com::VARIANT>, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ReceivePreviousByLookupId(::core::mem::transmute(&lookupid), ::core::mem::transmute_copy(&transaction), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReceiveFirstByLookupId<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueue3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ReceiveFirstByLookupId(::core::mem::transmute_copy(&transaction), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReceiveLastByLookupId<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueue3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ReceiveLastByLookupId(::core::mem::transmute_copy(&transaction), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeekByLookupId<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueue3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lookupid: ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PeekByLookupId(::core::mem::transmute(&lookupid), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeekNextByLookupId<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueue3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lookupid: ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PeekNextByLookupId(::core::mem::transmute(&lookupid), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeekPreviousByLookupId<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueue3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lookupid: ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PeekPreviousByLookupId(::core::mem::transmute(&lookupid), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeekFirstByLookupId<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueue3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PeekFirstByLookupId(::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeekLastByLookupId<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueue3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PeekLastByLookupId(::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Purge<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueue3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Purge().into()
        }
        unsafe extern "system" fn IsOpen2<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueue3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisopen: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsOpen2() {
                ::core::result::Result::Ok(ok__) => {
                    *pisopen = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Access: Access::<Identity, Impl, OFFSET>,
            ShareMode: ShareMode::<Identity, Impl, OFFSET>,
            QueueInfo: QueueInfo::<Identity, Impl, OFFSET>,
            Handle: Handle::<Identity, Impl, OFFSET>,
            IsOpen: IsOpen::<Identity, Impl, OFFSET>,
            Close: Close::<Identity, Impl, OFFSET>,
            Receive_v1: Receive_v1::<Identity, Impl, OFFSET>,
            Peek_v1: Peek_v1::<Identity, Impl, OFFSET>,
            EnableNotification: EnableNotification::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            ReceiveCurrent_v1: ReceiveCurrent_v1::<Identity, Impl, OFFSET>,
            PeekNext_v1: PeekNext_v1::<Identity, Impl, OFFSET>,
            PeekCurrent_v1: PeekCurrent_v1::<Identity, Impl, OFFSET>,
            Receive: Receive::<Identity, Impl, OFFSET>,
            Peek: Peek::<Identity, Impl, OFFSET>,
            ReceiveCurrent: ReceiveCurrent::<Identity, Impl, OFFSET>,
            PeekNext: PeekNext::<Identity, Impl, OFFSET>,
            PeekCurrent: PeekCurrent::<Identity, Impl, OFFSET>,
            Properties: Properties::<Identity, Impl, OFFSET>,
            Handle2: Handle2::<Identity, Impl, OFFSET>,
            ReceiveByLookupId: ReceiveByLookupId::<Identity, Impl, OFFSET>,
            ReceiveNextByLookupId: ReceiveNextByLookupId::<Identity, Impl, OFFSET>,
            ReceivePreviousByLookupId: ReceivePreviousByLookupId::<Identity, Impl, OFFSET>,
            ReceiveFirstByLookupId: ReceiveFirstByLookupId::<Identity, Impl, OFFSET>,
            ReceiveLastByLookupId: ReceiveLastByLookupId::<Identity, Impl, OFFSET>,
            PeekByLookupId: PeekByLookupId::<Identity, Impl, OFFSET>,
            PeekNextByLookupId: PeekNextByLookupId::<Identity, Impl, OFFSET>,
            PeekPreviousByLookupId: PeekPreviousByLookupId::<Identity, Impl, OFFSET>,
            PeekFirstByLookupId: PeekFirstByLookupId::<Identity, Impl, OFFSET>,
            PeekLastByLookupId: PeekLastByLookupId::<Identity, Impl, OFFSET>,
            Purge: Purge::<Identity, Impl, OFFSET>,
            IsOpen2: IsOpen2::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQQueue3 as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQQueue4_Impl: Sized + super::Com::IDispatch_Impl {
    fn Access(&self) -> ::windows::core::Result<i32>;
    fn ShareMode(&self) -> ::windows::core::Result<i32>;
    fn QueueInfo(&self) -> ::windows::core::Result<IMSMQQueueInfo4>;
    fn Handle(&self) -> ::windows::core::Result<i32>;
    fn IsOpen(&self) -> ::windows::core::Result<i16>;
    fn Close(&self) -> ::windows::core::Result<()>;
    fn Receive_v1(&self, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage>;
    fn Peek_v1(&self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage>;
    fn EnableNotification(&self, event: &::core::option::Option<IMSMQEvent3>, cursor: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Reset(&self) -> ::windows::core::Result<()>;
    fn ReceiveCurrent_v1(&self, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage>;
    fn PeekNext_v1(&self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage>;
    fn PeekCurrent_v1(&self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage>;
    fn Receive(&self, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage4>;
    fn Peek(&self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage4>;
    fn ReceiveCurrent(&self, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage4>;
    fn PeekNext(&self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage4>;
    fn PeekCurrent(&self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage4>;
    fn Properties(&self) -> ::windows::core::Result<super::Com::IDispatch>;
    fn Handle2(&self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn ReceiveByLookupId(&self, lookupid: &super::Com::VARIANT, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage4>;
    fn ReceiveNextByLookupId(&self, lookupid: &super::Com::VARIANT, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage4>;
    fn ReceivePreviousByLookupId(&self, lookupid: &super::Com::VARIANT, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage4>;
    fn ReceiveFirstByLookupId(&self, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage4>;
    fn ReceiveLastByLookupId(&self, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage4>;
    fn PeekByLookupId(&self, lookupid: &super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage4>;
    fn PeekNextByLookupId(&self, lookupid: &super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage4>;
    fn PeekPreviousByLookupId(&self, lookupid: &super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage4>;
    fn PeekFirstByLookupId(&self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage4>;
    fn PeekLastByLookupId(&self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage4>;
    fn Purge(&self) -> ::windows::core::Result<()>;
    fn IsOpen2(&self) -> ::windows::core::Result<i16>;
    fn ReceiveByLookupIdAllowPeek(&self, lookupid: &super::Com::VARIANT, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage4>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQQueue4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueue4_Impl, const OFFSET: isize>() -> IMSMQQueue4_Vtbl {
        unsafe extern "system" fn Access<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueue4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, placcess: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Access() {
                ::core::result::Result::Ok(ok__) => {
                    *placcess = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShareMode<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueue4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plsharemode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ShareMode() {
                ::core::result::Result::Ok(ok__) => {
                    *plsharemode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueueInfo<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueue4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).QueueInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *ppqinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Handle<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueue4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plhandle: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Handle() {
                ::core::result::Result::Ok(ok__) => {
                    *plhandle = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsOpen<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueue4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisopen: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsOpen() {
                ::core::result::Result::Ok(ok__) => {
                    *pisopen = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Close<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueue4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Close().into()
        }
        unsafe extern "system" fn Receive_v1<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueue4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Receive_v1(::core::mem::transmute_copy(&transaction), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Peek_v1<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueue4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Peek_v1(::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnableNotification<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueue4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, event: ::windows::core::RawPtr, cursor: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EnableNotification(::core::mem::transmute(&event), ::core::mem::transmute_copy(&cursor), ::core::mem::transmute_copy(&receivetimeout)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueue4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn ReceiveCurrent_v1<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueue4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ReceiveCurrent_v1(::core::mem::transmute_copy(&transaction), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeekNext_v1<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueue4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PeekNext_v1(::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeekCurrent_v1<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueue4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PeekCurrent_v1(::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Receive<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueue4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Receive(::core::mem::transmute_copy(&transaction), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Peek<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueue4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Peek(::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReceiveCurrent<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueue4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ReceiveCurrent(::core::mem::transmute_copy(&transaction), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeekNext<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueue4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PeekNext(::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeekCurrent<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueue4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PeekCurrent(::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueue4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcolproperties = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Handle2<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueue4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarhandle: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Handle2() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarhandle = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReceiveByLookupId<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueue4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lookupid: ::core::mem::ManuallyDrop<super::Com::VARIANT>, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ReceiveByLookupId(::core::mem::transmute(&lookupid), ::core::mem::transmute_copy(&transaction), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReceiveNextByLookupId<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueue4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lookupid: ::core::mem::ManuallyDrop<super::Com::VARIANT>, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ReceiveNextByLookupId(::core::mem::transmute(&lookupid), ::core::mem::transmute_copy(&transaction), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReceivePreviousByLookupId<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueue4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lookupid: ::core::mem::ManuallyDrop<super::Com::VARIANT>, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ReceivePreviousByLookupId(::core::mem::transmute(&lookupid), ::core::mem::transmute_copy(&transaction), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReceiveFirstByLookupId<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueue4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ReceiveFirstByLookupId(::core::mem::transmute_copy(&transaction), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReceiveLastByLookupId<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueue4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ReceiveLastByLookupId(::core::mem::transmute_copy(&transaction), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeekByLookupId<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueue4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lookupid: ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PeekByLookupId(::core::mem::transmute(&lookupid), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeekNextByLookupId<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueue4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lookupid: ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PeekNextByLookupId(::core::mem::transmute(&lookupid), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeekPreviousByLookupId<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueue4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lookupid: ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PeekPreviousByLookupId(::core::mem::transmute(&lookupid), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeekFirstByLookupId<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueue4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PeekFirstByLookupId(::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeekLastByLookupId<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueue4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PeekLastByLookupId(::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Purge<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueue4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Purge().into()
        }
        unsafe extern "system" fn IsOpen2<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueue4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisopen: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsOpen2() {
                ::core::result::Result::Ok(ok__) => {
                    *pisopen = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReceiveByLookupIdAllowPeek<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueue4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lookupid: ::core::mem::ManuallyDrop<super::Com::VARIANT>, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ReceiveByLookupIdAllowPeek(::core::mem::transmute(&lookupid), ::core::mem::transmute_copy(&transaction), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmsg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Access: Access::<Identity, Impl, OFFSET>,
            ShareMode: ShareMode::<Identity, Impl, OFFSET>,
            QueueInfo: QueueInfo::<Identity, Impl, OFFSET>,
            Handle: Handle::<Identity, Impl, OFFSET>,
            IsOpen: IsOpen::<Identity, Impl, OFFSET>,
            Close: Close::<Identity, Impl, OFFSET>,
            Receive_v1: Receive_v1::<Identity, Impl, OFFSET>,
            Peek_v1: Peek_v1::<Identity, Impl, OFFSET>,
            EnableNotification: EnableNotification::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            ReceiveCurrent_v1: ReceiveCurrent_v1::<Identity, Impl, OFFSET>,
            PeekNext_v1: PeekNext_v1::<Identity, Impl, OFFSET>,
            PeekCurrent_v1: PeekCurrent_v1::<Identity, Impl, OFFSET>,
            Receive: Receive::<Identity, Impl, OFFSET>,
            Peek: Peek::<Identity, Impl, OFFSET>,
            ReceiveCurrent: ReceiveCurrent::<Identity, Impl, OFFSET>,
            PeekNext: PeekNext::<Identity, Impl, OFFSET>,
            PeekCurrent: PeekCurrent::<Identity, Impl, OFFSET>,
            Properties: Properties::<Identity, Impl, OFFSET>,
            Handle2: Handle2::<Identity, Impl, OFFSET>,
            ReceiveByLookupId: ReceiveByLookupId::<Identity, Impl, OFFSET>,
            ReceiveNextByLookupId: ReceiveNextByLookupId::<Identity, Impl, OFFSET>,
            ReceivePreviousByLookupId: ReceivePreviousByLookupId::<Identity, Impl, OFFSET>,
            ReceiveFirstByLookupId: ReceiveFirstByLookupId::<Identity, Impl, OFFSET>,
            ReceiveLastByLookupId: ReceiveLastByLookupId::<Identity, Impl, OFFSET>,
            PeekByLookupId: PeekByLookupId::<Identity, Impl, OFFSET>,
            PeekNextByLookupId: PeekNextByLookupId::<Identity, Impl, OFFSET>,
            PeekPreviousByLookupId: PeekPreviousByLookupId::<Identity, Impl, OFFSET>,
            PeekFirstByLookupId: PeekFirstByLookupId::<Identity, Impl, OFFSET>,
            PeekLastByLookupId: PeekLastByLookupId::<Identity, Impl, OFFSET>,
            Purge: Purge::<Identity, Impl, OFFSET>,
            IsOpen2: IsOpen2::<Identity, Impl, OFFSET>,
            ReceiveByLookupIdAllowPeek: ReceiveByLookupIdAllowPeek::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQQueue4 as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQQueueInfo_Impl: Sized + super::Com::IDispatch_Impl {
    fn QueueGuid(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn ServiceTypeGuid(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetServiceTypeGuid(&self, bstrguidservicetype: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Label(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetLabel(&self, bstrlabel: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn PathName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetPathName(&self, bstrpathname: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn FormatName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetFormatName(&self, bstrformatname: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn IsTransactional(&self) -> ::windows::core::Result<i16>;
    fn PrivLevel(&self) -> ::windows::core::Result<i32>;
    fn SetPrivLevel(&self, lprivlevel: i32) -> ::windows::core::Result<()>;
    fn Journal(&self) -> ::windows::core::Result<i32>;
    fn SetJournal(&self, ljournal: i32) -> ::windows::core::Result<()>;
    fn Quota(&self) -> ::windows::core::Result<i32>;
    fn SetQuota(&self, lquota: i32) -> ::windows::core::Result<()>;
    fn BasePriority(&self) -> ::windows::core::Result<i32>;
    fn SetBasePriority(&self, lbasepriority: i32) -> ::windows::core::Result<()>;
    fn CreateTime(&self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn ModifyTime(&self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn Authenticate(&self) -> ::windows::core::Result<i32>;
    fn SetAuthenticate(&self, lauthenticate: i32) -> ::windows::core::Result<()>;
    fn JournalQuota(&self) -> ::windows::core::Result<i32>;
    fn SetJournalQuota(&self, ljournalquota: i32) -> ::windows::core::Result<()>;
    fn IsWorldReadable(&self) -> ::windows::core::Result<i16>;
    fn Create(&self, istransactional: *const super::Com::VARIANT, isworldreadable: *const super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Delete(&self) -> ::windows::core::Result<()>;
    fn Open(&self, access: i32, sharemode: i32) -> ::windows::core::Result<IMSMQQueue>;
    fn Refresh(&self) -> ::windows::core::Result<()>;
    fn Update(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQQueueInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo_Impl, const OFFSET: isize>() -> IMSMQQueueInfo_Vtbl {
        unsafe extern "system" fn QueueGuid<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrguidqueue: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).QueueGuid() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrguidqueue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServiceTypeGuid<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrguidservicetype: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ServiceTypeGuid() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrguidservicetype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetServiceTypeGuid<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrguidservicetype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetServiceTypeGuid(::core::mem::transmute(&bstrguidservicetype)).into()
        }
        unsafe extern "system" fn Label<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrlabel: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Label() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrlabel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLabel<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrlabel: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetLabel(::core::mem::transmute(&bstrlabel)).into()
        }
        unsafe extern "system" fn PathName<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrpathname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PathName() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrpathname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPathName<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpathname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPathName(::core::mem::transmute(&bstrpathname)).into()
        }
        unsafe extern "system" fn FormatName<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrformatname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).FormatName() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrformatname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFormatName<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrformatname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetFormatName(::core::mem::transmute(&bstrformatname)).into()
        }
        unsafe extern "system" fn IsTransactional<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pistransactional: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsTransactional() {
                ::core::result::Result::Ok(ok__) => {
                    *pistransactional = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrivLevel<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plprivlevel: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PrivLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *plprivlevel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrivLevel<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lprivlevel: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPrivLevel(::core::mem::transmute_copy(&lprivlevel)).into()
        }
        unsafe extern "system" fn Journal<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pljournal: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Journal() {
                ::core::result::Result::Ok(ok__) => {
                    *pljournal = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetJournal<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ljournal: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetJournal(::core::mem::transmute_copy(&ljournal)).into()
        }
        unsafe extern "system" fn Quota<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plquota: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Quota() {
                ::core::result::Result::Ok(ok__) => {
                    *plquota = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetQuota<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lquota: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetQuota(::core::mem::transmute_copy(&lquota)).into()
        }
        unsafe extern "system" fn BasePriority<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plbasepriority: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).BasePriority() {
                ::core::result::Result::Ok(ok__) => {
                    *plbasepriority = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBasePriority<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lbasepriority: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetBasePriority(::core::mem::transmute_copy(&lbasepriority)).into()
        }
        unsafe extern "system" fn CreateTime<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarcreatetime: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateTime() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarcreatetime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ModifyTime<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarmodifytime: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ModifyTime() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarmodifytime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Authenticate<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plauthenticate: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Authenticate() {
                ::core::result::Result::Ok(ok__) => {
                    *plauthenticate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAuthenticate<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lauthenticate: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAuthenticate(::core::mem::transmute_copy(&lauthenticate)).into()
        }
        unsafe extern "system" fn JournalQuota<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pljournalquota: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).JournalQuota() {
                ::core::result::Result::Ok(ok__) => {
                    *pljournalquota = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetJournalQuota<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ljournalquota: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetJournalQuota(::core::mem::transmute_copy(&ljournalquota)).into()
        }
        unsafe extern "system" fn IsWorldReadable<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisworldreadable: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsWorldReadable() {
                ::core::result::Result::Ok(ok__) => {
                    *pisworldreadable = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Create<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, istransactional: *const super::Com::VARIANT, isworldreadable: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Create(::core::mem::transmute_copy(&istransactional), ::core::mem::transmute_copy(&isworldreadable)).into()
        }
        unsafe extern "system" fn Delete<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Delete().into()
        }
        unsafe extern "system" fn Open<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, access: i32, sharemode: i32, ppq: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Open(::core::mem::transmute_copy(&access), ::core::mem::transmute_copy(&sharemode)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppq = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Refresh().into()
        }
        unsafe extern "system" fn Update<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Update().into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            QueueGuid: QueueGuid::<Identity, Impl, OFFSET>,
            ServiceTypeGuid: ServiceTypeGuid::<Identity, Impl, OFFSET>,
            SetServiceTypeGuid: SetServiceTypeGuid::<Identity, Impl, OFFSET>,
            Label: Label::<Identity, Impl, OFFSET>,
            SetLabel: SetLabel::<Identity, Impl, OFFSET>,
            PathName: PathName::<Identity, Impl, OFFSET>,
            SetPathName: SetPathName::<Identity, Impl, OFFSET>,
            FormatName: FormatName::<Identity, Impl, OFFSET>,
            SetFormatName: SetFormatName::<Identity, Impl, OFFSET>,
            IsTransactional: IsTransactional::<Identity, Impl, OFFSET>,
            PrivLevel: PrivLevel::<Identity, Impl, OFFSET>,
            SetPrivLevel: SetPrivLevel::<Identity, Impl, OFFSET>,
            Journal: Journal::<Identity, Impl, OFFSET>,
            SetJournal: SetJournal::<Identity, Impl, OFFSET>,
            Quota: Quota::<Identity, Impl, OFFSET>,
            SetQuota: SetQuota::<Identity, Impl, OFFSET>,
            BasePriority: BasePriority::<Identity, Impl, OFFSET>,
            SetBasePriority: SetBasePriority::<Identity, Impl, OFFSET>,
            CreateTime: CreateTime::<Identity, Impl, OFFSET>,
            ModifyTime: ModifyTime::<Identity, Impl, OFFSET>,
            Authenticate: Authenticate::<Identity, Impl, OFFSET>,
            SetAuthenticate: SetAuthenticate::<Identity, Impl, OFFSET>,
            JournalQuota: JournalQuota::<Identity, Impl, OFFSET>,
            SetJournalQuota: SetJournalQuota::<Identity, Impl, OFFSET>,
            IsWorldReadable: IsWorldReadable::<Identity, Impl, OFFSET>,
            Create: Create::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
            Open: Open::<Identity, Impl, OFFSET>,
            Refresh: Refresh::<Identity, Impl, OFFSET>,
            Update: Update::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQQueueInfo as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQQueueInfo2_Impl: Sized + super::Com::IDispatch_Impl {
    fn QueueGuid(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn ServiceTypeGuid(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetServiceTypeGuid(&self, bstrguidservicetype: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Label(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetLabel(&self, bstrlabel: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn PathName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetPathName(&self, bstrpathname: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn FormatName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetFormatName(&self, bstrformatname: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn IsTransactional(&self) -> ::windows::core::Result<i16>;
    fn PrivLevel(&self) -> ::windows::core::Result<i32>;
    fn SetPrivLevel(&self, lprivlevel: i32) -> ::windows::core::Result<()>;
    fn Journal(&self) -> ::windows::core::Result<i32>;
    fn SetJournal(&self, ljournal: i32) -> ::windows::core::Result<()>;
    fn Quota(&self) -> ::windows::core::Result<i32>;
    fn SetQuota(&self, lquota: i32) -> ::windows::core::Result<()>;
    fn BasePriority(&self) -> ::windows::core::Result<i32>;
    fn SetBasePriority(&self, lbasepriority: i32) -> ::windows::core::Result<()>;
    fn CreateTime(&self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn ModifyTime(&self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn Authenticate(&self) -> ::windows::core::Result<i32>;
    fn SetAuthenticate(&self, lauthenticate: i32) -> ::windows::core::Result<()>;
    fn JournalQuota(&self) -> ::windows::core::Result<i32>;
    fn SetJournalQuota(&self, ljournalquota: i32) -> ::windows::core::Result<()>;
    fn IsWorldReadable(&self) -> ::windows::core::Result<i16>;
    fn Create(&self, istransactional: *const super::Com::VARIANT, isworldreadable: *const super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Delete(&self) -> ::windows::core::Result<()>;
    fn Open(&self, access: i32, sharemode: i32) -> ::windows::core::Result<IMSMQQueue2>;
    fn Refresh(&self) -> ::windows::core::Result<()>;
    fn Update(&self) -> ::windows::core::Result<()>;
    fn PathNameDNS(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Properties(&self) -> ::windows::core::Result<super::Com::IDispatch>;
    fn Security(&self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SetSecurity(&self, varsecurity: &super::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQQueueInfo2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo2_Impl, const OFFSET: isize>() -> IMSMQQueueInfo2_Vtbl {
        unsafe extern "system" fn QueueGuid<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrguidqueue: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).QueueGuid() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrguidqueue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServiceTypeGuid<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrguidservicetype: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ServiceTypeGuid() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrguidservicetype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetServiceTypeGuid<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrguidservicetype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetServiceTypeGuid(::core::mem::transmute(&bstrguidservicetype)).into()
        }
        unsafe extern "system" fn Label<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrlabel: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Label() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrlabel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLabel<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrlabel: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetLabel(::core::mem::transmute(&bstrlabel)).into()
        }
        unsafe extern "system" fn PathName<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrpathname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PathName() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrpathname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPathName<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpathname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPathName(::core::mem::transmute(&bstrpathname)).into()
        }
        unsafe extern "system" fn FormatName<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrformatname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).FormatName() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrformatname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFormatName<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrformatname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetFormatName(::core::mem::transmute(&bstrformatname)).into()
        }
        unsafe extern "system" fn IsTransactional<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pistransactional: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsTransactional() {
                ::core::result::Result::Ok(ok__) => {
                    *pistransactional = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrivLevel<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plprivlevel: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PrivLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *plprivlevel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrivLevel<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lprivlevel: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPrivLevel(::core::mem::transmute_copy(&lprivlevel)).into()
        }
        unsafe extern "system" fn Journal<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pljournal: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Journal() {
                ::core::result::Result::Ok(ok__) => {
                    *pljournal = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetJournal<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ljournal: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetJournal(::core::mem::transmute_copy(&ljournal)).into()
        }
        unsafe extern "system" fn Quota<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plquota: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Quota() {
                ::core::result::Result::Ok(ok__) => {
                    *plquota = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetQuota<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lquota: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetQuota(::core::mem::transmute_copy(&lquota)).into()
        }
        unsafe extern "system" fn BasePriority<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plbasepriority: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).BasePriority() {
                ::core::result::Result::Ok(ok__) => {
                    *plbasepriority = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBasePriority<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lbasepriority: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetBasePriority(::core::mem::transmute_copy(&lbasepriority)).into()
        }
        unsafe extern "system" fn CreateTime<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarcreatetime: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateTime() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarcreatetime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ModifyTime<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarmodifytime: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ModifyTime() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarmodifytime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Authenticate<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plauthenticate: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Authenticate() {
                ::core::result::Result::Ok(ok__) => {
                    *plauthenticate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAuthenticate<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lauthenticate: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAuthenticate(::core::mem::transmute_copy(&lauthenticate)).into()
        }
        unsafe extern "system" fn JournalQuota<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pljournalquota: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).JournalQuota() {
                ::core::result::Result::Ok(ok__) => {
                    *pljournalquota = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetJournalQuota<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ljournalquota: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetJournalQuota(::core::mem::transmute_copy(&ljournalquota)).into()
        }
        unsafe extern "system" fn IsWorldReadable<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisworldreadable: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsWorldReadable() {
                ::core::result::Result::Ok(ok__) => {
                    *pisworldreadable = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Create<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, istransactional: *const super::Com::VARIANT, isworldreadable: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Create(::core::mem::transmute_copy(&istransactional), ::core::mem::transmute_copy(&isworldreadable)).into()
        }
        unsafe extern "system" fn Delete<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Delete().into()
        }
        unsafe extern "system" fn Open<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, access: i32, sharemode: i32, ppq: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Open(::core::mem::transmute_copy(&access), ::core::mem::transmute_copy(&sharemode)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppq = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Refresh().into()
        }
        unsafe extern "system" fn Update<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Update().into()
        }
        unsafe extern "system" fn PathNameDNS<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrpathnamedns: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PathNameDNS() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrpathnamedns = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcolproperties = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Security<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarsecurity: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Security() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarsecurity = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSecurity<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varsecurity: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSecurity(::core::mem::transmute(&varsecurity)).into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            QueueGuid: QueueGuid::<Identity, Impl, OFFSET>,
            ServiceTypeGuid: ServiceTypeGuid::<Identity, Impl, OFFSET>,
            SetServiceTypeGuid: SetServiceTypeGuid::<Identity, Impl, OFFSET>,
            Label: Label::<Identity, Impl, OFFSET>,
            SetLabel: SetLabel::<Identity, Impl, OFFSET>,
            PathName: PathName::<Identity, Impl, OFFSET>,
            SetPathName: SetPathName::<Identity, Impl, OFFSET>,
            FormatName: FormatName::<Identity, Impl, OFFSET>,
            SetFormatName: SetFormatName::<Identity, Impl, OFFSET>,
            IsTransactional: IsTransactional::<Identity, Impl, OFFSET>,
            PrivLevel: PrivLevel::<Identity, Impl, OFFSET>,
            SetPrivLevel: SetPrivLevel::<Identity, Impl, OFFSET>,
            Journal: Journal::<Identity, Impl, OFFSET>,
            SetJournal: SetJournal::<Identity, Impl, OFFSET>,
            Quota: Quota::<Identity, Impl, OFFSET>,
            SetQuota: SetQuota::<Identity, Impl, OFFSET>,
            BasePriority: BasePriority::<Identity, Impl, OFFSET>,
            SetBasePriority: SetBasePriority::<Identity, Impl, OFFSET>,
            CreateTime: CreateTime::<Identity, Impl, OFFSET>,
            ModifyTime: ModifyTime::<Identity, Impl, OFFSET>,
            Authenticate: Authenticate::<Identity, Impl, OFFSET>,
            SetAuthenticate: SetAuthenticate::<Identity, Impl, OFFSET>,
            JournalQuota: JournalQuota::<Identity, Impl, OFFSET>,
            SetJournalQuota: SetJournalQuota::<Identity, Impl, OFFSET>,
            IsWorldReadable: IsWorldReadable::<Identity, Impl, OFFSET>,
            Create: Create::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
            Open: Open::<Identity, Impl, OFFSET>,
            Refresh: Refresh::<Identity, Impl, OFFSET>,
            Update: Update::<Identity, Impl, OFFSET>,
            PathNameDNS: PathNameDNS::<Identity, Impl, OFFSET>,
            Properties: Properties::<Identity, Impl, OFFSET>,
            Security: Security::<Identity, Impl, OFFSET>,
            SetSecurity: SetSecurity::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQQueueInfo2 as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQQueueInfo3_Impl: Sized + super::Com::IDispatch_Impl {
    fn QueueGuid(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn ServiceTypeGuid(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetServiceTypeGuid(&self, bstrguidservicetype: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Label(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetLabel(&self, bstrlabel: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn PathName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetPathName(&self, bstrpathname: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn FormatName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetFormatName(&self, bstrformatname: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn IsTransactional(&self) -> ::windows::core::Result<i16>;
    fn PrivLevel(&self) -> ::windows::core::Result<i32>;
    fn SetPrivLevel(&self, lprivlevel: i32) -> ::windows::core::Result<()>;
    fn Journal(&self) -> ::windows::core::Result<i32>;
    fn SetJournal(&self, ljournal: i32) -> ::windows::core::Result<()>;
    fn Quota(&self) -> ::windows::core::Result<i32>;
    fn SetQuota(&self, lquota: i32) -> ::windows::core::Result<()>;
    fn BasePriority(&self) -> ::windows::core::Result<i32>;
    fn SetBasePriority(&self, lbasepriority: i32) -> ::windows::core::Result<()>;
    fn CreateTime(&self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn ModifyTime(&self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn Authenticate(&self) -> ::windows::core::Result<i32>;
    fn SetAuthenticate(&self, lauthenticate: i32) -> ::windows::core::Result<()>;
    fn JournalQuota(&self) -> ::windows::core::Result<i32>;
    fn SetJournalQuota(&self, ljournalquota: i32) -> ::windows::core::Result<()>;
    fn IsWorldReadable(&self) -> ::windows::core::Result<i16>;
    fn Create(&self, istransactional: *const super::Com::VARIANT, isworldreadable: *const super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Delete(&self) -> ::windows::core::Result<()>;
    fn Open(&self, access: i32, sharemode: i32) -> ::windows::core::Result<IMSMQQueue3>;
    fn Refresh(&self) -> ::windows::core::Result<()>;
    fn Update(&self) -> ::windows::core::Result<()>;
    fn PathNameDNS(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Properties(&self) -> ::windows::core::Result<super::Com::IDispatch>;
    fn Security(&self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SetSecurity(&self, varsecurity: &super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn IsTransactional2(&self) -> ::windows::core::Result<i16>;
    fn IsWorldReadable2(&self) -> ::windows::core::Result<i16>;
    fn MulticastAddress(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetMulticastAddress(&self, bstrmulticastaddress: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ADsPath(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQQueueInfo3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo3_Impl, const OFFSET: isize>() -> IMSMQQueueInfo3_Vtbl {
        unsafe extern "system" fn QueueGuid<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrguidqueue: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).QueueGuid() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrguidqueue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServiceTypeGuid<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrguidservicetype: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ServiceTypeGuid() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrguidservicetype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetServiceTypeGuid<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrguidservicetype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetServiceTypeGuid(::core::mem::transmute(&bstrguidservicetype)).into()
        }
        unsafe extern "system" fn Label<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrlabel: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Label() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrlabel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLabel<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrlabel: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetLabel(::core::mem::transmute(&bstrlabel)).into()
        }
        unsafe extern "system" fn PathName<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrpathname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PathName() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrpathname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPathName<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpathname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPathName(::core::mem::transmute(&bstrpathname)).into()
        }
        unsafe extern "system" fn FormatName<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrformatname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).FormatName() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrformatname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFormatName<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrformatname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetFormatName(::core::mem::transmute(&bstrformatname)).into()
        }
        unsafe extern "system" fn IsTransactional<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pistransactional: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsTransactional() {
                ::core::result::Result::Ok(ok__) => {
                    *pistransactional = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrivLevel<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plprivlevel: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PrivLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *plprivlevel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrivLevel<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lprivlevel: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPrivLevel(::core::mem::transmute_copy(&lprivlevel)).into()
        }
        unsafe extern "system" fn Journal<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pljournal: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Journal() {
                ::core::result::Result::Ok(ok__) => {
                    *pljournal = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetJournal<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ljournal: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetJournal(::core::mem::transmute_copy(&ljournal)).into()
        }
        unsafe extern "system" fn Quota<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plquota: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Quota() {
                ::core::result::Result::Ok(ok__) => {
                    *plquota = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetQuota<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lquota: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetQuota(::core::mem::transmute_copy(&lquota)).into()
        }
        unsafe extern "system" fn BasePriority<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plbasepriority: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).BasePriority() {
                ::core::result::Result::Ok(ok__) => {
                    *plbasepriority = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBasePriority<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lbasepriority: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetBasePriority(::core::mem::transmute_copy(&lbasepriority)).into()
        }
        unsafe extern "system" fn CreateTime<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarcreatetime: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateTime() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarcreatetime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ModifyTime<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarmodifytime: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ModifyTime() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarmodifytime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Authenticate<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plauthenticate: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Authenticate() {
                ::core::result::Result::Ok(ok__) => {
                    *plauthenticate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAuthenticate<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lauthenticate: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAuthenticate(::core::mem::transmute_copy(&lauthenticate)).into()
        }
        unsafe extern "system" fn JournalQuota<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pljournalquota: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).JournalQuota() {
                ::core::result::Result::Ok(ok__) => {
                    *pljournalquota = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetJournalQuota<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ljournalquota: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetJournalQuota(::core::mem::transmute_copy(&ljournalquota)).into()
        }
        unsafe extern "system" fn IsWorldReadable<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisworldreadable: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsWorldReadable() {
                ::core::result::Result::Ok(ok__) => {
                    *pisworldreadable = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Create<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, istransactional: *const super::Com::VARIANT, isworldreadable: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Create(::core::mem::transmute_copy(&istransactional), ::core::mem::transmute_copy(&isworldreadable)).into()
        }
        unsafe extern "system" fn Delete<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Delete().into()
        }
        unsafe extern "system" fn Open<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, access: i32, sharemode: i32, ppq: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Open(::core::mem::transmute_copy(&access), ::core::mem::transmute_copy(&sharemode)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppq = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Refresh().into()
        }
        unsafe extern "system" fn Update<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Update().into()
        }
        unsafe extern "system" fn PathNameDNS<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrpathnamedns: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PathNameDNS() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrpathnamedns = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcolproperties = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Security<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarsecurity: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Security() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarsecurity = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSecurity<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varsecurity: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSecurity(::core::mem::transmute(&varsecurity)).into()
        }
        unsafe extern "system" fn IsTransactional2<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pistransactional: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsTransactional2() {
                ::core::result::Result::Ok(ok__) => {
                    *pistransactional = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsWorldReadable2<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisworldreadable: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsWorldReadable2() {
                ::core::result::Result::Ok(ok__) => {
                    *pisworldreadable = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MulticastAddress<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrmulticastaddress: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MulticastAddress() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrmulticastaddress = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMulticastAddress<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrmulticastaddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetMulticastAddress(::core::mem::transmute(&bstrmulticastaddress)).into()
        }
        unsafe extern "system" fn ADsPath<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstradspath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ADsPath() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstradspath = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            QueueGuid: QueueGuid::<Identity, Impl, OFFSET>,
            ServiceTypeGuid: ServiceTypeGuid::<Identity, Impl, OFFSET>,
            SetServiceTypeGuid: SetServiceTypeGuid::<Identity, Impl, OFFSET>,
            Label: Label::<Identity, Impl, OFFSET>,
            SetLabel: SetLabel::<Identity, Impl, OFFSET>,
            PathName: PathName::<Identity, Impl, OFFSET>,
            SetPathName: SetPathName::<Identity, Impl, OFFSET>,
            FormatName: FormatName::<Identity, Impl, OFFSET>,
            SetFormatName: SetFormatName::<Identity, Impl, OFFSET>,
            IsTransactional: IsTransactional::<Identity, Impl, OFFSET>,
            PrivLevel: PrivLevel::<Identity, Impl, OFFSET>,
            SetPrivLevel: SetPrivLevel::<Identity, Impl, OFFSET>,
            Journal: Journal::<Identity, Impl, OFFSET>,
            SetJournal: SetJournal::<Identity, Impl, OFFSET>,
            Quota: Quota::<Identity, Impl, OFFSET>,
            SetQuota: SetQuota::<Identity, Impl, OFFSET>,
            BasePriority: BasePriority::<Identity, Impl, OFFSET>,
            SetBasePriority: SetBasePriority::<Identity, Impl, OFFSET>,
            CreateTime: CreateTime::<Identity, Impl, OFFSET>,
            ModifyTime: ModifyTime::<Identity, Impl, OFFSET>,
            Authenticate: Authenticate::<Identity, Impl, OFFSET>,
            SetAuthenticate: SetAuthenticate::<Identity, Impl, OFFSET>,
            JournalQuota: JournalQuota::<Identity, Impl, OFFSET>,
            SetJournalQuota: SetJournalQuota::<Identity, Impl, OFFSET>,
            IsWorldReadable: IsWorldReadable::<Identity, Impl, OFFSET>,
            Create: Create::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
            Open: Open::<Identity, Impl, OFFSET>,
            Refresh: Refresh::<Identity, Impl, OFFSET>,
            Update: Update::<Identity, Impl, OFFSET>,
            PathNameDNS: PathNameDNS::<Identity, Impl, OFFSET>,
            Properties: Properties::<Identity, Impl, OFFSET>,
            Security: Security::<Identity, Impl, OFFSET>,
            SetSecurity: SetSecurity::<Identity, Impl, OFFSET>,
            IsTransactional2: IsTransactional2::<Identity, Impl, OFFSET>,
            IsWorldReadable2: IsWorldReadable2::<Identity, Impl, OFFSET>,
            MulticastAddress: MulticastAddress::<Identity, Impl, OFFSET>,
            SetMulticastAddress: SetMulticastAddress::<Identity, Impl, OFFSET>,
            ADsPath: ADsPath::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQQueueInfo3 as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQQueueInfo4_Impl: Sized + super::Com::IDispatch_Impl {
    fn QueueGuid(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn ServiceTypeGuid(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetServiceTypeGuid(&self, bstrguidservicetype: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Label(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetLabel(&self, bstrlabel: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn PathName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetPathName(&self, bstrpathname: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn FormatName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetFormatName(&self, bstrformatname: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn IsTransactional(&self) -> ::windows::core::Result<i16>;
    fn PrivLevel(&self) -> ::windows::core::Result<i32>;
    fn SetPrivLevel(&self, lprivlevel: i32) -> ::windows::core::Result<()>;
    fn Journal(&self) -> ::windows::core::Result<i32>;
    fn SetJournal(&self, ljournal: i32) -> ::windows::core::Result<()>;
    fn Quota(&self) -> ::windows::core::Result<i32>;
    fn SetQuota(&self, lquota: i32) -> ::windows::core::Result<()>;
    fn BasePriority(&self) -> ::windows::core::Result<i32>;
    fn SetBasePriority(&self, lbasepriority: i32) -> ::windows::core::Result<()>;
    fn CreateTime(&self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn ModifyTime(&self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn Authenticate(&self) -> ::windows::core::Result<i32>;
    fn SetAuthenticate(&self, lauthenticate: i32) -> ::windows::core::Result<()>;
    fn JournalQuota(&self) -> ::windows::core::Result<i32>;
    fn SetJournalQuota(&self, ljournalquota: i32) -> ::windows::core::Result<()>;
    fn IsWorldReadable(&self) -> ::windows::core::Result<i16>;
    fn Create(&self, istransactional: *const super::Com::VARIANT, isworldreadable: *const super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Delete(&self) -> ::windows::core::Result<()>;
    fn Open(&self, access: i32, sharemode: i32) -> ::windows::core::Result<IMSMQQueue4>;
    fn Refresh(&self) -> ::windows::core::Result<()>;
    fn Update(&self) -> ::windows::core::Result<()>;
    fn PathNameDNS(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Properties(&self) -> ::windows::core::Result<super::Com::IDispatch>;
    fn Security(&self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SetSecurity(&self, varsecurity: &super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn IsTransactional2(&self) -> ::windows::core::Result<i16>;
    fn IsWorldReadable2(&self) -> ::windows::core::Result<i16>;
    fn MulticastAddress(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetMulticastAddress(&self, bstrmulticastaddress: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ADsPath(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQQueueInfo4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo4_Impl, const OFFSET: isize>() -> IMSMQQueueInfo4_Vtbl {
        unsafe extern "system" fn QueueGuid<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrguidqueue: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).QueueGuid() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrguidqueue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServiceTypeGuid<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrguidservicetype: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ServiceTypeGuid() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrguidservicetype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetServiceTypeGuid<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrguidservicetype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetServiceTypeGuid(::core::mem::transmute(&bstrguidservicetype)).into()
        }
        unsafe extern "system" fn Label<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrlabel: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Label() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrlabel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLabel<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrlabel: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetLabel(::core::mem::transmute(&bstrlabel)).into()
        }
        unsafe extern "system" fn PathName<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrpathname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PathName() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrpathname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPathName<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpathname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPathName(::core::mem::transmute(&bstrpathname)).into()
        }
        unsafe extern "system" fn FormatName<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrformatname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).FormatName() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrformatname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFormatName<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrformatname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetFormatName(::core::mem::transmute(&bstrformatname)).into()
        }
        unsafe extern "system" fn IsTransactional<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pistransactional: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsTransactional() {
                ::core::result::Result::Ok(ok__) => {
                    *pistransactional = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrivLevel<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plprivlevel: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PrivLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *plprivlevel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrivLevel<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lprivlevel: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPrivLevel(::core::mem::transmute_copy(&lprivlevel)).into()
        }
        unsafe extern "system" fn Journal<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pljournal: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Journal() {
                ::core::result::Result::Ok(ok__) => {
                    *pljournal = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetJournal<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ljournal: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetJournal(::core::mem::transmute_copy(&ljournal)).into()
        }
        unsafe extern "system" fn Quota<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plquota: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Quota() {
                ::core::result::Result::Ok(ok__) => {
                    *plquota = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetQuota<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lquota: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetQuota(::core::mem::transmute_copy(&lquota)).into()
        }
        unsafe extern "system" fn BasePriority<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plbasepriority: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).BasePriority() {
                ::core::result::Result::Ok(ok__) => {
                    *plbasepriority = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBasePriority<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lbasepriority: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetBasePriority(::core::mem::transmute_copy(&lbasepriority)).into()
        }
        unsafe extern "system" fn CreateTime<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarcreatetime: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateTime() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarcreatetime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ModifyTime<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarmodifytime: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ModifyTime() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarmodifytime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Authenticate<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plauthenticate: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Authenticate() {
                ::core::result::Result::Ok(ok__) => {
                    *plauthenticate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAuthenticate<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lauthenticate: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAuthenticate(::core::mem::transmute_copy(&lauthenticate)).into()
        }
        unsafe extern "system" fn JournalQuota<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pljournalquota: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).JournalQuota() {
                ::core::result::Result::Ok(ok__) => {
                    *pljournalquota = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetJournalQuota<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ljournalquota: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetJournalQuota(::core::mem::transmute_copy(&ljournalquota)).into()
        }
        unsafe extern "system" fn IsWorldReadable<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisworldreadable: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsWorldReadable() {
                ::core::result::Result::Ok(ok__) => {
                    *pisworldreadable = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Create<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, istransactional: *const super::Com::VARIANT, isworldreadable: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Create(::core::mem::transmute_copy(&istransactional), ::core::mem::transmute_copy(&isworldreadable)).into()
        }
        unsafe extern "system" fn Delete<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Delete().into()
        }
        unsafe extern "system" fn Open<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, access: i32, sharemode: i32, ppq: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Open(::core::mem::transmute_copy(&access), ::core::mem::transmute_copy(&sharemode)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppq = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Refresh().into()
        }
        unsafe extern "system" fn Update<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Update().into()
        }
        unsafe extern "system" fn PathNameDNS<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrpathnamedns: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PathNameDNS() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrpathnamedns = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcolproperties = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Security<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarsecurity: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Security() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarsecurity = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSecurity<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varsecurity: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSecurity(::core::mem::transmute(&varsecurity)).into()
        }
        unsafe extern "system" fn IsTransactional2<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pistransactional: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsTransactional2() {
                ::core::result::Result::Ok(ok__) => {
                    *pistransactional = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsWorldReadable2<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisworldreadable: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsWorldReadable2() {
                ::core::result::Result::Ok(ok__) => {
                    *pisworldreadable = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MulticastAddress<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrmulticastaddress: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MulticastAddress() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrmulticastaddress = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMulticastAddress<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrmulticastaddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetMulticastAddress(::core::mem::transmute(&bstrmulticastaddress)).into()
        }
        unsafe extern "system" fn ADsPath<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstradspath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ADsPath() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstradspath = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            QueueGuid: QueueGuid::<Identity, Impl, OFFSET>,
            ServiceTypeGuid: ServiceTypeGuid::<Identity, Impl, OFFSET>,
            SetServiceTypeGuid: SetServiceTypeGuid::<Identity, Impl, OFFSET>,
            Label: Label::<Identity, Impl, OFFSET>,
            SetLabel: SetLabel::<Identity, Impl, OFFSET>,
            PathName: PathName::<Identity, Impl, OFFSET>,
            SetPathName: SetPathName::<Identity, Impl, OFFSET>,
            FormatName: FormatName::<Identity, Impl, OFFSET>,
            SetFormatName: SetFormatName::<Identity, Impl, OFFSET>,
            IsTransactional: IsTransactional::<Identity, Impl, OFFSET>,
            PrivLevel: PrivLevel::<Identity, Impl, OFFSET>,
            SetPrivLevel: SetPrivLevel::<Identity, Impl, OFFSET>,
            Journal: Journal::<Identity, Impl, OFFSET>,
            SetJournal: SetJournal::<Identity, Impl, OFFSET>,
            Quota: Quota::<Identity, Impl, OFFSET>,
            SetQuota: SetQuota::<Identity, Impl, OFFSET>,
            BasePriority: BasePriority::<Identity, Impl, OFFSET>,
            SetBasePriority: SetBasePriority::<Identity, Impl, OFFSET>,
            CreateTime: CreateTime::<Identity, Impl, OFFSET>,
            ModifyTime: ModifyTime::<Identity, Impl, OFFSET>,
            Authenticate: Authenticate::<Identity, Impl, OFFSET>,
            SetAuthenticate: SetAuthenticate::<Identity, Impl, OFFSET>,
            JournalQuota: JournalQuota::<Identity, Impl, OFFSET>,
            SetJournalQuota: SetJournalQuota::<Identity, Impl, OFFSET>,
            IsWorldReadable: IsWorldReadable::<Identity, Impl, OFFSET>,
            Create: Create::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
            Open: Open::<Identity, Impl, OFFSET>,
            Refresh: Refresh::<Identity, Impl, OFFSET>,
            Update: Update::<Identity, Impl, OFFSET>,
            PathNameDNS: PathNameDNS::<Identity, Impl, OFFSET>,
            Properties: Properties::<Identity, Impl, OFFSET>,
            Security: Security::<Identity, Impl, OFFSET>,
            SetSecurity: SetSecurity::<Identity, Impl, OFFSET>,
            IsTransactional2: IsTransactional2::<Identity, Impl, OFFSET>,
            IsWorldReadable2: IsWorldReadable2::<Identity, Impl, OFFSET>,
            MulticastAddress: MulticastAddress::<Identity, Impl, OFFSET>,
            SetMulticastAddress: SetMulticastAddress::<Identity, Impl, OFFSET>,
            ADsPath: ADsPath::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQQueueInfo4 as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQQueueInfos_Impl: Sized + super::Com::IDispatch_Impl {
    fn Reset(&self) -> ::windows::core::Result<()>;
    fn Next(&self) -> ::windows::core::Result<IMSMQQueueInfo>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQQueueInfos_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfos_Impl, const OFFSET: isize>() -> IMSMQQueueInfos_Vtbl {
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfos_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfos_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinfonext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Next() {
                ::core::result::Result::Ok(ok__) => {
                    *ppqinfonext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(), Reset: Reset::<Identity, Impl, OFFSET>, Next: Next::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQQueueInfos as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQQueueInfos2_Impl: Sized + super::Com::IDispatch_Impl {
    fn Reset(&self) -> ::windows::core::Result<()>;
    fn Next(&self) -> ::windows::core::Result<IMSMQQueueInfo2>;
    fn Properties(&self) -> ::windows::core::Result<super::Com::IDispatch>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQQueueInfos2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfos2_Impl, const OFFSET: isize>() -> IMSMQQueueInfos2_Vtbl {
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfos2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfos2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinfonext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Next() {
                ::core::result::Result::Ok(ok__) => {
                    *ppqinfonext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfos2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcolproperties = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Reset: Reset::<Identity, Impl, OFFSET>,
            Next: Next::<Identity, Impl, OFFSET>,
            Properties: Properties::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQQueueInfos2 as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQQueueInfos3_Impl: Sized + super::Com::IDispatch_Impl {
    fn Reset(&self) -> ::windows::core::Result<()>;
    fn Next(&self) -> ::windows::core::Result<IMSMQQueueInfo3>;
    fn Properties(&self) -> ::windows::core::Result<super::Com::IDispatch>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQQueueInfos3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfos3_Impl, const OFFSET: isize>() -> IMSMQQueueInfos3_Vtbl {
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfos3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfos3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinfonext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Next() {
                ::core::result::Result::Ok(ok__) => {
                    *ppqinfonext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfos3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcolproperties = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Reset: Reset::<Identity, Impl, OFFSET>,
            Next: Next::<Identity, Impl, OFFSET>,
            Properties: Properties::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQQueueInfos3 as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQQueueInfos4_Impl: Sized + super::Com::IDispatch_Impl {
    fn Reset(&self) -> ::windows::core::Result<()>;
    fn Next(&self) -> ::windows::core::Result<IMSMQQueueInfo4>;
    fn Properties(&self) -> ::windows::core::Result<super::Com::IDispatch>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQQueueInfos4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfos4_Impl, const OFFSET: isize>() -> IMSMQQueueInfos4_Vtbl {
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfos4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfos4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinfonext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Next() {
                ::core::result::Result::Ok(ok__) => {
                    *ppqinfonext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfos4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcolproperties = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Reset: Reset::<Identity, Impl, OFFSET>,
            Next: Next::<Identity, Impl, OFFSET>,
            Properties: Properties::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQQueueInfos4 as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQQueueManagement_Impl: Sized + super::Com::IDispatch_Impl + IMSMQManagement_Impl {
    fn JournalMessageCount(&self) -> ::windows::core::Result<i32>;
    fn BytesInJournal(&self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn EodGetReceiveInfo(&self) -> ::windows::core::Result<super::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQQueueManagement_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueManagement_Impl, const OFFSET: isize>() -> IMSMQQueueManagement_Vtbl {
        unsafe extern "system" fn JournalMessageCount<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueManagement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pljournalmessagecount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).JournalMessageCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pljournalmessagecount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BytesInJournal<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueManagement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvbytesinjournal: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).BytesInJournal() {
                ::core::result::Result::Ok(ok__) => {
                    *pvbytesinjournal = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EodGetReceiveInfo<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueManagement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvcollection: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EodGetReceiveInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *pvcollection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IMSMQManagement_Vtbl::new::<Identity, Impl, OFFSET>(),
            JournalMessageCount: JournalMessageCount::<Identity, Impl, OFFSET>,
            BytesInJournal: BytesInJournal::<Identity, Impl, OFFSET>,
            EodGetReceiveInfo: EodGetReceiveInfo::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQQueueManagement as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IMSMQManagement as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQTransaction_Impl: Sized + super::Com::IDispatch_Impl {
    fn Transaction(&self) -> ::windows::core::Result<i32>;
    fn Commit(&self, fretaining: *const super::Com::VARIANT, grftc: *const super::Com::VARIANT, grfrm: *const super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Abort(&self, fretaining: *const super::Com::VARIANT, fasync: *const super::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQTransaction_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQTransaction_Impl, const OFFSET: isize>() -> IMSMQTransaction_Vtbl {
        unsafe extern "system" fn Transaction<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQTransaction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pltransaction: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Transaction() {
                ::core::result::Result::Ok(ok__) => {
                    *pltransaction = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Commit<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQTransaction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fretaining: *const super::Com::VARIANT, grftc: *const super::Com::VARIANT, grfrm: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Commit(::core::mem::transmute_copy(&fretaining), ::core::mem::transmute_copy(&grftc), ::core::mem::transmute_copy(&grfrm)).into()
        }
        unsafe extern "system" fn Abort<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQTransaction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fretaining: *const super::Com::VARIANT, fasync: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Abort(::core::mem::transmute_copy(&fretaining), ::core::mem::transmute_copy(&fasync)).into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Transaction: Transaction::<Identity, Impl, OFFSET>,
            Commit: Commit::<Identity, Impl, OFFSET>,
            Abort: Abort::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQTransaction as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQTransaction2_Impl: Sized + super::Com::IDispatch_Impl + IMSMQTransaction_Impl {
    fn InitNew(&self, vartransaction: &super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Properties(&self) -> ::windows::core::Result<super::Com::IDispatch>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQTransaction2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQTransaction2_Impl, const OFFSET: isize>() -> IMSMQTransaction2_Vtbl {
        unsafe extern "system" fn InitNew<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQTransaction2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vartransaction: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).InitNew(::core::mem::transmute(&vartransaction)).into()
        }
        unsafe extern "system" fn Properties<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQTransaction2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcolproperties = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IMSMQTransaction_Vtbl::new::<Identity, Impl, OFFSET>(),
            InitNew: InitNew::<Identity, Impl, OFFSET>,
            Properties: Properties::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQTransaction2 as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IMSMQTransaction as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQTransaction3_Impl: Sized + super::Com::IDispatch_Impl + IMSMQTransaction_Impl + IMSMQTransaction2_Impl {
    fn ITransaction(&self) -> ::windows::core::Result<super::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQTransaction3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQTransaction3_Impl, const OFFSET: isize>() -> IMSMQTransaction3_Vtbl {
        unsafe extern "system" fn ITransaction<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQTransaction3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvaritransaction: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ITransaction() {
                ::core::result::Result::Ok(ok__) => {
                    *pvaritransaction = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: IMSMQTransaction2_Vtbl::new::<Identity, Impl, OFFSET>(), ITransaction: ITransaction::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQTransaction3 as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IMSMQTransaction as ::windows::core::Interface>::IID || iid == &<IMSMQTransaction2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQTransactionDispenser_Impl: Sized + super::Com::IDispatch_Impl {
    fn BeginTransaction(&self) -> ::windows::core::Result<IMSMQTransaction>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQTransactionDispenser_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQTransactionDispenser_Impl, const OFFSET: isize>() -> IMSMQTransactionDispenser_Vtbl {
        unsafe extern "system" fn BeginTransaction<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQTransactionDispenser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptransaction: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).BeginTransaction() {
                ::core::result::Result::Ok(ok__) => {
                    *ptransaction = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(), BeginTransaction: BeginTransaction::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQTransactionDispenser as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQTransactionDispenser2_Impl: Sized + super::Com::IDispatch_Impl {
    fn BeginTransaction(&self) -> ::windows::core::Result<IMSMQTransaction2>;
    fn Properties(&self) -> ::windows::core::Result<super::Com::IDispatch>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQTransactionDispenser2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQTransactionDispenser2_Impl, const OFFSET: isize>() -> IMSMQTransactionDispenser2_Vtbl {
        unsafe extern "system" fn BeginTransaction<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQTransactionDispenser2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptransaction: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).BeginTransaction() {
                ::core::result::Result::Ok(ok__) => {
                    *ptransaction = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQTransactionDispenser2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcolproperties = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            BeginTransaction: BeginTransaction::<Identity, Impl, OFFSET>,
            Properties: Properties::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQTransactionDispenser2 as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQTransactionDispenser3_Impl: Sized + super::Com::IDispatch_Impl {
    fn BeginTransaction(&self) -> ::windows::core::Result<IMSMQTransaction3>;
    fn Properties(&self) -> ::windows::core::Result<super::Com::IDispatch>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQTransactionDispenser3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQTransactionDispenser3_Impl, const OFFSET: isize>() -> IMSMQTransactionDispenser3_Vtbl {
        unsafe extern "system" fn BeginTransaction<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQTransactionDispenser3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptransaction: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).BeginTransaction() {
                ::core::result::Result::Ok(ok__) => {
                    *ptransaction = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQTransactionDispenser3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcolproperties = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            BeginTransaction: BeginTransaction::<Identity, Impl, OFFSET>,
            Properties: Properties::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQTransactionDispenser3 as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait _DMSMQEventEvents_Impl: Sized + super::Com::IDispatch_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl _DMSMQEventEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: _DMSMQEventEvents_Impl, const OFFSET: isize>() -> _DMSMQEventEvents_Vtbl {
        Self { base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<_DMSMQEventEvents as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
