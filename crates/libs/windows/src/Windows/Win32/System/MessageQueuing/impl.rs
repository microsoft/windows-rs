#[cfg(feature = "Win32_System_Com")]
pub trait IMSMQApplicationImpl: Sized + IDispatchImpl {
    fn MachineIdOfMachineName();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IMSMQApplication {
    const NAME: &'static str = "Windows.Win32.System.MessageQueuing.IMSMQApplication";
}
#[cfg(feature = "Win32_System_Com")]
impl IMSMQApplicationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQApplicationImpl, const OFFSET: isize>() -> IMSMQApplicationVtbl {
        unsafe extern "system" fn MachineIdOfMachineName<Impl: IMSMQApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, machinename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrguid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MachineIdOfMachineName(&*(&machinename as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pbstrguid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMSMQApplication>, ::windows::core::GetTrustLevel, MachineIdOfMachineName::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSMQApplication2Impl: Sized + IMSMQApplicationImpl + IDispatchImpl {
    fn RegisterCertificate();
    fn MachineNameOfMachineId();
    fn MSMQVersionMajor();
    fn MSMQVersionMinor();
    fn MSMQVersionBuild();
    fn IsDsEnabled();
    fn Properties();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IMSMQApplication2 {
    const NAME: &'static str = "Windows.Win32.System.MessageQueuing.IMSMQApplication2";
}
#[cfg(feature = "Win32_System_Com")]
impl IMSMQApplication2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQApplication2Impl, const OFFSET: isize>() -> IMSMQApplication2Vtbl {
        unsafe extern "system" fn RegisterCertificate<Impl: IMSMQApplication2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: *const super::Com::VARIANT, externalcertificate: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RegisterCertificate(&*(&flags as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), &*(&externalcertificate as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MachineNameOfMachineId<Impl: IMSMQApplication2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrguid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrmachinename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MachineNameOfMachineId(&*(&bstrguid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pbstrmachinename)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MSMQVersionMajor<Impl: IMSMQApplication2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psmsmqversionmajor: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MSMQVersionMajor(::core::mem::transmute_copy(&psmsmqversionmajor)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MSMQVersionMinor<Impl: IMSMQApplication2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psmsmqversionminor: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MSMQVersionMinor(::core::mem::transmute_copy(&psmsmqversionminor)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MSMQVersionBuild<Impl: IMSMQApplication2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psmsmqversionbuild: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MSMQVersionBuild(::core::mem::transmute_copy(&psmsmqversionbuild)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsDsEnabled<Impl: IMSMQApplication2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfisdsenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsDsEnabled(::core::mem::transmute_copy(&pfisdsenabled)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Impl: IMSMQApplication2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Properties(::core::mem::transmute_copy(&ppcolproperties)) {
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
            ::windows::core::GetRuntimeClassName::<IMSMQApplication2>,
            ::windows::core::GetTrustLevel,
            RegisterCertificate::<Impl, OFFSET>,
            MachineNameOfMachineId::<Impl, OFFSET>,
            MSMQVersionMajor::<Impl, OFFSET>,
            MSMQVersionMinor::<Impl, OFFSET>,
            MSMQVersionBuild::<Impl, OFFSET>,
            IsDsEnabled::<Impl, OFFSET>,
            Properties::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
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
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IMSMQApplication3 {
    const NAME: &'static str = "Windows.Win32.System.MessageQueuing.IMSMQApplication3";
}
#[cfg(feature = "Win32_System_Com")]
impl IMSMQApplication3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQApplication3Impl, const OFFSET: isize>() -> IMSMQApplication3Vtbl {
        unsafe extern "system" fn ActiveQueues<Impl: IMSMQApplication3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvactivequeues: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ActiveQueues(::core::mem::transmute_copy(&pvactivequeues)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrivateQueues<Impl: IMSMQApplication3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvprivatequeues: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PrivateQueues(::core::mem::transmute_copy(&pvprivatequeues)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DirectoryServiceServer<Impl: IMSMQApplication3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdirectoryserviceserver: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DirectoryServiceServer(::core::mem::transmute_copy(&pbstrdirectoryserviceserver)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsConnected<Impl: IMSMQApplication3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfisconnected: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsConnected(::core::mem::transmute_copy(&pfisconnected)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BytesInAllQueues<Impl: IMSMQApplication3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvbytesinallqueues: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BytesInAllQueues(::core::mem::transmute_copy(&pvbytesinallqueues)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMachine<Impl: IMSMQApplication3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrmachine: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetMachine(&*(&bstrmachine as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Machine<Impl: IMSMQApplication3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrmachine: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Machine(::core::mem::transmute_copy(&pbstrmachine)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Connect<Impl: IMSMQApplication3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Disconnect<Impl: IMSMQApplication3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Tidy<Impl: IMSMQApplication3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Tidy() {
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
            ::windows::core::GetRuntimeClassName::<IMSMQApplication3>,
            ::windows::core::GetTrustLevel,
            ActiveQueues::<Impl, OFFSET>,
            PrivateQueues::<Impl, OFFSET>,
            DirectoryServiceServer::<Impl, OFFSET>,
            IsConnected::<Impl, OFFSET>,
            BytesInAllQueues::<Impl, OFFSET>,
            SetMachine::<Impl, OFFSET>,
            Machine::<Impl, OFFSET>,
            Connect::<Impl, OFFSET>,
            Disconnect::<Impl, OFFSET>,
            Tidy::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSMQCollectionImpl: Sized + IDispatchImpl {
    fn Item();
    fn Count();
    fn _NewEnum();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IMSMQCollection {
    const NAME: &'static str = "Windows.Win32.System.MessageQueuing.IMSMQCollection";
}
#[cfg(feature = "Win32_System_Com")]
impl IMSMQCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQCollectionImpl, const OFFSET: isize>() -> IMSMQCollectionVtbl {
        unsafe extern "system" fn Item<Impl: IMSMQCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: *const super::Com::VARIANT, pvarret: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(&*(&index as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pvarret)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: IMSMQCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count(::core::mem::transmute_copy(&pcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: IMSMQCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum(::core::mem::transmute_copy(&ppunk)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMSMQCollection>, ::windows::core::GetTrustLevel, Item::<Impl, OFFSET>, Count::<Impl, OFFSET>, _NewEnum::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSMQCoordinatedTransactionDispenserImpl: Sized + IDispatchImpl {
    fn BeginTransaction();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IMSMQCoordinatedTransactionDispenser {
    const NAME: &'static str = "Windows.Win32.System.MessageQueuing.IMSMQCoordinatedTransactionDispenser";
}
#[cfg(feature = "Win32_System_Com")]
impl IMSMQCoordinatedTransactionDispenserVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQCoordinatedTransactionDispenserImpl, const OFFSET: isize>() -> IMSMQCoordinatedTransactionDispenserVtbl {
        unsafe extern "system" fn BeginTransaction<Impl: IMSMQCoordinatedTransactionDispenserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptransaction: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BeginTransaction(::core::mem::transmute_copy(&ptransaction)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMSMQCoordinatedTransactionDispenser>, ::windows::core::GetTrustLevel, BeginTransaction::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSMQCoordinatedTransactionDispenser2Impl: Sized + IDispatchImpl {
    fn BeginTransaction();
    fn Properties();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IMSMQCoordinatedTransactionDispenser2 {
    const NAME: &'static str = "Windows.Win32.System.MessageQueuing.IMSMQCoordinatedTransactionDispenser2";
}
#[cfg(feature = "Win32_System_Com")]
impl IMSMQCoordinatedTransactionDispenser2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQCoordinatedTransactionDispenser2Impl, const OFFSET: isize>() -> IMSMQCoordinatedTransactionDispenser2Vtbl {
        unsafe extern "system" fn BeginTransaction<Impl: IMSMQCoordinatedTransactionDispenser2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptransaction: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BeginTransaction(::core::mem::transmute_copy(&ptransaction)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Impl: IMSMQCoordinatedTransactionDispenser2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Properties(::core::mem::transmute_copy(&ppcolproperties)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMSMQCoordinatedTransactionDispenser2>, ::windows::core::GetTrustLevel, BeginTransaction::<Impl, OFFSET>, Properties::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSMQCoordinatedTransactionDispenser3Impl: Sized + IDispatchImpl {
    fn BeginTransaction();
    fn Properties();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IMSMQCoordinatedTransactionDispenser3 {
    const NAME: &'static str = "Windows.Win32.System.MessageQueuing.IMSMQCoordinatedTransactionDispenser3";
}
#[cfg(feature = "Win32_System_Com")]
impl IMSMQCoordinatedTransactionDispenser3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQCoordinatedTransactionDispenser3Impl, const OFFSET: isize>() -> IMSMQCoordinatedTransactionDispenser3Vtbl {
        unsafe extern "system" fn BeginTransaction<Impl: IMSMQCoordinatedTransactionDispenser3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptransaction: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BeginTransaction(::core::mem::transmute_copy(&ptransaction)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Impl: IMSMQCoordinatedTransactionDispenser3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Properties(::core::mem::transmute_copy(&ppcolproperties)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMSMQCoordinatedTransactionDispenser3>, ::windows::core::GetTrustLevel, BeginTransaction::<Impl, OFFSET>, Properties::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
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
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IMSMQDestination {
    const NAME: &'static str = "Windows.Win32.System.MessageQueuing.IMSMQDestination";
}
#[cfg(feature = "Win32_System_Com")]
impl IMSMQDestinationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQDestinationImpl, const OFFSET: isize>() -> IMSMQDestinationVtbl {
        unsafe extern "system" fn Open<Impl: IMSMQDestinationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Open() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Close<Impl: IMSMQDestinationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Close() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsOpen<Impl: IMSMQDestinationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfisopen: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsOpen(::core::mem::transmute_copy(&pfisopen)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IADs<Impl: IMSMQDestinationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppiads: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IADs(::core::mem::transmute_copy(&ppiads)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_IADs<Impl: IMSMQDestinationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piads: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).putref_IADs(&*(&piads as *const <super::Com::IDispatch as ::windows::core::Abi>::Abi as *const <super::Com::IDispatch as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ADsPath<Impl: IMSMQDestinationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstradspath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ADsPath(::core::mem::transmute_copy(&pbstradspath)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetADsPath<Impl: IMSMQDestinationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstradspath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetADsPath(&*(&bstradspath as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PathName<Impl: IMSMQDestinationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrpathname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PathName(::core::mem::transmute_copy(&pbstrpathname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPathName<Impl: IMSMQDestinationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpathname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetPathName(&*(&bstrpathname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FormatName<Impl: IMSMQDestinationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrformatname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FormatName(::core::mem::transmute_copy(&pbstrformatname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFormatName<Impl: IMSMQDestinationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrformatname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetFormatName(&*(&bstrformatname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Destinations<Impl: IMSMQDestinationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdestinations: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Destinations(::core::mem::transmute_copy(&ppdestinations)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_Destinations<Impl: IMSMQDestinationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdestinations: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).putref_Destinations(&*(&pdestinations as *const <super::Com::IDispatch as ::windows::core::Abi>::Abi as *const <super::Com::IDispatch as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Impl: IMSMQDestinationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Properties(::core::mem::transmute_copy(&ppcolproperties)) {
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
            ::windows::core::GetRuntimeClassName::<IMSMQDestination>,
            ::windows::core::GetTrustLevel,
            Open::<Impl, OFFSET>,
            Close::<Impl, OFFSET>,
            IsOpen::<Impl, OFFSET>,
            IADs::<Impl, OFFSET>,
            putref_IADs::<Impl, OFFSET>,
            ADsPath::<Impl, OFFSET>,
            SetADsPath::<Impl, OFFSET>,
            PathName::<Impl, OFFSET>,
            SetPathName::<Impl, OFFSET>,
            FormatName::<Impl, OFFSET>,
            SetFormatName::<Impl, OFFSET>,
            Destinations::<Impl, OFFSET>,
            putref_Destinations::<Impl, OFFSET>,
            Properties::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSMQEventImpl: Sized + IDispatchImpl {}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IMSMQEvent {
    const NAME: &'static str = "Windows.Win32.System.MessageQueuing.IMSMQEvent";
}
#[cfg(feature = "Win32_System_Com")]
impl IMSMQEventVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQEventImpl, const OFFSET: isize>() -> IMSMQEventVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMSMQEvent>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSMQEvent2Impl: Sized + IMSMQEventImpl + IDispatchImpl {
    fn Properties();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IMSMQEvent2 {
    const NAME: &'static str = "Windows.Win32.System.MessageQueuing.IMSMQEvent2";
}
#[cfg(feature = "Win32_System_Com")]
impl IMSMQEvent2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQEvent2Impl, const OFFSET: isize>() -> IMSMQEvent2Vtbl {
        unsafe extern "system" fn Properties<Impl: IMSMQEvent2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Properties(::core::mem::transmute_copy(&ppcolproperties)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMSMQEvent2>, ::windows::core::GetTrustLevel, Properties::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSMQEvent3Impl: Sized + IMSMQEvent2Impl + IMSMQEventImpl + IDispatchImpl {}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IMSMQEvent3 {
    const NAME: &'static str = "Windows.Win32.System.MessageQueuing.IMSMQEvent3";
}
#[cfg(feature = "Win32_System_Com")]
impl IMSMQEvent3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQEvent3Impl, const OFFSET: isize>() -> IMSMQEvent3Vtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMSMQEvent3>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "Win32_System_Com")]
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
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IMSMQManagement {
    const NAME: &'static str = "Windows.Win32.System.MessageQueuing.IMSMQManagement";
}
#[cfg(feature = "Win32_System_Com")]
impl IMSMQManagementVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQManagementImpl, const OFFSET: isize>() -> IMSMQManagementVtbl {
        unsafe extern "system" fn Init<Impl: IMSMQManagementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, machine: *const super::Com::VARIANT, pathname: *const super::Com::VARIANT, formatname: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Init(&*(&machine as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), &*(&pathname as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), &*(&formatname as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FormatName<Impl: IMSMQManagementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrformatname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FormatName(::core::mem::transmute_copy(&pbstrformatname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Machine<Impl: IMSMQManagementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrmachine: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Machine(::core::mem::transmute_copy(&pbstrmachine)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MessageCount<Impl: IMSMQManagementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmessagecount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MessageCount(::core::mem::transmute_copy(&plmessagecount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ForeignStatus<Impl: IMSMQManagementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plforeignstatus: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ForeignStatus(::core::mem::transmute_copy(&plforeignstatus)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueueType<Impl: IMSMQManagementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plqueuetype: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueueType(::core::mem::transmute_copy(&plqueuetype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsLocal<Impl: IMSMQManagementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfislocal: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsLocal(::core::mem::transmute_copy(&pfislocal)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransactionalStatus<Impl: IMSMQManagementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pltransactionalstatus: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TransactionalStatus(::core::mem::transmute_copy(&pltransactionalstatus)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BytesInQueue<Impl: IMSMQManagementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvbytesinqueue: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BytesInQueue(::core::mem::transmute_copy(&pvbytesinqueue)) {
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
            ::windows::core::GetRuntimeClassName::<IMSMQManagement>,
            ::windows::core::GetTrustLevel,
            Init::<Impl, OFFSET>,
            FormatName::<Impl, OFFSET>,
            Machine::<Impl, OFFSET>,
            MessageCount::<Impl, OFFSET>,
            ForeignStatus::<Impl, OFFSET>,
            QueueType::<Impl, OFFSET>,
            IsLocal::<Impl, OFFSET>,
            TransactionalStatus::<Impl, OFFSET>,
            BytesInQueue::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
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
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IMSMQMessage {
    const NAME: &'static str = "Windows.Win32.System.MessageQueuing.IMSMQMessage";
}
#[cfg(feature = "Win32_System_Com")]
impl IMSMQMessageVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessageImpl, const OFFSET: isize>() -> IMSMQMessageVtbl {
        unsafe extern "system" fn Class<Impl: IMSMQMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plclass: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Class(::core::mem::transmute_copy(&plclass)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrivLevel<Impl: IMSMQMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plprivlevel: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PrivLevel(::core::mem::transmute_copy(&plprivlevel)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrivLevel<Impl: IMSMQMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lprivlevel: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetPrivLevel(lprivlevel) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AuthLevel<Impl: IMSMQMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plauthlevel: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AuthLevel(::core::mem::transmute_copy(&plauthlevel)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAuthLevel<Impl: IMSMQMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lauthlevel: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAuthLevel(lauthlevel) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsAuthenticated<Impl: IMSMQMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisauthenticated: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsAuthenticated(::core::mem::transmute_copy(&pisauthenticated)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delivery<Impl: IMSMQMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pldelivery: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Delivery(::core::mem::transmute_copy(&pldelivery)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDelivery<Impl: IMSMQMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ldelivery: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetDelivery(ldelivery) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Trace<Impl: IMSMQMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pltrace: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Trace(::core::mem::transmute_copy(&pltrace)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTrace<Impl: IMSMQMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ltrace: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetTrace(ltrace) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Priority<Impl: IMSMQMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plpriority: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Priority(::core::mem::transmute_copy(&plpriority)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPriority<Impl: IMSMQMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpriority: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetPriority(lpriority) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Journal<Impl: IMSMQMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pljournal: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Journal(::core::mem::transmute_copy(&pljournal)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetJournal<Impl: IMSMQMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ljournal: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetJournal(ljournal) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResponseQueueInfo<Impl: IMSMQMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinforesponse: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResponseQueueInfo(::core::mem::transmute_copy(&ppqinforesponse)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_ResponseQueueInfo<Impl: IMSMQMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqinforesponse: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).putref_ResponseQueueInfo(&*(&pqinforesponse as *const <IMSMQQueueInfo as ::windows::core::Abi>::Abi as *const <IMSMQQueueInfo as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AppSpecific<Impl: IMSMQMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plappspecific: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AppSpecific(::core::mem::transmute_copy(&plappspecific)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAppSpecific<Impl: IMSMQMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lappspecific: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAppSpecific(lappspecific) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SourceMachineGuid<Impl: IMSMQMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrguidsrcmachine: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SourceMachineGuid(::core::mem::transmute_copy(&pbstrguidsrcmachine)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BodyLength<Impl: IMSMQMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcbbody: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BodyLength(::core::mem::transmute_copy(&pcbbody)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Body<Impl: IMSMQMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarbody: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Body(::core::mem::transmute_copy(&pvarbody)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBody<Impl: IMSMQMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varbody: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetBody(&*(&varbody as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AdminQueueInfo<Impl: IMSMQMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinfoadmin: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AdminQueueInfo(::core::mem::transmute_copy(&ppqinfoadmin)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_AdminQueueInfo<Impl: IMSMQMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqinfoadmin: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).putref_AdminQueueInfo(&*(&pqinfoadmin as *const <IMSMQQueueInfo as ::windows::core::Abi>::Abi as *const <IMSMQQueueInfo as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Id<Impl: IMSMQMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarmsgid: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Id(::core::mem::transmute_copy(&pvarmsgid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CorrelationId<Impl: IMSMQMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarmsgid: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CorrelationId(::core::mem::transmute_copy(&pvarmsgid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCorrelationId<Impl: IMSMQMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varmsgid: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetCorrelationId(&*(&varmsgid as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Ack<Impl: IMSMQMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plack: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Ack(::core::mem::transmute_copy(&plack)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAck<Impl: IMSMQMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lack: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAck(lack) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Label<Impl: IMSMQMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrlabel: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Label(::core::mem::transmute_copy(&pbstrlabel)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLabel<Impl: IMSMQMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrlabel: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetLabel(&*(&bstrlabel as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxTimeToReachQueue<Impl: IMSMQMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmaxtimetoreachqueue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxTimeToReachQueue(::core::mem::transmute_copy(&plmaxtimetoreachqueue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxTimeToReachQueue<Impl: IMSMQMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmaxtimetoreachqueue: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetMaxTimeToReachQueue(lmaxtimetoreachqueue) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxTimeToReceive<Impl: IMSMQMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmaxtimetoreceive: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxTimeToReceive(::core::mem::transmute_copy(&plmaxtimetoreceive)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxTimeToReceive<Impl: IMSMQMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmaxtimetoreceive: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetMaxTimeToReceive(lmaxtimetoreceive) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HashAlgorithm<Impl: IMSMQMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plhashalg: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HashAlgorithm(::core::mem::transmute_copy(&plhashalg)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHashAlgorithm<Impl: IMSMQMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lhashalg: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetHashAlgorithm(lhashalg) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EncryptAlgorithm<Impl: IMSMQMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plencryptalg: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EncryptAlgorithm(::core::mem::transmute_copy(&plencryptalg)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEncryptAlgorithm<Impl: IMSMQMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lencryptalg: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetEncryptAlgorithm(lencryptalg) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SentTime<Impl: IMSMQMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarsenttime: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SentTime(::core::mem::transmute_copy(&pvarsenttime)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ArrivedTime<Impl: IMSMQMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plarrivedtime: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ArrivedTime(::core::mem::transmute_copy(&plarrivedtime)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DestinationQueueInfo<Impl: IMSMQMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinfodest: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DestinationQueueInfo(::core::mem::transmute_copy(&ppqinfodest)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SenderCertificate<Impl: IMSMQMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarsendercert: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SenderCertificate(::core::mem::transmute_copy(&pvarsendercert)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSenderCertificate<Impl: IMSMQMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varsendercert: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSenderCertificate(&*(&varsendercert as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SenderId<Impl: IMSMQMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarsenderid: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SenderId(::core::mem::transmute_copy(&pvarsenderid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SenderIdType<Impl: IMSMQMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plsenderidtype: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SenderIdType(::core::mem::transmute_copy(&plsenderidtype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSenderIdType<Impl: IMSMQMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lsenderidtype: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSenderIdType(lsenderidtype) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Send<Impl: IMSMQMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, destinationqueue: ::windows::core::RawPtr, transaction: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Send(&*(&destinationqueue as *const <IMSMQQueue as ::windows::core::Abi>::Abi as *const <IMSMQQueue as ::windows::core::DefaultType>::DefaultType), &*(&transaction as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AttachCurrentSecurityContext<Impl: IMSMQMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AttachCurrentSecurityContext() {
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
            ::windows::core::GetRuntimeClassName::<IMSMQMessage>,
            ::windows::core::GetTrustLevel,
            Class::<Impl, OFFSET>,
            PrivLevel::<Impl, OFFSET>,
            SetPrivLevel::<Impl, OFFSET>,
            AuthLevel::<Impl, OFFSET>,
            SetAuthLevel::<Impl, OFFSET>,
            IsAuthenticated::<Impl, OFFSET>,
            Delivery::<Impl, OFFSET>,
            SetDelivery::<Impl, OFFSET>,
            Trace::<Impl, OFFSET>,
            SetTrace::<Impl, OFFSET>,
            Priority::<Impl, OFFSET>,
            SetPriority::<Impl, OFFSET>,
            Journal::<Impl, OFFSET>,
            SetJournal::<Impl, OFFSET>,
            ResponseQueueInfo::<Impl, OFFSET>,
            putref_ResponseQueueInfo::<Impl, OFFSET>,
            AppSpecific::<Impl, OFFSET>,
            SetAppSpecific::<Impl, OFFSET>,
            SourceMachineGuid::<Impl, OFFSET>,
            BodyLength::<Impl, OFFSET>,
            Body::<Impl, OFFSET>,
            SetBody::<Impl, OFFSET>,
            AdminQueueInfo::<Impl, OFFSET>,
            putref_AdminQueueInfo::<Impl, OFFSET>,
            Id::<Impl, OFFSET>,
            CorrelationId::<Impl, OFFSET>,
            SetCorrelationId::<Impl, OFFSET>,
            Ack::<Impl, OFFSET>,
            SetAck::<Impl, OFFSET>,
            Label::<Impl, OFFSET>,
            SetLabel::<Impl, OFFSET>,
            MaxTimeToReachQueue::<Impl, OFFSET>,
            SetMaxTimeToReachQueue::<Impl, OFFSET>,
            MaxTimeToReceive::<Impl, OFFSET>,
            SetMaxTimeToReceive::<Impl, OFFSET>,
            HashAlgorithm::<Impl, OFFSET>,
            SetHashAlgorithm::<Impl, OFFSET>,
            EncryptAlgorithm::<Impl, OFFSET>,
            SetEncryptAlgorithm::<Impl, OFFSET>,
            SentTime::<Impl, OFFSET>,
            ArrivedTime::<Impl, OFFSET>,
            DestinationQueueInfo::<Impl, OFFSET>,
            SenderCertificate::<Impl, OFFSET>,
            SetSenderCertificate::<Impl, OFFSET>,
            SenderId::<Impl, OFFSET>,
            SenderIdType::<Impl, OFFSET>,
            SetSenderIdType::<Impl, OFFSET>,
            Send::<Impl, OFFSET>,
            AttachCurrentSecurityContext::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
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
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IMSMQMessage2 {
    const NAME: &'static str = "Windows.Win32.System.MessageQueuing.IMSMQMessage2";
}
#[cfg(feature = "Win32_System_Com")]
impl IMSMQMessage2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage2Impl, const OFFSET: isize>() -> IMSMQMessage2Vtbl {
        unsafe extern "system" fn Class<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plclass: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Class(::core::mem::transmute_copy(&plclass)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrivLevel<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plprivlevel: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PrivLevel(::core::mem::transmute_copy(&plprivlevel)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrivLevel<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lprivlevel: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetPrivLevel(lprivlevel) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AuthLevel<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plauthlevel: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AuthLevel(::core::mem::transmute_copy(&plauthlevel)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAuthLevel<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lauthlevel: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAuthLevel(lauthlevel) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsAuthenticated<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisauthenticated: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsAuthenticated(::core::mem::transmute_copy(&pisauthenticated)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delivery<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pldelivery: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Delivery(::core::mem::transmute_copy(&pldelivery)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDelivery<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ldelivery: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetDelivery(ldelivery) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Trace<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pltrace: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Trace(::core::mem::transmute_copy(&pltrace)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTrace<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ltrace: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetTrace(ltrace) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Priority<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plpriority: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Priority(::core::mem::transmute_copy(&plpriority)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPriority<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpriority: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetPriority(lpriority) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Journal<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pljournal: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Journal(::core::mem::transmute_copy(&pljournal)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetJournal<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ljournal: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetJournal(ljournal) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResponseQueueInfo_v1<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinforesponse: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResponseQueueInfo_v1(::core::mem::transmute_copy(&ppqinforesponse)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_ResponseQueueInfo_v1<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqinforesponse: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).putref_ResponseQueueInfo_v1(&*(&pqinforesponse as *const <IMSMQQueueInfo as ::windows::core::Abi>::Abi as *const <IMSMQQueueInfo as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AppSpecific<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plappspecific: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AppSpecific(::core::mem::transmute_copy(&plappspecific)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAppSpecific<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lappspecific: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAppSpecific(lappspecific) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SourceMachineGuid<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrguidsrcmachine: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SourceMachineGuid(::core::mem::transmute_copy(&pbstrguidsrcmachine)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BodyLength<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcbbody: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BodyLength(::core::mem::transmute_copy(&pcbbody)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Body<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarbody: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Body(::core::mem::transmute_copy(&pvarbody)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBody<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varbody: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetBody(&*(&varbody as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AdminQueueInfo_v1<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinfoadmin: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AdminQueueInfo_v1(::core::mem::transmute_copy(&ppqinfoadmin)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_AdminQueueInfo_v1<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqinfoadmin: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).putref_AdminQueueInfo_v1(&*(&pqinfoadmin as *const <IMSMQQueueInfo as ::windows::core::Abi>::Abi as *const <IMSMQQueueInfo as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Id<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarmsgid: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Id(::core::mem::transmute_copy(&pvarmsgid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CorrelationId<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarmsgid: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CorrelationId(::core::mem::transmute_copy(&pvarmsgid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCorrelationId<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varmsgid: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetCorrelationId(&*(&varmsgid as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Ack<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plack: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Ack(::core::mem::transmute_copy(&plack)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAck<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lack: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAck(lack) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Label<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrlabel: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Label(::core::mem::transmute_copy(&pbstrlabel)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLabel<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrlabel: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetLabel(&*(&bstrlabel as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxTimeToReachQueue<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmaxtimetoreachqueue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxTimeToReachQueue(::core::mem::transmute_copy(&plmaxtimetoreachqueue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxTimeToReachQueue<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmaxtimetoreachqueue: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetMaxTimeToReachQueue(lmaxtimetoreachqueue) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxTimeToReceive<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmaxtimetoreceive: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxTimeToReceive(::core::mem::transmute_copy(&plmaxtimetoreceive)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxTimeToReceive<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmaxtimetoreceive: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetMaxTimeToReceive(lmaxtimetoreceive) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HashAlgorithm<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plhashalg: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HashAlgorithm(::core::mem::transmute_copy(&plhashalg)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHashAlgorithm<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lhashalg: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetHashAlgorithm(lhashalg) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EncryptAlgorithm<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plencryptalg: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EncryptAlgorithm(::core::mem::transmute_copy(&plencryptalg)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEncryptAlgorithm<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lencryptalg: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetEncryptAlgorithm(lencryptalg) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SentTime<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarsenttime: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SentTime(::core::mem::transmute_copy(&pvarsenttime)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ArrivedTime<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plarrivedtime: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ArrivedTime(::core::mem::transmute_copy(&plarrivedtime)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DestinationQueueInfo<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinfodest: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DestinationQueueInfo(::core::mem::transmute_copy(&ppqinfodest)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SenderCertificate<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarsendercert: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SenderCertificate(::core::mem::transmute_copy(&pvarsendercert)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSenderCertificate<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varsendercert: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSenderCertificate(&*(&varsendercert as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SenderId<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarsenderid: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SenderId(::core::mem::transmute_copy(&pvarsenderid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SenderIdType<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plsenderidtype: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SenderIdType(::core::mem::transmute_copy(&plsenderidtype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSenderIdType<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lsenderidtype: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSenderIdType(lsenderidtype) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Send<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, destinationqueue: ::windows::core::RawPtr, transaction: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Send(&*(&destinationqueue as *const <IMSMQQueue2 as ::windows::core::Abi>::Abi as *const <IMSMQQueue2 as ::windows::core::DefaultType>::DefaultType), &*(&transaction as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AttachCurrentSecurityContext<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AttachCurrentSecurityContext() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SenderVersion<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plsenderversion: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SenderVersion(::core::mem::transmute_copy(&plsenderversion)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Extension<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarextension: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Extension(::core::mem::transmute_copy(&pvarextension)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExtension<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varextension: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetExtension(&*(&varextension as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConnectorTypeGuid<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrguidconnectortype: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConnectorTypeGuid(::core::mem::transmute_copy(&pbstrguidconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetConnectorTypeGuid<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrguidconnectortype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetConnectorTypeGuid(&*(&bstrguidconnectortype as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransactionStatusQueueInfo<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinfoxactstatus: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TransactionStatusQueueInfo(::core::mem::transmute_copy(&ppqinfoxactstatus)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DestinationSymmetricKey<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvardestsymmkey: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DestinationSymmetricKey(::core::mem::transmute_copy(&pvardestsymmkey)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDestinationSymmetricKey<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vardestsymmkey: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetDestinationSymmetricKey(&*(&vardestsymmkey as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Signature<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarsignature: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Signature(::core::mem::transmute_copy(&pvarsignature)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSignature<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varsignature: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSignature(&*(&varsignature as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AuthenticationProviderType<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plauthprovtype: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AuthenticationProviderType(::core::mem::transmute_copy(&plauthprovtype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAuthenticationProviderType<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lauthprovtype: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAuthenticationProviderType(lauthprovtype) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AuthenticationProviderName<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrauthprovname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AuthenticationProviderName(::core::mem::transmute_copy(&pbstrauthprovname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAuthenticationProviderName<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrauthprovname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAuthenticationProviderName(&*(&bstrauthprovname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSenderId<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varsenderid: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSenderId(&*(&varsenderid as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MsgClass<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmsgclass: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MsgClass(::core::mem::transmute_copy(&plmsgclass)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMsgClass<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmsgclass: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetMsgClass(lmsgclass) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Properties(::core::mem::transmute_copy(&ppcolproperties)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransactionId<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarxactid: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TransactionId(::core::mem::transmute_copy(&pvarxactid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsFirstInTransaction<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisfirstinxact: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsFirstInTransaction(::core::mem::transmute_copy(&pisfirstinxact)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsLastInTransaction<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pislastinxact: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsLastInTransaction(::core::mem::transmute_copy(&pislastinxact)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResponseQueueInfo<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinforesponse: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResponseQueueInfo(::core::mem::transmute_copy(&ppqinforesponse)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_ResponseQueueInfo<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqinforesponse: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).putref_ResponseQueueInfo(&*(&pqinforesponse as *const <IMSMQQueueInfo2 as ::windows::core::Abi>::Abi as *const <IMSMQQueueInfo2 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AdminQueueInfo<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinfoadmin: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AdminQueueInfo(::core::mem::transmute_copy(&ppqinfoadmin)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_AdminQueueInfo<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqinfoadmin: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).putref_AdminQueueInfo(&*(&pqinfoadmin as *const <IMSMQQueueInfo2 as ::windows::core::Abi>::Abi as *const <IMSMQQueueInfo2 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReceivedAuthenticationLevel<Impl: IMSMQMessage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psreceivedauthenticationlevel: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReceivedAuthenticationLevel(::core::mem::transmute_copy(&psreceivedauthenticationlevel)) {
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
            ::windows::core::GetRuntimeClassName::<IMSMQMessage2>,
            ::windows::core::GetTrustLevel,
            Class::<Impl, OFFSET>,
            PrivLevel::<Impl, OFFSET>,
            SetPrivLevel::<Impl, OFFSET>,
            AuthLevel::<Impl, OFFSET>,
            SetAuthLevel::<Impl, OFFSET>,
            IsAuthenticated::<Impl, OFFSET>,
            Delivery::<Impl, OFFSET>,
            SetDelivery::<Impl, OFFSET>,
            Trace::<Impl, OFFSET>,
            SetTrace::<Impl, OFFSET>,
            Priority::<Impl, OFFSET>,
            SetPriority::<Impl, OFFSET>,
            Journal::<Impl, OFFSET>,
            SetJournal::<Impl, OFFSET>,
            ResponseQueueInfo_v1::<Impl, OFFSET>,
            putref_ResponseQueueInfo_v1::<Impl, OFFSET>,
            AppSpecific::<Impl, OFFSET>,
            SetAppSpecific::<Impl, OFFSET>,
            SourceMachineGuid::<Impl, OFFSET>,
            BodyLength::<Impl, OFFSET>,
            Body::<Impl, OFFSET>,
            SetBody::<Impl, OFFSET>,
            AdminQueueInfo_v1::<Impl, OFFSET>,
            putref_AdminQueueInfo_v1::<Impl, OFFSET>,
            Id::<Impl, OFFSET>,
            CorrelationId::<Impl, OFFSET>,
            SetCorrelationId::<Impl, OFFSET>,
            Ack::<Impl, OFFSET>,
            SetAck::<Impl, OFFSET>,
            Label::<Impl, OFFSET>,
            SetLabel::<Impl, OFFSET>,
            MaxTimeToReachQueue::<Impl, OFFSET>,
            SetMaxTimeToReachQueue::<Impl, OFFSET>,
            MaxTimeToReceive::<Impl, OFFSET>,
            SetMaxTimeToReceive::<Impl, OFFSET>,
            HashAlgorithm::<Impl, OFFSET>,
            SetHashAlgorithm::<Impl, OFFSET>,
            EncryptAlgorithm::<Impl, OFFSET>,
            SetEncryptAlgorithm::<Impl, OFFSET>,
            SentTime::<Impl, OFFSET>,
            ArrivedTime::<Impl, OFFSET>,
            DestinationQueueInfo::<Impl, OFFSET>,
            SenderCertificate::<Impl, OFFSET>,
            SetSenderCertificate::<Impl, OFFSET>,
            SenderId::<Impl, OFFSET>,
            SenderIdType::<Impl, OFFSET>,
            SetSenderIdType::<Impl, OFFSET>,
            Send::<Impl, OFFSET>,
            AttachCurrentSecurityContext::<Impl, OFFSET>,
            SenderVersion::<Impl, OFFSET>,
            Extension::<Impl, OFFSET>,
            SetExtension::<Impl, OFFSET>,
            ConnectorTypeGuid::<Impl, OFFSET>,
            SetConnectorTypeGuid::<Impl, OFFSET>,
            TransactionStatusQueueInfo::<Impl, OFFSET>,
            DestinationSymmetricKey::<Impl, OFFSET>,
            SetDestinationSymmetricKey::<Impl, OFFSET>,
            Signature::<Impl, OFFSET>,
            SetSignature::<Impl, OFFSET>,
            AuthenticationProviderType::<Impl, OFFSET>,
            SetAuthenticationProviderType::<Impl, OFFSET>,
            AuthenticationProviderName::<Impl, OFFSET>,
            SetAuthenticationProviderName::<Impl, OFFSET>,
            SetSenderId::<Impl, OFFSET>,
            MsgClass::<Impl, OFFSET>,
            SetMsgClass::<Impl, OFFSET>,
            Properties::<Impl, OFFSET>,
            TransactionId::<Impl, OFFSET>,
            IsFirstInTransaction::<Impl, OFFSET>,
            IsLastInTransaction::<Impl, OFFSET>,
            ResponseQueueInfo::<Impl, OFFSET>,
            putref_ResponseQueueInfo::<Impl, OFFSET>,
            AdminQueueInfo::<Impl, OFFSET>,
            putref_AdminQueueInfo::<Impl, OFFSET>,
            ReceivedAuthenticationLevel::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
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
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IMSMQMessage3 {
    const NAME: &'static str = "Windows.Win32.System.MessageQueuing.IMSMQMessage3";
}
#[cfg(feature = "Win32_System_Com")]
impl IMSMQMessage3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage3Impl, const OFFSET: isize>() -> IMSMQMessage3Vtbl {
        unsafe extern "system" fn Class<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plclass: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Class(::core::mem::transmute_copy(&plclass)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrivLevel<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plprivlevel: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PrivLevel(::core::mem::transmute_copy(&plprivlevel)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrivLevel<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lprivlevel: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetPrivLevel(lprivlevel) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AuthLevel<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plauthlevel: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AuthLevel(::core::mem::transmute_copy(&plauthlevel)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAuthLevel<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lauthlevel: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAuthLevel(lauthlevel) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsAuthenticated<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisauthenticated: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsAuthenticated(::core::mem::transmute_copy(&pisauthenticated)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delivery<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pldelivery: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Delivery(::core::mem::transmute_copy(&pldelivery)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDelivery<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ldelivery: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetDelivery(ldelivery) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Trace<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pltrace: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Trace(::core::mem::transmute_copy(&pltrace)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTrace<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ltrace: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetTrace(ltrace) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Priority<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plpriority: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Priority(::core::mem::transmute_copy(&plpriority)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPriority<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpriority: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetPriority(lpriority) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Journal<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pljournal: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Journal(::core::mem::transmute_copy(&pljournal)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetJournal<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ljournal: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetJournal(ljournal) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResponseQueueInfo_v1<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinforesponse: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResponseQueueInfo_v1(::core::mem::transmute_copy(&ppqinforesponse)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_ResponseQueueInfo_v1<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqinforesponse: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).putref_ResponseQueueInfo_v1(&*(&pqinforesponse as *const <IMSMQQueueInfo as ::windows::core::Abi>::Abi as *const <IMSMQQueueInfo as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AppSpecific<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plappspecific: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AppSpecific(::core::mem::transmute_copy(&plappspecific)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAppSpecific<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lappspecific: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAppSpecific(lappspecific) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SourceMachineGuid<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrguidsrcmachine: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SourceMachineGuid(::core::mem::transmute_copy(&pbstrguidsrcmachine)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BodyLength<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcbbody: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BodyLength(::core::mem::transmute_copy(&pcbbody)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Body<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarbody: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Body(::core::mem::transmute_copy(&pvarbody)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBody<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varbody: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetBody(&*(&varbody as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AdminQueueInfo_v1<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinfoadmin: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AdminQueueInfo_v1(::core::mem::transmute_copy(&ppqinfoadmin)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_AdminQueueInfo_v1<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqinfoadmin: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).putref_AdminQueueInfo_v1(&*(&pqinfoadmin as *const <IMSMQQueueInfo as ::windows::core::Abi>::Abi as *const <IMSMQQueueInfo as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Id<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarmsgid: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Id(::core::mem::transmute_copy(&pvarmsgid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CorrelationId<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarmsgid: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CorrelationId(::core::mem::transmute_copy(&pvarmsgid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCorrelationId<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varmsgid: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetCorrelationId(&*(&varmsgid as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Ack<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plack: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Ack(::core::mem::transmute_copy(&plack)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAck<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lack: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAck(lack) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Label<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrlabel: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Label(::core::mem::transmute_copy(&pbstrlabel)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLabel<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrlabel: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetLabel(&*(&bstrlabel as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxTimeToReachQueue<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmaxtimetoreachqueue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxTimeToReachQueue(::core::mem::transmute_copy(&plmaxtimetoreachqueue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxTimeToReachQueue<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmaxtimetoreachqueue: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetMaxTimeToReachQueue(lmaxtimetoreachqueue) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxTimeToReceive<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmaxtimetoreceive: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxTimeToReceive(::core::mem::transmute_copy(&plmaxtimetoreceive)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxTimeToReceive<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmaxtimetoreceive: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetMaxTimeToReceive(lmaxtimetoreceive) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HashAlgorithm<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plhashalg: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HashAlgorithm(::core::mem::transmute_copy(&plhashalg)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHashAlgorithm<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lhashalg: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetHashAlgorithm(lhashalg) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EncryptAlgorithm<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plencryptalg: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EncryptAlgorithm(::core::mem::transmute_copy(&plencryptalg)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEncryptAlgorithm<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lencryptalg: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetEncryptAlgorithm(lencryptalg) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SentTime<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarsenttime: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SentTime(::core::mem::transmute_copy(&pvarsenttime)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ArrivedTime<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plarrivedtime: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ArrivedTime(::core::mem::transmute_copy(&plarrivedtime)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DestinationQueueInfo<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinfodest: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DestinationQueueInfo(::core::mem::transmute_copy(&ppqinfodest)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SenderCertificate<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarsendercert: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SenderCertificate(::core::mem::transmute_copy(&pvarsendercert)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSenderCertificate<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varsendercert: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSenderCertificate(&*(&varsendercert as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SenderId<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarsenderid: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SenderId(::core::mem::transmute_copy(&pvarsenderid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SenderIdType<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plsenderidtype: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SenderIdType(::core::mem::transmute_copy(&plsenderidtype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSenderIdType<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lsenderidtype: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSenderIdType(lsenderidtype) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Send<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, destinationqueue: ::windows::core::RawPtr, transaction: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Send(&*(&destinationqueue as *const <super::Com::IDispatch as ::windows::core::Abi>::Abi as *const <super::Com::IDispatch as ::windows::core::DefaultType>::DefaultType), &*(&transaction as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AttachCurrentSecurityContext<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AttachCurrentSecurityContext() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SenderVersion<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plsenderversion: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SenderVersion(::core::mem::transmute_copy(&plsenderversion)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Extension<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarextension: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Extension(::core::mem::transmute_copy(&pvarextension)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExtension<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varextension: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetExtension(&*(&varextension as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConnectorTypeGuid<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrguidconnectortype: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConnectorTypeGuid(::core::mem::transmute_copy(&pbstrguidconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetConnectorTypeGuid<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrguidconnectortype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetConnectorTypeGuid(&*(&bstrguidconnectortype as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransactionStatusQueueInfo<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinfoxactstatus: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TransactionStatusQueueInfo(::core::mem::transmute_copy(&ppqinfoxactstatus)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DestinationSymmetricKey<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvardestsymmkey: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DestinationSymmetricKey(::core::mem::transmute_copy(&pvardestsymmkey)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDestinationSymmetricKey<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vardestsymmkey: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetDestinationSymmetricKey(&*(&vardestsymmkey as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Signature<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarsignature: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Signature(::core::mem::transmute_copy(&pvarsignature)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSignature<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varsignature: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSignature(&*(&varsignature as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AuthenticationProviderType<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plauthprovtype: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AuthenticationProviderType(::core::mem::transmute_copy(&plauthprovtype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAuthenticationProviderType<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lauthprovtype: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAuthenticationProviderType(lauthprovtype) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AuthenticationProviderName<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrauthprovname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AuthenticationProviderName(::core::mem::transmute_copy(&pbstrauthprovname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAuthenticationProviderName<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrauthprovname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAuthenticationProviderName(&*(&bstrauthprovname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSenderId<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varsenderid: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSenderId(&*(&varsenderid as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MsgClass<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmsgclass: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MsgClass(::core::mem::transmute_copy(&plmsgclass)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMsgClass<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmsgclass: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetMsgClass(lmsgclass) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Properties(::core::mem::transmute_copy(&ppcolproperties)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransactionId<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarxactid: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TransactionId(::core::mem::transmute_copy(&pvarxactid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsFirstInTransaction<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisfirstinxact: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsFirstInTransaction(::core::mem::transmute_copy(&pisfirstinxact)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsLastInTransaction<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pislastinxact: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsLastInTransaction(::core::mem::transmute_copy(&pislastinxact)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResponseQueueInfo_v2<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinforesponse: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResponseQueueInfo_v2(::core::mem::transmute_copy(&ppqinforesponse)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_ResponseQueueInfo_v2<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqinforesponse: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).putref_ResponseQueueInfo_v2(&*(&pqinforesponse as *const <IMSMQQueueInfo2 as ::windows::core::Abi>::Abi as *const <IMSMQQueueInfo2 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AdminQueueInfo_v2<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinfoadmin: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AdminQueueInfo_v2(::core::mem::transmute_copy(&ppqinfoadmin)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_AdminQueueInfo_v2<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqinfoadmin: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).putref_AdminQueueInfo_v2(&*(&pqinfoadmin as *const <IMSMQQueueInfo2 as ::windows::core::Abi>::Abi as *const <IMSMQQueueInfo2 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReceivedAuthenticationLevel<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psreceivedauthenticationlevel: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReceivedAuthenticationLevel(::core::mem::transmute_copy(&psreceivedauthenticationlevel)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResponseQueueInfo<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinforesponse: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResponseQueueInfo(::core::mem::transmute_copy(&ppqinforesponse)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_ResponseQueueInfo<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqinforesponse: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).putref_ResponseQueueInfo(&*(&pqinforesponse as *const <IMSMQQueueInfo3 as ::windows::core::Abi>::Abi as *const <IMSMQQueueInfo3 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AdminQueueInfo<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinfoadmin: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AdminQueueInfo(::core::mem::transmute_copy(&ppqinfoadmin)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_AdminQueueInfo<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqinfoadmin: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).putref_AdminQueueInfo(&*(&pqinfoadmin as *const <IMSMQQueueInfo3 as ::windows::core::Abi>::Abi as *const <IMSMQQueueInfo3 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResponseDestination<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdestresponse: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResponseDestination(::core::mem::transmute_copy(&ppdestresponse)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_ResponseDestination<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdestresponse: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).putref_ResponseDestination(&*(&pdestresponse as *const <super::Com::IDispatch as ::windows::core::Abi>::Abi as *const <super::Com::IDispatch as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Destination<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdestdestination: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Destination(::core::mem::transmute_copy(&ppdestdestination)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LookupId<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarlookupid: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LookupId(::core::mem::transmute_copy(&pvarlookupid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsAuthenticated2<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisauthenticated: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsAuthenticated2(::core::mem::transmute_copy(&pisauthenticated)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsFirstInTransaction2<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisfirstinxact: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsFirstInTransaction2(::core::mem::transmute_copy(&pisfirstinxact)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsLastInTransaction2<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pislastinxact: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsLastInTransaction2(::core::mem::transmute_copy(&pislastinxact)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AttachCurrentSecurityContext2<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AttachCurrentSecurityContext2() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SoapEnvelope<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrsoapenvelope: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SoapEnvelope(::core::mem::transmute_copy(&pbstrsoapenvelope)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CompoundMessage<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarcompoundmessage: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CompoundMessage(::core::mem::transmute_copy(&pvarcompoundmessage)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSoapHeader<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsoapheader: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSoapHeader(&*(&bstrsoapheader as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSoapBody<Impl: IMSMQMessage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsoapbody: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSoapBody(&*(&bstrsoapbody as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
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
            ::windows::core::GetRuntimeClassName::<IMSMQMessage3>,
            ::windows::core::GetTrustLevel,
            Class::<Impl, OFFSET>,
            PrivLevel::<Impl, OFFSET>,
            SetPrivLevel::<Impl, OFFSET>,
            AuthLevel::<Impl, OFFSET>,
            SetAuthLevel::<Impl, OFFSET>,
            IsAuthenticated::<Impl, OFFSET>,
            Delivery::<Impl, OFFSET>,
            SetDelivery::<Impl, OFFSET>,
            Trace::<Impl, OFFSET>,
            SetTrace::<Impl, OFFSET>,
            Priority::<Impl, OFFSET>,
            SetPriority::<Impl, OFFSET>,
            Journal::<Impl, OFFSET>,
            SetJournal::<Impl, OFFSET>,
            ResponseQueueInfo_v1::<Impl, OFFSET>,
            putref_ResponseQueueInfo_v1::<Impl, OFFSET>,
            AppSpecific::<Impl, OFFSET>,
            SetAppSpecific::<Impl, OFFSET>,
            SourceMachineGuid::<Impl, OFFSET>,
            BodyLength::<Impl, OFFSET>,
            Body::<Impl, OFFSET>,
            SetBody::<Impl, OFFSET>,
            AdminQueueInfo_v1::<Impl, OFFSET>,
            putref_AdminQueueInfo_v1::<Impl, OFFSET>,
            Id::<Impl, OFFSET>,
            CorrelationId::<Impl, OFFSET>,
            SetCorrelationId::<Impl, OFFSET>,
            Ack::<Impl, OFFSET>,
            SetAck::<Impl, OFFSET>,
            Label::<Impl, OFFSET>,
            SetLabel::<Impl, OFFSET>,
            MaxTimeToReachQueue::<Impl, OFFSET>,
            SetMaxTimeToReachQueue::<Impl, OFFSET>,
            MaxTimeToReceive::<Impl, OFFSET>,
            SetMaxTimeToReceive::<Impl, OFFSET>,
            HashAlgorithm::<Impl, OFFSET>,
            SetHashAlgorithm::<Impl, OFFSET>,
            EncryptAlgorithm::<Impl, OFFSET>,
            SetEncryptAlgorithm::<Impl, OFFSET>,
            SentTime::<Impl, OFFSET>,
            ArrivedTime::<Impl, OFFSET>,
            DestinationQueueInfo::<Impl, OFFSET>,
            SenderCertificate::<Impl, OFFSET>,
            SetSenderCertificate::<Impl, OFFSET>,
            SenderId::<Impl, OFFSET>,
            SenderIdType::<Impl, OFFSET>,
            SetSenderIdType::<Impl, OFFSET>,
            Send::<Impl, OFFSET>,
            AttachCurrentSecurityContext::<Impl, OFFSET>,
            SenderVersion::<Impl, OFFSET>,
            Extension::<Impl, OFFSET>,
            SetExtension::<Impl, OFFSET>,
            ConnectorTypeGuid::<Impl, OFFSET>,
            SetConnectorTypeGuid::<Impl, OFFSET>,
            TransactionStatusQueueInfo::<Impl, OFFSET>,
            DestinationSymmetricKey::<Impl, OFFSET>,
            SetDestinationSymmetricKey::<Impl, OFFSET>,
            Signature::<Impl, OFFSET>,
            SetSignature::<Impl, OFFSET>,
            AuthenticationProviderType::<Impl, OFFSET>,
            SetAuthenticationProviderType::<Impl, OFFSET>,
            AuthenticationProviderName::<Impl, OFFSET>,
            SetAuthenticationProviderName::<Impl, OFFSET>,
            SetSenderId::<Impl, OFFSET>,
            MsgClass::<Impl, OFFSET>,
            SetMsgClass::<Impl, OFFSET>,
            Properties::<Impl, OFFSET>,
            TransactionId::<Impl, OFFSET>,
            IsFirstInTransaction::<Impl, OFFSET>,
            IsLastInTransaction::<Impl, OFFSET>,
            ResponseQueueInfo_v2::<Impl, OFFSET>,
            putref_ResponseQueueInfo_v2::<Impl, OFFSET>,
            AdminQueueInfo_v2::<Impl, OFFSET>,
            putref_AdminQueueInfo_v2::<Impl, OFFSET>,
            ReceivedAuthenticationLevel::<Impl, OFFSET>,
            ResponseQueueInfo::<Impl, OFFSET>,
            putref_ResponseQueueInfo::<Impl, OFFSET>,
            AdminQueueInfo::<Impl, OFFSET>,
            putref_AdminQueueInfo::<Impl, OFFSET>,
            ResponseDestination::<Impl, OFFSET>,
            putref_ResponseDestination::<Impl, OFFSET>,
            Destination::<Impl, OFFSET>,
            LookupId::<Impl, OFFSET>,
            IsAuthenticated2::<Impl, OFFSET>,
            IsFirstInTransaction2::<Impl, OFFSET>,
            IsLastInTransaction2::<Impl, OFFSET>,
            AttachCurrentSecurityContext2::<Impl, OFFSET>,
            SoapEnvelope::<Impl, OFFSET>,
            CompoundMessage::<Impl, OFFSET>,
            SetSoapHeader::<Impl, OFFSET>,
            SetSoapBody::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
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
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IMSMQMessage4 {
    const NAME: &'static str = "Windows.Win32.System.MessageQueuing.IMSMQMessage4";
}
#[cfg(feature = "Win32_System_Com")]
impl IMSMQMessage4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQMessage4Impl, const OFFSET: isize>() -> IMSMQMessage4Vtbl {
        unsafe extern "system" fn Class<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plclass: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Class(::core::mem::transmute_copy(&plclass)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrivLevel<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plprivlevel: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PrivLevel(::core::mem::transmute_copy(&plprivlevel)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrivLevel<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lprivlevel: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetPrivLevel(lprivlevel) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AuthLevel<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plauthlevel: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AuthLevel(::core::mem::transmute_copy(&plauthlevel)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAuthLevel<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lauthlevel: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAuthLevel(lauthlevel) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsAuthenticated<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisauthenticated: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsAuthenticated(::core::mem::transmute_copy(&pisauthenticated)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delivery<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pldelivery: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Delivery(::core::mem::transmute_copy(&pldelivery)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDelivery<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ldelivery: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetDelivery(ldelivery) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Trace<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pltrace: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Trace(::core::mem::transmute_copy(&pltrace)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTrace<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ltrace: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetTrace(ltrace) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Priority<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plpriority: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Priority(::core::mem::transmute_copy(&plpriority)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPriority<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpriority: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetPriority(lpriority) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Journal<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pljournal: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Journal(::core::mem::transmute_copy(&pljournal)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetJournal<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ljournal: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetJournal(ljournal) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResponseQueueInfo_v1<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinforesponse: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResponseQueueInfo_v1(::core::mem::transmute_copy(&ppqinforesponse)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_ResponseQueueInfo_v1<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqinforesponse: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).putref_ResponseQueueInfo_v1(&*(&pqinforesponse as *const <IMSMQQueueInfo as ::windows::core::Abi>::Abi as *const <IMSMQQueueInfo as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AppSpecific<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plappspecific: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AppSpecific(::core::mem::transmute_copy(&plappspecific)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAppSpecific<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lappspecific: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAppSpecific(lappspecific) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SourceMachineGuid<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrguidsrcmachine: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SourceMachineGuid(::core::mem::transmute_copy(&pbstrguidsrcmachine)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BodyLength<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcbbody: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BodyLength(::core::mem::transmute_copy(&pcbbody)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Body<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarbody: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Body(::core::mem::transmute_copy(&pvarbody)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBody<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varbody: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetBody(&*(&varbody as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AdminQueueInfo_v1<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinfoadmin: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AdminQueueInfo_v1(::core::mem::transmute_copy(&ppqinfoadmin)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_AdminQueueInfo_v1<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqinfoadmin: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).putref_AdminQueueInfo_v1(&*(&pqinfoadmin as *const <IMSMQQueueInfo as ::windows::core::Abi>::Abi as *const <IMSMQQueueInfo as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Id<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarmsgid: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Id(::core::mem::transmute_copy(&pvarmsgid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CorrelationId<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarmsgid: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CorrelationId(::core::mem::transmute_copy(&pvarmsgid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCorrelationId<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varmsgid: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetCorrelationId(&*(&varmsgid as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Ack<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plack: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Ack(::core::mem::transmute_copy(&plack)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAck<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lack: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAck(lack) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Label<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrlabel: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Label(::core::mem::transmute_copy(&pbstrlabel)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLabel<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrlabel: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetLabel(&*(&bstrlabel as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxTimeToReachQueue<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmaxtimetoreachqueue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxTimeToReachQueue(::core::mem::transmute_copy(&plmaxtimetoreachqueue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxTimeToReachQueue<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmaxtimetoreachqueue: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetMaxTimeToReachQueue(lmaxtimetoreachqueue) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxTimeToReceive<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmaxtimetoreceive: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxTimeToReceive(::core::mem::transmute_copy(&plmaxtimetoreceive)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxTimeToReceive<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmaxtimetoreceive: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetMaxTimeToReceive(lmaxtimetoreceive) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HashAlgorithm<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plhashalg: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HashAlgorithm(::core::mem::transmute_copy(&plhashalg)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHashAlgorithm<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lhashalg: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetHashAlgorithm(lhashalg) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EncryptAlgorithm<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plencryptalg: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EncryptAlgorithm(::core::mem::transmute_copy(&plencryptalg)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEncryptAlgorithm<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lencryptalg: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetEncryptAlgorithm(lencryptalg) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SentTime<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarsenttime: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SentTime(::core::mem::transmute_copy(&pvarsenttime)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ArrivedTime<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plarrivedtime: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ArrivedTime(::core::mem::transmute_copy(&plarrivedtime)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DestinationQueueInfo<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinfodest: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DestinationQueueInfo(::core::mem::transmute_copy(&ppqinfodest)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SenderCertificate<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarsendercert: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SenderCertificate(::core::mem::transmute_copy(&pvarsendercert)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSenderCertificate<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varsendercert: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSenderCertificate(&*(&varsendercert as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SenderId<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarsenderid: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SenderId(::core::mem::transmute_copy(&pvarsenderid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SenderIdType<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plsenderidtype: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SenderIdType(::core::mem::transmute_copy(&plsenderidtype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSenderIdType<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lsenderidtype: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSenderIdType(lsenderidtype) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Send<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, destinationqueue: ::windows::core::RawPtr, transaction: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Send(&*(&destinationqueue as *const <super::Com::IDispatch as ::windows::core::Abi>::Abi as *const <super::Com::IDispatch as ::windows::core::DefaultType>::DefaultType), &*(&transaction as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AttachCurrentSecurityContext<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AttachCurrentSecurityContext() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SenderVersion<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plsenderversion: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SenderVersion(::core::mem::transmute_copy(&plsenderversion)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Extension<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarextension: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Extension(::core::mem::transmute_copy(&pvarextension)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExtension<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varextension: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetExtension(&*(&varextension as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConnectorTypeGuid<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrguidconnectortype: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConnectorTypeGuid(::core::mem::transmute_copy(&pbstrguidconnectortype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetConnectorTypeGuid<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrguidconnectortype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetConnectorTypeGuid(&*(&bstrguidconnectortype as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransactionStatusQueueInfo<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinfoxactstatus: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TransactionStatusQueueInfo(::core::mem::transmute_copy(&ppqinfoxactstatus)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DestinationSymmetricKey<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvardestsymmkey: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DestinationSymmetricKey(::core::mem::transmute_copy(&pvardestsymmkey)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDestinationSymmetricKey<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vardestsymmkey: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetDestinationSymmetricKey(&*(&vardestsymmkey as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Signature<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarsignature: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Signature(::core::mem::transmute_copy(&pvarsignature)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSignature<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varsignature: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSignature(&*(&varsignature as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AuthenticationProviderType<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plauthprovtype: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AuthenticationProviderType(::core::mem::transmute_copy(&plauthprovtype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAuthenticationProviderType<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lauthprovtype: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAuthenticationProviderType(lauthprovtype) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AuthenticationProviderName<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrauthprovname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AuthenticationProviderName(::core::mem::transmute_copy(&pbstrauthprovname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAuthenticationProviderName<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrauthprovname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAuthenticationProviderName(&*(&bstrauthprovname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSenderId<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varsenderid: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSenderId(&*(&varsenderid as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MsgClass<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmsgclass: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MsgClass(::core::mem::transmute_copy(&plmsgclass)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMsgClass<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmsgclass: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetMsgClass(lmsgclass) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Properties(::core::mem::transmute_copy(&ppcolproperties)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransactionId<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarxactid: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TransactionId(::core::mem::transmute_copy(&pvarxactid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsFirstInTransaction<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisfirstinxact: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsFirstInTransaction(::core::mem::transmute_copy(&pisfirstinxact)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsLastInTransaction<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pislastinxact: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsLastInTransaction(::core::mem::transmute_copy(&pislastinxact)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResponseQueueInfo_v2<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinforesponse: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResponseQueueInfo_v2(::core::mem::transmute_copy(&ppqinforesponse)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_ResponseQueueInfo_v2<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqinforesponse: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).putref_ResponseQueueInfo_v2(&*(&pqinforesponse as *const <IMSMQQueueInfo2 as ::windows::core::Abi>::Abi as *const <IMSMQQueueInfo2 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AdminQueueInfo_v2<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinfoadmin: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AdminQueueInfo_v2(::core::mem::transmute_copy(&ppqinfoadmin)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_AdminQueueInfo_v2<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqinfoadmin: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).putref_AdminQueueInfo_v2(&*(&pqinfoadmin as *const <IMSMQQueueInfo2 as ::windows::core::Abi>::Abi as *const <IMSMQQueueInfo2 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReceivedAuthenticationLevel<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psreceivedauthenticationlevel: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReceivedAuthenticationLevel(::core::mem::transmute_copy(&psreceivedauthenticationlevel)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResponseQueueInfo<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinforesponse: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResponseQueueInfo(::core::mem::transmute_copy(&ppqinforesponse)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_ResponseQueueInfo<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqinforesponse: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).putref_ResponseQueueInfo(&*(&pqinforesponse as *const <IMSMQQueueInfo4 as ::windows::core::Abi>::Abi as *const <IMSMQQueueInfo4 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AdminQueueInfo<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinfoadmin: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AdminQueueInfo(::core::mem::transmute_copy(&ppqinfoadmin)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_AdminQueueInfo<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqinfoadmin: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).putref_AdminQueueInfo(&*(&pqinfoadmin as *const <IMSMQQueueInfo4 as ::windows::core::Abi>::Abi as *const <IMSMQQueueInfo4 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResponseDestination<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdestresponse: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResponseDestination(::core::mem::transmute_copy(&ppdestresponse)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_ResponseDestination<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdestresponse: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).putref_ResponseDestination(&*(&pdestresponse as *const <super::Com::IDispatch as ::windows::core::Abi>::Abi as *const <super::Com::IDispatch as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Destination<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdestdestination: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Destination(::core::mem::transmute_copy(&ppdestdestination)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LookupId<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarlookupid: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LookupId(::core::mem::transmute_copy(&pvarlookupid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsAuthenticated2<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisauthenticated: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsAuthenticated2(::core::mem::transmute_copy(&pisauthenticated)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsFirstInTransaction2<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisfirstinxact: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsFirstInTransaction2(::core::mem::transmute_copy(&pisfirstinxact)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsLastInTransaction2<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pislastinxact: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsLastInTransaction2(::core::mem::transmute_copy(&pislastinxact)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AttachCurrentSecurityContext2<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AttachCurrentSecurityContext2() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SoapEnvelope<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrsoapenvelope: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SoapEnvelope(::core::mem::transmute_copy(&pbstrsoapenvelope)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CompoundMessage<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarcompoundmessage: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CompoundMessage(::core::mem::transmute_copy(&pvarcompoundmessage)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSoapHeader<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsoapheader: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSoapHeader(&*(&bstrsoapheader as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSoapBody<Impl: IMSMQMessage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsoapbody: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSoapBody(&*(&bstrsoapbody as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
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
            ::windows::core::GetRuntimeClassName::<IMSMQMessage4>,
            ::windows::core::GetTrustLevel,
            Class::<Impl, OFFSET>,
            PrivLevel::<Impl, OFFSET>,
            SetPrivLevel::<Impl, OFFSET>,
            AuthLevel::<Impl, OFFSET>,
            SetAuthLevel::<Impl, OFFSET>,
            IsAuthenticated::<Impl, OFFSET>,
            Delivery::<Impl, OFFSET>,
            SetDelivery::<Impl, OFFSET>,
            Trace::<Impl, OFFSET>,
            SetTrace::<Impl, OFFSET>,
            Priority::<Impl, OFFSET>,
            SetPriority::<Impl, OFFSET>,
            Journal::<Impl, OFFSET>,
            SetJournal::<Impl, OFFSET>,
            ResponseQueueInfo_v1::<Impl, OFFSET>,
            putref_ResponseQueueInfo_v1::<Impl, OFFSET>,
            AppSpecific::<Impl, OFFSET>,
            SetAppSpecific::<Impl, OFFSET>,
            SourceMachineGuid::<Impl, OFFSET>,
            BodyLength::<Impl, OFFSET>,
            Body::<Impl, OFFSET>,
            SetBody::<Impl, OFFSET>,
            AdminQueueInfo_v1::<Impl, OFFSET>,
            putref_AdminQueueInfo_v1::<Impl, OFFSET>,
            Id::<Impl, OFFSET>,
            CorrelationId::<Impl, OFFSET>,
            SetCorrelationId::<Impl, OFFSET>,
            Ack::<Impl, OFFSET>,
            SetAck::<Impl, OFFSET>,
            Label::<Impl, OFFSET>,
            SetLabel::<Impl, OFFSET>,
            MaxTimeToReachQueue::<Impl, OFFSET>,
            SetMaxTimeToReachQueue::<Impl, OFFSET>,
            MaxTimeToReceive::<Impl, OFFSET>,
            SetMaxTimeToReceive::<Impl, OFFSET>,
            HashAlgorithm::<Impl, OFFSET>,
            SetHashAlgorithm::<Impl, OFFSET>,
            EncryptAlgorithm::<Impl, OFFSET>,
            SetEncryptAlgorithm::<Impl, OFFSET>,
            SentTime::<Impl, OFFSET>,
            ArrivedTime::<Impl, OFFSET>,
            DestinationQueueInfo::<Impl, OFFSET>,
            SenderCertificate::<Impl, OFFSET>,
            SetSenderCertificate::<Impl, OFFSET>,
            SenderId::<Impl, OFFSET>,
            SenderIdType::<Impl, OFFSET>,
            SetSenderIdType::<Impl, OFFSET>,
            Send::<Impl, OFFSET>,
            AttachCurrentSecurityContext::<Impl, OFFSET>,
            SenderVersion::<Impl, OFFSET>,
            Extension::<Impl, OFFSET>,
            SetExtension::<Impl, OFFSET>,
            ConnectorTypeGuid::<Impl, OFFSET>,
            SetConnectorTypeGuid::<Impl, OFFSET>,
            TransactionStatusQueueInfo::<Impl, OFFSET>,
            DestinationSymmetricKey::<Impl, OFFSET>,
            SetDestinationSymmetricKey::<Impl, OFFSET>,
            Signature::<Impl, OFFSET>,
            SetSignature::<Impl, OFFSET>,
            AuthenticationProviderType::<Impl, OFFSET>,
            SetAuthenticationProviderType::<Impl, OFFSET>,
            AuthenticationProviderName::<Impl, OFFSET>,
            SetAuthenticationProviderName::<Impl, OFFSET>,
            SetSenderId::<Impl, OFFSET>,
            MsgClass::<Impl, OFFSET>,
            SetMsgClass::<Impl, OFFSET>,
            Properties::<Impl, OFFSET>,
            TransactionId::<Impl, OFFSET>,
            IsFirstInTransaction::<Impl, OFFSET>,
            IsLastInTransaction::<Impl, OFFSET>,
            ResponseQueueInfo_v2::<Impl, OFFSET>,
            putref_ResponseQueueInfo_v2::<Impl, OFFSET>,
            AdminQueueInfo_v2::<Impl, OFFSET>,
            putref_AdminQueueInfo_v2::<Impl, OFFSET>,
            ReceivedAuthenticationLevel::<Impl, OFFSET>,
            ResponseQueueInfo::<Impl, OFFSET>,
            putref_ResponseQueueInfo::<Impl, OFFSET>,
            AdminQueueInfo::<Impl, OFFSET>,
            putref_AdminQueueInfo::<Impl, OFFSET>,
            ResponseDestination::<Impl, OFFSET>,
            putref_ResponseDestination::<Impl, OFFSET>,
            Destination::<Impl, OFFSET>,
            LookupId::<Impl, OFFSET>,
            IsAuthenticated2::<Impl, OFFSET>,
            IsFirstInTransaction2::<Impl, OFFSET>,
            IsLastInTransaction2::<Impl, OFFSET>,
            AttachCurrentSecurityContext2::<Impl, OFFSET>,
            SoapEnvelope::<Impl, OFFSET>,
            CompoundMessage::<Impl, OFFSET>,
            SetSoapHeader::<Impl, OFFSET>,
            SetSoapBody::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSMQOutgoingQueueManagementImpl: Sized + IMSMQManagementImpl + IDispatchImpl {
    fn State();
    fn NextHops();
    fn EodGetSendInfo();
    fn Resume();
    fn Pause();
    fn EodResend();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IMSMQOutgoingQueueManagement {
    const NAME: &'static str = "Windows.Win32.System.MessageQueuing.IMSMQOutgoingQueueManagement";
}
#[cfg(feature = "Win32_System_Com")]
impl IMSMQOutgoingQueueManagementVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQOutgoingQueueManagementImpl, const OFFSET: isize>() -> IMSMQOutgoingQueueManagementVtbl {
        unsafe extern "system" fn State<Impl: IMSMQOutgoingQueueManagementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plstate: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).State(::core::mem::transmute_copy(&plstate)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NextHops<Impl: IMSMQOutgoingQueueManagementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvnexthops: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NextHops(::core::mem::transmute_copy(&pvnexthops)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EodGetSendInfo<Impl: IMSMQOutgoingQueueManagementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EodGetSendInfo(::core::mem::transmute_copy(&ppcollection)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Resume<Impl: IMSMQOutgoingQueueManagementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Resume() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Pause<Impl: IMSMQOutgoingQueueManagementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Pause() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EodResend<Impl: IMSMQOutgoingQueueManagementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EodResend() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMSMQOutgoingQueueManagement>, ::windows::core::GetTrustLevel, State::<Impl, OFFSET>, NextHops::<Impl, OFFSET>, EodGetSendInfo::<Impl, OFFSET>, Resume::<Impl, OFFSET>, Pause::<Impl, OFFSET>, EodResend::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSMQPrivateDestinationImpl: Sized + IDispatchImpl {
    fn Handle();
    fn SetHandle();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IMSMQPrivateDestination {
    const NAME: &'static str = "Windows.Win32.System.MessageQueuing.IMSMQPrivateDestination";
}
#[cfg(feature = "Win32_System_Com")]
impl IMSMQPrivateDestinationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQPrivateDestinationImpl, const OFFSET: isize>() -> IMSMQPrivateDestinationVtbl {
        unsafe extern "system" fn Handle<Impl: IMSMQPrivateDestinationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarhandle: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Handle(::core::mem::transmute_copy(&pvarhandle)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHandle<Impl: IMSMQPrivateDestinationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varhandle: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetHandle(&*(&varhandle as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMSMQPrivateDestination>, ::windows::core::GetTrustLevel, Handle::<Impl, OFFSET>, SetHandle::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSMQPrivateEventImpl: Sized + IDispatchImpl {
    fn Hwnd();
    fn FireArrivedEvent();
    fn FireArrivedErrorEvent();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IMSMQPrivateEvent {
    const NAME: &'static str = "Windows.Win32.System.MessageQueuing.IMSMQPrivateEvent";
}
#[cfg(feature = "Win32_System_Com")]
impl IMSMQPrivateEventVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQPrivateEventImpl, const OFFSET: isize>() -> IMSMQPrivateEventVtbl {
        unsafe extern "system" fn Hwnd<Impl: IMSMQPrivateEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phwnd: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Hwnd(::core::mem::transmute_copy(&phwnd)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FireArrivedEvent<Impl: IMSMQPrivateEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pq: ::windows::core::RawPtr, msgcursor: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FireArrivedEvent(&*(&pq as *const <IMSMQQueue as ::windows::core::Abi>::Abi as *const <IMSMQQueue as ::windows::core::DefaultType>::DefaultType), msgcursor) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FireArrivedErrorEvent<Impl: IMSMQPrivateEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pq: ::windows::core::RawPtr, hrstatus: ::windows::core::HRESULT, msgcursor: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FireArrivedErrorEvent(&*(&pq as *const <IMSMQQueue as ::windows::core::Abi>::Abi as *const <IMSMQQueue as ::windows::core::DefaultType>::DefaultType), hrstatus, msgcursor) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMSMQPrivateEvent>, ::windows::core::GetTrustLevel, Hwnd::<Impl, OFFSET>, FireArrivedEvent::<Impl, OFFSET>, FireArrivedErrorEvent::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSMQQueryImpl: Sized + IDispatchImpl {
    fn LookupQueue();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IMSMQQuery {
    const NAME: &'static str = "Windows.Win32.System.MessageQueuing.IMSMQQuery";
}
#[cfg(feature = "Win32_System_Com")]
impl IMSMQQueryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueryImpl, const OFFSET: isize>() -> IMSMQQueryVtbl {
        unsafe extern "system" fn LookupQueue<Impl: IMSMQQueryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, queueguid: *const super::Com::VARIANT, servicetypeguid: *const super::Com::VARIANT, label: *const super::Com::VARIANT, createtime: *const super::Com::VARIANT, modifytime: *const super::Com::VARIANT, relservicetype: *const super::Com::VARIANT, rellabel: *const super::Com::VARIANT, relcreatetime: *const super::Com::VARIANT, relmodifytime: *const super::Com::VARIANT, ppqinfos: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LookupQueue(
                &*(&queueguid as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&servicetypeguid as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&label as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&createtime as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&modifytime as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&relservicetype as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&rellabel as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&relcreatetime as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&relmodifytime as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppqinfos),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMSMQQuery>, ::windows::core::GetTrustLevel, LookupQueue::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSMQQuery2Impl: Sized + IDispatchImpl {
    fn LookupQueue();
    fn Properties();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IMSMQQuery2 {
    const NAME: &'static str = "Windows.Win32.System.MessageQueuing.IMSMQQuery2";
}
#[cfg(feature = "Win32_System_Com")]
impl IMSMQQuery2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQuery2Impl, const OFFSET: isize>() -> IMSMQQuery2Vtbl {
        unsafe extern "system" fn LookupQueue<Impl: IMSMQQuery2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, queueguid: *const super::Com::VARIANT, servicetypeguid: *const super::Com::VARIANT, label: *const super::Com::VARIANT, createtime: *const super::Com::VARIANT, modifytime: *const super::Com::VARIANT, relservicetype: *const super::Com::VARIANT, rellabel: *const super::Com::VARIANT, relcreatetime: *const super::Com::VARIANT, relmodifytime: *const super::Com::VARIANT, ppqinfos: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LookupQueue(
                &*(&queueguid as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&servicetypeguid as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&label as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&createtime as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&modifytime as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&relservicetype as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&rellabel as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&relcreatetime as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&relmodifytime as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppqinfos),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Impl: IMSMQQuery2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Properties(::core::mem::transmute_copy(&ppcolproperties)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMSMQQuery2>, ::windows::core::GetTrustLevel, LookupQueue::<Impl, OFFSET>, Properties::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSMQQuery3Impl: Sized + IDispatchImpl {
    fn LookupQueue_v2();
    fn Properties();
    fn LookupQueue();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IMSMQQuery3 {
    const NAME: &'static str = "Windows.Win32.System.MessageQueuing.IMSMQQuery3";
}
#[cfg(feature = "Win32_System_Com")]
impl IMSMQQuery3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQuery3Impl, const OFFSET: isize>() -> IMSMQQuery3Vtbl {
        unsafe extern "system" fn LookupQueue_v2<Impl: IMSMQQuery3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, queueguid: *const super::Com::VARIANT, servicetypeguid: *const super::Com::VARIANT, label: *const super::Com::VARIANT, createtime: *const super::Com::VARIANT, modifytime: *const super::Com::VARIANT, relservicetype: *const super::Com::VARIANT, rellabel: *const super::Com::VARIANT, relcreatetime: *const super::Com::VARIANT, relmodifytime: *const super::Com::VARIANT, ppqinfos: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LookupQueue_v2(
                &*(&queueguid as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&servicetypeguid as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&label as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&createtime as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&modifytime as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&relservicetype as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&rellabel as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&relcreatetime as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&relmodifytime as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppqinfos),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Impl: IMSMQQuery3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Properties(::core::mem::transmute_copy(&ppcolproperties)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LookupQueue<Impl: IMSMQQuery3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, queueguid: *const super::Com::VARIANT, servicetypeguid: *const super::Com::VARIANT, label: *const super::Com::VARIANT, createtime: *const super::Com::VARIANT, modifytime: *const super::Com::VARIANT, relservicetype: *const super::Com::VARIANT, rellabel: *const super::Com::VARIANT, relcreatetime: *const super::Com::VARIANT, relmodifytime: *const super::Com::VARIANT, multicastaddress: *const super::Com::VARIANT, relmulticastaddress: *const super::Com::VARIANT, ppqinfos: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LookupQueue(
                &*(&queueguid as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&servicetypeguid as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&label as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&createtime as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&modifytime as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&relservicetype as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&rellabel as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&relcreatetime as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&relmodifytime as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&multicastaddress as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&relmulticastaddress as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppqinfos),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMSMQQuery3>, ::windows::core::GetTrustLevel, LookupQueue_v2::<Impl, OFFSET>, Properties::<Impl, OFFSET>, LookupQueue::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSMQQuery4Impl: Sized + IDispatchImpl {
    fn LookupQueue_v2();
    fn Properties();
    fn LookupQueue();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IMSMQQuery4 {
    const NAME: &'static str = "Windows.Win32.System.MessageQueuing.IMSMQQuery4";
}
#[cfg(feature = "Win32_System_Com")]
impl IMSMQQuery4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQuery4Impl, const OFFSET: isize>() -> IMSMQQuery4Vtbl {
        unsafe extern "system" fn LookupQueue_v2<Impl: IMSMQQuery4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, queueguid: *const super::Com::VARIANT, servicetypeguid: *const super::Com::VARIANT, label: *const super::Com::VARIANT, createtime: *const super::Com::VARIANT, modifytime: *const super::Com::VARIANT, relservicetype: *const super::Com::VARIANT, rellabel: *const super::Com::VARIANT, relcreatetime: *const super::Com::VARIANT, relmodifytime: *const super::Com::VARIANT, ppqinfos: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LookupQueue_v2(
                &*(&queueguid as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&servicetypeguid as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&label as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&createtime as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&modifytime as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&relservicetype as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&rellabel as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&relcreatetime as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&relmodifytime as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppqinfos),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Impl: IMSMQQuery4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Properties(::core::mem::transmute_copy(&ppcolproperties)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LookupQueue<Impl: IMSMQQuery4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, queueguid: *const super::Com::VARIANT, servicetypeguid: *const super::Com::VARIANT, label: *const super::Com::VARIANT, createtime: *const super::Com::VARIANT, modifytime: *const super::Com::VARIANT, relservicetype: *const super::Com::VARIANT, rellabel: *const super::Com::VARIANT, relcreatetime: *const super::Com::VARIANT, relmodifytime: *const super::Com::VARIANT, multicastaddress: *const super::Com::VARIANT, relmulticastaddress: *const super::Com::VARIANT, ppqinfos: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LookupQueue(
                &*(&queueguid as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&servicetypeguid as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&label as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&createtime as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&modifytime as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&relservicetype as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&rellabel as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&relcreatetime as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&relmodifytime as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&multicastaddress as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&relmulticastaddress as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppqinfos),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMSMQQuery4>, ::windows::core::GetTrustLevel, LookupQueue_v2::<Impl, OFFSET>, Properties::<Impl, OFFSET>, LookupQueue::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
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
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IMSMQQueue {
    const NAME: &'static str = "Windows.Win32.System.MessageQueuing.IMSMQQueue";
}
#[cfg(feature = "Win32_System_Com")]
impl IMSMQQueueVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueImpl, const OFFSET: isize>() -> IMSMQQueueVtbl {
        unsafe extern "system" fn Access<Impl: IMSMQQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, placcess: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Access(::core::mem::transmute_copy(&placcess)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShareMode<Impl: IMSMQQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plsharemode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShareMode(::core::mem::transmute_copy(&plsharemode)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueueInfo<Impl: IMSMQQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueueInfo(::core::mem::transmute_copy(&ppqinfo)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Handle<Impl: IMSMQQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plhandle: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Handle(::core::mem::transmute_copy(&plhandle)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsOpen<Impl: IMSMQQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisopen: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsOpen(::core::mem::transmute_copy(&pisopen)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Close<Impl: IMSMQQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Close() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Receive<Impl: IMSMQQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Receive(
                &*(&transaction as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&wantdestinationqueue as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&wantbody as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&receivetimeout as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppmsg),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Peek<Impl: IMSMQQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Peek(
                &*(&wantdestinationqueue as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&wantbody as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&receivetimeout as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppmsg),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnableNotification<Impl: IMSMQQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, event: ::windows::core::RawPtr, cursor: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnableNotification(&*(&event as *const <IMSMQEvent as ::windows::core::Abi>::Abi as *const <IMSMQEvent as ::windows::core::DefaultType>::DefaultType), &*(&cursor as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), &*(&receivetimeout as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IMSMQQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ReceiveCurrent<Impl: IMSMQQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReceiveCurrent(
                &*(&transaction as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&wantdestinationqueue as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&wantbody as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&receivetimeout as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppmsg),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeekNext<Impl: IMSMQQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PeekNext(
                &*(&wantdestinationqueue as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&wantbody as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&receivetimeout as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppmsg),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeekCurrent<Impl: IMSMQQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PeekCurrent(
                &*(&wantdestinationqueue as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&wantbody as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&receivetimeout as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppmsg),
            ) {
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
            ::windows::core::GetRuntimeClassName::<IMSMQQueue>,
            ::windows::core::GetTrustLevel,
            Access::<Impl, OFFSET>,
            ShareMode::<Impl, OFFSET>,
            QueueInfo::<Impl, OFFSET>,
            Handle::<Impl, OFFSET>,
            IsOpen::<Impl, OFFSET>,
            Close::<Impl, OFFSET>,
            Receive::<Impl, OFFSET>,
            Peek::<Impl, OFFSET>,
            EnableNotification::<Impl, OFFSET>,
            Reset::<Impl, OFFSET>,
            ReceiveCurrent::<Impl, OFFSET>,
            PeekNext::<Impl, OFFSET>,
            PeekCurrent::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
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
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IMSMQQueue2 {
    const NAME: &'static str = "Windows.Win32.System.MessageQueuing.IMSMQQueue2";
}
#[cfg(feature = "Win32_System_Com")]
impl IMSMQQueue2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueue2Impl, const OFFSET: isize>() -> IMSMQQueue2Vtbl {
        unsafe extern "system" fn Access<Impl: IMSMQQueue2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, placcess: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Access(::core::mem::transmute_copy(&placcess)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShareMode<Impl: IMSMQQueue2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plsharemode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShareMode(::core::mem::transmute_copy(&plsharemode)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueueInfo<Impl: IMSMQQueue2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueueInfo(::core::mem::transmute_copy(&ppqinfo)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Handle<Impl: IMSMQQueue2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plhandle: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Handle(::core::mem::transmute_copy(&plhandle)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsOpen<Impl: IMSMQQueue2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisopen: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsOpen(::core::mem::transmute_copy(&pisopen)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Close<Impl: IMSMQQueue2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Close() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Receive_v1<Impl: IMSMQQueue2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Receive_v1(
                &*(&transaction as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&wantdestinationqueue as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&wantbody as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&receivetimeout as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppmsg),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Peek_v1<Impl: IMSMQQueue2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Peek_v1(
                &*(&wantdestinationqueue as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&wantbody as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&receivetimeout as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppmsg),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnableNotification<Impl: IMSMQQueue2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, event: ::windows::core::RawPtr, cursor: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnableNotification(&*(&event as *const <IMSMQEvent2 as ::windows::core::Abi>::Abi as *const <IMSMQEvent2 as ::windows::core::DefaultType>::DefaultType), &*(&cursor as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), &*(&receivetimeout as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IMSMQQueue2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ReceiveCurrent_v1<Impl: IMSMQQueue2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReceiveCurrent_v1(
                &*(&transaction as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&wantdestinationqueue as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&wantbody as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&receivetimeout as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppmsg),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeekNext_v1<Impl: IMSMQQueue2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PeekNext_v1(
                &*(&wantdestinationqueue as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&wantbody as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&receivetimeout as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppmsg),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeekCurrent_v1<Impl: IMSMQQueue2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PeekCurrent_v1(
                &*(&wantdestinationqueue as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&wantbody as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&receivetimeout as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppmsg),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Receive<Impl: IMSMQQueue2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Receive(
                &*(&transaction as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&wantdestinationqueue as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&wantbody as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&receivetimeout as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&wantconnectortype as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppmsg),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Peek<Impl: IMSMQQueue2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Peek(
                &*(&wantdestinationqueue as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&wantbody as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&receivetimeout as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&wantconnectortype as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppmsg),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReceiveCurrent<Impl: IMSMQQueue2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReceiveCurrent(
                &*(&transaction as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&wantdestinationqueue as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&wantbody as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&receivetimeout as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&wantconnectortype as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppmsg),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeekNext<Impl: IMSMQQueue2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PeekNext(
                &*(&wantdestinationqueue as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&wantbody as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&receivetimeout as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&wantconnectortype as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppmsg),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeekCurrent<Impl: IMSMQQueue2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PeekCurrent(
                &*(&wantdestinationqueue as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&wantbody as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&receivetimeout as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&wantconnectortype as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppmsg),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Impl: IMSMQQueue2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Properties(::core::mem::transmute_copy(&ppcolproperties)) {
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
            ::windows::core::GetRuntimeClassName::<IMSMQQueue2>,
            ::windows::core::GetTrustLevel,
            Access::<Impl, OFFSET>,
            ShareMode::<Impl, OFFSET>,
            QueueInfo::<Impl, OFFSET>,
            Handle::<Impl, OFFSET>,
            IsOpen::<Impl, OFFSET>,
            Close::<Impl, OFFSET>,
            Receive_v1::<Impl, OFFSET>,
            Peek_v1::<Impl, OFFSET>,
            EnableNotification::<Impl, OFFSET>,
            Reset::<Impl, OFFSET>,
            ReceiveCurrent_v1::<Impl, OFFSET>,
            PeekNext_v1::<Impl, OFFSET>,
            PeekCurrent_v1::<Impl, OFFSET>,
            Receive::<Impl, OFFSET>,
            Peek::<Impl, OFFSET>,
            ReceiveCurrent::<Impl, OFFSET>,
            PeekNext::<Impl, OFFSET>,
            PeekCurrent::<Impl, OFFSET>,
            Properties::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
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
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IMSMQQueue3 {
    const NAME: &'static str = "Windows.Win32.System.MessageQueuing.IMSMQQueue3";
}
#[cfg(feature = "Win32_System_Com")]
impl IMSMQQueue3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueue3Impl, const OFFSET: isize>() -> IMSMQQueue3Vtbl {
        unsafe extern "system" fn Access<Impl: IMSMQQueue3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, placcess: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Access(::core::mem::transmute_copy(&placcess)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShareMode<Impl: IMSMQQueue3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plsharemode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShareMode(::core::mem::transmute_copy(&plsharemode)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueueInfo<Impl: IMSMQQueue3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueueInfo(::core::mem::transmute_copy(&ppqinfo)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Handle<Impl: IMSMQQueue3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plhandle: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Handle(::core::mem::transmute_copy(&plhandle)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsOpen<Impl: IMSMQQueue3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisopen: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsOpen(::core::mem::transmute_copy(&pisopen)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Close<Impl: IMSMQQueue3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Close() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Receive_v1<Impl: IMSMQQueue3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Receive_v1(
                &*(&transaction as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&wantdestinationqueue as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&wantbody as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&receivetimeout as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppmsg),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Peek_v1<Impl: IMSMQQueue3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Peek_v1(
                &*(&wantdestinationqueue as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&wantbody as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&receivetimeout as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppmsg),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnableNotification<Impl: IMSMQQueue3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, event: ::windows::core::RawPtr, cursor: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnableNotification(&*(&event as *const <IMSMQEvent3 as ::windows::core::Abi>::Abi as *const <IMSMQEvent3 as ::windows::core::DefaultType>::DefaultType), &*(&cursor as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), &*(&receivetimeout as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IMSMQQueue3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ReceiveCurrent_v1<Impl: IMSMQQueue3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReceiveCurrent_v1(
                &*(&transaction as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&wantdestinationqueue as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&wantbody as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&receivetimeout as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppmsg),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeekNext_v1<Impl: IMSMQQueue3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PeekNext_v1(
                &*(&wantdestinationqueue as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&wantbody as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&receivetimeout as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppmsg),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeekCurrent_v1<Impl: IMSMQQueue3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PeekCurrent_v1(
                &*(&wantdestinationqueue as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&wantbody as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&receivetimeout as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppmsg),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Receive<Impl: IMSMQQueue3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Receive(
                &*(&transaction as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&wantdestinationqueue as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&wantbody as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&receivetimeout as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&wantconnectortype as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppmsg),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Peek<Impl: IMSMQQueue3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Peek(
                &*(&wantdestinationqueue as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&wantbody as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&receivetimeout as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&wantconnectortype as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppmsg),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReceiveCurrent<Impl: IMSMQQueue3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReceiveCurrent(
                &*(&transaction as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&wantdestinationqueue as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&wantbody as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&receivetimeout as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&wantconnectortype as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppmsg),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeekNext<Impl: IMSMQQueue3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PeekNext(
                &*(&wantdestinationqueue as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&wantbody as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&receivetimeout as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&wantconnectortype as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppmsg),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeekCurrent<Impl: IMSMQQueue3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PeekCurrent(
                &*(&wantdestinationqueue as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&wantbody as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&receivetimeout as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&wantconnectortype as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppmsg),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Impl: IMSMQQueue3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Properties(::core::mem::transmute_copy(&ppcolproperties)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Handle2<Impl: IMSMQQueue3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarhandle: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Handle2(::core::mem::transmute_copy(&pvarhandle)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReceiveByLookupId<Impl: IMSMQQueue3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lookupid: ::core::mem::ManuallyDrop<super::Com::VARIANT>, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReceiveByLookupId(
                &*(&lookupid as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&transaction as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&wantdestinationqueue as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&wantbody as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&wantconnectortype as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppmsg),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReceiveNextByLookupId<Impl: IMSMQQueue3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lookupid: ::core::mem::ManuallyDrop<super::Com::VARIANT>, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReceiveNextByLookupId(
                &*(&lookupid as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&transaction as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&wantdestinationqueue as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&wantbody as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&wantconnectortype as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppmsg),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReceivePreviousByLookupId<Impl: IMSMQQueue3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lookupid: ::core::mem::ManuallyDrop<super::Com::VARIANT>, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReceivePreviousByLookupId(
                &*(&lookupid as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&transaction as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&wantdestinationqueue as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&wantbody as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&wantconnectortype as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppmsg),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReceiveFirstByLookupId<Impl: IMSMQQueue3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReceiveFirstByLookupId(
                &*(&transaction as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&wantdestinationqueue as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&wantbody as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&wantconnectortype as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppmsg),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReceiveLastByLookupId<Impl: IMSMQQueue3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReceiveLastByLookupId(
                &*(&transaction as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&wantdestinationqueue as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&wantbody as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&wantconnectortype as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppmsg),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeekByLookupId<Impl: IMSMQQueue3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lookupid: ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PeekByLookupId(
                &*(&lookupid as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&wantdestinationqueue as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&wantbody as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&wantconnectortype as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppmsg),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeekNextByLookupId<Impl: IMSMQQueue3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lookupid: ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PeekNextByLookupId(
                &*(&lookupid as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&wantdestinationqueue as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&wantbody as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&wantconnectortype as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppmsg),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeekPreviousByLookupId<Impl: IMSMQQueue3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lookupid: ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PeekPreviousByLookupId(
                &*(&lookupid as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&wantdestinationqueue as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&wantbody as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&wantconnectortype as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppmsg),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeekFirstByLookupId<Impl: IMSMQQueue3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PeekFirstByLookupId(
                &*(&wantdestinationqueue as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&wantbody as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&wantconnectortype as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppmsg),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeekLastByLookupId<Impl: IMSMQQueue3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PeekLastByLookupId(
                &*(&wantdestinationqueue as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&wantbody as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&wantconnectortype as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppmsg),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Purge<Impl: IMSMQQueue3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Purge() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsOpen2<Impl: IMSMQQueue3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisopen: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsOpen2(::core::mem::transmute_copy(&pisopen)) {
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
            ::windows::core::GetRuntimeClassName::<IMSMQQueue3>,
            ::windows::core::GetTrustLevel,
            Access::<Impl, OFFSET>,
            ShareMode::<Impl, OFFSET>,
            QueueInfo::<Impl, OFFSET>,
            Handle::<Impl, OFFSET>,
            IsOpen::<Impl, OFFSET>,
            Close::<Impl, OFFSET>,
            Receive_v1::<Impl, OFFSET>,
            Peek_v1::<Impl, OFFSET>,
            EnableNotification::<Impl, OFFSET>,
            Reset::<Impl, OFFSET>,
            ReceiveCurrent_v1::<Impl, OFFSET>,
            PeekNext_v1::<Impl, OFFSET>,
            PeekCurrent_v1::<Impl, OFFSET>,
            Receive::<Impl, OFFSET>,
            Peek::<Impl, OFFSET>,
            ReceiveCurrent::<Impl, OFFSET>,
            PeekNext::<Impl, OFFSET>,
            PeekCurrent::<Impl, OFFSET>,
            Properties::<Impl, OFFSET>,
            Handle2::<Impl, OFFSET>,
            ReceiveByLookupId::<Impl, OFFSET>,
            ReceiveNextByLookupId::<Impl, OFFSET>,
            ReceivePreviousByLookupId::<Impl, OFFSET>,
            ReceiveFirstByLookupId::<Impl, OFFSET>,
            ReceiveLastByLookupId::<Impl, OFFSET>,
            PeekByLookupId::<Impl, OFFSET>,
            PeekNextByLookupId::<Impl, OFFSET>,
            PeekPreviousByLookupId::<Impl, OFFSET>,
            PeekFirstByLookupId::<Impl, OFFSET>,
            PeekLastByLookupId::<Impl, OFFSET>,
            Purge::<Impl, OFFSET>,
            IsOpen2::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
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
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IMSMQQueue4 {
    const NAME: &'static str = "Windows.Win32.System.MessageQueuing.IMSMQQueue4";
}
#[cfg(feature = "Win32_System_Com")]
impl IMSMQQueue4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueue4Impl, const OFFSET: isize>() -> IMSMQQueue4Vtbl {
        unsafe extern "system" fn Access<Impl: IMSMQQueue4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, placcess: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Access(::core::mem::transmute_copy(&placcess)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShareMode<Impl: IMSMQQueue4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plsharemode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShareMode(::core::mem::transmute_copy(&plsharemode)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueueInfo<Impl: IMSMQQueue4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueueInfo(::core::mem::transmute_copy(&ppqinfo)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Handle<Impl: IMSMQQueue4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plhandle: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Handle(::core::mem::transmute_copy(&plhandle)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsOpen<Impl: IMSMQQueue4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisopen: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsOpen(::core::mem::transmute_copy(&pisopen)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Close<Impl: IMSMQQueue4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Close() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Receive_v1<Impl: IMSMQQueue4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Receive_v1(
                &*(&transaction as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&wantdestinationqueue as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&wantbody as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&receivetimeout as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppmsg),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Peek_v1<Impl: IMSMQQueue4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Peek_v1(
                &*(&wantdestinationqueue as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&wantbody as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&receivetimeout as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppmsg),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnableNotification<Impl: IMSMQQueue4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, event: ::windows::core::RawPtr, cursor: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnableNotification(&*(&event as *const <IMSMQEvent3 as ::windows::core::Abi>::Abi as *const <IMSMQEvent3 as ::windows::core::DefaultType>::DefaultType), &*(&cursor as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), &*(&receivetimeout as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IMSMQQueue4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ReceiveCurrent_v1<Impl: IMSMQQueue4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReceiveCurrent_v1(
                &*(&transaction as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&wantdestinationqueue as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&wantbody as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&receivetimeout as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppmsg),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeekNext_v1<Impl: IMSMQQueue4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PeekNext_v1(
                &*(&wantdestinationqueue as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&wantbody as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&receivetimeout as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppmsg),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeekCurrent_v1<Impl: IMSMQQueue4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PeekCurrent_v1(
                &*(&wantdestinationqueue as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&wantbody as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&receivetimeout as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppmsg),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Receive<Impl: IMSMQQueue4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Receive(
                &*(&transaction as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&wantdestinationqueue as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&wantbody as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&receivetimeout as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&wantconnectortype as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppmsg),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Peek<Impl: IMSMQQueue4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Peek(
                &*(&wantdestinationqueue as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&wantbody as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&receivetimeout as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&wantconnectortype as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppmsg),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReceiveCurrent<Impl: IMSMQQueue4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReceiveCurrent(
                &*(&transaction as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&wantdestinationqueue as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&wantbody as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&receivetimeout as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&wantconnectortype as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppmsg),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeekNext<Impl: IMSMQQueue4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PeekNext(
                &*(&wantdestinationqueue as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&wantbody as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&receivetimeout as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&wantconnectortype as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppmsg),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeekCurrent<Impl: IMSMQQueue4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PeekCurrent(
                &*(&wantdestinationqueue as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&wantbody as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&receivetimeout as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&wantconnectortype as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppmsg),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Impl: IMSMQQueue4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Properties(::core::mem::transmute_copy(&ppcolproperties)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Handle2<Impl: IMSMQQueue4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarhandle: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Handle2(::core::mem::transmute_copy(&pvarhandle)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReceiveByLookupId<Impl: IMSMQQueue4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lookupid: ::core::mem::ManuallyDrop<super::Com::VARIANT>, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReceiveByLookupId(
                &*(&lookupid as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&transaction as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&wantdestinationqueue as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&wantbody as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&wantconnectortype as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppmsg),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReceiveNextByLookupId<Impl: IMSMQQueue4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lookupid: ::core::mem::ManuallyDrop<super::Com::VARIANT>, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReceiveNextByLookupId(
                &*(&lookupid as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&transaction as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&wantdestinationqueue as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&wantbody as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&wantconnectortype as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppmsg),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReceivePreviousByLookupId<Impl: IMSMQQueue4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lookupid: ::core::mem::ManuallyDrop<super::Com::VARIANT>, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReceivePreviousByLookupId(
                &*(&lookupid as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&transaction as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&wantdestinationqueue as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&wantbody as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&wantconnectortype as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppmsg),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReceiveFirstByLookupId<Impl: IMSMQQueue4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReceiveFirstByLookupId(
                &*(&transaction as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&wantdestinationqueue as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&wantbody as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&wantconnectortype as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppmsg),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReceiveLastByLookupId<Impl: IMSMQQueue4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReceiveLastByLookupId(
                &*(&transaction as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&wantdestinationqueue as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&wantbody as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&wantconnectortype as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppmsg),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeekByLookupId<Impl: IMSMQQueue4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lookupid: ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PeekByLookupId(
                &*(&lookupid as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&wantdestinationqueue as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&wantbody as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&wantconnectortype as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppmsg),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeekNextByLookupId<Impl: IMSMQQueue4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lookupid: ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PeekNextByLookupId(
                &*(&lookupid as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&wantdestinationqueue as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&wantbody as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&wantconnectortype as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppmsg),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeekPreviousByLookupId<Impl: IMSMQQueue4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lookupid: ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PeekPreviousByLookupId(
                &*(&lookupid as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&wantdestinationqueue as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&wantbody as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&wantconnectortype as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppmsg),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeekFirstByLookupId<Impl: IMSMQQueue4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PeekFirstByLookupId(
                &*(&wantdestinationqueue as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&wantbody as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&wantconnectortype as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppmsg),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeekLastByLookupId<Impl: IMSMQQueue4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PeekLastByLookupId(
                &*(&wantdestinationqueue as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&wantbody as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&wantconnectortype as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppmsg),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Purge<Impl: IMSMQQueue4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Purge() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsOpen2<Impl: IMSMQQueue4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisopen: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsOpen2(::core::mem::transmute_copy(&pisopen)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReceiveByLookupIdAllowPeek<Impl: IMSMQQueue4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lookupid: ::core::mem::ManuallyDrop<super::Com::VARIANT>, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReceiveByLookupIdAllowPeek(
                &*(&lookupid as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&transaction as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&wantdestinationqueue as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&wantbody as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&wantconnectortype as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppmsg),
            ) {
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
            ::windows::core::GetRuntimeClassName::<IMSMQQueue4>,
            ::windows::core::GetTrustLevel,
            Access::<Impl, OFFSET>,
            ShareMode::<Impl, OFFSET>,
            QueueInfo::<Impl, OFFSET>,
            Handle::<Impl, OFFSET>,
            IsOpen::<Impl, OFFSET>,
            Close::<Impl, OFFSET>,
            Receive_v1::<Impl, OFFSET>,
            Peek_v1::<Impl, OFFSET>,
            EnableNotification::<Impl, OFFSET>,
            Reset::<Impl, OFFSET>,
            ReceiveCurrent_v1::<Impl, OFFSET>,
            PeekNext_v1::<Impl, OFFSET>,
            PeekCurrent_v1::<Impl, OFFSET>,
            Receive::<Impl, OFFSET>,
            Peek::<Impl, OFFSET>,
            ReceiveCurrent::<Impl, OFFSET>,
            PeekNext::<Impl, OFFSET>,
            PeekCurrent::<Impl, OFFSET>,
            Properties::<Impl, OFFSET>,
            Handle2::<Impl, OFFSET>,
            ReceiveByLookupId::<Impl, OFFSET>,
            ReceiveNextByLookupId::<Impl, OFFSET>,
            ReceivePreviousByLookupId::<Impl, OFFSET>,
            ReceiveFirstByLookupId::<Impl, OFFSET>,
            ReceiveLastByLookupId::<Impl, OFFSET>,
            PeekByLookupId::<Impl, OFFSET>,
            PeekNextByLookupId::<Impl, OFFSET>,
            PeekPreviousByLookupId::<Impl, OFFSET>,
            PeekFirstByLookupId::<Impl, OFFSET>,
            PeekLastByLookupId::<Impl, OFFSET>,
            Purge::<Impl, OFFSET>,
            IsOpen2::<Impl, OFFSET>,
            ReceiveByLookupIdAllowPeek::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
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
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IMSMQQueueInfo {
    const NAME: &'static str = "Windows.Win32.System.MessageQueuing.IMSMQQueueInfo";
}
#[cfg(feature = "Win32_System_Com")]
impl IMSMQQueueInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfoImpl, const OFFSET: isize>() -> IMSMQQueueInfoVtbl {
        unsafe extern "system" fn QueueGuid<Impl: IMSMQQueueInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrguidqueue: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueueGuid(::core::mem::transmute_copy(&pbstrguidqueue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServiceTypeGuid<Impl: IMSMQQueueInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrguidservicetype: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ServiceTypeGuid(::core::mem::transmute_copy(&pbstrguidservicetype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetServiceTypeGuid<Impl: IMSMQQueueInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrguidservicetype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetServiceTypeGuid(&*(&bstrguidservicetype as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Label<Impl: IMSMQQueueInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrlabel: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Label(::core::mem::transmute_copy(&pbstrlabel)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLabel<Impl: IMSMQQueueInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrlabel: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetLabel(&*(&bstrlabel as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PathName<Impl: IMSMQQueueInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrpathname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PathName(::core::mem::transmute_copy(&pbstrpathname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPathName<Impl: IMSMQQueueInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpathname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetPathName(&*(&bstrpathname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FormatName<Impl: IMSMQQueueInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrformatname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FormatName(::core::mem::transmute_copy(&pbstrformatname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFormatName<Impl: IMSMQQueueInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrformatname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetFormatName(&*(&bstrformatname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsTransactional<Impl: IMSMQQueueInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pistransactional: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsTransactional(::core::mem::transmute_copy(&pistransactional)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrivLevel<Impl: IMSMQQueueInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plprivlevel: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PrivLevel(::core::mem::transmute_copy(&plprivlevel)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrivLevel<Impl: IMSMQQueueInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lprivlevel: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetPrivLevel(lprivlevel) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Journal<Impl: IMSMQQueueInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pljournal: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Journal(::core::mem::transmute_copy(&pljournal)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetJournal<Impl: IMSMQQueueInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ljournal: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetJournal(ljournal) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Quota<Impl: IMSMQQueueInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plquota: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Quota(::core::mem::transmute_copy(&plquota)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetQuota<Impl: IMSMQQueueInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lquota: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetQuota(lquota) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BasePriority<Impl: IMSMQQueueInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plbasepriority: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BasePriority(::core::mem::transmute_copy(&plbasepriority)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBasePriority<Impl: IMSMQQueueInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lbasepriority: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetBasePriority(lbasepriority) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTime<Impl: IMSMQQueueInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarcreatetime: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateTime(::core::mem::transmute_copy(&pvarcreatetime)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ModifyTime<Impl: IMSMQQueueInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarmodifytime: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ModifyTime(::core::mem::transmute_copy(&pvarmodifytime)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Authenticate<Impl: IMSMQQueueInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plauthenticate: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Authenticate(::core::mem::transmute_copy(&plauthenticate)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAuthenticate<Impl: IMSMQQueueInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lauthenticate: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAuthenticate(lauthenticate) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn JournalQuota<Impl: IMSMQQueueInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pljournalquota: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).JournalQuota(::core::mem::transmute_copy(&pljournalquota)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetJournalQuota<Impl: IMSMQQueueInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ljournalquota: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetJournalQuota(ljournalquota) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsWorldReadable<Impl: IMSMQQueueInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisworldreadable: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsWorldReadable(::core::mem::transmute_copy(&pisworldreadable)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Create<Impl: IMSMQQueueInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, istransactional: *const super::Com::VARIANT, isworldreadable: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(&*(&istransactional as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), &*(&isworldreadable as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Impl: IMSMQQueueInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Open<Impl: IMSMQQueueInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, access: i32, sharemode: i32, ppq: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Open(access, sharemode, ::core::mem::transmute_copy(&ppq)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Impl: IMSMQQueueInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Refresh() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Update<Impl: IMSMQQueueInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Update() {
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
            ::windows::core::GetRuntimeClassName::<IMSMQQueueInfo>,
            ::windows::core::GetTrustLevel,
            QueueGuid::<Impl, OFFSET>,
            ServiceTypeGuid::<Impl, OFFSET>,
            SetServiceTypeGuid::<Impl, OFFSET>,
            Label::<Impl, OFFSET>,
            SetLabel::<Impl, OFFSET>,
            PathName::<Impl, OFFSET>,
            SetPathName::<Impl, OFFSET>,
            FormatName::<Impl, OFFSET>,
            SetFormatName::<Impl, OFFSET>,
            IsTransactional::<Impl, OFFSET>,
            PrivLevel::<Impl, OFFSET>,
            SetPrivLevel::<Impl, OFFSET>,
            Journal::<Impl, OFFSET>,
            SetJournal::<Impl, OFFSET>,
            Quota::<Impl, OFFSET>,
            SetQuota::<Impl, OFFSET>,
            BasePriority::<Impl, OFFSET>,
            SetBasePriority::<Impl, OFFSET>,
            CreateTime::<Impl, OFFSET>,
            ModifyTime::<Impl, OFFSET>,
            Authenticate::<Impl, OFFSET>,
            SetAuthenticate::<Impl, OFFSET>,
            JournalQuota::<Impl, OFFSET>,
            SetJournalQuota::<Impl, OFFSET>,
            IsWorldReadable::<Impl, OFFSET>,
            Create::<Impl, OFFSET>,
            Delete::<Impl, OFFSET>,
            Open::<Impl, OFFSET>,
            Refresh::<Impl, OFFSET>,
            Update::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
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
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IMSMQQueueInfo2 {
    const NAME: &'static str = "Windows.Win32.System.MessageQueuing.IMSMQQueueInfo2";
}
#[cfg(feature = "Win32_System_Com")]
impl IMSMQQueueInfo2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo2Impl, const OFFSET: isize>() -> IMSMQQueueInfo2Vtbl {
        unsafe extern "system" fn QueueGuid<Impl: IMSMQQueueInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrguidqueue: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueueGuid(::core::mem::transmute_copy(&pbstrguidqueue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServiceTypeGuid<Impl: IMSMQQueueInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrguidservicetype: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ServiceTypeGuid(::core::mem::transmute_copy(&pbstrguidservicetype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetServiceTypeGuid<Impl: IMSMQQueueInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrguidservicetype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetServiceTypeGuid(&*(&bstrguidservicetype as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Label<Impl: IMSMQQueueInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrlabel: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Label(::core::mem::transmute_copy(&pbstrlabel)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLabel<Impl: IMSMQQueueInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrlabel: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetLabel(&*(&bstrlabel as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PathName<Impl: IMSMQQueueInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrpathname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PathName(::core::mem::transmute_copy(&pbstrpathname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPathName<Impl: IMSMQQueueInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpathname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetPathName(&*(&bstrpathname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FormatName<Impl: IMSMQQueueInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrformatname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FormatName(::core::mem::transmute_copy(&pbstrformatname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFormatName<Impl: IMSMQQueueInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrformatname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetFormatName(&*(&bstrformatname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsTransactional<Impl: IMSMQQueueInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pistransactional: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsTransactional(::core::mem::transmute_copy(&pistransactional)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrivLevel<Impl: IMSMQQueueInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plprivlevel: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PrivLevel(::core::mem::transmute_copy(&plprivlevel)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrivLevel<Impl: IMSMQQueueInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lprivlevel: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetPrivLevel(lprivlevel) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Journal<Impl: IMSMQQueueInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pljournal: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Journal(::core::mem::transmute_copy(&pljournal)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetJournal<Impl: IMSMQQueueInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ljournal: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetJournal(ljournal) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Quota<Impl: IMSMQQueueInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plquota: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Quota(::core::mem::transmute_copy(&plquota)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetQuota<Impl: IMSMQQueueInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lquota: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetQuota(lquota) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BasePriority<Impl: IMSMQQueueInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plbasepriority: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BasePriority(::core::mem::transmute_copy(&plbasepriority)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBasePriority<Impl: IMSMQQueueInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lbasepriority: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetBasePriority(lbasepriority) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTime<Impl: IMSMQQueueInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarcreatetime: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateTime(::core::mem::transmute_copy(&pvarcreatetime)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ModifyTime<Impl: IMSMQQueueInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarmodifytime: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ModifyTime(::core::mem::transmute_copy(&pvarmodifytime)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Authenticate<Impl: IMSMQQueueInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plauthenticate: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Authenticate(::core::mem::transmute_copy(&plauthenticate)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAuthenticate<Impl: IMSMQQueueInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lauthenticate: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAuthenticate(lauthenticate) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn JournalQuota<Impl: IMSMQQueueInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pljournalquota: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).JournalQuota(::core::mem::transmute_copy(&pljournalquota)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetJournalQuota<Impl: IMSMQQueueInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ljournalquota: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetJournalQuota(ljournalquota) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsWorldReadable<Impl: IMSMQQueueInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisworldreadable: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsWorldReadable(::core::mem::transmute_copy(&pisworldreadable)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Create<Impl: IMSMQQueueInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, istransactional: *const super::Com::VARIANT, isworldreadable: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(&*(&istransactional as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), &*(&isworldreadable as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Impl: IMSMQQueueInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Open<Impl: IMSMQQueueInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, access: i32, sharemode: i32, ppq: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Open(access, sharemode, ::core::mem::transmute_copy(&ppq)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Impl: IMSMQQueueInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Refresh() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Update<Impl: IMSMQQueueInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Update() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PathNameDNS<Impl: IMSMQQueueInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrpathnamedns: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PathNameDNS(::core::mem::transmute_copy(&pbstrpathnamedns)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Impl: IMSMQQueueInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Properties(::core::mem::transmute_copy(&ppcolproperties)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Security<Impl: IMSMQQueueInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarsecurity: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Security(::core::mem::transmute_copy(&pvarsecurity)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSecurity<Impl: IMSMQQueueInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varsecurity: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSecurity(&*(&varsecurity as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
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
            ::windows::core::GetRuntimeClassName::<IMSMQQueueInfo2>,
            ::windows::core::GetTrustLevel,
            QueueGuid::<Impl, OFFSET>,
            ServiceTypeGuid::<Impl, OFFSET>,
            SetServiceTypeGuid::<Impl, OFFSET>,
            Label::<Impl, OFFSET>,
            SetLabel::<Impl, OFFSET>,
            PathName::<Impl, OFFSET>,
            SetPathName::<Impl, OFFSET>,
            FormatName::<Impl, OFFSET>,
            SetFormatName::<Impl, OFFSET>,
            IsTransactional::<Impl, OFFSET>,
            PrivLevel::<Impl, OFFSET>,
            SetPrivLevel::<Impl, OFFSET>,
            Journal::<Impl, OFFSET>,
            SetJournal::<Impl, OFFSET>,
            Quota::<Impl, OFFSET>,
            SetQuota::<Impl, OFFSET>,
            BasePriority::<Impl, OFFSET>,
            SetBasePriority::<Impl, OFFSET>,
            CreateTime::<Impl, OFFSET>,
            ModifyTime::<Impl, OFFSET>,
            Authenticate::<Impl, OFFSET>,
            SetAuthenticate::<Impl, OFFSET>,
            JournalQuota::<Impl, OFFSET>,
            SetJournalQuota::<Impl, OFFSET>,
            IsWorldReadable::<Impl, OFFSET>,
            Create::<Impl, OFFSET>,
            Delete::<Impl, OFFSET>,
            Open::<Impl, OFFSET>,
            Refresh::<Impl, OFFSET>,
            Update::<Impl, OFFSET>,
            PathNameDNS::<Impl, OFFSET>,
            Properties::<Impl, OFFSET>,
            Security::<Impl, OFFSET>,
            SetSecurity::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
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
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IMSMQQueueInfo3 {
    const NAME: &'static str = "Windows.Win32.System.MessageQueuing.IMSMQQueueInfo3";
}
#[cfg(feature = "Win32_System_Com")]
impl IMSMQQueueInfo3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo3Impl, const OFFSET: isize>() -> IMSMQQueueInfo3Vtbl {
        unsafe extern "system" fn QueueGuid<Impl: IMSMQQueueInfo3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrguidqueue: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueueGuid(::core::mem::transmute_copy(&pbstrguidqueue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServiceTypeGuid<Impl: IMSMQQueueInfo3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrguidservicetype: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ServiceTypeGuid(::core::mem::transmute_copy(&pbstrguidservicetype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetServiceTypeGuid<Impl: IMSMQQueueInfo3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrguidservicetype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetServiceTypeGuid(&*(&bstrguidservicetype as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Label<Impl: IMSMQQueueInfo3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrlabel: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Label(::core::mem::transmute_copy(&pbstrlabel)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLabel<Impl: IMSMQQueueInfo3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrlabel: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetLabel(&*(&bstrlabel as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PathName<Impl: IMSMQQueueInfo3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrpathname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PathName(::core::mem::transmute_copy(&pbstrpathname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPathName<Impl: IMSMQQueueInfo3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpathname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetPathName(&*(&bstrpathname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FormatName<Impl: IMSMQQueueInfo3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrformatname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FormatName(::core::mem::transmute_copy(&pbstrformatname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFormatName<Impl: IMSMQQueueInfo3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrformatname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetFormatName(&*(&bstrformatname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsTransactional<Impl: IMSMQQueueInfo3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pistransactional: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsTransactional(::core::mem::transmute_copy(&pistransactional)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrivLevel<Impl: IMSMQQueueInfo3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plprivlevel: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PrivLevel(::core::mem::transmute_copy(&plprivlevel)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrivLevel<Impl: IMSMQQueueInfo3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lprivlevel: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetPrivLevel(lprivlevel) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Journal<Impl: IMSMQQueueInfo3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pljournal: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Journal(::core::mem::transmute_copy(&pljournal)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetJournal<Impl: IMSMQQueueInfo3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ljournal: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetJournal(ljournal) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Quota<Impl: IMSMQQueueInfo3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plquota: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Quota(::core::mem::transmute_copy(&plquota)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetQuota<Impl: IMSMQQueueInfo3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lquota: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetQuota(lquota) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BasePriority<Impl: IMSMQQueueInfo3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plbasepriority: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BasePriority(::core::mem::transmute_copy(&plbasepriority)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBasePriority<Impl: IMSMQQueueInfo3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lbasepriority: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetBasePriority(lbasepriority) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTime<Impl: IMSMQQueueInfo3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarcreatetime: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateTime(::core::mem::transmute_copy(&pvarcreatetime)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ModifyTime<Impl: IMSMQQueueInfo3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarmodifytime: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ModifyTime(::core::mem::transmute_copy(&pvarmodifytime)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Authenticate<Impl: IMSMQQueueInfo3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plauthenticate: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Authenticate(::core::mem::transmute_copy(&plauthenticate)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAuthenticate<Impl: IMSMQQueueInfo3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lauthenticate: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAuthenticate(lauthenticate) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn JournalQuota<Impl: IMSMQQueueInfo3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pljournalquota: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).JournalQuota(::core::mem::transmute_copy(&pljournalquota)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetJournalQuota<Impl: IMSMQQueueInfo3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ljournalquota: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetJournalQuota(ljournalquota) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsWorldReadable<Impl: IMSMQQueueInfo3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisworldreadable: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsWorldReadable(::core::mem::transmute_copy(&pisworldreadable)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Create<Impl: IMSMQQueueInfo3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, istransactional: *const super::Com::VARIANT, isworldreadable: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(&*(&istransactional as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), &*(&isworldreadable as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Impl: IMSMQQueueInfo3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Open<Impl: IMSMQQueueInfo3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, access: i32, sharemode: i32, ppq: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Open(access, sharemode, ::core::mem::transmute_copy(&ppq)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Impl: IMSMQQueueInfo3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Refresh() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Update<Impl: IMSMQQueueInfo3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Update() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PathNameDNS<Impl: IMSMQQueueInfo3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrpathnamedns: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PathNameDNS(::core::mem::transmute_copy(&pbstrpathnamedns)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Impl: IMSMQQueueInfo3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Properties(::core::mem::transmute_copy(&ppcolproperties)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Security<Impl: IMSMQQueueInfo3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarsecurity: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Security(::core::mem::transmute_copy(&pvarsecurity)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSecurity<Impl: IMSMQQueueInfo3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varsecurity: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSecurity(&*(&varsecurity as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsTransactional2<Impl: IMSMQQueueInfo3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pistransactional: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsTransactional2(::core::mem::transmute_copy(&pistransactional)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsWorldReadable2<Impl: IMSMQQueueInfo3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisworldreadable: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsWorldReadable2(::core::mem::transmute_copy(&pisworldreadable)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MulticastAddress<Impl: IMSMQQueueInfo3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrmulticastaddress: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MulticastAddress(::core::mem::transmute_copy(&pbstrmulticastaddress)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMulticastAddress<Impl: IMSMQQueueInfo3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrmulticastaddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetMulticastAddress(&*(&bstrmulticastaddress as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ADsPath<Impl: IMSMQQueueInfo3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstradspath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ADsPath(::core::mem::transmute_copy(&pbstradspath)) {
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
            ::windows::core::GetRuntimeClassName::<IMSMQQueueInfo3>,
            ::windows::core::GetTrustLevel,
            QueueGuid::<Impl, OFFSET>,
            ServiceTypeGuid::<Impl, OFFSET>,
            SetServiceTypeGuid::<Impl, OFFSET>,
            Label::<Impl, OFFSET>,
            SetLabel::<Impl, OFFSET>,
            PathName::<Impl, OFFSET>,
            SetPathName::<Impl, OFFSET>,
            FormatName::<Impl, OFFSET>,
            SetFormatName::<Impl, OFFSET>,
            IsTransactional::<Impl, OFFSET>,
            PrivLevel::<Impl, OFFSET>,
            SetPrivLevel::<Impl, OFFSET>,
            Journal::<Impl, OFFSET>,
            SetJournal::<Impl, OFFSET>,
            Quota::<Impl, OFFSET>,
            SetQuota::<Impl, OFFSET>,
            BasePriority::<Impl, OFFSET>,
            SetBasePriority::<Impl, OFFSET>,
            CreateTime::<Impl, OFFSET>,
            ModifyTime::<Impl, OFFSET>,
            Authenticate::<Impl, OFFSET>,
            SetAuthenticate::<Impl, OFFSET>,
            JournalQuota::<Impl, OFFSET>,
            SetJournalQuota::<Impl, OFFSET>,
            IsWorldReadable::<Impl, OFFSET>,
            Create::<Impl, OFFSET>,
            Delete::<Impl, OFFSET>,
            Open::<Impl, OFFSET>,
            Refresh::<Impl, OFFSET>,
            Update::<Impl, OFFSET>,
            PathNameDNS::<Impl, OFFSET>,
            Properties::<Impl, OFFSET>,
            Security::<Impl, OFFSET>,
            SetSecurity::<Impl, OFFSET>,
            IsTransactional2::<Impl, OFFSET>,
            IsWorldReadable2::<Impl, OFFSET>,
            MulticastAddress::<Impl, OFFSET>,
            SetMulticastAddress::<Impl, OFFSET>,
            ADsPath::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
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
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IMSMQQueueInfo4 {
    const NAME: &'static str = "Windows.Win32.System.MessageQueuing.IMSMQQueueInfo4";
}
#[cfg(feature = "Win32_System_Com")]
impl IMSMQQueueInfo4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfo4Impl, const OFFSET: isize>() -> IMSMQQueueInfo4Vtbl {
        unsafe extern "system" fn QueueGuid<Impl: IMSMQQueueInfo4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrguidqueue: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueueGuid(::core::mem::transmute_copy(&pbstrguidqueue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServiceTypeGuid<Impl: IMSMQQueueInfo4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrguidservicetype: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ServiceTypeGuid(::core::mem::transmute_copy(&pbstrguidservicetype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetServiceTypeGuid<Impl: IMSMQQueueInfo4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrguidservicetype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetServiceTypeGuid(&*(&bstrguidservicetype as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Label<Impl: IMSMQQueueInfo4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrlabel: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Label(::core::mem::transmute_copy(&pbstrlabel)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLabel<Impl: IMSMQQueueInfo4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrlabel: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetLabel(&*(&bstrlabel as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PathName<Impl: IMSMQQueueInfo4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrpathname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PathName(::core::mem::transmute_copy(&pbstrpathname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPathName<Impl: IMSMQQueueInfo4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpathname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetPathName(&*(&bstrpathname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FormatName<Impl: IMSMQQueueInfo4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrformatname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FormatName(::core::mem::transmute_copy(&pbstrformatname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFormatName<Impl: IMSMQQueueInfo4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrformatname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetFormatName(&*(&bstrformatname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsTransactional<Impl: IMSMQQueueInfo4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pistransactional: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsTransactional(::core::mem::transmute_copy(&pistransactional)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrivLevel<Impl: IMSMQQueueInfo4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plprivlevel: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PrivLevel(::core::mem::transmute_copy(&plprivlevel)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrivLevel<Impl: IMSMQQueueInfo4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lprivlevel: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetPrivLevel(lprivlevel) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Journal<Impl: IMSMQQueueInfo4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pljournal: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Journal(::core::mem::transmute_copy(&pljournal)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetJournal<Impl: IMSMQQueueInfo4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ljournal: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetJournal(ljournal) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Quota<Impl: IMSMQQueueInfo4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plquota: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Quota(::core::mem::transmute_copy(&plquota)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetQuota<Impl: IMSMQQueueInfo4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lquota: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetQuota(lquota) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BasePriority<Impl: IMSMQQueueInfo4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plbasepriority: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BasePriority(::core::mem::transmute_copy(&plbasepriority)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBasePriority<Impl: IMSMQQueueInfo4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lbasepriority: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetBasePriority(lbasepriority) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTime<Impl: IMSMQQueueInfo4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarcreatetime: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateTime(::core::mem::transmute_copy(&pvarcreatetime)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ModifyTime<Impl: IMSMQQueueInfo4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarmodifytime: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ModifyTime(::core::mem::transmute_copy(&pvarmodifytime)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Authenticate<Impl: IMSMQQueueInfo4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plauthenticate: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Authenticate(::core::mem::transmute_copy(&plauthenticate)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAuthenticate<Impl: IMSMQQueueInfo4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lauthenticate: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAuthenticate(lauthenticate) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn JournalQuota<Impl: IMSMQQueueInfo4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pljournalquota: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).JournalQuota(::core::mem::transmute_copy(&pljournalquota)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetJournalQuota<Impl: IMSMQQueueInfo4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ljournalquota: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetJournalQuota(ljournalquota) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsWorldReadable<Impl: IMSMQQueueInfo4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisworldreadable: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsWorldReadable(::core::mem::transmute_copy(&pisworldreadable)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Create<Impl: IMSMQQueueInfo4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, istransactional: *const super::Com::VARIANT, isworldreadable: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(&*(&istransactional as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), &*(&isworldreadable as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Impl: IMSMQQueueInfo4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Open<Impl: IMSMQQueueInfo4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, access: i32, sharemode: i32, ppq: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Open(access, sharemode, ::core::mem::transmute_copy(&ppq)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Impl: IMSMQQueueInfo4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Refresh() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Update<Impl: IMSMQQueueInfo4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Update() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PathNameDNS<Impl: IMSMQQueueInfo4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrpathnamedns: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PathNameDNS(::core::mem::transmute_copy(&pbstrpathnamedns)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Impl: IMSMQQueueInfo4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Properties(::core::mem::transmute_copy(&ppcolproperties)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Security<Impl: IMSMQQueueInfo4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarsecurity: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Security(::core::mem::transmute_copy(&pvarsecurity)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSecurity<Impl: IMSMQQueueInfo4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varsecurity: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSecurity(&*(&varsecurity as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsTransactional2<Impl: IMSMQQueueInfo4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pistransactional: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsTransactional2(::core::mem::transmute_copy(&pistransactional)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsWorldReadable2<Impl: IMSMQQueueInfo4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisworldreadable: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsWorldReadable2(::core::mem::transmute_copy(&pisworldreadable)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MulticastAddress<Impl: IMSMQQueueInfo4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrmulticastaddress: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MulticastAddress(::core::mem::transmute_copy(&pbstrmulticastaddress)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMulticastAddress<Impl: IMSMQQueueInfo4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrmulticastaddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetMulticastAddress(&*(&bstrmulticastaddress as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ADsPath<Impl: IMSMQQueueInfo4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstradspath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ADsPath(::core::mem::transmute_copy(&pbstradspath)) {
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
            ::windows::core::GetRuntimeClassName::<IMSMQQueueInfo4>,
            ::windows::core::GetTrustLevel,
            QueueGuid::<Impl, OFFSET>,
            ServiceTypeGuid::<Impl, OFFSET>,
            SetServiceTypeGuid::<Impl, OFFSET>,
            Label::<Impl, OFFSET>,
            SetLabel::<Impl, OFFSET>,
            PathName::<Impl, OFFSET>,
            SetPathName::<Impl, OFFSET>,
            FormatName::<Impl, OFFSET>,
            SetFormatName::<Impl, OFFSET>,
            IsTransactional::<Impl, OFFSET>,
            PrivLevel::<Impl, OFFSET>,
            SetPrivLevel::<Impl, OFFSET>,
            Journal::<Impl, OFFSET>,
            SetJournal::<Impl, OFFSET>,
            Quota::<Impl, OFFSET>,
            SetQuota::<Impl, OFFSET>,
            BasePriority::<Impl, OFFSET>,
            SetBasePriority::<Impl, OFFSET>,
            CreateTime::<Impl, OFFSET>,
            ModifyTime::<Impl, OFFSET>,
            Authenticate::<Impl, OFFSET>,
            SetAuthenticate::<Impl, OFFSET>,
            JournalQuota::<Impl, OFFSET>,
            SetJournalQuota::<Impl, OFFSET>,
            IsWorldReadable::<Impl, OFFSET>,
            Create::<Impl, OFFSET>,
            Delete::<Impl, OFFSET>,
            Open::<Impl, OFFSET>,
            Refresh::<Impl, OFFSET>,
            Update::<Impl, OFFSET>,
            PathNameDNS::<Impl, OFFSET>,
            Properties::<Impl, OFFSET>,
            Security::<Impl, OFFSET>,
            SetSecurity::<Impl, OFFSET>,
            IsTransactional2::<Impl, OFFSET>,
            IsWorldReadable2::<Impl, OFFSET>,
            MulticastAddress::<Impl, OFFSET>,
            SetMulticastAddress::<Impl, OFFSET>,
            ADsPath::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSMQQueueInfosImpl: Sized + IDispatchImpl {
    fn Reset();
    fn Next();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IMSMQQueueInfos {
    const NAME: &'static str = "Windows.Win32.System.MessageQueuing.IMSMQQueueInfos";
}
#[cfg(feature = "Win32_System_Com")]
impl IMSMQQueueInfosVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfosImpl, const OFFSET: isize>() -> IMSMQQueueInfosVtbl {
        unsafe extern "system" fn Reset<Impl: IMSMQQueueInfosImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Next<Impl: IMSMQQueueInfosImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinfonext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Next(::core::mem::transmute_copy(&ppqinfonext)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMSMQQueueInfos>, ::windows::core::GetTrustLevel, Reset::<Impl, OFFSET>, Next::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSMQQueueInfos2Impl: Sized + IDispatchImpl {
    fn Reset();
    fn Next();
    fn Properties();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IMSMQQueueInfos2 {
    const NAME: &'static str = "Windows.Win32.System.MessageQueuing.IMSMQQueueInfos2";
}
#[cfg(feature = "Win32_System_Com")]
impl IMSMQQueueInfos2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfos2Impl, const OFFSET: isize>() -> IMSMQQueueInfos2Vtbl {
        unsafe extern "system" fn Reset<Impl: IMSMQQueueInfos2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Next<Impl: IMSMQQueueInfos2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinfonext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Next(::core::mem::transmute_copy(&ppqinfonext)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Impl: IMSMQQueueInfos2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Properties(::core::mem::transmute_copy(&ppcolproperties)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMSMQQueueInfos2>, ::windows::core::GetTrustLevel, Reset::<Impl, OFFSET>, Next::<Impl, OFFSET>, Properties::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSMQQueueInfos3Impl: Sized + IDispatchImpl {
    fn Reset();
    fn Next();
    fn Properties();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IMSMQQueueInfos3 {
    const NAME: &'static str = "Windows.Win32.System.MessageQueuing.IMSMQQueueInfos3";
}
#[cfg(feature = "Win32_System_Com")]
impl IMSMQQueueInfos3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfos3Impl, const OFFSET: isize>() -> IMSMQQueueInfos3Vtbl {
        unsafe extern "system" fn Reset<Impl: IMSMQQueueInfos3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Next<Impl: IMSMQQueueInfos3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinfonext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Next(::core::mem::transmute_copy(&ppqinfonext)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Impl: IMSMQQueueInfos3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Properties(::core::mem::transmute_copy(&ppcolproperties)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMSMQQueueInfos3>, ::windows::core::GetTrustLevel, Reset::<Impl, OFFSET>, Next::<Impl, OFFSET>, Properties::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSMQQueueInfos4Impl: Sized + IDispatchImpl {
    fn Reset();
    fn Next();
    fn Properties();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IMSMQQueueInfos4 {
    const NAME: &'static str = "Windows.Win32.System.MessageQueuing.IMSMQQueueInfos4";
}
#[cfg(feature = "Win32_System_Com")]
impl IMSMQQueueInfos4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueInfos4Impl, const OFFSET: isize>() -> IMSMQQueueInfos4Vtbl {
        unsafe extern "system" fn Reset<Impl: IMSMQQueueInfos4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Next<Impl: IMSMQQueueInfos4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqinfonext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Next(::core::mem::transmute_copy(&ppqinfonext)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Impl: IMSMQQueueInfos4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Properties(::core::mem::transmute_copy(&ppcolproperties)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMSMQQueueInfos4>, ::windows::core::GetTrustLevel, Reset::<Impl, OFFSET>, Next::<Impl, OFFSET>, Properties::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSMQQueueManagementImpl: Sized + IMSMQManagementImpl + IDispatchImpl {
    fn JournalMessageCount();
    fn BytesInJournal();
    fn EodGetReceiveInfo();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IMSMQQueueManagement {
    const NAME: &'static str = "Windows.Win32.System.MessageQueuing.IMSMQQueueManagement";
}
#[cfg(feature = "Win32_System_Com")]
impl IMSMQQueueManagementVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQQueueManagementImpl, const OFFSET: isize>() -> IMSMQQueueManagementVtbl {
        unsafe extern "system" fn JournalMessageCount<Impl: IMSMQQueueManagementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pljournalmessagecount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).JournalMessageCount(::core::mem::transmute_copy(&pljournalmessagecount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BytesInJournal<Impl: IMSMQQueueManagementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvbytesinjournal: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BytesInJournal(::core::mem::transmute_copy(&pvbytesinjournal)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EodGetReceiveInfo<Impl: IMSMQQueueManagementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvcollection: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EodGetReceiveInfo(::core::mem::transmute_copy(&pvcollection)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMSMQQueueManagement>, ::windows::core::GetTrustLevel, JournalMessageCount::<Impl, OFFSET>, BytesInJournal::<Impl, OFFSET>, EodGetReceiveInfo::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSMQTransactionImpl: Sized + IDispatchImpl {
    fn Transaction();
    fn Commit();
    fn Abort();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IMSMQTransaction {
    const NAME: &'static str = "Windows.Win32.System.MessageQueuing.IMSMQTransaction";
}
#[cfg(feature = "Win32_System_Com")]
impl IMSMQTransactionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQTransactionImpl, const OFFSET: isize>() -> IMSMQTransactionVtbl {
        unsafe extern "system" fn Transaction<Impl: IMSMQTransactionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pltransaction: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Transaction(::core::mem::transmute_copy(&pltransaction)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Commit<Impl: IMSMQTransactionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fretaining: *const super::Com::VARIANT, grftc: *const super::Com::VARIANT, grfrm: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Commit(&*(&fretaining as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), &*(&grftc as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), &*(&grfrm as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Abort<Impl: IMSMQTransactionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fretaining: *const super::Com::VARIANT, fasync: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Abort(&*(&fretaining as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), &*(&fasync as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMSMQTransaction>, ::windows::core::GetTrustLevel, Transaction::<Impl, OFFSET>, Commit::<Impl, OFFSET>, Abort::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSMQTransaction2Impl: Sized + IMSMQTransactionImpl + IDispatchImpl {
    fn InitNew();
    fn Properties();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IMSMQTransaction2 {
    const NAME: &'static str = "Windows.Win32.System.MessageQueuing.IMSMQTransaction2";
}
#[cfg(feature = "Win32_System_Com")]
impl IMSMQTransaction2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQTransaction2Impl, const OFFSET: isize>() -> IMSMQTransaction2Vtbl {
        unsafe extern "system" fn InitNew<Impl: IMSMQTransaction2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vartransaction: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InitNew(&*(&vartransaction as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Impl: IMSMQTransaction2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Properties(::core::mem::transmute_copy(&ppcolproperties)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMSMQTransaction2>, ::windows::core::GetTrustLevel, InitNew::<Impl, OFFSET>, Properties::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSMQTransaction3Impl: Sized + IMSMQTransaction2Impl + IMSMQTransactionImpl + IDispatchImpl {
    fn ITransaction();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IMSMQTransaction3 {
    const NAME: &'static str = "Windows.Win32.System.MessageQueuing.IMSMQTransaction3";
}
#[cfg(feature = "Win32_System_Com")]
impl IMSMQTransaction3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQTransaction3Impl, const OFFSET: isize>() -> IMSMQTransaction3Vtbl {
        unsafe extern "system" fn ITransaction<Impl: IMSMQTransaction3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvaritransaction: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ITransaction(::core::mem::transmute_copy(&pvaritransaction)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMSMQTransaction3>, ::windows::core::GetTrustLevel, ITransaction::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSMQTransactionDispenserImpl: Sized + IDispatchImpl {
    fn BeginTransaction();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IMSMQTransactionDispenser {
    const NAME: &'static str = "Windows.Win32.System.MessageQueuing.IMSMQTransactionDispenser";
}
#[cfg(feature = "Win32_System_Com")]
impl IMSMQTransactionDispenserVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQTransactionDispenserImpl, const OFFSET: isize>() -> IMSMQTransactionDispenserVtbl {
        unsafe extern "system" fn BeginTransaction<Impl: IMSMQTransactionDispenserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptransaction: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BeginTransaction(::core::mem::transmute_copy(&ptransaction)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMSMQTransactionDispenser>, ::windows::core::GetTrustLevel, BeginTransaction::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSMQTransactionDispenser2Impl: Sized + IDispatchImpl {
    fn BeginTransaction();
    fn Properties();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IMSMQTransactionDispenser2 {
    const NAME: &'static str = "Windows.Win32.System.MessageQueuing.IMSMQTransactionDispenser2";
}
#[cfg(feature = "Win32_System_Com")]
impl IMSMQTransactionDispenser2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQTransactionDispenser2Impl, const OFFSET: isize>() -> IMSMQTransactionDispenser2Vtbl {
        unsafe extern "system" fn BeginTransaction<Impl: IMSMQTransactionDispenser2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptransaction: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BeginTransaction(::core::mem::transmute_copy(&ptransaction)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Impl: IMSMQTransactionDispenser2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Properties(::core::mem::transmute_copy(&ppcolproperties)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMSMQTransactionDispenser2>, ::windows::core::GetTrustLevel, BeginTransaction::<Impl, OFFSET>, Properties::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSMQTransactionDispenser3Impl: Sized + IDispatchImpl {
    fn BeginTransaction();
    fn Properties();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IMSMQTransactionDispenser3 {
    const NAME: &'static str = "Windows.Win32.System.MessageQueuing.IMSMQTransactionDispenser3";
}
#[cfg(feature = "Win32_System_Com")]
impl IMSMQTransactionDispenser3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMSMQTransactionDispenser3Impl, const OFFSET: isize>() -> IMSMQTransactionDispenser3Vtbl {
        unsafe extern "system" fn BeginTransaction<Impl: IMSMQTransactionDispenser3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptransaction: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BeginTransaction(::core::mem::transmute_copy(&ptransaction)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Impl: IMSMQTransactionDispenser3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcolproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Properties(::core::mem::transmute_copy(&ppcolproperties)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMSMQTransactionDispenser3>, ::windows::core::GetTrustLevel, BeginTransaction::<Impl, OFFSET>, Properties::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait _DMSMQEventEventsImpl: Sized + IDispatchImpl {}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for _DMSMQEventEvents {
    const NAME: &'static str = "Windows.Win32.System.MessageQueuing._DMSMQEventEvents";
}
#[cfg(feature = "Win32_System_Com")]
impl _DMSMQEventEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: _DMSMQEventEventsImpl, const OFFSET: isize>() -> _DMSMQEventEventsVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<_DMSMQEventEvents>, ::windows::core::GetTrustLevel)
    }
}
