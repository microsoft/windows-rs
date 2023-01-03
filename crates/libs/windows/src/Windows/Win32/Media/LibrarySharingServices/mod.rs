#[doc = "*Required features: `\"Win32_Media_LibrarySharingServices\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IWindowsMediaLibrarySharingDevice(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWindowsMediaLibrarySharingDevice {
    pub unsafe fn DeviceID(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).DeviceID)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Authorization(&self) -> ::windows::core::Result<WindowsMediaLibrarySharingDeviceAuthorizationStatus> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Authorization)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetAuthorization(&self, authorization: WindowsMediaLibrarySharingDeviceAuthorizationStatus) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetAuthorization)(::windows::core::Vtable::as_raw(self), authorization).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Properties(&self) -> ::windows::core::Result<IWindowsMediaLibrarySharingDeviceProperties> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Properties)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IWindowsMediaLibrarySharingDevice, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IWindowsMediaLibrarySharingDevice {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IWindowsMediaLibrarySharingDevice {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IWindowsMediaLibrarySharingDevice {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IWindowsMediaLibrarySharingDevice {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWindowsMediaLibrarySharingDevice").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IWindowsMediaLibrarySharingDevice {
    type Vtable = IWindowsMediaLibrarySharingDevice_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IWindowsMediaLibrarySharingDevice {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3dccc293_4fd9_4191_a25b_8e57c5d27bd4);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsMediaLibrarySharingDevice_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub DeviceID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Authorization: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, authorization: *mut WindowsMediaLibrarySharingDeviceAuthorizationStatus) -> ::windows::core::HRESULT,
    pub SetAuthorization: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, authorization: WindowsMediaLibrarySharingDeviceAuthorizationStatus) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceproperties: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Properties: usize,
}
#[doc = "*Required features: `\"Win32_Media_LibrarySharingServices\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IWindowsMediaLibrarySharingDeviceProperties(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWindowsMediaLibrarySharingDeviceProperties {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_Item(&self, index: i32) -> ::windows::core::Result<IWindowsMediaLibrarySharingDeviceProperty> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).get_Item)(::windows::core::Vtable::as_raw(self), index, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Count)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetProperty(&self, name: &::windows::core::BSTR) -> ::windows::core::Result<IWindowsMediaLibrarySharingDeviceProperty> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetProperty)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(name), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IWindowsMediaLibrarySharingDeviceProperties, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IWindowsMediaLibrarySharingDeviceProperties {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IWindowsMediaLibrarySharingDeviceProperties {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IWindowsMediaLibrarySharingDeviceProperties {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IWindowsMediaLibrarySharingDeviceProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWindowsMediaLibrarySharingDeviceProperties").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IWindowsMediaLibrarySharingDeviceProperties {
    type Vtable = IWindowsMediaLibrarySharingDeviceProperties_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IWindowsMediaLibrarySharingDeviceProperties {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc4623214_6b06_40c5_a623_b2ff4c076bfd);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsMediaLibrarySharingDeviceProperties_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32, property: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_Item: usize,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut ::core::ffi::c_void, property: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetProperty: usize,
}
#[doc = "*Required features: `\"Win32_Media_LibrarySharingServices\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IWindowsMediaLibrarySharingDeviceProperty(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWindowsMediaLibrarySharingDeviceProperty {
    pub unsafe fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Name)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Value(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Value)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IWindowsMediaLibrarySharingDeviceProperty, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IWindowsMediaLibrarySharingDeviceProperty {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IWindowsMediaLibrarySharingDeviceProperty {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IWindowsMediaLibrarySharingDeviceProperty {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IWindowsMediaLibrarySharingDeviceProperty {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWindowsMediaLibrarySharingDeviceProperty").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IWindowsMediaLibrarySharingDeviceProperty {
    type Vtable = IWindowsMediaLibrarySharingDeviceProperty_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IWindowsMediaLibrarySharingDeviceProperty {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x81e26927_7a7d_40a7_81d4_bddc02960e3e);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsMediaLibrarySharingDeviceProperty_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Value: usize,
}
#[doc = "*Required features: `\"Win32_Media_LibrarySharingServices\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IWindowsMediaLibrarySharingDevices(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWindowsMediaLibrarySharingDevices {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_Item(&self, index: i32) -> ::windows::core::Result<IWindowsMediaLibrarySharingDevice> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).get_Item)(::windows::core::Vtable::as_raw(self), index, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Count)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetDevice(&self, deviceid: &::windows::core::BSTR) -> ::windows::core::Result<IWindowsMediaLibrarySharingDevice> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetDevice)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(deviceid), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IWindowsMediaLibrarySharingDevices, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IWindowsMediaLibrarySharingDevices {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IWindowsMediaLibrarySharingDevices {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IWindowsMediaLibrarySharingDevices {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IWindowsMediaLibrarySharingDevices {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWindowsMediaLibrarySharingDevices").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IWindowsMediaLibrarySharingDevices {
    type Vtable = IWindowsMediaLibrarySharingDevices_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IWindowsMediaLibrarySharingDevices {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1803f9d6_fe6d_4546_bf5b_992fe8ec12d1);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsMediaLibrarySharingDevices_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32, device: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_Item: usize,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: *mut ::core::ffi::c_void, device: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetDevice: usize,
}
#[doc = "*Required features: `\"Win32_Media_LibrarySharingServices\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IWindowsMediaLibrarySharingServices(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWindowsMediaLibrarySharingServices {
    pub unsafe fn showShareMediaCPL(&self, device: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).showShareMediaCPL)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(device)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn userHomeMediaSharingState(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).userHomeMediaSharingState)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetuserHomeMediaSharingState<P0>(&self, sharingenabled: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).SetuserHomeMediaSharingState)(::windows::core::Vtable::as_raw(self), sharingenabled.into()).ok()
    }
    pub unsafe fn userHomeMediaSharingLibraryName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).userHomeMediaSharingLibraryName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetuserHomeMediaSharingLibraryName(&self, libraryname: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetuserHomeMediaSharingLibraryName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(libraryname)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn computerHomeMediaSharingAllowedState(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).computerHomeMediaSharingAllowedState)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetcomputerHomeMediaSharingAllowedState<P0>(&self, sharingallowed: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).SetcomputerHomeMediaSharingAllowedState)(::windows::core::Vtable::as_raw(self), sharingallowed.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn userInternetMediaSharingState(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).userInternetMediaSharingState)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetuserInternetMediaSharingState<P0>(&self, sharingenabled: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).SetuserInternetMediaSharingState)(::windows::core::Vtable::as_raw(self), sharingenabled.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn computerInternetMediaSharingAllowedState(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).computerInternetMediaSharingAllowedState)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetcomputerInternetMediaSharingAllowedState<P0>(&self, sharingallowed: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).SetcomputerInternetMediaSharingAllowedState)(::windows::core::Vtable::as_raw(self), sharingallowed.into()).ok()
    }
    pub unsafe fn internetMediaSharingSecurityGroup(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).internetMediaSharingSecurityGroup)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetinternetMediaSharingSecurityGroup(&self, securitygroup: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetinternetMediaSharingSecurityGroup)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(securitygroup)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn allowSharingToAllDevices(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).allowSharingToAllDevices)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetallowSharingToAllDevices<P0>(&self, sharingenabled: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).SetallowSharingToAllDevices)(::windows::core::Vtable::as_raw(self), sharingenabled.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn setDefaultAuthorization<P0>(&self, macaddresses: &::windows::core::BSTR, friendlyname: &::windows::core::BSTR, authorization: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).setDefaultAuthorization)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(macaddresses), ::core::mem::transmute_copy(friendlyname), authorization.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn setAuthorizationState<P0>(&self, macaddress: &::windows::core::BSTR, authorizationstate: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).setAuthorizationState)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(macaddress), authorizationstate.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn getAllDevices(&self) -> ::windows::core::Result<IWindowsMediaLibrarySharingDevices> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).getAllDevices)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn customSettingsApplied(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).customSettingsApplied)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IWindowsMediaLibrarySharingServices, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IWindowsMediaLibrarySharingServices {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IWindowsMediaLibrarySharingServices {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IWindowsMediaLibrarySharingServices {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IWindowsMediaLibrarySharingServices {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWindowsMediaLibrarySharingServices").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IWindowsMediaLibrarySharingServices {
    type Vtable = IWindowsMediaLibrarySharingServices_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IWindowsMediaLibrarySharingServices {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x01f5f85e_0a81_40da_a7c8_21ef3af8440c);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsMediaLibrarySharingServices_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub showShareMediaCPL: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, device: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub userHomeMediaSharingState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sharingenabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    userHomeMediaSharingState: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetuserHomeMediaSharingState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sharingenabled: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetuserHomeMediaSharingState: usize,
    pub userHomeMediaSharingLibraryName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, libraryname: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetuserHomeMediaSharingLibraryName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, libraryname: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub computerHomeMediaSharingAllowedState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sharingallowed: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    computerHomeMediaSharingAllowedState: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetcomputerHomeMediaSharingAllowedState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sharingallowed: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetcomputerHomeMediaSharingAllowedState: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub userInternetMediaSharingState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sharingenabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    userInternetMediaSharingState: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetuserInternetMediaSharingState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sharingenabled: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetuserInternetMediaSharingState: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub computerInternetMediaSharingAllowedState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sharingallowed: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    computerInternetMediaSharingAllowedState: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetcomputerInternetMediaSharingAllowedState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sharingallowed: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetcomputerInternetMediaSharingAllowedState: usize,
    pub internetMediaSharingSecurityGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, securitygroup: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetinternetMediaSharingSecurityGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, securitygroup: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub allowSharingToAllDevices: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sharingenabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    allowSharingToAllDevices: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetallowSharingToAllDevices: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sharingenabled: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetallowSharingToAllDevices: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub setDefaultAuthorization: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, macaddresses: *mut ::core::ffi::c_void, friendlyname: *mut ::core::ffi::c_void, authorization: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    setDefaultAuthorization: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub setAuthorizationState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, macaddress: *mut ::core::ffi::c_void, authorizationstate: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    setAuthorizationState: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub getAllDevices: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, devices: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    getAllDevices: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub customSettingsApplied: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, customsettingsapplied: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    customSettingsApplied: usize,
}
#[doc = "*Required features: `\"Win32_Media_LibrarySharingServices\"`*"]
pub const WindowsMediaLibrarySharingServices: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xad581b00_7b64_4e59_a38d_d2c5bf51ddb3);
#[doc = "*Required features: `\"Win32_Media_LibrarySharingServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WindowsMediaLibrarySharingDeviceAuthorizationStatus(pub i32);
#[doc = "*Required features: `\"Win32_Media_LibrarySharingServices\"`*"]
pub const DEVICE_AUTHORIZATION_UNKNOWN: WindowsMediaLibrarySharingDeviceAuthorizationStatus = WindowsMediaLibrarySharingDeviceAuthorizationStatus(0i32);
#[doc = "*Required features: `\"Win32_Media_LibrarySharingServices\"`*"]
pub const DEVICE_AUTHORIZATION_ALLOWED: WindowsMediaLibrarySharingDeviceAuthorizationStatus = WindowsMediaLibrarySharingDeviceAuthorizationStatus(1i32);
#[doc = "*Required features: `\"Win32_Media_LibrarySharingServices\"`*"]
pub const DEVICE_AUTHORIZATION_DENIED: WindowsMediaLibrarySharingDeviceAuthorizationStatus = WindowsMediaLibrarySharingDeviceAuthorizationStatus(2i32);
impl ::core::marker::Copy for WindowsMediaLibrarySharingDeviceAuthorizationStatus {}
impl ::core::clone::Clone for WindowsMediaLibrarySharingDeviceAuthorizationStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WindowsMediaLibrarySharingDeviceAuthorizationStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WindowsMediaLibrarySharingDeviceAuthorizationStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for WindowsMediaLibrarySharingDeviceAuthorizationStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WindowsMediaLibrarySharingDeviceAuthorizationStatus").field(&self.0).finish()
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
