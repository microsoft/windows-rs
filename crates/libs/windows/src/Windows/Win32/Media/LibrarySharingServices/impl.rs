#[cfg(feature = "Win32_System_Com")]
pub trait IWindowsMediaLibrarySharingDeviceImpl: Sized + IDispatchImpl {
    fn DeviceID();
    fn Authorization();
    fn SetAuthorization();
    fn Properties();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IWindowsMediaLibrarySharingDevice {
    const NAME: &'static str = "Windows.Win32.Media.LibrarySharingServices.IWindowsMediaLibrarySharingDevice";
}
#[cfg(feature = "Win32_System_Com")]
impl IWindowsMediaLibrarySharingDeviceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWindowsMediaLibrarySharingDeviceImpl, const OFFSET: isize>() -> IWindowsMediaLibrarySharingDeviceVtbl {
        unsafe extern "system" fn DeviceID<Impl: IWindowsMediaLibrarySharingDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeviceID(::core::mem::transmute_copy(&deviceid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Authorization<Impl: IWindowsMediaLibrarySharingDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, authorization: *mut WindowsMediaLibrarySharingDeviceAuthorizationStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Authorization(::core::mem::transmute_copy(&authorization)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAuthorization<Impl: IWindowsMediaLibrarySharingDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, authorization: WindowsMediaLibrarySharingDeviceAuthorizationStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAuthorization(authorization) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Impl: IWindowsMediaLibrarySharingDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Properties(::core::mem::transmute_copy(&deviceproperties)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWindowsMediaLibrarySharingDevice>, ::windows::core::GetTrustLevel, DeviceID::<Impl, OFFSET>, Authorization::<Impl, OFFSET>, SetAuthorization::<Impl, OFFSET>, Properties::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWindowsMediaLibrarySharingDevicePropertiesImpl: Sized + IDispatchImpl {
    fn Item();
    fn Count();
    fn GetProperty();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IWindowsMediaLibrarySharingDeviceProperties {
    const NAME: &'static str = "Windows.Win32.Media.LibrarySharingServices.IWindowsMediaLibrarySharingDeviceProperties";
}
#[cfg(feature = "Win32_System_Com")]
impl IWindowsMediaLibrarySharingDevicePropertiesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWindowsMediaLibrarySharingDevicePropertiesImpl, const OFFSET: isize>() -> IWindowsMediaLibrarySharingDevicePropertiesVtbl {
        unsafe extern "system" fn Item<Impl: IWindowsMediaLibrarySharingDevicePropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, property: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(index, ::core::mem::transmute_copy(&property)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: IWindowsMediaLibrarySharingDevicePropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count(::core::mem::transmute_copy(&count)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProperty<Impl: IWindowsMediaLibrarySharingDevicePropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, property: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProperty(&*(&name as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&property)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWindowsMediaLibrarySharingDeviceProperties>, ::windows::core::GetTrustLevel, Item::<Impl, OFFSET>, Count::<Impl, OFFSET>, GetProperty::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWindowsMediaLibrarySharingDevicePropertyImpl: Sized + IDispatchImpl {
    fn Name();
    fn Value();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IWindowsMediaLibrarySharingDeviceProperty {
    const NAME: &'static str = "Windows.Win32.Media.LibrarySharingServices.IWindowsMediaLibrarySharingDeviceProperty";
}
#[cfg(feature = "Win32_System_Com")]
impl IWindowsMediaLibrarySharingDevicePropertyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWindowsMediaLibrarySharingDevicePropertyImpl, const OFFSET: isize>() -> IWindowsMediaLibrarySharingDevicePropertyVtbl {
        unsafe extern "system" fn Name<Impl: IWindowsMediaLibrarySharingDevicePropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name(::core::mem::transmute_copy(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Value<Impl: IWindowsMediaLibrarySharingDevicePropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Value(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWindowsMediaLibrarySharingDeviceProperty>, ::windows::core::GetTrustLevel, Name::<Impl, OFFSET>, Value::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWindowsMediaLibrarySharingDevicesImpl: Sized + IDispatchImpl {
    fn Item();
    fn Count();
    fn GetDevice();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IWindowsMediaLibrarySharingDevices {
    const NAME: &'static str = "Windows.Win32.Media.LibrarySharingServices.IWindowsMediaLibrarySharingDevices";
}
#[cfg(feature = "Win32_System_Com")]
impl IWindowsMediaLibrarySharingDevicesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWindowsMediaLibrarySharingDevicesImpl, const OFFSET: isize>() -> IWindowsMediaLibrarySharingDevicesVtbl {
        unsafe extern "system" fn Item<Impl: IWindowsMediaLibrarySharingDevicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, device: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(index, ::core::mem::transmute_copy(&device)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: IWindowsMediaLibrarySharingDevicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count(::core::mem::transmute_copy(&count)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDevice<Impl: IWindowsMediaLibrarySharingDevicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, device: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDevice(&*(&deviceid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&device)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWindowsMediaLibrarySharingDevices>, ::windows::core::GetTrustLevel, Item::<Impl, OFFSET>, Count::<Impl, OFFSET>, GetDevice::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWindowsMediaLibrarySharingServicesImpl: Sized + IDispatchImpl {
    fn showShareMediaCPL();
    fn userHomeMediaSharingState();
    fn SetuserHomeMediaSharingState();
    fn userHomeMediaSharingLibraryName();
    fn SetuserHomeMediaSharingLibraryName();
    fn computerHomeMediaSharingAllowedState();
    fn SetcomputerHomeMediaSharingAllowedState();
    fn userInternetMediaSharingState();
    fn SetuserInternetMediaSharingState();
    fn computerInternetMediaSharingAllowedState();
    fn SetcomputerInternetMediaSharingAllowedState();
    fn internetMediaSharingSecurityGroup();
    fn SetinternetMediaSharingSecurityGroup();
    fn allowSharingToAllDevices();
    fn SetallowSharingToAllDevices();
    fn setDefaultAuthorization();
    fn setAuthorizationState();
    fn getAllDevices();
    fn customSettingsApplied();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IWindowsMediaLibrarySharingServices {
    const NAME: &'static str = "Windows.Win32.Media.LibrarySharingServices.IWindowsMediaLibrarySharingServices";
}
#[cfg(feature = "Win32_System_Com")]
impl IWindowsMediaLibrarySharingServicesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWindowsMediaLibrarySharingServicesImpl, const OFFSET: isize>() -> IWindowsMediaLibrarySharingServicesVtbl {
        unsafe extern "system" fn showShareMediaCPL<Impl: IWindowsMediaLibrarySharingServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, device: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).showShareMediaCPL(&*(&device as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn userHomeMediaSharingState<Impl: IWindowsMediaLibrarySharingServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sharingenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).userHomeMediaSharingState(::core::mem::transmute_copy(&sharingenabled)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetuserHomeMediaSharingState<Impl: IWindowsMediaLibrarySharingServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sharingenabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetuserHomeMediaSharingState(sharingenabled) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn userHomeMediaSharingLibraryName<Impl: IWindowsMediaLibrarySharingServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, libraryname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).userHomeMediaSharingLibraryName(::core::mem::transmute_copy(&libraryname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetuserHomeMediaSharingLibraryName<Impl: IWindowsMediaLibrarySharingServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, libraryname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetuserHomeMediaSharingLibraryName(&*(&libraryname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn computerHomeMediaSharingAllowedState<Impl: IWindowsMediaLibrarySharingServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sharingallowed: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).computerHomeMediaSharingAllowedState(::core::mem::transmute_copy(&sharingallowed)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetcomputerHomeMediaSharingAllowedState<Impl: IWindowsMediaLibrarySharingServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sharingallowed: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetcomputerHomeMediaSharingAllowedState(sharingallowed) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn userInternetMediaSharingState<Impl: IWindowsMediaLibrarySharingServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sharingenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).userInternetMediaSharingState(::core::mem::transmute_copy(&sharingenabled)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetuserInternetMediaSharingState<Impl: IWindowsMediaLibrarySharingServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sharingenabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetuserInternetMediaSharingState(sharingenabled) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn computerInternetMediaSharingAllowedState<Impl: IWindowsMediaLibrarySharingServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sharingallowed: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).computerInternetMediaSharingAllowedState(::core::mem::transmute_copy(&sharingallowed)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetcomputerInternetMediaSharingAllowedState<Impl: IWindowsMediaLibrarySharingServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sharingallowed: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetcomputerInternetMediaSharingAllowedState(sharingallowed) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn internetMediaSharingSecurityGroup<Impl: IWindowsMediaLibrarySharingServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, securitygroup: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).internetMediaSharingSecurityGroup(::core::mem::transmute_copy(&securitygroup)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetinternetMediaSharingSecurityGroup<Impl: IWindowsMediaLibrarySharingServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, securitygroup: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetinternetMediaSharingSecurityGroup(&*(&securitygroup as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn allowSharingToAllDevices<Impl: IWindowsMediaLibrarySharingServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sharingenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).allowSharingToAllDevices(::core::mem::transmute_copy(&sharingenabled)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetallowSharingToAllDevices<Impl: IWindowsMediaLibrarySharingServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sharingenabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetallowSharingToAllDevices(sharingenabled) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn setDefaultAuthorization<Impl: IWindowsMediaLibrarySharingServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, macaddresses: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, friendlyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, authorization: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).setDefaultAuthorization(&*(&macaddresses as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&friendlyname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), authorization) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn setAuthorizationState<Impl: IWindowsMediaLibrarySharingServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, macaddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, authorizationstate: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).setAuthorizationState(&*(&macaddress as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), authorizationstate) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getAllDevices<Impl: IWindowsMediaLibrarySharingServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, devices: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).getAllDevices(::core::mem::transmute_copy(&devices)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn customSettingsApplied<Impl: IWindowsMediaLibrarySharingServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, customsettingsapplied: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).customSettingsApplied(::core::mem::transmute_copy(&customsettingsapplied)) {
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
            ::windows::core::GetRuntimeClassName::<IWindowsMediaLibrarySharingServices>,
            ::windows::core::GetTrustLevel,
            showShareMediaCPL::<Impl, OFFSET>,
            userHomeMediaSharingState::<Impl, OFFSET>,
            SetuserHomeMediaSharingState::<Impl, OFFSET>,
            userHomeMediaSharingLibraryName::<Impl, OFFSET>,
            SetuserHomeMediaSharingLibraryName::<Impl, OFFSET>,
            computerHomeMediaSharingAllowedState::<Impl, OFFSET>,
            SetcomputerHomeMediaSharingAllowedState::<Impl, OFFSET>,
            userInternetMediaSharingState::<Impl, OFFSET>,
            SetuserInternetMediaSharingState::<Impl, OFFSET>,
            computerInternetMediaSharingAllowedState::<Impl, OFFSET>,
            SetcomputerInternetMediaSharingAllowedState::<Impl, OFFSET>,
            internetMediaSharingSecurityGroup::<Impl, OFFSET>,
            SetinternetMediaSharingSecurityGroup::<Impl, OFFSET>,
            allowSharingToAllDevices::<Impl, OFFSET>,
            SetallowSharingToAllDevices::<Impl, OFFSET>,
            setDefaultAuthorization::<Impl, OFFSET>,
            setAuthorizationState::<Impl, OFFSET>,
            getAllDevices::<Impl, OFFSET>,
            customSettingsApplied::<Impl, OFFSET>,
        )
    }
}
