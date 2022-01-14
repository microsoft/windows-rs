#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWindowsMediaLibrarySharingDevice_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn DeviceID(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Authorization(&mut self) -> ::windows::core::Result<WindowsMediaLibrarySharingDeviceAuthorizationStatus>;
    fn SetAuthorization(&mut self, authorization: WindowsMediaLibrarySharingDeviceAuthorizationStatus) -> ::windows::core::Result<()>;
    fn Properties(&mut self) -> ::windows::core::Result<IWindowsMediaLibrarySharingDeviceProperties>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWindowsMediaLibrarySharingDevice_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWindowsMediaLibrarySharingDevice_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWindowsMediaLibrarySharingDevice_Vtbl {
        unsafe extern "system" fn DeviceID<Impl: IWindowsMediaLibrarySharingDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeviceID() {
                ::core::result::Result::Ok(ok__) => {
                    *deviceid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Authorization<Impl: IWindowsMediaLibrarySharingDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, authorization: *mut WindowsMediaLibrarySharingDeviceAuthorizationStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Authorization() {
                ::core::result::Result::Ok(ok__) => {
                    *authorization = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAuthorization<Impl: IWindowsMediaLibrarySharingDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, authorization: WindowsMediaLibrarySharingDeviceAuthorizationStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAuthorization(::core::mem::transmute_copy(&authorization)).into()
        }
        unsafe extern "system" fn Properties<Impl: IWindowsMediaLibrarySharingDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *deviceproperties = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            DeviceID: DeviceID::<Impl, IMPL_OFFSET>,
            Authorization: Authorization::<Impl, IMPL_OFFSET>,
            SetAuthorization: SetAuthorization::<Impl, IMPL_OFFSET>,
            Properties: Properties::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWindowsMediaLibrarySharingDevice as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWindowsMediaLibrarySharingDeviceProperties_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Item(&mut self, index: i32) -> ::windows::core::Result<IWindowsMediaLibrarySharingDeviceProperty>;
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn GetProperty(&mut self, name: super::super::Foundation::BSTR) -> ::windows::core::Result<IWindowsMediaLibrarySharingDeviceProperty>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWindowsMediaLibrarySharingDeviceProperties_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWindowsMediaLibrarySharingDeviceProperties_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWindowsMediaLibrarySharingDeviceProperties_Vtbl {
        unsafe extern "system" fn Item<Impl: IWindowsMediaLibrarySharingDeviceProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, property: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *property = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: IWindowsMediaLibrarySharingDeviceProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *count = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProperty<Impl: IWindowsMediaLibrarySharingDeviceProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, property: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProperty(::core::mem::transmute_copy(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    *property = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Item: Item::<Impl, IMPL_OFFSET>,
            Count: Count::<Impl, IMPL_OFFSET>,
            GetProperty: GetProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWindowsMediaLibrarySharingDeviceProperties as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWindowsMediaLibrarySharingDeviceProperty_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Name(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Value(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWindowsMediaLibrarySharingDeviceProperty_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWindowsMediaLibrarySharingDeviceProperty_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWindowsMediaLibrarySharingDeviceProperty_Vtbl {
        unsafe extern "system" fn Name<Impl: IWindowsMediaLibrarySharingDeviceProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *name = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Value<Impl: IWindowsMediaLibrarySharingDeviceProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Value() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Name: Name::<Impl, IMPL_OFFSET>,
            Value: Value::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWindowsMediaLibrarySharingDeviceProperty as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWindowsMediaLibrarySharingDevices_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Item(&mut self, index: i32) -> ::windows::core::Result<IWindowsMediaLibrarySharingDevice>;
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn GetDevice(&mut self, deviceid: super::super::Foundation::BSTR) -> ::windows::core::Result<IWindowsMediaLibrarySharingDevice>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWindowsMediaLibrarySharingDevices_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWindowsMediaLibrarySharingDevices_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWindowsMediaLibrarySharingDevices_Vtbl {
        unsafe extern "system" fn Item<Impl: IWindowsMediaLibrarySharingDevices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, device: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *device = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: IWindowsMediaLibrarySharingDevices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *count = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDevice<Impl: IWindowsMediaLibrarySharingDevices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, device: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDevice(::core::mem::transmute_copy(&deviceid)) {
                ::core::result::Result::Ok(ok__) => {
                    *device = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Item: Item::<Impl, IMPL_OFFSET>,
            Count: Count::<Impl, IMPL_OFFSET>,
            GetDevice: GetDevice::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWindowsMediaLibrarySharingDevices as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWindowsMediaLibrarySharingServices_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn showShareMediaCPL(&mut self, device: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn userHomeMediaSharingState(&mut self) -> ::windows::core::Result<i16>;
    fn SetuserHomeMediaSharingState(&mut self, sharingenabled: i16) -> ::windows::core::Result<()>;
    fn userHomeMediaSharingLibraryName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetuserHomeMediaSharingLibraryName(&mut self, libraryname: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn computerHomeMediaSharingAllowedState(&mut self) -> ::windows::core::Result<i16>;
    fn SetcomputerHomeMediaSharingAllowedState(&mut self, sharingallowed: i16) -> ::windows::core::Result<()>;
    fn userInternetMediaSharingState(&mut self) -> ::windows::core::Result<i16>;
    fn SetuserInternetMediaSharingState(&mut self, sharingenabled: i16) -> ::windows::core::Result<()>;
    fn computerInternetMediaSharingAllowedState(&mut self) -> ::windows::core::Result<i16>;
    fn SetcomputerInternetMediaSharingAllowedState(&mut self, sharingallowed: i16) -> ::windows::core::Result<()>;
    fn internetMediaSharingSecurityGroup(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetinternetMediaSharingSecurityGroup(&mut self, securitygroup: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn allowSharingToAllDevices(&mut self) -> ::windows::core::Result<i16>;
    fn SetallowSharingToAllDevices(&mut self, sharingenabled: i16) -> ::windows::core::Result<()>;
    fn setDefaultAuthorization(&mut self, macaddresses: super::super::Foundation::BSTR, friendlyname: super::super::Foundation::BSTR, authorization: i16) -> ::windows::core::Result<()>;
    fn setAuthorizationState(&mut self, macaddress: super::super::Foundation::BSTR, authorizationstate: i16) -> ::windows::core::Result<()>;
    fn getAllDevices(&mut self) -> ::windows::core::Result<IWindowsMediaLibrarySharingDevices>;
    fn customSettingsApplied(&mut self) -> ::windows::core::Result<i16>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWindowsMediaLibrarySharingServices_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWindowsMediaLibrarySharingServices_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWindowsMediaLibrarySharingServices_Vtbl {
        unsafe extern "system" fn showShareMediaCPL<Impl: IWindowsMediaLibrarySharingServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, device: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).showShareMediaCPL(::core::mem::transmute_copy(&device)).into()
        }
        unsafe extern "system" fn userHomeMediaSharingState<Impl: IWindowsMediaLibrarySharingServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sharingenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).userHomeMediaSharingState() {
                ::core::result::Result::Ok(ok__) => {
                    *sharingenabled = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetuserHomeMediaSharingState<Impl: IWindowsMediaLibrarySharingServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sharingenabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetuserHomeMediaSharingState(::core::mem::transmute_copy(&sharingenabled)).into()
        }
        unsafe extern "system" fn userHomeMediaSharingLibraryName<Impl: IWindowsMediaLibrarySharingServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, libraryname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).userHomeMediaSharingLibraryName() {
                ::core::result::Result::Ok(ok__) => {
                    *libraryname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetuserHomeMediaSharingLibraryName<Impl: IWindowsMediaLibrarySharingServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, libraryname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetuserHomeMediaSharingLibraryName(::core::mem::transmute_copy(&libraryname)).into()
        }
        unsafe extern "system" fn computerHomeMediaSharingAllowedState<Impl: IWindowsMediaLibrarySharingServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sharingallowed: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).computerHomeMediaSharingAllowedState() {
                ::core::result::Result::Ok(ok__) => {
                    *sharingallowed = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetcomputerHomeMediaSharingAllowedState<Impl: IWindowsMediaLibrarySharingServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sharingallowed: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetcomputerHomeMediaSharingAllowedState(::core::mem::transmute_copy(&sharingallowed)).into()
        }
        unsafe extern "system" fn userInternetMediaSharingState<Impl: IWindowsMediaLibrarySharingServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sharingenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).userInternetMediaSharingState() {
                ::core::result::Result::Ok(ok__) => {
                    *sharingenabled = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetuserInternetMediaSharingState<Impl: IWindowsMediaLibrarySharingServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sharingenabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetuserInternetMediaSharingState(::core::mem::transmute_copy(&sharingenabled)).into()
        }
        unsafe extern "system" fn computerInternetMediaSharingAllowedState<Impl: IWindowsMediaLibrarySharingServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sharingallowed: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).computerInternetMediaSharingAllowedState() {
                ::core::result::Result::Ok(ok__) => {
                    *sharingallowed = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetcomputerInternetMediaSharingAllowedState<Impl: IWindowsMediaLibrarySharingServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sharingallowed: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetcomputerInternetMediaSharingAllowedState(::core::mem::transmute_copy(&sharingallowed)).into()
        }
        unsafe extern "system" fn internetMediaSharingSecurityGroup<Impl: IWindowsMediaLibrarySharingServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, securitygroup: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).internetMediaSharingSecurityGroup() {
                ::core::result::Result::Ok(ok__) => {
                    *securitygroup = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetinternetMediaSharingSecurityGroup<Impl: IWindowsMediaLibrarySharingServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, securitygroup: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetinternetMediaSharingSecurityGroup(::core::mem::transmute_copy(&securitygroup)).into()
        }
        unsafe extern "system" fn allowSharingToAllDevices<Impl: IWindowsMediaLibrarySharingServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sharingenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).allowSharingToAllDevices() {
                ::core::result::Result::Ok(ok__) => {
                    *sharingenabled = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetallowSharingToAllDevices<Impl: IWindowsMediaLibrarySharingServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sharingenabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetallowSharingToAllDevices(::core::mem::transmute_copy(&sharingenabled)).into()
        }
        unsafe extern "system" fn setDefaultAuthorization<Impl: IWindowsMediaLibrarySharingServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, macaddresses: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, friendlyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, authorization: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).setDefaultAuthorization(::core::mem::transmute_copy(&macaddresses), ::core::mem::transmute_copy(&friendlyname), ::core::mem::transmute_copy(&authorization)).into()
        }
        unsafe extern "system" fn setAuthorizationState<Impl: IWindowsMediaLibrarySharingServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, macaddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, authorizationstate: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).setAuthorizationState(::core::mem::transmute_copy(&macaddress), ::core::mem::transmute_copy(&authorizationstate)).into()
        }
        unsafe extern "system" fn getAllDevices<Impl: IWindowsMediaLibrarySharingServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, devices: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).getAllDevices() {
                ::core::result::Result::Ok(ok__) => {
                    *devices = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn customSettingsApplied<Impl: IWindowsMediaLibrarySharingServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, customsettingsapplied: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).customSettingsApplied() {
                ::core::result::Result::Ok(ok__) => {
                    *customsettingsapplied = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            showShareMediaCPL: showShareMediaCPL::<Impl, IMPL_OFFSET>,
            userHomeMediaSharingState: userHomeMediaSharingState::<Impl, IMPL_OFFSET>,
            SetuserHomeMediaSharingState: SetuserHomeMediaSharingState::<Impl, IMPL_OFFSET>,
            userHomeMediaSharingLibraryName: userHomeMediaSharingLibraryName::<Impl, IMPL_OFFSET>,
            SetuserHomeMediaSharingLibraryName: SetuserHomeMediaSharingLibraryName::<Impl, IMPL_OFFSET>,
            computerHomeMediaSharingAllowedState: computerHomeMediaSharingAllowedState::<Impl, IMPL_OFFSET>,
            SetcomputerHomeMediaSharingAllowedState: SetcomputerHomeMediaSharingAllowedState::<Impl, IMPL_OFFSET>,
            userInternetMediaSharingState: userInternetMediaSharingState::<Impl, IMPL_OFFSET>,
            SetuserInternetMediaSharingState: SetuserInternetMediaSharingState::<Impl, IMPL_OFFSET>,
            computerInternetMediaSharingAllowedState: computerInternetMediaSharingAllowedState::<Impl, IMPL_OFFSET>,
            SetcomputerInternetMediaSharingAllowedState: SetcomputerInternetMediaSharingAllowedState::<Impl, IMPL_OFFSET>,
            internetMediaSharingSecurityGroup: internetMediaSharingSecurityGroup::<Impl, IMPL_OFFSET>,
            SetinternetMediaSharingSecurityGroup: SetinternetMediaSharingSecurityGroup::<Impl, IMPL_OFFSET>,
            allowSharingToAllDevices: allowSharingToAllDevices::<Impl, IMPL_OFFSET>,
            SetallowSharingToAllDevices: SetallowSharingToAllDevices::<Impl, IMPL_OFFSET>,
            setDefaultAuthorization: setDefaultAuthorization::<Impl, IMPL_OFFSET>,
            setAuthorizationState: setAuthorizationState::<Impl, IMPL_OFFSET>,
            getAllDevices: getAllDevices::<Impl, IMPL_OFFSET>,
            customSettingsApplied: customSettingsApplied::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWindowsMediaLibrarySharingServices as ::windows::core::Interface>::IID
    }
}
