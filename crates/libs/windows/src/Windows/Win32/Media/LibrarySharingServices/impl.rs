#[doc = "*Required features: `\"Win32_Media_LibrarySharingServices\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWindowsMediaLibrarySharingDevice_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn DeviceID(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Authorization(&self) -> ::windows_core::Result<WindowsMediaLibrarySharingDeviceAuthorizationStatus>;
    fn SetAuthorization(&self, authorization: WindowsMediaLibrarySharingDeviceAuthorizationStatus) -> ::windows_core::Result<()>;
    fn Properties(&self) -> ::windows_core::Result<IWindowsMediaLibrarySharingDeviceProperties>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IWindowsMediaLibrarySharingDevice {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IWindowsMediaLibrarySharingDevice_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWindowsMediaLibrarySharingDevice_Impl, const OFFSET: isize>() -> IWindowsMediaLibrarySharingDevice_Vtbl {
        unsafe extern "system" fn DeviceID<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWindowsMediaLibrarySharingDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DeviceID() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(deviceid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Authorization<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWindowsMediaLibrarySharingDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, authorization: *mut WindowsMediaLibrarySharingDeviceAuthorizationStatus) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Authorization() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(authorization, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAuthorization<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWindowsMediaLibrarySharingDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, authorization: WindowsMediaLibrarySharingDeviceAuthorizationStatus) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAuthorization(::core::mem::transmute_copy(&authorization)).into()
        }
        unsafe extern "system" fn Properties<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWindowsMediaLibrarySharingDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceproperties: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Properties() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(deviceproperties, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            DeviceID: DeviceID::<Identity, Impl, OFFSET>,
            Authorization: Authorization::<Identity, Impl, OFFSET>,
            SetAuthorization: SetAuthorization::<Identity, Impl, OFFSET>,
            Properties: Properties::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IWindowsMediaLibrarySharingDevice as ::windows_core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_LibrarySharingServices\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWindowsMediaLibrarySharingDeviceProperties_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn get_Item(&self, index: i32) -> ::windows_core::Result<IWindowsMediaLibrarySharingDeviceProperty>;
    fn Count(&self) -> ::windows_core::Result<i32>;
    fn GetProperty(&self, name: &::windows_core::BSTR) -> ::windows_core::Result<IWindowsMediaLibrarySharingDeviceProperty>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IWindowsMediaLibrarySharingDeviceProperties {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IWindowsMediaLibrarySharingDeviceProperties_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWindowsMediaLibrarySharingDeviceProperties_Impl, const OFFSET: isize>() -> IWindowsMediaLibrarySharingDeviceProperties_Vtbl {
        unsafe extern "system" fn get_Item<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWindowsMediaLibrarySharingDeviceProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, property: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(property, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWindowsMediaLibrarySharingDeviceProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(count, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProperty<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWindowsMediaLibrarySharingDeviceProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::BSTR>, property: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetProperty(::core::mem::transmute(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(property, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            GetProperty: GetProperty::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IWindowsMediaLibrarySharingDeviceProperties as ::windows_core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_LibrarySharingServices\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWindowsMediaLibrarySharingDeviceProperty_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Name(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Value(&self) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IWindowsMediaLibrarySharingDeviceProperty {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IWindowsMediaLibrarySharingDeviceProperty_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWindowsMediaLibrarySharingDeviceProperty_Impl, const OFFSET: isize>() -> IWindowsMediaLibrarySharingDeviceProperty_Vtbl {
        unsafe extern "system" fn Name<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWindowsMediaLibrarySharingDeviceProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Name() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(name, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Value<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWindowsMediaLibrarySharingDeviceProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Value() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Name: Name::<Identity, Impl, OFFSET>,
            Value: Value::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IWindowsMediaLibrarySharingDeviceProperty as ::windows_core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_LibrarySharingServices\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWindowsMediaLibrarySharingDevices_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn get_Item(&self, index: i32) -> ::windows_core::Result<IWindowsMediaLibrarySharingDevice>;
    fn Count(&self) -> ::windows_core::Result<i32>;
    fn GetDevice(&self, deviceid: &::windows_core::BSTR) -> ::windows_core::Result<IWindowsMediaLibrarySharingDevice>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IWindowsMediaLibrarySharingDevices {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IWindowsMediaLibrarySharingDevices_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWindowsMediaLibrarySharingDevices_Impl, const OFFSET: isize>() -> IWindowsMediaLibrarySharingDevices_Vtbl {
        unsafe extern "system" fn get_Item<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWindowsMediaLibrarySharingDevices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, device: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(device, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWindowsMediaLibrarySharingDevices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(count, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDevice<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWindowsMediaLibrarySharingDevices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceid: ::std::mem::MaybeUninit<::windows_core::BSTR>, device: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDevice(::core::mem::transmute(&deviceid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(device, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            GetDevice: GetDevice::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IWindowsMediaLibrarySharingDevices as ::windows_core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_LibrarySharingServices\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWindowsMediaLibrarySharingServices_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn showShareMediaCPL(&self, device: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn userHomeMediaSharingState(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetuserHomeMediaSharingState(&self, sharingenabled: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn userHomeMediaSharingLibraryName(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetuserHomeMediaSharingLibraryName(&self, libraryname: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn computerHomeMediaSharingAllowedState(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetcomputerHomeMediaSharingAllowedState(&self, sharingallowed: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn userInternetMediaSharingState(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetuserInternetMediaSharingState(&self, sharingenabled: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn computerInternetMediaSharingAllowedState(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetcomputerInternetMediaSharingAllowedState(&self, sharingallowed: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn internetMediaSharingSecurityGroup(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetinternetMediaSharingSecurityGroup(&self, securitygroup: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn allowSharingToAllDevices(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetallowSharingToAllDevices(&self, sharingenabled: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn setDefaultAuthorization(&self, macaddresses: &::windows_core::BSTR, friendlyname: &::windows_core::BSTR, authorization: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn setAuthorizationState(&self, macaddress: &::windows_core::BSTR, authorizationstate: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn getAllDevices(&self) -> ::windows_core::Result<IWindowsMediaLibrarySharingDevices>;
    fn customSettingsApplied(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IWindowsMediaLibrarySharingServices {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IWindowsMediaLibrarySharingServices_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWindowsMediaLibrarySharingServices_Impl, const OFFSET: isize>() -> IWindowsMediaLibrarySharingServices_Vtbl {
        unsafe extern "system" fn showShareMediaCPL<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWindowsMediaLibrarySharingServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, device: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.showShareMediaCPL(::core::mem::transmute(&device)).into()
        }
        unsafe extern "system" fn userHomeMediaSharingState<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWindowsMediaLibrarySharingServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sharingenabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.userHomeMediaSharingState() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(sharingenabled, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetuserHomeMediaSharingState<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWindowsMediaLibrarySharingServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sharingenabled: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetuserHomeMediaSharingState(::core::mem::transmute_copy(&sharingenabled)).into()
        }
        unsafe extern "system" fn userHomeMediaSharingLibraryName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWindowsMediaLibrarySharingServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, libraryname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.userHomeMediaSharingLibraryName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(libraryname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetuserHomeMediaSharingLibraryName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWindowsMediaLibrarySharingServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, libraryname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetuserHomeMediaSharingLibraryName(::core::mem::transmute(&libraryname)).into()
        }
        unsafe extern "system" fn computerHomeMediaSharingAllowedState<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWindowsMediaLibrarySharingServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sharingallowed: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.computerHomeMediaSharingAllowedState() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(sharingallowed, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetcomputerHomeMediaSharingAllowedState<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWindowsMediaLibrarySharingServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sharingallowed: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetcomputerHomeMediaSharingAllowedState(::core::mem::transmute_copy(&sharingallowed)).into()
        }
        unsafe extern "system" fn userInternetMediaSharingState<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWindowsMediaLibrarySharingServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sharingenabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.userInternetMediaSharingState() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(sharingenabled, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetuserInternetMediaSharingState<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWindowsMediaLibrarySharingServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sharingenabled: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetuserInternetMediaSharingState(::core::mem::transmute_copy(&sharingenabled)).into()
        }
        unsafe extern "system" fn computerInternetMediaSharingAllowedState<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWindowsMediaLibrarySharingServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sharingallowed: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.computerInternetMediaSharingAllowedState() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(sharingallowed, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetcomputerInternetMediaSharingAllowedState<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWindowsMediaLibrarySharingServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sharingallowed: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetcomputerInternetMediaSharingAllowedState(::core::mem::transmute_copy(&sharingallowed)).into()
        }
        unsafe extern "system" fn internetMediaSharingSecurityGroup<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWindowsMediaLibrarySharingServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, securitygroup: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.internetMediaSharingSecurityGroup() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(securitygroup, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetinternetMediaSharingSecurityGroup<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWindowsMediaLibrarySharingServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, securitygroup: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetinternetMediaSharingSecurityGroup(::core::mem::transmute(&securitygroup)).into()
        }
        unsafe extern "system" fn allowSharingToAllDevices<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWindowsMediaLibrarySharingServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sharingenabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.allowSharingToAllDevices() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(sharingenabled, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetallowSharingToAllDevices<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWindowsMediaLibrarySharingServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sharingenabled: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetallowSharingToAllDevices(::core::mem::transmute_copy(&sharingenabled)).into()
        }
        unsafe extern "system" fn setDefaultAuthorization<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWindowsMediaLibrarySharingServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, macaddresses: ::std::mem::MaybeUninit<::windows_core::BSTR>, friendlyname: ::std::mem::MaybeUninit<::windows_core::BSTR>, authorization: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.setDefaultAuthorization(::core::mem::transmute(&macaddresses), ::core::mem::transmute(&friendlyname), ::core::mem::transmute_copy(&authorization)).into()
        }
        unsafe extern "system" fn setAuthorizationState<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWindowsMediaLibrarySharingServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, macaddress: ::std::mem::MaybeUninit<::windows_core::BSTR>, authorizationstate: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.setAuthorizationState(::core::mem::transmute(&macaddress), ::core::mem::transmute_copy(&authorizationstate)).into()
        }
        unsafe extern "system" fn getAllDevices<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWindowsMediaLibrarySharingServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, devices: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.getAllDevices() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(devices, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn customSettingsApplied<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWindowsMediaLibrarySharingServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, customsettingsapplied: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.customSettingsApplied() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(customsettingsapplied, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            showShareMediaCPL: showShareMediaCPL::<Identity, Impl, OFFSET>,
            userHomeMediaSharingState: userHomeMediaSharingState::<Identity, Impl, OFFSET>,
            SetuserHomeMediaSharingState: SetuserHomeMediaSharingState::<Identity, Impl, OFFSET>,
            userHomeMediaSharingLibraryName: userHomeMediaSharingLibraryName::<Identity, Impl, OFFSET>,
            SetuserHomeMediaSharingLibraryName: SetuserHomeMediaSharingLibraryName::<Identity, Impl, OFFSET>,
            computerHomeMediaSharingAllowedState: computerHomeMediaSharingAllowedState::<Identity, Impl, OFFSET>,
            SetcomputerHomeMediaSharingAllowedState: SetcomputerHomeMediaSharingAllowedState::<Identity, Impl, OFFSET>,
            userInternetMediaSharingState: userInternetMediaSharingState::<Identity, Impl, OFFSET>,
            SetuserInternetMediaSharingState: SetuserInternetMediaSharingState::<Identity, Impl, OFFSET>,
            computerInternetMediaSharingAllowedState: computerInternetMediaSharingAllowedState::<Identity, Impl, OFFSET>,
            SetcomputerInternetMediaSharingAllowedState: SetcomputerInternetMediaSharingAllowedState::<Identity, Impl, OFFSET>,
            internetMediaSharingSecurityGroup: internetMediaSharingSecurityGroup::<Identity, Impl, OFFSET>,
            SetinternetMediaSharingSecurityGroup: SetinternetMediaSharingSecurityGroup::<Identity, Impl, OFFSET>,
            allowSharingToAllDevices: allowSharingToAllDevices::<Identity, Impl, OFFSET>,
            SetallowSharingToAllDevices: SetallowSharingToAllDevices::<Identity, Impl, OFFSET>,
            setDefaultAuthorization: setDefaultAuthorization::<Identity, Impl, OFFSET>,
            setAuthorizationState: setAuthorizationState::<Identity, Impl, OFFSET>,
            getAllDevices: getAllDevices::<Identity, Impl, OFFSET>,
            customSettingsApplied: customSettingsApplied::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IWindowsMediaLibrarySharingServices as ::windows_core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
