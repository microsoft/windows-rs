#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQApplication_Impl: Sized + super::Com::IDispatch_Impl {
    fn MachineIdOfMachineName(&self, machinename: &::windows::core::BSTR) -> ::windows::core::Result<::windows::core::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IMSMQApplication {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQApplication_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQApplication_Impl, const OFFSET: isize>() -> IMSMQApplication_Vtbl {
        unsafe extern "system" fn MachineIdOfMachineName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, machinename: ::std::mem::MaybeUninit<::windows::core::BSTR>, pbstrguid: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MachineIdOfMachineName(::core::mem::transmute(&machinename)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrguid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(), MachineIdOfMachineName: MachineIdOfMachineName::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQApplication as ::windows::core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQApplication2_Impl: Sized + IMSMQApplication_Impl {
    fn RegisterCertificate(&self, flags: *const super::Com::VARIANT, externalcertificate: *const super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn MachineNameOfMachineId(&self, bstrguid: &::windows::core::BSTR) -> ::windows::core::Result<::windows::core::BSTR>;
    fn MSMQVersionMajor(&self) -> ::windows::core::Result<i16>;
    fn MSMQVersionMinor(&self) -> ::windows::core::Result<i16>;
    fn MSMQVersionBuild(&self) -> ::windows::core::Result<i16>;
    fn IsDsEnabled(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn Properties(&self) -> ::windows::core::Result<super::Com::IDispatch>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IMSMQApplication2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQApplication2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQApplication2_Impl, const OFFSET: isize>() -> IMSMQApplication2_Vtbl {
        unsafe extern "system" fn RegisterCertificate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQApplication2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: *const super::Com::VARIANT, externalcertificate: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RegisterCertificate(::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&externalcertificate)).into()
        }
        unsafe extern "system" fn MachineNameOfMachineId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQApplication2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrguid: ::std::mem::MaybeUninit<::windows::core::BSTR>, pbstrmachinename: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MachineNameOfMachineId(::core::mem::transmute(&bstrguid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrmachinename, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MSMQVersionMajor<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQApplication2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psmsmqversionmajor: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MSMQVersionMajor() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(psmsmqversionmajor, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MSMQVersionMinor<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQApplication2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psmsmqversionminor: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MSMQVersionMinor() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(psmsmqversionminor, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MSMQVersionBuild<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQApplication2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psmsmqversionbuild: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MSMQVersionBuild() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(psmsmqversionbuild, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsDsEnabled<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQApplication2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfisdsenabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsDsEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfisdsenabled, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQApplication2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Properties() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcolproperties, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IMSMQApplication_Vtbl::new::<Identity, Impl, OFFSET>(),
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
        iid == &<IMSMQApplication2 as ::windows::core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows::core::ComInterface>::IID || iid == &<IMSMQApplication as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQApplication3_Impl: Sized + IMSMQApplication2_Impl {
    fn ActiveQueues(&self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn PrivateQueues(&self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn DirectoryServiceServer(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn IsConnected(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn BytesInAllQueues(&self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SetMachine(&self, bstrmachine: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn Machine(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn Connect(&self) -> ::windows::core::Result<()>;
    fn Disconnect(&self) -> ::windows::core::Result<()>;
    fn Tidy(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IMSMQApplication3 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQApplication3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQApplication3_Impl, const OFFSET: isize>() -> IMSMQApplication3_Vtbl {
        unsafe extern "system" fn ActiveQueues<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQApplication3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvactivequeues: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ActiveQueues() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvactivequeues, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrivateQueues<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQApplication3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvprivatequeues: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PrivateQueues() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvprivatequeues, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DirectoryServiceServer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQApplication3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdirectoryserviceserver: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DirectoryServiceServer() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrdirectoryserviceserver, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsConnected<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQApplication3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfisconnected: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsConnected() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfisconnected, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BytesInAllQueues<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQApplication3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvbytesinallqueues: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BytesInAllQueues() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvbytesinallqueues, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMachine<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQApplication3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrmachine: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMachine(::core::mem::transmute(&bstrmachine)).into()
        }
        unsafe extern "system" fn Machine<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQApplication3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrmachine: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Machine() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrmachine, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Connect<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQApplication3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Connect().into()
        }
        unsafe extern "system" fn Disconnect<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQApplication3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Disconnect().into()
        }
        unsafe extern "system" fn Tidy<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQApplication3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Tidy().into()
        }
        Self {
            base__: IMSMQApplication2_Vtbl::new::<Identity, Impl, OFFSET>(),
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
        iid == &<IMSMQApplication3 as ::windows::core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows::core::ComInterface>::IID || iid == &<IMSMQApplication as ::windows::core::ComInterface>::IID || iid == &<IMSMQApplication2 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQCollection_Impl: Sized + super::Com::IDispatch_Impl {
    fn Item(&self, index: *const super::Com::VARIANT) -> ::windows::core::Result<super::Com::VARIANT>;
    fn Count(&self) -> ::windows::core::Result<i32>;
    fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IMSMQCollection {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQCollection_Impl, const OFFSET: isize>() -> IMSMQCollection_Vtbl {
        unsafe extern "system" fn Item<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: *const super::Com::VARIANT, pvarret: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarret, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunk, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Item: Item::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQCollection as ::windows::core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQCoordinatedTransactionDispenser_Impl: Sized + super::Com::IDispatch_Impl {
    fn BeginTransaction(&self) -> ::windows::core::Result<IMSMQTransaction>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IMSMQCoordinatedTransactionDispenser {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQCoordinatedTransactionDispenser_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQCoordinatedTransactionDispenser_Impl, const OFFSET: isize>() -> IMSMQCoordinatedTransactionDispenser_Vtbl {
        unsafe extern "system" fn BeginTransaction<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQCoordinatedTransactionDispenser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptransaction: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginTransaction() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ptransaction, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(), BeginTransaction: BeginTransaction::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQCoordinatedTransactionDispenser as ::windows::core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQCoordinatedTransactionDispenser2_Impl: Sized + super::Com::IDispatch_Impl {
    fn BeginTransaction(&self) -> ::windows::core::Result<IMSMQTransaction2>;
    fn Properties(&self) -> ::windows::core::Result<super::Com::IDispatch>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IMSMQCoordinatedTransactionDispenser2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQCoordinatedTransactionDispenser2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQCoordinatedTransactionDispenser2_Impl, const OFFSET: isize>() -> IMSMQCoordinatedTransactionDispenser2_Vtbl {
        unsafe extern "system" fn BeginTransaction<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQCoordinatedTransactionDispenser2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptransaction: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginTransaction() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ptransaction, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQCoordinatedTransactionDispenser2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Properties() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcolproperties, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            BeginTransaction: BeginTransaction::<Identity, Impl, OFFSET>,
            Properties: Properties::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQCoordinatedTransactionDispenser2 as ::windows::core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQCoordinatedTransactionDispenser3_Impl: Sized + super::Com::IDispatch_Impl {
    fn BeginTransaction(&self) -> ::windows::core::Result<IMSMQTransaction3>;
    fn Properties(&self) -> ::windows::core::Result<super::Com::IDispatch>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IMSMQCoordinatedTransactionDispenser3 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQCoordinatedTransactionDispenser3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQCoordinatedTransactionDispenser3_Impl, const OFFSET: isize>() -> IMSMQCoordinatedTransactionDispenser3_Vtbl {
        unsafe extern "system" fn BeginTransaction<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQCoordinatedTransactionDispenser3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptransaction: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginTransaction() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ptransaction, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQCoordinatedTransactionDispenser3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Properties() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcolproperties, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            BeginTransaction: BeginTransaction::<Identity, Impl, OFFSET>,
            Properties: Properties::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQCoordinatedTransactionDispenser3 as ::windows::core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQDestination_Impl: Sized + super::Com::IDispatch_Impl {
    fn Open(&self) -> ::windows::core::Result<()>;
    fn Close(&self) -> ::windows::core::Result<()>;
    fn IsOpen(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn IADs(&self) -> ::windows::core::Result<super::Com::IDispatch>;
    fn putref_IADs(&self, piads: ::core::option::Option<&super::Com::IDispatch>) -> ::windows::core::Result<()>;
    fn ADsPath(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetADsPath(&self, bstradspath: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn PathName(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetPathName(&self, bstrpathname: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn FormatName(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetFormatName(&self, bstrformatname: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn Destinations(&self) -> ::windows::core::Result<super::Com::IDispatch>;
    fn putref_Destinations(&self, pdestinations: ::core::option::Option<&super::Com::IDispatch>) -> ::windows::core::Result<()>;
    fn Properties(&self) -> ::windows::core::Result<super::Com::IDispatch>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IMSMQDestination {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQDestination_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQDestination_Impl, const OFFSET: isize>() -> IMSMQDestination_Vtbl {
        unsafe extern "system" fn Open<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQDestination_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Open().into()
        }
        unsafe extern "system" fn Close<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQDestination_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Close().into()
        }
        unsafe extern "system" fn IsOpen<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQDestination_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfisopen: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsOpen() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfisopen, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IADs<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQDestination_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppiads: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IADs() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppiads, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_IADs<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQDestination_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piads: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.putref_IADs(::windows::core::from_raw_borrowed(&piads)).into()
        }
        unsafe extern "system" fn ADsPath<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQDestination_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstradspath: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ADsPath() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstradspath, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetADsPath<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQDestination_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstradspath: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetADsPath(::core::mem::transmute(&bstradspath)).into()
        }
        unsafe extern "system" fn PathName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQDestination_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrpathname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PathName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrpathname, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPathName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQDestination_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpathname: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPathName(::core::mem::transmute(&bstrpathname)).into()
        }
        unsafe extern "system" fn FormatName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQDestination_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrformatname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FormatName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrformatname, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFormatName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQDestination_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrformatname: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetFormatName(::core::mem::transmute(&bstrformatname)).into()
        }
        unsafe extern "system" fn Destinations<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQDestination_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdestinations: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Destinations() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdestinations, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_Destinations<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQDestination_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdestinations: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.putref_Destinations(::windows::core::from_raw_borrowed(&pdestinations)).into()
        }
        unsafe extern "system" fn Properties<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQDestination_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Properties() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcolproperties, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
        iid == &<IMSMQDestination as ::windows::core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQEvent_Impl: Sized + super::Com::IDispatch_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IMSMQEvent {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQEvent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQEvent_Impl, const OFFSET: isize>() -> IMSMQEvent_Vtbl {
        Self { base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQEvent as ::windows::core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQEvent2_Impl: Sized + IMSMQEvent_Impl {
    fn Properties(&self) -> ::windows::core::Result<super::Com::IDispatch>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IMSMQEvent2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQEvent2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQEvent2_Impl, const OFFSET: isize>() -> IMSMQEvent2_Vtbl {
        unsafe extern "system" fn Properties<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQEvent2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Properties() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcolproperties, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: IMSMQEvent_Vtbl::new::<Identity, Impl, OFFSET>(), Properties: Properties::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQEvent2 as ::windows::core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows::core::ComInterface>::IID || iid == &<IMSMQEvent as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQEvent3_Impl: Sized + IMSMQEvent2_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IMSMQEvent3 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQEvent3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQEvent3_Impl, const OFFSET: isize>() -> IMSMQEvent3_Vtbl {
        Self { base__: IMSMQEvent2_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQEvent3 as ::windows::core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows::core::ComInterface>::IID || iid == &<IMSMQEvent as ::windows::core::ComInterface>::IID || iid == &<IMSMQEvent2 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQManagement_Impl: Sized + super::Com::IDispatch_Impl {
    fn Init(&self, machine: *const super::Com::VARIANT, pathname: *const super::Com::VARIANT, formatname: *const super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn FormatName(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn Machine(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn MessageCount(&self) -> ::windows::core::Result<i32>;
    fn ForeignStatus(&self) -> ::windows::core::Result<i32>;
    fn QueueType(&self) -> ::windows::core::Result<i32>;
    fn IsLocal(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn TransactionalStatus(&self) -> ::windows::core::Result<i32>;
    fn BytesInQueue(&self) -> ::windows::core::Result<super::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IMSMQManagement {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQManagement_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQManagement_Impl, const OFFSET: isize>() -> IMSMQManagement_Vtbl {
        unsafe extern "system" fn Init<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQManagement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, machine: *const super::Com::VARIANT, pathname: *const super::Com::VARIANT, formatname: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Init(::core::mem::transmute_copy(&machine), ::core::mem::transmute_copy(&pathname), ::core::mem::transmute_copy(&formatname)).into()
        }
        unsafe extern "system" fn FormatName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQManagement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrformatname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FormatName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrformatname, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Machine<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQManagement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrmachine: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Machine() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrmachine, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MessageCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQManagement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmessagecount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MessageCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plmessagecount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ForeignStatus<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQManagement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plforeignstatus: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ForeignStatus() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plforeignstatus, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueueType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQManagement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plqueuetype: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.QueueType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plqueuetype, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsLocal<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQManagement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfislocal: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsLocal() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfislocal, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransactionalStatus<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQManagement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pltransactionalstatus: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.TransactionalStatus() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pltransactionalstatus, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BytesInQueue<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQManagement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvbytesinqueue: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BytesInQueue() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvbytesinqueue, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
        iid == &<IMSMQManagement as ::windows::core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
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
    fn putref_ResponseQueueInfo(&self, pqinforesponse: ::core::option::Option<&IMSMQQueueInfo>) -> ::windows::core::Result<()>;
    fn AppSpecific(&self) -> ::windows::core::Result<i32>;
    fn SetAppSpecific(&self, lappspecific: i32) -> ::windows::core::Result<()>;
    fn SourceMachineGuid(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn BodyLength(&self) -> ::windows::core::Result<i32>;
    fn Body(&self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SetBody(&self, varbody: &super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn AdminQueueInfo(&self) -> ::windows::core::Result<IMSMQQueueInfo>;
    fn putref_AdminQueueInfo(&self, pqinfoadmin: ::core::option::Option<&IMSMQQueueInfo>) -> ::windows::core::Result<()>;
    fn Id(&self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn CorrelationId(&self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SetCorrelationId(&self, varmsgid: &super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Ack(&self) -> ::windows::core::Result<i32>;
    fn SetAck(&self, lack: i32) -> ::windows::core::Result<()>;
    fn Label(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetLabel(&self, bstrlabel: &::windows::core::BSTR) -> ::windows::core::Result<()>;
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
    fn Send(&self, destinationqueue: ::core::option::Option<&IMSMQQueue>, transaction: *const super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn AttachCurrentSecurityContext(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IMSMQMessage {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQMessage_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage_Impl, const OFFSET: isize>() -> IMSMQMessage_Vtbl {
        unsafe extern "system" fn Class<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plclass: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Class() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plclass, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrivLevel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plprivlevel: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PrivLevel() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plprivlevel, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrivLevel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lprivlevel: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPrivLevel(::core::mem::transmute_copy(&lprivlevel)).into()
        }
        unsafe extern "system" fn AuthLevel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plauthlevel: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AuthLevel() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plauthlevel, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAuthLevel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lauthlevel: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAuthLevel(::core::mem::transmute_copy(&lauthlevel)).into()
        }
        unsafe extern "system" fn IsAuthenticated<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisauthenticated: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsAuthenticated() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pisauthenticated, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delivery<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pldelivery: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Delivery() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pldelivery, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDelivery<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ldelivery: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDelivery(::core::mem::transmute_copy(&ldelivery)).into()
        }
        unsafe extern "system" fn Trace<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pltrace: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Trace() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pltrace, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTrace<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ltrace: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetTrace(::core::mem::transmute_copy(&ltrace)).into()
        }
        unsafe extern "system" fn Priority<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plpriority: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Priority() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plpriority, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPriority<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpriority: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPriority(::core::mem::transmute_copy(&lpriority)).into()
        }
        unsafe extern "system" fn Journal<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pljournal: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Journal() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pljournal, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetJournal<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ljournal: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetJournal(::core::mem::transmute_copy(&ljournal)).into()
        }
        unsafe extern "system" fn ResponseQueueInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinforesponse: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ResponseQueueInfo() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppqinforesponse, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_ResponseQueueInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqinforesponse: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.putref_ResponseQueueInfo(::windows::core::from_raw_borrowed(&pqinforesponse)).into()
        }
        unsafe extern "system" fn AppSpecific<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plappspecific: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AppSpecific() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plappspecific, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAppSpecific<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lappspecific: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAppSpecific(::core::mem::transmute_copy(&lappspecific)).into()
        }
        unsafe extern "system" fn SourceMachineGuid<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrguidsrcmachine: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SourceMachineGuid() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrguidsrcmachine, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BodyLength<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcbbody: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BodyLength() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcbbody, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Body<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarbody: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Body() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarbody, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBody<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varbody: super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetBody(::core::mem::transmute(&varbody)).into()
        }
        unsafe extern "system" fn AdminQueueInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinfoadmin: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AdminQueueInfo() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppqinfoadmin, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_AdminQueueInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqinfoadmin: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.putref_AdminQueueInfo(::windows::core::from_raw_borrowed(&pqinfoadmin)).into()
        }
        unsafe extern "system" fn Id<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarmsgid: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Id() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarmsgid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CorrelationId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarmsgid: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CorrelationId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarmsgid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCorrelationId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varmsgid: super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetCorrelationId(::core::mem::transmute(&varmsgid)).into()
        }
        unsafe extern "system" fn Ack<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plack: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Ack() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plack, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAck<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lack: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAck(::core::mem::transmute_copy(&lack)).into()
        }
        unsafe extern "system" fn Label<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrlabel: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Label() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrlabel, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLabel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrlabel: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetLabel(::core::mem::transmute(&bstrlabel)).into()
        }
        unsafe extern "system" fn MaxTimeToReachQueue<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmaxtimetoreachqueue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MaxTimeToReachQueue() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plmaxtimetoreachqueue, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxTimeToReachQueue<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmaxtimetoreachqueue: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMaxTimeToReachQueue(::core::mem::transmute_copy(&lmaxtimetoreachqueue)).into()
        }
        unsafe extern "system" fn MaxTimeToReceive<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmaxtimetoreceive: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MaxTimeToReceive() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plmaxtimetoreceive, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxTimeToReceive<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmaxtimetoreceive: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMaxTimeToReceive(::core::mem::transmute_copy(&lmaxtimetoreceive)).into()
        }
        unsafe extern "system" fn HashAlgorithm<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plhashalg: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.HashAlgorithm() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plhashalg, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHashAlgorithm<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lhashalg: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetHashAlgorithm(::core::mem::transmute_copy(&lhashalg)).into()
        }
        unsafe extern "system" fn EncryptAlgorithm<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plencryptalg: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EncryptAlgorithm() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plencryptalg, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEncryptAlgorithm<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lencryptalg: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetEncryptAlgorithm(::core::mem::transmute_copy(&lencryptalg)).into()
        }
        unsafe extern "system" fn SentTime<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarsenttime: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SentTime() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarsenttime, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ArrivedTime<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plarrivedtime: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ArrivedTime() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plarrivedtime, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DestinationQueueInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinfodest: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DestinationQueueInfo() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppqinfodest, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SenderCertificate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarsendercert: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SenderCertificate() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarsendercert, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSenderCertificate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varsendercert: super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSenderCertificate(::core::mem::transmute(&varsendercert)).into()
        }
        unsafe extern "system" fn SenderId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarsenderid: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SenderId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarsenderid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SenderIdType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plsenderidtype: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SenderIdType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plsenderidtype, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSenderIdType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lsenderidtype: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSenderIdType(::core::mem::transmute_copy(&lsenderidtype)).into()
        }
        unsafe extern "system" fn Send<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, destinationqueue: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Send(::windows::core::from_raw_borrowed(&destinationqueue), ::core::mem::transmute_copy(&transaction)).into()
        }
        unsafe extern "system" fn AttachCurrentSecurityContext<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AttachCurrentSecurityContext().into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
        iid == &<IMSMQMessage as ::windows::core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
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
    fn putref_ResponseQueueInfo_v1(&self, pqinforesponse: ::core::option::Option<&IMSMQQueueInfo>) -> ::windows::core::Result<()>;
    fn AppSpecific(&self) -> ::windows::core::Result<i32>;
    fn SetAppSpecific(&self, lappspecific: i32) -> ::windows::core::Result<()>;
    fn SourceMachineGuid(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn BodyLength(&self) -> ::windows::core::Result<i32>;
    fn Body(&self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SetBody(&self, varbody: &super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn AdminQueueInfo_v1(&self) -> ::windows::core::Result<IMSMQQueueInfo>;
    fn putref_AdminQueueInfo_v1(&self, pqinfoadmin: ::core::option::Option<&IMSMQQueueInfo>) -> ::windows::core::Result<()>;
    fn Id(&self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn CorrelationId(&self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SetCorrelationId(&self, varmsgid: &super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Ack(&self) -> ::windows::core::Result<i32>;
    fn SetAck(&self, lack: i32) -> ::windows::core::Result<()>;
    fn Label(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetLabel(&self, bstrlabel: &::windows::core::BSTR) -> ::windows::core::Result<()>;
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
    fn Send(&self, destinationqueue: ::core::option::Option<&IMSMQQueue2>, transaction: *const super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn AttachCurrentSecurityContext(&self) -> ::windows::core::Result<()>;
    fn SenderVersion(&self) -> ::windows::core::Result<i32>;
    fn Extension(&self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SetExtension(&self, varextension: &super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn ConnectorTypeGuid(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetConnectorTypeGuid(&self, bstrguidconnectortype: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn TransactionStatusQueueInfo(&self) -> ::windows::core::Result<IMSMQQueueInfo2>;
    fn DestinationSymmetricKey(&self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SetDestinationSymmetricKey(&self, vardestsymmkey: &super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Signature(&self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SetSignature(&self, varsignature: &super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn AuthenticationProviderType(&self) -> ::windows::core::Result<i32>;
    fn SetAuthenticationProviderType(&self, lauthprovtype: i32) -> ::windows::core::Result<()>;
    fn AuthenticationProviderName(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetAuthenticationProviderName(&self, bstrauthprovname: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn SetSenderId(&self, varsenderid: &super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn MsgClass(&self) -> ::windows::core::Result<i32>;
    fn SetMsgClass(&self, lmsgclass: i32) -> ::windows::core::Result<()>;
    fn Properties(&self) -> ::windows::core::Result<super::Com::IDispatch>;
    fn TransactionId(&self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn IsFirstInTransaction(&self) -> ::windows::core::Result<i16>;
    fn IsLastInTransaction(&self) -> ::windows::core::Result<i16>;
    fn ResponseQueueInfo(&self) -> ::windows::core::Result<IMSMQQueueInfo2>;
    fn putref_ResponseQueueInfo(&self, pqinforesponse: ::core::option::Option<&IMSMQQueueInfo2>) -> ::windows::core::Result<()>;
    fn AdminQueueInfo(&self) -> ::windows::core::Result<IMSMQQueueInfo2>;
    fn putref_AdminQueueInfo(&self, pqinfoadmin: ::core::option::Option<&IMSMQQueueInfo2>) -> ::windows::core::Result<()>;
    fn ReceivedAuthenticationLevel(&self) -> ::windows::core::Result<i16>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IMSMQMessage2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQMessage2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: isize>() -> IMSMQMessage2_Vtbl {
        unsafe extern "system" fn Class<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plclass: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Class() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plclass, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrivLevel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plprivlevel: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PrivLevel() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plprivlevel, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrivLevel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lprivlevel: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPrivLevel(::core::mem::transmute_copy(&lprivlevel)).into()
        }
        unsafe extern "system" fn AuthLevel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plauthlevel: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AuthLevel() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plauthlevel, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAuthLevel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lauthlevel: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAuthLevel(::core::mem::transmute_copy(&lauthlevel)).into()
        }
        unsafe extern "system" fn IsAuthenticated<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisauthenticated: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsAuthenticated() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pisauthenticated, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delivery<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pldelivery: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Delivery() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pldelivery, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDelivery<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ldelivery: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDelivery(::core::mem::transmute_copy(&ldelivery)).into()
        }
        unsafe extern "system" fn Trace<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pltrace: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Trace() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pltrace, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTrace<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ltrace: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetTrace(::core::mem::transmute_copy(&ltrace)).into()
        }
        unsafe extern "system" fn Priority<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plpriority: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Priority() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plpriority, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPriority<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpriority: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPriority(::core::mem::transmute_copy(&lpriority)).into()
        }
        unsafe extern "system" fn Journal<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pljournal: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Journal() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pljournal, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetJournal<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ljournal: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetJournal(::core::mem::transmute_copy(&ljournal)).into()
        }
        unsafe extern "system" fn ResponseQueueInfo_v1<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinforesponse: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ResponseQueueInfo_v1() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppqinforesponse, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_ResponseQueueInfo_v1<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqinforesponse: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.putref_ResponseQueueInfo_v1(::windows::core::from_raw_borrowed(&pqinforesponse)).into()
        }
        unsafe extern "system" fn AppSpecific<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plappspecific: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AppSpecific() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plappspecific, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAppSpecific<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lappspecific: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAppSpecific(::core::mem::transmute_copy(&lappspecific)).into()
        }
        unsafe extern "system" fn SourceMachineGuid<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrguidsrcmachine: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SourceMachineGuid() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrguidsrcmachine, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BodyLength<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcbbody: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BodyLength() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcbbody, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Body<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarbody: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Body() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarbody, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBody<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varbody: super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetBody(::core::mem::transmute(&varbody)).into()
        }
        unsafe extern "system" fn AdminQueueInfo_v1<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinfoadmin: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AdminQueueInfo_v1() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppqinfoadmin, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_AdminQueueInfo_v1<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqinfoadmin: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.putref_AdminQueueInfo_v1(::windows::core::from_raw_borrowed(&pqinfoadmin)).into()
        }
        unsafe extern "system" fn Id<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarmsgid: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Id() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarmsgid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CorrelationId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarmsgid: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CorrelationId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarmsgid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCorrelationId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varmsgid: super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetCorrelationId(::core::mem::transmute(&varmsgid)).into()
        }
        unsafe extern "system" fn Ack<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plack: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Ack() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plack, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAck<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lack: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAck(::core::mem::transmute_copy(&lack)).into()
        }
        unsafe extern "system" fn Label<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrlabel: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Label() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrlabel, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLabel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrlabel: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetLabel(::core::mem::transmute(&bstrlabel)).into()
        }
        unsafe extern "system" fn MaxTimeToReachQueue<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmaxtimetoreachqueue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MaxTimeToReachQueue() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plmaxtimetoreachqueue, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxTimeToReachQueue<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmaxtimetoreachqueue: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMaxTimeToReachQueue(::core::mem::transmute_copy(&lmaxtimetoreachqueue)).into()
        }
        unsafe extern "system" fn MaxTimeToReceive<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmaxtimetoreceive: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MaxTimeToReceive() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plmaxtimetoreceive, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxTimeToReceive<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmaxtimetoreceive: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMaxTimeToReceive(::core::mem::transmute_copy(&lmaxtimetoreceive)).into()
        }
        unsafe extern "system" fn HashAlgorithm<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plhashalg: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.HashAlgorithm() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plhashalg, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHashAlgorithm<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lhashalg: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetHashAlgorithm(::core::mem::transmute_copy(&lhashalg)).into()
        }
        unsafe extern "system" fn EncryptAlgorithm<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plencryptalg: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EncryptAlgorithm() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plencryptalg, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEncryptAlgorithm<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lencryptalg: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetEncryptAlgorithm(::core::mem::transmute_copy(&lencryptalg)).into()
        }
        unsafe extern "system" fn SentTime<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarsenttime: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SentTime() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarsenttime, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ArrivedTime<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plarrivedtime: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ArrivedTime() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plarrivedtime, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DestinationQueueInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinfodest: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DestinationQueueInfo() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppqinfodest, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SenderCertificate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarsendercert: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SenderCertificate() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarsendercert, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSenderCertificate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varsendercert: super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSenderCertificate(::core::mem::transmute(&varsendercert)).into()
        }
        unsafe extern "system" fn SenderId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarsenderid: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SenderId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarsenderid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SenderIdType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plsenderidtype: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SenderIdType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plsenderidtype, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSenderIdType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lsenderidtype: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSenderIdType(::core::mem::transmute_copy(&lsenderidtype)).into()
        }
        unsafe extern "system" fn Send<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, destinationqueue: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Send(::windows::core::from_raw_borrowed(&destinationqueue), ::core::mem::transmute_copy(&transaction)).into()
        }
        unsafe extern "system" fn AttachCurrentSecurityContext<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AttachCurrentSecurityContext().into()
        }
        unsafe extern "system" fn SenderVersion<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plsenderversion: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SenderVersion() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plsenderversion, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Extension<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarextension: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Extension() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarextension, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExtension<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varextension: super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetExtension(::core::mem::transmute(&varextension)).into()
        }
        unsafe extern "system" fn ConnectorTypeGuid<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrguidconnectortype: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ConnectorTypeGuid() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrguidconnectortype, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetConnectorTypeGuid<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrguidconnectortype: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetConnectorTypeGuid(::core::mem::transmute(&bstrguidconnectortype)).into()
        }
        unsafe extern "system" fn TransactionStatusQueueInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinfoxactstatus: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.TransactionStatusQueueInfo() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppqinfoxactstatus, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DestinationSymmetricKey<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvardestsymmkey: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DestinationSymmetricKey() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvardestsymmkey, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDestinationSymmetricKey<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vardestsymmkey: super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDestinationSymmetricKey(::core::mem::transmute(&vardestsymmkey)).into()
        }
        unsafe extern "system" fn Signature<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarsignature: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Signature() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarsignature, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSignature<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varsignature: super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSignature(::core::mem::transmute(&varsignature)).into()
        }
        unsafe extern "system" fn AuthenticationProviderType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plauthprovtype: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AuthenticationProviderType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plauthprovtype, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAuthenticationProviderType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lauthprovtype: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAuthenticationProviderType(::core::mem::transmute_copy(&lauthprovtype)).into()
        }
        unsafe extern "system" fn AuthenticationProviderName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrauthprovname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AuthenticationProviderName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrauthprovname, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAuthenticationProviderName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrauthprovname: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAuthenticationProviderName(::core::mem::transmute(&bstrauthprovname)).into()
        }
        unsafe extern "system" fn SetSenderId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varsenderid: super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSenderId(::core::mem::transmute(&varsenderid)).into()
        }
        unsafe extern "system" fn MsgClass<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmsgclass: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MsgClass() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plmsgclass, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMsgClass<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmsgclass: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMsgClass(::core::mem::transmute_copy(&lmsgclass)).into()
        }
        unsafe extern "system" fn Properties<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Properties() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcolproperties, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransactionId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarxactid: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.TransactionId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarxactid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsFirstInTransaction<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisfirstinxact: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsFirstInTransaction() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pisfirstinxact, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsLastInTransaction<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pislastinxact: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsLastInTransaction() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pislastinxact, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResponseQueueInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinforesponse: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ResponseQueueInfo() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppqinforesponse, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_ResponseQueueInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqinforesponse: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.putref_ResponseQueueInfo(::windows::core::from_raw_borrowed(&pqinforesponse)).into()
        }
        unsafe extern "system" fn AdminQueueInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinfoadmin: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AdminQueueInfo() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppqinfoadmin, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_AdminQueueInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqinfoadmin: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.putref_AdminQueueInfo(::windows::core::from_raw_borrowed(&pqinfoadmin)).into()
        }
        unsafe extern "system" fn ReceivedAuthenticationLevel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psreceivedauthenticationlevel: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ReceivedAuthenticationLevel() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(psreceivedauthenticationlevel, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
        iid == &<IMSMQMessage2 as ::windows::core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
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
    fn putref_ResponseQueueInfo_v1(&self, pqinforesponse: ::core::option::Option<&IMSMQQueueInfo>) -> ::windows::core::Result<()>;
    fn AppSpecific(&self) -> ::windows::core::Result<i32>;
    fn SetAppSpecific(&self, lappspecific: i32) -> ::windows::core::Result<()>;
    fn SourceMachineGuid(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn BodyLength(&self) -> ::windows::core::Result<i32>;
    fn Body(&self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SetBody(&self, varbody: &super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn AdminQueueInfo_v1(&self) -> ::windows::core::Result<IMSMQQueueInfo>;
    fn putref_AdminQueueInfo_v1(&self, pqinfoadmin: ::core::option::Option<&IMSMQQueueInfo>) -> ::windows::core::Result<()>;
    fn Id(&self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn CorrelationId(&self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SetCorrelationId(&self, varmsgid: &super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Ack(&self) -> ::windows::core::Result<i32>;
    fn SetAck(&self, lack: i32) -> ::windows::core::Result<()>;
    fn Label(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetLabel(&self, bstrlabel: &::windows::core::BSTR) -> ::windows::core::Result<()>;
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
    fn Send(&self, destinationqueue: ::core::option::Option<&super::Com::IDispatch>, transaction: *const super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn AttachCurrentSecurityContext(&self) -> ::windows::core::Result<()>;
    fn SenderVersion(&self) -> ::windows::core::Result<i32>;
    fn Extension(&self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SetExtension(&self, varextension: &super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn ConnectorTypeGuid(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetConnectorTypeGuid(&self, bstrguidconnectortype: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn TransactionStatusQueueInfo(&self) -> ::windows::core::Result<IMSMQQueueInfo3>;
    fn DestinationSymmetricKey(&self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SetDestinationSymmetricKey(&self, vardestsymmkey: &super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Signature(&self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SetSignature(&self, varsignature: &super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn AuthenticationProviderType(&self) -> ::windows::core::Result<i32>;
    fn SetAuthenticationProviderType(&self, lauthprovtype: i32) -> ::windows::core::Result<()>;
    fn AuthenticationProviderName(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetAuthenticationProviderName(&self, bstrauthprovname: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn SetSenderId(&self, varsenderid: &super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn MsgClass(&self) -> ::windows::core::Result<i32>;
    fn SetMsgClass(&self, lmsgclass: i32) -> ::windows::core::Result<()>;
    fn Properties(&self) -> ::windows::core::Result<super::Com::IDispatch>;
    fn TransactionId(&self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn IsFirstInTransaction(&self) -> ::windows::core::Result<i16>;
    fn IsLastInTransaction(&self) -> ::windows::core::Result<i16>;
    fn ResponseQueueInfo_v2(&self) -> ::windows::core::Result<IMSMQQueueInfo2>;
    fn putref_ResponseQueueInfo_v2(&self, pqinforesponse: ::core::option::Option<&IMSMQQueueInfo2>) -> ::windows::core::Result<()>;
    fn AdminQueueInfo_v2(&self) -> ::windows::core::Result<IMSMQQueueInfo2>;
    fn putref_AdminQueueInfo_v2(&self, pqinfoadmin: ::core::option::Option<&IMSMQQueueInfo2>) -> ::windows::core::Result<()>;
    fn ReceivedAuthenticationLevel(&self) -> ::windows::core::Result<i16>;
    fn ResponseQueueInfo(&self) -> ::windows::core::Result<IMSMQQueueInfo3>;
    fn putref_ResponseQueueInfo(&self, pqinforesponse: ::core::option::Option<&IMSMQQueueInfo3>) -> ::windows::core::Result<()>;
    fn AdminQueueInfo(&self) -> ::windows::core::Result<IMSMQQueueInfo3>;
    fn putref_AdminQueueInfo(&self, pqinfoadmin: ::core::option::Option<&IMSMQQueueInfo3>) -> ::windows::core::Result<()>;
    fn ResponseDestination(&self) -> ::windows::core::Result<super::Com::IDispatch>;
    fn putref_ResponseDestination(&self, pdestresponse: ::core::option::Option<&super::Com::IDispatch>) -> ::windows::core::Result<()>;
    fn Destination(&self) -> ::windows::core::Result<super::Com::IDispatch>;
    fn LookupId(&self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn IsAuthenticated2(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn IsFirstInTransaction2(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn IsLastInTransaction2(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn AttachCurrentSecurityContext2(&self) -> ::windows::core::Result<()>;
    fn SoapEnvelope(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn CompoundMessage(&self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SetSoapHeader(&self, bstrsoapheader: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn SetSoapBody(&self, bstrsoapbody: &::windows::core::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IMSMQMessage3 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQMessage3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: isize>() -> IMSMQMessage3_Vtbl {
        unsafe extern "system" fn Class<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plclass: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Class() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plclass, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrivLevel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plprivlevel: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PrivLevel() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plprivlevel, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrivLevel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lprivlevel: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPrivLevel(::core::mem::transmute_copy(&lprivlevel)).into()
        }
        unsafe extern "system" fn AuthLevel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plauthlevel: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AuthLevel() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plauthlevel, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAuthLevel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lauthlevel: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAuthLevel(::core::mem::transmute_copy(&lauthlevel)).into()
        }
        unsafe extern "system" fn IsAuthenticated<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisauthenticated: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsAuthenticated() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pisauthenticated, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delivery<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pldelivery: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Delivery() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pldelivery, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDelivery<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ldelivery: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDelivery(::core::mem::transmute_copy(&ldelivery)).into()
        }
        unsafe extern "system" fn Trace<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pltrace: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Trace() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pltrace, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTrace<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ltrace: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetTrace(::core::mem::transmute_copy(&ltrace)).into()
        }
        unsafe extern "system" fn Priority<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plpriority: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Priority() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plpriority, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPriority<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpriority: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPriority(::core::mem::transmute_copy(&lpriority)).into()
        }
        unsafe extern "system" fn Journal<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pljournal: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Journal() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pljournal, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetJournal<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ljournal: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetJournal(::core::mem::transmute_copy(&ljournal)).into()
        }
        unsafe extern "system" fn ResponseQueueInfo_v1<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinforesponse: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ResponseQueueInfo_v1() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppqinforesponse, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_ResponseQueueInfo_v1<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqinforesponse: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.putref_ResponseQueueInfo_v1(::windows::core::from_raw_borrowed(&pqinforesponse)).into()
        }
        unsafe extern "system" fn AppSpecific<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plappspecific: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AppSpecific() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plappspecific, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAppSpecific<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lappspecific: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAppSpecific(::core::mem::transmute_copy(&lappspecific)).into()
        }
        unsafe extern "system" fn SourceMachineGuid<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrguidsrcmachine: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SourceMachineGuid() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrguidsrcmachine, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BodyLength<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcbbody: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BodyLength() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcbbody, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Body<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarbody: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Body() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarbody, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBody<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varbody: super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetBody(::core::mem::transmute(&varbody)).into()
        }
        unsafe extern "system" fn AdminQueueInfo_v1<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinfoadmin: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AdminQueueInfo_v1() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppqinfoadmin, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_AdminQueueInfo_v1<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqinfoadmin: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.putref_AdminQueueInfo_v1(::windows::core::from_raw_borrowed(&pqinfoadmin)).into()
        }
        unsafe extern "system" fn Id<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarmsgid: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Id() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarmsgid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CorrelationId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarmsgid: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CorrelationId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarmsgid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCorrelationId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varmsgid: super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetCorrelationId(::core::mem::transmute(&varmsgid)).into()
        }
        unsafe extern "system" fn Ack<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plack: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Ack() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plack, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAck<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lack: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAck(::core::mem::transmute_copy(&lack)).into()
        }
        unsafe extern "system" fn Label<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrlabel: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Label() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrlabel, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLabel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrlabel: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetLabel(::core::mem::transmute(&bstrlabel)).into()
        }
        unsafe extern "system" fn MaxTimeToReachQueue<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmaxtimetoreachqueue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MaxTimeToReachQueue() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plmaxtimetoreachqueue, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxTimeToReachQueue<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmaxtimetoreachqueue: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMaxTimeToReachQueue(::core::mem::transmute_copy(&lmaxtimetoreachqueue)).into()
        }
        unsafe extern "system" fn MaxTimeToReceive<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmaxtimetoreceive: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MaxTimeToReceive() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plmaxtimetoreceive, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxTimeToReceive<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmaxtimetoreceive: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMaxTimeToReceive(::core::mem::transmute_copy(&lmaxtimetoreceive)).into()
        }
        unsafe extern "system" fn HashAlgorithm<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plhashalg: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.HashAlgorithm() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plhashalg, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHashAlgorithm<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lhashalg: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetHashAlgorithm(::core::mem::transmute_copy(&lhashalg)).into()
        }
        unsafe extern "system" fn EncryptAlgorithm<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plencryptalg: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EncryptAlgorithm() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plencryptalg, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEncryptAlgorithm<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lencryptalg: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetEncryptAlgorithm(::core::mem::transmute_copy(&lencryptalg)).into()
        }
        unsafe extern "system" fn SentTime<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarsenttime: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SentTime() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarsenttime, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ArrivedTime<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plarrivedtime: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ArrivedTime() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plarrivedtime, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DestinationQueueInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinfodest: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DestinationQueueInfo() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppqinfodest, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SenderCertificate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarsendercert: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SenderCertificate() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarsendercert, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSenderCertificate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varsendercert: super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSenderCertificate(::core::mem::transmute(&varsendercert)).into()
        }
        unsafe extern "system" fn SenderId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarsenderid: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SenderId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarsenderid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SenderIdType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plsenderidtype: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SenderIdType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plsenderidtype, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSenderIdType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lsenderidtype: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSenderIdType(::core::mem::transmute_copy(&lsenderidtype)).into()
        }
        unsafe extern "system" fn Send<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, destinationqueue: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Send(::windows::core::from_raw_borrowed(&destinationqueue), ::core::mem::transmute_copy(&transaction)).into()
        }
        unsafe extern "system" fn AttachCurrentSecurityContext<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AttachCurrentSecurityContext().into()
        }
        unsafe extern "system" fn SenderVersion<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plsenderversion: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SenderVersion() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plsenderversion, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Extension<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarextension: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Extension() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarextension, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExtension<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varextension: super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetExtension(::core::mem::transmute(&varextension)).into()
        }
        unsafe extern "system" fn ConnectorTypeGuid<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrguidconnectortype: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ConnectorTypeGuid() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrguidconnectortype, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetConnectorTypeGuid<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrguidconnectortype: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetConnectorTypeGuid(::core::mem::transmute(&bstrguidconnectortype)).into()
        }
        unsafe extern "system" fn TransactionStatusQueueInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinfoxactstatus: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.TransactionStatusQueueInfo() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppqinfoxactstatus, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DestinationSymmetricKey<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvardestsymmkey: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DestinationSymmetricKey() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvardestsymmkey, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDestinationSymmetricKey<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vardestsymmkey: super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDestinationSymmetricKey(::core::mem::transmute(&vardestsymmkey)).into()
        }
        unsafe extern "system" fn Signature<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarsignature: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Signature() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarsignature, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSignature<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varsignature: super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSignature(::core::mem::transmute(&varsignature)).into()
        }
        unsafe extern "system" fn AuthenticationProviderType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plauthprovtype: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AuthenticationProviderType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plauthprovtype, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAuthenticationProviderType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lauthprovtype: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAuthenticationProviderType(::core::mem::transmute_copy(&lauthprovtype)).into()
        }
        unsafe extern "system" fn AuthenticationProviderName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrauthprovname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AuthenticationProviderName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrauthprovname, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAuthenticationProviderName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrauthprovname: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAuthenticationProviderName(::core::mem::transmute(&bstrauthprovname)).into()
        }
        unsafe extern "system" fn SetSenderId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varsenderid: super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSenderId(::core::mem::transmute(&varsenderid)).into()
        }
        unsafe extern "system" fn MsgClass<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmsgclass: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MsgClass() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plmsgclass, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMsgClass<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmsgclass: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMsgClass(::core::mem::transmute_copy(&lmsgclass)).into()
        }
        unsafe extern "system" fn Properties<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Properties() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcolproperties, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransactionId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarxactid: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.TransactionId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarxactid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsFirstInTransaction<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisfirstinxact: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsFirstInTransaction() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pisfirstinxact, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsLastInTransaction<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pislastinxact: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsLastInTransaction() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pislastinxact, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResponseQueueInfo_v2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinforesponse: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ResponseQueueInfo_v2() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppqinforesponse, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_ResponseQueueInfo_v2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqinforesponse: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.putref_ResponseQueueInfo_v2(::windows::core::from_raw_borrowed(&pqinforesponse)).into()
        }
        unsafe extern "system" fn AdminQueueInfo_v2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinfoadmin: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AdminQueueInfo_v2() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppqinfoadmin, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_AdminQueueInfo_v2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqinfoadmin: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.putref_AdminQueueInfo_v2(::windows::core::from_raw_borrowed(&pqinfoadmin)).into()
        }
        unsafe extern "system" fn ReceivedAuthenticationLevel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psreceivedauthenticationlevel: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ReceivedAuthenticationLevel() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(psreceivedauthenticationlevel, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResponseQueueInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinforesponse: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ResponseQueueInfo() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppqinforesponse, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_ResponseQueueInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqinforesponse: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.putref_ResponseQueueInfo(::windows::core::from_raw_borrowed(&pqinforesponse)).into()
        }
        unsafe extern "system" fn AdminQueueInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinfoadmin: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AdminQueueInfo() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppqinfoadmin, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_AdminQueueInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqinfoadmin: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.putref_AdminQueueInfo(::windows::core::from_raw_borrowed(&pqinfoadmin)).into()
        }
        unsafe extern "system" fn ResponseDestination<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdestresponse: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ResponseDestination() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdestresponse, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_ResponseDestination<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdestresponse: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.putref_ResponseDestination(::windows::core::from_raw_borrowed(&pdestresponse)).into()
        }
        unsafe extern "system" fn Destination<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdestdestination: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Destination() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdestdestination, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LookupId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarlookupid: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LookupId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarlookupid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsAuthenticated2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisauthenticated: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsAuthenticated2() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pisauthenticated, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsFirstInTransaction2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisfirstinxact: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsFirstInTransaction2() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pisfirstinxact, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsLastInTransaction2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pislastinxact: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsLastInTransaction2() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pislastinxact, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AttachCurrentSecurityContext2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AttachCurrentSecurityContext2().into()
        }
        unsafe extern "system" fn SoapEnvelope<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrsoapenvelope: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SoapEnvelope() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrsoapenvelope, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CompoundMessage<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarcompoundmessage: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CompoundMessage() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarcompoundmessage, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSoapHeader<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsoapheader: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSoapHeader(::core::mem::transmute(&bstrsoapheader)).into()
        }
        unsafe extern "system" fn SetSoapBody<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsoapbody: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSoapBody(::core::mem::transmute(&bstrsoapbody)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
        iid == &<IMSMQMessage3 as ::windows::core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
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
    fn putref_ResponseQueueInfo_v1(&self, pqinforesponse: ::core::option::Option<&IMSMQQueueInfo>) -> ::windows::core::Result<()>;
    fn AppSpecific(&self) -> ::windows::core::Result<i32>;
    fn SetAppSpecific(&self, lappspecific: i32) -> ::windows::core::Result<()>;
    fn SourceMachineGuid(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn BodyLength(&self) -> ::windows::core::Result<i32>;
    fn Body(&self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SetBody(&self, varbody: &super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn AdminQueueInfo_v1(&self) -> ::windows::core::Result<IMSMQQueueInfo>;
    fn putref_AdminQueueInfo_v1(&self, pqinfoadmin: ::core::option::Option<&IMSMQQueueInfo>) -> ::windows::core::Result<()>;
    fn Id(&self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn CorrelationId(&self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SetCorrelationId(&self, varmsgid: &super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Ack(&self) -> ::windows::core::Result<i32>;
    fn SetAck(&self, lack: i32) -> ::windows::core::Result<()>;
    fn Label(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetLabel(&self, bstrlabel: &::windows::core::BSTR) -> ::windows::core::Result<()>;
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
    fn Send(&self, destinationqueue: ::core::option::Option<&super::Com::IDispatch>, transaction: *const super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn AttachCurrentSecurityContext(&self) -> ::windows::core::Result<()>;
    fn SenderVersion(&self) -> ::windows::core::Result<i32>;
    fn Extension(&self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SetExtension(&self, varextension: &super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn ConnectorTypeGuid(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetConnectorTypeGuid(&self, bstrguidconnectortype: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn TransactionStatusQueueInfo(&self) -> ::windows::core::Result<IMSMQQueueInfo4>;
    fn DestinationSymmetricKey(&self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SetDestinationSymmetricKey(&self, vardestsymmkey: &super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Signature(&self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SetSignature(&self, varsignature: &super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn AuthenticationProviderType(&self) -> ::windows::core::Result<i32>;
    fn SetAuthenticationProviderType(&self, lauthprovtype: i32) -> ::windows::core::Result<()>;
    fn AuthenticationProviderName(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetAuthenticationProviderName(&self, bstrauthprovname: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn SetSenderId(&self, varsenderid: &super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn MsgClass(&self) -> ::windows::core::Result<i32>;
    fn SetMsgClass(&self, lmsgclass: i32) -> ::windows::core::Result<()>;
    fn Properties(&self) -> ::windows::core::Result<super::Com::IDispatch>;
    fn TransactionId(&self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn IsFirstInTransaction(&self) -> ::windows::core::Result<i16>;
    fn IsLastInTransaction(&self) -> ::windows::core::Result<i16>;
    fn ResponseQueueInfo_v2(&self) -> ::windows::core::Result<IMSMQQueueInfo2>;
    fn putref_ResponseQueueInfo_v2(&self, pqinforesponse: ::core::option::Option<&IMSMQQueueInfo2>) -> ::windows::core::Result<()>;
    fn AdminQueueInfo_v2(&self) -> ::windows::core::Result<IMSMQQueueInfo2>;
    fn putref_AdminQueueInfo_v2(&self, pqinfoadmin: ::core::option::Option<&IMSMQQueueInfo2>) -> ::windows::core::Result<()>;
    fn ReceivedAuthenticationLevel(&self) -> ::windows::core::Result<i16>;
    fn ResponseQueueInfo(&self) -> ::windows::core::Result<IMSMQQueueInfo4>;
    fn putref_ResponseQueueInfo(&self, pqinforesponse: ::core::option::Option<&IMSMQQueueInfo4>) -> ::windows::core::Result<()>;
    fn AdminQueueInfo(&self) -> ::windows::core::Result<IMSMQQueueInfo4>;
    fn putref_AdminQueueInfo(&self, pqinfoadmin: ::core::option::Option<&IMSMQQueueInfo4>) -> ::windows::core::Result<()>;
    fn ResponseDestination(&self) -> ::windows::core::Result<super::Com::IDispatch>;
    fn putref_ResponseDestination(&self, pdestresponse: ::core::option::Option<&super::Com::IDispatch>) -> ::windows::core::Result<()>;
    fn Destination(&self) -> ::windows::core::Result<super::Com::IDispatch>;
    fn LookupId(&self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn IsAuthenticated2(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn IsFirstInTransaction2(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn IsLastInTransaction2(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn AttachCurrentSecurityContext2(&self) -> ::windows::core::Result<()>;
    fn SoapEnvelope(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn CompoundMessage(&self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SetSoapHeader(&self, bstrsoapheader: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn SetSoapBody(&self, bstrsoapbody: &::windows::core::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IMSMQMessage4 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQMessage4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: isize>() -> IMSMQMessage4_Vtbl {
        unsafe extern "system" fn Class<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plclass: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Class() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plclass, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrivLevel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plprivlevel: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PrivLevel() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plprivlevel, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrivLevel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lprivlevel: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPrivLevel(::core::mem::transmute_copy(&lprivlevel)).into()
        }
        unsafe extern "system" fn AuthLevel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plauthlevel: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AuthLevel() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plauthlevel, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAuthLevel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lauthlevel: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAuthLevel(::core::mem::transmute_copy(&lauthlevel)).into()
        }
        unsafe extern "system" fn IsAuthenticated<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisauthenticated: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsAuthenticated() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pisauthenticated, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delivery<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pldelivery: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Delivery() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pldelivery, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDelivery<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ldelivery: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDelivery(::core::mem::transmute_copy(&ldelivery)).into()
        }
        unsafe extern "system" fn Trace<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pltrace: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Trace() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pltrace, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTrace<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ltrace: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetTrace(::core::mem::transmute_copy(&ltrace)).into()
        }
        unsafe extern "system" fn Priority<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plpriority: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Priority() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plpriority, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPriority<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpriority: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPriority(::core::mem::transmute_copy(&lpriority)).into()
        }
        unsafe extern "system" fn Journal<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pljournal: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Journal() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pljournal, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetJournal<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ljournal: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetJournal(::core::mem::transmute_copy(&ljournal)).into()
        }
        unsafe extern "system" fn ResponseQueueInfo_v1<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinforesponse: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ResponseQueueInfo_v1() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppqinforesponse, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_ResponseQueueInfo_v1<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqinforesponse: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.putref_ResponseQueueInfo_v1(::windows::core::from_raw_borrowed(&pqinforesponse)).into()
        }
        unsafe extern "system" fn AppSpecific<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plappspecific: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AppSpecific() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plappspecific, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAppSpecific<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lappspecific: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAppSpecific(::core::mem::transmute_copy(&lappspecific)).into()
        }
        unsafe extern "system" fn SourceMachineGuid<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrguidsrcmachine: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SourceMachineGuid() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrguidsrcmachine, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BodyLength<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcbbody: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BodyLength() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcbbody, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Body<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarbody: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Body() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarbody, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBody<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varbody: super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetBody(::core::mem::transmute(&varbody)).into()
        }
        unsafe extern "system" fn AdminQueueInfo_v1<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinfoadmin: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AdminQueueInfo_v1() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppqinfoadmin, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_AdminQueueInfo_v1<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqinfoadmin: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.putref_AdminQueueInfo_v1(::windows::core::from_raw_borrowed(&pqinfoadmin)).into()
        }
        unsafe extern "system" fn Id<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarmsgid: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Id() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarmsgid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CorrelationId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarmsgid: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CorrelationId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarmsgid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCorrelationId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varmsgid: super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetCorrelationId(::core::mem::transmute(&varmsgid)).into()
        }
        unsafe extern "system" fn Ack<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plack: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Ack() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plack, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAck<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lack: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAck(::core::mem::transmute_copy(&lack)).into()
        }
        unsafe extern "system" fn Label<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrlabel: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Label() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrlabel, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLabel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrlabel: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetLabel(::core::mem::transmute(&bstrlabel)).into()
        }
        unsafe extern "system" fn MaxTimeToReachQueue<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmaxtimetoreachqueue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MaxTimeToReachQueue() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plmaxtimetoreachqueue, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxTimeToReachQueue<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmaxtimetoreachqueue: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMaxTimeToReachQueue(::core::mem::transmute_copy(&lmaxtimetoreachqueue)).into()
        }
        unsafe extern "system" fn MaxTimeToReceive<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmaxtimetoreceive: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MaxTimeToReceive() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plmaxtimetoreceive, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxTimeToReceive<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmaxtimetoreceive: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMaxTimeToReceive(::core::mem::transmute_copy(&lmaxtimetoreceive)).into()
        }
        unsafe extern "system" fn HashAlgorithm<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plhashalg: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.HashAlgorithm() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plhashalg, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHashAlgorithm<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lhashalg: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetHashAlgorithm(::core::mem::transmute_copy(&lhashalg)).into()
        }
        unsafe extern "system" fn EncryptAlgorithm<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plencryptalg: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EncryptAlgorithm() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plencryptalg, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEncryptAlgorithm<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lencryptalg: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetEncryptAlgorithm(::core::mem::transmute_copy(&lencryptalg)).into()
        }
        unsafe extern "system" fn SentTime<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarsenttime: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SentTime() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarsenttime, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ArrivedTime<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plarrivedtime: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ArrivedTime() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plarrivedtime, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DestinationQueueInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinfodest: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DestinationQueueInfo() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppqinfodest, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SenderCertificate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarsendercert: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SenderCertificate() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarsendercert, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSenderCertificate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varsendercert: super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSenderCertificate(::core::mem::transmute(&varsendercert)).into()
        }
        unsafe extern "system" fn SenderId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarsenderid: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SenderId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarsenderid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SenderIdType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plsenderidtype: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SenderIdType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plsenderidtype, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSenderIdType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lsenderidtype: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSenderIdType(::core::mem::transmute_copy(&lsenderidtype)).into()
        }
        unsafe extern "system" fn Send<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, destinationqueue: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Send(::windows::core::from_raw_borrowed(&destinationqueue), ::core::mem::transmute_copy(&transaction)).into()
        }
        unsafe extern "system" fn AttachCurrentSecurityContext<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AttachCurrentSecurityContext().into()
        }
        unsafe extern "system" fn SenderVersion<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plsenderversion: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SenderVersion() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plsenderversion, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Extension<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarextension: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Extension() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarextension, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExtension<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varextension: super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetExtension(::core::mem::transmute(&varextension)).into()
        }
        unsafe extern "system" fn ConnectorTypeGuid<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrguidconnectortype: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ConnectorTypeGuid() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrguidconnectortype, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetConnectorTypeGuid<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrguidconnectortype: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetConnectorTypeGuid(::core::mem::transmute(&bstrguidconnectortype)).into()
        }
        unsafe extern "system" fn TransactionStatusQueueInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinfoxactstatus: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.TransactionStatusQueueInfo() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppqinfoxactstatus, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DestinationSymmetricKey<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvardestsymmkey: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DestinationSymmetricKey() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvardestsymmkey, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDestinationSymmetricKey<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vardestsymmkey: super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDestinationSymmetricKey(::core::mem::transmute(&vardestsymmkey)).into()
        }
        unsafe extern "system" fn Signature<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarsignature: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Signature() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarsignature, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSignature<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varsignature: super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSignature(::core::mem::transmute(&varsignature)).into()
        }
        unsafe extern "system" fn AuthenticationProviderType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plauthprovtype: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AuthenticationProviderType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plauthprovtype, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAuthenticationProviderType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lauthprovtype: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAuthenticationProviderType(::core::mem::transmute_copy(&lauthprovtype)).into()
        }
        unsafe extern "system" fn AuthenticationProviderName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrauthprovname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AuthenticationProviderName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrauthprovname, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAuthenticationProviderName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrauthprovname: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAuthenticationProviderName(::core::mem::transmute(&bstrauthprovname)).into()
        }
        unsafe extern "system" fn SetSenderId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varsenderid: super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSenderId(::core::mem::transmute(&varsenderid)).into()
        }
        unsafe extern "system" fn MsgClass<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmsgclass: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MsgClass() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plmsgclass, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMsgClass<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmsgclass: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMsgClass(::core::mem::transmute_copy(&lmsgclass)).into()
        }
        unsafe extern "system" fn Properties<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Properties() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcolproperties, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransactionId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarxactid: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.TransactionId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarxactid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsFirstInTransaction<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisfirstinxact: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsFirstInTransaction() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pisfirstinxact, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsLastInTransaction<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pislastinxact: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsLastInTransaction() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pislastinxact, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResponseQueueInfo_v2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinforesponse: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ResponseQueueInfo_v2() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppqinforesponse, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_ResponseQueueInfo_v2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqinforesponse: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.putref_ResponseQueueInfo_v2(::windows::core::from_raw_borrowed(&pqinforesponse)).into()
        }
        unsafe extern "system" fn AdminQueueInfo_v2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinfoadmin: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AdminQueueInfo_v2() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppqinfoadmin, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_AdminQueueInfo_v2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqinfoadmin: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.putref_AdminQueueInfo_v2(::windows::core::from_raw_borrowed(&pqinfoadmin)).into()
        }
        unsafe extern "system" fn ReceivedAuthenticationLevel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psreceivedauthenticationlevel: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ReceivedAuthenticationLevel() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(psreceivedauthenticationlevel, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResponseQueueInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinforesponse: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ResponseQueueInfo() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppqinforesponse, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_ResponseQueueInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqinforesponse: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.putref_ResponseQueueInfo(::windows::core::from_raw_borrowed(&pqinforesponse)).into()
        }
        unsafe extern "system" fn AdminQueueInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinfoadmin: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AdminQueueInfo() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppqinfoadmin, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_AdminQueueInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqinfoadmin: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.putref_AdminQueueInfo(::windows::core::from_raw_borrowed(&pqinfoadmin)).into()
        }
        unsafe extern "system" fn ResponseDestination<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdestresponse: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ResponseDestination() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdestresponse, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_ResponseDestination<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdestresponse: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.putref_ResponseDestination(::windows::core::from_raw_borrowed(&pdestresponse)).into()
        }
        unsafe extern "system" fn Destination<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdestdestination: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Destination() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdestdestination, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LookupId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarlookupid: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LookupId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarlookupid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsAuthenticated2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisauthenticated: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsAuthenticated2() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pisauthenticated, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsFirstInTransaction2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisfirstinxact: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsFirstInTransaction2() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pisfirstinxact, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsLastInTransaction2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pislastinxact: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsLastInTransaction2() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pislastinxact, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AttachCurrentSecurityContext2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AttachCurrentSecurityContext2().into()
        }
        unsafe extern "system" fn SoapEnvelope<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrsoapenvelope: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SoapEnvelope() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrsoapenvelope, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CompoundMessage<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarcompoundmessage: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CompoundMessage() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarcompoundmessage, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSoapHeader<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsoapheader: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSoapHeader(::core::mem::transmute(&bstrsoapheader)).into()
        }
        unsafe extern "system" fn SetSoapBody<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQMessage4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsoapbody: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSoapBody(::core::mem::transmute(&bstrsoapbody)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
        iid == &<IMSMQMessage4 as ::windows::core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQOutgoingQueueManagement_Impl: Sized + IMSMQManagement_Impl {
    fn State(&self) -> ::windows::core::Result<i32>;
    fn NextHops(&self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn EodGetSendInfo(&self) -> ::windows::core::Result<IMSMQCollection>;
    fn Resume(&self) -> ::windows::core::Result<()>;
    fn Pause(&self) -> ::windows::core::Result<()>;
    fn EodResend(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IMSMQOutgoingQueueManagement {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQOutgoingQueueManagement_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQOutgoingQueueManagement_Impl, const OFFSET: isize>() -> IMSMQOutgoingQueueManagement_Vtbl {
        unsafe extern "system" fn State<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQOutgoingQueueManagement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plstate: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.State() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plstate, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NextHops<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQOutgoingQueueManagement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvnexthops: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.NextHops() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvnexthops, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EodGetSendInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQOutgoingQueueManagement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcollection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EodGetSendInfo() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcollection, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Resume<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQOutgoingQueueManagement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Resume().into()
        }
        unsafe extern "system" fn Pause<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQOutgoingQueueManagement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Pause().into()
        }
        unsafe extern "system" fn EodResend<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQOutgoingQueueManagement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EodResend().into()
        }
        Self {
            base__: IMSMQManagement_Vtbl::new::<Identity, Impl, OFFSET>(),
            State: State::<Identity, Impl, OFFSET>,
            NextHops: NextHops::<Identity, Impl, OFFSET>,
            EodGetSendInfo: EodGetSendInfo::<Identity, Impl, OFFSET>,
            Resume: Resume::<Identity, Impl, OFFSET>,
            Pause: Pause::<Identity, Impl, OFFSET>,
            EodResend: EodResend::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQOutgoingQueueManagement as ::windows::core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows::core::ComInterface>::IID || iid == &<IMSMQManagement as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQPrivateDestination_Impl: Sized + super::Com::IDispatch_Impl {
    fn Handle(&self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SetHandle(&self, varhandle: &super::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IMSMQPrivateDestination {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQPrivateDestination_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQPrivateDestination_Impl, const OFFSET: isize>() -> IMSMQPrivateDestination_Vtbl {
        unsafe extern "system" fn Handle<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQPrivateDestination_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarhandle: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Handle() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarhandle, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHandle<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQPrivateDestination_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varhandle: super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetHandle(::core::mem::transmute(&varhandle)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Handle: Handle::<Identity, Impl, OFFSET>,
            SetHandle: SetHandle::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQPrivateDestination as ::windows::core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQPrivateEvent_Impl: Sized + super::Com::IDispatch_Impl {
    fn Hwnd(&self) -> ::windows::core::Result<i32>;
    fn FireArrivedEvent(&self, pq: ::core::option::Option<&IMSMQQueue>, msgcursor: i32) -> ::windows::core::Result<()>;
    fn FireArrivedErrorEvent(&self, pq: ::core::option::Option<&IMSMQQueue>, hrstatus: ::windows::core::HRESULT, msgcursor: i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IMSMQPrivateEvent {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQPrivateEvent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQPrivateEvent_Impl, const OFFSET: isize>() -> IMSMQPrivateEvent_Vtbl {
        unsafe extern "system" fn Hwnd<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQPrivateEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phwnd: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Hwnd() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phwnd, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FireArrivedEvent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQPrivateEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pq: *mut ::core::ffi::c_void, msgcursor: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FireArrivedEvent(::windows::core::from_raw_borrowed(&pq), ::core::mem::transmute_copy(&msgcursor)).into()
        }
        unsafe extern "system" fn FireArrivedErrorEvent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQPrivateEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pq: *mut ::core::ffi::c_void, hrstatus: ::windows::core::HRESULT, msgcursor: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FireArrivedErrorEvent(::windows::core::from_raw_borrowed(&pq), ::core::mem::transmute_copy(&hrstatus), ::core::mem::transmute_copy(&msgcursor)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Hwnd: Hwnd::<Identity, Impl, OFFSET>,
            FireArrivedEvent: FireArrivedEvent::<Identity, Impl, OFFSET>,
            FireArrivedErrorEvent: FireArrivedErrorEvent::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQPrivateEvent as ::windows::core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQQuery_Impl: Sized + super::Com::IDispatch_Impl {
    fn LookupQueue(&self, queueguid: *const super::Com::VARIANT, servicetypeguid: *const super::Com::VARIANT, label: *const super::Com::VARIANT, createtime: *const super::Com::VARIANT, modifytime: *const super::Com::VARIANT, relservicetype: *const super::Com::VARIANT, rellabel: *const super::Com::VARIANT, relcreatetime: *const super::Com::VARIANT, relmodifytime: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQQueueInfos>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IMSMQQuery {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQQuery_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQuery_Impl, const OFFSET: isize>() -> IMSMQQuery_Vtbl {
        unsafe extern "system" fn LookupQueue<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQuery_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, queueguid: *const super::Com::VARIANT, servicetypeguid: *const super::Com::VARIANT, label: *const super::Com::VARIANT, createtime: *const super::Com::VARIANT, modifytime: *const super::Com::VARIANT, relservicetype: *const super::Com::VARIANT, rellabel: *const super::Com::VARIANT, relcreatetime: *const super::Com::VARIANT, relmodifytime: *const super::Com::VARIANT, ppqinfos: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LookupQueue(::core::mem::transmute_copy(&queueguid), ::core::mem::transmute_copy(&servicetypeguid), ::core::mem::transmute_copy(&label), ::core::mem::transmute_copy(&createtime), ::core::mem::transmute_copy(&modifytime), ::core::mem::transmute_copy(&relservicetype), ::core::mem::transmute_copy(&rellabel), ::core::mem::transmute_copy(&relcreatetime), ::core::mem::transmute_copy(&relmodifytime)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppqinfos, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(), LookupQueue: LookupQueue::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQQuery as ::windows::core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQQuery2_Impl: Sized + super::Com::IDispatch_Impl {
    fn LookupQueue(&self, queueguid: *const super::Com::VARIANT, servicetypeguid: *const super::Com::VARIANT, label: *const super::Com::VARIANT, createtime: *const super::Com::VARIANT, modifytime: *const super::Com::VARIANT, relservicetype: *const super::Com::VARIANT, rellabel: *const super::Com::VARIANT, relcreatetime: *const super::Com::VARIANT, relmodifytime: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQQueueInfos2>;
    fn Properties(&self) -> ::windows::core::Result<super::Com::IDispatch>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IMSMQQuery2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQQuery2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQuery2_Impl, const OFFSET: isize>() -> IMSMQQuery2_Vtbl {
        unsafe extern "system" fn LookupQueue<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQuery2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, queueguid: *const super::Com::VARIANT, servicetypeguid: *const super::Com::VARIANT, label: *const super::Com::VARIANT, createtime: *const super::Com::VARIANT, modifytime: *const super::Com::VARIANT, relservicetype: *const super::Com::VARIANT, rellabel: *const super::Com::VARIANT, relcreatetime: *const super::Com::VARIANT, relmodifytime: *const super::Com::VARIANT, ppqinfos: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LookupQueue(::core::mem::transmute_copy(&queueguid), ::core::mem::transmute_copy(&servicetypeguid), ::core::mem::transmute_copy(&label), ::core::mem::transmute_copy(&createtime), ::core::mem::transmute_copy(&modifytime), ::core::mem::transmute_copy(&relservicetype), ::core::mem::transmute_copy(&rellabel), ::core::mem::transmute_copy(&relcreatetime), ::core::mem::transmute_copy(&relmodifytime)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppqinfos, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQuery2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Properties() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcolproperties, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            LookupQueue: LookupQueue::<Identity, Impl, OFFSET>,
            Properties: Properties::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQQuery2 as ::windows::core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQQuery3_Impl: Sized + super::Com::IDispatch_Impl {
    fn LookupQueue_v2(&self, queueguid: *const super::Com::VARIANT, servicetypeguid: *const super::Com::VARIANT, label: *const super::Com::VARIANT, createtime: *const super::Com::VARIANT, modifytime: *const super::Com::VARIANT, relservicetype: *const super::Com::VARIANT, rellabel: *const super::Com::VARIANT, relcreatetime: *const super::Com::VARIANT, relmodifytime: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQQueueInfos3>;
    fn Properties(&self) -> ::windows::core::Result<super::Com::IDispatch>;
    fn LookupQueue(&self, queueguid: *const super::Com::VARIANT, servicetypeguid: *const super::Com::VARIANT, label: *const super::Com::VARIANT, createtime: *const super::Com::VARIANT, modifytime: *const super::Com::VARIANT, relservicetype: *const super::Com::VARIANT, rellabel: *const super::Com::VARIANT, relcreatetime: *const super::Com::VARIANT, relmodifytime: *const super::Com::VARIANT, multicastaddress: *const super::Com::VARIANT, relmulticastaddress: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQQueueInfos3>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IMSMQQuery3 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQQuery3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQuery3_Impl, const OFFSET: isize>() -> IMSMQQuery3_Vtbl {
        unsafe extern "system" fn LookupQueue_v2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQuery3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, queueguid: *const super::Com::VARIANT, servicetypeguid: *const super::Com::VARIANT, label: *const super::Com::VARIANT, createtime: *const super::Com::VARIANT, modifytime: *const super::Com::VARIANT, relservicetype: *const super::Com::VARIANT, rellabel: *const super::Com::VARIANT, relcreatetime: *const super::Com::VARIANT, relmodifytime: *const super::Com::VARIANT, ppqinfos: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LookupQueue_v2(::core::mem::transmute_copy(&queueguid), ::core::mem::transmute_copy(&servicetypeguid), ::core::mem::transmute_copy(&label), ::core::mem::transmute_copy(&createtime), ::core::mem::transmute_copy(&modifytime), ::core::mem::transmute_copy(&relservicetype), ::core::mem::transmute_copy(&rellabel), ::core::mem::transmute_copy(&relcreatetime), ::core::mem::transmute_copy(&relmodifytime)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppqinfos, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQuery3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Properties() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcolproperties, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LookupQueue<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQuery3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, queueguid: *const super::Com::VARIANT, servicetypeguid: *const super::Com::VARIANT, label: *const super::Com::VARIANT, createtime: *const super::Com::VARIANT, modifytime: *const super::Com::VARIANT, relservicetype: *const super::Com::VARIANT, rellabel: *const super::Com::VARIANT, relcreatetime: *const super::Com::VARIANT, relmodifytime: *const super::Com::VARIANT, multicastaddress: *const super::Com::VARIANT, relmulticastaddress: *const super::Com::VARIANT, ppqinfos: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LookupQueue(
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
                    ::core::ptr::write(ppqinfos, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            LookupQueue_v2: LookupQueue_v2::<Identity, Impl, OFFSET>,
            Properties: Properties::<Identity, Impl, OFFSET>,
            LookupQueue: LookupQueue::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQQuery3 as ::windows::core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQQuery4_Impl: Sized + super::Com::IDispatch_Impl {
    fn LookupQueue_v2(&self, queueguid: *const super::Com::VARIANT, servicetypeguid: *const super::Com::VARIANT, label: *const super::Com::VARIANT, createtime: *const super::Com::VARIANT, modifytime: *const super::Com::VARIANT, relservicetype: *const super::Com::VARIANT, rellabel: *const super::Com::VARIANT, relcreatetime: *const super::Com::VARIANT, relmodifytime: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQQueueInfos4>;
    fn Properties(&self) -> ::windows::core::Result<super::Com::IDispatch>;
    fn LookupQueue(&self, queueguid: *const super::Com::VARIANT, servicetypeguid: *const super::Com::VARIANT, label: *const super::Com::VARIANT, createtime: *const super::Com::VARIANT, modifytime: *const super::Com::VARIANT, relservicetype: *const super::Com::VARIANT, rellabel: *const super::Com::VARIANT, relcreatetime: *const super::Com::VARIANT, relmodifytime: *const super::Com::VARIANT, multicastaddress: *const super::Com::VARIANT, relmulticastaddress: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQQueueInfos4>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IMSMQQuery4 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQQuery4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQuery4_Impl, const OFFSET: isize>() -> IMSMQQuery4_Vtbl {
        unsafe extern "system" fn LookupQueue_v2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQuery4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, queueguid: *const super::Com::VARIANT, servicetypeguid: *const super::Com::VARIANT, label: *const super::Com::VARIANT, createtime: *const super::Com::VARIANT, modifytime: *const super::Com::VARIANT, relservicetype: *const super::Com::VARIANT, rellabel: *const super::Com::VARIANT, relcreatetime: *const super::Com::VARIANT, relmodifytime: *const super::Com::VARIANT, ppqinfos: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LookupQueue_v2(::core::mem::transmute_copy(&queueguid), ::core::mem::transmute_copy(&servicetypeguid), ::core::mem::transmute_copy(&label), ::core::mem::transmute_copy(&createtime), ::core::mem::transmute_copy(&modifytime), ::core::mem::transmute_copy(&relservicetype), ::core::mem::transmute_copy(&rellabel), ::core::mem::transmute_copy(&relcreatetime), ::core::mem::transmute_copy(&relmodifytime)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppqinfos, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQuery4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Properties() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcolproperties, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LookupQueue<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQuery4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, queueguid: *const super::Com::VARIANT, servicetypeguid: *const super::Com::VARIANT, label: *const super::Com::VARIANT, createtime: *const super::Com::VARIANT, modifytime: *const super::Com::VARIANT, relservicetype: *const super::Com::VARIANT, rellabel: *const super::Com::VARIANT, relcreatetime: *const super::Com::VARIANT, relmodifytime: *const super::Com::VARIANT, multicastaddress: *const super::Com::VARIANT, relmulticastaddress: *const super::Com::VARIANT, ppqinfos: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LookupQueue(
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
                    ::core::ptr::write(ppqinfos, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            LookupQueue_v2: LookupQueue_v2::<Identity, Impl, OFFSET>,
            Properties: Properties::<Identity, Impl, OFFSET>,
            LookupQueue: LookupQueue::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQQuery4 as ::windows::core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
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
    fn EnableNotification(&self, event: ::core::option::Option<&IMSMQEvent>, cursor: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Reset(&self) -> ::windows::core::Result<()>;
    fn ReceiveCurrent(&self, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage>;
    fn PeekNext(&self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage>;
    fn PeekCurrent(&self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IMSMQQueue {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQQueue_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueue_Impl, const OFFSET: isize>() -> IMSMQQueue_Vtbl {
        unsafe extern "system" fn Access<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, placcess: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Access() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(placcess, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShareMode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plsharemode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ShareMode() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plsharemode, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueueInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinfo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.QueueInfo() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppqinfo, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Handle<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plhandle: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Handle() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plhandle, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsOpen<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisopen: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsOpen() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pisopen, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Close<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Close().into()
        }
        unsafe extern "system" fn Receive<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Receive(::core::mem::transmute_copy(&transaction), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmsg, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Peek<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Peek(::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmsg, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnableNotification<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, event: *mut ::core::ffi::c_void, cursor: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EnableNotification(::windows::core::from_raw_borrowed(&event), ::core::mem::transmute_copy(&cursor), ::core::mem::transmute_copy(&receivetimeout)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset().into()
        }
        unsafe extern "system" fn ReceiveCurrent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ReceiveCurrent(::core::mem::transmute_copy(&transaction), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmsg, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeekNext<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PeekNext(::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmsg, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeekCurrent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PeekCurrent(::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmsg, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
        iid == &<IMSMQQueue as ::windows::core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
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
    fn EnableNotification(&self, event: ::core::option::Option<&IMSMQEvent2>, cursor: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows::core::Result<()>;
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
impl ::windows::core::RuntimeName for IMSMQQueue2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQQueue2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueue2_Impl, const OFFSET: isize>() -> IMSMQQueue2_Vtbl {
        unsafe extern "system" fn Access<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueue2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, placcess: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Access() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(placcess, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShareMode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueue2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plsharemode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ShareMode() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plsharemode, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueueInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueue2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinfo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.QueueInfo() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppqinfo, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Handle<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueue2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plhandle: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Handle() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plhandle, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsOpen<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueue2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisopen: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsOpen() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pisopen, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Close<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueue2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Close().into()
        }
        unsafe extern "system" fn Receive_v1<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueue2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Receive_v1(::core::mem::transmute_copy(&transaction), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmsg, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Peek_v1<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueue2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Peek_v1(::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmsg, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnableNotification<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueue2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, event: *mut ::core::ffi::c_void, cursor: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EnableNotification(::windows::core::from_raw_borrowed(&event), ::core::mem::transmute_copy(&cursor), ::core::mem::transmute_copy(&receivetimeout)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueue2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset().into()
        }
        unsafe extern "system" fn ReceiveCurrent_v1<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueue2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ReceiveCurrent_v1(::core::mem::transmute_copy(&transaction), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmsg, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeekNext_v1<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueue2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PeekNext_v1(::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmsg, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeekCurrent_v1<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueue2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PeekCurrent_v1(::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmsg, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Receive<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueue2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Receive(::core::mem::transmute_copy(&transaction), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmsg, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Peek<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueue2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Peek(::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmsg, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReceiveCurrent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueue2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ReceiveCurrent(::core::mem::transmute_copy(&transaction), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmsg, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeekNext<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueue2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PeekNext(::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmsg, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeekCurrent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueue2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PeekCurrent(::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmsg, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueue2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Properties() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcolproperties, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
        iid == &<IMSMQQueue2 as ::windows::core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
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
    fn EnableNotification(&self, event: ::core::option::Option<&IMSMQEvent3>, cursor: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows::core::Result<()>;
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
    fn IsOpen2(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IMSMQQueue3 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQQueue3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueue3_Impl, const OFFSET: isize>() -> IMSMQQueue3_Vtbl {
        unsafe extern "system" fn Access<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueue3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, placcess: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Access() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(placcess, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShareMode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueue3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plsharemode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ShareMode() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plsharemode, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueueInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueue3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinfo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.QueueInfo() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppqinfo, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Handle<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueue3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plhandle: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Handle() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plhandle, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsOpen<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueue3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisopen: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsOpen() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pisopen, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Close<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueue3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Close().into()
        }
        unsafe extern "system" fn Receive_v1<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueue3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Receive_v1(::core::mem::transmute_copy(&transaction), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmsg, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Peek_v1<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueue3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Peek_v1(::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmsg, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnableNotification<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueue3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, event: *mut ::core::ffi::c_void, cursor: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EnableNotification(::windows::core::from_raw_borrowed(&event), ::core::mem::transmute_copy(&cursor), ::core::mem::transmute_copy(&receivetimeout)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueue3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset().into()
        }
        unsafe extern "system" fn ReceiveCurrent_v1<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueue3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ReceiveCurrent_v1(::core::mem::transmute_copy(&transaction), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmsg, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeekNext_v1<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueue3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PeekNext_v1(::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmsg, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeekCurrent_v1<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueue3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PeekCurrent_v1(::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmsg, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Receive<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueue3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Receive(::core::mem::transmute_copy(&transaction), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmsg, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Peek<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueue3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Peek(::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmsg, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReceiveCurrent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueue3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ReceiveCurrent(::core::mem::transmute_copy(&transaction), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmsg, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeekNext<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueue3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PeekNext(::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmsg, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeekCurrent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueue3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PeekCurrent(::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmsg, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueue3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Properties() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcolproperties, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Handle2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueue3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarhandle: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Handle2() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarhandle, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReceiveByLookupId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueue3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lookupid: super::Com::VARIANT, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ReceiveByLookupId(::core::mem::transmute(&lookupid), ::core::mem::transmute_copy(&transaction), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmsg, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReceiveNextByLookupId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueue3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lookupid: super::Com::VARIANT, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ReceiveNextByLookupId(::core::mem::transmute(&lookupid), ::core::mem::transmute_copy(&transaction), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmsg, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReceivePreviousByLookupId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueue3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lookupid: super::Com::VARIANT, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ReceivePreviousByLookupId(::core::mem::transmute(&lookupid), ::core::mem::transmute_copy(&transaction), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmsg, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReceiveFirstByLookupId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueue3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ReceiveFirstByLookupId(::core::mem::transmute_copy(&transaction), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmsg, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReceiveLastByLookupId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueue3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ReceiveLastByLookupId(::core::mem::transmute_copy(&transaction), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmsg, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeekByLookupId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueue3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lookupid: super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PeekByLookupId(::core::mem::transmute(&lookupid), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmsg, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeekNextByLookupId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueue3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lookupid: super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PeekNextByLookupId(::core::mem::transmute(&lookupid), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmsg, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeekPreviousByLookupId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueue3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lookupid: super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PeekPreviousByLookupId(::core::mem::transmute(&lookupid), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmsg, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeekFirstByLookupId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueue3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PeekFirstByLookupId(::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmsg, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeekLastByLookupId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueue3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PeekLastByLookupId(::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmsg, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Purge<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueue3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Purge().into()
        }
        unsafe extern "system" fn IsOpen2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueue3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisopen: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsOpen2() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pisopen, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
        iid == &<IMSMQQueue3 as ::windows::core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
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
    fn EnableNotification(&self, event: ::core::option::Option<&IMSMQEvent3>, cursor: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows::core::Result<()>;
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
    fn IsOpen2(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn ReceiveByLookupIdAllowPeek(&self, lookupid: &super::Com::VARIANT, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage4>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IMSMQQueue4 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQQueue4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueue4_Impl, const OFFSET: isize>() -> IMSMQQueue4_Vtbl {
        unsafe extern "system" fn Access<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueue4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, placcess: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Access() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(placcess, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShareMode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueue4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plsharemode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ShareMode() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plsharemode, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueueInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueue4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinfo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.QueueInfo() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppqinfo, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Handle<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueue4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plhandle: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Handle() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plhandle, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsOpen<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueue4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisopen: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsOpen() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pisopen, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Close<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueue4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Close().into()
        }
        unsafe extern "system" fn Receive_v1<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueue4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Receive_v1(::core::mem::transmute_copy(&transaction), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmsg, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Peek_v1<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueue4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Peek_v1(::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmsg, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnableNotification<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueue4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, event: *mut ::core::ffi::c_void, cursor: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EnableNotification(::windows::core::from_raw_borrowed(&event), ::core::mem::transmute_copy(&cursor), ::core::mem::transmute_copy(&receivetimeout)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueue4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset().into()
        }
        unsafe extern "system" fn ReceiveCurrent_v1<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueue4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ReceiveCurrent_v1(::core::mem::transmute_copy(&transaction), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmsg, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeekNext_v1<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueue4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PeekNext_v1(::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmsg, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeekCurrent_v1<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueue4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PeekCurrent_v1(::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmsg, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Receive<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueue4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Receive(::core::mem::transmute_copy(&transaction), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmsg, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Peek<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueue4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Peek(::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmsg, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReceiveCurrent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueue4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ReceiveCurrent(::core::mem::transmute_copy(&transaction), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmsg, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeekNext<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueue4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PeekNext(::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmsg, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeekCurrent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueue4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PeekCurrent(::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&receivetimeout), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmsg, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueue4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Properties() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcolproperties, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Handle2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueue4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarhandle: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Handle2() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarhandle, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReceiveByLookupId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueue4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lookupid: super::Com::VARIANT, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ReceiveByLookupId(::core::mem::transmute(&lookupid), ::core::mem::transmute_copy(&transaction), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmsg, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReceiveNextByLookupId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueue4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lookupid: super::Com::VARIANT, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ReceiveNextByLookupId(::core::mem::transmute(&lookupid), ::core::mem::transmute_copy(&transaction), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmsg, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReceivePreviousByLookupId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueue4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lookupid: super::Com::VARIANT, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ReceivePreviousByLookupId(::core::mem::transmute(&lookupid), ::core::mem::transmute_copy(&transaction), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmsg, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReceiveFirstByLookupId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueue4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ReceiveFirstByLookupId(::core::mem::transmute_copy(&transaction), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmsg, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReceiveLastByLookupId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueue4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ReceiveLastByLookupId(::core::mem::transmute_copy(&transaction), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmsg, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeekByLookupId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueue4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lookupid: super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PeekByLookupId(::core::mem::transmute(&lookupid), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmsg, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeekNextByLookupId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueue4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lookupid: super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PeekNextByLookupId(::core::mem::transmute(&lookupid), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmsg, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeekPreviousByLookupId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueue4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lookupid: super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PeekPreviousByLookupId(::core::mem::transmute(&lookupid), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmsg, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeekFirstByLookupId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueue4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PeekFirstByLookupId(::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmsg, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeekLastByLookupId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueue4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PeekLastByLookupId(::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmsg, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Purge<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueue4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Purge().into()
        }
        unsafe extern "system" fn IsOpen2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueue4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisopen: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsOpen2() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pisopen, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReceiveByLookupIdAllowPeek<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueue4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lookupid: super::Com::VARIANT, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ReceiveByLookupIdAllowPeek(::core::mem::transmute(&lookupid), ::core::mem::transmute_copy(&transaction), ::core::mem::transmute_copy(&wantdestinationqueue), ::core::mem::transmute_copy(&wantbody), ::core::mem::transmute_copy(&wantconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmsg, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
        iid == &<IMSMQQueue4 as ::windows::core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQQueueInfo_Impl: Sized + super::Com::IDispatch_Impl {
    fn QueueGuid(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn ServiceTypeGuid(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetServiceTypeGuid(&self, bstrguidservicetype: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn Label(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetLabel(&self, bstrlabel: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn PathName(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetPathName(&self, bstrpathname: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn FormatName(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetFormatName(&self, bstrformatname: &::windows::core::BSTR) -> ::windows::core::Result<()>;
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
impl ::windows::core::RuntimeName for IMSMQQueueInfo {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQQueueInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfo_Impl, const OFFSET: isize>() -> IMSMQQueueInfo_Vtbl {
        unsafe extern "system" fn QueueGuid<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrguidqueue: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.QueueGuid() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrguidqueue, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServiceTypeGuid<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrguidservicetype: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ServiceTypeGuid() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrguidservicetype, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetServiceTypeGuid<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrguidservicetype: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetServiceTypeGuid(::core::mem::transmute(&bstrguidservicetype)).into()
        }
        unsafe extern "system" fn Label<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrlabel: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Label() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrlabel, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLabel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrlabel: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetLabel(::core::mem::transmute(&bstrlabel)).into()
        }
        unsafe extern "system" fn PathName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrpathname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PathName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrpathname, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPathName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpathname: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPathName(::core::mem::transmute(&bstrpathname)).into()
        }
        unsafe extern "system" fn FormatName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrformatname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FormatName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrformatname, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFormatName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrformatname: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetFormatName(::core::mem::transmute(&bstrformatname)).into()
        }
        unsafe extern "system" fn IsTransactional<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pistransactional: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsTransactional() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pistransactional, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrivLevel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plprivlevel: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PrivLevel() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plprivlevel, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrivLevel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lprivlevel: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPrivLevel(::core::mem::transmute_copy(&lprivlevel)).into()
        }
        unsafe extern "system" fn Journal<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pljournal: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Journal() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pljournal, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetJournal<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ljournal: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetJournal(::core::mem::transmute_copy(&ljournal)).into()
        }
        unsafe extern "system" fn Quota<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plquota: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Quota() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plquota, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetQuota<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lquota: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetQuota(::core::mem::transmute_copy(&lquota)).into()
        }
        unsafe extern "system" fn BasePriority<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plbasepriority: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BasePriority() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plbasepriority, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBasePriority<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lbasepriority: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetBasePriority(::core::mem::transmute_copy(&lbasepriority)).into()
        }
        unsafe extern "system" fn CreateTime<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarcreatetime: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateTime() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarcreatetime, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ModifyTime<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarmodifytime: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ModifyTime() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarmodifytime, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Authenticate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plauthenticate: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Authenticate() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plauthenticate, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAuthenticate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lauthenticate: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAuthenticate(::core::mem::transmute_copy(&lauthenticate)).into()
        }
        unsafe extern "system" fn JournalQuota<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pljournalquota: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.JournalQuota() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pljournalquota, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetJournalQuota<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ljournalquota: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetJournalQuota(::core::mem::transmute_copy(&ljournalquota)).into()
        }
        unsafe extern "system" fn IsWorldReadable<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisworldreadable: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsWorldReadable() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pisworldreadable, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Create<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, istransactional: *const super::Com::VARIANT, isworldreadable: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Create(::core::mem::transmute_copy(&istransactional), ::core::mem::transmute_copy(&isworldreadable)).into()
        }
        unsafe extern "system" fn Delete<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Delete().into()
        }
        unsafe extern "system" fn Open<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, access: i32, sharemode: i32, ppq: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Open(::core::mem::transmute_copy(&access), ::core::mem::transmute_copy(&sharemode)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppq, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Refresh().into()
        }
        unsafe extern "system" fn Update<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Update().into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
        iid == &<IMSMQQueueInfo as ::windows::core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQQueueInfo2_Impl: Sized + super::Com::IDispatch_Impl {
    fn QueueGuid(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn ServiceTypeGuid(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetServiceTypeGuid(&self, bstrguidservicetype: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn Label(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetLabel(&self, bstrlabel: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn PathName(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetPathName(&self, bstrpathname: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn FormatName(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetFormatName(&self, bstrformatname: &::windows::core::BSTR) -> ::windows::core::Result<()>;
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
    fn PathNameDNS(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn Properties(&self) -> ::windows::core::Result<super::Com::IDispatch>;
    fn Security(&self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SetSecurity(&self, varsecurity: &super::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IMSMQQueueInfo2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQQueueInfo2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfo2_Impl, const OFFSET: isize>() -> IMSMQQueueInfo2_Vtbl {
        unsafe extern "system" fn QueueGuid<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrguidqueue: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.QueueGuid() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrguidqueue, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServiceTypeGuid<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrguidservicetype: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ServiceTypeGuid() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrguidservicetype, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetServiceTypeGuid<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrguidservicetype: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetServiceTypeGuid(::core::mem::transmute(&bstrguidservicetype)).into()
        }
        unsafe extern "system" fn Label<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrlabel: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Label() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrlabel, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLabel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrlabel: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetLabel(::core::mem::transmute(&bstrlabel)).into()
        }
        unsafe extern "system" fn PathName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrpathname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PathName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrpathname, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPathName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpathname: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPathName(::core::mem::transmute(&bstrpathname)).into()
        }
        unsafe extern "system" fn FormatName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrformatname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FormatName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrformatname, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFormatName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrformatname: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetFormatName(::core::mem::transmute(&bstrformatname)).into()
        }
        unsafe extern "system" fn IsTransactional<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pistransactional: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsTransactional() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pistransactional, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrivLevel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plprivlevel: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PrivLevel() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plprivlevel, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrivLevel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lprivlevel: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPrivLevel(::core::mem::transmute_copy(&lprivlevel)).into()
        }
        unsafe extern "system" fn Journal<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pljournal: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Journal() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pljournal, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetJournal<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ljournal: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetJournal(::core::mem::transmute_copy(&ljournal)).into()
        }
        unsafe extern "system" fn Quota<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plquota: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Quota() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plquota, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetQuota<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lquota: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetQuota(::core::mem::transmute_copy(&lquota)).into()
        }
        unsafe extern "system" fn BasePriority<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plbasepriority: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BasePriority() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plbasepriority, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBasePriority<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lbasepriority: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetBasePriority(::core::mem::transmute_copy(&lbasepriority)).into()
        }
        unsafe extern "system" fn CreateTime<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarcreatetime: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateTime() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarcreatetime, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ModifyTime<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarmodifytime: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ModifyTime() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarmodifytime, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Authenticate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plauthenticate: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Authenticate() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plauthenticate, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAuthenticate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lauthenticate: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAuthenticate(::core::mem::transmute_copy(&lauthenticate)).into()
        }
        unsafe extern "system" fn JournalQuota<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pljournalquota: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.JournalQuota() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pljournalquota, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetJournalQuota<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ljournalquota: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetJournalQuota(::core::mem::transmute_copy(&ljournalquota)).into()
        }
        unsafe extern "system" fn IsWorldReadable<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisworldreadable: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsWorldReadable() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pisworldreadable, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Create<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, istransactional: *const super::Com::VARIANT, isworldreadable: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Create(::core::mem::transmute_copy(&istransactional), ::core::mem::transmute_copy(&isworldreadable)).into()
        }
        unsafe extern "system" fn Delete<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Delete().into()
        }
        unsafe extern "system" fn Open<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, access: i32, sharemode: i32, ppq: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Open(::core::mem::transmute_copy(&access), ::core::mem::transmute_copy(&sharemode)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppq, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Refresh().into()
        }
        unsafe extern "system" fn Update<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Update().into()
        }
        unsafe extern "system" fn PathNameDNS<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrpathnamedns: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PathNameDNS() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrpathnamedns, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Properties() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcolproperties, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Security<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarsecurity: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Security() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarsecurity, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSecurity<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varsecurity: super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSecurity(::core::mem::transmute(&varsecurity)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
        iid == &<IMSMQQueueInfo2 as ::windows::core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQQueueInfo3_Impl: Sized + super::Com::IDispatch_Impl {
    fn QueueGuid(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn ServiceTypeGuid(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetServiceTypeGuid(&self, bstrguidservicetype: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn Label(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetLabel(&self, bstrlabel: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn PathName(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetPathName(&self, bstrpathname: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn FormatName(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetFormatName(&self, bstrformatname: &::windows::core::BSTR) -> ::windows::core::Result<()>;
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
    fn PathNameDNS(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn Properties(&self) -> ::windows::core::Result<super::Com::IDispatch>;
    fn Security(&self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SetSecurity(&self, varsecurity: &super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn IsTransactional2(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn IsWorldReadable2(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn MulticastAddress(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetMulticastAddress(&self, bstrmulticastaddress: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn ADsPath(&self) -> ::windows::core::Result<::windows::core::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IMSMQQueueInfo3 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQQueueInfo3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfo3_Impl, const OFFSET: isize>() -> IMSMQQueueInfo3_Vtbl {
        unsafe extern "system" fn QueueGuid<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrguidqueue: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.QueueGuid() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrguidqueue, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServiceTypeGuid<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrguidservicetype: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ServiceTypeGuid() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrguidservicetype, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetServiceTypeGuid<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrguidservicetype: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetServiceTypeGuid(::core::mem::transmute(&bstrguidservicetype)).into()
        }
        unsafe extern "system" fn Label<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrlabel: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Label() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrlabel, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLabel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrlabel: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetLabel(::core::mem::transmute(&bstrlabel)).into()
        }
        unsafe extern "system" fn PathName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrpathname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PathName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrpathname, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPathName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpathname: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPathName(::core::mem::transmute(&bstrpathname)).into()
        }
        unsafe extern "system" fn FormatName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrformatname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FormatName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrformatname, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFormatName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrformatname: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetFormatName(::core::mem::transmute(&bstrformatname)).into()
        }
        unsafe extern "system" fn IsTransactional<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pistransactional: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsTransactional() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pistransactional, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrivLevel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plprivlevel: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PrivLevel() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plprivlevel, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrivLevel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lprivlevel: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPrivLevel(::core::mem::transmute_copy(&lprivlevel)).into()
        }
        unsafe extern "system" fn Journal<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pljournal: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Journal() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pljournal, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetJournal<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ljournal: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetJournal(::core::mem::transmute_copy(&ljournal)).into()
        }
        unsafe extern "system" fn Quota<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plquota: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Quota() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plquota, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetQuota<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lquota: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetQuota(::core::mem::transmute_copy(&lquota)).into()
        }
        unsafe extern "system" fn BasePriority<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plbasepriority: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BasePriority() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plbasepriority, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBasePriority<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lbasepriority: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetBasePriority(::core::mem::transmute_copy(&lbasepriority)).into()
        }
        unsafe extern "system" fn CreateTime<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarcreatetime: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateTime() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarcreatetime, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ModifyTime<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarmodifytime: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ModifyTime() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarmodifytime, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Authenticate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plauthenticate: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Authenticate() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plauthenticate, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAuthenticate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lauthenticate: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAuthenticate(::core::mem::transmute_copy(&lauthenticate)).into()
        }
        unsafe extern "system" fn JournalQuota<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pljournalquota: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.JournalQuota() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pljournalquota, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetJournalQuota<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ljournalquota: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetJournalQuota(::core::mem::transmute_copy(&ljournalquota)).into()
        }
        unsafe extern "system" fn IsWorldReadable<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisworldreadable: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsWorldReadable() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pisworldreadable, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Create<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, istransactional: *const super::Com::VARIANT, isworldreadable: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Create(::core::mem::transmute_copy(&istransactional), ::core::mem::transmute_copy(&isworldreadable)).into()
        }
        unsafe extern "system" fn Delete<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Delete().into()
        }
        unsafe extern "system" fn Open<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, access: i32, sharemode: i32, ppq: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Open(::core::mem::transmute_copy(&access), ::core::mem::transmute_copy(&sharemode)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppq, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Refresh().into()
        }
        unsafe extern "system" fn Update<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Update().into()
        }
        unsafe extern "system" fn PathNameDNS<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrpathnamedns: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PathNameDNS() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrpathnamedns, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Properties() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcolproperties, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Security<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarsecurity: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Security() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarsecurity, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSecurity<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varsecurity: super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSecurity(::core::mem::transmute(&varsecurity)).into()
        }
        unsafe extern "system" fn IsTransactional2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pistransactional: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsTransactional2() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pistransactional, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsWorldReadable2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisworldreadable: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsWorldReadable2() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pisworldreadable, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MulticastAddress<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrmulticastaddress: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MulticastAddress() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrmulticastaddress, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMulticastAddress<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrmulticastaddress: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMulticastAddress(::core::mem::transmute(&bstrmulticastaddress)).into()
        }
        unsafe extern "system" fn ADsPath<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstradspath: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ADsPath() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstradspath, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
        iid == &<IMSMQQueueInfo3 as ::windows::core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQQueueInfo4_Impl: Sized + super::Com::IDispatch_Impl {
    fn QueueGuid(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn ServiceTypeGuid(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetServiceTypeGuid(&self, bstrguidservicetype: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn Label(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetLabel(&self, bstrlabel: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn PathName(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetPathName(&self, bstrpathname: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn FormatName(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetFormatName(&self, bstrformatname: &::windows::core::BSTR) -> ::windows::core::Result<()>;
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
    fn PathNameDNS(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn Properties(&self) -> ::windows::core::Result<super::Com::IDispatch>;
    fn Security(&self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SetSecurity(&self, varsecurity: &super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn IsTransactional2(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn IsWorldReadable2(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn MulticastAddress(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetMulticastAddress(&self, bstrmulticastaddress: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn ADsPath(&self) -> ::windows::core::Result<::windows::core::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IMSMQQueueInfo4 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQQueueInfo4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfo4_Impl, const OFFSET: isize>() -> IMSMQQueueInfo4_Vtbl {
        unsafe extern "system" fn QueueGuid<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrguidqueue: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.QueueGuid() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrguidqueue, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServiceTypeGuid<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrguidservicetype: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ServiceTypeGuid() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrguidservicetype, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetServiceTypeGuid<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrguidservicetype: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetServiceTypeGuid(::core::mem::transmute(&bstrguidservicetype)).into()
        }
        unsafe extern "system" fn Label<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrlabel: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Label() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrlabel, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLabel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrlabel: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetLabel(::core::mem::transmute(&bstrlabel)).into()
        }
        unsafe extern "system" fn PathName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrpathname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PathName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrpathname, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPathName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpathname: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPathName(::core::mem::transmute(&bstrpathname)).into()
        }
        unsafe extern "system" fn FormatName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrformatname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FormatName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrformatname, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFormatName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrformatname: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetFormatName(::core::mem::transmute(&bstrformatname)).into()
        }
        unsafe extern "system" fn IsTransactional<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pistransactional: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsTransactional() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pistransactional, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrivLevel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plprivlevel: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PrivLevel() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plprivlevel, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrivLevel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lprivlevel: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPrivLevel(::core::mem::transmute_copy(&lprivlevel)).into()
        }
        unsafe extern "system" fn Journal<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pljournal: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Journal() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pljournal, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetJournal<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ljournal: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetJournal(::core::mem::transmute_copy(&ljournal)).into()
        }
        unsafe extern "system" fn Quota<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plquota: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Quota() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plquota, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetQuota<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lquota: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetQuota(::core::mem::transmute_copy(&lquota)).into()
        }
        unsafe extern "system" fn BasePriority<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plbasepriority: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BasePriority() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plbasepriority, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBasePriority<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lbasepriority: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetBasePriority(::core::mem::transmute_copy(&lbasepriority)).into()
        }
        unsafe extern "system" fn CreateTime<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarcreatetime: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateTime() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarcreatetime, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ModifyTime<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarmodifytime: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ModifyTime() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarmodifytime, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Authenticate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plauthenticate: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Authenticate() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plauthenticate, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAuthenticate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lauthenticate: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAuthenticate(::core::mem::transmute_copy(&lauthenticate)).into()
        }
        unsafe extern "system" fn JournalQuota<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pljournalquota: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.JournalQuota() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pljournalquota, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetJournalQuota<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ljournalquota: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetJournalQuota(::core::mem::transmute_copy(&ljournalquota)).into()
        }
        unsafe extern "system" fn IsWorldReadable<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisworldreadable: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsWorldReadable() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pisworldreadable, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Create<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, istransactional: *const super::Com::VARIANT, isworldreadable: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Create(::core::mem::transmute_copy(&istransactional), ::core::mem::transmute_copy(&isworldreadable)).into()
        }
        unsafe extern "system" fn Delete<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Delete().into()
        }
        unsafe extern "system" fn Open<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, access: i32, sharemode: i32, ppq: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Open(::core::mem::transmute_copy(&access), ::core::mem::transmute_copy(&sharemode)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppq, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Refresh().into()
        }
        unsafe extern "system" fn Update<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Update().into()
        }
        unsafe extern "system" fn PathNameDNS<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrpathnamedns: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PathNameDNS() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrpathnamedns, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Properties() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcolproperties, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Security<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarsecurity: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Security() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarsecurity, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSecurity<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varsecurity: super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSecurity(::core::mem::transmute(&varsecurity)).into()
        }
        unsafe extern "system" fn IsTransactional2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pistransactional: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsTransactional2() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pistransactional, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsWorldReadable2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisworldreadable: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsWorldReadable2() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pisworldreadable, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MulticastAddress<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrmulticastaddress: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MulticastAddress() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrmulticastaddress, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMulticastAddress<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrmulticastaddress: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMulticastAddress(::core::mem::transmute(&bstrmulticastaddress)).into()
        }
        unsafe extern "system" fn ADsPath<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfo4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstradspath: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ADsPath() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstradspath, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
        iid == &<IMSMQQueueInfo4 as ::windows::core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQQueueInfos_Impl: Sized + super::Com::IDispatch_Impl {
    fn Reset(&self) -> ::windows::core::Result<()>;
    fn Next(&self) -> ::windows::core::Result<IMSMQQueueInfo>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IMSMQQueueInfos {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQQueueInfos_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfos_Impl, const OFFSET: isize>() -> IMSMQQueueInfos_Vtbl {
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfos_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset().into()
        }
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfos_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinfonext: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Next() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppqinfonext, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Reset: Reset::<Identity, Impl, OFFSET>,
            Next: Next::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQQueueInfos as ::windows::core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQQueueInfos2_Impl: Sized + super::Com::IDispatch_Impl {
    fn Reset(&self) -> ::windows::core::Result<()>;
    fn Next(&self) -> ::windows::core::Result<IMSMQQueueInfo2>;
    fn Properties(&self) -> ::windows::core::Result<super::Com::IDispatch>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IMSMQQueueInfos2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQQueueInfos2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfos2_Impl, const OFFSET: isize>() -> IMSMQQueueInfos2_Vtbl {
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfos2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset().into()
        }
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfos2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinfonext: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Next() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppqinfonext, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfos2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Properties() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcolproperties, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Reset: Reset::<Identity, Impl, OFFSET>,
            Next: Next::<Identity, Impl, OFFSET>,
            Properties: Properties::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQQueueInfos2 as ::windows::core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQQueueInfos3_Impl: Sized + super::Com::IDispatch_Impl {
    fn Reset(&self) -> ::windows::core::Result<()>;
    fn Next(&self) -> ::windows::core::Result<IMSMQQueueInfo3>;
    fn Properties(&self) -> ::windows::core::Result<super::Com::IDispatch>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IMSMQQueueInfos3 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQQueueInfos3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfos3_Impl, const OFFSET: isize>() -> IMSMQQueueInfos3_Vtbl {
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfos3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset().into()
        }
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfos3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinfonext: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Next() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppqinfonext, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfos3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Properties() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcolproperties, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Reset: Reset::<Identity, Impl, OFFSET>,
            Next: Next::<Identity, Impl, OFFSET>,
            Properties: Properties::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQQueueInfos3 as ::windows::core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQQueueInfos4_Impl: Sized + super::Com::IDispatch_Impl {
    fn Reset(&self) -> ::windows::core::Result<()>;
    fn Next(&self) -> ::windows::core::Result<IMSMQQueueInfo4>;
    fn Properties(&self) -> ::windows::core::Result<super::Com::IDispatch>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IMSMQQueueInfos4 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQQueueInfos4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfos4_Impl, const OFFSET: isize>() -> IMSMQQueueInfos4_Vtbl {
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfos4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset().into()
        }
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfos4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinfonext: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Next() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppqinfonext, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueInfos4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Properties() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcolproperties, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Reset: Reset::<Identity, Impl, OFFSET>,
            Next: Next::<Identity, Impl, OFFSET>,
            Properties: Properties::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQQueueInfos4 as ::windows::core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQQueueManagement_Impl: Sized + IMSMQManagement_Impl {
    fn JournalMessageCount(&self) -> ::windows::core::Result<i32>;
    fn BytesInJournal(&self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn EodGetReceiveInfo(&self) -> ::windows::core::Result<super::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IMSMQQueueManagement {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQQueueManagement_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueManagement_Impl, const OFFSET: isize>() -> IMSMQQueueManagement_Vtbl {
        unsafe extern "system" fn JournalMessageCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueManagement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pljournalmessagecount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.JournalMessageCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pljournalmessagecount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BytesInJournal<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueManagement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvbytesinjournal: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BytesInJournal() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvbytesinjournal, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EodGetReceiveInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQQueueManagement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvcollection: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EodGetReceiveInfo() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvcollection, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IMSMQManagement_Vtbl::new::<Identity, Impl, OFFSET>(),
            JournalMessageCount: JournalMessageCount::<Identity, Impl, OFFSET>,
            BytesInJournal: BytesInJournal::<Identity, Impl, OFFSET>,
            EodGetReceiveInfo: EodGetReceiveInfo::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQQueueManagement as ::windows::core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows::core::ComInterface>::IID || iid == &<IMSMQManagement as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQTransaction_Impl: Sized + super::Com::IDispatch_Impl {
    fn Transaction(&self) -> ::windows::core::Result<i32>;
    fn Commit(&self, fretaining: *const super::Com::VARIANT, grftc: *const super::Com::VARIANT, grfrm: *const super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Abort(&self, fretaining: *const super::Com::VARIANT, fasync: *const super::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IMSMQTransaction {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQTransaction_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQTransaction_Impl, const OFFSET: isize>() -> IMSMQTransaction_Vtbl {
        unsafe extern "system" fn Transaction<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQTransaction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pltransaction: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Transaction() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pltransaction, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Commit<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQTransaction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fretaining: *const super::Com::VARIANT, grftc: *const super::Com::VARIANT, grfrm: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Commit(::core::mem::transmute_copy(&fretaining), ::core::mem::transmute_copy(&grftc), ::core::mem::transmute_copy(&grfrm)).into()
        }
        unsafe extern "system" fn Abort<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQTransaction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fretaining: *const super::Com::VARIANT, fasync: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Abort(::core::mem::transmute_copy(&fretaining), ::core::mem::transmute_copy(&fasync)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Transaction: Transaction::<Identity, Impl, OFFSET>,
            Commit: Commit::<Identity, Impl, OFFSET>,
            Abort: Abort::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQTransaction as ::windows::core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQTransaction2_Impl: Sized + IMSMQTransaction_Impl {
    fn InitNew(&self, vartransaction: &super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Properties(&self) -> ::windows::core::Result<super::Com::IDispatch>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IMSMQTransaction2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQTransaction2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQTransaction2_Impl, const OFFSET: isize>() -> IMSMQTransaction2_Vtbl {
        unsafe extern "system" fn InitNew<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQTransaction2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vartransaction: super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InitNew(::core::mem::transmute(&vartransaction)).into()
        }
        unsafe extern "system" fn Properties<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQTransaction2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Properties() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcolproperties, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IMSMQTransaction_Vtbl::new::<Identity, Impl, OFFSET>(),
            InitNew: InitNew::<Identity, Impl, OFFSET>,
            Properties: Properties::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQTransaction2 as ::windows::core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows::core::ComInterface>::IID || iid == &<IMSMQTransaction as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQTransaction3_Impl: Sized + IMSMQTransaction2_Impl {
    fn ITransaction(&self) -> ::windows::core::Result<super::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IMSMQTransaction3 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQTransaction3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQTransaction3_Impl, const OFFSET: isize>() -> IMSMQTransaction3_Vtbl {
        unsafe extern "system" fn ITransaction<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQTransaction3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvaritransaction: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ITransaction() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvaritransaction, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: IMSMQTransaction2_Vtbl::new::<Identity, Impl, OFFSET>(), ITransaction: ITransaction::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQTransaction3 as ::windows::core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows::core::ComInterface>::IID || iid == &<IMSMQTransaction as ::windows::core::ComInterface>::IID || iid == &<IMSMQTransaction2 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQTransactionDispenser_Impl: Sized + super::Com::IDispatch_Impl {
    fn BeginTransaction(&self) -> ::windows::core::Result<IMSMQTransaction>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IMSMQTransactionDispenser {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQTransactionDispenser_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQTransactionDispenser_Impl, const OFFSET: isize>() -> IMSMQTransactionDispenser_Vtbl {
        unsafe extern "system" fn BeginTransaction<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQTransactionDispenser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptransaction: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginTransaction() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ptransaction, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(), BeginTransaction: BeginTransaction::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQTransactionDispenser as ::windows::core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQTransactionDispenser2_Impl: Sized + super::Com::IDispatch_Impl {
    fn BeginTransaction(&self) -> ::windows::core::Result<IMSMQTransaction2>;
    fn Properties(&self) -> ::windows::core::Result<super::Com::IDispatch>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IMSMQTransactionDispenser2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQTransactionDispenser2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQTransactionDispenser2_Impl, const OFFSET: isize>() -> IMSMQTransactionDispenser2_Vtbl {
        unsafe extern "system" fn BeginTransaction<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQTransactionDispenser2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptransaction: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginTransaction() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ptransaction, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQTransactionDispenser2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Properties() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcolproperties, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            BeginTransaction: BeginTransaction::<Identity, Impl, OFFSET>,
            Properties: Properties::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQTransactionDispenser2 as ::windows::core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMSMQTransactionDispenser3_Impl: Sized + super::Com::IDispatch_Impl {
    fn BeginTransaction(&self) -> ::windows::core::Result<IMSMQTransaction3>;
    fn Properties(&self) -> ::windows::core::Result<super::Com::IDispatch>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IMSMQTransactionDispenser3 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMSMQTransactionDispenser3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQTransactionDispenser3_Impl, const OFFSET: isize>() -> IMSMQTransactionDispenser3_Vtbl {
        unsafe extern "system" fn BeginTransaction<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQTransactionDispenser3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptransaction: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginTransaction() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ptransaction, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMSMQTransactionDispenser3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Properties() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcolproperties, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            BeginTransaction: BeginTransaction::<Identity, Impl, OFFSET>,
            Properties: Properties::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMSMQTransactionDispenser3 as ::windows::core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait _DMSMQEventEvents_Impl: Sized + super::Com::IDispatch_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for _DMSMQEventEvents {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl _DMSMQEventEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: _DMSMQEventEvents_Impl, const OFFSET: isize>() -> _DMSMQEventEvents_Vtbl {
        Self { base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<_DMSMQEventEvents as ::windows::core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
