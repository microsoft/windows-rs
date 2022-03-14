#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: `\"Win32_Media_LibrarySharingServices\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IWindowsMediaLibrarySharingDevice(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWindowsMediaLibrarySharingDevice {
    #[doc = "*Required features: `\"Win32_Media_LibrarySharingServices\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DeviceID(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).DeviceID)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_LibrarySharingServices\"`*"]
    pub unsafe fn Authorization(&self) -> ::windows::core::Result<WindowsMediaLibrarySharingDeviceAuthorizationStatus> {
        let mut result__: WindowsMediaLibrarySharingDeviceAuthorizationStatus = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Authorization)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<WindowsMediaLibrarySharingDeviceAuthorizationStatus>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_LibrarySharingServices\"`*"]
    pub unsafe fn SetAuthorization(&self, authorization: WindowsMediaLibrarySharingDeviceAuthorizationStatus) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAuthorization)(::core::mem::transmute_copy(self), ::core::mem::transmute(authorization)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_LibrarySharingServices\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Properties(&self) -> ::windows::core::Result<IWindowsMediaLibrarySharingDeviceProperties> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Properties)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IWindowsMediaLibrarySharingDeviceProperties>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IWindowsMediaLibrarySharingDevice> for ::windows::core::IUnknown {
    fn from(value: IWindowsMediaLibrarySharingDevice) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IWindowsMediaLibrarySharingDevice> for ::windows::core::IUnknown {
    fn from(value: &IWindowsMediaLibrarySharingDevice) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWindowsMediaLibrarySharingDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWindowsMediaLibrarySharingDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IWindowsMediaLibrarySharingDevice> for super::super::System::Com::IDispatch {
    fn from(value: IWindowsMediaLibrarySharingDevice) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IWindowsMediaLibrarySharingDevice> for super::super::System::Com::IDispatch {
    fn from(value: &IWindowsMediaLibrarySharingDevice) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for IWindowsMediaLibrarySharingDevice {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &'a IWindowsMediaLibrarySharingDevice {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
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
unsafe impl ::windows::core::Interface for IWindowsMediaLibrarySharingDevice {
    type Vtable = IWindowsMediaLibrarySharingDevice_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3dccc293_4fd9_4191_a25b_8e57c5d27bd4);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsMediaLibrarySharingDevice_Vtbl {
    pub base: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub DeviceID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DeviceID: usize,
    pub Authorization: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, authorization: *mut WindowsMediaLibrarySharingDeviceAuthorizationStatus) -> ::windows::core::HRESULT,
    pub SetAuthorization: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, authorization: WindowsMediaLibrarySharingDeviceAuthorizationStatus) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Properties: usize,
}
#[doc = "*Required features: `\"Win32_Media_LibrarySharingServices\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IWindowsMediaLibrarySharingDeviceProperties(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWindowsMediaLibrarySharingDeviceProperties {
    #[doc = "*Required features: `\"Win32_Media_LibrarySharingServices\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Item(&self, index: i32) -> ::windows::core::Result<IWindowsMediaLibrarySharingDeviceProperty> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Item)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(&mut result__)).from_abi::<IWindowsMediaLibrarySharingDeviceProperty>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_LibrarySharingServices\"`*"]
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Count)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_LibrarySharingServices\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetProperty<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::core::Result<IWindowsMediaLibrarySharingDeviceProperty> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetProperty)(::core::mem::transmute_copy(self), name.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IWindowsMediaLibrarySharingDeviceProperty>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IWindowsMediaLibrarySharingDeviceProperties> for ::windows::core::IUnknown {
    fn from(value: IWindowsMediaLibrarySharingDeviceProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IWindowsMediaLibrarySharingDeviceProperties> for ::windows::core::IUnknown {
    fn from(value: &IWindowsMediaLibrarySharingDeviceProperties) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWindowsMediaLibrarySharingDeviceProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWindowsMediaLibrarySharingDeviceProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IWindowsMediaLibrarySharingDeviceProperties> for super::super::System::Com::IDispatch {
    fn from(value: IWindowsMediaLibrarySharingDeviceProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IWindowsMediaLibrarySharingDeviceProperties> for super::super::System::Com::IDispatch {
    fn from(value: &IWindowsMediaLibrarySharingDeviceProperties) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for IWindowsMediaLibrarySharingDeviceProperties {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &'a IWindowsMediaLibrarySharingDeviceProperties {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
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
unsafe impl ::windows::core::Interface for IWindowsMediaLibrarySharingDeviceProperties {
    type Vtable = IWindowsMediaLibrarySharingDeviceProperties_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc4623214_6b06_40c5_a623_b2ff4c076bfd);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsMediaLibrarySharingDeviceProperties_Vtbl {
    pub base: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32, property: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Item: usize,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, property: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GetProperty: usize,
}
#[doc = "*Required features: `\"Win32_Media_LibrarySharingServices\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IWindowsMediaLibrarySharingDeviceProperty(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWindowsMediaLibrarySharingDeviceProperty {
    #[doc = "*Required features: `\"Win32_Media_LibrarySharingServices\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Name)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_LibrarySharingServices\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Value(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Value)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IWindowsMediaLibrarySharingDeviceProperty> for ::windows::core::IUnknown {
    fn from(value: IWindowsMediaLibrarySharingDeviceProperty) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IWindowsMediaLibrarySharingDeviceProperty> for ::windows::core::IUnknown {
    fn from(value: &IWindowsMediaLibrarySharingDeviceProperty) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWindowsMediaLibrarySharingDeviceProperty {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWindowsMediaLibrarySharingDeviceProperty {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IWindowsMediaLibrarySharingDeviceProperty> for super::super::System::Com::IDispatch {
    fn from(value: IWindowsMediaLibrarySharingDeviceProperty) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IWindowsMediaLibrarySharingDeviceProperty> for super::super::System::Com::IDispatch {
    fn from(value: &IWindowsMediaLibrarySharingDeviceProperty) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for IWindowsMediaLibrarySharingDeviceProperty {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &'a IWindowsMediaLibrarySharingDeviceProperty {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
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
unsafe impl ::windows::core::Interface for IWindowsMediaLibrarySharingDeviceProperty {
    type Vtable = IWindowsMediaLibrarySharingDeviceProperty_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x81e26927_7a7d_40a7_81d4_bddc02960e3e);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsMediaLibrarySharingDeviceProperty_Vtbl {
    pub base: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Name: usize,
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
    #[doc = "*Required features: `\"Win32_Media_LibrarySharingServices\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Item(&self, index: i32) -> ::windows::core::Result<IWindowsMediaLibrarySharingDevice> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Item)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(&mut result__)).from_abi::<IWindowsMediaLibrarySharingDevice>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_LibrarySharingServices\"`*"]
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Count)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_LibrarySharingServices\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetDevice<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, deviceid: Param0) -> ::windows::core::Result<IWindowsMediaLibrarySharingDevice> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetDevice)(::core::mem::transmute_copy(self), deviceid.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IWindowsMediaLibrarySharingDevice>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IWindowsMediaLibrarySharingDevices> for ::windows::core::IUnknown {
    fn from(value: IWindowsMediaLibrarySharingDevices) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IWindowsMediaLibrarySharingDevices> for ::windows::core::IUnknown {
    fn from(value: &IWindowsMediaLibrarySharingDevices) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWindowsMediaLibrarySharingDevices {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWindowsMediaLibrarySharingDevices {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IWindowsMediaLibrarySharingDevices> for super::super::System::Com::IDispatch {
    fn from(value: IWindowsMediaLibrarySharingDevices) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IWindowsMediaLibrarySharingDevices> for super::super::System::Com::IDispatch {
    fn from(value: &IWindowsMediaLibrarySharingDevices) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for IWindowsMediaLibrarySharingDevices {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &'a IWindowsMediaLibrarySharingDevices {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
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
unsafe impl ::windows::core::Interface for IWindowsMediaLibrarySharingDevices {
    type Vtable = IWindowsMediaLibrarySharingDevices_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1803f9d6_fe6d_4546_bf5b_992fe8ec12d1);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsMediaLibrarySharingDevices_Vtbl {
    pub base: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32, device: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Item: usize,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GetDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, device: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GetDevice: usize,
}
#[doc = "*Required features: `\"Win32_Media_LibrarySharingServices\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IWindowsMediaLibrarySharingServices(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWindowsMediaLibrarySharingServices {
    #[doc = "*Required features: `\"Win32_Media_LibrarySharingServices\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn showShareMediaCPL<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, device: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).showShareMediaCPL)(::core::mem::transmute_copy(self), device.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_LibrarySharingServices\"`*"]
    pub unsafe fn userHomeMediaSharingState(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).userHomeMediaSharingState)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_LibrarySharingServices\"`*"]
    pub unsafe fn SetuserHomeMediaSharingState(&self, sharingenabled: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetuserHomeMediaSharingState)(::core::mem::transmute_copy(self), ::core::mem::transmute(sharingenabled)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_LibrarySharingServices\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn userHomeMediaSharingLibraryName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).userHomeMediaSharingLibraryName)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_LibrarySharingServices\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetuserHomeMediaSharingLibraryName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, libraryname: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetuserHomeMediaSharingLibraryName)(::core::mem::transmute_copy(self), libraryname.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_LibrarySharingServices\"`*"]
    pub unsafe fn computerHomeMediaSharingAllowedState(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).computerHomeMediaSharingAllowedState)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_LibrarySharingServices\"`*"]
    pub unsafe fn SetcomputerHomeMediaSharingAllowedState(&self, sharingallowed: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetcomputerHomeMediaSharingAllowedState)(::core::mem::transmute_copy(self), ::core::mem::transmute(sharingallowed)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_LibrarySharingServices\"`*"]
    pub unsafe fn userInternetMediaSharingState(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).userInternetMediaSharingState)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_LibrarySharingServices\"`*"]
    pub unsafe fn SetuserInternetMediaSharingState(&self, sharingenabled: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetuserInternetMediaSharingState)(::core::mem::transmute_copy(self), ::core::mem::transmute(sharingenabled)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_LibrarySharingServices\"`*"]
    pub unsafe fn computerInternetMediaSharingAllowedState(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).computerInternetMediaSharingAllowedState)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_LibrarySharingServices\"`*"]
    pub unsafe fn SetcomputerInternetMediaSharingAllowedState(&self, sharingallowed: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetcomputerInternetMediaSharingAllowedState)(::core::mem::transmute_copy(self), ::core::mem::transmute(sharingallowed)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_LibrarySharingServices\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn internetMediaSharingSecurityGroup(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).internetMediaSharingSecurityGroup)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_LibrarySharingServices\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetinternetMediaSharingSecurityGroup<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, securitygroup: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetinternetMediaSharingSecurityGroup)(::core::mem::transmute_copy(self), securitygroup.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_LibrarySharingServices\"`*"]
    pub unsafe fn allowSharingToAllDevices(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).allowSharingToAllDevices)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_LibrarySharingServices\"`*"]
    pub unsafe fn SetallowSharingToAllDevices(&self, sharingenabled: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetallowSharingToAllDevices)(::core::mem::transmute_copy(self), ::core::mem::transmute(sharingenabled)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_LibrarySharingServices\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn setDefaultAuthorization<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, macaddresses: Param0, friendlyname: Param1, authorization: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).setDefaultAuthorization)(::core::mem::transmute_copy(self), macaddresses.into_param().abi(), friendlyname.into_param().abi(), ::core::mem::transmute(authorization)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_LibrarySharingServices\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn setAuthorizationState<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, macaddress: Param0, authorizationstate: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).setAuthorizationState)(::core::mem::transmute_copy(self), macaddress.into_param().abi(), ::core::mem::transmute(authorizationstate)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_LibrarySharingServices\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn getAllDevices(&self) -> ::windows::core::Result<IWindowsMediaLibrarySharingDevices> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).getAllDevices)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IWindowsMediaLibrarySharingDevices>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_LibrarySharingServices\"`*"]
    pub unsafe fn customSettingsApplied(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).customSettingsApplied)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IWindowsMediaLibrarySharingServices> for ::windows::core::IUnknown {
    fn from(value: IWindowsMediaLibrarySharingServices) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IWindowsMediaLibrarySharingServices> for ::windows::core::IUnknown {
    fn from(value: &IWindowsMediaLibrarySharingServices) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWindowsMediaLibrarySharingServices {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWindowsMediaLibrarySharingServices {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IWindowsMediaLibrarySharingServices> for super::super::System::Com::IDispatch {
    fn from(value: IWindowsMediaLibrarySharingServices) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IWindowsMediaLibrarySharingServices> for super::super::System::Com::IDispatch {
    fn from(value: &IWindowsMediaLibrarySharingServices) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for IWindowsMediaLibrarySharingServices {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &'a IWindowsMediaLibrarySharingServices {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
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
unsafe impl ::windows::core::Interface for IWindowsMediaLibrarySharingServices {
    type Vtable = IWindowsMediaLibrarySharingServices_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x01f5f85e_0a81_40da_a7c8_21ef3af8440c);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsMediaLibrarySharingServices_Vtbl {
    pub base: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub showShareMediaCPL: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, device: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    showShareMediaCPL: usize,
    pub userHomeMediaSharingState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sharingenabled: *mut i16) -> ::windows::core::HRESULT,
    pub SetuserHomeMediaSharingState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sharingenabled: i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub userHomeMediaSharingLibraryName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, libraryname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    userHomeMediaSharingLibraryName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetuserHomeMediaSharingLibraryName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, libraryname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetuserHomeMediaSharingLibraryName: usize,
    pub computerHomeMediaSharingAllowedState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sharingallowed: *mut i16) -> ::windows::core::HRESULT,
    pub SetcomputerHomeMediaSharingAllowedState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sharingallowed: i16) -> ::windows::core::HRESULT,
    pub userInternetMediaSharingState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sharingenabled: *mut i16) -> ::windows::core::HRESULT,
    pub SetuserInternetMediaSharingState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sharingenabled: i16) -> ::windows::core::HRESULT,
    pub computerInternetMediaSharingAllowedState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sharingallowed: *mut i16) -> ::windows::core::HRESULT,
    pub SetcomputerInternetMediaSharingAllowedState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sharingallowed: i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub internetMediaSharingSecurityGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, securitygroup: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    internetMediaSharingSecurityGroup: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetinternetMediaSharingSecurityGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, securitygroup: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetinternetMediaSharingSecurityGroup: usize,
    pub allowSharingToAllDevices: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sharingenabled: *mut i16) -> ::windows::core::HRESULT,
    pub SetallowSharingToAllDevices: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sharingenabled: i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub setDefaultAuthorization: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, macaddresses: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, friendlyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, authorization: i16) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    setDefaultAuthorization: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub setAuthorizationState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, macaddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, authorizationstate: i16) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    setAuthorizationState: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub getAllDevices: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, devices: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    getAllDevices: usize,
    pub customSettingsApplied: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, customsettingsapplied: *mut i16) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_LibrarySharingServices\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
pub const WindowsMediaLibrarySharingServices: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xad581b00_7b64_4e59_a38d_d2c5bf51ddb3);
#[cfg(feature = "implement")]
::core::include!("impl.rs");
