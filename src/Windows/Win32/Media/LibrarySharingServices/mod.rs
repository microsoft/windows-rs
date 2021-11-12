#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IWindowsMediaLibrarySharingDevice(pub ::windows::core::IUnknown);
impl IWindowsMediaLibrarySharingDevice {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DeviceID(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn Authorization(&self) -> ::windows::core::Result<WindowsMediaLibrarySharingDeviceAuthorizationStatus> {
        let mut result__: <WindowsMediaLibrarySharingDeviceAuthorizationStatus as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<WindowsMediaLibrarySharingDeviceAuthorizationStatus>(result__)
    }
    pub unsafe fn SetAuthorization(&self, authorization: WindowsMediaLibrarySharingDeviceAuthorizationStatus) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(authorization)).ok()
    }
    pub unsafe fn Properties(&self) -> ::windows::core::Result<IWindowsMediaLibrarySharingDeviceProperties> {
        let mut result__: <IWindowsMediaLibrarySharingDeviceProperties as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IWindowsMediaLibrarySharingDeviceProperties>(result__)
    }
}
unsafe impl ::windows::core::Interface for IWindowsMediaLibrarySharingDevice {
    type Vtable = IWindowsMediaLibrarySharingDevice_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3dccc293_4fd9_4191_a25b_8e57c5d27bd4);
}
impl ::core::convert::From<IWindowsMediaLibrarySharingDevice> for ::windows::core::IUnknown {
    fn from(value: IWindowsMediaLibrarySharingDevice) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IWindowsMediaLibrarySharingDevice> for ::windows::core::IUnknown {
    fn from(value: &IWindowsMediaLibrarySharingDevice) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWindowsMediaLibrarySharingDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWindowsMediaLibrarySharingDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
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
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &IWindowsMediaLibrarySharingDevice {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsMediaLibrarySharingDevice_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub  unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::super::System::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, deviceid: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, authorization: *mut WindowsMediaLibrarySharingDeviceAuthorizationStatus) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, authorization: WindowsMediaLibrarySharingDeviceAuthorizationStatus) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, deviceproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IWindowsMediaLibrarySharingDeviceProperties(pub ::windows::core::IUnknown);
impl IWindowsMediaLibrarySharingDeviceProperties {
    pub unsafe fn Item(&self, index: i32) -> ::windows::core::Result<IWindowsMediaLibrarySharingDeviceProperty> {
        let mut result__: <IWindowsMediaLibrarySharingDeviceProperty as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), &mut result__).from_abi::<IWindowsMediaLibrarySharingDeviceProperty>(result__)
    }
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetProperty<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::core::Result<IWindowsMediaLibrarySharingDeviceProperty> {
        let mut result__: <IWindowsMediaLibrarySharingDeviceProperty as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), name.into_param().abi(), &mut result__).from_abi::<IWindowsMediaLibrarySharingDeviceProperty>(result__)
    }
}
unsafe impl ::windows::core::Interface for IWindowsMediaLibrarySharingDeviceProperties {
    type Vtable = IWindowsMediaLibrarySharingDeviceProperties_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc4623214_6b06_40c5_a623_b2ff4c076bfd);
}
impl ::core::convert::From<IWindowsMediaLibrarySharingDeviceProperties> for ::windows::core::IUnknown {
    fn from(value: IWindowsMediaLibrarySharingDeviceProperties) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IWindowsMediaLibrarySharingDeviceProperties> for ::windows::core::IUnknown {
    fn from(value: &IWindowsMediaLibrarySharingDeviceProperties) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWindowsMediaLibrarySharingDeviceProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWindowsMediaLibrarySharingDeviceProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
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
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &IWindowsMediaLibrarySharingDeviceProperties {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsMediaLibrarySharingDeviceProperties_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub  unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::super::System::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, index: i32, property: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, property: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IWindowsMediaLibrarySharingDeviceProperty(pub ::windows::core::IUnknown);
impl IWindowsMediaLibrarySharingDeviceProperty {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Value(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__: <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
}
unsafe impl ::windows::core::Interface for IWindowsMediaLibrarySharingDeviceProperty {
    type Vtable = IWindowsMediaLibrarySharingDeviceProperty_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x81e26927_7a7d_40a7_81d4_bddc02960e3e);
}
impl ::core::convert::From<IWindowsMediaLibrarySharingDeviceProperty> for ::windows::core::IUnknown {
    fn from(value: IWindowsMediaLibrarySharingDeviceProperty) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IWindowsMediaLibrarySharingDeviceProperty> for ::windows::core::IUnknown {
    fn from(value: &IWindowsMediaLibrarySharingDeviceProperty) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWindowsMediaLibrarySharingDeviceProperty {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWindowsMediaLibrarySharingDeviceProperty {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
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
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &IWindowsMediaLibrarySharingDeviceProperty {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsMediaLibrarySharingDeviceProperty_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub  unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::super::System::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IWindowsMediaLibrarySharingDevices(pub ::windows::core::IUnknown);
impl IWindowsMediaLibrarySharingDevices {
    pub unsafe fn Item(&self, index: i32) -> ::windows::core::Result<IWindowsMediaLibrarySharingDevice> {
        let mut result__: <IWindowsMediaLibrarySharingDevice as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), &mut result__).from_abi::<IWindowsMediaLibrarySharingDevice>(result__)
    }
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDevice<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, deviceid: Param0) -> ::windows::core::Result<IWindowsMediaLibrarySharingDevice> {
        let mut result__: <IWindowsMediaLibrarySharingDevice as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), deviceid.into_param().abi(), &mut result__).from_abi::<IWindowsMediaLibrarySharingDevice>(result__)
    }
}
unsafe impl ::windows::core::Interface for IWindowsMediaLibrarySharingDevices {
    type Vtable = IWindowsMediaLibrarySharingDevices_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1803f9d6_fe6d_4546_bf5b_992fe8ec12d1);
}
impl ::core::convert::From<IWindowsMediaLibrarySharingDevices> for ::windows::core::IUnknown {
    fn from(value: IWindowsMediaLibrarySharingDevices) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IWindowsMediaLibrarySharingDevices> for ::windows::core::IUnknown {
    fn from(value: &IWindowsMediaLibrarySharingDevices) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWindowsMediaLibrarySharingDevices {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWindowsMediaLibrarySharingDevices {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
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
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &IWindowsMediaLibrarySharingDevices {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsMediaLibrarySharingDevices_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub  unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::super::System::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, index: i32, device: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, deviceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, device: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IWindowsMediaLibrarySharingServices(pub ::windows::core::IUnknown);
impl IWindowsMediaLibrarySharingServices {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn showShareMediaCPL<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, device: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), device.into_param().abi()).ok()
    }
    pub unsafe fn userHomeMediaSharingState(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetuserHomeMediaSharingState(&self, sharingenabled: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(sharingenabled)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn userHomeMediaSharingLibraryName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetuserHomeMediaSharingLibraryName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, libraryname: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), libraryname.into_param().abi()).ok()
    }
    pub unsafe fn computerHomeMediaSharingAllowedState(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetcomputerHomeMediaSharingAllowedState(&self, sharingallowed: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(sharingallowed)).ok()
    }
    pub unsafe fn userInternetMediaSharingState(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetuserInternetMediaSharingState(&self, sharingenabled: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(sharingenabled)).ok()
    }
    pub unsafe fn computerInternetMediaSharingAllowedState(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetcomputerInternetMediaSharingAllowedState(&self, sharingallowed: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(sharingallowed)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn internetMediaSharingSecurityGroup(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetinternetMediaSharingSecurityGroup<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, securitygroup: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), securitygroup.into_param().abi()).ok()
    }
    pub unsafe fn allowSharingToAllDevices(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetallowSharingToAllDevices(&self, sharingenabled: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(sharingenabled)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn setDefaultAuthorization<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, macaddresses: Param0, friendlyname: Param1, authorization: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), macaddresses.into_param().abi(), friendlyname.into_param().abi(), ::core::mem::transmute(authorization)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn setAuthorizationState<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, macaddress: Param0, authorizationstate: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), macaddress.into_param().abi(), ::core::mem::transmute(authorizationstate)).ok()
    }
    pub unsafe fn getAllDevices(&self) -> ::windows::core::Result<IWindowsMediaLibrarySharingDevices> {
        let mut result__: <IWindowsMediaLibrarySharingDevices as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IWindowsMediaLibrarySharingDevices>(result__)
    }
    pub unsafe fn customSettingsApplied(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
}
unsafe impl ::windows::core::Interface for IWindowsMediaLibrarySharingServices {
    type Vtable = IWindowsMediaLibrarySharingServices_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x01f5f85e_0a81_40da_a7c8_21ef3af8440c);
}
impl ::core::convert::From<IWindowsMediaLibrarySharingServices> for ::windows::core::IUnknown {
    fn from(value: IWindowsMediaLibrarySharingServices) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IWindowsMediaLibrarySharingServices> for ::windows::core::IUnknown {
    fn from(value: &IWindowsMediaLibrarySharingServices) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWindowsMediaLibrarySharingServices {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWindowsMediaLibrarySharingServices {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
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
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &IWindowsMediaLibrarySharingServices {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsMediaLibrarySharingServices_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub  unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::super::System::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, device: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, sharingenabled: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, sharingenabled: i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, libraryname: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, libraryname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, sharingallowed: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, sharingallowed: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, sharingenabled: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, sharingenabled: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, sharingallowed: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, sharingallowed: i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, securitygroup: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, securitygroup: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, sharingenabled: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, sharingenabled: i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, macaddresses: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, friendlyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, authorization: i16) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, macaddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, authorizationstate: i16) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, devices: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, customsettingsapplied: *mut i16) -> ::windows::core::HRESULT,
);
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct WindowsMediaLibrarySharingDeviceAuthorizationStatus(pub i32);
pub const DEVICE_AUTHORIZATION_UNKNOWN: WindowsMediaLibrarySharingDeviceAuthorizationStatus = WindowsMediaLibrarySharingDeviceAuthorizationStatus(0i32);
pub const DEVICE_AUTHORIZATION_ALLOWED: WindowsMediaLibrarySharingDeviceAuthorizationStatus = WindowsMediaLibrarySharingDeviceAuthorizationStatus(1i32);
pub const DEVICE_AUTHORIZATION_DENIED: WindowsMediaLibrarySharingDeviceAuthorizationStatus = WindowsMediaLibrarySharingDeviceAuthorizationStatus(2i32);
impl ::core::convert::From<i32> for WindowsMediaLibrarySharingDeviceAuthorizationStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for WindowsMediaLibrarySharingDeviceAuthorizationStatus {
    type Abi = Self;
}
pub const WindowsMediaLibrarySharingServices: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xad581b00_7b64_4e59_a38d_d2c5bf51ddb3);
