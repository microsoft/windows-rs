#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FOREIGN_STATUS(pub i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_STATUS_FOREIGN: FOREIGN_STATUS = FOREIGN_STATUS(0i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_STATUS_NOT_FOREIGN: FOREIGN_STATUS = FOREIGN_STATUS(1i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_STATUS_UNKNOWN: FOREIGN_STATUS = FOREIGN_STATUS(2i32);
impl ::core::marker::Copy for FOREIGN_STATUS {}
impl ::core::clone::Clone for FOREIGN_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FOREIGN_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FOREIGN_STATUS {
    type Abi = Self;
}
impl ::core::fmt::Debug for FOREIGN_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FOREIGN_STATUS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IMSMQApplication(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IMSMQApplication {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MachineIdOfMachineName<'a, P0>(&self, machinename: P0) -> ::windows::core::Result<super::super::Foundation::BSTR>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).MachineIdOfMachineName)(::windows::core::Interface::as_raw(self), machinename.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMSMQApplication> for ::windows::core::IUnknown {
    fn from(value: IMSMQApplication) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IMSMQApplication> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IMSMQApplication) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMSMQApplication> for ::windows::core::IUnknown {
    fn from(value: &IMSMQApplication) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMSMQApplication> for super::Com::IDispatch {
    fn from(value: IMSMQApplication) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IMSMQApplication> for &'a super::Com::IDispatch {
    fn from(value: &'a IMSMQApplication) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMSMQApplication> for super::Com::IDispatch {
    fn from(value: &IMSMQApplication) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IMSMQApplication {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IMSMQApplication {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IMSMQApplication {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IMSMQApplication {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMSMQApplication").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IMSMQApplication {
    type Vtable = IMSMQApplication_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd7d6e085_dccd_11d0_aa4b_0060970debae);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQApplication_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub MachineIdOfMachineName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, machinename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrguid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MachineIdOfMachineName: usize,
}
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IMSMQApplication2(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IMSMQApplication2 {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MachineIdOfMachineName<'a, P0>(&self, machinename: P0) -> ::windows::core::Result<super::super::Foundation::BSTR>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.MachineIdOfMachineName)(::windows::core::Interface::as_raw(self), machinename.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn RegisterCertificate(&self, flags: *const super::Com::VARIANT, externalcertificate: *const super::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RegisterCertificate)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(flags), ::core::mem::transmute(externalcertificate)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MachineNameOfMachineId<'a, P0>(&self, bstrguid: P0) -> ::windows::core::Result<super::super::Foundation::BSTR>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).MachineNameOfMachineId)(::windows::core::Interface::as_raw(self), bstrguid.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn MSMQVersionMajor(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).MSMQVersionMajor)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn MSMQVersionMinor(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).MSMQVersionMinor)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn MSMQVersionBuild(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).MSMQVersionBuild)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn IsDsEnabled(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).IsDsEnabled)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Properties(&self) -> ::windows::core::Result<super::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Properties)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::IDispatch>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMSMQApplication2> for ::windows::core::IUnknown {
    fn from(value: IMSMQApplication2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IMSMQApplication2> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IMSMQApplication2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMSMQApplication2> for ::windows::core::IUnknown {
    fn from(value: &IMSMQApplication2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMSMQApplication2> for super::Com::IDispatch {
    fn from(value: IMSMQApplication2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IMSMQApplication2> for &'a super::Com::IDispatch {
    fn from(value: &'a IMSMQApplication2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMSMQApplication2> for super::Com::IDispatch {
    fn from(value: &IMSMQApplication2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMSMQApplication2> for IMSMQApplication {
    fn from(value: IMSMQApplication2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IMSMQApplication2> for &'a IMSMQApplication {
    fn from(value: &'a IMSMQApplication2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMSMQApplication2> for IMSMQApplication {
    fn from(value: &IMSMQApplication2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IMSMQApplication2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IMSMQApplication2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IMSMQApplication2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IMSMQApplication2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMSMQApplication2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IMSMQApplication2 {
    type Vtable = IMSMQApplication2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x12a30900_7300_11d2_b0e6_00e02c074f6b);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQApplication2_Vtbl {
    pub base__: IMSMQApplication_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub RegisterCertificate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flags: *const super::Com::VARIANT, externalcertificate: *const super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    RegisterCertificate: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub MachineNameOfMachineId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrguid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrmachinename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MachineNameOfMachineId: usize,
    pub MSMQVersionMajor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psmsmqversionmajor: *mut i16) -> ::windows::core::HRESULT,
    pub MSMQVersionMinor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psmsmqversionminor: *mut i16) -> ::windows::core::HRESULT,
    pub MSMQVersionBuild: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psmsmqversionbuild: *mut i16) -> ::windows::core::HRESULT,
    pub IsDsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfisdsenabled: *mut i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcolproperties: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Properties: usize,
}
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IMSMQApplication3(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IMSMQApplication3 {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MachineIdOfMachineName<'a, P0>(&self, machinename: P0) -> ::windows::core::Result<super::super::Foundation::BSTR>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.MachineIdOfMachineName)(::windows::core::Interface::as_raw(self), machinename.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn RegisterCertificate(&self, flags: *const super::Com::VARIANT, externalcertificate: *const super::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.RegisterCertificate)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(flags), ::core::mem::transmute(externalcertificate)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MachineNameOfMachineId<'a, P0>(&self, bstrguid: P0) -> ::windows::core::Result<super::super::Foundation::BSTR>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.MachineNameOfMachineId)(::windows::core::Interface::as_raw(self), bstrguid.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn MSMQVersionMajor(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.MSMQVersionMajor)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn MSMQVersionMinor(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.MSMQVersionMinor)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn MSMQVersionBuild(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.MSMQVersionBuild)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn IsDsEnabled(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.IsDsEnabled)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Properties(&self) -> ::windows::core::Result<super::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.Properties)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::IDispatch>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ActiveQueues(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).ActiveQueues)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PrivateQueues(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).PrivateQueues)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DirectoryServiceServer(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).DirectoryServiceServer)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn IsConnected(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).IsConnected)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn BytesInAllQueues(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).BytesInAllQueues)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetMachine<'a, P0>(&self, bstrmachine: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).SetMachine)(::windows::core::Interface::as_raw(self), bstrmachine.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Machine(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Machine)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn Connect(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Connect)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Disconnect(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Disconnect)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Tidy(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Tidy)(::windows::core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMSMQApplication3> for ::windows::core::IUnknown {
    fn from(value: IMSMQApplication3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IMSMQApplication3> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IMSMQApplication3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMSMQApplication3> for ::windows::core::IUnknown {
    fn from(value: &IMSMQApplication3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMSMQApplication3> for super::Com::IDispatch {
    fn from(value: IMSMQApplication3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IMSMQApplication3> for &'a super::Com::IDispatch {
    fn from(value: &'a IMSMQApplication3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMSMQApplication3> for super::Com::IDispatch {
    fn from(value: &IMSMQApplication3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMSMQApplication3> for IMSMQApplication {
    fn from(value: IMSMQApplication3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IMSMQApplication3> for &'a IMSMQApplication {
    fn from(value: &'a IMSMQApplication3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMSMQApplication3> for IMSMQApplication {
    fn from(value: &IMSMQApplication3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMSMQApplication3> for IMSMQApplication2 {
    fn from(value: IMSMQApplication3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IMSMQApplication3> for &'a IMSMQApplication2 {
    fn from(value: &'a IMSMQApplication3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMSMQApplication3> for IMSMQApplication2 {
    fn from(value: &IMSMQApplication3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IMSMQApplication3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IMSMQApplication3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IMSMQApplication3 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IMSMQApplication3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMSMQApplication3").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IMSMQApplication3 {
    type Vtable = IMSMQApplication3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeba96b1f_2168_11d3_898c_00e02c074f6b);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQApplication3_Vtbl {
    pub base__: IMSMQApplication2_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub ActiveQueues: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvactivequeues: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    ActiveQueues: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub PrivateQueues: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvprivatequeues: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    PrivateQueues: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DirectoryServiceServer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrdirectoryserviceserver: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DirectoryServiceServer: usize,
    pub IsConnected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfisconnected: *mut i16) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub BytesInAllQueues: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvbytesinallqueues: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    BytesInAllQueues: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetMachine: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrmachine: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetMachine: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Machine: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrmachine: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Machine: usize,
    pub Connect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Disconnect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Tidy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IMSMQCollection(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IMSMQCollection {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Item(&self, index: *const super::Com::VARIANT) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Item)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(index), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Count)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self)._NewEnum)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::IUnknown>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMSMQCollection> for ::windows::core::IUnknown {
    fn from(value: IMSMQCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IMSMQCollection> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IMSMQCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMSMQCollection> for ::windows::core::IUnknown {
    fn from(value: &IMSMQCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMSMQCollection> for super::Com::IDispatch {
    fn from(value: IMSMQCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IMSMQCollection> for &'a super::Com::IDispatch {
    fn from(value: &'a IMSMQCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMSMQCollection> for super::Com::IDispatch {
    fn from(value: &IMSMQCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IMSMQCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IMSMQCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IMSMQCollection {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IMSMQCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMSMQCollection").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IMSMQCollection {
    type Vtable = IMSMQCollection_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0188ac2f_ecb3_4173_9779_635ca2039c72);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQCollection_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: *const super::Com::VARIANT, pvarret: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Item: usize,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows::core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IMSMQCoordinatedTransactionDispenser(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IMSMQCoordinatedTransactionDispenser {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn BeginTransaction(&self) -> ::windows::core::Result<IMSMQTransaction> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).BeginTransaction)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQTransaction>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMSMQCoordinatedTransactionDispenser> for ::windows::core::IUnknown {
    fn from(value: IMSMQCoordinatedTransactionDispenser) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IMSMQCoordinatedTransactionDispenser> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IMSMQCoordinatedTransactionDispenser) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMSMQCoordinatedTransactionDispenser> for ::windows::core::IUnknown {
    fn from(value: &IMSMQCoordinatedTransactionDispenser) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMSMQCoordinatedTransactionDispenser> for super::Com::IDispatch {
    fn from(value: IMSMQCoordinatedTransactionDispenser) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IMSMQCoordinatedTransactionDispenser> for &'a super::Com::IDispatch {
    fn from(value: &'a IMSMQCoordinatedTransactionDispenser) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMSMQCoordinatedTransactionDispenser> for super::Com::IDispatch {
    fn from(value: &IMSMQCoordinatedTransactionDispenser) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IMSMQCoordinatedTransactionDispenser {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IMSMQCoordinatedTransactionDispenser {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IMSMQCoordinatedTransactionDispenser {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IMSMQCoordinatedTransactionDispenser {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMSMQCoordinatedTransactionDispenser").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IMSMQCoordinatedTransactionDispenser {
    type Vtable = IMSMQCoordinatedTransactionDispenser_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd7d6e081_dccd_11d0_aa4b_0060970debae);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQCoordinatedTransactionDispenser_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub BeginTransaction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptransaction: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    BeginTransaction: usize,
}
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IMSMQCoordinatedTransactionDispenser2(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IMSMQCoordinatedTransactionDispenser2 {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn BeginTransaction(&self) -> ::windows::core::Result<IMSMQTransaction2> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).BeginTransaction)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQTransaction2>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Properties(&self) -> ::windows::core::Result<super::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Properties)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::IDispatch>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMSMQCoordinatedTransactionDispenser2> for ::windows::core::IUnknown {
    fn from(value: IMSMQCoordinatedTransactionDispenser2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IMSMQCoordinatedTransactionDispenser2> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IMSMQCoordinatedTransactionDispenser2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMSMQCoordinatedTransactionDispenser2> for ::windows::core::IUnknown {
    fn from(value: &IMSMQCoordinatedTransactionDispenser2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMSMQCoordinatedTransactionDispenser2> for super::Com::IDispatch {
    fn from(value: IMSMQCoordinatedTransactionDispenser2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IMSMQCoordinatedTransactionDispenser2> for &'a super::Com::IDispatch {
    fn from(value: &'a IMSMQCoordinatedTransactionDispenser2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMSMQCoordinatedTransactionDispenser2> for super::Com::IDispatch {
    fn from(value: &IMSMQCoordinatedTransactionDispenser2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IMSMQCoordinatedTransactionDispenser2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IMSMQCoordinatedTransactionDispenser2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IMSMQCoordinatedTransactionDispenser2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IMSMQCoordinatedTransactionDispenser2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMSMQCoordinatedTransactionDispenser2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IMSMQCoordinatedTransactionDispenser2 {
    type Vtable = IMSMQCoordinatedTransactionDispenser2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeba96b10_2168_11d3_898c_00e02c074f6b);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQCoordinatedTransactionDispenser2_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub BeginTransaction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptransaction: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    BeginTransaction: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcolproperties: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Properties: usize,
}
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IMSMQCoordinatedTransactionDispenser3(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IMSMQCoordinatedTransactionDispenser3 {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn BeginTransaction(&self) -> ::windows::core::Result<IMSMQTransaction3> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).BeginTransaction)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQTransaction3>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Properties(&self) -> ::windows::core::Result<super::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Properties)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::IDispatch>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMSMQCoordinatedTransactionDispenser3> for ::windows::core::IUnknown {
    fn from(value: IMSMQCoordinatedTransactionDispenser3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IMSMQCoordinatedTransactionDispenser3> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IMSMQCoordinatedTransactionDispenser3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMSMQCoordinatedTransactionDispenser3> for ::windows::core::IUnknown {
    fn from(value: &IMSMQCoordinatedTransactionDispenser3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMSMQCoordinatedTransactionDispenser3> for super::Com::IDispatch {
    fn from(value: IMSMQCoordinatedTransactionDispenser3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IMSMQCoordinatedTransactionDispenser3> for &'a super::Com::IDispatch {
    fn from(value: &'a IMSMQCoordinatedTransactionDispenser3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMSMQCoordinatedTransactionDispenser3> for super::Com::IDispatch {
    fn from(value: &IMSMQCoordinatedTransactionDispenser3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IMSMQCoordinatedTransactionDispenser3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IMSMQCoordinatedTransactionDispenser3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IMSMQCoordinatedTransactionDispenser3 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IMSMQCoordinatedTransactionDispenser3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMSMQCoordinatedTransactionDispenser3").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IMSMQCoordinatedTransactionDispenser3 {
    type Vtable = IMSMQCoordinatedTransactionDispenser3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeba96b14_2168_11d3_898c_00e02c074f6b);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQCoordinatedTransactionDispenser3_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub BeginTransaction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptransaction: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    BeginTransaction: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcolproperties: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Properties: usize,
}
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IMSMQDestination(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IMSMQDestination {
    pub unsafe fn Open(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Open)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Close)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn IsOpen(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).IsOpen)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn IADs(&self) -> ::windows::core::Result<super::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).IADs)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::IDispatch>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn putref_IADs<'a, P0>(&self, piads: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::Com::IDispatch>>,
    {
        (::windows::core::Interface::vtable(self).putref_IADs)(::windows::core::Interface::as_raw(self), piads.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ADsPath(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).ADsPath)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetADsPath<'a, P0>(&self, bstradspath: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).SetADsPath)(::windows::core::Interface::as_raw(self), bstradspath.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PathName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).PathName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetPathName<'a, P0>(&self, bstrpathname: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).SetPathName)(::windows::core::Interface::as_raw(self), bstrpathname.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FormatName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).FormatName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetFormatName<'a, P0>(&self, bstrformatname: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).SetFormatName)(::windows::core::Interface::as_raw(self), bstrformatname.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Destinations(&self) -> ::windows::core::Result<super::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Destinations)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::IDispatch>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn putref_Destinations<'a, P0>(&self, pdestinations: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::Com::IDispatch>>,
    {
        (::windows::core::Interface::vtable(self).putref_Destinations)(::windows::core::Interface::as_raw(self), pdestinations.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Properties(&self) -> ::windows::core::Result<super::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Properties)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::IDispatch>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMSMQDestination> for ::windows::core::IUnknown {
    fn from(value: IMSMQDestination) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IMSMQDestination> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IMSMQDestination) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMSMQDestination> for ::windows::core::IUnknown {
    fn from(value: &IMSMQDestination) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMSMQDestination> for super::Com::IDispatch {
    fn from(value: IMSMQDestination) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IMSMQDestination> for &'a super::Com::IDispatch {
    fn from(value: &'a IMSMQDestination) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMSMQDestination> for super::Com::IDispatch {
    fn from(value: &IMSMQDestination) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IMSMQDestination {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IMSMQDestination {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IMSMQDestination {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IMSMQDestination {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMSMQDestination").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IMSMQDestination {
    type Vtable = IMSMQDestination_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeba96b16_2168_11d3_898c_00e02c074f6b);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQDestination_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Open: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub IsOpen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfisopen: *mut i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub IADs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppiads: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    IADs: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub putref_IADs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, piads: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    putref_IADs: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ADsPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstradspath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ADsPath: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetADsPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstradspath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetADsPath: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub PathName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrpathname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    PathName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetPathName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpathname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetPathName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub FormatName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrformatname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    FormatName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetFormatName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrformatname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetFormatName: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Destinations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdestinations: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Destinations: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub putref_Destinations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdestinations: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    putref_Destinations: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcolproperties: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Properties: usize,
}
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IMSMQEvent(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IMSMQEvent {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMSMQEvent> for ::windows::core::IUnknown {
    fn from(value: IMSMQEvent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IMSMQEvent> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IMSMQEvent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMSMQEvent> for ::windows::core::IUnknown {
    fn from(value: &IMSMQEvent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMSMQEvent> for super::Com::IDispatch {
    fn from(value: IMSMQEvent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IMSMQEvent> for &'a super::Com::IDispatch {
    fn from(value: &'a IMSMQEvent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMSMQEvent> for super::Com::IDispatch {
    fn from(value: &IMSMQEvent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IMSMQEvent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IMSMQEvent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IMSMQEvent {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IMSMQEvent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMSMQEvent").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IMSMQEvent {
    type Vtable = IMSMQEvent_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd7d6e077_dccd_11d0_aa4b_0060970debae);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQEvent_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
}
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IMSMQEvent2(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IMSMQEvent2 {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Properties(&self) -> ::windows::core::Result<super::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Properties)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::IDispatch>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMSMQEvent2> for ::windows::core::IUnknown {
    fn from(value: IMSMQEvent2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IMSMQEvent2> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IMSMQEvent2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMSMQEvent2> for ::windows::core::IUnknown {
    fn from(value: &IMSMQEvent2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMSMQEvent2> for super::Com::IDispatch {
    fn from(value: IMSMQEvent2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IMSMQEvent2> for &'a super::Com::IDispatch {
    fn from(value: &'a IMSMQEvent2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMSMQEvent2> for super::Com::IDispatch {
    fn from(value: &IMSMQEvent2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMSMQEvent2> for IMSMQEvent {
    fn from(value: IMSMQEvent2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IMSMQEvent2> for &'a IMSMQEvent {
    fn from(value: &'a IMSMQEvent2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMSMQEvent2> for IMSMQEvent {
    fn from(value: &IMSMQEvent2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IMSMQEvent2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IMSMQEvent2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IMSMQEvent2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IMSMQEvent2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMSMQEvent2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IMSMQEvent2 {
    type Vtable = IMSMQEvent2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeba96b12_2168_11d3_898c_00e02c074f6b);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQEvent2_Vtbl {
    pub base__: IMSMQEvent_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcolproperties: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Properties: usize,
}
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IMSMQEvent3(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IMSMQEvent3 {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Properties(&self) -> ::windows::core::Result<super::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.Properties)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::IDispatch>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMSMQEvent3> for ::windows::core::IUnknown {
    fn from(value: IMSMQEvent3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IMSMQEvent3> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IMSMQEvent3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMSMQEvent3> for ::windows::core::IUnknown {
    fn from(value: &IMSMQEvent3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMSMQEvent3> for super::Com::IDispatch {
    fn from(value: IMSMQEvent3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IMSMQEvent3> for &'a super::Com::IDispatch {
    fn from(value: &'a IMSMQEvent3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMSMQEvent3> for super::Com::IDispatch {
    fn from(value: &IMSMQEvent3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMSMQEvent3> for IMSMQEvent {
    fn from(value: IMSMQEvent3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IMSMQEvent3> for &'a IMSMQEvent {
    fn from(value: &'a IMSMQEvent3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMSMQEvent3> for IMSMQEvent {
    fn from(value: &IMSMQEvent3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMSMQEvent3> for IMSMQEvent2 {
    fn from(value: IMSMQEvent3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IMSMQEvent3> for &'a IMSMQEvent2 {
    fn from(value: &'a IMSMQEvent3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMSMQEvent3> for IMSMQEvent2 {
    fn from(value: &IMSMQEvent3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IMSMQEvent3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IMSMQEvent3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IMSMQEvent3 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IMSMQEvent3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMSMQEvent3").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IMSMQEvent3 {
    type Vtable = IMSMQEvent3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeba96b1c_2168_11d3_898c_00e02c074f6b);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQEvent3_Vtbl {
    pub base__: IMSMQEvent2_Vtbl,
}
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IMSMQManagement(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IMSMQManagement {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Init(&self, machine: *const super::Com::VARIANT, pathname: *const super::Com::VARIANT, formatname: *const super::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Init)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(machine), ::core::mem::transmute(pathname), ::core::mem::transmute(formatname)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FormatName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).FormatName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Machine(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Machine)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn MessageCount(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).MessageCount)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn ForeignStatus(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).ForeignStatus)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn QueueType(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).QueueType)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn IsLocal(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).IsLocal)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn TransactionalStatus(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).TransactionalStatus)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn BytesInQueue(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).BytesInQueue)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMSMQManagement> for ::windows::core::IUnknown {
    fn from(value: IMSMQManagement) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IMSMQManagement> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IMSMQManagement) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMSMQManagement> for ::windows::core::IUnknown {
    fn from(value: &IMSMQManagement) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMSMQManagement> for super::Com::IDispatch {
    fn from(value: IMSMQManagement) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IMSMQManagement> for &'a super::Com::IDispatch {
    fn from(value: &'a IMSMQManagement) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMSMQManagement> for super::Com::IDispatch {
    fn from(value: &IMSMQManagement) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IMSMQManagement {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IMSMQManagement {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IMSMQManagement {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IMSMQManagement {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMSMQManagement").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IMSMQManagement {
    type Vtable = IMSMQManagement_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbe5f0241_e489_4957_8cc4_a452fcf3e23e);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQManagement_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Init: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, machine: *const super::Com::VARIANT, pathname: *const super::Com::VARIANT, formatname: *const super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Init: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub FormatName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrformatname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    FormatName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Machine: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrmachine: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Machine: usize,
    pub MessageCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plmessagecount: *mut i32) -> ::windows::core::HRESULT,
    pub ForeignStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plforeignstatus: *mut i32) -> ::windows::core::HRESULT,
    pub QueueType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plqueuetype: *mut i32) -> ::windows::core::HRESULT,
    pub IsLocal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfislocal: *mut i16) -> ::windows::core::HRESULT,
    pub TransactionalStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pltransactionalstatus: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub BytesInQueue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvbytesinqueue: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    BytesInQueue: usize,
}
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IMSMQMessage(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IMSMQMessage {
    pub unsafe fn Class(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Class)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn PrivLevel(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).PrivLevel)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetPrivLevel(&self, lprivlevel: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetPrivLevel)(::windows::core::Interface::as_raw(self), lprivlevel).ok()
    }
    pub unsafe fn AuthLevel(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).AuthLevel)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetAuthLevel(&self, lauthlevel: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAuthLevel)(::windows::core::Interface::as_raw(self), lauthlevel).ok()
    }
    pub unsafe fn IsAuthenticated(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).IsAuthenticated)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn Delivery(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Delivery)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetDelivery(&self, ldelivery: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDelivery)(::windows::core::Interface::as_raw(self), ldelivery).ok()
    }
    pub unsafe fn Trace(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Trace)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetTrace(&self, ltrace: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetTrace)(::windows::core::Interface::as_raw(self), ltrace).ok()
    }
    pub unsafe fn Priority(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Priority)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetPriority(&self, lpriority: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetPriority)(::windows::core::Interface::as_raw(self), lpriority).ok()
    }
    pub unsafe fn Journal(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Journal)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetJournal(&self, ljournal: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetJournal)(::windows::core::Interface::as_raw(self), ljournal).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ResponseQueueInfo(&self) -> ::windows::core::Result<IMSMQQueueInfo> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).ResponseQueueInfo)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQQueueInfo>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn putref_ResponseQueueInfo<'a, P0>(&self, pqinforesponse: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IMSMQQueueInfo>>,
    {
        (::windows::core::Interface::vtable(self).putref_ResponseQueueInfo)(::windows::core::Interface::as_raw(self), pqinforesponse.into().abi()).ok()
    }
    pub unsafe fn AppSpecific(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).AppSpecific)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetAppSpecific(&self, lappspecific: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAppSpecific)(::windows::core::Interface::as_raw(self), lappspecific).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SourceMachineGuid(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).SourceMachineGuid)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn BodyLength(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).BodyLength)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Body(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Body)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetBody<'a, P0>(&self, varbody: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::Com::VARIANT>>,
    {
        (::windows::core::Interface::vtable(self).SetBody)(::windows::core::Interface::as_raw(self), varbody.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AdminQueueInfo(&self) -> ::windows::core::Result<IMSMQQueueInfo> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).AdminQueueInfo)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQQueueInfo>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn putref_AdminQueueInfo<'a, P0>(&self, pqinfoadmin: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IMSMQQueueInfo>>,
    {
        (::windows::core::Interface::vtable(self).putref_AdminQueueInfo)(::windows::core::Interface::as_raw(self), pqinfoadmin.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Id(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Id)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CorrelationId(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).CorrelationId)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetCorrelationId<'a, P0>(&self, varmsgid: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::Com::VARIANT>>,
    {
        (::windows::core::Interface::vtable(self).SetCorrelationId)(::windows::core::Interface::as_raw(self), varmsgid.into().abi()).ok()
    }
    pub unsafe fn Ack(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Ack)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetAck(&self, lack: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAck)(::windows::core::Interface::as_raw(self), lack).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Label(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Label)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLabel<'a, P0>(&self, bstrlabel: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).SetLabel)(::windows::core::Interface::as_raw(self), bstrlabel.into().abi()).ok()
    }
    pub unsafe fn MaxTimeToReachQueue(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).MaxTimeToReachQueue)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetMaxTimeToReachQueue(&self, lmaxtimetoreachqueue: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetMaxTimeToReachQueue)(::windows::core::Interface::as_raw(self), lmaxtimetoreachqueue).ok()
    }
    pub unsafe fn MaxTimeToReceive(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).MaxTimeToReceive)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetMaxTimeToReceive(&self, lmaxtimetoreceive: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetMaxTimeToReceive)(::windows::core::Interface::as_raw(self), lmaxtimetoreceive).ok()
    }
    pub unsafe fn HashAlgorithm(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).HashAlgorithm)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetHashAlgorithm(&self, lhashalg: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetHashAlgorithm)(::windows::core::Interface::as_raw(self), lhashalg).ok()
    }
    pub unsafe fn EncryptAlgorithm(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).EncryptAlgorithm)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetEncryptAlgorithm(&self, lencryptalg: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetEncryptAlgorithm)(::windows::core::Interface::as_raw(self), lencryptalg).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SentTime(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).SentTime)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ArrivedTime(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).ArrivedTime)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn DestinationQueueInfo(&self) -> ::windows::core::Result<IMSMQQueueInfo> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).DestinationQueueInfo)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQQueueInfo>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SenderCertificate(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).SenderCertificate)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetSenderCertificate<'a, P0>(&self, varsendercert: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::Com::VARIANT>>,
    {
        (::windows::core::Interface::vtable(self).SetSenderCertificate)(::windows::core::Interface::as_raw(self), varsendercert.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SenderId(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).SenderId)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    pub unsafe fn SenderIdType(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).SenderIdType)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetSenderIdType(&self, lsenderidtype: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSenderIdType)(::windows::core::Interface::as_raw(self), lsenderidtype).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Send<'a, P0>(&self, destinationqueue: P0, transaction: *const super::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IMSMQQueue>>,
    {
        (::windows::core::Interface::vtable(self).Send)(::windows::core::Interface::as_raw(self), destinationqueue.into().abi(), ::core::mem::transmute(transaction)).ok()
    }
    pub unsafe fn AttachCurrentSecurityContext(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AttachCurrentSecurityContext)(::windows::core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMSMQMessage> for ::windows::core::IUnknown {
    fn from(value: IMSMQMessage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IMSMQMessage> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IMSMQMessage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMSMQMessage> for ::windows::core::IUnknown {
    fn from(value: &IMSMQMessage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMSMQMessage> for super::Com::IDispatch {
    fn from(value: IMSMQMessage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IMSMQMessage> for &'a super::Com::IDispatch {
    fn from(value: &'a IMSMQMessage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMSMQMessage> for super::Com::IDispatch {
    fn from(value: &IMSMQMessage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IMSMQMessage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IMSMQMessage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IMSMQMessage {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IMSMQMessage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMSMQMessage").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IMSMQMessage {
    type Vtable = IMSMQMessage_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd7d6e074_dccd_11d0_aa4b_0060970debae);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQMessage_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Class: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plclass: *mut i32) -> ::windows::core::HRESULT,
    pub PrivLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plprivlevel: *mut i32) -> ::windows::core::HRESULT,
    pub SetPrivLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lprivlevel: i32) -> ::windows::core::HRESULT,
    pub AuthLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plauthlevel: *mut i32) -> ::windows::core::HRESULT,
    pub SetAuthLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lauthlevel: i32) -> ::windows::core::HRESULT,
    pub IsAuthenticated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pisauthenticated: *mut i16) -> ::windows::core::HRESULT,
    pub Delivery: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pldelivery: *mut i32) -> ::windows::core::HRESULT,
    pub SetDelivery: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ldelivery: i32) -> ::windows::core::HRESULT,
    pub Trace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pltrace: *mut i32) -> ::windows::core::HRESULT,
    pub SetTrace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ltrace: i32) -> ::windows::core::HRESULT,
    pub Priority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plpriority: *mut i32) -> ::windows::core::HRESULT,
    pub SetPriority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpriority: i32) -> ::windows::core::HRESULT,
    pub Journal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pljournal: *mut i32) -> ::windows::core::HRESULT,
    pub SetJournal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ljournal: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub ResponseQueueInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppqinforesponse: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ResponseQueueInfo: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub putref_ResponseQueueInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pqinforesponse: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    putref_ResponseQueueInfo: usize,
    pub AppSpecific: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plappspecific: *mut i32) -> ::windows::core::HRESULT,
    pub SetAppSpecific: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lappspecific: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SourceMachineGuid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrguidsrcmachine: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SourceMachineGuid: usize,
    pub BodyLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcbbody: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Body: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarbody: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Body: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetBody: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varbody: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetBody: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub AdminQueueInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppqinfoadmin: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AdminQueueInfo: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub putref_AdminQueueInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pqinfoadmin: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    putref_AdminQueueInfo: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarmsgid: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Id: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub CorrelationId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarmsgid: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    CorrelationId: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetCorrelationId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varmsgid: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetCorrelationId: usize,
    pub Ack: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plack: *mut i32) -> ::windows::core::HRESULT,
    pub SetAck: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lack: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Label: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrlabel: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Label: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetLabel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrlabel: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetLabel: usize,
    pub MaxTimeToReachQueue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plmaxtimetoreachqueue: *mut i32) -> ::windows::core::HRESULT,
    pub SetMaxTimeToReachQueue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lmaxtimetoreachqueue: i32) -> ::windows::core::HRESULT,
    pub MaxTimeToReceive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plmaxtimetoreceive: *mut i32) -> ::windows::core::HRESULT,
    pub SetMaxTimeToReceive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lmaxtimetoreceive: i32) -> ::windows::core::HRESULT,
    pub HashAlgorithm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plhashalg: *mut i32) -> ::windows::core::HRESULT,
    pub SetHashAlgorithm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lhashalg: i32) -> ::windows::core::HRESULT,
    pub EncryptAlgorithm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plencryptalg: *mut i32) -> ::windows::core::HRESULT,
    pub SetEncryptAlgorithm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lencryptalg: i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SentTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarsenttime: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SentTime: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub ArrivedTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plarrivedtime: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    ArrivedTime: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub DestinationQueueInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppqinfodest: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    DestinationQueueInfo: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SenderCertificate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarsendercert: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SenderCertificate: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetSenderCertificate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varsendercert: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetSenderCertificate: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SenderId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarsenderid: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SenderId: usize,
    pub SenderIdType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plsenderidtype: *mut i32) -> ::windows::core::HRESULT,
    pub SetSenderIdType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lsenderidtype: i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Send: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, destinationqueue: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Send: usize,
    pub AttachCurrentSecurityContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IMSMQMessage2(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IMSMQMessage2 {
    pub unsafe fn Class(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Class)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn PrivLevel(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).PrivLevel)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetPrivLevel(&self, lprivlevel: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetPrivLevel)(::windows::core::Interface::as_raw(self), lprivlevel).ok()
    }
    pub unsafe fn AuthLevel(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).AuthLevel)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetAuthLevel(&self, lauthlevel: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAuthLevel)(::windows::core::Interface::as_raw(self), lauthlevel).ok()
    }
    pub unsafe fn IsAuthenticated(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).IsAuthenticated)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn Delivery(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Delivery)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetDelivery(&self, ldelivery: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDelivery)(::windows::core::Interface::as_raw(self), ldelivery).ok()
    }
    pub unsafe fn Trace(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Trace)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetTrace(&self, ltrace: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetTrace)(::windows::core::Interface::as_raw(self), ltrace).ok()
    }
    pub unsafe fn Priority(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Priority)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetPriority(&self, lpriority: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetPriority)(::windows::core::Interface::as_raw(self), lpriority).ok()
    }
    pub unsafe fn Journal(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Journal)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetJournal(&self, ljournal: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetJournal)(::windows::core::Interface::as_raw(self), ljournal).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ResponseQueueInfo_v1(&self) -> ::windows::core::Result<IMSMQQueueInfo> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).ResponseQueueInfo_v1)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQQueueInfo>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn putref_ResponseQueueInfo_v1<'a, P0>(&self, pqinforesponse: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IMSMQQueueInfo>>,
    {
        (::windows::core::Interface::vtable(self).putref_ResponseQueueInfo_v1)(::windows::core::Interface::as_raw(self), pqinforesponse.into().abi()).ok()
    }
    pub unsafe fn AppSpecific(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).AppSpecific)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetAppSpecific(&self, lappspecific: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAppSpecific)(::windows::core::Interface::as_raw(self), lappspecific).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SourceMachineGuid(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).SourceMachineGuid)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn BodyLength(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).BodyLength)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Body(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Body)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetBody<'a, P0>(&self, varbody: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::Com::VARIANT>>,
    {
        (::windows::core::Interface::vtable(self).SetBody)(::windows::core::Interface::as_raw(self), varbody.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AdminQueueInfo_v1(&self) -> ::windows::core::Result<IMSMQQueueInfo> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).AdminQueueInfo_v1)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQQueueInfo>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn putref_AdminQueueInfo_v1<'a, P0>(&self, pqinfoadmin: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IMSMQQueueInfo>>,
    {
        (::windows::core::Interface::vtable(self).putref_AdminQueueInfo_v1)(::windows::core::Interface::as_raw(self), pqinfoadmin.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Id(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Id)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CorrelationId(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).CorrelationId)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetCorrelationId<'a, P0>(&self, varmsgid: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::Com::VARIANT>>,
    {
        (::windows::core::Interface::vtable(self).SetCorrelationId)(::windows::core::Interface::as_raw(self), varmsgid.into().abi()).ok()
    }
    pub unsafe fn Ack(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Ack)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetAck(&self, lack: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAck)(::windows::core::Interface::as_raw(self), lack).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Label(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Label)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLabel<'a, P0>(&self, bstrlabel: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).SetLabel)(::windows::core::Interface::as_raw(self), bstrlabel.into().abi()).ok()
    }
    pub unsafe fn MaxTimeToReachQueue(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).MaxTimeToReachQueue)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetMaxTimeToReachQueue(&self, lmaxtimetoreachqueue: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetMaxTimeToReachQueue)(::windows::core::Interface::as_raw(self), lmaxtimetoreachqueue).ok()
    }
    pub unsafe fn MaxTimeToReceive(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).MaxTimeToReceive)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetMaxTimeToReceive(&self, lmaxtimetoreceive: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetMaxTimeToReceive)(::windows::core::Interface::as_raw(self), lmaxtimetoreceive).ok()
    }
    pub unsafe fn HashAlgorithm(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).HashAlgorithm)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetHashAlgorithm(&self, lhashalg: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetHashAlgorithm)(::windows::core::Interface::as_raw(self), lhashalg).ok()
    }
    pub unsafe fn EncryptAlgorithm(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).EncryptAlgorithm)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetEncryptAlgorithm(&self, lencryptalg: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetEncryptAlgorithm)(::windows::core::Interface::as_raw(self), lencryptalg).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SentTime(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).SentTime)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ArrivedTime(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).ArrivedTime)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn DestinationQueueInfo(&self) -> ::windows::core::Result<IMSMQQueueInfo2> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).DestinationQueueInfo)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQQueueInfo2>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SenderCertificate(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).SenderCertificate)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetSenderCertificate<'a, P0>(&self, varsendercert: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::Com::VARIANT>>,
    {
        (::windows::core::Interface::vtable(self).SetSenderCertificate)(::windows::core::Interface::as_raw(self), varsendercert.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SenderId(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).SenderId)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    pub unsafe fn SenderIdType(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).SenderIdType)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetSenderIdType(&self, lsenderidtype: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSenderIdType)(::windows::core::Interface::as_raw(self), lsenderidtype).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Send<'a, P0>(&self, destinationqueue: P0, transaction: *const super::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IMSMQQueue2>>,
    {
        (::windows::core::Interface::vtable(self).Send)(::windows::core::Interface::as_raw(self), destinationqueue.into().abi(), ::core::mem::transmute(transaction)).ok()
    }
    pub unsafe fn AttachCurrentSecurityContext(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AttachCurrentSecurityContext)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SenderVersion(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).SenderVersion)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Extension(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Extension)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetExtension<'a, P0>(&self, varextension: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::Com::VARIANT>>,
    {
        (::windows::core::Interface::vtable(self).SetExtension)(::windows::core::Interface::as_raw(self), varextension.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ConnectorTypeGuid(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).ConnectorTypeGuid)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetConnectorTypeGuid<'a, P0>(&self, bstrguidconnectortype: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).SetConnectorTypeGuid)(::windows::core::Interface::as_raw(self), bstrguidconnectortype.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn TransactionStatusQueueInfo(&self) -> ::windows::core::Result<IMSMQQueueInfo2> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).TransactionStatusQueueInfo)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQQueueInfo2>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DestinationSymmetricKey(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).DestinationSymmetricKey)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetDestinationSymmetricKey<'a, P0>(&self, vardestsymmkey: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::Com::VARIANT>>,
    {
        (::windows::core::Interface::vtable(self).SetDestinationSymmetricKey)(::windows::core::Interface::as_raw(self), vardestsymmkey.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Signature(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Signature)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetSignature<'a, P0>(&self, varsignature: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::Com::VARIANT>>,
    {
        (::windows::core::Interface::vtable(self).SetSignature)(::windows::core::Interface::as_raw(self), varsignature.into().abi()).ok()
    }
    pub unsafe fn AuthenticationProviderType(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).AuthenticationProviderType)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetAuthenticationProviderType(&self, lauthprovtype: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAuthenticationProviderType)(::windows::core::Interface::as_raw(self), lauthprovtype).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AuthenticationProviderName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).AuthenticationProviderName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAuthenticationProviderName<'a, P0>(&self, bstrauthprovname: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).SetAuthenticationProviderName)(::windows::core::Interface::as_raw(self), bstrauthprovname.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetSenderId<'a, P0>(&self, varsenderid: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::Com::VARIANT>>,
    {
        (::windows::core::Interface::vtable(self).SetSenderId)(::windows::core::Interface::as_raw(self), varsenderid.into().abi()).ok()
    }
    pub unsafe fn MsgClass(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).MsgClass)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetMsgClass(&self, lmsgclass: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetMsgClass)(::windows::core::Interface::as_raw(self), lmsgclass).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Properties(&self) -> ::windows::core::Result<super::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Properties)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::IDispatch>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn TransactionId(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).TransactionId)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    pub unsafe fn IsFirstInTransaction(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).IsFirstInTransaction)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn IsLastInTransaction(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).IsLastInTransaction)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ResponseQueueInfo(&self) -> ::windows::core::Result<IMSMQQueueInfo2> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).ResponseQueueInfo)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQQueueInfo2>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn putref_ResponseQueueInfo<'a, P0>(&self, pqinforesponse: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IMSMQQueueInfo2>>,
    {
        (::windows::core::Interface::vtable(self).putref_ResponseQueueInfo)(::windows::core::Interface::as_raw(self), pqinforesponse.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AdminQueueInfo(&self) -> ::windows::core::Result<IMSMQQueueInfo2> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).AdminQueueInfo)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQQueueInfo2>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn putref_AdminQueueInfo<'a, P0>(&self, pqinfoadmin: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IMSMQQueueInfo2>>,
    {
        (::windows::core::Interface::vtable(self).putref_AdminQueueInfo)(::windows::core::Interface::as_raw(self), pqinfoadmin.into().abi()).ok()
    }
    pub unsafe fn ReceivedAuthenticationLevel(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).ReceivedAuthenticationLevel)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMSMQMessage2> for ::windows::core::IUnknown {
    fn from(value: IMSMQMessage2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IMSMQMessage2> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IMSMQMessage2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMSMQMessage2> for ::windows::core::IUnknown {
    fn from(value: &IMSMQMessage2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMSMQMessage2> for super::Com::IDispatch {
    fn from(value: IMSMQMessage2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IMSMQMessage2> for &'a super::Com::IDispatch {
    fn from(value: &'a IMSMQMessage2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMSMQMessage2> for super::Com::IDispatch {
    fn from(value: &IMSMQMessage2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IMSMQMessage2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IMSMQMessage2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IMSMQMessage2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IMSMQMessage2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMSMQMessage2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IMSMQMessage2 {
    type Vtable = IMSMQMessage2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd9933be0_a567_11d2_b0f3_00e02c074f6b);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQMessage2_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Class: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plclass: *mut i32) -> ::windows::core::HRESULT,
    pub PrivLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plprivlevel: *mut i32) -> ::windows::core::HRESULT,
    pub SetPrivLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lprivlevel: i32) -> ::windows::core::HRESULT,
    pub AuthLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plauthlevel: *mut i32) -> ::windows::core::HRESULT,
    pub SetAuthLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lauthlevel: i32) -> ::windows::core::HRESULT,
    pub IsAuthenticated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pisauthenticated: *mut i16) -> ::windows::core::HRESULT,
    pub Delivery: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pldelivery: *mut i32) -> ::windows::core::HRESULT,
    pub SetDelivery: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ldelivery: i32) -> ::windows::core::HRESULT,
    pub Trace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pltrace: *mut i32) -> ::windows::core::HRESULT,
    pub SetTrace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ltrace: i32) -> ::windows::core::HRESULT,
    pub Priority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plpriority: *mut i32) -> ::windows::core::HRESULT,
    pub SetPriority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpriority: i32) -> ::windows::core::HRESULT,
    pub Journal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pljournal: *mut i32) -> ::windows::core::HRESULT,
    pub SetJournal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ljournal: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub ResponseQueueInfo_v1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppqinforesponse: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ResponseQueueInfo_v1: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub putref_ResponseQueueInfo_v1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pqinforesponse: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    putref_ResponseQueueInfo_v1: usize,
    pub AppSpecific: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plappspecific: *mut i32) -> ::windows::core::HRESULT,
    pub SetAppSpecific: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lappspecific: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SourceMachineGuid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrguidsrcmachine: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SourceMachineGuid: usize,
    pub BodyLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcbbody: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Body: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarbody: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Body: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetBody: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varbody: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetBody: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub AdminQueueInfo_v1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppqinfoadmin: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AdminQueueInfo_v1: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub putref_AdminQueueInfo_v1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pqinfoadmin: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    putref_AdminQueueInfo_v1: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarmsgid: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Id: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub CorrelationId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarmsgid: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    CorrelationId: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetCorrelationId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varmsgid: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetCorrelationId: usize,
    pub Ack: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plack: *mut i32) -> ::windows::core::HRESULT,
    pub SetAck: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lack: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Label: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrlabel: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Label: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetLabel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrlabel: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetLabel: usize,
    pub MaxTimeToReachQueue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plmaxtimetoreachqueue: *mut i32) -> ::windows::core::HRESULT,
    pub SetMaxTimeToReachQueue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lmaxtimetoreachqueue: i32) -> ::windows::core::HRESULT,
    pub MaxTimeToReceive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plmaxtimetoreceive: *mut i32) -> ::windows::core::HRESULT,
    pub SetMaxTimeToReceive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lmaxtimetoreceive: i32) -> ::windows::core::HRESULT,
    pub HashAlgorithm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plhashalg: *mut i32) -> ::windows::core::HRESULT,
    pub SetHashAlgorithm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lhashalg: i32) -> ::windows::core::HRESULT,
    pub EncryptAlgorithm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plencryptalg: *mut i32) -> ::windows::core::HRESULT,
    pub SetEncryptAlgorithm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lencryptalg: i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SentTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarsenttime: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SentTime: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub ArrivedTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plarrivedtime: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    ArrivedTime: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub DestinationQueueInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppqinfodest: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    DestinationQueueInfo: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SenderCertificate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarsendercert: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SenderCertificate: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetSenderCertificate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varsendercert: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetSenderCertificate: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SenderId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarsenderid: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SenderId: usize,
    pub SenderIdType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plsenderidtype: *mut i32) -> ::windows::core::HRESULT,
    pub SetSenderIdType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lsenderidtype: i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Send: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, destinationqueue: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Send: usize,
    pub AttachCurrentSecurityContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SenderVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plsenderversion: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Extension: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarextension: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Extension: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetExtension: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varextension: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetExtension: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ConnectorTypeGuid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrguidconnectortype: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ConnectorTypeGuid: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetConnectorTypeGuid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrguidconnectortype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetConnectorTypeGuid: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub TransactionStatusQueueInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppqinfoxactstatus: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    TransactionStatusQueueInfo: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DestinationSymmetricKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvardestsymmkey: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DestinationSymmetricKey: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetDestinationSymmetricKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vardestsymmkey: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetDestinationSymmetricKey: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Signature: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarsignature: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Signature: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetSignature: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varsignature: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetSignature: usize,
    pub AuthenticationProviderType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plauthprovtype: *mut i32) -> ::windows::core::HRESULT,
    pub SetAuthenticationProviderType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lauthprovtype: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub AuthenticationProviderName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrauthprovname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AuthenticationProviderName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetAuthenticationProviderName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrauthprovname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetAuthenticationProviderName: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetSenderId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varsenderid: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetSenderId: usize,
    pub MsgClass: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plmsgclass: *mut i32) -> ::windows::core::HRESULT,
    pub SetMsgClass: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lmsgclass: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcolproperties: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Properties: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub TransactionId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarxactid: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    TransactionId: usize,
    pub IsFirstInTransaction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pisfirstinxact: *mut i16) -> ::windows::core::HRESULT,
    pub IsLastInTransaction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pislastinxact: *mut i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub ResponseQueueInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppqinforesponse: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ResponseQueueInfo: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub putref_ResponseQueueInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pqinforesponse: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    putref_ResponseQueueInfo: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub AdminQueueInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppqinfoadmin: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AdminQueueInfo: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub putref_AdminQueueInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pqinfoadmin: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    putref_AdminQueueInfo: usize,
    pub ReceivedAuthenticationLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psreceivedauthenticationlevel: *mut i16) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IMSMQMessage3(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IMSMQMessage3 {
    pub unsafe fn Class(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Class)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn PrivLevel(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).PrivLevel)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetPrivLevel(&self, lprivlevel: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetPrivLevel)(::windows::core::Interface::as_raw(self), lprivlevel).ok()
    }
    pub unsafe fn AuthLevel(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).AuthLevel)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetAuthLevel(&self, lauthlevel: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAuthLevel)(::windows::core::Interface::as_raw(self), lauthlevel).ok()
    }
    pub unsafe fn IsAuthenticated(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).IsAuthenticated)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn Delivery(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Delivery)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetDelivery(&self, ldelivery: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDelivery)(::windows::core::Interface::as_raw(self), ldelivery).ok()
    }
    pub unsafe fn Trace(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Trace)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetTrace(&self, ltrace: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetTrace)(::windows::core::Interface::as_raw(self), ltrace).ok()
    }
    pub unsafe fn Priority(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Priority)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetPriority(&self, lpriority: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetPriority)(::windows::core::Interface::as_raw(self), lpriority).ok()
    }
    pub unsafe fn Journal(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Journal)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetJournal(&self, ljournal: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetJournal)(::windows::core::Interface::as_raw(self), ljournal).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ResponseQueueInfo_v1(&self) -> ::windows::core::Result<IMSMQQueueInfo> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).ResponseQueueInfo_v1)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQQueueInfo>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn putref_ResponseQueueInfo_v1<'a, P0>(&self, pqinforesponse: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IMSMQQueueInfo>>,
    {
        (::windows::core::Interface::vtable(self).putref_ResponseQueueInfo_v1)(::windows::core::Interface::as_raw(self), pqinforesponse.into().abi()).ok()
    }
    pub unsafe fn AppSpecific(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).AppSpecific)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetAppSpecific(&self, lappspecific: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAppSpecific)(::windows::core::Interface::as_raw(self), lappspecific).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SourceMachineGuid(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).SourceMachineGuid)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn BodyLength(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).BodyLength)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Body(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Body)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetBody<'a, P0>(&self, varbody: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::Com::VARIANT>>,
    {
        (::windows::core::Interface::vtable(self).SetBody)(::windows::core::Interface::as_raw(self), varbody.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AdminQueueInfo_v1(&self) -> ::windows::core::Result<IMSMQQueueInfo> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).AdminQueueInfo_v1)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQQueueInfo>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn putref_AdminQueueInfo_v1<'a, P0>(&self, pqinfoadmin: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IMSMQQueueInfo>>,
    {
        (::windows::core::Interface::vtable(self).putref_AdminQueueInfo_v1)(::windows::core::Interface::as_raw(self), pqinfoadmin.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Id(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Id)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CorrelationId(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).CorrelationId)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetCorrelationId<'a, P0>(&self, varmsgid: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::Com::VARIANT>>,
    {
        (::windows::core::Interface::vtable(self).SetCorrelationId)(::windows::core::Interface::as_raw(self), varmsgid.into().abi()).ok()
    }
    pub unsafe fn Ack(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Ack)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetAck(&self, lack: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAck)(::windows::core::Interface::as_raw(self), lack).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Label(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Label)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLabel<'a, P0>(&self, bstrlabel: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).SetLabel)(::windows::core::Interface::as_raw(self), bstrlabel.into().abi()).ok()
    }
    pub unsafe fn MaxTimeToReachQueue(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).MaxTimeToReachQueue)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetMaxTimeToReachQueue(&self, lmaxtimetoreachqueue: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetMaxTimeToReachQueue)(::windows::core::Interface::as_raw(self), lmaxtimetoreachqueue).ok()
    }
    pub unsafe fn MaxTimeToReceive(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).MaxTimeToReceive)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetMaxTimeToReceive(&self, lmaxtimetoreceive: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetMaxTimeToReceive)(::windows::core::Interface::as_raw(self), lmaxtimetoreceive).ok()
    }
    pub unsafe fn HashAlgorithm(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).HashAlgorithm)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetHashAlgorithm(&self, lhashalg: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetHashAlgorithm)(::windows::core::Interface::as_raw(self), lhashalg).ok()
    }
    pub unsafe fn EncryptAlgorithm(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).EncryptAlgorithm)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetEncryptAlgorithm(&self, lencryptalg: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetEncryptAlgorithm)(::windows::core::Interface::as_raw(self), lencryptalg).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SentTime(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).SentTime)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ArrivedTime(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).ArrivedTime)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn DestinationQueueInfo(&self) -> ::windows::core::Result<IMSMQQueueInfo3> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).DestinationQueueInfo)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQQueueInfo3>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SenderCertificate(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).SenderCertificate)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetSenderCertificate<'a, P0>(&self, varsendercert: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::Com::VARIANT>>,
    {
        (::windows::core::Interface::vtable(self).SetSenderCertificate)(::windows::core::Interface::as_raw(self), varsendercert.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SenderId(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).SenderId)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    pub unsafe fn SenderIdType(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).SenderIdType)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetSenderIdType(&self, lsenderidtype: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSenderIdType)(::windows::core::Interface::as_raw(self), lsenderidtype).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Send<'a, P0>(&self, destinationqueue: P0, transaction: *const super::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::Com::IDispatch>>,
    {
        (::windows::core::Interface::vtable(self).Send)(::windows::core::Interface::as_raw(self), destinationqueue.into().abi(), ::core::mem::transmute(transaction)).ok()
    }
    pub unsafe fn AttachCurrentSecurityContext(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AttachCurrentSecurityContext)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SenderVersion(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).SenderVersion)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Extension(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Extension)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetExtension<'a, P0>(&self, varextension: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::Com::VARIANT>>,
    {
        (::windows::core::Interface::vtable(self).SetExtension)(::windows::core::Interface::as_raw(self), varextension.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ConnectorTypeGuid(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).ConnectorTypeGuid)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetConnectorTypeGuid<'a, P0>(&self, bstrguidconnectortype: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).SetConnectorTypeGuid)(::windows::core::Interface::as_raw(self), bstrguidconnectortype.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn TransactionStatusQueueInfo(&self) -> ::windows::core::Result<IMSMQQueueInfo3> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).TransactionStatusQueueInfo)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQQueueInfo3>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DestinationSymmetricKey(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).DestinationSymmetricKey)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetDestinationSymmetricKey<'a, P0>(&self, vardestsymmkey: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::Com::VARIANT>>,
    {
        (::windows::core::Interface::vtable(self).SetDestinationSymmetricKey)(::windows::core::Interface::as_raw(self), vardestsymmkey.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Signature(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Signature)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetSignature<'a, P0>(&self, varsignature: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::Com::VARIANT>>,
    {
        (::windows::core::Interface::vtable(self).SetSignature)(::windows::core::Interface::as_raw(self), varsignature.into().abi()).ok()
    }
    pub unsafe fn AuthenticationProviderType(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).AuthenticationProviderType)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetAuthenticationProviderType(&self, lauthprovtype: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAuthenticationProviderType)(::windows::core::Interface::as_raw(self), lauthprovtype).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AuthenticationProviderName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).AuthenticationProviderName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAuthenticationProviderName<'a, P0>(&self, bstrauthprovname: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).SetAuthenticationProviderName)(::windows::core::Interface::as_raw(self), bstrauthprovname.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetSenderId<'a, P0>(&self, varsenderid: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::Com::VARIANT>>,
    {
        (::windows::core::Interface::vtable(self).SetSenderId)(::windows::core::Interface::as_raw(self), varsenderid.into().abi()).ok()
    }
    pub unsafe fn MsgClass(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).MsgClass)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetMsgClass(&self, lmsgclass: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetMsgClass)(::windows::core::Interface::as_raw(self), lmsgclass).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Properties(&self) -> ::windows::core::Result<super::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Properties)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::IDispatch>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn TransactionId(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).TransactionId)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    pub unsafe fn IsFirstInTransaction(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).IsFirstInTransaction)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn IsLastInTransaction(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).IsLastInTransaction)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ResponseQueueInfo_v2(&self) -> ::windows::core::Result<IMSMQQueueInfo2> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).ResponseQueueInfo_v2)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQQueueInfo2>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn putref_ResponseQueueInfo_v2<'a, P0>(&self, pqinforesponse: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IMSMQQueueInfo2>>,
    {
        (::windows::core::Interface::vtable(self).putref_ResponseQueueInfo_v2)(::windows::core::Interface::as_raw(self), pqinforesponse.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AdminQueueInfo_v2(&self) -> ::windows::core::Result<IMSMQQueueInfo2> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).AdminQueueInfo_v2)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQQueueInfo2>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn putref_AdminQueueInfo_v2<'a, P0>(&self, pqinfoadmin: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IMSMQQueueInfo2>>,
    {
        (::windows::core::Interface::vtable(self).putref_AdminQueueInfo_v2)(::windows::core::Interface::as_raw(self), pqinfoadmin.into().abi()).ok()
    }
    pub unsafe fn ReceivedAuthenticationLevel(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).ReceivedAuthenticationLevel)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ResponseQueueInfo(&self) -> ::windows::core::Result<IMSMQQueueInfo3> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).ResponseQueueInfo)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQQueueInfo3>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn putref_ResponseQueueInfo<'a, P0>(&self, pqinforesponse: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IMSMQQueueInfo3>>,
    {
        (::windows::core::Interface::vtable(self).putref_ResponseQueueInfo)(::windows::core::Interface::as_raw(self), pqinforesponse.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AdminQueueInfo(&self) -> ::windows::core::Result<IMSMQQueueInfo3> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).AdminQueueInfo)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQQueueInfo3>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn putref_AdminQueueInfo<'a, P0>(&self, pqinfoadmin: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IMSMQQueueInfo3>>,
    {
        (::windows::core::Interface::vtable(self).putref_AdminQueueInfo)(::windows::core::Interface::as_raw(self), pqinfoadmin.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ResponseDestination(&self) -> ::windows::core::Result<super::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).ResponseDestination)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::IDispatch>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn putref_ResponseDestination<'a, P0>(&self, pdestresponse: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::Com::IDispatch>>,
    {
        (::windows::core::Interface::vtable(self).putref_ResponseDestination)(::windows::core::Interface::as_raw(self), pdestresponse.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Destination(&self) -> ::windows::core::Result<super::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Destination)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::IDispatch>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn LookupId(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).LookupId)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    pub unsafe fn IsAuthenticated2(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).IsAuthenticated2)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn IsFirstInTransaction2(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).IsFirstInTransaction2)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn IsLastInTransaction2(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).IsLastInTransaction2)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn AttachCurrentSecurityContext2(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AttachCurrentSecurityContext2)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SoapEnvelope(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).SoapEnvelope)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CompoundMessage(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).CompoundMessage)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSoapHeader<'a, P0>(&self, bstrsoapheader: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).SetSoapHeader)(::windows::core::Interface::as_raw(self), bstrsoapheader.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSoapBody<'a, P0>(&self, bstrsoapbody: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).SetSoapBody)(::windows::core::Interface::as_raw(self), bstrsoapbody.into().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMSMQMessage3> for ::windows::core::IUnknown {
    fn from(value: IMSMQMessage3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IMSMQMessage3> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IMSMQMessage3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMSMQMessage3> for ::windows::core::IUnknown {
    fn from(value: &IMSMQMessage3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMSMQMessage3> for super::Com::IDispatch {
    fn from(value: IMSMQMessage3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IMSMQMessage3> for &'a super::Com::IDispatch {
    fn from(value: &'a IMSMQMessage3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMSMQMessage3> for super::Com::IDispatch {
    fn from(value: &IMSMQMessage3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IMSMQMessage3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IMSMQMessage3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IMSMQMessage3 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IMSMQMessage3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMSMQMessage3").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IMSMQMessage3 {
    type Vtable = IMSMQMessage3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeba96b1a_2168_11d3_898c_00e02c074f6b);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQMessage3_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Class: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plclass: *mut i32) -> ::windows::core::HRESULT,
    pub PrivLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plprivlevel: *mut i32) -> ::windows::core::HRESULT,
    pub SetPrivLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lprivlevel: i32) -> ::windows::core::HRESULT,
    pub AuthLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plauthlevel: *mut i32) -> ::windows::core::HRESULT,
    pub SetAuthLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lauthlevel: i32) -> ::windows::core::HRESULT,
    pub IsAuthenticated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pisauthenticated: *mut i16) -> ::windows::core::HRESULT,
    pub Delivery: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pldelivery: *mut i32) -> ::windows::core::HRESULT,
    pub SetDelivery: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ldelivery: i32) -> ::windows::core::HRESULT,
    pub Trace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pltrace: *mut i32) -> ::windows::core::HRESULT,
    pub SetTrace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ltrace: i32) -> ::windows::core::HRESULT,
    pub Priority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plpriority: *mut i32) -> ::windows::core::HRESULT,
    pub SetPriority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpriority: i32) -> ::windows::core::HRESULT,
    pub Journal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pljournal: *mut i32) -> ::windows::core::HRESULT,
    pub SetJournal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ljournal: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub ResponseQueueInfo_v1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppqinforesponse: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ResponseQueueInfo_v1: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub putref_ResponseQueueInfo_v1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pqinforesponse: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    putref_ResponseQueueInfo_v1: usize,
    pub AppSpecific: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plappspecific: *mut i32) -> ::windows::core::HRESULT,
    pub SetAppSpecific: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lappspecific: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SourceMachineGuid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrguidsrcmachine: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SourceMachineGuid: usize,
    pub BodyLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcbbody: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Body: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarbody: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Body: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetBody: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varbody: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetBody: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub AdminQueueInfo_v1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppqinfoadmin: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AdminQueueInfo_v1: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub putref_AdminQueueInfo_v1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pqinfoadmin: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    putref_AdminQueueInfo_v1: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarmsgid: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Id: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub CorrelationId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarmsgid: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    CorrelationId: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetCorrelationId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varmsgid: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetCorrelationId: usize,
    pub Ack: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plack: *mut i32) -> ::windows::core::HRESULT,
    pub SetAck: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lack: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Label: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrlabel: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Label: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetLabel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrlabel: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetLabel: usize,
    pub MaxTimeToReachQueue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plmaxtimetoreachqueue: *mut i32) -> ::windows::core::HRESULT,
    pub SetMaxTimeToReachQueue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lmaxtimetoreachqueue: i32) -> ::windows::core::HRESULT,
    pub MaxTimeToReceive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plmaxtimetoreceive: *mut i32) -> ::windows::core::HRESULT,
    pub SetMaxTimeToReceive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lmaxtimetoreceive: i32) -> ::windows::core::HRESULT,
    pub HashAlgorithm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plhashalg: *mut i32) -> ::windows::core::HRESULT,
    pub SetHashAlgorithm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lhashalg: i32) -> ::windows::core::HRESULT,
    pub EncryptAlgorithm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plencryptalg: *mut i32) -> ::windows::core::HRESULT,
    pub SetEncryptAlgorithm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lencryptalg: i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SentTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarsenttime: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SentTime: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub ArrivedTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plarrivedtime: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    ArrivedTime: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub DestinationQueueInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppqinfodest: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    DestinationQueueInfo: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SenderCertificate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarsendercert: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SenderCertificate: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetSenderCertificate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varsendercert: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetSenderCertificate: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SenderId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarsenderid: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SenderId: usize,
    pub SenderIdType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plsenderidtype: *mut i32) -> ::windows::core::HRESULT,
    pub SetSenderIdType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lsenderidtype: i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Send: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, destinationqueue: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Send: usize,
    pub AttachCurrentSecurityContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SenderVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plsenderversion: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Extension: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarextension: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Extension: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetExtension: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varextension: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetExtension: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ConnectorTypeGuid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrguidconnectortype: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ConnectorTypeGuid: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetConnectorTypeGuid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrguidconnectortype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetConnectorTypeGuid: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub TransactionStatusQueueInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppqinfoxactstatus: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    TransactionStatusQueueInfo: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DestinationSymmetricKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvardestsymmkey: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DestinationSymmetricKey: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetDestinationSymmetricKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vardestsymmkey: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetDestinationSymmetricKey: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Signature: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarsignature: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Signature: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetSignature: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varsignature: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetSignature: usize,
    pub AuthenticationProviderType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plauthprovtype: *mut i32) -> ::windows::core::HRESULT,
    pub SetAuthenticationProviderType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lauthprovtype: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub AuthenticationProviderName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrauthprovname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AuthenticationProviderName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetAuthenticationProviderName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrauthprovname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetAuthenticationProviderName: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetSenderId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varsenderid: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetSenderId: usize,
    pub MsgClass: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plmsgclass: *mut i32) -> ::windows::core::HRESULT,
    pub SetMsgClass: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lmsgclass: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcolproperties: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Properties: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub TransactionId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarxactid: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    TransactionId: usize,
    pub IsFirstInTransaction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pisfirstinxact: *mut i16) -> ::windows::core::HRESULT,
    pub IsLastInTransaction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pislastinxact: *mut i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub ResponseQueueInfo_v2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppqinforesponse: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ResponseQueueInfo_v2: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub putref_ResponseQueueInfo_v2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pqinforesponse: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    putref_ResponseQueueInfo_v2: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub AdminQueueInfo_v2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppqinfoadmin: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AdminQueueInfo_v2: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub putref_AdminQueueInfo_v2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pqinfoadmin: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    putref_AdminQueueInfo_v2: usize,
    pub ReceivedAuthenticationLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psreceivedauthenticationlevel: *mut i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub ResponseQueueInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppqinforesponse: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ResponseQueueInfo: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub putref_ResponseQueueInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pqinforesponse: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    putref_ResponseQueueInfo: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub AdminQueueInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppqinfoadmin: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AdminQueueInfo: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub putref_AdminQueueInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pqinfoadmin: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    putref_AdminQueueInfo: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ResponseDestination: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdestresponse: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ResponseDestination: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub putref_ResponseDestination: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdestresponse: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    putref_ResponseDestination: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Destination: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdestdestination: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Destination: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub LookupId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarlookupid: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    LookupId: usize,
    pub IsAuthenticated2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pisauthenticated: *mut i16) -> ::windows::core::HRESULT,
    pub IsFirstInTransaction2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pisfirstinxact: *mut i16) -> ::windows::core::HRESULT,
    pub IsLastInTransaction2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pislastinxact: *mut i16) -> ::windows::core::HRESULT,
    pub AttachCurrentSecurityContext2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SoapEnvelope: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrsoapenvelope: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SoapEnvelope: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub CompoundMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarcompoundmessage: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    CompoundMessage: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetSoapHeader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrsoapheader: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetSoapHeader: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetSoapBody: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrsoapbody: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetSoapBody: usize,
}
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IMSMQMessage4(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IMSMQMessage4 {
    pub unsafe fn Class(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Class)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn PrivLevel(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).PrivLevel)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetPrivLevel(&self, lprivlevel: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetPrivLevel)(::windows::core::Interface::as_raw(self), lprivlevel).ok()
    }
    pub unsafe fn AuthLevel(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).AuthLevel)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetAuthLevel(&self, lauthlevel: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAuthLevel)(::windows::core::Interface::as_raw(self), lauthlevel).ok()
    }
    pub unsafe fn IsAuthenticated(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).IsAuthenticated)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn Delivery(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Delivery)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetDelivery(&self, ldelivery: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDelivery)(::windows::core::Interface::as_raw(self), ldelivery).ok()
    }
    pub unsafe fn Trace(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Trace)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetTrace(&self, ltrace: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetTrace)(::windows::core::Interface::as_raw(self), ltrace).ok()
    }
    pub unsafe fn Priority(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Priority)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetPriority(&self, lpriority: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetPriority)(::windows::core::Interface::as_raw(self), lpriority).ok()
    }
    pub unsafe fn Journal(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Journal)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetJournal(&self, ljournal: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetJournal)(::windows::core::Interface::as_raw(self), ljournal).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ResponseQueueInfo_v1(&self) -> ::windows::core::Result<IMSMQQueueInfo> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).ResponseQueueInfo_v1)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQQueueInfo>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn putref_ResponseQueueInfo_v1<'a, P0>(&self, pqinforesponse: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IMSMQQueueInfo>>,
    {
        (::windows::core::Interface::vtable(self).putref_ResponseQueueInfo_v1)(::windows::core::Interface::as_raw(self), pqinforesponse.into().abi()).ok()
    }
    pub unsafe fn AppSpecific(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).AppSpecific)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetAppSpecific(&self, lappspecific: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAppSpecific)(::windows::core::Interface::as_raw(self), lappspecific).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SourceMachineGuid(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).SourceMachineGuid)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn BodyLength(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).BodyLength)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Body(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Body)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetBody<'a, P0>(&self, varbody: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::Com::VARIANT>>,
    {
        (::windows::core::Interface::vtable(self).SetBody)(::windows::core::Interface::as_raw(self), varbody.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AdminQueueInfo_v1(&self) -> ::windows::core::Result<IMSMQQueueInfo> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).AdminQueueInfo_v1)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQQueueInfo>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn putref_AdminQueueInfo_v1<'a, P0>(&self, pqinfoadmin: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IMSMQQueueInfo>>,
    {
        (::windows::core::Interface::vtable(self).putref_AdminQueueInfo_v1)(::windows::core::Interface::as_raw(self), pqinfoadmin.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Id(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Id)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CorrelationId(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).CorrelationId)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetCorrelationId<'a, P0>(&self, varmsgid: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::Com::VARIANT>>,
    {
        (::windows::core::Interface::vtable(self).SetCorrelationId)(::windows::core::Interface::as_raw(self), varmsgid.into().abi()).ok()
    }
    pub unsafe fn Ack(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Ack)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetAck(&self, lack: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAck)(::windows::core::Interface::as_raw(self), lack).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Label(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Label)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLabel<'a, P0>(&self, bstrlabel: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).SetLabel)(::windows::core::Interface::as_raw(self), bstrlabel.into().abi()).ok()
    }
    pub unsafe fn MaxTimeToReachQueue(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).MaxTimeToReachQueue)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetMaxTimeToReachQueue(&self, lmaxtimetoreachqueue: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetMaxTimeToReachQueue)(::windows::core::Interface::as_raw(self), lmaxtimetoreachqueue).ok()
    }
    pub unsafe fn MaxTimeToReceive(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).MaxTimeToReceive)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetMaxTimeToReceive(&self, lmaxtimetoreceive: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetMaxTimeToReceive)(::windows::core::Interface::as_raw(self), lmaxtimetoreceive).ok()
    }
    pub unsafe fn HashAlgorithm(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).HashAlgorithm)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetHashAlgorithm(&self, lhashalg: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetHashAlgorithm)(::windows::core::Interface::as_raw(self), lhashalg).ok()
    }
    pub unsafe fn EncryptAlgorithm(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).EncryptAlgorithm)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetEncryptAlgorithm(&self, lencryptalg: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetEncryptAlgorithm)(::windows::core::Interface::as_raw(self), lencryptalg).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SentTime(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).SentTime)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ArrivedTime(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).ArrivedTime)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn DestinationQueueInfo(&self) -> ::windows::core::Result<IMSMQQueueInfo4> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).DestinationQueueInfo)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQQueueInfo4>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SenderCertificate(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).SenderCertificate)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetSenderCertificate<'a, P0>(&self, varsendercert: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::Com::VARIANT>>,
    {
        (::windows::core::Interface::vtable(self).SetSenderCertificate)(::windows::core::Interface::as_raw(self), varsendercert.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SenderId(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).SenderId)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    pub unsafe fn SenderIdType(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).SenderIdType)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetSenderIdType(&self, lsenderidtype: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSenderIdType)(::windows::core::Interface::as_raw(self), lsenderidtype).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Send<'a, P0>(&self, destinationqueue: P0, transaction: *const super::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::Com::IDispatch>>,
    {
        (::windows::core::Interface::vtable(self).Send)(::windows::core::Interface::as_raw(self), destinationqueue.into().abi(), ::core::mem::transmute(transaction)).ok()
    }
    pub unsafe fn AttachCurrentSecurityContext(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AttachCurrentSecurityContext)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SenderVersion(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).SenderVersion)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Extension(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Extension)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetExtension<'a, P0>(&self, varextension: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::Com::VARIANT>>,
    {
        (::windows::core::Interface::vtable(self).SetExtension)(::windows::core::Interface::as_raw(self), varextension.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ConnectorTypeGuid(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).ConnectorTypeGuid)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetConnectorTypeGuid<'a, P0>(&self, bstrguidconnectortype: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).SetConnectorTypeGuid)(::windows::core::Interface::as_raw(self), bstrguidconnectortype.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn TransactionStatusQueueInfo(&self) -> ::windows::core::Result<IMSMQQueueInfo4> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).TransactionStatusQueueInfo)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQQueueInfo4>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DestinationSymmetricKey(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).DestinationSymmetricKey)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetDestinationSymmetricKey<'a, P0>(&self, vardestsymmkey: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::Com::VARIANT>>,
    {
        (::windows::core::Interface::vtable(self).SetDestinationSymmetricKey)(::windows::core::Interface::as_raw(self), vardestsymmkey.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Signature(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Signature)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetSignature<'a, P0>(&self, varsignature: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::Com::VARIANT>>,
    {
        (::windows::core::Interface::vtable(self).SetSignature)(::windows::core::Interface::as_raw(self), varsignature.into().abi()).ok()
    }
    pub unsafe fn AuthenticationProviderType(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).AuthenticationProviderType)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetAuthenticationProviderType(&self, lauthprovtype: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAuthenticationProviderType)(::windows::core::Interface::as_raw(self), lauthprovtype).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AuthenticationProviderName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).AuthenticationProviderName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAuthenticationProviderName<'a, P0>(&self, bstrauthprovname: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).SetAuthenticationProviderName)(::windows::core::Interface::as_raw(self), bstrauthprovname.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetSenderId<'a, P0>(&self, varsenderid: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::Com::VARIANT>>,
    {
        (::windows::core::Interface::vtable(self).SetSenderId)(::windows::core::Interface::as_raw(self), varsenderid.into().abi()).ok()
    }
    pub unsafe fn MsgClass(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).MsgClass)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetMsgClass(&self, lmsgclass: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetMsgClass)(::windows::core::Interface::as_raw(self), lmsgclass).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Properties(&self) -> ::windows::core::Result<super::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Properties)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::IDispatch>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn TransactionId(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).TransactionId)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    pub unsafe fn IsFirstInTransaction(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).IsFirstInTransaction)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn IsLastInTransaction(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).IsLastInTransaction)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ResponseQueueInfo_v2(&self) -> ::windows::core::Result<IMSMQQueueInfo2> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).ResponseQueueInfo_v2)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQQueueInfo2>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn putref_ResponseQueueInfo_v2<'a, P0>(&self, pqinforesponse: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IMSMQQueueInfo2>>,
    {
        (::windows::core::Interface::vtable(self).putref_ResponseQueueInfo_v2)(::windows::core::Interface::as_raw(self), pqinforesponse.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AdminQueueInfo_v2(&self) -> ::windows::core::Result<IMSMQQueueInfo2> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).AdminQueueInfo_v2)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQQueueInfo2>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn putref_AdminQueueInfo_v2<'a, P0>(&self, pqinfoadmin: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IMSMQQueueInfo2>>,
    {
        (::windows::core::Interface::vtable(self).putref_AdminQueueInfo_v2)(::windows::core::Interface::as_raw(self), pqinfoadmin.into().abi()).ok()
    }
    pub unsafe fn ReceivedAuthenticationLevel(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).ReceivedAuthenticationLevel)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ResponseQueueInfo(&self) -> ::windows::core::Result<IMSMQQueueInfo4> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).ResponseQueueInfo)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQQueueInfo4>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn putref_ResponseQueueInfo<'a, P0>(&self, pqinforesponse: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IMSMQQueueInfo4>>,
    {
        (::windows::core::Interface::vtable(self).putref_ResponseQueueInfo)(::windows::core::Interface::as_raw(self), pqinforesponse.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AdminQueueInfo(&self) -> ::windows::core::Result<IMSMQQueueInfo4> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).AdminQueueInfo)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQQueueInfo4>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn putref_AdminQueueInfo<'a, P0>(&self, pqinfoadmin: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IMSMQQueueInfo4>>,
    {
        (::windows::core::Interface::vtable(self).putref_AdminQueueInfo)(::windows::core::Interface::as_raw(self), pqinfoadmin.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ResponseDestination(&self) -> ::windows::core::Result<super::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).ResponseDestination)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::IDispatch>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn putref_ResponseDestination<'a, P0>(&self, pdestresponse: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::Com::IDispatch>>,
    {
        (::windows::core::Interface::vtable(self).putref_ResponseDestination)(::windows::core::Interface::as_raw(self), pdestresponse.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Destination(&self) -> ::windows::core::Result<super::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Destination)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::IDispatch>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn LookupId(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).LookupId)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    pub unsafe fn IsAuthenticated2(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).IsAuthenticated2)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn IsFirstInTransaction2(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).IsFirstInTransaction2)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn IsLastInTransaction2(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).IsLastInTransaction2)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn AttachCurrentSecurityContext2(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AttachCurrentSecurityContext2)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SoapEnvelope(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).SoapEnvelope)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CompoundMessage(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).CompoundMessage)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSoapHeader<'a, P0>(&self, bstrsoapheader: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).SetSoapHeader)(::windows::core::Interface::as_raw(self), bstrsoapheader.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSoapBody<'a, P0>(&self, bstrsoapbody: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).SetSoapBody)(::windows::core::Interface::as_raw(self), bstrsoapbody.into().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMSMQMessage4> for ::windows::core::IUnknown {
    fn from(value: IMSMQMessage4) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IMSMQMessage4> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IMSMQMessage4) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMSMQMessage4> for ::windows::core::IUnknown {
    fn from(value: &IMSMQMessage4) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMSMQMessage4> for super::Com::IDispatch {
    fn from(value: IMSMQMessage4) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IMSMQMessage4> for &'a super::Com::IDispatch {
    fn from(value: &'a IMSMQMessage4) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMSMQMessage4> for super::Com::IDispatch {
    fn from(value: &IMSMQMessage4) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IMSMQMessage4 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IMSMQMessage4 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IMSMQMessage4 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IMSMQMessage4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMSMQMessage4").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IMSMQMessage4 {
    type Vtable = IMSMQMessage4_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeba96b23_2168_11d3_898c_00e02c074f6b);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQMessage4_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Class: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plclass: *mut i32) -> ::windows::core::HRESULT,
    pub PrivLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plprivlevel: *mut i32) -> ::windows::core::HRESULT,
    pub SetPrivLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lprivlevel: i32) -> ::windows::core::HRESULT,
    pub AuthLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plauthlevel: *mut i32) -> ::windows::core::HRESULT,
    pub SetAuthLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lauthlevel: i32) -> ::windows::core::HRESULT,
    pub IsAuthenticated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pisauthenticated: *mut i16) -> ::windows::core::HRESULT,
    pub Delivery: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pldelivery: *mut i32) -> ::windows::core::HRESULT,
    pub SetDelivery: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ldelivery: i32) -> ::windows::core::HRESULT,
    pub Trace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pltrace: *mut i32) -> ::windows::core::HRESULT,
    pub SetTrace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ltrace: i32) -> ::windows::core::HRESULT,
    pub Priority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plpriority: *mut i32) -> ::windows::core::HRESULT,
    pub SetPriority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpriority: i32) -> ::windows::core::HRESULT,
    pub Journal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pljournal: *mut i32) -> ::windows::core::HRESULT,
    pub SetJournal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ljournal: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub ResponseQueueInfo_v1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppqinforesponse: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ResponseQueueInfo_v1: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub putref_ResponseQueueInfo_v1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pqinforesponse: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    putref_ResponseQueueInfo_v1: usize,
    pub AppSpecific: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plappspecific: *mut i32) -> ::windows::core::HRESULT,
    pub SetAppSpecific: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lappspecific: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SourceMachineGuid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrguidsrcmachine: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SourceMachineGuid: usize,
    pub BodyLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcbbody: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Body: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarbody: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Body: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetBody: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varbody: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetBody: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub AdminQueueInfo_v1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppqinfoadmin: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AdminQueueInfo_v1: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub putref_AdminQueueInfo_v1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pqinfoadmin: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    putref_AdminQueueInfo_v1: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarmsgid: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Id: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub CorrelationId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarmsgid: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    CorrelationId: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetCorrelationId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varmsgid: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetCorrelationId: usize,
    pub Ack: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plack: *mut i32) -> ::windows::core::HRESULT,
    pub SetAck: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lack: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Label: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrlabel: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Label: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetLabel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrlabel: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetLabel: usize,
    pub MaxTimeToReachQueue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plmaxtimetoreachqueue: *mut i32) -> ::windows::core::HRESULT,
    pub SetMaxTimeToReachQueue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lmaxtimetoreachqueue: i32) -> ::windows::core::HRESULT,
    pub MaxTimeToReceive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plmaxtimetoreceive: *mut i32) -> ::windows::core::HRESULT,
    pub SetMaxTimeToReceive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lmaxtimetoreceive: i32) -> ::windows::core::HRESULT,
    pub HashAlgorithm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plhashalg: *mut i32) -> ::windows::core::HRESULT,
    pub SetHashAlgorithm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lhashalg: i32) -> ::windows::core::HRESULT,
    pub EncryptAlgorithm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plencryptalg: *mut i32) -> ::windows::core::HRESULT,
    pub SetEncryptAlgorithm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lencryptalg: i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SentTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarsenttime: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SentTime: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub ArrivedTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plarrivedtime: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    ArrivedTime: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub DestinationQueueInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppqinfodest: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    DestinationQueueInfo: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SenderCertificate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarsendercert: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SenderCertificate: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetSenderCertificate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varsendercert: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetSenderCertificate: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SenderId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarsenderid: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SenderId: usize,
    pub SenderIdType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plsenderidtype: *mut i32) -> ::windows::core::HRESULT,
    pub SetSenderIdType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lsenderidtype: i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Send: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, destinationqueue: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Send: usize,
    pub AttachCurrentSecurityContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SenderVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plsenderversion: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Extension: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarextension: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Extension: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetExtension: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varextension: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetExtension: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ConnectorTypeGuid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrguidconnectortype: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ConnectorTypeGuid: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetConnectorTypeGuid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrguidconnectortype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetConnectorTypeGuid: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub TransactionStatusQueueInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppqinfoxactstatus: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    TransactionStatusQueueInfo: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DestinationSymmetricKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvardestsymmkey: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DestinationSymmetricKey: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetDestinationSymmetricKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vardestsymmkey: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetDestinationSymmetricKey: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Signature: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarsignature: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Signature: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetSignature: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varsignature: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetSignature: usize,
    pub AuthenticationProviderType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plauthprovtype: *mut i32) -> ::windows::core::HRESULT,
    pub SetAuthenticationProviderType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lauthprovtype: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub AuthenticationProviderName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrauthprovname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AuthenticationProviderName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetAuthenticationProviderName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrauthprovname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetAuthenticationProviderName: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetSenderId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varsenderid: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetSenderId: usize,
    pub MsgClass: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plmsgclass: *mut i32) -> ::windows::core::HRESULT,
    pub SetMsgClass: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lmsgclass: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcolproperties: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Properties: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub TransactionId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarxactid: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    TransactionId: usize,
    pub IsFirstInTransaction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pisfirstinxact: *mut i16) -> ::windows::core::HRESULT,
    pub IsLastInTransaction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pislastinxact: *mut i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub ResponseQueueInfo_v2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppqinforesponse: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ResponseQueueInfo_v2: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub putref_ResponseQueueInfo_v2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pqinforesponse: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    putref_ResponseQueueInfo_v2: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub AdminQueueInfo_v2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppqinfoadmin: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AdminQueueInfo_v2: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub putref_AdminQueueInfo_v2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pqinfoadmin: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    putref_AdminQueueInfo_v2: usize,
    pub ReceivedAuthenticationLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psreceivedauthenticationlevel: *mut i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub ResponseQueueInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppqinforesponse: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ResponseQueueInfo: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub putref_ResponseQueueInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pqinforesponse: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    putref_ResponseQueueInfo: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub AdminQueueInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppqinfoadmin: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AdminQueueInfo: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub putref_AdminQueueInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pqinfoadmin: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    putref_AdminQueueInfo: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ResponseDestination: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdestresponse: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ResponseDestination: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub putref_ResponseDestination: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdestresponse: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    putref_ResponseDestination: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Destination: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdestdestination: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Destination: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub LookupId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarlookupid: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    LookupId: usize,
    pub IsAuthenticated2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pisauthenticated: *mut i16) -> ::windows::core::HRESULT,
    pub IsFirstInTransaction2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pisfirstinxact: *mut i16) -> ::windows::core::HRESULT,
    pub IsLastInTransaction2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pislastinxact: *mut i16) -> ::windows::core::HRESULT,
    pub AttachCurrentSecurityContext2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SoapEnvelope: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrsoapenvelope: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SoapEnvelope: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub CompoundMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarcompoundmessage: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    CompoundMessage: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetSoapHeader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrsoapheader: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetSoapHeader: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetSoapBody: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrsoapbody: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetSoapBody: usize,
}
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IMSMQOutgoingQueueManagement(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IMSMQOutgoingQueueManagement {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Init(&self, machine: *const super::Com::VARIANT, pathname: *const super::Com::VARIANT, formatname: *const super::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Init)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(machine), ::core::mem::transmute(pathname), ::core::mem::transmute(formatname)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FormatName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.FormatName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Machine(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.Machine)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn MessageCount(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.MessageCount)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn ForeignStatus(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.ForeignStatus)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn QueueType(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.QueueType)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn IsLocal(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.IsLocal)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn TransactionalStatus(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.TransactionalStatus)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn BytesInQueue(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.BytesInQueue)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    pub unsafe fn State(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).State)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn NextHops(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).NextHops)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EodGetSendInfo(&self) -> ::windows::core::Result<IMSMQCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).EodGetSendInfo)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQCollection>(result__)
    }
    pub unsafe fn Resume(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Resume)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Pause(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Pause)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn EodResend(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EodResend)(::windows::core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMSMQOutgoingQueueManagement> for ::windows::core::IUnknown {
    fn from(value: IMSMQOutgoingQueueManagement) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IMSMQOutgoingQueueManagement> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IMSMQOutgoingQueueManagement) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMSMQOutgoingQueueManagement> for ::windows::core::IUnknown {
    fn from(value: &IMSMQOutgoingQueueManagement) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMSMQOutgoingQueueManagement> for super::Com::IDispatch {
    fn from(value: IMSMQOutgoingQueueManagement) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IMSMQOutgoingQueueManagement> for &'a super::Com::IDispatch {
    fn from(value: &'a IMSMQOutgoingQueueManagement) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMSMQOutgoingQueueManagement> for super::Com::IDispatch {
    fn from(value: &IMSMQOutgoingQueueManagement) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMSMQOutgoingQueueManagement> for IMSMQManagement {
    fn from(value: IMSMQOutgoingQueueManagement) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IMSMQOutgoingQueueManagement> for &'a IMSMQManagement {
    fn from(value: &'a IMSMQOutgoingQueueManagement) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMSMQOutgoingQueueManagement> for IMSMQManagement {
    fn from(value: &IMSMQOutgoingQueueManagement) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IMSMQOutgoingQueueManagement {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IMSMQOutgoingQueueManagement {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IMSMQOutgoingQueueManagement {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IMSMQOutgoingQueueManagement {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMSMQOutgoingQueueManagement").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IMSMQOutgoingQueueManagement {
    type Vtable = IMSMQOutgoingQueueManagement_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x64c478fb_f9b0_4695_8a7f_439ac94326d3);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQOutgoingQueueManagement_Vtbl {
    pub base__: IMSMQManagement_Vtbl,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plstate: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub NextHops: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvnexthops: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    NextHops: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub EodGetSendInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcollection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EodGetSendInfo: usize,
    pub Resume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Pause: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub EodResend: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IMSMQPrivateDestination(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IMSMQPrivateDestination {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Handle(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Handle)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetHandle<'a, P0>(&self, varhandle: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::Com::VARIANT>>,
    {
        (::windows::core::Interface::vtable(self).SetHandle)(::windows::core::Interface::as_raw(self), varhandle.into().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMSMQPrivateDestination> for ::windows::core::IUnknown {
    fn from(value: IMSMQPrivateDestination) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IMSMQPrivateDestination> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IMSMQPrivateDestination) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMSMQPrivateDestination> for ::windows::core::IUnknown {
    fn from(value: &IMSMQPrivateDestination) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMSMQPrivateDestination> for super::Com::IDispatch {
    fn from(value: IMSMQPrivateDestination) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IMSMQPrivateDestination> for &'a super::Com::IDispatch {
    fn from(value: &'a IMSMQPrivateDestination) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMSMQPrivateDestination> for super::Com::IDispatch {
    fn from(value: &IMSMQPrivateDestination) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IMSMQPrivateDestination {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IMSMQPrivateDestination {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IMSMQPrivateDestination {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IMSMQPrivateDestination {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMSMQPrivateDestination").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IMSMQPrivateDestination {
    type Vtable = IMSMQPrivateDestination_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeba96b17_2168_11d3_898c_00e02c074f6b);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQPrivateDestination_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Handle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarhandle: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Handle: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetHandle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varhandle: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetHandle: usize,
}
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IMSMQPrivateEvent(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IMSMQPrivateEvent {
    pub unsafe fn Hwnd(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Hwnd)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn FireArrivedEvent<'a, P0>(&self, pq: P0, msgcursor: i32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IMSMQQueue>>,
    {
        (::windows::core::Interface::vtable(self).FireArrivedEvent)(::windows::core::Interface::as_raw(self), pq.into().abi(), msgcursor).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn FireArrivedErrorEvent<'a, P0>(&self, pq: P0, hrstatus: ::windows::core::HRESULT, msgcursor: i32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IMSMQQueue>>,
    {
        (::windows::core::Interface::vtable(self).FireArrivedErrorEvent)(::windows::core::Interface::as_raw(self), pq.into().abi(), hrstatus, msgcursor).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMSMQPrivateEvent> for ::windows::core::IUnknown {
    fn from(value: IMSMQPrivateEvent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IMSMQPrivateEvent> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IMSMQPrivateEvent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMSMQPrivateEvent> for ::windows::core::IUnknown {
    fn from(value: &IMSMQPrivateEvent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMSMQPrivateEvent> for super::Com::IDispatch {
    fn from(value: IMSMQPrivateEvent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IMSMQPrivateEvent> for &'a super::Com::IDispatch {
    fn from(value: &'a IMSMQPrivateEvent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMSMQPrivateEvent> for super::Com::IDispatch {
    fn from(value: &IMSMQPrivateEvent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IMSMQPrivateEvent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IMSMQPrivateEvent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IMSMQPrivateEvent {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IMSMQPrivateEvent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMSMQPrivateEvent").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IMSMQPrivateEvent {
    type Vtable = IMSMQPrivateEvent_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd7ab3341_c9d3_11d1_bb47_0080c7c5a2c0);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQPrivateEvent_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Hwnd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phwnd: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub FireArrivedEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pq: *mut ::core::ffi::c_void, msgcursor: i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    FireArrivedEvent: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub FireArrivedErrorEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pq: *mut ::core::ffi::c_void, hrstatus: ::windows::core::HRESULT, msgcursor: i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    FireArrivedErrorEvent: usize,
}
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IMSMQQuery(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IMSMQQuery {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn LookupQueue(&self, queueguid: *const super::Com::VARIANT, servicetypeguid: *const super::Com::VARIANT, label: *const super::Com::VARIANT, createtime: *const super::Com::VARIANT, modifytime: *const super::Com::VARIANT, relservicetype: *const super::Com::VARIANT, rellabel: *const super::Com::VARIANT, relcreatetime: *const super::Com::VARIANT, relmodifytime: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQQueueInfos> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).LookupQueue)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(queueguid), ::core::mem::transmute(servicetypeguid), ::core::mem::transmute(label), ::core::mem::transmute(createtime), ::core::mem::transmute(modifytime), ::core::mem::transmute(relservicetype), ::core::mem::transmute(rellabel), ::core::mem::transmute(relcreatetime), ::core::mem::transmute(relmodifytime), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQQueueInfos>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMSMQQuery> for ::windows::core::IUnknown {
    fn from(value: IMSMQQuery) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IMSMQQuery> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IMSMQQuery) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMSMQQuery> for ::windows::core::IUnknown {
    fn from(value: &IMSMQQuery) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMSMQQuery> for super::Com::IDispatch {
    fn from(value: IMSMQQuery) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IMSMQQuery> for &'a super::Com::IDispatch {
    fn from(value: &'a IMSMQQuery) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMSMQQuery> for super::Com::IDispatch {
    fn from(value: &IMSMQQuery) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IMSMQQuery {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IMSMQQuery {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IMSMQQuery {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IMSMQQuery {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMSMQQuery").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IMSMQQuery {
    type Vtable = IMSMQQuery_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd7d6e072_dccd_11d0_aa4b_0060970debae);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQQuery_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub LookupQueue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, queueguid: *const super::Com::VARIANT, servicetypeguid: *const super::Com::VARIANT, label: *const super::Com::VARIANT, createtime: *const super::Com::VARIANT, modifytime: *const super::Com::VARIANT, relservicetype: *const super::Com::VARIANT, rellabel: *const super::Com::VARIANT, relcreatetime: *const super::Com::VARIANT, relmodifytime: *const super::Com::VARIANT, ppqinfos: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    LookupQueue: usize,
}
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IMSMQQuery2(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IMSMQQuery2 {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn LookupQueue(&self, queueguid: *const super::Com::VARIANT, servicetypeguid: *const super::Com::VARIANT, label: *const super::Com::VARIANT, createtime: *const super::Com::VARIANT, modifytime: *const super::Com::VARIANT, relservicetype: *const super::Com::VARIANT, rellabel: *const super::Com::VARIANT, relcreatetime: *const super::Com::VARIANT, relmodifytime: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQQueueInfos2> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).LookupQueue)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(queueguid), ::core::mem::transmute(servicetypeguid), ::core::mem::transmute(label), ::core::mem::transmute(createtime), ::core::mem::transmute(modifytime), ::core::mem::transmute(relservicetype), ::core::mem::transmute(rellabel), ::core::mem::transmute(relcreatetime), ::core::mem::transmute(relmodifytime), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQQueueInfos2>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Properties(&self) -> ::windows::core::Result<super::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Properties)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::IDispatch>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMSMQQuery2> for ::windows::core::IUnknown {
    fn from(value: IMSMQQuery2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IMSMQQuery2> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IMSMQQuery2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMSMQQuery2> for ::windows::core::IUnknown {
    fn from(value: &IMSMQQuery2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMSMQQuery2> for super::Com::IDispatch {
    fn from(value: IMSMQQuery2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IMSMQQuery2> for &'a super::Com::IDispatch {
    fn from(value: &'a IMSMQQuery2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMSMQQuery2> for super::Com::IDispatch {
    fn from(value: &IMSMQQuery2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IMSMQQuery2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IMSMQQuery2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IMSMQQuery2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IMSMQQuery2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMSMQQuery2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IMSMQQuery2 {
    type Vtable = IMSMQQuery2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeba96b0e_2168_11d3_898c_00e02c074f6b);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQQuery2_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub LookupQueue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, queueguid: *const super::Com::VARIANT, servicetypeguid: *const super::Com::VARIANT, label: *const super::Com::VARIANT, createtime: *const super::Com::VARIANT, modifytime: *const super::Com::VARIANT, relservicetype: *const super::Com::VARIANT, rellabel: *const super::Com::VARIANT, relcreatetime: *const super::Com::VARIANT, relmodifytime: *const super::Com::VARIANT, ppqinfos: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    LookupQueue: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcolproperties: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Properties: usize,
}
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IMSMQQuery3(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IMSMQQuery3 {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn LookupQueue_v2(&self, queueguid: *const super::Com::VARIANT, servicetypeguid: *const super::Com::VARIANT, label: *const super::Com::VARIANT, createtime: *const super::Com::VARIANT, modifytime: *const super::Com::VARIANT, relservicetype: *const super::Com::VARIANT, rellabel: *const super::Com::VARIANT, relcreatetime: *const super::Com::VARIANT, relmodifytime: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQQueueInfos3> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).LookupQueue_v2)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(queueguid), ::core::mem::transmute(servicetypeguid), ::core::mem::transmute(label), ::core::mem::transmute(createtime), ::core::mem::transmute(modifytime), ::core::mem::transmute(relservicetype), ::core::mem::transmute(rellabel), ::core::mem::transmute(relcreatetime), ::core::mem::transmute(relmodifytime), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQQueueInfos3>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Properties(&self) -> ::windows::core::Result<super::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Properties)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::IDispatch>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn LookupQueue(&self, queueguid: *const super::Com::VARIANT, servicetypeguid: *const super::Com::VARIANT, label: *const super::Com::VARIANT, createtime: *const super::Com::VARIANT, modifytime: *const super::Com::VARIANT, relservicetype: *const super::Com::VARIANT, rellabel: *const super::Com::VARIANT, relcreatetime: *const super::Com::VARIANT, relmodifytime: *const super::Com::VARIANT, multicastaddress: *const super::Com::VARIANT, relmulticastaddress: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQQueueInfos3> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).LookupQueue)(
            ::windows::core::Interface::as_raw(self),
            ::core::mem::transmute(queueguid),
            ::core::mem::transmute(servicetypeguid),
            ::core::mem::transmute(label),
            ::core::mem::transmute(createtime),
            ::core::mem::transmute(modifytime),
            ::core::mem::transmute(relservicetype),
            ::core::mem::transmute(rellabel),
            ::core::mem::transmute(relcreatetime),
            ::core::mem::transmute(relmodifytime),
            ::core::mem::transmute(multicastaddress),
            ::core::mem::transmute(relmulticastaddress),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<IMSMQQueueInfos3>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMSMQQuery3> for ::windows::core::IUnknown {
    fn from(value: IMSMQQuery3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IMSMQQuery3> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IMSMQQuery3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMSMQQuery3> for ::windows::core::IUnknown {
    fn from(value: &IMSMQQuery3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMSMQQuery3> for super::Com::IDispatch {
    fn from(value: IMSMQQuery3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IMSMQQuery3> for &'a super::Com::IDispatch {
    fn from(value: &'a IMSMQQuery3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMSMQQuery3> for super::Com::IDispatch {
    fn from(value: &IMSMQQuery3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IMSMQQuery3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IMSMQQuery3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IMSMQQuery3 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IMSMQQuery3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMSMQQuery3").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IMSMQQuery3 {
    type Vtable = IMSMQQuery3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeba96b19_2168_11d3_898c_00e02c074f6b);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQQuery3_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub LookupQueue_v2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, queueguid: *const super::Com::VARIANT, servicetypeguid: *const super::Com::VARIANT, label: *const super::Com::VARIANT, createtime: *const super::Com::VARIANT, modifytime: *const super::Com::VARIANT, relservicetype: *const super::Com::VARIANT, rellabel: *const super::Com::VARIANT, relcreatetime: *const super::Com::VARIANT, relmodifytime: *const super::Com::VARIANT, ppqinfos: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    LookupQueue_v2: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcolproperties: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Properties: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub LookupQueue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, queueguid: *const super::Com::VARIANT, servicetypeguid: *const super::Com::VARIANT, label: *const super::Com::VARIANT, createtime: *const super::Com::VARIANT, modifytime: *const super::Com::VARIANT, relservicetype: *const super::Com::VARIANT, rellabel: *const super::Com::VARIANT, relcreatetime: *const super::Com::VARIANT, relmodifytime: *const super::Com::VARIANT, multicastaddress: *const super::Com::VARIANT, relmulticastaddress: *const super::Com::VARIANT, ppqinfos: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    LookupQueue: usize,
}
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IMSMQQuery4(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IMSMQQuery4 {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn LookupQueue_v2(&self, queueguid: *const super::Com::VARIANT, servicetypeguid: *const super::Com::VARIANT, label: *const super::Com::VARIANT, createtime: *const super::Com::VARIANT, modifytime: *const super::Com::VARIANT, relservicetype: *const super::Com::VARIANT, rellabel: *const super::Com::VARIANT, relcreatetime: *const super::Com::VARIANT, relmodifytime: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQQueueInfos4> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).LookupQueue_v2)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(queueguid), ::core::mem::transmute(servicetypeguid), ::core::mem::transmute(label), ::core::mem::transmute(createtime), ::core::mem::transmute(modifytime), ::core::mem::transmute(relservicetype), ::core::mem::transmute(rellabel), ::core::mem::transmute(relcreatetime), ::core::mem::transmute(relmodifytime), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQQueueInfos4>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Properties(&self) -> ::windows::core::Result<super::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Properties)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::IDispatch>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn LookupQueue(&self, queueguid: *const super::Com::VARIANT, servicetypeguid: *const super::Com::VARIANT, label: *const super::Com::VARIANT, createtime: *const super::Com::VARIANT, modifytime: *const super::Com::VARIANT, relservicetype: *const super::Com::VARIANT, rellabel: *const super::Com::VARIANT, relcreatetime: *const super::Com::VARIANT, relmodifytime: *const super::Com::VARIANT, multicastaddress: *const super::Com::VARIANT, relmulticastaddress: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQQueueInfos4> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).LookupQueue)(
            ::windows::core::Interface::as_raw(self),
            ::core::mem::transmute(queueguid),
            ::core::mem::transmute(servicetypeguid),
            ::core::mem::transmute(label),
            ::core::mem::transmute(createtime),
            ::core::mem::transmute(modifytime),
            ::core::mem::transmute(relservicetype),
            ::core::mem::transmute(rellabel),
            ::core::mem::transmute(relcreatetime),
            ::core::mem::transmute(relmodifytime),
            ::core::mem::transmute(multicastaddress),
            ::core::mem::transmute(relmulticastaddress),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<IMSMQQueueInfos4>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMSMQQuery4> for ::windows::core::IUnknown {
    fn from(value: IMSMQQuery4) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IMSMQQuery4> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IMSMQQuery4) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMSMQQuery4> for ::windows::core::IUnknown {
    fn from(value: &IMSMQQuery4) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMSMQQuery4> for super::Com::IDispatch {
    fn from(value: IMSMQQuery4) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IMSMQQuery4> for &'a super::Com::IDispatch {
    fn from(value: &'a IMSMQQuery4) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMSMQQuery4> for super::Com::IDispatch {
    fn from(value: &IMSMQQuery4) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IMSMQQuery4 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IMSMQQuery4 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IMSMQQuery4 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IMSMQQuery4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMSMQQuery4").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IMSMQQuery4 {
    type Vtable = IMSMQQuery4_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeba96b24_2168_11d3_898c_00e02c074f6b);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQQuery4_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub LookupQueue_v2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, queueguid: *const super::Com::VARIANT, servicetypeguid: *const super::Com::VARIANT, label: *const super::Com::VARIANT, createtime: *const super::Com::VARIANT, modifytime: *const super::Com::VARIANT, relservicetype: *const super::Com::VARIANT, rellabel: *const super::Com::VARIANT, relcreatetime: *const super::Com::VARIANT, relmodifytime: *const super::Com::VARIANT, ppqinfos: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    LookupQueue_v2: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcolproperties: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Properties: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub LookupQueue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, queueguid: *const super::Com::VARIANT, servicetypeguid: *const super::Com::VARIANT, label: *const super::Com::VARIANT, createtime: *const super::Com::VARIANT, modifytime: *const super::Com::VARIANT, relservicetype: *const super::Com::VARIANT, rellabel: *const super::Com::VARIANT, relcreatetime: *const super::Com::VARIANT, relmodifytime: *const super::Com::VARIANT, multicastaddress: *const super::Com::VARIANT, relmulticastaddress: *const super::Com::VARIANT, ppqinfos: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    LookupQueue: usize,
}
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IMSMQQueue(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IMSMQQueue {
    pub unsafe fn Access(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Access)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn ShareMode(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).ShareMode)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn QueueInfo(&self) -> ::windows::core::Result<IMSMQQueueInfo> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).QueueInfo)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQQueueInfo>(result__)
    }
    pub unsafe fn Handle(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Handle)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn IsOpen(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).IsOpen)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Close)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Receive(&self, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Receive)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(transaction), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(receivetimeout), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQMessage>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Peek(&self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Peek)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(receivetimeout), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQMessage>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn EnableNotification<'a, P0>(&self, event: P0, cursor: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IMSMQEvent>>,
    {
        (::windows::core::Interface::vtable(self).EnableNotification)(::windows::core::Interface::as_raw(self), event.into().abi(), ::core::mem::transmute(cursor), ::core::mem::transmute(receivetimeout)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ReceiveCurrent(&self, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).ReceiveCurrent)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(transaction), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(receivetimeout), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQMessage>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PeekNext(&self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).PeekNext)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(receivetimeout), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQMessage>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PeekCurrent(&self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).PeekCurrent)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(receivetimeout), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQMessage>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMSMQQueue> for ::windows::core::IUnknown {
    fn from(value: IMSMQQueue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IMSMQQueue> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IMSMQQueue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMSMQQueue> for ::windows::core::IUnknown {
    fn from(value: &IMSMQQueue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMSMQQueue> for super::Com::IDispatch {
    fn from(value: IMSMQQueue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IMSMQQueue> for &'a super::Com::IDispatch {
    fn from(value: &'a IMSMQQueue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMSMQQueue> for super::Com::IDispatch {
    fn from(value: &IMSMQQueue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IMSMQQueue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IMSMQQueue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IMSMQQueue {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IMSMQQueue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMSMQQueue").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IMSMQQueue {
    type Vtable = IMSMQQueue_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd7d6e076_dccd_11d0_aa4b_0060970debae);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQQueue_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Access: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, placcess: *mut i32) -> ::windows::core::HRESULT,
    pub ShareMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plsharemode: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub QueueInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppqinfo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    QueueInfo: usize,
    pub Handle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plhandle: *mut i32) -> ::windows::core::HRESULT,
    pub IsOpen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pisopen: *mut i16) -> ::windows::core::HRESULT,
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Receive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Receive: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Peek: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Peek: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub EnableNotification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, event: *mut ::core::ffi::c_void, cursor: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    EnableNotification: usize,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub ReceiveCurrent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    ReceiveCurrent: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub PeekNext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    PeekNext: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub PeekCurrent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    PeekCurrent: usize,
}
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IMSMQQueue2(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IMSMQQueue2 {
    pub unsafe fn Access(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Access)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn ShareMode(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).ShareMode)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn QueueInfo(&self) -> ::windows::core::Result<IMSMQQueueInfo2> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).QueueInfo)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQQueueInfo2>(result__)
    }
    pub unsafe fn Handle(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Handle)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn IsOpen(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).IsOpen)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Close)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Receive_v1(&self, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Receive_v1)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(transaction), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(receivetimeout), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQMessage>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Peek_v1(&self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Peek_v1)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(receivetimeout), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQMessage>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn EnableNotification<'a, P0>(&self, event: P0, cursor: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IMSMQEvent2>>,
    {
        (::windows::core::Interface::vtable(self).EnableNotification)(::windows::core::Interface::as_raw(self), event.into().abi(), ::core::mem::transmute(cursor), ::core::mem::transmute(receivetimeout)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ReceiveCurrent_v1(&self, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).ReceiveCurrent_v1)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(transaction), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(receivetimeout), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQMessage>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PeekNext_v1(&self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).PeekNext_v1)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(receivetimeout), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQMessage>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PeekCurrent_v1(&self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).PeekCurrent_v1)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(receivetimeout), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQMessage>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Receive(&self, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage2> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Receive)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(transaction), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(receivetimeout), ::core::mem::transmute(wantconnectortype), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQMessage2>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Peek(&self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage2> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Peek)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(receivetimeout), ::core::mem::transmute(wantconnectortype), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQMessage2>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ReceiveCurrent(&self, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage2> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).ReceiveCurrent)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(transaction), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(receivetimeout), ::core::mem::transmute(wantconnectortype), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQMessage2>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PeekNext(&self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage2> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).PeekNext)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(receivetimeout), ::core::mem::transmute(wantconnectortype), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQMessage2>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PeekCurrent(&self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage2> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).PeekCurrent)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(receivetimeout), ::core::mem::transmute(wantconnectortype), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQMessage2>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Properties(&self) -> ::windows::core::Result<super::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Properties)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::IDispatch>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMSMQQueue2> for ::windows::core::IUnknown {
    fn from(value: IMSMQQueue2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IMSMQQueue2> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IMSMQQueue2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMSMQQueue2> for ::windows::core::IUnknown {
    fn from(value: &IMSMQQueue2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMSMQQueue2> for super::Com::IDispatch {
    fn from(value: IMSMQQueue2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IMSMQQueue2> for &'a super::Com::IDispatch {
    fn from(value: &'a IMSMQQueue2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMSMQQueue2> for super::Com::IDispatch {
    fn from(value: &IMSMQQueue2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IMSMQQueue2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IMSMQQueue2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IMSMQQueue2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IMSMQQueue2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMSMQQueue2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IMSMQQueue2 {
    type Vtable = IMSMQQueue2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xef0574e0_06d8_11d3_b100_00e02c074f6b);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQQueue2_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Access: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, placcess: *mut i32) -> ::windows::core::HRESULT,
    pub ShareMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plsharemode: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub QueueInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppqinfo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    QueueInfo: usize,
    pub Handle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plhandle: *mut i32) -> ::windows::core::HRESULT,
    pub IsOpen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pisopen: *mut i16) -> ::windows::core::HRESULT,
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Receive_v1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Receive_v1: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Peek_v1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Peek_v1: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub EnableNotification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, event: *mut ::core::ffi::c_void, cursor: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    EnableNotification: usize,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub ReceiveCurrent_v1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    ReceiveCurrent_v1: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub PeekNext_v1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    PeekNext_v1: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub PeekCurrent_v1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    PeekCurrent_v1: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Receive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Receive: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Peek: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Peek: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub ReceiveCurrent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    ReceiveCurrent: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub PeekNext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    PeekNext: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub PeekCurrent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    PeekCurrent: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcolproperties: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Properties: usize,
}
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IMSMQQueue3(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IMSMQQueue3 {
    pub unsafe fn Access(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Access)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn ShareMode(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).ShareMode)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn QueueInfo(&self) -> ::windows::core::Result<IMSMQQueueInfo3> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).QueueInfo)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQQueueInfo3>(result__)
    }
    pub unsafe fn Handle(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Handle)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn IsOpen(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).IsOpen)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Close)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Receive_v1(&self, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Receive_v1)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(transaction), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(receivetimeout), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQMessage>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Peek_v1(&self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Peek_v1)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(receivetimeout), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQMessage>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn EnableNotification<'a, P0>(&self, event: P0, cursor: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IMSMQEvent3>>,
    {
        (::windows::core::Interface::vtable(self).EnableNotification)(::windows::core::Interface::as_raw(self), event.into().abi(), ::core::mem::transmute(cursor), ::core::mem::transmute(receivetimeout)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ReceiveCurrent_v1(&self, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).ReceiveCurrent_v1)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(transaction), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(receivetimeout), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQMessage>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PeekNext_v1(&self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).PeekNext_v1)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(receivetimeout), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQMessage>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PeekCurrent_v1(&self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).PeekCurrent_v1)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(receivetimeout), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQMessage>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Receive(&self, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage3> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Receive)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(transaction), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(receivetimeout), ::core::mem::transmute(wantconnectortype), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQMessage3>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Peek(&self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage3> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Peek)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(receivetimeout), ::core::mem::transmute(wantconnectortype), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQMessage3>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ReceiveCurrent(&self, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage3> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).ReceiveCurrent)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(transaction), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(receivetimeout), ::core::mem::transmute(wantconnectortype), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQMessage3>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PeekNext(&self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage3> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).PeekNext)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(receivetimeout), ::core::mem::transmute(wantconnectortype), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQMessage3>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PeekCurrent(&self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage3> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).PeekCurrent)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(receivetimeout), ::core::mem::transmute(wantconnectortype), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQMessage3>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Properties(&self) -> ::windows::core::Result<super::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Properties)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::IDispatch>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Handle2(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Handle2)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ReceiveByLookupId<'a, P0>(&self, lookupid: P0, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage3>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::Com::VARIANT>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).ReceiveByLookupId)(::windows::core::Interface::as_raw(self), lookupid.into().abi(), ::core::mem::transmute(transaction), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(wantconnectortype), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQMessage3>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ReceiveNextByLookupId<'a, P0>(&self, lookupid: P0, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage3>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::Com::VARIANT>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).ReceiveNextByLookupId)(::windows::core::Interface::as_raw(self), lookupid.into().abi(), ::core::mem::transmute(transaction), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(wantconnectortype), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQMessage3>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ReceivePreviousByLookupId<'a, P0>(&self, lookupid: P0, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage3>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::Com::VARIANT>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).ReceivePreviousByLookupId)(::windows::core::Interface::as_raw(self), lookupid.into().abi(), ::core::mem::transmute(transaction), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(wantconnectortype), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQMessage3>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ReceiveFirstByLookupId(&self, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage3> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).ReceiveFirstByLookupId)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(transaction), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(wantconnectortype), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQMessage3>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ReceiveLastByLookupId(&self, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage3> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).ReceiveLastByLookupId)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(transaction), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(wantconnectortype), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQMessage3>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PeekByLookupId<'a, P0>(&self, lookupid: P0, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage3>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::Com::VARIANT>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).PeekByLookupId)(::windows::core::Interface::as_raw(self), lookupid.into().abi(), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(wantconnectortype), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQMessage3>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PeekNextByLookupId<'a, P0>(&self, lookupid: P0, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage3>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::Com::VARIANT>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).PeekNextByLookupId)(::windows::core::Interface::as_raw(self), lookupid.into().abi(), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(wantconnectortype), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQMessage3>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PeekPreviousByLookupId<'a, P0>(&self, lookupid: P0, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage3>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::Com::VARIANT>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).PeekPreviousByLookupId)(::windows::core::Interface::as_raw(self), lookupid.into().abi(), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(wantconnectortype), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQMessage3>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PeekFirstByLookupId(&self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage3> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).PeekFirstByLookupId)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(wantconnectortype), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQMessage3>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PeekLastByLookupId(&self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage3> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).PeekLastByLookupId)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(wantconnectortype), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQMessage3>(result__)
    }
    pub unsafe fn Purge(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Purge)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn IsOpen2(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).IsOpen2)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMSMQQueue3> for ::windows::core::IUnknown {
    fn from(value: IMSMQQueue3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IMSMQQueue3> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IMSMQQueue3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMSMQQueue3> for ::windows::core::IUnknown {
    fn from(value: &IMSMQQueue3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMSMQQueue3> for super::Com::IDispatch {
    fn from(value: IMSMQQueue3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IMSMQQueue3> for &'a super::Com::IDispatch {
    fn from(value: &'a IMSMQQueue3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMSMQQueue3> for super::Com::IDispatch {
    fn from(value: &IMSMQQueue3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IMSMQQueue3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IMSMQQueue3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IMSMQQueue3 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IMSMQQueue3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMSMQQueue3").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IMSMQQueue3 {
    type Vtable = IMSMQQueue3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeba96b1b_2168_11d3_898c_00e02c074f6b);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQQueue3_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Access: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, placcess: *mut i32) -> ::windows::core::HRESULT,
    pub ShareMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plsharemode: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub QueueInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppqinfo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    QueueInfo: usize,
    pub Handle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plhandle: *mut i32) -> ::windows::core::HRESULT,
    pub IsOpen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pisopen: *mut i16) -> ::windows::core::HRESULT,
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Receive_v1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Receive_v1: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Peek_v1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Peek_v1: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub EnableNotification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, event: *mut ::core::ffi::c_void, cursor: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    EnableNotification: usize,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub ReceiveCurrent_v1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    ReceiveCurrent_v1: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub PeekNext_v1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    PeekNext_v1: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub PeekCurrent_v1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    PeekCurrent_v1: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Receive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Receive: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Peek: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Peek: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub ReceiveCurrent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    ReceiveCurrent: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub PeekNext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    PeekNext: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub PeekCurrent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    PeekCurrent: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcolproperties: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Properties: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Handle2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarhandle: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Handle2: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub ReceiveByLookupId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lookupid: ::core::mem::ManuallyDrop<super::Com::VARIANT>, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    ReceiveByLookupId: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub ReceiveNextByLookupId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lookupid: ::core::mem::ManuallyDrop<super::Com::VARIANT>, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    ReceiveNextByLookupId: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub ReceivePreviousByLookupId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lookupid: ::core::mem::ManuallyDrop<super::Com::VARIANT>, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    ReceivePreviousByLookupId: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub ReceiveFirstByLookupId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    ReceiveFirstByLookupId: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub ReceiveLastByLookupId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    ReceiveLastByLookupId: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub PeekByLookupId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lookupid: ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    PeekByLookupId: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub PeekNextByLookupId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lookupid: ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    PeekNextByLookupId: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub PeekPreviousByLookupId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lookupid: ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    PeekPreviousByLookupId: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub PeekFirstByLookupId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    PeekFirstByLookupId: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub PeekLastByLookupId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    PeekLastByLookupId: usize,
    pub Purge: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub IsOpen2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pisopen: *mut i16) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IMSMQQueue4(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IMSMQQueue4 {
    pub unsafe fn Access(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Access)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn ShareMode(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).ShareMode)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn QueueInfo(&self) -> ::windows::core::Result<IMSMQQueueInfo4> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).QueueInfo)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQQueueInfo4>(result__)
    }
    pub unsafe fn Handle(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Handle)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn IsOpen(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).IsOpen)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Close)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Receive_v1(&self, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Receive_v1)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(transaction), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(receivetimeout), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQMessage>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Peek_v1(&self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Peek_v1)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(receivetimeout), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQMessage>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn EnableNotification<'a, P0>(&self, event: P0, cursor: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IMSMQEvent3>>,
    {
        (::windows::core::Interface::vtable(self).EnableNotification)(::windows::core::Interface::as_raw(self), event.into().abi(), ::core::mem::transmute(cursor), ::core::mem::transmute(receivetimeout)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ReceiveCurrent_v1(&self, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).ReceiveCurrent_v1)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(transaction), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(receivetimeout), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQMessage>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PeekNext_v1(&self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).PeekNext_v1)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(receivetimeout), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQMessage>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PeekCurrent_v1(&self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).PeekCurrent_v1)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(receivetimeout), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQMessage>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Receive(&self, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage4> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Receive)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(transaction), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(receivetimeout), ::core::mem::transmute(wantconnectortype), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQMessage4>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Peek(&self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage4> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Peek)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(receivetimeout), ::core::mem::transmute(wantconnectortype), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQMessage4>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ReceiveCurrent(&self, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage4> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).ReceiveCurrent)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(transaction), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(receivetimeout), ::core::mem::transmute(wantconnectortype), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQMessage4>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PeekNext(&self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage4> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).PeekNext)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(receivetimeout), ::core::mem::transmute(wantconnectortype), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQMessage4>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PeekCurrent(&self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage4> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).PeekCurrent)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(receivetimeout), ::core::mem::transmute(wantconnectortype), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQMessage4>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Properties(&self) -> ::windows::core::Result<super::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Properties)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::IDispatch>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Handle2(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Handle2)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ReceiveByLookupId<'a, P0>(&self, lookupid: P0, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage4>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::Com::VARIANT>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).ReceiveByLookupId)(::windows::core::Interface::as_raw(self), lookupid.into().abi(), ::core::mem::transmute(transaction), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(wantconnectortype), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQMessage4>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ReceiveNextByLookupId<'a, P0>(&self, lookupid: P0, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage4>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::Com::VARIANT>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).ReceiveNextByLookupId)(::windows::core::Interface::as_raw(self), lookupid.into().abi(), ::core::mem::transmute(transaction), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(wantconnectortype), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQMessage4>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ReceivePreviousByLookupId<'a, P0>(&self, lookupid: P0, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage4>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::Com::VARIANT>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).ReceivePreviousByLookupId)(::windows::core::Interface::as_raw(self), lookupid.into().abi(), ::core::mem::transmute(transaction), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(wantconnectortype), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQMessage4>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ReceiveFirstByLookupId(&self, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage4> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).ReceiveFirstByLookupId)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(transaction), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(wantconnectortype), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQMessage4>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ReceiveLastByLookupId(&self, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage4> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).ReceiveLastByLookupId)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(transaction), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(wantconnectortype), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQMessage4>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PeekByLookupId<'a, P0>(&self, lookupid: P0, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage4>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::Com::VARIANT>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).PeekByLookupId)(::windows::core::Interface::as_raw(self), lookupid.into().abi(), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(wantconnectortype), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQMessage4>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PeekNextByLookupId<'a, P0>(&self, lookupid: P0, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage4>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::Com::VARIANT>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).PeekNextByLookupId)(::windows::core::Interface::as_raw(self), lookupid.into().abi(), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(wantconnectortype), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQMessage4>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PeekPreviousByLookupId<'a, P0>(&self, lookupid: P0, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage4>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::Com::VARIANT>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).PeekPreviousByLookupId)(::windows::core::Interface::as_raw(self), lookupid.into().abi(), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(wantconnectortype), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQMessage4>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PeekFirstByLookupId(&self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage4> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).PeekFirstByLookupId)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(wantconnectortype), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQMessage4>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PeekLastByLookupId(&self, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage4> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).PeekLastByLookupId)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(wantconnectortype), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQMessage4>(result__)
    }
    pub unsafe fn Purge(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Purge)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn IsOpen2(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).IsOpen2)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ReceiveByLookupIdAllowPeek<'a, P0>(&self, lookupid: P0, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT) -> ::windows::core::Result<IMSMQMessage4>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::Com::VARIANT>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).ReceiveByLookupIdAllowPeek)(::windows::core::Interface::as_raw(self), lookupid.into().abi(), ::core::mem::transmute(transaction), ::core::mem::transmute(wantdestinationqueue), ::core::mem::transmute(wantbody), ::core::mem::transmute(wantconnectortype), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQMessage4>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMSMQQueue4> for ::windows::core::IUnknown {
    fn from(value: IMSMQQueue4) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IMSMQQueue4> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IMSMQQueue4) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMSMQQueue4> for ::windows::core::IUnknown {
    fn from(value: &IMSMQQueue4) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMSMQQueue4> for super::Com::IDispatch {
    fn from(value: IMSMQQueue4) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IMSMQQueue4> for &'a super::Com::IDispatch {
    fn from(value: &'a IMSMQQueue4) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMSMQQueue4> for super::Com::IDispatch {
    fn from(value: &IMSMQQueue4) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IMSMQQueue4 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IMSMQQueue4 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IMSMQQueue4 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IMSMQQueue4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMSMQQueue4").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IMSMQQueue4 {
    type Vtable = IMSMQQueue4_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeba96b20_2168_11d3_898c_00e02c074f6b);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQQueue4_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Access: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, placcess: *mut i32) -> ::windows::core::HRESULT,
    pub ShareMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plsharemode: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub QueueInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppqinfo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    QueueInfo: usize,
    pub Handle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plhandle: *mut i32) -> ::windows::core::HRESULT,
    pub IsOpen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pisopen: *mut i16) -> ::windows::core::HRESULT,
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Receive_v1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Receive_v1: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Peek_v1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Peek_v1: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub EnableNotification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, event: *mut ::core::ffi::c_void, cursor: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    EnableNotification: usize,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub ReceiveCurrent_v1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    ReceiveCurrent_v1: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub PeekNext_v1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    PeekNext_v1: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub PeekCurrent_v1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    PeekCurrent_v1: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Receive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Receive: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Peek: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Peek: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub ReceiveCurrent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    ReceiveCurrent: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub PeekNext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    PeekNext: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub PeekCurrent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, receivetimeout: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    PeekCurrent: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcolproperties: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Properties: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Handle2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarhandle: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Handle2: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub ReceiveByLookupId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lookupid: ::core::mem::ManuallyDrop<super::Com::VARIANT>, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    ReceiveByLookupId: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub ReceiveNextByLookupId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lookupid: ::core::mem::ManuallyDrop<super::Com::VARIANT>, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    ReceiveNextByLookupId: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub ReceivePreviousByLookupId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lookupid: ::core::mem::ManuallyDrop<super::Com::VARIANT>, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    ReceivePreviousByLookupId: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub ReceiveFirstByLookupId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    ReceiveFirstByLookupId: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub ReceiveLastByLookupId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    ReceiveLastByLookupId: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub PeekByLookupId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lookupid: ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    PeekByLookupId: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub PeekNextByLookupId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lookupid: ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    PeekNextByLookupId: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub PeekPreviousByLookupId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lookupid: ::core::mem::ManuallyDrop<super::Com::VARIANT>, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    PeekPreviousByLookupId: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub PeekFirstByLookupId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    PeekFirstByLookupId: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub PeekLastByLookupId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    PeekLastByLookupId: usize,
    pub Purge: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub IsOpen2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pisopen: *mut i16) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub ReceiveByLookupIdAllowPeek: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lookupid: ::core::mem::ManuallyDrop<super::Com::VARIANT>, transaction: *const super::Com::VARIANT, wantdestinationqueue: *const super::Com::VARIANT, wantbody: *const super::Com::VARIANT, wantconnectortype: *const super::Com::VARIANT, ppmsg: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    ReceiveByLookupIdAllowPeek: usize,
}
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IMSMQQueueInfo(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IMSMQQueueInfo {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn QueueGuid(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).QueueGuid)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ServiceTypeGuid(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).ServiceTypeGuid)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetServiceTypeGuid<'a, P0>(&self, bstrguidservicetype: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).SetServiceTypeGuid)(::windows::core::Interface::as_raw(self), bstrguidservicetype.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Label(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Label)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLabel<'a, P0>(&self, bstrlabel: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).SetLabel)(::windows::core::Interface::as_raw(self), bstrlabel.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PathName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).PathName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetPathName<'a, P0>(&self, bstrpathname: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).SetPathName)(::windows::core::Interface::as_raw(self), bstrpathname.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FormatName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).FormatName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetFormatName<'a, P0>(&self, bstrformatname: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).SetFormatName)(::windows::core::Interface::as_raw(self), bstrformatname.into().abi()).ok()
    }
    pub unsafe fn IsTransactional(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).IsTransactional)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn PrivLevel(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).PrivLevel)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetPrivLevel(&self, lprivlevel: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetPrivLevel)(::windows::core::Interface::as_raw(self), lprivlevel).ok()
    }
    pub unsafe fn Journal(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Journal)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetJournal(&self, ljournal: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetJournal)(::windows::core::Interface::as_raw(self), ljournal).ok()
    }
    pub unsafe fn Quota(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Quota)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetQuota(&self, lquota: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetQuota)(::windows::core::Interface::as_raw(self), lquota).ok()
    }
    pub unsafe fn BasePriority(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).BasePriority)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetBasePriority(&self, lbasepriority: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetBasePriority)(::windows::core::Interface::as_raw(self), lbasepriority).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CreateTime(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).CreateTime)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ModifyTime(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).ModifyTime)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    pub unsafe fn Authenticate(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Authenticate)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetAuthenticate(&self, lauthenticate: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAuthenticate)(::windows::core::Interface::as_raw(self), lauthenticate).ok()
    }
    pub unsafe fn JournalQuota(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).JournalQuota)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetJournalQuota(&self, ljournalquota: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetJournalQuota)(::windows::core::Interface::as_raw(self), ljournalquota).ok()
    }
    pub unsafe fn IsWorldReadable(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).IsWorldReadable)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Create(&self, istransactional: *const super::Com::VARIANT, isworldreadable: *const super::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Create)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(istransactional), ::core::mem::transmute(isworldreadable)).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Delete)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Open(&self, access: i32, sharemode: i32) -> ::windows::core::Result<IMSMQQueue> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Open)(::windows::core::Interface::as_raw(self), access, sharemode, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQQueue>(result__)
    }
    pub unsafe fn Refresh(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Refresh)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Update(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Update)(::windows::core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMSMQQueueInfo> for ::windows::core::IUnknown {
    fn from(value: IMSMQQueueInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IMSMQQueueInfo> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IMSMQQueueInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMSMQQueueInfo> for ::windows::core::IUnknown {
    fn from(value: &IMSMQQueueInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMSMQQueueInfo> for super::Com::IDispatch {
    fn from(value: IMSMQQueueInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IMSMQQueueInfo> for &'a super::Com::IDispatch {
    fn from(value: &'a IMSMQQueueInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMSMQQueueInfo> for super::Com::IDispatch {
    fn from(value: &IMSMQQueueInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IMSMQQueueInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IMSMQQueueInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IMSMQQueueInfo {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IMSMQQueueInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMSMQQueueInfo").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IMSMQQueueInfo {
    type Vtable = IMSMQQueueInfo_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd7d6e07b_dccd_11d0_aa4b_0060970debae);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQQueueInfo_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub QueueGuid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrguidqueue: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    QueueGuid: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ServiceTypeGuid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrguidservicetype: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ServiceTypeGuid: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetServiceTypeGuid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrguidservicetype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetServiceTypeGuid: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Label: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrlabel: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Label: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetLabel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrlabel: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetLabel: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub PathName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrpathname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    PathName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetPathName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpathname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetPathName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub FormatName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrformatname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    FormatName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetFormatName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrformatname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetFormatName: usize,
    pub IsTransactional: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pistransactional: *mut i16) -> ::windows::core::HRESULT,
    pub PrivLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plprivlevel: *mut i32) -> ::windows::core::HRESULT,
    pub SetPrivLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lprivlevel: i32) -> ::windows::core::HRESULT,
    pub Journal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pljournal: *mut i32) -> ::windows::core::HRESULT,
    pub SetJournal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ljournal: i32) -> ::windows::core::HRESULT,
    pub Quota: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plquota: *mut i32) -> ::windows::core::HRESULT,
    pub SetQuota: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lquota: i32) -> ::windows::core::HRESULT,
    pub BasePriority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plbasepriority: *mut i32) -> ::windows::core::HRESULT,
    pub SetBasePriority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lbasepriority: i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub CreateTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarcreatetime: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    CreateTime: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub ModifyTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarmodifytime: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    ModifyTime: usize,
    pub Authenticate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plauthenticate: *mut i32) -> ::windows::core::HRESULT,
    pub SetAuthenticate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lauthenticate: i32) -> ::windows::core::HRESULT,
    pub JournalQuota: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pljournalquota: *mut i32) -> ::windows::core::HRESULT,
    pub SetJournalQuota: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ljournalquota: i32) -> ::windows::core::HRESULT,
    pub IsWorldReadable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pisworldreadable: *mut i16) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, istransactional: *const super::Com::VARIANT, isworldreadable: *const super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Create: usize,
    pub Delete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Open: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, access: i32, sharemode: i32, ppq: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Open: usize,
    pub Refresh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Update: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IMSMQQueueInfo2(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IMSMQQueueInfo2 {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn QueueGuid(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).QueueGuid)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ServiceTypeGuid(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).ServiceTypeGuid)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetServiceTypeGuid<'a, P0>(&self, bstrguidservicetype: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).SetServiceTypeGuid)(::windows::core::Interface::as_raw(self), bstrguidservicetype.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Label(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Label)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLabel<'a, P0>(&self, bstrlabel: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).SetLabel)(::windows::core::Interface::as_raw(self), bstrlabel.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PathName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).PathName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetPathName<'a, P0>(&self, bstrpathname: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).SetPathName)(::windows::core::Interface::as_raw(self), bstrpathname.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FormatName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).FormatName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetFormatName<'a, P0>(&self, bstrformatname: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).SetFormatName)(::windows::core::Interface::as_raw(self), bstrformatname.into().abi()).ok()
    }
    pub unsafe fn IsTransactional(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).IsTransactional)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn PrivLevel(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).PrivLevel)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetPrivLevel(&self, lprivlevel: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetPrivLevel)(::windows::core::Interface::as_raw(self), lprivlevel).ok()
    }
    pub unsafe fn Journal(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Journal)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetJournal(&self, ljournal: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetJournal)(::windows::core::Interface::as_raw(self), ljournal).ok()
    }
    pub unsafe fn Quota(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Quota)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetQuota(&self, lquota: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetQuota)(::windows::core::Interface::as_raw(self), lquota).ok()
    }
    pub unsafe fn BasePriority(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).BasePriority)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetBasePriority(&self, lbasepriority: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetBasePriority)(::windows::core::Interface::as_raw(self), lbasepriority).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CreateTime(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).CreateTime)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ModifyTime(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).ModifyTime)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    pub unsafe fn Authenticate(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Authenticate)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetAuthenticate(&self, lauthenticate: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAuthenticate)(::windows::core::Interface::as_raw(self), lauthenticate).ok()
    }
    pub unsafe fn JournalQuota(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).JournalQuota)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetJournalQuota(&self, ljournalquota: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetJournalQuota)(::windows::core::Interface::as_raw(self), ljournalquota).ok()
    }
    pub unsafe fn IsWorldReadable(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).IsWorldReadable)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Create(&self, istransactional: *const super::Com::VARIANT, isworldreadable: *const super::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Create)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(istransactional), ::core::mem::transmute(isworldreadable)).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Delete)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Open(&self, access: i32, sharemode: i32) -> ::windows::core::Result<IMSMQQueue2> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Open)(::windows::core::Interface::as_raw(self), access, sharemode, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQQueue2>(result__)
    }
    pub unsafe fn Refresh(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Refresh)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Update(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Update)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PathNameDNS(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).PathNameDNS)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Properties(&self) -> ::windows::core::Result<super::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Properties)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::IDispatch>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Security(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Security)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetSecurity<'a, P0>(&self, varsecurity: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::Com::VARIANT>>,
    {
        (::windows::core::Interface::vtable(self).SetSecurity)(::windows::core::Interface::as_raw(self), varsecurity.into().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMSMQQueueInfo2> for ::windows::core::IUnknown {
    fn from(value: IMSMQQueueInfo2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IMSMQQueueInfo2> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IMSMQQueueInfo2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMSMQQueueInfo2> for ::windows::core::IUnknown {
    fn from(value: &IMSMQQueueInfo2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMSMQQueueInfo2> for super::Com::IDispatch {
    fn from(value: IMSMQQueueInfo2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IMSMQQueueInfo2> for &'a super::Com::IDispatch {
    fn from(value: &'a IMSMQQueueInfo2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMSMQQueueInfo2> for super::Com::IDispatch {
    fn from(value: &IMSMQQueueInfo2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IMSMQQueueInfo2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IMSMQQueueInfo2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IMSMQQueueInfo2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IMSMQQueueInfo2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMSMQQueueInfo2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IMSMQQueueInfo2 {
    type Vtable = IMSMQQueueInfo2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfd174a80_89cf_11d2_b0f2_00e02c074f6b);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQQueueInfo2_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub QueueGuid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrguidqueue: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    QueueGuid: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ServiceTypeGuid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrguidservicetype: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ServiceTypeGuid: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetServiceTypeGuid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrguidservicetype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetServiceTypeGuid: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Label: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrlabel: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Label: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetLabel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrlabel: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetLabel: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub PathName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrpathname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    PathName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetPathName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpathname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetPathName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub FormatName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrformatname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    FormatName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetFormatName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrformatname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetFormatName: usize,
    pub IsTransactional: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pistransactional: *mut i16) -> ::windows::core::HRESULT,
    pub PrivLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plprivlevel: *mut i32) -> ::windows::core::HRESULT,
    pub SetPrivLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lprivlevel: i32) -> ::windows::core::HRESULT,
    pub Journal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pljournal: *mut i32) -> ::windows::core::HRESULT,
    pub SetJournal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ljournal: i32) -> ::windows::core::HRESULT,
    pub Quota: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plquota: *mut i32) -> ::windows::core::HRESULT,
    pub SetQuota: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lquota: i32) -> ::windows::core::HRESULT,
    pub BasePriority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plbasepriority: *mut i32) -> ::windows::core::HRESULT,
    pub SetBasePriority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lbasepriority: i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub CreateTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarcreatetime: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    CreateTime: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub ModifyTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarmodifytime: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    ModifyTime: usize,
    pub Authenticate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plauthenticate: *mut i32) -> ::windows::core::HRESULT,
    pub SetAuthenticate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lauthenticate: i32) -> ::windows::core::HRESULT,
    pub JournalQuota: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pljournalquota: *mut i32) -> ::windows::core::HRESULT,
    pub SetJournalQuota: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ljournalquota: i32) -> ::windows::core::HRESULT,
    pub IsWorldReadable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pisworldreadable: *mut i16) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, istransactional: *const super::Com::VARIANT, isworldreadable: *const super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Create: usize,
    pub Delete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Open: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, access: i32, sharemode: i32, ppq: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Open: usize,
    pub Refresh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Update: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub PathNameDNS: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrpathnamedns: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    PathNameDNS: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcolproperties: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Properties: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Security: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarsecurity: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Security: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetSecurity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varsecurity: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetSecurity: usize,
}
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IMSMQQueueInfo3(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IMSMQQueueInfo3 {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn QueueGuid(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).QueueGuid)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ServiceTypeGuid(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).ServiceTypeGuid)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetServiceTypeGuid<'a, P0>(&self, bstrguidservicetype: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).SetServiceTypeGuid)(::windows::core::Interface::as_raw(self), bstrguidservicetype.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Label(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Label)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLabel<'a, P0>(&self, bstrlabel: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).SetLabel)(::windows::core::Interface::as_raw(self), bstrlabel.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PathName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).PathName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetPathName<'a, P0>(&self, bstrpathname: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).SetPathName)(::windows::core::Interface::as_raw(self), bstrpathname.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FormatName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).FormatName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetFormatName<'a, P0>(&self, bstrformatname: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).SetFormatName)(::windows::core::Interface::as_raw(self), bstrformatname.into().abi()).ok()
    }
    pub unsafe fn IsTransactional(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).IsTransactional)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn PrivLevel(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).PrivLevel)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetPrivLevel(&self, lprivlevel: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetPrivLevel)(::windows::core::Interface::as_raw(self), lprivlevel).ok()
    }
    pub unsafe fn Journal(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Journal)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetJournal(&self, ljournal: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetJournal)(::windows::core::Interface::as_raw(self), ljournal).ok()
    }
    pub unsafe fn Quota(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Quota)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetQuota(&self, lquota: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetQuota)(::windows::core::Interface::as_raw(self), lquota).ok()
    }
    pub unsafe fn BasePriority(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).BasePriority)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetBasePriority(&self, lbasepriority: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetBasePriority)(::windows::core::Interface::as_raw(self), lbasepriority).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CreateTime(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).CreateTime)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ModifyTime(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).ModifyTime)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    pub unsafe fn Authenticate(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Authenticate)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetAuthenticate(&self, lauthenticate: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAuthenticate)(::windows::core::Interface::as_raw(self), lauthenticate).ok()
    }
    pub unsafe fn JournalQuota(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).JournalQuota)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetJournalQuota(&self, ljournalquota: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetJournalQuota)(::windows::core::Interface::as_raw(self), ljournalquota).ok()
    }
    pub unsafe fn IsWorldReadable(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).IsWorldReadable)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Create(&self, istransactional: *const super::Com::VARIANT, isworldreadable: *const super::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Create)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(istransactional), ::core::mem::transmute(isworldreadable)).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Delete)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Open(&self, access: i32, sharemode: i32) -> ::windows::core::Result<IMSMQQueue3> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Open)(::windows::core::Interface::as_raw(self), access, sharemode, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQQueue3>(result__)
    }
    pub unsafe fn Refresh(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Refresh)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Update(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Update)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PathNameDNS(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).PathNameDNS)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Properties(&self) -> ::windows::core::Result<super::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Properties)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::IDispatch>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Security(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Security)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetSecurity<'a, P0>(&self, varsecurity: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::Com::VARIANT>>,
    {
        (::windows::core::Interface::vtable(self).SetSecurity)(::windows::core::Interface::as_raw(self), varsecurity.into().abi()).ok()
    }
    pub unsafe fn IsTransactional2(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).IsTransactional2)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn IsWorldReadable2(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).IsWorldReadable2)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MulticastAddress(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).MulticastAddress)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetMulticastAddress<'a, P0>(&self, bstrmulticastaddress: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).SetMulticastAddress)(::windows::core::Interface::as_raw(self), bstrmulticastaddress.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ADsPath(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).ADsPath)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMSMQQueueInfo3> for ::windows::core::IUnknown {
    fn from(value: IMSMQQueueInfo3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IMSMQQueueInfo3> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IMSMQQueueInfo3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMSMQQueueInfo3> for ::windows::core::IUnknown {
    fn from(value: &IMSMQQueueInfo3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMSMQQueueInfo3> for super::Com::IDispatch {
    fn from(value: IMSMQQueueInfo3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IMSMQQueueInfo3> for &'a super::Com::IDispatch {
    fn from(value: &'a IMSMQQueueInfo3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMSMQQueueInfo3> for super::Com::IDispatch {
    fn from(value: &IMSMQQueueInfo3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IMSMQQueueInfo3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IMSMQQueueInfo3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IMSMQQueueInfo3 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IMSMQQueueInfo3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMSMQQueueInfo3").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IMSMQQueueInfo3 {
    type Vtable = IMSMQQueueInfo3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeba96b1d_2168_11d3_898c_00e02c074f6b);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQQueueInfo3_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub QueueGuid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrguidqueue: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    QueueGuid: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ServiceTypeGuid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrguidservicetype: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ServiceTypeGuid: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetServiceTypeGuid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrguidservicetype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetServiceTypeGuid: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Label: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrlabel: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Label: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetLabel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrlabel: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetLabel: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub PathName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrpathname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    PathName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetPathName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpathname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetPathName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub FormatName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrformatname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    FormatName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetFormatName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrformatname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetFormatName: usize,
    pub IsTransactional: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pistransactional: *mut i16) -> ::windows::core::HRESULT,
    pub PrivLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plprivlevel: *mut i32) -> ::windows::core::HRESULT,
    pub SetPrivLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lprivlevel: i32) -> ::windows::core::HRESULT,
    pub Journal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pljournal: *mut i32) -> ::windows::core::HRESULT,
    pub SetJournal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ljournal: i32) -> ::windows::core::HRESULT,
    pub Quota: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plquota: *mut i32) -> ::windows::core::HRESULT,
    pub SetQuota: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lquota: i32) -> ::windows::core::HRESULT,
    pub BasePriority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plbasepriority: *mut i32) -> ::windows::core::HRESULT,
    pub SetBasePriority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lbasepriority: i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub CreateTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarcreatetime: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    CreateTime: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub ModifyTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarmodifytime: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    ModifyTime: usize,
    pub Authenticate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plauthenticate: *mut i32) -> ::windows::core::HRESULT,
    pub SetAuthenticate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lauthenticate: i32) -> ::windows::core::HRESULT,
    pub JournalQuota: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pljournalquota: *mut i32) -> ::windows::core::HRESULT,
    pub SetJournalQuota: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ljournalquota: i32) -> ::windows::core::HRESULT,
    pub IsWorldReadable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pisworldreadable: *mut i16) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, istransactional: *const super::Com::VARIANT, isworldreadable: *const super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Create: usize,
    pub Delete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Open: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, access: i32, sharemode: i32, ppq: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Open: usize,
    pub Refresh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Update: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub PathNameDNS: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrpathnamedns: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    PathNameDNS: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcolproperties: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Properties: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Security: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarsecurity: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Security: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetSecurity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varsecurity: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetSecurity: usize,
    pub IsTransactional2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pistransactional: *mut i16) -> ::windows::core::HRESULT,
    pub IsWorldReadable2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pisworldreadable: *mut i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub MulticastAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrmulticastaddress: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MulticastAddress: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetMulticastAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrmulticastaddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetMulticastAddress: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ADsPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstradspath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ADsPath: usize,
}
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IMSMQQueueInfo4(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IMSMQQueueInfo4 {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn QueueGuid(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).QueueGuid)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ServiceTypeGuid(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).ServiceTypeGuid)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetServiceTypeGuid<'a, P0>(&self, bstrguidservicetype: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).SetServiceTypeGuid)(::windows::core::Interface::as_raw(self), bstrguidservicetype.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Label(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Label)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLabel<'a, P0>(&self, bstrlabel: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).SetLabel)(::windows::core::Interface::as_raw(self), bstrlabel.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PathName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).PathName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetPathName<'a, P0>(&self, bstrpathname: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).SetPathName)(::windows::core::Interface::as_raw(self), bstrpathname.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FormatName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).FormatName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetFormatName<'a, P0>(&self, bstrformatname: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).SetFormatName)(::windows::core::Interface::as_raw(self), bstrformatname.into().abi()).ok()
    }
    pub unsafe fn IsTransactional(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).IsTransactional)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn PrivLevel(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).PrivLevel)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetPrivLevel(&self, lprivlevel: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetPrivLevel)(::windows::core::Interface::as_raw(self), lprivlevel).ok()
    }
    pub unsafe fn Journal(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Journal)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetJournal(&self, ljournal: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetJournal)(::windows::core::Interface::as_raw(self), ljournal).ok()
    }
    pub unsafe fn Quota(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Quota)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetQuota(&self, lquota: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetQuota)(::windows::core::Interface::as_raw(self), lquota).ok()
    }
    pub unsafe fn BasePriority(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).BasePriority)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetBasePriority(&self, lbasepriority: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetBasePriority)(::windows::core::Interface::as_raw(self), lbasepriority).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CreateTime(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).CreateTime)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ModifyTime(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).ModifyTime)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    pub unsafe fn Authenticate(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Authenticate)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetAuthenticate(&self, lauthenticate: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAuthenticate)(::windows::core::Interface::as_raw(self), lauthenticate).ok()
    }
    pub unsafe fn JournalQuota(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).JournalQuota)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetJournalQuota(&self, ljournalquota: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetJournalQuota)(::windows::core::Interface::as_raw(self), ljournalquota).ok()
    }
    pub unsafe fn IsWorldReadable(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).IsWorldReadable)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Create(&self, istransactional: *const super::Com::VARIANT, isworldreadable: *const super::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Create)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(istransactional), ::core::mem::transmute(isworldreadable)).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Delete)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Open(&self, access: i32, sharemode: i32) -> ::windows::core::Result<IMSMQQueue4> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Open)(::windows::core::Interface::as_raw(self), access, sharemode, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQQueue4>(result__)
    }
    pub unsafe fn Refresh(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Refresh)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Update(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Update)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PathNameDNS(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).PathNameDNS)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Properties(&self) -> ::windows::core::Result<super::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Properties)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::IDispatch>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Security(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Security)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetSecurity<'a, P0>(&self, varsecurity: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::Com::VARIANT>>,
    {
        (::windows::core::Interface::vtable(self).SetSecurity)(::windows::core::Interface::as_raw(self), varsecurity.into().abi()).ok()
    }
    pub unsafe fn IsTransactional2(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).IsTransactional2)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn IsWorldReadable2(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).IsWorldReadable2)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MulticastAddress(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).MulticastAddress)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetMulticastAddress<'a, P0>(&self, bstrmulticastaddress: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).SetMulticastAddress)(::windows::core::Interface::as_raw(self), bstrmulticastaddress.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ADsPath(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).ADsPath)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMSMQQueueInfo4> for ::windows::core::IUnknown {
    fn from(value: IMSMQQueueInfo4) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IMSMQQueueInfo4> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IMSMQQueueInfo4) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMSMQQueueInfo4> for ::windows::core::IUnknown {
    fn from(value: &IMSMQQueueInfo4) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMSMQQueueInfo4> for super::Com::IDispatch {
    fn from(value: IMSMQQueueInfo4) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IMSMQQueueInfo4> for &'a super::Com::IDispatch {
    fn from(value: &'a IMSMQQueueInfo4) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMSMQQueueInfo4> for super::Com::IDispatch {
    fn from(value: &IMSMQQueueInfo4) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IMSMQQueueInfo4 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IMSMQQueueInfo4 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IMSMQQueueInfo4 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IMSMQQueueInfo4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMSMQQueueInfo4").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IMSMQQueueInfo4 {
    type Vtable = IMSMQQueueInfo4_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeba96b21_2168_11d3_898c_00e02c074f6b);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQQueueInfo4_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub QueueGuid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrguidqueue: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    QueueGuid: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ServiceTypeGuid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrguidservicetype: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ServiceTypeGuid: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetServiceTypeGuid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrguidservicetype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetServiceTypeGuid: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Label: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrlabel: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Label: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetLabel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrlabel: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetLabel: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub PathName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrpathname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    PathName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetPathName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpathname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetPathName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub FormatName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrformatname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    FormatName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetFormatName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrformatname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetFormatName: usize,
    pub IsTransactional: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pistransactional: *mut i16) -> ::windows::core::HRESULT,
    pub PrivLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plprivlevel: *mut i32) -> ::windows::core::HRESULT,
    pub SetPrivLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lprivlevel: i32) -> ::windows::core::HRESULT,
    pub Journal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pljournal: *mut i32) -> ::windows::core::HRESULT,
    pub SetJournal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ljournal: i32) -> ::windows::core::HRESULT,
    pub Quota: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plquota: *mut i32) -> ::windows::core::HRESULT,
    pub SetQuota: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lquota: i32) -> ::windows::core::HRESULT,
    pub BasePriority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plbasepriority: *mut i32) -> ::windows::core::HRESULT,
    pub SetBasePriority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lbasepriority: i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub CreateTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarcreatetime: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    CreateTime: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub ModifyTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarmodifytime: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    ModifyTime: usize,
    pub Authenticate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plauthenticate: *mut i32) -> ::windows::core::HRESULT,
    pub SetAuthenticate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lauthenticate: i32) -> ::windows::core::HRESULT,
    pub JournalQuota: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pljournalquota: *mut i32) -> ::windows::core::HRESULT,
    pub SetJournalQuota: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ljournalquota: i32) -> ::windows::core::HRESULT,
    pub IsWorldReadable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pisworldreadable: *mut i16) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, istransactional: *const super::Com::VARIANT, isworldreadable: *const super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Create: usize,
    pub Delete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Open: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, access: i32, sharemode: i32, ppq: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Open: usize,
    pub Refresh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Update: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub PathNameDNS: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrpathnamedns: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    PathNameDNS: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcolproperties: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Properties: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Security: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarsecurity: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Security: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetSecurity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varsecurity: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetSecurity: usize,
    pub IsTransactional2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pistransactional: *mut i16) -> ::windows::core::HRESULT,
    pub IsWorldReadable2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pisworldreadable: *mut i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub MulticastAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrmulticastaddress: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MulticastAddress: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetMulticastAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrmulticastaddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetMulticastAddress: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ADsPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstradspath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ADsPath: usize,
}
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IMSMQQueueInfos(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IMSMQQueueInfos {
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Next(&self) -> ::windows::core::Result<IMSMQQueueInfo> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Next)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQQueueInfo>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMSMQQueueInfos> for ::windows::core::IUnknown {
    fn from(value: IMSMQQueueInfos) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IMSMQQueueInfos> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IMSMQQueueInfos) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMSMQQueueInfos> for ::windows::core::IUnknown {
    fn from(value: &IMSMQQueueInfos) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMSMQQueueInfos> for super::Com::IDispatch {
    fn from(value: IMSMQQueueInfos) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IMSMQQueueInfos> for &'a super::Com::IDispatch {
    fn from(value: &'a IMSMQQueueInfos) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMSMQQueueInfos> for super::Com::IDispatch {
    fn from(value: &IMSMQQueueInfos) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IMSMQQueueInfos {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IMSMQQueueInfos {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IMSMQQueueInfos {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IMSMQQueueInfos {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMSMQQueueInfos").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IMSMQQueueInfos {
    type Vtable = IMSMQQueueInfos_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd7d6e07d_dccd_11d0_aa4b_0060970debae);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQQueueInfos_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppqinfonext: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Next: usize,
}
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IMSMQQueueInfos2(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IMSMQQueueInfos2 {
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Next(&self) -> ::windows::core::Result<IMSMQQueueInfo2> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Next)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQQueueInfo2>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Properties(&self) -> ::windows::core::Result<super::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Properties)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::IDispatch>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMSMQQueueInfos2> for ::windows::core::IUnknown {
    fn from(value: IMSMQQueueInfos2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IMSMQQueueInfos2> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IMSMQQueueInfos2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMSMQQueueInfos2> for ::windows::core::IUnknown {
    fn from(value: &IMSMQQueueInfos2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMSMQQueueInfos2> for super::Com::IDispatch {
    fn from(value: IMSMQQueueInfos2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IMSMQQueueInfos2> for &'a super::Com::IDispatch {
    fn from(value: &'a IMSMQQueueInfos2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMSMQQueueInfos2> for super::Com::IDispatch {
    fn from(value: &IMSMQQueueInfos2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IMSMQQueueInfos2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IMSMQQueueInfos2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IMSMQQueueInfos2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IMSMQQueueInfos2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMSMQQueueInfos2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IMSMQQueueInfos2 {
    type Vtable = IMSMQQueueInfos2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeba96b0f_2168_11d3_898c_00e02c074f6b);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQQueueInfos2_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppqinfonext: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Next: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcolproperties: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Properties: usize,
}
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IMSMQQueueInfos3(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IMSMQQueueInfos3 {
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Next(&self) -> ::windows::core::Result<IMSMQQueueInfo3> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Next)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQQueueInfo3>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Properties(&self) -> ::windows::core::Result<super::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Properties)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::IDispatch>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMSMQQueueInfos3> for ::windows::core::IUnknown {
    fn from(value: IMSMQQueueInfos3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IMSMQQueueInfos3> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IMSMQQueueInfos3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMSMQQueueInfos3> for ::windows::core::IUnknown {
    fn from(value: &IMSMQQueueInfos3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMSMQQueueInfos3> for super::Com::IDispatch {
    fn from(value: IMSMQQueueInfos3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IMSMQQueueInfos3> for &'a super::Com::IDispatch {
    fn from(value: &'a IMSMQQueueInfos3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMSMQQueueInfos3> for super::Com::IDispatch {
    fn from(value: &IMSMQQueueInfos3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IMSMQQueueInfos3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IMSMQQueueInfos3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IMSMQQueueInfos3 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IMSMQQueueInfos3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMSMQQueueInfos3").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IMSMQQueueInfos3 {
    type Vtable = IMSMQQueueInfos3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeba96b1e_2168_11d3_898c_00e02c074f6b);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQQueueInfos3_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppqinfonext: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Next: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcolproperties: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Properties: usize,
}
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IMSMQQueueInfos4(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IMSMQQueueInfos4 {
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Next(&self) -> ::windows::core::Result<IMSMQQueueInfo4> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Next)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQQueueInfo4>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Properties(&self) -> ::windows::core::Result<super::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Properties)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::IDispatch>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMSMQQueueInfos4> for ::windows::core::IUnknown {
    fn from(value: IMSMQQueueInfos4) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IMSMQQueueInfos4> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IMSMQQueueInfos4) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMSMQQueueInfos4> for ::windows::core::IUnknown {
    fn from(value: &IMSMQQueueInfos4) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMSMQQueueInfos4> for super::Com::IDispatch {
    fn from(value: IMSMQQueueInfos4) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IMSMQQueueInfos4> for &'a super::Com::IDispatch {
    fn from(value: &'a IMSMQQueueInfos4) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMSMQQueueInfos4> for super::Com::IDispatch {
    fn from(value: &IMSMQQueueInfos4) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IMSMQQueueInfos4 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IMSMQQueueInfos4 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IMSMQQueueInfos4 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IMSMQQueueInfos4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMSMQQueueInfos4").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IMSMQQueueInfos4 {
    type Vtable = IMSMQQueueInfos4_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeba96b22_2168_11d3_898c_00e02c074f6b);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQQueueInfos4_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppqinfonext: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Next: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcolproperties: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Properties: usize,
}
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IMSMQQueueManagement(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IMSMQQueueManagement {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Init(&self, machine: *const super::Com::VARIANT, pathname: *const super::Com::VARIANT, formatname: *const super::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Init)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(machine), ::core::mem::transmute(pathname), ::core::mem::transmute(formatname)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FormatName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.FormatName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Machine(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.Machine)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn MessageCount(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.MessageCount)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn ForeignStatus(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.ForeignStatus)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn QueueType(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.QueueType)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn IsLocal(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.IsLocal)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn TransactionalStatus(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.TransactionalStatus)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn BytesInQueue(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.BytesInQueue)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    pub unsafe fn JournalMessageCount(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).JournalMessageCount)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn BytesInJournal(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).BytesInJournal)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn EodGetReceiveInfo(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).EodGetReceiveInfo)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMSMQQueueManagement> for ::windows::core::IUnknown {
    fn from(value: IMSMQQueueManagement) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IMSMQQueueManagement> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IMSMQQueueManagement) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMSMQQueueManagement> for ::windows::core::IUnknown {
    fn from(value: &IMSMQQueueManagement) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMSMQQueueManagement> for super::Com::IDispatch {
    fn from(value: IMSMQQueueManagement) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IMSMQQueueManagement> for &'a super::Com::IDispatch {
    fn from(value: &'a IMSMQQueueManagement) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMSMQQueueManagement> for super::Com::IDispatch {
    fn from(value: &IMSMQQueueManagement) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMSMQQueueManagement> for IMSMQManagement {
    fn from(value: IMSMQQueueManagement) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IMSMQQueueManagement> for &'a IMSMQManagement {
    fn from(value: &'a IMSMQQueueManagement) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMSMQQueueManagement> for IMSMQManagement {
    fn from(value: &IMSMQQueueManagement) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IMSMQQueueManagement {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IMSMQQueueManagement {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IMSMQQueueManagement {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IMSMQQueueManagement {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMSMQQueueManagement").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IMSMQQueueManagement {
    type Vtable = IMSMQQueueManagement_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7fbe7759_5760_444d_b8a5_5e7ab9a84cce);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQQueueManagement_Vtbl {
    pub base__: IMSMQManagement_Vtbl,
    pub JournalMessageCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pljournalmessagecount: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub BytesInJournal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvbytesinjournal: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    BytesInJournal: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub EodGetReceiveInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvcollection: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    EodGetReceiveInfo: usize,
}
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IMSMQTransaction(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IMSMQTransaction {
    pub unsafe fn Transaction(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Transaction)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Commit(&self, fretaining: *const super::Com::VARIANT, grftc: *const super::Com::VARIANT, grfrm: *const super::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Commit)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(fretaining), ::core::mem::transmute(grftc), ::core::mem::transmute(grfrm)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Abort(&self, fretaining: *const super::Com::VARIANT, fasync: *const super::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Abort)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(fretaining), ::core::mem::transmute(fasync)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMSMQTransaction> for ::windows::core::IUnknown {
    fn from(value: IMSMQTransaction) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IMSMQTransaction> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IMSMQTransaction) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMSMQTransaction> for ::windows::core::IUnknown {
    fn from(value: &IMSMQTransaction) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMSMQTransaction> for super::Com::IDispatch {
    fn from(value: IMSMQTransaction) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IMSMQTransaction> for &'a super::Com::IDispatch {
    fn from(value: &'a IMSMQTransaction) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMSMQTransaction> for super::Com::IDispatch {
    fn from(value: &IMSMQTransaction) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IMSMQTransaction {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IMSMQTransaction {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IMSMQTransaction {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IMSMQTransaction {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMSMQTransaction").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IMSMQTransaction {
    type Vtable = IMSMQTransaction_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd7d6e07f_dccd_11d0_aa4b_0060970debae);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQTransaction_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Transaction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pltransaction: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Commit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fretaining: *const super::Com::VARIANT, grftc: *const super::Com::VARIANT, grfrm: *const super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Commit: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Abort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fretaining: *const super::Com::VARIANT, fasync: *const super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Abort: usize,
}
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IMSMQTransaction2(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IMSMQTransaction2 {
    pub unsafe fn Transaction(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.Transaction)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Commit(&self, fretaining: *const super::Com::VARIANT, grftc: *const super::Com::VARIANT, grfrm: *const super::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Commit)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(fretaining), ::core::mem::transmute(grftc), ::core::mem::transmute(grfrm)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Abort(&self, fretaining: *const super::Com::VARIANT, fasync: *const super::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Abort)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(fretaining), ::core::mem::transmute(fasync)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn InitNew<'a, P0>(&self, vartransaction: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::Com::VARIANT>>,
    {
        (::windows::core::Interface::vtable(self).InitNew)(::windows::core::Interface::as_raw(self), vartransaction.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Properties(&self) -> ::windows::core::Result<super::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Properties)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::IDispatch>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMSMQTransaction2> for ::windows::core::IUnknown {
    fn from(value: IMSMQTransaction2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IMSMQTransaction2> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IMSMQTransaction2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMSMQTransaction2> for ::windows::core::IUnknown {
    fn from(value: &IMSMQTransaction2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMSMQTransaction2> for super::Com::IDispatch {
    fn from(value: IMSMQTransaction2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IMSMQTransaction2> for &'a super::Com::IDispatch {
    fn from(value: &'a IMSMQTransaction2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMSMQTransaction2> for super::Com::IDispatch {
    fn from(value: &IMSMQTransaction2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMSMQTransaction2> for IMSMQTransaction {
    fn from(value: IMSMQTransaction2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IMSMQTransaction2> for &'a IMSMQTransaction {
    fn from(value: &'a IMSMQTransaction2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMSMQTransaction2> for IMSMQTransaction {
    fn from(value: &IMSMQTransaction2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IMSMQTransaction2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IMSMQTransaction2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IMSMQTransaction2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IMSMQTransaction2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMSMQTransaction2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IMSMQTransaction2 {
    type Vtable = IMSMQTransaction2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2ce0c5b0_6e67_11d2_b0e6_00e02c074f6b);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQTransaction2_Vtbl {
    pub base__: IMSMQTransaction_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub InitNew: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vartransaction: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    InitNew: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcolproperties: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Properties: usize,
}
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IMSMQTransaction3(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IMSMQTransaction3 {
    pub unsafe fn Transaction(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.Transaction)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Commit(&self, fretaining: *const super::Com::VARIANT, grftc: *const super::Com::VARIANT, grfrm: *const super::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.Commit)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(fretaining), ::core::mem::transmute(grftc), ::core::mem::transmute(grfrm)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Abort(&self, fretaining: *const super::Com::VARIANT, fasync: *const super::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.Abort)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(fretaining), ::core::mem::transmute(fasync)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn InitNew<'a, P0>(&self, vartransaction: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::Com::VARIANT>>,
    {
        (::windows::core::Interface::vtable(self).base__.InitNew)(::windows::core::Interface::as_raw(self), vartransaction.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Properties(&self) -> ::windows::core::Result<super::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.Properties)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::IDispatch>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ITransaction(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).ITransaction)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMSMQTransaction3> for ::windows::core::IUnknown {
    fn from(value: IMSMQTransaction3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IMSMQTransaction3> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IMSMQTransaction3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMSMQTransaction3> for ::windows::core::IUnknown {
    fn from(value: &IMSMQTransaction3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMSMQTransaction3> for super::Com::IDispatch {
    fn from(value: IMSMQTransaction3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IMSMQTransaction3> for &'a super::Com::IDispatch {
    fn from(value: &'a IMSMQTransaction3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMSMQTransaction3> for super::Com::IDispatch {
    fn from(value: &IMSMQTransaction3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMSMQTransaction3> for IMSMQTransaction {
    fn from(value: IMSMQTransaction3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IMSMQTransaction3> for &'a IMSMQTransaction {
    fn from(value: &'a IMSMQTransaction3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMSMQTransaction3> for IMSMQTransaction {
    fn from(value: &IMSMQTransaction3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMSMQTransaction3> for IMSMQTransaction2 {
    fn from(value: IMSMQTransaction3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IMSMQTransaction3> for &'a IMSMQTransaction2 {
    fn from(value: &'a IMSMQTransaction3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMSMQTransaction3> for IMSMQTransaction2 {
    fn from(value: &IMSMQTransaction3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IMSMQTransaction3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IMSMQTransaction3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IMSMQTransaction3 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IMSMQTransaction3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMSMQTransaction3").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IMSMQTransaction3 {
    type Vtable = IMSMQTransaction3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeba96b13_2168_11d3_898c_00e02c074f6b);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQTransaction3_Vtbl {
    pub base__: IMSMQTransaction2_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub ITransaction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvaritransaction: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    ITransaction: usize,
}
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IMSMQTransactionDispenser(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IMSMQTransactionDispenser {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn BeginTransaction(&self) -> ::windows::core::Result<IMSMQTransaction> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).BeginTransaction)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQTransaction>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMSMQTransactionDispenser> for ::windows::core::IUnknown {
    fn from(value: IMSMQTransactionDispenser) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IMSMQTransactionDispenser> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IMSMQTransactionDispenser) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMSMQTransactionDispenser> for ::windows::core::IUnknown {
    fn from(value: &IMSMQTransactionDispenser) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMSMQTransactionDispenser> for super::Com::IDispatch {
    fn from(value: IMSMQTransactionDispenser) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IMSMQTransactionDispenser> for &'a super::Com::IDispatch {
    fn from(value: &'a IMSMQTransactionDispenser) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMSMQTransactionDispenser> for super::Com::IDispatch {
    fn from(value: &IMSMQTransactionDispenser) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IMSMQTransactionDispenser {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IMSMQTransactionDispenser {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IMSMQTransactionDispenser {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IMSMQTransactionDispenser {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMSMQTransactionDispenser").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IMSMQTransactionDispenser {
    type Vtable = IMSMQTransactionDispenser_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd7d6e083_dccd_11d0_aa4b_0060970debae);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQTransactionDispenser_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub BeginTransaction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptransaction: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    BeginTransaction: usize,
}
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IMSMQTransactionDispenser2(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IMSMQTransactionDispenser2 {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn BeginTransaction(&self) -> ::windows::core::Result<IMSMQTransaction2> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).BeginTransaction)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQTransaction2>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Properties(&self) -> ::windows::core::Result<super::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Properties)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::IDispatch>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMSMQTransactionDispenser2> for ::windows::core::IUnknown {
    fn from(value: IMSMQTransactionDispenser2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IMSMQTransactionDispenser2> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IMSMQTransactionDispenser2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMSMQTransactionDispenser2> for ::windows::core::IUnknown {
    fn from(value: &IMSMQTransactionDispenser2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMSMQTransactionDispenser2> for super::Com::IDispatch {
    fn from(value: IMSMQTransactionDispenser2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IMSMQTransactionDispenser2> for &'a super::Com::IDispatch {
    fn from(value: &'a IMSMQTransactionDispenser2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMSMQTransactionDispenser2> for super::Com::IDispatch {
    fn from(value: &IMSMQTransactionDispenser2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IMSMQTransactionDispenser2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IMSMQTransactionDispenser2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IMSMQTransactionDispenser2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IMSMQTransactionDispenser2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMSMQTransactionDispenser2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IMSMQTransactionDispenser2 {
    type Vtable = IMSMQTransactionDispenser2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeba96b11_2168_11d3_898c_00e02c074f6b);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQTransactionDispenser2_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub BeginTransaction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptransaction: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    BeginTransaction: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcolproperties: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Properties: usize,
}
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IMSMQTransactionDispenser3(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IMSMQTransactionDispenser3 {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn BeginTransaction(&self) -> ::windows::core::Result<IMSMQTransaction3> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).BeginTransaction)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSMQTransaction3>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Properties(&self) -> ::windows::core::Result<super::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Properties)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::IDispatch>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMSMQTransactionDispenser3> for ::windows::core::IUnknown {
    fn from(value: IMSMQTransactionDispenser3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IMSMQTransactionDispenser3> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IMSMQTransactionDispenser3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMSMQTransactionDispenser3> for ::windows::core::IUnknown {
    fn from(value: &IMSMQTransactionDispenser3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMSMQTransactionDispenser3> for super::Com::IDispatch {
    fn from(value: IMSMQTransactionDispenser3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IMSMQTransactionDispenser3> for &'a super::Com::IDispatch {
    fn from(value: &'a IMSMQTransactionDispenser3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMSMQTransactionDispenser3> for super::Com::IDispatch {
    fn from(value: &IMSMQTransactionDispenser3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IMSMQTransactionDispenser3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IMSMQTransactionDispenser3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IMSMQTransactionDispenser3 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IMSMQTransactionDispenser3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMSMQTransactionDispenser3").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IMSMQTransactionDispenser3 {
    type Vtable = IMSMQTransactionDispenser3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeba96b15_2168_11d3_898c_00e02c074f6b);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMSMQTransactionDispenser3_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub BeginTransaction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptransaction: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    BeginTransaction: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcolproperties: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Properties: usize,
}
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const LONG_LIVED: u32 = 4294967294u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MACHINE_ACTION_CONNECT: &str = "CONNECT";
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MACHINE_ACTION_DISCONNECT: &str = "DISCONNECT";
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MACHINE_ACTION_TIDY: &str = "TIDY";
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MGMT_QUEUE_CORRECT_TYPE: &str = "YES";
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MGMT_QUEUE_FOREIGN_TYPE: &str = "YES";
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MGMT_QUEUE_INCORRECT_TYPE: &str = "NO";
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MGMT_QUEUE_LOCAL_LOCATION: &str = "LOCAL";
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MGMT_QUEUE_NOT_FOREIGN_TYPE: &str = "NO";
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MGMT_QUEUE_NOT_TRANSACTIONAL_TYPE: &str = "NO";
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MGMT_QUEUE_REMOTE_LOCATION: &str = "REMOTE";
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MGMT_QUEUE_STATE_CONNECTED: &str = "CONNECTED";
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MGMT_QUEUE_STATE_DISCONNECTED: &str = "DISCONNECTED";
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MGMT_QUEUE_STATE_DISCONNECTING: &str = "DISCONNECTING";
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MGMT_QUEUE_STATE_LOCAL: &str = "LOCAL CONNECTION";
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MGMT_QUEUE_STATE_LOCKED: &str = "LOCKED";
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MGMT_QUEUE_STATE_NEED_VALIDATE: &str = "NEED VALIDATION";
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MGMT_QUEUE_STATE_NONACTIVE: &str = "INACTIVE";
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MGMT_QUEUE_STATE_ONHOLD: &str = "ONHOLD";
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MGMT_QUEUE_STATE_WAITING: &str = "WAITING";
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MGMT_QUEUE_TRANSACTIONAL_TYPE: &str = "YES";
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MGMT_QUEUE_TYPE_CONNECTOR: &str = "CONNECTOR";
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MGMT_QUEUE_TYPE_MACHINE: &str = "MACHINE";
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MGMT_QUEUE_TYPE_MULTICAST: &str = "MULTICAST";
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MGMT_QUEUE_TYPE_PRIVATE: &str = "PRIVATE";
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MGMT_QUEUE_TYPE_PUBLIC: &str = "PUBLIC";
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MGMT_QUEUE_UNKNOWN_TYPE: &str = "UNKNOWN";
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MO_MACHINE_TOKEN: &str = "MACHINE";
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MO_QUEUE_TOKEN: &str = "QUEUE";
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MQACCESS(pub i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_RECEIVE_ACCESS: MQACCESS = MQACCESS(1i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_SEND_ACCESS: MQACCESS = MQACCESS(2i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_PEEK_ACCESS: MQACCESS = MQACCESS(32i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ADMIN_ACCESS: MQACCESS = MQACCESS(128i32);
impl ::core::marker::Copy for MQACCESS {}
impl ::core::clone::Clone for MQACCESS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MQACCESS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MQACCESS {
    type Abi = Self;
}
impl ::core::fmt::Debug for MQACCESS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MQACCESS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MQAUTHENTICATE(pub i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_AUTHENTICATE_NONE: MQAUTHENTICATE = MQAUTHENTICATE(0i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_AUTHENTICATE: MQAUTHENTICATE = MQAUTHENTICATE(1i32);
impl ::core::marker::Copy for MQAUTHENTICATE {}
impl ::core::clone::Clone for MQAUTHENTICATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MQAUTHENTICATE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MQAUTHENTICATE {
    type Abi = Self;
}
impl ::core::fmt::Debug for MQAUTHENTICATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MQAUTHENTICATE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MQCALG(pub i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_CALG_MD2: MQCALG = MQCALG(32769i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_CALG_MD4: MQCALG = MQCALG(32770i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_CALG_MD5: MQCALG = MQCALG(32771i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_CALG_SHA: MQCALG = MQCALG(32772i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_CALG_SHA1: MQCALG = MQCALG(32772i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_CALG_MAC: MQCALG = MQCALG(32773i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_CALG_RSA_SIGN: MQCALG = MQCALG(9216i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_CALG_DSS_SIGN: MQCALG = MQCALG(8704i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_CALG_RSA_KEYX: MQCALG = MQCALG(41984i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_CALG_DES: MQCALG = MQCALG(26113i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_CALG_RC2: MQCALG = MQCALG(26114i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_CALG_RC4: MQCALG = MQCALG(26625i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_CALG_SEAL: MQCALG = MQCALG(26626i32);
impl ::core::marker::Copy for MQCALG {}
impl ::core::clone::Clone for MQCALG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MQCALG {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MQCALG {
    type Abi = Self;
}
impl ::core::fmt::Debug for MQCALG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MQCALG").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MQCERT_REGISTER(pub i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQCERT_REGISTER_ALWAYS: MQCERT_REGISTER = MQCERT_REGISTER(1i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQCERT_REGISTER_IF_NOT_EXIST: MQCERT_REGISTER = MQCERT_REGISTER(2i32);
impl ::core::marker::Copy for MQCERT_REGISTER {}
impl ::core::clone::Clone for MQCERT_REGISTER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MQCERT_REGISTER {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MQCERT_REGISTER {
    type Abi = Self;
}
impl ::core::fmt::Debug for MQCERT_REGISTER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MQCERT_REGISTER").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MQDEFAULT(pub i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const DEFAULT_M_PRIORITY: MQDEFAULT = MQDEFAULT(3i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const DEFAULT_M_DELIVERY: MQDEFAULT = MQDEFAULT(0i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const DEFAULT_M_ACKNOWLEDGE: MQDEFAULT = MQDEFAULT(0i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const DEFAULT_M_JOURNAL: MQDEFAULT = MQDEFAULT(0i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const DEFAULT_M_APPSPECIFIC: MQDEFAULT = MQDEFAULT(0i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const DEFAULT_M_PRIV_LEVEL: MQDEFAULT = MQDEFAULT(0i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const DEFAULT_M_AUTH_LEVEL: MQDEFAULT = MQDEFAULT(0i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const DEFAULT_M_SENDERID_TYPE: MQDEFAULT = MQDEFAULT(1i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const DEFAULT_Q_JOURNAL: MQDEFAULT = MQDEFAULT(0i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const DEFAULT_Q_BASEPRIORITY: MQDEFAULT = MQDEFAULT(0i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const DEFAULT_Q_QUOTA: MQDEFAULT = MQDEFAULT(-1i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const DEFAULT_Q_JOURNAL_QUOTA: MQDEFAULT = MQDEFAULT(-1i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const DEFAULT_Q_TRANSACTION: MQDEFAULT = MQDEFAULT(0i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const DEFAULT_Q_AUTHENTICATE: MQDEFAULT = MQDEFAULT(0i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const DEFAULT_Q_PRIV_LEVEL: MQDEFAULT = MQDEFAULT(1i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const DEFAULT_M_LOOKUPID: MQDEFAULT = MQDEFAULT(0i32);
impl ::core::marker::Copy for MQDEFAULT {}
impl ::core::clone::Clone for MQDEFAULT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MQDEFAULT {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MQDEFAULT {
    type Abi = Self;
}
impl ::core::fmt::Debug for MQDEFAULT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MQDEFAULT").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MQERROR(pub i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR: MQERROR = MQERROR(-1072824319i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_PROPERTY: MQERROR = MQERROR(-1072824318i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_QUEUE_NOT_FOUND: MQERROR = MQERROR(-1072824317i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_QUEUE_NOT_ACTIVE: MQERROR = MQERROR(-1072824316i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_QUEUE_EXISTS: MQERROR = MQERROR(-1072824315i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_INVALID_PARAMETER: MQERROR = MQERROR(-1072824314i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_INVALID_HANDLE: MQERROR = MQERROR(-1072824313i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_OPERATION_CANCELLED: MQERROR = MQERROR(-1072824312i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_SHARING_VIOLATION: MQERROR = MQERROR(-1072824311i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_SERVICE_NOT_AVAILABLE: MQERROR = MQERROR(-1072824309i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_MACHINE_NOT_FOUND: MQERROR = MQERROR(-1072824307i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_ILLEGAL_SORT: MQERROR = MQERROR(-1072824304i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_ILLEGAL_USER: MQERROR = MQERROR(-1072824303i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_NO_DS: MQERROR = MQERROR(-1072824301i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_ILLEGAL_QUEUE_PATHNAME: MQERROR = MQERROR(-1072824300i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_ILLEGAL_PROPERTY_VALUE: MQERROR = MQERROR(-1072824296i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_ILLEGAL_PROPERTY_VT: MQERROR = MQERROR(-1072824295i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_BUFFER_OVERFLOW: MQERROR = MQERROR(-1072824294i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_IO_TIMEOUT: MQERROR = MQERROR(-1072824293i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_ILLEGAL_CURSOR_ACTION: MQERROR = MQERROR(-1072824292i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_MESSAGE_ALREADY_RECEIVED: MQERROR = MQERROR(-1072824291i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_ILLEGAL_FORMATNAME: MQERROR = MQERROR(-1072824290i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_FORMATNAME_BUFFER_TOO_SMALL: MQERROR = MQERROR(-1072824289i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_UNSUPPORTED_FORMATNAME_OPERATION: MQERROR = MQERROR(-1072824288i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_ILLEGAL_SECURITY_DESCRIPTOR: MQERROR = MQERROR(-1072824287i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_SENDERID_BUFFER_TOO_SMALL: MQERROR = MQERROR(-1072824286i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_SECURITY_DESCRIPTOR_TOO_SMALL: MQERROR = MQERROR(-1072824285i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_CANNOT_IMPERSONATE_CLIENT: MQERROR = MQERROR(-1072824284i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_ACCESS_DENIED: MQERROR = MQERROR(-1072824283i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_PRIVILEGE_NOT_HELD: MQERROR = MQERROR(-1072824282i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_INSUFFICIENT_RESOURCES: MQERROR = MQERROR(-1072824281i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_USER_BUFFER_TOO_SMALL: MQERROR = MQERROR(-1072824280i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_MESSAGE_STORAGE_FAILED: MQERROR = MQERROR(-1072824278i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_SENDER_CERT_BUFFER_TOO_SMALL: MQERROR = MQERROR(-1072824277i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_INVALID_CERTIFICATE: MQERROR = MQERROR(-1072824276i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_CORRUPTED_INTERNAL_CERTIFICATE: MQERROR = MQERROR(-1072824275i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_INTERNAL_USER_CERT_EXIST: MQERROR = MQERROR(-1072824274i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_NO_INTERNAL_USER_CERT: MQERROR = MQERROR(-1072824273i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_CORRUPTED_SECURITY_DATA: MQERROR = MQERROR(-1072824272i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_CORRUPTED_PERSONAL_CERT_STORE: MQERROR = MQERROR(-1072824271i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_COMPUTER_DOES_NOT_SUPPORT_ENCRYPTION: MQERROR = MQERROR(-1072824269i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_BAD_SECURITY_CONTEXT: MQERROR = MQERROR(-1072824267i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_COULD_NOT_GET_USER_SID: MQERROR = MQERROR(-1072824266i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_COULD_NOT_GET_ACCOUNT_INFO: MQERROR = MQERROR(-1072824265i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_ILLEGAL_MQCOLUMNS: MQERROR = MQERROR(-1072824264i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_ILLEGAL_PROPID: MQERROR = MQERROR(-1072824263i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_ILLEGAL_RELATION: MQERROR = MQERROR(-1072824262i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_ILLEGAL_PROPERTY_SIZE: MQERROR = MQERROR(-1072824261i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_ILLEGAL_RESTRICTION_PROPID: MQERROR = MQERROR(-1072824260i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_ILLEGAL_MQQUEUEPROPS: MQERROR = MQERROR(-1072824259i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_PROPERTY_NOTALLOWED: MQERROR = MQERROR(-1072824258i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_INSUFFICIENT_PROPERTIES: MQERROR = MQERROR(-1072824257i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_MACHINE_EXISTS: MQERROR = MQERROR(-1072824256i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_ILLEGAL_MQQMPROPS: MQERROR = MQERROR(-1072824255i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_DS_IS_FULL: MQERROR = MQERROR(-1072824254i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_DS_ERROR: MQERROR = MQERROR(-1072824253i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_INVALID_OWNER: MQERROR = MQERROR(-1072824252i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_UNSUPPORTED_ACCESS_MODE: MQERROR = MQERROR(-1072824251i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_RESULT_BUFFER_TOO_SMALL: MQERROR = MQERROR(-1072824250i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_DELETE_CN_IN_USE: MQERROR = MQERROR(-1072824248i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_NO_RESPONSE_FROM_OBJECT_SERVER: MQERROR = MQERROR(-1072824247i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_OBJECT_SERVER_NOT_AVAILABLE: MQERROR = MQERROR(-1072824246i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_QUEUE_NOT_AVAILABLE: MQERROR = MQERROR(-1072824245i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_DTC_CONNECT: MQERROR = MQERROR(-1072824244i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_TRANSACTION_IMPORT: MQERROR = MQERROR(-1072824242i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_TRANSACTION_USAGE: MQERROR = MQERROR(-1072824240i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_TRANSACTION_SEQUENCE: MQERROR = MQERROR(-1072824239i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_MISSING_CONNECTOR_TYPE: MQERROR = MQERROR(-1072824235i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_STALE_HANDLE: MQERROR = MQERROR(-1072824234i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_TRANSACTION_ENLIST: MQERROR = MQERROR(-1072824232i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_QUEUE_DELETED: MQERROR = MQERROR(-1072824230i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_ILLEGAL_CONTEXT: MQERROR = MQERROR(-1072824229i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_ILLEGAL_SORT_PROPID: MQERROR = MQERROR(-1072824228i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_LABEL_TOO_LONG: MQERROR = MQERROR(-1072824227i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_LABEL_BUFFER_TOO_SMALL: MQERROR = MQERROR(-1072824226i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_MQIS_SERVER_EMPTY: MQERROR = MQERROR(-1072824225i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_MQIS_READONLY_MODE: MQERROR = MQERROR(-1072824224i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_SYMM_KEY_BUFFER_TOO_SMALL: MQERROR = MQERROR(-1072824223i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_SIGNATURE_BUFFER_TOO_SMALL: MQERROR = MQERROR(-1072824222i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_PROV_NAME_BUFFER_TOO_SMALL: MQERROR = MQERROR(-1072824221i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_ILLEGAL_OPERATION: MQERROR = MQERROR(-1072824220i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_WRITE_NOT_ALLOWED: MQERROR = MQERROR(-1072824219i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_WKS_CANT_SERVE_CLIENT: MQERROR = MQERROR(-1072824218i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_DEPEND_WKS_LICENSE_OVERFLOW: MQERROR = MQERROR(-1072824217i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_CORRUPTED_QUEUE_WAS_DELETED: MQERROR = MQERROR(-1072824216i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_REMOTE_MACHINE_NOT_AVAILABLE: MQERROR = MQERROR(-1072824215i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_UNSUPPORTED_OPERATION: MQERROR = MQERROR(-1072824214i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_ENCRYPTION_PROVIDER_NOT_SUPPORTED: MQERROR = MQERROR(-1072824213i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_CANNOT_SET_CRYPTO_SEC_DESCR: MQERROR = MQERROR(-1072824212i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_CERTIFICATE_NOT_PROVIDED: MQERROR = MQERROR(-1072824211i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_Q_DNS_PROPERTY_NOT_SUPPORTED: MQERROR = MQERROR(-1072824210i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_CANT_CREATE_CERT_STORE: MQERROR = MQERROR(-1072824209i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_CANNOT_CREATE_CERT_STORE: MQERROR = MQERROR(-1072824209i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_CANT_OPEN_CERT_STORE: MQERROR = MQERROR(-1072824208i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_CANNOT_OPEN_CERT_STORE: MQERROR = MQERROR(-1072824208i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_ILLEGAL_ENTERPRISE_OPERATION: MQERROR = MQERROR(-1072824207i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_CANNOT_GRANT_ADD_GUID: MQERROR = MQERROR(-1072824206i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_CANNOT_LOAD_MSMQOCM: MQERROR = MQERROR(-1072824205i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_NO_ENTRY_POINT_MSMQOCM: MQERROR = MQERROR(-1072824204i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_NO_MSMQ_SERVERS_ON_DC: MQERROR = MQERROR(-1072824203i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_CANNOT_JOIN_DOMAIN: MQERROR = MQERROR(-1072824202i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_CANNOT_CREATE_ON_GC: MQERROR = MQERROR(-1072824201i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_GUID_NOT_MATCHING: MQERROR = MQERROR(-1072824200i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_PUBLIC_KEY_NOT_FOUND: MQERROR = MQERROR(-1072824199i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_PUBLIC_KEY_DOES_NOT_EXIST: MQERROR = MQERROR(-1072824198i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_ILLEGAL_MQPRIVATEPROPS: MQERROR = MQERROR(-1072824197i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_NO_GC_IN_DOMAIN: MQERROR = MQERROR(-1072824196i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_NO_MSMQ_SERVERS_ON_GC: MQERROR = MQERROR(-1072824195i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_CANNOT_GET_DN: MQERROR = MQERROR(-1072824194i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_CANNOT_HASH_DATA_EX: MQERROR = MQERROR(-1072824193i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_CANNOT_SIGN_DATA_EX: MQERROR = MQERROR(-1072824192i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_CANNOT_CREATE_HASH_EX: MQERROR = MQERROR(-1072824191i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_FAIL_VERIFY_SIGNATURE_EX: MQERROR = MQERROR(-1072824190i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_CANNOT_DELETE_PSC_OBJECTS: MQERROR = MQERROR(-1072824189i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_NO_MQUSER_OU: MQERROR = MQERROR(-1072824188i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_CANNOT_LOAD_MQAD: MQERROR = MQERROR(-1072824187i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_CANNOT_LOAD_MQDSSRV: MQERROR = MQERROR(-1072824186i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_PROPERTIES_CONFLICT: MQERROR = MQERROR(-1072824185i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_MESSAGE_NOT_FOUND: MQERROR = MQERROR(-1072824184i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_CANT_RESOLVE_SITES: MQERROR = MQERROR(-1072824183i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_NOT_SUPPORTED_BY_DEPENDENT_CLIENTS: MQERROR = MQERROR(-1072824182i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_OPERATION_NOT_SUPPORTED_BY_REMOTE_COMPUTER: MQERROR = MQERROR(-1072824181i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_NOT_A_CORRECT_OBJECT_CLASS: MQERROR = MQERROR(-1072824180i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_MULTI_SORT_KEYS: MQERROR = MQERROR(-1072824179i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_GC_NEEDED: MQERROR = MQERROR(-1072824178i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_DS_BIND_ROOT_FOREST: MQERROR = MQERROR(-1072824177i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_DS_LOCAL_USER: MQERROR = MQERROR(-1072824176i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_Q_ADS_PROPERTY_NOT_SUPPORTED: MQERROR = MQERROR(-1072824175i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_BAD_XML_FORMAT: MQERROR = MQERROR(-1072824174i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_UNSUPPORTED_CLASS: MQERROR = MQERROR(-1072824173i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_UNINITIALIZED_OBJECT: MQERROR = MQERROR(-1072824172i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_CANNOT_CREATE_PSC_OBJECTS: MQERROR = MQERROR(-1072824171i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_CANNOT_UPDATE_PSC_OBJECTS: MQERROR = MQERROR(-1072824170i32);
impl ::core::marker::Copy for MQERROR {}
impl ::core::clone::Clone for MQERROR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MQERROR {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MQERROR {
    type Abi = Self;
}
impl ::core::fmt::Debug for MQERROR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MQERROR").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MQJOURNAL(pub i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_JOURNAL_NONE: MQJOURNAL = MQJOURNAL(0i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_JOURNAL: MQJOURNAL = MQJOURNAL(1i32);
impl ::core::marker::Copy for MQJOURNAL {}
impl ::core::clone::Clone for MQJOURNAL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MQJOURNAL {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MQJOURNAL {
    type Abi = Self;
}
impl ::core::fmt::Debug for MQJOURNAL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MQJOURNAL").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MQMAX(pub i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_MAX_Q_NAME_LEN: MQMAX = MQMAX(124i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_MAX_Q_LABEL_LEN: MQMAX = MQMAX(124i32);
impl ::core::marker::Copy for MQMAX {}
impl ::core::clone::Clone for MQMAX {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MQMAX {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MQMAX {
    type Abi = Self;
}
impl ::core::fmt::Debug for MQMAX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MQMAX").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MQMSGACKNOWLEDGEMENT(pub i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_ACKNOWLEDGMENT_NONE: MQMSGACKNOWLEDGEMENT = MQMSGACKNOWLEDGEMENT(0i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_ACKNOWLEDGMENT_POS_ARRIVAL: MQMSGACKNOWLEDGEMENT = MQMSGACKNOWLEDGEMENT(1i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_ACKNOWLEDGMENT_POS_RECEIVE: MQMSGACKNOWLEDGEMENT = MQMSGACKNOWLEDGEMENT(2i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_ACKNOWLEDGMENT_NEG_ARRIVAL: MQMSGACKNOWLEDGEMENT = MQMSGACKNOWLEDGEMENT(4i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_ACKNOWLEDGMENT_NEG_RECEIVE: MQMSGACKNOWLEDGEMENT = MQMSGACKNOWLEDGEMENT(8i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_ACKNOWLEDGMENT_NACK_REACH_QUEUE: MQMSGACKNOWLEDGEMENT = MQMSGACKNOWLEDGEMENT(4i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_ACKNOWLEDGMENT_FULL_REACH_QUEUE: MQMSGACKNOWLEDGEMENT = MQMSGACKNOWLEDGEMENT(5i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_ACKNOWLEDGMENT_NACK_RECEIVE: MQMSGACKNOWLEDGEMENT = MQMSGACKNOWLEDGEMENT(12i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_ACKNOWLEDGMENT_FULL_RECEIVE: MQMSGACKNOWLEDGEMENT = MQMSGACKNOWLEDGEMENT(14i32);
impl ::core::marker::Copy for MQMSGACKNOWLEDGEMENT {}
impl ::core::clone::Clone for MQMSGACKNOWLEDGEMENT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MQMSGACKNOWLEDGEMENT {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MQMSGACKNOWLEDGEMENT {
    type Abi = Self;
}
impl ::core::fmt::Debug for MQMSGACKNOWLEDGEMENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MQMSGACKNOWLEDGEMENT").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MQMSGAUTHENTICATION(pub i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_AUTHENTICATION_NOT_REQUESTED: MQMSGAUTHENTICATION = MQMSGAUTHENTICATION(0i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_AUTHENTICATION_REQUESTED: MQMSGAUTHENTICATION = MQMSGAUTHENTICATION(1i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_AUTHENTICATED_SIG10: MQMSGAUTHENTICATION = MQMSGAUTHENTICATION(1i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_AUTHENTICATION_REQUESTED_EX: MQMSGAUTHENTICATION = MQMSGAUTHENTICATION(3i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_AUTHENTICATED_SIG20: MQMSGAUTHENTICATION = MQMSGAUTHENTICATION(3i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_AUTHENTICATED_SIG30: MQMSGAUTHENTICATION = MQMSGAUTHENTICATION(5i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_AUTHENTICATED_SIGXML: MQMSGAUTHENTICATION = MQMSGAUTHENTICATION(9i32);
impl ::core::marker::Copy for MQMSGAUTHENTICATION {}
impl ::core::clone::Clone for MQMSGAUTHENTICATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MQMSGAUTHENTICATION {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MQMSGAUTHENTICATION {
    type Abi = Self;
}
impl ::core::fmt::Debug for MQMSGAUTHENTICATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MQMSGAUTHENTICATION").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MQMSGAUTHLEVEL(pub i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_AUTH_LEVEL_NONE: MQMSGAUTHLEVEL = MQMSGAUTHLEVEL(0i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_AUTH_LEVEL_ALWAYS: MQMSGAUTHLEVEL = MQMSGAUTHLEVEL(1i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_AUTH_LEVEL_MSMQ10: MQMSGAUTHLEVEL = MQMSGAUTHLEVEL(2i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_AUTH_LEVEL_SIG10: MQMSGAUTHLEVEL = MQMSGAUTHLEVEL(2i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_AUTH_LEVEL_MSMQ20: MQMSGAUTHLEVEL = MQMSGAUTHLEVEL(4i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_AUTH_LEVEL_SIG20: MQMSGAUTHLEVEL = MQMSGAUTHLEVEL(4i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_AUTH_LEVEL_SIG30: MQMSGAUTHLEVEL = MQMSGAUTHLEVEL(8i32);
impl ::core::marker::Copy for MQMSGAUTHLEVEL {}
impl ::core::clone::Clone for MQMSGAUTHLEVEL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MQMSGAUTHLEVEL {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MQMSGAUTHLEVEL {
    type Abi = Self;
}
impl ::core::fmt::Debug for MQMSGAUTHLEVEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MQMSGAUTHLEVEL").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MQMSGCLASS(pub i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_CLASS_NORMAL: MQMSGCLASS = MQMSGCLASS(0i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_CLASS_REPORT: MQMSGCLASS = MQMSGCLASS(1i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_CLASS_ACK_REACH_QUEUE: MQMSGCLASS = MQMSGCLASS(2i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_CLASS_ACK_RECEIVE: MQMSGCLASS = MQMSGCLASS(16384i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_CLASS_NACK_BAD_DST_Q: MQMSGCLASS = MQMSGCLASS(32768i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_CLASS_NACK_PURGED: MQMSGCLASS = MQMSGCLASS(32769i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_CLASS_NACK_REACH_QUEUE_TIMEOUT: MQMSGCLASS = MQMSGCLASS(32770i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_CLASS_NACK_Q_EXCEED_QUOTA: MQMSGCLASS = MQMSGCLASS(32771i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_CLASS_NACK_ACCESS_DENIED: MQMSGCLASS = MQMSGCLASS(32772i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_CLASS_NACK_HOP_COUNT_EXCEEDED: MQMSGCLASS = MQMSGCLASS(32773i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_CLASS_NACK_BAD_SIGNATURE: MQMSGCLASS = MQMSGCLASS(32774i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_CLASS_NACK_BAD_ENCRYPTION: MQMSGCLASS = MQMSGCLASS(32775i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_CLASS_NACK_COULD_NOT_ENCRYPT: MQMSGCLASS = MQMSGCLASS(32776i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_CLASS_NACK_NOT_TRANSACTIONAL_Q: MQMSGCLASS = MQMSGCLASS(32777i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_CLASS_NACK_NOT_TRANSACTIONAL_MSG: MQMSGCLASS = MQMSGCLASS(32778i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_CLASS_NACK_UNSUPPORTED_CRYPTO_PROVIDER: MQMSGCLASS = MQMSGCLASS(32779i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_CLASS_NACK_SOURCE_COMPUTER_GUID_CHANGED: MQMSGCLASS = MQMSGCLASS(32780i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_CLASS_NACK_Q_DELETED: MQMSGCLASS = MQMSGCLASS(49152i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_CLASS_NACK_Q_PURGED: MQMSGCLASS = MQMSGCLASS(49153i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_CLASS_NACK_RECEIVE_TIMEOUT: MQMSGCLASS = MQMSGCLASS(49154i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_CLASS_NACK_RECEIVE_TIMEOUT_AT_SENDER: MQMSGCLASS = MQMSGCLASS(49155i32);
impl ::core::marker::Copy for MQMSGCLASS {}
impl ::core::clone::Clone for MQMSGCLASS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MQMSGCLASS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MQMSGCLASS {
    type Abi = Self;
}
impl ::core::fmt::Debug for MQMSGCLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MQMSGCLASS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MQMSGCURSOR(pub i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_FIRST: MQMSGCURSOR = MQMSGCURSOR(0i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_CURRENT: MQMSGCURSOR = MQMSGCURSOR(1i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_NEXT: MQMSGCURSOR = MQMSGCURSOR(2i32);
impl ::core::marker::Copy for MQMSGCURSOR {}
impl ::core::clone::Clone for MQMSGCURSOR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MQMSGCURSOR {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MQMSGCURSOR {
    type Abi = Self;
}
impl ::core::fmt::Debug for MQMSGCURSOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MQMSGCURSOR").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MQMSGDELIVERY(pub i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_DELIVERY_EXPRESS: MQMSGDELIVERY = MQMSGDELIVERY(0i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_DELIVERY_RECOVERABLE: MQMSGDELIVERY = MQMSGDELIVERY(1i32);
impl ::core::marker::Copy for MQMSGDELIVERY {}
impl ::core::clone::Clone for MQMSGDELIVERY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MQMSGDELIVERY {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MQMSGDELIVERY {
    type Abi = Self;
}
impl ::core::fmt::Debug for MQMSGDELIVERY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MQMSGDELIVERY").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MQMSGIDSIZE(pub i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_MSGID_SIZE: MQMSGIDSIZE = MQMSGIDSIZE(20i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_CORRELATIONID_SIZE: MQMSGIDSIZE = MQMSGIDSIZE(20i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_XACTID_SIZE: MQMSGIDSIZE = MQMSGIDSIZE(20i32);
impl ::core::marker::Copy for MQMSGIDSIZE {}
impl ::core::clone::Clone for MQMSGIDSIZE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MQMSGIDSIZE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MQMSGIDSIZE {
    type Abi = Self;
}
impl ::core::fmt::Debug for MQMSGIDSIZE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MQMSGIDSIZE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MQMSGJOURNAL(pub i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_JOURNAL_NONE: MQMSGJOURNAL = MQMSGJOURNAL(0i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_DEADLETTER: MQMSGJOURNAL = MQMSGJOURNAL(1i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_JOURNAL: MQMSGJOURNAL = MQMSGJOURNAL(2i32);
impl ::core::marker::Copy for MQMSGJOURNAL {}
impl ::core::clone::Clone for MQMSGJOURNAL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MQMSGJOURNAL {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MQMSGJOURNAL {
    type Abi = Self;
}
impl ::core::fmt::Debug for MQMSGJOURNAL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MQMSGJOURNAL").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MQMSGMAX(pub i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_MAX_MSG_LABEL_LEN: MQMSGMAX = MQMSGMAX(249i32);
impl ::core::marker::Copy for MQMSGMAX {}
impl ::core::clone::Clone for MQMSGMAX {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MQMSGMAX {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MQMSGMAX {
    type Abi = Self;
}
impl ::core::fmt::Debug for MQMSGMAX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MQMSGMAX").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MQMSGPRIVLEVEL(pub i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_PRIV_LEVEL_NONE: MQMSGPRIVLEVEL = MQMSGPRIVLEVEL(0i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_PRIV_LEVEL_BODY_BASE: MQMSGPRIVLEVEL = MQMSGPRIVLEVEL(1i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_PRIV_LEVEL_BODY_ENHANCED: MQMSGPRIVLEVEL = MQMSGPRIVLEVEL(3i32);
impl ::core::marker::Copy for MQMSGPRIVLEVEL {}
impl ::core::clone::Clone for MQMSGPRIVLEVEL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MQMSGPRIVLEVEL {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MQMSGPRIVLEVEL {
    type Abi = Self;
}
impl ::core::fmt::Debug for MQMSGPRIVLEVEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MQMSGPRIVLEVEL").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MQMSGSENDERIDTYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_SENDERID_TYPE_NONE: MQMSGSENDERIDTYPE = MQMSGSENDERIDTYPE(0i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_SENDERID_TYPE_SID: MQMSGSENDERIDTYPE = MQMSGSENDERIDTYPE(1i32);
impl ::core::marker::Copy for MQMSGSENDERIDTYPE {}
impl ::core::clone::Clone for MQMSGSENDERIDTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MQMSGSENDERIDTYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MQMSGSENDERIDTYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for MQMSGSENDERIDTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MQMSGSENDERIDTYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MQMSGTRACE(pub i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_TRACE_NONE: MQMSGTRACE = MQMSGTRACE(0i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_SEND_ROUTE_TO_REPORT_QUEUE: MQMSGTRACE = MQMSGTRACE(1i32);
impl ::core::marker::Copy for MQMSGTRACE {}
impl ::core::clone::Clone for MQMSGTRACE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MQMSGTRACE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MQMSGTRACE {
    type Abi = Self;
}
impl ::core::fmt::Debug for MQMSGTRACE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MQMSGTRACE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_AUTHENTICATED_QM_MESSAGE: u32 = 11u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_FIRST_IN_XACT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_LAST_IN_XACT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_NOT_FIRST_IN_XACT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_NOT_LAST_IN_XACT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQMSG_PRIV_LEVEL_BODY_AES: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MQPRIORITY(pub i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_MIN_PRIORITY: MQPRIORITY = MQPRIORITY(0i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_MAX_PRIORITY: MQPRIORITY = MQPRIORITY(7i32);
impl ::core::marker::Copy for MQPRIORITY {}
impl ::core::clone::Clone for MQPRIORITY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MQPRIORITY {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MQPRIORITY {
    type Abi = Self;
}
impl ::core::fmt::Debug for MQPRIORITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MQPRIORITY").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MQPRIVLEVEL(pub i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_PRIV_LEVEL_NONE: MQPRIVLEVEL = MQPRIVLEVEL(0i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_PRIV_LEVEL_OPTIONAL: MQPRIVLEVEL = MQPRIVLEVEL(1i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_PRIV_LEVEL_BODY: MQPRIVLEVEL = MQPRIVLEVEL(2i32);
impl ::core::marker::Copy for MQPRIVLEVEL {}
impl ::core::clone::Clone for MQPRIVLEVEL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MQPRIVLEVEL {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MQPRIVLEVEL {
    type Abi = Self;
}
impl ::core::fmt::Debug for MQPRIVLEVEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MQPRIVLEVEL").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQSEC_CHANGE_QUEUE_PERMISSIONS: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQSEC_DELETE_JOURNAL_MESSAGE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQSEC_DELETE_MESSAGE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQSEC_DELETE_QUEUE: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQSEC_GET_QUEUE_PROPERTIES: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQSEC_PEEK_MESSAGE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQSEC_QUEUE_GENERIC_EXECUTE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQSEC_SET_QUEUE_PROPERTIES: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQSEC_TAKE_QUEUE_OWNERSHIP: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQSEC_WRITE_MESSAGE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MQSHARE(pub i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_DENY_NONE: MQSHARE = MQSHARE(0i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_DENY_RECEIVE_SHARE: MQSHARE = MQSHARE(1i32);
impl ::core::marker::Copy for MQSHARE {}
impl ::core::clone::Clone for MQSHARE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MQSHARE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MQSHARE {
    type Abi = Self;
}
impl ::core::fmt::Debug for MQSHARE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MQSHARE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MQTRANSACTION(pub i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_NO_TRANSACTION: MQTRANSACTION = MQTRANSACTION(0i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_MTS_TRANSACTION: MQTRANSACTION = MQTRANSACTION(1i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_XA_TRANSACTION: MQTRANSACTION = MQTRANSACTION(2i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_SINGLE_MESSAGE: MQTRANSACTION = MQTRANSACTION(3i32);
impl ::core::marker::Copy for MQTRANSACTION {}
impl ::core::clone::Clone for MQTRANSACTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MQTRANSACTION {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MQTRANSACTION {
    type Abi = Self;
}
impl ::core::fmt::Debug for MQTRANSACTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MQTRANSACTION").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MQTRANSACTIONAL(pub i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_TRANSACTIONAL_NONE: MQTRANSACTIONAL = MQTRANSACTIONAL(0i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_TRANSACTIONAL: MQTRANSACTIONAL = MQTRANSACTIONAL(1i32);
impl ::core::marker::Copy for MQTRANSACTIONAL {}
impl ::core::clone::Clone for MQTRANSACTIONAL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MQTRANSACTIONAL {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MQTRANSACTIONAL {
    type Abi = Self;
}
impl ::core::fmt::Debug for MQTRANSACTIONAL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MQTRANSACTIONAL").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MQWARNING(pub i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_INFORMATION_PROPERTY: MQWARNING = MQWARNING(1074659329i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_INFORMATION_ILLEGAL_PROPERTY: MQWARNING = MQWARNING(1074659330i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_INFORMATION_PROPERTY_IGNORED: MQWARNING = MQWARNING(1074659331i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_INFORMATION_UNSUPPORTED_PROPERTY: MQWARNING = MQWARNING(1074659332i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_INFORMATION_DUPLICATE_PROPERTY: MQWARNING = MQWARNING(1074659333i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_INFORMATION_OPERATION_PENDING: MQWARNING = MQWARNING(1074659334i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_INFORMATION_FORMATNAME_BUFFER_TOO_SMALL: MQWARNING = MQWARNING(1074659337i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_INFORMATION_INTERNAL_USER_CERT_EXIST: MQWARNING = MQWARNING(1074659338i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_INFORMATION_OWNER_IGNORED: MQWARNING = MQWARNING(1074659339i32);
impl ::core::marker::Copy for MQWARNING {}
impl ::core::clone::Clone for MQWARNING {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MQWARNING {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MQWARNING {
    type Abi = Self;
}
impl ::core::fmt::Debug for MQWARNING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MQWARNING").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ACTION_PEEK_CURRENT: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ACTION_PEEK_NEXT: u32 = 2147483649u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ACTION_RECEIVE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_MESSAGE_LOCKED_UNDER_TRANSACTION: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072824164i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_MESSAGE_NOT_AUTHENTICATED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072824165i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_RESOLVE_ADDRESS: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072824167i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_ERROR_TOO_MANY_PROPERTIES: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072824166i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_LOOKUP_PEEK_CURRENT: u32 = 1073741840u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_LOOKUP_PEEK_FIRST: u32 = 1073741844u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_LOOKUP_PEEK_LAST: u32 = 1073741848u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_LOOKUP_PEEK_NEXT: u32 = 1073741841u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_LOOKUP_PEEK_PREV: u32 = 1073741842u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_LOOKUP_RECEIVE_ALLOW_PEEK: u32 = 1073742112u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_LOOKUP_RECEIVE_CURRENT: u32 = 1073741856u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_LOOKUP_RECEIVE_FIRST: u32 = 1073741860u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_LOOKUP_RECEIVE_LAST: u32 = 1073741864u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_LOOKUP_RECEIVE_NEXT: u32 = 1073741857u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_LOOKUP_RECEIVE_PREV: u32 = 1073741858u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_MOVE_ACCESS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_OK: ::windows::core::HRESULT = ::windows::core::HRESULT(0i32);
pub const MSMQApplication: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd7d6e086_dccd_11d0_aa4b_0060970debae);
pub const MSMQCollection: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf72b9031_2f0c_43e8_924e_e6052cdc493f);
pub const MSMQCoordinatedTransactionDispenser: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd7d6e082_dccd_11d0_aa4b_0060970debae);
pub const MSMQDestination: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeba96b18_2168_11d3_898c_00e02c074f6b);
pub const MSMQEvent: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd7d6e07a_dccd_11d0_aa4b_0060970debae);
pub const MSMQManagement: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x39ce96fe_f4c5_4484_a143_4c2d5d324229);
pub const MSMQMessage: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd7d6e075_dccd_11d0_aa4b_0060970debae);
pub const MSMQOutgoingQueueManagement: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0188401c_247a_4fed_99c6_bf14119d7055);
pub const MSMQQuery: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd7d6e073_dccd_11d0_aa4b_0060970debae);
pub const MSMQQueue: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd7d6e079_dccd_11d0_aa4b_0060970debae);
pub const MSMQQueueInfo: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd7d6e07c_dccd_11d0_aa4b_0060970debae);
pub const MSMQQueueInfos: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd7d6e07e_dccd_11d0_aa4b_0060970debae);
pub const MSMQQueueManagement: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x33b6d07e_f27d_42fa_b2d7_bf82e11e9374);
pub const MSMQTransaction: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd7d6e080_dccd_11d0_aa4b_0060970debae);
pub const MSMQTransactionDispenser: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd7d6e084_dccd_11d0_aa4b_0060970debae);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MSMQ_CONNECTED: &str = "CONNECTED";
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MSMQ_DISCONNECTED: &str = "DISCONNECTED";
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PREQ: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PRGE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PRGT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PRLE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PRLT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PRNE: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_MGMT_MSMQ_ACTIVEQUEUES: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_MGMT_MSMQ_BASE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_MGMT_MSMQ_BYTES_IN_ALL_QUEUES: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_MGMT_MSMQ_CONNECTED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_MGMT_MSMQ_DSSERVER: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_MGMT_MSMQ_PRIVATEQ: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_MGMT_MSMQ_TYPE: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_MGMT_QUEUE_BASE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_MGMT_QUEUE_BYTES_IN_JOURNAL: u32 = 10u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_MGMT_QUEUE_BYTES_IN_QUEUE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_MGMT_QUEUE_CONNECTION_HISTORY: u32 = 25u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_MGMT_QUEUE_EOD_FIRST_NON_ACK: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_MGMT_QUEUE_EOD_LAST_ACK: u32 = 13u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_MGMT_QUEUE_EOD_LAST_ACK_COUNT: u32 = 15u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_MGMT_QUEUE_EOD_LAST_ACK_TIME: u32 = 14u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_MGMT_QUEUE_EOD_LAST_NON_ACK: u32 = 17u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_MGMT_QUEUE_EOD_NEXT_SEQ: u32 = 18u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_MGMT_QUEUE_EOD_NO_ACK_COUNT: u32 = 20u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_MGMT_QUEUE_EOD_NO_READ_COUNT: u32 = 19u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_MGMT_QUEUE_EOD_RESEND_COUNT: u32 = 23u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_MGMT_QUEUE_EOD_RESEND_INTERVAL: u32 = 22u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_MGMT_QUEUE_EOD_RESEND_TIME: u32 = 21u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_MGMT_QUEUE_EOD_SOURCE_INFO: u32 = 24u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_MGMT_QUEUE_FOREIGN: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_MGMT_QUEUE_FORMATNAME: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_MGMT_QUEUE_JOURNAL_MESSAGE_COUNT: u32 = 9u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_MGMT_QUEUE_JOURNAL_USED_QUOTA: u32 = 10u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_MGMT_QUEUE_LOCATION: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_MGMT_QUEUE_MESSAGE_COUNT: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_MGMT_QUEUE_NEXTHOPS: u32 = 12u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_MGMT_QUEUE_PATHNAME: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_MGMT_QUEUE_STATE: u32 = 11u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_MGMT_QUEUE_SUBQUEUE_COUNT: u32 = 26u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_MGMT_QUEUE_SUBQUEUE_NAMES: u32 = 27u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_MGMT_QUEUE_TYPE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_MGMT_QUEUE_USED_QUOTA: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_MGMT_QUEUE_XACT: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_M_ABORT_COUNT: u32 = 69u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_M_ACKNOWLEDGE: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_M_ADMIN_QUEUE: u32 = 17u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_M_ADMIN_QUEUE_LEN: u32 = 18u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_M_APPSPECIFIC: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_M_ARRIVEDTIME: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_M_AUTHENTICATED: u32 = 25u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_M_AUTHENTICATED_EX: u32 = 53u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_M_AUTH_LEVEL: u32 = 24u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_M_BASE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_M_BODY: u32 = 9u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_M_BODY_SIZE: u32 = 10u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_M_BODY_TYPE: u32 = 42u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_M_CLASS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_M_COMPOUND_MESSAGE: u32 = 63u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_M_COMPOUND_MESSAGE_SIZE: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_M_CONNECTOR_TYPE: u32 = 38u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_M_CORRELATIONID: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_M_CORRELATIONID_SIZE: u32 = 20u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_M_DEADLETTER_QUEUE: u32 = 67u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_M_DEADLETTER_QUEUE_LEN: u32 = 68u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_M_DELIVERY: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_M_DEST_FORMAT_NAME: u32 = 58u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_M_DEST_FORMAT_NAME_LEN: u32 = 59u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_M_DEST_QUEUE: u32 = 33u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_M_DEST_QUEUE_LEN: u32 = 34u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_M_DEST_SYMM_KEY: u32 = 43u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_M_DEST_SYMM_KEY_LEN: u32 = 44u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_M_ENCRYPTION_ALG: u32 = 27u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_M_EXTENSION: u32 = 35u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_M_EXTENSION_LEN: u32 = 36u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_M_FIRST_IN_XACT: u32 = 50u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_M_HASH_ALG: u32 = 26u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_M_JOURNAL: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_M_LABEL: u32 = 11u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_M_LABEL_LEN: u32 = 12u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_M_LAST_IN_XACT: u32 = 51u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_M_LAST_MOVE_TIME: u32 = 75u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_M_LOOKUPID: u32 = 60u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_M_MOVE_COUNT: u32 = 70u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_M_MSGID: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_M_MSGID_SIZE: u32 = 20u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_M_PRIORITY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_M_PRIV_LEVEL: u32 = 23u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_M_PROV_NAME: u32 = 48u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_M_PROV_NAME_LEN: u32 = 49u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_M_PROV_TYPE: u32 = 47u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_M_RESP_FORMAT_NAME: u32 = 54u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_M_RESP_FORMAT_NAME_LEN: u32 = 55u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_M_RESP_QUEUE: u32 = 15u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_M_RESP_QUEUE_LEN: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_M_SECURITY_CONTEXT: u32 = 37u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_M_SENDERID: u32 = 20u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_M_SENDERID_LEN: u32 = 21u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_M_SENDERID_TYPE: u32 = 22u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_M_SENDER_CERT: u32 = 28u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_M_SENDER_CERT_LEN: u32 = 29u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_M_SENTTIME: u32 = 31u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_M_SIGNATURE: u32 = 45u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_M_SIGNATURE_LEN: u32 = 46u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_M_SOAP_BODY: u32 = 66u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_M_SOAP_ENVELOPE: u32 = 61u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_M_SOAP_ENVELOPE_LEN: u32 = 62u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_M_SOAP_HEADER: u32 = 65u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_M_SRC_MACHINE_ID: u32 = 30u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_M_TIME_TO_BE_RECEIVED: u32 = 14u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_M_TIME_TO_REACH_QUEUE: u32 = 13u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_M_TRACE: u32 = 41u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_M_VERSION: u32 = 19u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_M_XACTID: u32 = 52u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_M_XACTID_SIZE: u32 = 20u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_M_XACT_STATUS_QUEUE: u32 = 39u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_M_XACT_STATUS_QUEUE_LEN: u32 = 40u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_PC_BASE: u32 = 5800u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_PC_DS_ENABLED: u32 = 5802u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_PC_VERSION: u32 = 5801u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_QM_BASE: u32 = 200u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_QM_CONNECTION: u32 = 204u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_QM_ENCRYPTION_PK: u32 = 205u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_QM_ENCRYPTION_PK_AES: u32 = 244u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_QM_ENCRYPTION_PK_BASE: u32 = 231u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_QM_ENCRYPTION_PK_ENHANCED: u32 = 232u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_QM_MACHINE_ID: u32 = 202u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_QM_PATHNAME: u32 = 203u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_QM_PATHNAME_DNS: u32 = 233u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_QM_SITE_ID: u32 = 201u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_Q_ADS_PATH: u32 = 126u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_Q_AUTHENTICATE: u32 = 111u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_Q_BASE: u32 = 100u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_Q_BASEPRIORITY: u32 = 106u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_Q_CREATE_TIME: u32 = 109u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_Q_INSTANCE: u32 = 101u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_Q_JOURNAL: u32 = 104u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_Q_JOURNAL_QUOTA: u32 = 107u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_Q_LABEL: u32 = 108u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_Q_MODIFY_TIME: u32 = 110u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_Q_MULTICAST_ADDRESS: u32 = 125u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_Q_PATHNAME: u32 = 103u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_Q_PATHNAME_DNS: u32 = 124u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_Q_PRIV_LEVEL: u32 = 112u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_Q_QUOTA: u32 = 105u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_Q_TRANSACTION: u32 = 113u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const PROPID_Q_TYPE: u32 = 102u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const QUERY_SORTASCEND: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const QUERY_SORTDESCEND: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const QUEUE_ACTION_EOD_RESEND: &str = "EOD_RESEND";
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const QUEUE_ACTION_PAUSE: &str = "PAUSE";
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const QUEUE_ACTION_RESUME: &str = "RESUME";
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct QUEUE_STATE(pub i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_QUEUE_STATE_LOCAL_CONNECTION: QUEUE_STATE = QUEUE_STATE(0i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_QUEUE_STATE_DISCONNECTED: QUEUE_STATE = QUEUE_STATE(1i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_QUEUE_STATE_WAITING: QUEUE_STATE = QUEUE_STATE(2i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_QUEUE_STATE_NEEDVALIDATE: QUEUE_STATE = QUEUE_STATE(3i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_QUEUE_STATE_ONHOLD: QUEUE_STATE = QUEUE_STATE(4i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_QUEUE_STATE_NONACTIVE: QUEUE_STATE = QUEUE_STATE(5i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_QUEUE_STATE_CONNECTED: QUEUE_STATE = QUEUE_STATE(6i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_QUEUE_STATE_DISCONNECTING: QUEUE_STATE = QUEUE_STATE(7i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_QUEUE_STATE_LOCKED: QUEUE_STATE = QUEUE_STATE(8i32);
impl ::core::marker::Copy for QUEUE_STATE {}
impl ::core::clone::Clone for QUEUE_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for QUEUE_STATE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for QUEUE_STATE {
    type Abi = Self;
}
impl ::core::fmt::Debug for QUEUE_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("QUEUE_STATE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct QUEUE_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_TYPE_PUBLIC: QUEUE_TYPE = QUEUE_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_TYPE_PRIVATE: QUEUE_TYPE = QUEUE_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_TYPE_MACHINE: QUEUE_TYPE = QUEUE_TYPE(2i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_TYPE_CONNECTOR: QUEUE_TYPE = QUEUE_TYPE(3i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_TYPE_MULTICAST: QUEUE_TYPE = QUEUE_TYPE(4i32);
impl ::core::marker::Copy for QUEUE_TYPE {}
impl ::core::clone::Clone for QUEUE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for QUEUE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for QUEUE_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for QUEUE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("QUEUE_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RELOPS(pub i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const REL_NOP: RELOPS = RELOPS(0i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const REL_EQ: RELOPS = RELOPS(1i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const REL_NEQ: RELOPS = RELOPS(2i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const REL_LT: RELOPS = RELOPS(3i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const REL_GT: RELOPS = RELOPS(4i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const REL_LE: RELOPS = RELOPS(5i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const REL_GE: RELOPS = RELOPS(6i32);
impl ::core::marker::Copy for RELOPS {}
impl ::core::clone::Clone for RELOPS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RELOPS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for RELOPS {
    type Abi = Self;
}
impl ::core::fmt::Debug for RELOPS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RELOPS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct XACT_STATUS(pub i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_XACT_STATUS_XACT: XACT_STATUS = XACT_STATUS(0i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_XACT_STATUS_NOT_XACT: XACT_STATUS = XACT_STATUS(1i32);
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`*"]
pub const MQ_XACT_STATUS_UNKNOWN: XACT_STATUS = XACT_STATUS(2i32);
impl ::core::marker::Copy for XACT_STATUS {}
impl ::core::clone::Clone for XACT_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for XACT_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for XACT_STATUS {
    type Abi = Self;
}
impl ::core::fmt::Debug for XACT_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XACT_STATUS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_MessageQueuing\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct _DMSMQEventEvents(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl _DMSMQEventEvents {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<_DMSMQEventEvents> for ::windows::core::IUnknown {
    fn from(value: _DMSMQEventEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a _DMSMQEventEvents> for &'a ::windows::core::IUnknown {
    fn from(value: &'a _DMSMQEventEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&_DMSMQEventEvents> for ::windows::core::IUnknown {
    fn from(value: &_DMSMQEventEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<_DMSMQEventEvents> for super::Com::IDispatch {
    fn from(value: _DMSMQEventEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a _DMSMQEventEvents> for &'a super::Com::IDispatch {
    fn from(value: &'a _DMSMQEventEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&_DMSMQEventEvents> for super::Com::IDispatch {
    fn from(value: &_DMSMQEventEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for _DMSMQEventEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for _DMSMQEventEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for _DMSMQEventEvents {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for _DMSMQEventEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_DMSMQEventEvents").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for _DMSMQEventEvents {
    type Vtable = _DMSMQEventEvents_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd7d6e078_dccd_11d0_aa4b_0060970debae);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct _DMSMQEventEvents_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
