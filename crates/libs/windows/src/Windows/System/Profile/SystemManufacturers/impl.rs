#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IOemSupportInfoImpl: Sized {
    fn SupportLink(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Uri>;
    fn SupportAppLink(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Uri>;
    fn SupportProvider(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IOemSupportInfo {
    const NAME: &'static str = "Windows.System.Profile.SystemManufacturers.IOemSupportInfo";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IOemSupportInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOemSupportInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOemSupportInfoVtbl {
        unsafe extern "system" fn SupportLink<Impl: IOemSupportInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportLink() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportAppLink<Impl: IOemSupportInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportAppLink() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportProvider<Impl: IOemSupportInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportProvider() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IOemSupportInfo, BASE_OFFSET>(),
            SupportLink: SupportLink::<Impl, IMPL_OFFSET>,
            SupportAppLink: SupportAppLink::<Impl, IMPL_OFFSET>,
            SupportProvider: SupportProvider::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOemSupportInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISmbiosInformationStaticsImpl: Sized {
    fn SerialNumber(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISmbiosInformationStatics {
    const NAME: &'static str = "Windows.System.Profile.SystemManufacturers.ISmbiosInformationStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ISmbiosInformationStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISmbiosInformationStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISmbiosInformationStaticsVtbl {
        unsafe extern "system" fn SerialNumber<Impl: ISmbiosInformationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SerialNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISmbiosInformationStatics, BASE_OFFSET>(),
            SerialNumber: SerialNumber::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISmbiosInformationStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISystemSupportDeviceInfoImpl: Sized {
    fn OperatingSystem(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FriendlyName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SystemManufacturer(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SystemProductName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SystemSku(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SystemHardwareVersion(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SystemFirmwareVersion(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISystemSupportDeviceInfo {
    const NAME: &'static str = "Windows.System.Profile.SystemManufacturers.ISystemSupportDeviceInfo";
}
#[cfg(feature = "implement_exclusive")]
impl ISystemSupportDeviceInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISystemSupportDeviceInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISystemSupportDeviceInfoVtbl {
        unsafe extern "system" fn OperatingSystem<Impl: ISystemSupportDeviceInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OperatingSystem() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FriendlyName<Impl: ISystemSupportDeviceInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FriendlyName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SystemManufacturer<Impl: ISystemSupportDeviceInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SystemManufacturer() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SystemProductName<Impl: ISystemSupportDeviceInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SystemProductName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SystemSku<Impl: ISystemSupportDeviceInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SystemSku() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SystemHardwareVersion<Impl: ISystemSupportDeviceInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SystemHardwareVersion() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SystemFirmwareVersion<Impl: ISystemSupportDeviceInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SystemFirmwareVersion() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISystemSupportDeviceInfo, BASE_OFFSET>(),
            OperatingSystem: OperatingSystem::<Impl, IMPL_OFFSET>,
            FriendlyName: FriendlyName::<Impl, IMPL_OFFSET>,
            SystemManufacturer: SystemManufacturer::<Impl, IMPL_OFFSET>,
            SystemProductName: SystemProductName::<Impl, IMPL_OFFSET>,
            SystemSku: SystemSku::<Impl, IMPL_OFFSET>,
            SystemHardwareVersion: SystemHardwareVersion::<Impl, IMPL_OFFSET>,
            SystemFirmwareVersion: SystemFirmwareVersion::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISystemSupportDeviceInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISystemSupportInfoStaticsImpl: Sized {
    fn LocalSystemEdition(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn OemSupportInfo(&mut self) -> ::windows::core::Result<OemSupportInfo>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISystemSupportInfoStatics {
    const NAME: &'static str = "Windows.System.Profile.SystemManufacturers.ISystemSupportInfoStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ISystemSupportInfoStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISystemSupportInfoStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISystemSupportInfoStaticsVtbl {
        unsafe extern "system" fn LocalSystemEdition<Impl: ISystemSupportInfoStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LocalSystemEdition() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OemSupportInfo<Impl: ISystemSupportInfoStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OemSupportInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISystemSupportInfoStatics, BASE_OFFSET>(),
            LocalSystemEdition: LocalSystemEdition::<Impl, IMPL_OFFSET>,
            OemSupportInfo: OemSupportInfo::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISystemSupportInfoStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISystemSupportInfoStatics2Impl: Sized {
    fn LocalDeviceInfo(&mut self) -> ::windows::core::Result<SystemSupportDeviceInfo>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISystemSupportInfoStatics2 {
    const NAME: &'static str = "Windows.System.Profile.SystemManufacturers.ISystemSupportInfoStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl ISystemSupportInfoStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISystemSupportInfoStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISystemSupportInfoStatics2Vtbl {
        unsafe extern "system" fn LocalDeviceInfo<Impl: ISystemSupportInfoStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LocalDeviceInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISystemSupportInfoStatics2, BASE_OFFSET>(),
            LocalDeviceInfo: LocalDeviceInfo::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISystemSupportInfoStatics2 as ::windows::core::Interface>::IID
    }
}
