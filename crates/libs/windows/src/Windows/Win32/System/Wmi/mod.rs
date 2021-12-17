#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub type CIMTYPE_ENUMERATION = i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const CIM_ILLEGAL: CIMTYPE_ENUMERATION = 4095i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const CIM_EMPTY: CIMTYPE_ENUMERATION = 0i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const CIM_SINT8: CIMTYPE_ENUMERATION = 16i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const CIM_UINT8: CIMTYPE_ENUMERATION = 17i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const CIM_SINT16: CIMTYPE_ENUMERATION = 2i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const CIM_UINT16: CIMTYPE_ENUMERATION = 18i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const CIM_SINT32: CIMTYPE_ENUMERATION = 3i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const CIM_UINT32: CIMTYPE_ENUMERATION = 19i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const CIM_SINT64: CIMTYPE_ENUMERATION = 20i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const CIM_UINT64: CIMTYPE_ENUMERATION = 21i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const CIM_REAL32: CIMTYPE_ENUMERATION = 4i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const CIM_REAL64: CIMTYPE_ENUMERATION = 5i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const CIM_BOOLEAN: CIMTYPE_ENUMERATION = 11i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const CIM_STRING: CIMTYPE_ENUMERATION = 8i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const CIM_DATETIME: CIMTYPE_ENUMERATION = 101i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const CIM_REFERENCE: CIMTYPE_ENUMERATION = 102i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const CIM_CHAR16: CIMTYPE_ENUMERATION = 103i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const CIM_OBJECT: CIMTYPE_ENUMERATION = 13i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const CIM_FLAG_ARRAY: CIMTYPE_ENUMERATION = 8192i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
#[repr(transparent)]
pub struct IEnumWbemClassObject(::windows::core::IUnknown);
impl IEnumWbemClassObject {
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn Next(&self, ltimeout: i32, ucount: u32, apobjects: *mut ::core::option::Option<IWbemClassObject>, pureturned: *mut u32) -> ::windows::core::HRESULT {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(ltimeout), ::core::mem::transmute(ucount), ::core::mem::transmute(apobjects), ::core::mem::transmute(pureturned)))
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn NextAsync<'a, Param1: ::windows::core::IntoParam<'a, IWbemObjectSink>>(&self, ucount: u32, psink: Param1) -> ::windows::core::HRESULT {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(ucount), psink.into_param().abi()))
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumWbemClassObject> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IEnumWbemClassObject>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn Skip(&self, ltimeout: i32, ncount: u32) -> ::windows::core::HRESULT {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(ltimeout), ::core::mem::transmute(ncount)))
    }
}
impl ::core::convert::From<IEnumWbemClassObject> for ::windows::core::IUnknown {
    fn from(value: IEnumWbemClassObject) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnumWbemClassObject> for ::windows::core::IUnknown {
    fn from(value: &IEnumWbemClassObject) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IEnumWbemClassObject {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IEnumWbemClassObject {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IEnumWbemClassObject {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEnumWbemClassObject {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumWbemClassObject {}
unsafe impl ::windows::core::Interface for IEnumWbemClassObject {
    type Vtable = IEnumWbemClassObjectVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x027947e1_d731_11ce_a357_000000000001);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumWbemClassObjectVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ltimeout: i32, ucount: u32, apobjects: *mut ::windows::core::RawPtr, pureturned: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ucount: u32, psink: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ltimeout: i32, ncount: u32) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_System_Wmi'*"]
#[repr(transparent)]
pub struct IMofCompiler(::windows::core::IUnknown);
impl IMofCompiler {
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CompileFile<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, filename: Param0, serverandnamespace: Param1, user: Param2, authority: Param3, password: Param4, loptionflags: i32, lclassflags: i32, linstanceflags: i32, pinfo: *mut WBEM_COMPILE_STATUS_INFO) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), filename.into_param().abi(), serverandnamespace.into_param().abi(), user.into_param().abi(), authority.into_param().abi(), password.into_param().abi(), ::core::mem::transmute(loptionflags), ::core::mem::transmute(lclassflags), ::core::mem::transmute(linstanceflags), ::core::mem::transmute(pinfo)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CompileBuffer<'a, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param5: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, buffsize: i32, pbuffer: *const u8, serverandnamespace: Param2, user: Param3, authority: Param4, password: Param5, loptionflags: i32, lclassflags: i32, linstanceflags: i32, pinfo: *mut WBEM_COMPILE_STATUS_INFO) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(buffsize), ::core::mem::transmute(pbuffer), serverandnamespace.into_param().abi(), user.into_param().abi(), authority.into_param().abi(), password.into_param().abi(), ::core::mem::transmute(loptionflags), ::core::mem::transmute(lclassflags), ::core::mem::transmute(linstanceflags), ::core::mem::transmute(pinfo)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateBMOF<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, textfilename: Param0, bmoffilename: Param1, serverandnamespace: Param2, loptionflags: i32, lclassflags: i32, linstanceflags: i32, pinfo: *mut WBEM_COMPILE_STATUS_INFO) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), textfilename.into_param().abi(), bmoffilename.into_param().abi(), serverandnamespace.into_param().abi(), ::core::mem::transmute(loptionflags), ::core::mem::transmute(lclassflags), ::core::mem::transmute(linstanceflags), ::core::mem::transmute(pinfo)).ok()
    }
}
impl ::core::convert::From<IMofCompiler> for ::windows::core::IUnknown {
    fn from(value: IMofCompiler) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMofCompiler> for ::windows::core::IUnknown {
    fn from(value: &IMofCompiler) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMofCompiler {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IMofCompiler {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMofCompiler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IMofCompiler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMofCompiler {}
unsafe impl ::windows::core::Interface for IMofCompiler {
    type Vtable = IMofCompilerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6daf974e_2e37_11d2_aec9_00c04fb68820);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMofCompilerVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filename: super::super::Foundation::PWSTR, serverandnamespace: super::super::Foundation::PWSTR, user: super::super::Foundation::PWSTR, authority: super::super::Foundation::PWSTR, password: super::super::Foundation::PWSTR, loptionflags: i32, lclassflags: i32, linstanceflags: i32, pinfo: *mut WBEM_COMPILE_STATUS_INFO) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buffsize: i32, pbuffer: *const u8, serverandnamespace: super::super::Foundation::PWSTR, user: super::super::Foundation::PWSTR, authority: super::super::Foundation::PWSTR, password: super::super::Foundation::PWSTR, loptionflags: i32, lclassflags: i32, linstanceflags: i32, pinfo: *mut WBEM_COMPILE_STATUS_INFO) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, textfilename: super::super::Foundation::PWSTR, bmoffilename: super::super::Foundation::PWSTR, serverandnamespace: super::super::Foundation::PWSTR, loptionflags: i32, lclassflags: i32, linstanceflags: i32, pinfo: *mut WBEM_COMPILE_STATUS_INFO) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_System_Wmi'*"]
#[repr(transparent)]
pub struct ISWbemDateTime(::windows::core::IUnknown);
impl ISWbemDateTime {
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::ITypeInfo>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Value(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetValue<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, strvalue: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), strvalue.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn Year(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn SetYear(&self, iyear: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(iyear)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn YearSpecified(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn SetYearSpecified(&self, byearspecified: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(byearspecified)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn Month(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn SetMonth(&self, imonth: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(imonth)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn MonthSpecified(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn SetMonthSpecified(&self, bmonthspecified: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(bmonthspecified)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn Day(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn SetDay(&self, iday: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(iday)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn DaySpecified(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn SetDaySpecified(&self, bdayspecified: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(bdayspecified)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn Hours(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn SetHours(&self, ihours: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(ihours)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn HoursSpecified(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn SetHoursSpecified(&self, bhoursspecified: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), ::core::mem::transmute(bhoursspecified)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn Minutes(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn SetMinutes(&self, iminutes: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), ::core::mem::transmute(iminutes)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn MinutesSpecified(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn SetMinutesSpecified(&self, bminutesspecified: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).28)(::core::mem::transmute_copy(self), ::core::mem::transmute(bminutesspecified)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn Seconds(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).29)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn SetSeconds(&self, iseconds: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).30)(::core::mem::transmute_copy(self), ::core::mem::transmute(iseconds)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn SecondsSpecified(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).31)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn SetSecondsSpecified(&self, bsecondsspecified: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).32)(::core::mem::transmute_copy(self), ::core::mem::transmute(bsecondsspecified)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn Microseconds(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).33)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn SetMicroseconds(&self, imicroseconds: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).34)(::core::mem::transmute_copy(self), ::core::mem::transmute(imicroseconds)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn MicrosecondsSpecified(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).35)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn SetMicrosecondsSpecified(&self, bmicrosecondsspecified: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).36)(::core::mem::transmute_copy(self), ::core::mem::transmute(bmicrosecondsspecified)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn UTC(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).37)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn SetUTC(&self, iutc: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).38)(::core::mem::transmute_copy(self), ::core::mem::transmute(iutc)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn UTCSpecified(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).39)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn SetUTCSpecified(&self, butcspecified: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).40)(::core::mem::transmute_copy(self), ::core::mem::transmute(butcspecified)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn IsInterval(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).41)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn SetIsInterval(&self, bisinterval: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).42)(::core::mem::transmute_copy(self), ::core::mem::transmute(bisinterval)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn GetVarDate(&self, bislocal: i16) -> ::windows::core::Result<f64> {
        let mut result__: f64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).43)(::core::mem::transmute_copy(self), ::core::mem::transmute(bislocal), ::core::mem::transmute(&mut result__)).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn SetVarDate(&self, dvardate: f64, bislocal: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).44)(::core::mem::transmute_copy(self), ::core::mem::transmute(dvardate), ::core::mem::transmute(bislocal)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFileTime(&self, bislocal: i16) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).45)(::core::mem::transmute_copy(self), ::core::mem::transmute(bislocal), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetFileTime<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, strfiletime: Param0, bislocal: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).46)(::core::mem::transmute_copy(self), strfiletime.into_param().abi(), ::core::mem::transmute(bislocal)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ISWbemDateTime> for super::Com::IDispatch {
    fn from(value: ISWbemDateTime) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ISWbemDateTime> for super::Com::IDispatch {
    fn from(value: &ISWbemDateTime) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for ISWbemDateTime {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &ISWbemDateTime {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ISWbemDateTime> for ::windows::core::IUnknown {
    fn from(value: ISWbemDateTime) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISWbemDateTime> for ::windows::core::IUnknown {
    fn from(value: &ISWbemDateTime) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISWbemDateTime {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ISWbemDateTime {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISWbemDateTime {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISWbemDateTime {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISWbemDateTime {}
unsafe impl ::windows::core::Interface for ISWbemDateTime {
    type Vtable = ISWbemDateTimeVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5e97458a_cf77_11d3_b38f_00105a1f473a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISWbemDateTimeVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strvalue: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strvalue: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iyear: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iyear: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, byearspecified: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, byearspecified: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, imonth: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, imonth: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bmonthspecified: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bmonthspecified: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iday: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iday: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bdayspecified: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bdayspecified: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ihours: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ihours: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bhoursspecified: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bhoursspecified: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iminutes: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iminutes: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bminutesspecified: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bminutesspecified: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iseconds: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iseconds: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bsecondsspecified: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bsecondsspecified: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, imicroseconds: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, imicroseconds: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bmicrosecondsspecified: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bmicrosecondsspecified: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iutc: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iutc: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, butcspecified: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, butcspecified: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bisinterval: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bisinterval: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bislocal: i16, dvardate: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dvardate: f64, bislocal: i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bislocal: i16, strfiletime: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strfiletime: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bislocal: i16) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_System_Wmi'*"]
#[repr(transparent)]
pub struct ISWbemEventSource(::windows::core::IUnknown);
impl ISWbemEventSource {
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::ITypeInfo>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn NextEvent(&self, itimeoutms: i32) -> ::windows::core::Result<ISWbemObject> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(itimeoutms), ::core::mem::transmute(&mut result__)).from_abi::<ISWbemObject>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn Security_(&self) -> ::windows::core::Result<ISWbemSecurity> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ISWbemSecurity>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ISWbemEventSource> for super::Com::IDispatch {
    fn from(value: ISWbemEventSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ISWbemEventSource> for super::Com::IDispatch {
    fn from(value: &ISWbemEventSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for ISWbemEventSource {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &ISWbemEventSource {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ISWbemEventSource> for ::windows::core::IUnknown {
    fn from(value: ISWbemEventSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISWbemEventSource> for ::windows::core::IUnknown {
    fn from(value: &ISWbemEventSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISWbemEventSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ISWbemEventSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISWbemEventSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISWbemEventSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISWbemEventSource {}
unsafe impl ::windows::core::Interface for ISWbemEventSource {
    type Vtable = ISWbemEventSourceVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x27d54d92_0ebe_11d2_8b22_00600806d9b6);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISWbemEventSourceVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itimeoutms: i32, objwbemobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemsecurity: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_System_Wmi'*"]
#[repr(transparent)]
pub struct ISWbemLastError(::windows::core::IUnknown);
impl ISWbemLastError {
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::ITypeInfo>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Put_<'a, Param1: ::windows::core::IntoParam<'a, super::Com::IDispatch>>(&self, iflags: i32, objwbemnamedvalueset: Param1) -> ::windows::core::Result<ISWbemObjectPath> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<ISWbemObjectPath>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PutAsync_<'a, Param0: ::windows::core::IntoParam<'a, super::Com::IDispatch>, Param2: ::windows::core::IntoParam<'a, super::Com::IDispatch>, Param3: ::windows::core::IntoParam<'a, super::Com::IDispatch>>(&self, objwbemsink: Param0, iflags: i32, objwbemnamedvalueset: Param2, objwbemasynccontext: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), objwbemsink.into_param().abi(), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi(), objwbemasynccontext.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Delete_<'a, Param1: ::windows::core::IntoParam<'a, super::Com::IDispatch>>(&self, iflags: i32, objwbemnamedvalueset: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn DeleteAsync_<'a, Param0: ::windows::core::IntoParam<'a, super::Com::IDispatch>, Param2: ::windows::core::IntoParam<'a, super::Com::IDispatch>, Param3: ::windows::core::IntoParam<'a, super::Com::IDispatch>>(&self, objwbemsink: Param0, iflags: i32, objwbemnamedvalueset: Param2, objwbemasynccontext: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), objwbemsink.into_param().abi(), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi(), objwbemasynccontext.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Instances_<'a, Param1: ::windows::core::IntoParam<'a, super::Com::IDispatch>>(&self, iflags: i32, objwbemnamedvalueset: Param1) -> ::windows::core::Result<ISWbemObjectSet> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<ISWbemObjectSet>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InstancesAsync_<'a, Param0: ::windows::core::IntoParam<'a, super::Com::IDispatch>, Param2: ::windows::core::IntoParam<'a, super::Com::IDispatch>, Param3: ::windows::core::IntoParam<'a, super::Com::IDispatch>>(&self, objwbemsink: Param0, iflags: i32, objwbemnamedvalueset: Param2, objwbemasynccontext: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), objwbemsink.into_param().abi(), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi(), objwbemasynccontext.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Subclasses_<'a, Param1: ::windows::core::IntoParam<'a, super::Com::IDispatch>>(&self, iflags: i32, objwbemnamedvalueset: Param1) -> ::windows::core::Result<ISWbemObjectSet> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<ISWbemObjectSet>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SubclassesAsync_<'a, Param0: ::windows::core::IntoParam<'a, super::Com::IDispatch>, Param2: ::windows::core::IntoParam<'a, super::Com::IDispatch>, Param3: ::windows::core::IntoParam<'a, super::Com::IDispatch>>(&self, objwbemsink: Param0, iflags: i32, objwbemnamedvalueset: Param2, objwbemasynccontext: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), objwbemsink.into_param().abi(), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi(), objwbemasynccontext.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn Associators_<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param6: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param7: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param9: ::windows::core::IntoParam<'a, super::Com::IDispatch>>(
        &self,
        strassocclass: Param0,
        strresultclass: Param1,
        strresultrole: Param2,
        strrole: Param3,
        bclassesonly: i16,
        bschemaonly: i16,
        strrequiredassocqualifier: Param6,
        strrequiredqualifier: Param7,
        iflags: i32,
        objwbemnamedvalueset: Param9,
    ) -> ::windows::core::Result<ISWbemObjectSet> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), strassocclass.into_param().abi(), strresultclass.into_param().abi(), strresultrole.into_param().abi(), strrole.into_param().abi(), ::core::mem::transmute(bclassesonly), ::core::mem::transmute(bschemaonly), strrequiredassocqualifier.into_param().abi(), strrequiredqualifier.into_param().abi(), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<ISWbemObjectSet>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn AssociatorsAsync_<'a, Param0: ::windows::core::IntoParam<'a, super::Com::IDispatch>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param7: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param8: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param10: ::windows::core::IntoParam<'a, super::Com::IDispatch>, Param11: ::windows::core::IntoParam<'a, super::Com::IDispatch>>(
        &self,
        objwbemsink: Param0,
        strassocclass: Param1,
        strresultclass: Param2,
        strresultrole: Param3,
        strrole: Param4,
        bclassesonly: i16,
        bschemaonly: i16,
        strrequiredassocqualifier: Param7,
        strrequiredqualifier: Param8,
        iflags: i32,
        objwbemnamedvalueset: Param10,
        objwbemasynccontext: Param11,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), objwbemsink.into_param().abi(), strassocclass.into_param().abi(), strresultclass.into_param().abi(), strresultrole.into_param().abi(), strrole.into_param().abi(), ::core::mem::transmute(bclassesonly), ::core::mem::transmute(bschemaonly), strrequiredassocqualifier.into_param().abi(), strrequiredqualifier.into_param().abi(), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi(), objwbemasynccontext.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn References_<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param6: ::windows::core::IntoParam<'a, super::Com::IDispatch>>(&self, strresultclass: Param0, strrole: Param1, bclassesonly: i16, bschemaonly: i16, strrequiredqualifier: Param4, iflags: i32, objwbemnamedvalueset: Param6) -> ::windows::core::Result<ISWbemObjectSet> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), strresultclass.into_param().abi(), strrole.into_param().abi(), ::core::mem::transmute(bclassesonly), ::core::mem::transmute(bschemaonly), strrequiredqualifier.into_param().abi(), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<ISWbemObjectSet>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn ReferencesAsync_<'a, Param0: ::windows::core::IntoParam<'a, super::Com::IDispatch>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param5: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param7: ::windows::core::IntoParam<'a, super::Com::IDispatch>, Param8: ::windows::core::IntoParam<'a, super::Com::IDispatch>>(&self, objwbemsink: Param0, strresultclass: Param1, strrole: Param2, bclassesonly: i16, bschemaonly: i16, strrequiredqualifier: Param5, iflags: i32, objwbemnamedvalueset: Param7, objwbemasynccontext: Param8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), objwbemsink.into_param().abi(), strresultclass.into_param().abi(), strrole.into_param().abi(), ::core::mem::transmute(bclassesonly), ::core::mem::transmute(bschemaonly), strrequiredqualifier.into_param().abi(), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi(), objwbemasynccontext.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn ExecMethod_<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::Com::IDispatch>, Param3: ::windows::core::IntoParam<'a, super::Com::IDispatch>>(&self, strmethodname: Param0, objwbeminparameters: Param1, iflags: i32, objwbemnamedvalueset: Param3) -> ::windows::core::Result<ISWbemObject> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), strmethodname.into_param().abi(), objwbeminparameters.into_param().abi(), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<ISWbemObject>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn ExecMethodAsync_<'a, Param0: ::windows::core::IntoParam<'a, super::Com::IDispatch>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::Com::IDispatch>, Param4: ::windows::core::IntoParam<'a, super::Com::IDispatch>, Param5: ::windows::core::IntoParam<'a, super::Com::IDispatch>>(&self, objwbemsink: Param0, strmethodname: Param1, objwbeminparameters: Param2, iflags: i32, objwbemnamedvalueset: Param4, objwbemasynccontext: Param5) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), objwbemsink.into_param().abi(), strmethodname.into_param().abi(), objwbeminparameters.into_param().abi(), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi(), objwbemasynccontext.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn Clone_(&self) -> ::windows::core::Result<ISWbemObject> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ISWbemObject>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetObjectText_(&self, iflags: i32) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(iflags), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn SpawnDerivedClass_(&self, iflags: i32) -> ::windows::core::Result<ISWbemObject> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(iflags), ::core::mem::transmute(&mut result__)).from_abi::<ISWbemObject>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn SpawnInstance_(&self, iflags: i32) -> ::windows::core::Result<ISWbemObject> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), ::core::mem::transmute(iflags), ::core::mem::transmute(&mut result__)).from_abi::<ISWbemObject>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CompareTo_<'a, Param0: ::windows::core::IntoParam<'a, super::Com::IDispatch>>(&self, objwbemobject: Param0, iflags: i32) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), objwbemobject.into_param().abi(), ::core::mem::transmute(iflags), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn Qualifiers_(&self) -> ::windows::core::Result<ISWbemQualifierSet> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ISWbemQualifierSet>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn Properties_(&self) -> ::windows::core::Result<ISWbemPropertySet> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ISWbemPropertySet>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn Methods_(&self) -> ::windows::core::Result<ISWbemMethodSet> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).28)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ISWbemMethodSet>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Derivation_(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).29)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn Path_(&self) -> ::windows::core::Result<ISWbemObjectPath> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).30)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ISWbemObjectPath>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn Security_(&self) -> ::windows::core::Result<ISWbemSecurity> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).31)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ISWbemSecurity>(result__)
    }
}
impl ::core::convert::From<ISWbemLastError> for ISWbemObject {
    fn from(value: ISWbemLastError) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISWbemLastError> for ISWbemObject {
    fn from(value: &ISWbemLastError) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ISWbemObject> for ISWbemLastError {
    fn into_param(self) -> ::windows::core::Param<'a, ISWbemObject> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ISWbemObject> for &ISWbemLastError {
    fn into_param(self) -> ::windows::core::Param<'a, ISWbemObject> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ISWbemLastError> for super::Com::IDispatch {
    fn from(value: ISWbemLastError) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ISWbemLastError> for super::Com::IDispatch {
    fn from(value: &ISWbemLastError) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for ISWbemLastError {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &ISWbemLastError {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ISWbemLastError> for ::windows::core::IUnknown {
    fn from(value: ISWbemLastError) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISWbemLastError> for ::windows::core::IUnknown {
    fn from(value: &ISWbemLastError) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISWbemLastError {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ISWbemLastError {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISWbemLastError {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISWbemLastError {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISWbemLastError {}
unsafe impl ::windows::core::Interface for ISWbemLastError {
    type Vtable = ISWbemLastErrorVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd962db84_d4bb_11d1_8b09_00600806d9b6);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISWbemLastErrorVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemobjectpath: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemsink: ::windows::core::RawPtr, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemasynccontext: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemsink: ::windows::core::RawPtr, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemasynccontext: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemobjectset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemsink: ::windows::core::RawPtr, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemasynccontext: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemobjectset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemsink: ::windows::core::RawPtr, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemasynccontext: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strassocclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strresultclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strresultrole: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strrole: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bclassesonly: i16, bschemaonly: i16, strrequiredassocqualifier: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strrequiredqualifier: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemobjectset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub  unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemsink: ::windows::core::RawPtr, strassocclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strresultclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strresultrole: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strrole: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bclassesonly: i16, bschemaonly: i16, strrequiredassocqualifier: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strrequiredqualifier: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemasynccontext: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strresultclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strrole: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bclassesonly: i16, bschemaonly: i16, strrequiredqualifier: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemobjectset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemsink: ::windows::core::RawPtr, strresultclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strrole: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bclassesonly: i16, bschemaonly: i16, strrequiredqualifier: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemasynccontext: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strmethodname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, objwbeminparameters: ::windows::core::RawPtr, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemoutparameters: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemsink: ::windows::core::RawPtr, strmethodname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, objwbeminparameters: ::windows::core::RawPtr, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemasynccontext: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iflags: i32, strobjecttext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iflags: i32, objwbemobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iflags: i32, objwbemobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemobject: ::windows::core::RawPtr, iflags: i32, bresult: *mut i16) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemqualifierset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbempropertyset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemmethodset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strclassnamearray: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemobjectpath: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemsecurity: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_System_Wmi'*"]
#[repr(transparent)]
pub struct ISWbemLocator(::windows::core::IUnknown);
impl ISWbemLocator {
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::ITypeInfo>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn ConnectServer<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param5: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param7: ::windows::core::IntoParam<'a, super::Com::IDispatch>>(&self, strserver: Param0, strnamespace: Param1, struser: Param2, strpassword: Param3, strlocale: Param4, strauthority: Param5, isecurityflags: i32, objwbemnamedvalueset: Param7) -> ::windows::core::Result<ISWbemServices> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), strserver.into_param().abi(), strnamespace.into_param().abi(), struser.into_param().abi(), strpassword.into_param().abi(), strlocale.into_param().abi(), strauthority.into_param().abi(), ::core::mem::transmute(isecurityflags), objwbemnamedvalueset.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<ISWbemServices>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn Security_(&self) -> ::windows::core::Result<ISWbemSecurity> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ISWbemSecurity>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ISWbemLocator> for super::Com::IDispatch {
    fn from(value: ISWbemLocator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ISWbemLocator> for super::Com::IDispatch {
    fn from(value: &ISWbemLocator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for ISWbemLocator {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &ISWbemLocator {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ISWbemLocator> for ::windows::core::IUnknown {
    fn from(value: ISWbemLocator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISWbemLocator> for ::windows::core::IUnknown {
    fn from(value: &ISWbemLocator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISWbemLocator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ISWbemLocator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISWbemLocator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISWbemLocator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISWbemLocator {}
unsafe impl ::windows::core::Interface for ISWbemLocator {
    type Vtable = ISWbemLocatorVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x76a6415b_cb41_11d1_8b02_00600806d9b6);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISWbemLocatorVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strserver: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strnamespace: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, struser: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strlocale: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strauthority: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, isecurityflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemservices: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemsecurity: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_System_Wmi'*"]
#[repr(transparent)]
pub struct ISWbemMethod(::windows::core::IUnknown);
impl ISWbemMethod {
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::ITypeInfo>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Origin(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn InParameters(&self) -> ::windows::core::Result<ISWbemObject> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ISWbemObject>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn OutParameters(&self) -> ::windows::core::Result<ISWbemObject> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ISWbemObject>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn Qualifiers_(&self) -> ::windows::core::Result<ISWbemQualifierSet> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ISWbemQualifierSet>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ISWbemMethod> for super::Com::IDispatch {
    fn from(value: ISWbemMethod) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ISWbemMethod> for super::Com::IDispatch {
    fn from(value: &ISWbemMethod) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for ISWbemMethod {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &ISWbemMethod {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ISWbemMethod> for ::windows::core::IUnknown {
    fn from(value: ISWbemMethod) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISWbemMethod> for ::windows::core::IUnknown {
    fn from(value: &ISWbemMethod) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISWbemMethod {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ISWbemMethod {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISWbemMethod {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISWbemMethod {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISWbemMethod {}
unsafe impl ::windows::core::Interface for ISWbemMethod {
    type Vtable = ISWbemMethodVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x422e8e90_d955_11d1_8b09_00600806d9b6);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISWbemMethodVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strorigin: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbeminparameters: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemoutparameters: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemqualifierset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_System_Wmi'*"]
#[repr(transparent)]
pub struct ISWbemMethodSet(::windows::core::IUnknown);
impl ISWbemMethodSet {
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::ITypeInfo>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Item<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, strname: Param0, iflags: i32) -> ::windows::core::Result<ISWbemMethod> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), strname.into_param().abi(), ::core::mem::transmute(iflags), ::core::mem::transmute(&mut result__)).from_abi::<ISWbemMethod>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ISWbemMethodSet> for super::Com::IDispatch {
    fn from(value: ISWbemMethodSet) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ISWbemMethodSet> for super::Com::IDispatch {
    fn from(value: &ISWbemMethodSet) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for ISWbemMethodSet {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &ISWbemMethodSet {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ISWbemMethodSet> for ::windows::core::IUnknown {
    fn from(value: ISWbemMethodSet) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISWbemMethodSet> for ::windows::core::IUnknown {
    fn from(value: &ISWbemMethodSet) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISWbemMethodSet {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ISWbemMethodSet {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISWbemMethodSet {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISWbemMethodSet {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISWbemMethodSet {}
unsafe impl ::windows::core::Interface for ISWbemMethodSet {
    type Vtable = ISWbemMethodSetVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc93ba292_d955_11d1_8b09_00600806d9b6);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISWbemMethodSetVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemmethod: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, icount: *mut i32) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_System_Wmi'*"]
#[repr(transparent)]
pub struct ISWbemNamedValue(::windows::core::IUnknown);
impl ISWbemNamedValue {
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::ITypeInfo>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Value(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetValue(&self, varvalue: *const super::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(varvalue)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ISWbemNamedValue> for super::Com::IDispatch {
    fn from(value: ISWbemNamedValue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ISWbemNamedValue> for super::Com::IDispatch {
    fn from(value: &ISWbemNamedValue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for ISWbemNamedValue {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &ISWbemNamedValue {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ISWbemNamedValue> for ::windows::core::IUnknown {
    fn from(value: ISWbemNamedValue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISWbemNamedValue> for ::windows::core::IUnknown {
    fn from(value: &ISWbemNamedValue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISWbemNamedValue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ISWbemNamedValue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISWbemNamedValue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISWbemNamedValue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISWbemNamedValue {}
unsafe impl ::windows::core::Interface for ISWbemNamedValue {
    type Vtable = ISWbemNamedValueVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x76a64164_cb41_11d1_8b02_00600806d9b6);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISWbemNamedValueVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varvalue: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varvalue: *const super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_System_Wmi'*"]
#[repr(transparent)]
pub struct ISWbemNamedValueSet(::windows::core::IUnknown);
impl ISWbemNamedValueSet {
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::ITypeInfo>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Item<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, strname: Param0, iflags: i32) -> ::windows::core::Result<ISWbemNamedValue> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), strname.into_param().abi(), ::core::mem::transmute(iflags), ::core::mem::transmute(&mut result__)).from_abi::<ISWbemNamedValue>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Add<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, strname: Param0, varvalue: *const super::Com::VARIANT, iflags: i32) -> ::windows::core::Result<ISWbemNamedValue> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), strname.into_param().abi(), ::core::mem::transmute(varvalue), ::core::mem::transmute(iflags), ::core::mem::transmute(&mut result__)).from_abi::<ISWbemNamedValue>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Remove<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, strname: Param0, iflags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), strname.into_param().abi(), ::core::mem::transmute(iflags)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn Clone(&self) -> ::windows::core::Result<ISWbemNamedValueSet> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ISWbemNamedValueSet>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn DeleteAll(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ISWbemNamedValueSet> for super::Com::IDispatch {
    fn from(value: ISWbemNamedValueSet) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ISWbemNamedValueSet> for super::Com::IDispatch {
    fn from(value: &ISWbemNamedValueSet) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for ISWbemNamedValueSet {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &ISWbemNamedValueSet {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ISWbemNamedValueSet> for ::windows::core::IUnknown {
    fn from(value: ISWbemNamedValueSet) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISWbemNamedValueSet> for ::windows::core::IUnknown {
    fn from(value: &ISWbemNamedValueSet) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISWbemNamedValueSet {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ISWbemNamedValueSet {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISWbemNamedValueSet {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISWbemNamedValueSet {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISWbemNamedValueSet {}
unsafe impl ::windows::core::Interface for ISWbemNamedValueSet {
    type Vtable = ISWbemNamedValueSetVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcf2376ea_ce8c_11d1_8b05_00600806d9b6);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISWbemNamedValueSetVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemnamedvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, icount: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varvalue: *const super::Com::VARIANT, iflags: i32, objwbemnamedvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemnamedvalueset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_System_Wmi'*"]
#[repr(transparent)]
pub struct ISWbemObject(::windows::core::IUnknown);
impl ISWbemObject {
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::ITypeInfo>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Put_<'a, Param1: ::windows::core::IntoParam<'a, super::Com::IDispatch>>(&self, iflags: i32, objwbemnamedvalueset: Param1) -> ::windows::core::Result<ISWbemObjectPath> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<ISWbemObjectPath>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PutAsync_<'a, Param0: ::windows::core::IntoParam<'a, super::Com::IDispatch>, Param2: ::windows::core::IntoParam<'a, super::Com::IDispatch>, Param3: ::windows::core::IntoParam<'a, super::Com::IDispatch>>(&self, objwbemsink: Param0, iflags: i32, objwbemnamedvalueset: Param2, objwbemasynccontext: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), objwbemsink.into_param().abi(), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi(), objwbemasynccontext.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Delete_<'a, Param1: ::windows::core::IntoParam<'a, super::Com::IDispatch>>(&self, iflags: i32, objwbemnamedvalueset: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn DeleteAsync_<'a, Param0: ::windows::core::IntoParam<'a, super::Com::IDispatch>, Param2: ::windows::core::IntoParam<'a, super::Com::IDispatch>, Param3: ::windows::core::IntoParam<'a, super::Com::IDispatch>>(&self, objwbemsink: Param0, iflags: i32, objwbemnamedvalueset: Param2, objwbemasynccontext: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), objwbemsink.into_param().abi(), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi(), objwbemasynccontext.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Instances_<'a, Param1: ::windows::core::IntoParam<'a, super::Com::IDispatch>>(&self, iflags: i32, objwbemnamedvalueset: Param1) -> ::windows::core::Result<ISWbemObjectSet> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<ISWbemObjectSet>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InstancesAsync_<'a, Param0: ::windows::core::IntoParam<'a, super::Com::IDispatch>, Param2: ::windows::core::IntoParam<'a, super::Com::IDispatch>, Param3: ::windows::core::IntoParam<'a, super::Com::IDispatch>>(&self, objwbemsink: Param0, iflags: i32, objwbemnamedvalueset: Param2, objwbemasynccontext: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), objwbemsink.into_param().abi(), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi(), objwbemasynccontext.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Subclasses_<'a, Param1: ::windows::core::IntoParam<'a, super::Com::IDispatch>>(&self, iflags: i32, objwbemnamedvalueset: Param1) -> ::windows::core::Result<ISWbemObjectSet> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<ISWbemObjectSet>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SubclassesAsync_<'a, Param0: ::windows::core::IntoParam<'a, super::Com::IDispatch>, Param2: ::windows::core::IntoParam<'a, super::Com::IDispatch>, Param3: ::windows::core::IntoParam<'a, super::Com::IDispatch>>(&self, objwbemsink: Param0, iflags: i32, objwbemnamedvalueset: Param2, objwbemasynccontext: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), objwbemsink.into_param().abi(), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi(), objwbemasynccontext.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn Associators_<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param6: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param7: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param9: ::windows::core::IntoParam<'a, super::Com::IDispatch>>(
        &self,
        strassocclass: Param0,
        strresultclass: Param1,
        strresultrole: Param2,
        strrole: Param3,
        bclassesonly: i16,
        bschemaonly: i16,
        strrequiredassocqualifier: Param6,
        strrequiredqualifier: Param7,
        iflags: i32,
        objwbemnamedvalueset: Param9,
    ) -> ::windows::core::Result<ISWbemObjectSet> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), strassocclass.into_param().abi(), strresultclass.into_param().abi(), strresultrole.into_param().abi(), strrole.into_param().abi(), ::core::mem::transmute(bclassesonly), ::core::mem::transmute(bschemaonly), strrequiredassocqualifier.into_param().abi(), strrequiredqualifier.into_param().abi(), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<ISWbemObjectSet>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn AssociatorsAsync_<'a, Param0: ::windows::core::IntoParam<'a, super::Com::IDispatch>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param7: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param8: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param10: ::windows::core::IntoParam<'a, super::Com::IDispatch>, Param11: ::windows::core::IntoParam<'a, super::Com::IDispatch>>(
        &self,
        objwbemsink: Param0,
        strassocclass: Param1,
        strresultclass: Param2,
        strresultrole: Param3,
        strrole: Param4,
        bclassesonly: i16,
        bschemaonly: i16,
        strrequiredassocqualifier: Param7,
        strrequiredqualifier: Param8,
        iflags: i32,
        objwbemnamedvalueset: Param10,
        objwbemasynccontext: Param11,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), objwbemsink.into_param().abi(), strassocclass.into_param().abi(), strresultclass.into_param().abi(), strresultrole.into_param().abi(), strrole.into_param().abi(), ::core::mem::transmute(bclassesonly), ::core::mem::transmute(bschemaonly), strrequiredassocqualifier.into_param().abi(), strrequiredqualifier.into_param().abi(), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi(), objwbemasynccontext.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn References_<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param6: ::windows::core::IntoParam<'a, super::Com::IDispatch>>(&self, strresultclass: Param0, strrole: Param1, bclassesonly: i16, bschemaonly: i16, strrequiredqualifier: Param4, iflags: i32, objwbemnamedvalueset: Param6) -> ::windows::core::Result<ISWbemObjectSet> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), strresultclass.into_param().abi(), strrole.into_param().abi(), ::core::mem::transmute(bclassesonly), ::core::mem::transmute(bschemaonly), strrequiredqualifier.into_param().abi(), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<ISWbemObjectSet>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn ReferencesAsync_<'a, Param0: ::windows::core::IntoParam<'a, super::Com::IDispatch>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param5: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param7: ::windows::core::IntoParam<'a, super::Com::IDispatch>, Param8: ::windows::core::IntoParam<'a, super::Com::IDispatch>>(&self, objwbemsink: Param0, strresultclass: Param1, strrole: Param2, bclassesonly: i16, bschemaonly: i16, strrequiredqualifier: Param5, iflags: i32, objwbemnamedvalueset: Param7, objwbemasynccontext: Param8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), objwbemsink.into_param().abi(), strresultclass.into_param().abi(), strrole.into_param().abi(), ::core::mem::transmute(bclassesonly), ::core::mem::transmute(bschemaonly), strrequiredqualifier.into_param().abi(), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi(), objwbemasynccontext.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn ExecMethod_<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::Com::IDispatch>, Param3: ::windows::core::IntoParam<'a, super::Com::IDispatch>>(&self, strmethodname: Param0, objwbeminparameters: Param1, iflags: i32, objwbemnamedvalueset: Param3) -> ::windows::core::Result<ISWbemObject> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), strmethodname.into_param().abi(), objwbeminparameters.into_param().abi(), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<ISWbemObject>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn ExecMethodAsync_<'a, Param0: ::windows::core::IntoParam<'a, super::Com::IDispatch>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::Com::IDispatch>, Param4: ::windows::core::IntoParam<'a, super::Com::IDispatch>, Param5: ::windows::core::IntoParam<'a, super::Com::IDispatch>>(&self, objwbemsink: Param0, strmethodname: Param1, objwbeminparameters: Param2, iflags: i32, objwbemnamedvalueset: Param4, objwbemasynccontext: Param5) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), objwbemsink.into_param().abi(), strmethodname.into_param().abi(), objwbeminparameters.into_param().abi(), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi(), objwbemasynccontext.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn Clone_(&self) -> ::windows::core::Result<ISWbemObject> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ISWbemObject>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetObjectText_(&self, iflags: i32) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(iflags), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn SpawnDerivedClass_(&self, iflags: i32) -> ::windows::core::Result<ISWbemObject> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(iflags), ::core::mem::transmute(&mut result__)).from_abi::<ISWbemObject>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn SpawnInstance_(&self, iflags: i32) -> ::windows::core::Result<ISWbemObject> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), ::core::mem::transmute(iflags), ::core::mem::transmute(&mut result__)).from_abi::<ISWbemObject>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CompareTo_<'a, Param0: ::windows::core::IntoParam<'a, super::Com::IDispatch>>(&self, objwbemobject: Param0, iflags: i32) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), objwbemobject.into_param().abi(), ::core::mem::transmute(iflags), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn Qualifiers_(&self) -> ::windows::core::Result<ISWbemQualifierSet> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ISWbemQualifierSet>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn Properties_(&self) -> ::windows::core::Result<ISWbemPropertySet> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ISWbemPropertySet>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn Methods_(&self) -> ::windows::core::Result<ISWbemMethodSet> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).28)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ISWbemMethodSet>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Derivation_(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).29)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn Path_(&self) -> ::windows::core::Result<ISWbemObjectPath> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).30)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ISWbemObjectPath>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn Security_(&self) -> ::windows::core::Result<ISWbemSecurity> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).31)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ISWbemSecurity>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ISWbemObject> for super::Com::IDispatch {
    fn from(value: ISWbemObject) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ISWbemObject> for super::Com::IDispatch {
    fn from(value: &ISWbemObject) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for ISWbemObject {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &ISWbemObject {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ISWbemObject> for ::windows::core::IUnknown {
    fn from(value: ISWbemObject) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISWbemObject> for ::windows::core::IUnknown {
    fn from(value: &ISWbemObject) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISWbemObject {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ISWbemObject {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISWbemObject {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISWbemObject {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISWbemObject {}
unsafe impl ::windows::core::Interface for ISWbemObject {
    type Vtable = ISWbemObjectVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x76a6415a_cb41_11d1_8b02_00600806d9b6);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISWbemObjectVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemobjectpath: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemsink: ::windows::core::RawPtr, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemasynccontext: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemsink: ::windows::core::RawPtr, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemasynccontext: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemobjectset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemsink: ::windows::core::RawPtr, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemasynccontext: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemobjectset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemsink: ::windows::core::RawPtr, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemasynccontext: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strassocclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strresultclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strresultrole: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strrole: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bclassesonly: i16, bschemaonly: i16, strrequiredassocqualifier: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strrequiredqualifier: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemobjectset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub  unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemsink: ::windows::core::RawPtr, strassocclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strresultclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strresultrole: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strrole: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bclassesonly: i16, bschemaonly: i16, strrequiredassocqualifier: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strrequiredqualifier: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemasynccontext: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strresultclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strrole: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bclassesonly: i16, bschemaonly: i16, strrequiredqualifier: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemobjectset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemsink: ::windows::core::RawPtr, strresultclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strrole: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bclassesonly: i16, bschemaonly: i16, strrequiredqualifier: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemasynccontext: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strmethodname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, objwbeminparameters: ::windows::core::RawPtr, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemoutparameters: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemsink: ::windows::core::RawPtr, strmethodname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, objwbeminparameters: ::windows::core::RawPtr, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemasynccontext: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iflags: i32, strobjecttext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iflags: i32, objwbemobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iflags: i32, objwbemobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemobject: ::windows::core::RawPtr, iflags: i32, bresult: *mut i16) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemqualifierset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbempropertyset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemmethodset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strclassnamearray: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemobjectpath: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemsecurity: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_System_Wmi'*"]
#[repr(transparent)]
pub struct ISWbemObjectEx(::windows::core::IUnknown);
impl ISWbemObjectEx {
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::ITypeInfo>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Put_<'a, Param1: ::windows::core::IntoParam<'a, super::Com::IDispatch>>(&self, iflags: i32, objwbemnamedvalueset: Param1) -> ::windows::core::Result<ISWbemObjectPath> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<ISWbemObjectPath>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PutAsync_<'a, Param0: ::windows::core::IntoParam<'a, super::Com::IDispatch>, Param2: ::windows::core::IntoParam<'a, super::Com::IDispatch>, Param3: ::windows::core::IntoParam<'a, super::Com::IDispatch>>(&self, objwbemsink: Param0, iflags: i32, objwbemnamedvalueset: Param2, objwbemasynccontext: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), objwbemsink.into_param().abi(), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi(), objwbemasynccontext.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Delete_<'a, Param1: ::windows::core::IntoParam<'a, super::Com::IDispatch>>(&self, iflags: i32, objwbemnamedvalueset: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn DeleteAsync_<'a, Param0: ::windows::core::IntoParam<'a, super::Com::IDispatch>, Param2: ::windows::core::IntoParam<'a, super::Com::IDispatch>, Param3: ::windows::core::IntoParam<'a, super::Com::IDispatch>>(&self, objwbemsink: Param0, iflags: i32, objwbemnamedvalueset: Param2, objwbemasynccontext: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), objwbemsink.into_param().abi(), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi(), objwbemasynccontext.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Instances_<'a, Param1: ::windows::core::IntoParam<'a, super::Com::IDispatch>>(&self, iflags: i32, objwbemnamedvalueset: Param1) -> ::windows::core::Result<ISWbemObjectSet> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<ISWbemObjectSet>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InstancesAsync_<'a, Param0: ::windows::core::IntoParam<'a, super::Com::IDispatch>, Param2: ::windows::core::IntoParam<'a, super::Com::IDispatch>, Param3: ::windows::core::IntoParam<'a, super::Com::IDispatch>>(&self, objwbemsink: Param0, iflags: i32, objwbemnamedvalueset: Param2, objwbemasynccontext: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), objwbemsink.into_param().abi(), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi(), objwbemasynccontext.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Subclasses_<'a, Param1: ::windows::core::IntoParam<'a, super::Com::IDispatch>>(&self, iflags: i32, objwbemnamedvalueset: Param1) -> ::windows::core::Result<ISWbemObjectSet> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<ISWbemObjectSet>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SubclassesAsync_<'a, Param0: ::windows::core::IntoParam<'a, super::Com::IDispatch>, Param2: ::windows::core::IntoParam<'a, super::Com::IDispatch>, Param3: ::windows::core::IntoParam<'a, super::Com::IDispatch>>(&self, objwbemsink: Param0, iflags: i32, objwbemnamedvalueset: Param2, objwbemasynccontext: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), objwbemsink.into_param().abi(), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi(), objwbemasynccontext.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn Associators_<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param6: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param7: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param9: ::windows::core::IntoParam<'a, super::Com::IDispatch>>(
        &self,
        strassocclass: Param0,
        strresultclass: Param1,
        strresultrole: Param2,
        strrole: Param3,
        bclassesonly: i16,
        bschemaonly: i16,
        strrequiredassocqualifier: Param6,
        strrequiredqualifier: Param7,
        iflags: i32,
        objwbemnamedvalueset: Param9,
    ) -> ::windows::core::Result<ISWbemObjectSet> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), strassocclass.into_param().abi(), strresultclass.into_param().abi(), strresultrole.into_param().abi(), strrole.into_param().abi(), ::core::mem::transmute(bclassesonly), ::core::mem::transmute(bschemaonly), strrequiredassocqualifier.into_param().abi(), strrequiredqualifier.into_param().abi(), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<ISWbemObjectSet>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn AssociatorsAsync_<'a, Param0: ::windows::core::IntoParam<'a, super::Com::IDispatch>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param7: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param8: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param10: ::windows::core::IntoParam<'a, super::Com::IDispatch>, Param11: ::windows::core::IntoParam<'a, super::Com::IDispatch>>(
        &self,
        objwbemsink: Param0,
        strassocclass: Param1,
        strresultclass: Param2,
        strresultrole: Param3,
        strrole: Param4,
        bclassesonly: i16,
        bschemaonly: i16,
        strrequiredassocqualifier: Param7,
        strrequiredqualifier: Param8,
        iflags: i32,
        objwbemnamedvalueset: Param10,
        objwbemasynccontext: Param11,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), objwbemsink.into_param().abi(), strassocclass.into_param().abi(), strresultclass.into_param().abi(), strresultrole.into_param().abi(), strrole.into_param().abi(), ::core::mem::transmute(bclassesonly), ::core::mem::transmute(bschemaonly), strrequiredassocqualifier.into_param().abi(), strrequiredqualifier.into_param().abi(), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi(), objwbemasynccontext.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn References_<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param6: ::windows::core::IntoParam<'a, super::Com::IDispatch>>(&self, strresultclass: Param0, strrole: Param1, bclassesonly: i16, bschemaonly: i16, strrequiredqualifier: Param4, iflags: i32, objwbemnamedvalueset: Param6) -> ::windows::core::Result<ISWbemObjectSet> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), strresultclass.into_param().abi(), strrole.into_param().abi(), ::core::mem::transmute(bclassesonly), ::core::mem::transmute(bschemaonly), strrequiredqualifier.into_param().abi(), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<ISWbemObjectSet>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn ReferencesAsync_<'a, Param0: ::windows::core::IntoParam<'a, super::Com::IDispatch>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param5: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param7: ::windows::core::IntoParam<'a, super::Com::IDispatch>, Param8: ::windows::core::IntoParam<'a, super::Com::IDispatch>>(&self, objwbemsink: Param0, strresultclass: Param1, strrole: Param2, bclassesonly: i16, bschemaonly: i16, strrequiredqualifier: Param5, iflags: i32, objwbemnamedvalueset: Param7, objwbemasynccontext: Param8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), objwbemsink.into_param().abi(), strresultclass.into_param().abi(), strrole.into_param().abi(), ::core::mem::transmute(bclassesonly), ::core::mem::transmute(bschemaonly), strrequiredqualifier.into_param().abi(), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi(), objwbemasynccontext.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn ExecMethod_<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::Com::IDispatch>, Param3: ::windows::core::IntoParam<'a, super::Com::IDispatch>>(&self, strmethodname: Param0, objwbeminparameters: Param1, iflags: i32, objwbemnamedvalueset: Param3) -> ::windows::core::Result<ISWbemObject> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), strmethodname.into_param().abi(), objwbeminparameters.into_param().abi(), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<ISWbemObject>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn ExecMethodAsync_<'a, Param0: ::windows::core::IntoParam<'a, super::Com::IDispatch>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::Com::IDispatch>, Param4: ::windows::core::IntoParam<'a, super::Com::IDispatch>, Param5: ::windows::core::IntoParam<'a, super::Com::IDispatch>>(&self, objwbemsink: Param0, strmethodname: Param1, objwbeminparameters: Param2, iflags: i32, objwbemnamedvalueset: Param4, objwbemasynccontext: Param5) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), objwbemsink.into_param().abi(), strmethodname.into_param().abi(), objwbeminparameters.into_param().abi(), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi(), objwbemasynccontext.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn Clone_(&self) -> ::windows::core::Result<ISWbemObject> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ISWbemObject>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetObjectText_(&self, iflags: i32) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(iflags), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn SpawnDerivedClass_(&self, iflags: i32) -> ::windows::core::Result<ISWbemObject> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(iflags), ::core::mem::transmute(&mut result__)).from_abi::<ISWbemObject>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn SpawnInstance_(&self, iflags: i32) -> ::windows::core::Result<ISWbemObject> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), ::core::mem::transmute(iflags), ::core::mem::transmute(&mut result__)).from_abi::<ISWbemObject>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CompareTo_<'a, Param0: ::windows::core::IntoParam<'a, super::Com::IDispatch>>(&self, objwbemobject: Param0, iflags: i32) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), objwbemobject.into_param().abi(), ::core::mem::transmute(iflags), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn Qualifiers_(&self) -> ::windows::core::Result<ISWbemQualifierSet> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ISWbemQualifierSet>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn Properties_(&self) -> ::windows::core::Result<ISWbemPropertySet> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ISWbemPropertySet>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn Methods_(&self) -> ::windows::core::Result<ISWbemMethodSet> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).28)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ISWbemMethodSet>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Derivation_(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).29)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn Path_(&self) -> ::windows::core::Result<ISWbemObjectPath> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).30)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ISWbemObjectPath>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn Security_(&self) -> ::windows::core::Result<ISWbemSecurity> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).31)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ISWbemSecurity>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Refresh_<'a, Param1: ::windows::core::IntoParam<'a, super::Com::IDispatch>>(&self, iflags: i32, objwbemnamedvalueset: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).32)(::core::mem::transmute_copy(self), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn SystemProperties_(&self) -> ::windows::core::Result<ISWbemPropertySet> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).33)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ISWbemPropertySet>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetText_<'a, Param2: ::windows::core::IntoParam<'a, super::Com::IDispatch>>(&self, iobjecttextformat: WbemObjectTextFormatEnum, iflags: i32, objwbemnamedvalueset: Param2) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).34)(::core::mem::transmute_copy(self), ::core::mem::transmute(iobjecttextformat), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn SetFromText_<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param3: ::windows::core::IntoParam<'a, super::Com::IDispatch>>(&self, bstext: Param0, iobjecttextformat: WbemObjectTextFormatEnum, iflags: i32, objwbemnamedvalueset: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).35)(::core::mem::transmute_copy(self), bstext.into_param().abi(), ::core::mem::transmute(iobjecttextformat), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi()).ok()
    }
}
impl ::core::convert::From<ISWbemObjectEx> for ISWbemObject {
    fn from(value: ISWbemObjectEx) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISWbemObjectEx> for ISWbemObject {
    fn from(value: &ISWbemObjectEx) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ISWbemObject> for ISWbemObjectEx {
    fn into_param(self) -> ::windows::core::Param<'a, ISWbemObject> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ISWbemObject> for &ISWbemObjectEx {
    fn into_param(self) -> ::windows::core::Param<'a, ISWbemObject> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ISWbemObjectEx> for super::Com::IDispatch {
    fn from(value: ISWbemObjectEx) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ISWbemObjectEx> for super::Com::IDispatch {
    fn from(value: &ISWbemObjectEx) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for ISWbemObjectEx {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &ISWbemObjectEx {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ISWbemObjectEx> for ::windows::core::IUnknown {
    fn from(value: ISWbemObjectEx) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISWbemObjectEx> for ::windows::core::IUnknown {
    fn from(value: &ISWbemObjectEx) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISWbemObjectEx {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ISWbemObjectEx {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISWbemObjectEx {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISWbemObjectEx {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISWbemObjectEx {}
unsafe impl ::windows::core::Interface for ISWbemObjectEx {
    type Vtable = ISWbemObjectExVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x269ad56a_8a67_4129_bc8c_0506dcfe9880);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISWbemObjectExVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemobjectpath: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemsink: ::windows::core::RawPtr, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemasynccontext: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemsink: ::windows::core::RawPtr, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemasynccontext: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemobjectset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemsink: ::windows::core::RawPtr, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemasynccontext: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemobjectset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemsink: ::windows::core::RawPtr, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemasynccontext: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strassocclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strresultclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strresultrole: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strrole: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bclassesonly: i16, bschemaonly: i16, strrequiredassocqualifier: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strrequiredqualifier: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemobjectset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub  unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemsink: ::windows::core::RawPtr, strassocclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strresultclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strresultrole: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strrole: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bclassesonly: i16, bschemaonly: i16, strrequiredassocqualifier: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strrequiredqualifier: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemasynccontext: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strresultclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strrole: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bclassesonly: i16, bschemaonly: i16, strrequiredqualifier: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemobjectset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemsink: ::windows::core::RawPtr, strresultclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strrole: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bclassesonly: i16, bschemaonly: i16, strrequiredqualifier: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemasynccontext: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strmethodname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, objwbeminparameters: ::windows::core::RawPtr, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemoutparameters: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemsink: ::windows::core::RawPtr, strmethodname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, objwbeminparameters: ::windows::core::RawPtr, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemasynccontext: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iflags: i32, strobjecttext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iflags: i32, objwbemobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iflags: i32, objwbemobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemobject: ::windows::core::RawPtr, iflags: i32, bresult: *mut i16) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemqualifierset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbempropertyset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemmethodset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strclassnamearray: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemobjectpath: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemsecurity: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbempropertyset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iobjecttextformat: WbemObjectTextFormatEnum, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, bstext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstext: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iobjecttextformat: WbemObjectTextFormatEnum, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
);
#[doc = "*Required features: 'Win32_System_Wmi'*"]
#[repr(transparent)]
pub struct ISWbemObjectPath(::windows::core::IUnknown);
impl ISWbemObjectPath {
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::ITypeInfo>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Path(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetPath<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, strpath: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), strpath.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RelPath(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetRelPath<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, strrelpath: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), strrelpath.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Server(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetServer<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, strserver: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), strserver.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Namespace(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetNamespace<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, strnamespace: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), strnamespace.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ParentNamespace(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DisplayName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDisplayName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, strdisplayname: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), strdisplayname.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Class(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetClass<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, strclass: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), strclass.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn IsClass(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn SetAsClass(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn IsSingleton(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn SetAsSingleton(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn Keys(&self) -> ::windows::core::Result<ISWbemNamedValueSet> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ISWbemNamedValueSet>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn Security_(&self) -> ::windows::core::Result<ISWbemSecurity> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ISWbemSecurity>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Locale(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLocale<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, strlocale: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self), strlocale.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Authority(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).28)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAuthority<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, strauthority: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).29)(::core::mem::transmute_copy(self), strauthority.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ISWbemObjectPath> for super::Com::IDispatch {
    fn from(value: ISWbemObjectPath) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ISWbemObjectPath> for super::Com::IDispatch {
    fn from(value: &ISWbemObjectPath) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for ISWbemObjectPath {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &ISWbemObjectPath {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ISWbemObjectPath> for ::windows::core::IUnknown {
    fn from(value: ISWbemObjectPath) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISWbemObjectPath> for ::windows::core::IUnknown {
    fn from(value: &ISWbemObjectPath) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISWbemObjectPath {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ISWbemObjectPath {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISWbemObjectPath {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISWbemObjectPath {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISWbemObjectPath {}
unsafe impl ::windows::core::Interface for ISWbemObjectPath {
    type Vtable = ISWbemObjectPathVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5791bc27_ce9c_11d1_97bf_0000f81e849c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISWbemObjectPathVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strpath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strrelpath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strrelpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strserver: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strserver: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strnamespace: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strnamespace: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strparentnamespace: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strdisplayname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strdisplayname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strclass: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bisclass: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bissingleton: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemnamedvalueset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemsecurity: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strlocale: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strlocale: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strauthority: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strauthority: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_System_Wmi'*"]
#[repr(transparent)]
pub struct ISWbemObjectSet(::windows::core::IUnknown);
impl ISWbemObjectSet {
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::ITypeInfo>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Item<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, strobjectpath: Param0, iflags: i32) -> ::windows::core::Result<ISWbemObject> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), strobjectpath.into_param().abi(), ::core::mem::transmute(iflags), ::core::mem::transmute(&mut result__)).from_abi::<ISWbemObject>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn Security_(&self) -> ::windows::core::Result<ISWbemSecurity> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ISWbemSecurity>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn ItemIndex(&self, lindex: i32) -> ::windows::core::Result<ISWbemObject> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(lindex), ::core::mem::transmute(&mut result__)).from_abi::<ISWbemObject>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ISWbemObjectSet> for super::Com::IDispatch {
    fn from(value: ISWbemObjectSet) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ISWbemObjectSet> for super::Com::IDispatch {
    fn from(value: &ISWbemObjectSet) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for ISWbemObjectSet {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &ISWbemObjectSet {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ISWbemObjectSet> for ::windows::core::IUnknown {
    fn from(value: ISWbemObjectSet) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISWbemObjectSet> for ::windows::core::IUnknown {
    fn from(value: &ISWbemObjectSet) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISWbemObjectSet {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ISWbemObjectSet {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISWbemObjectSet {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISWbemObjectSet {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISWbemObjectSet {}
unsafe impl ::windows::core::Interface for ISWbemObjectSet {
    type Vtable = ISWbemObjectSetVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x76a6415f_cb41_11d1_8b02_00600806d9b6);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISWbemObjectSetVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strobjectpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, icount: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemsecurity: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lindex: i32, objwbemobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_System_Wmi'*"]
#[repr(transparent)]
pub struct ISWbemPrivilege(::windows::core::IUnknown);
impl ISWbemPrivilege {
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::ITypeInfo>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn IsEnabled(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn SetIsEnabled(&self, bisenabled: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(bisenabled)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DisplayName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn Identifier(&self) -> ::windows::core::Result<WbemPrivilegeEnum> {
        let mut result__: WbemPrivilegeEnum = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<WbemPrivilegeEnum>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ISWbemPrivilege> for super::Com::IDispatch {
    fn from(value: ISWbemPrivilege) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ISWbemPrivilege> for super::Com::IDispatch {
    fn from(value: &ISWbemPrivilege) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for ISWbemPrivilege {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &ISWbemPrivilege {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ISWbemPrivilege> for ::windows::core::IUnknown {
    fn from(value: ISWbemPrivilege) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISWbemPrivilege> for ::windows::core::IUnknown {
    fn from(value: &ISWbemPrivilege) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISWbemPrivilege {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ISWbemPrivilege {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISWbemPrivilege {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISWbemPrivilege {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISWbemPrivilege {}
unsafe impl ::windows::core::Interface for ISWbemPrivilege {
    type Vtable = ISWbemPrivilegeVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x26ee67bd_5804_11d2_8b4a_00600806d9b6);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISWbemPrivilegeVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bisenabled: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bisenabled: i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strdisplayname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strdisplayname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iprivilege: *mut WbemPrivilegeEnum) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_System_Wmi'*"]
#[repr(transparent)]
pub struct ISWbemPrivilegeSet(::windows::core::IUnknown);
impl ISWbemPrivilegeSet {
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::ITypeInfo>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn Item(&self, iprivilege: WbemPrivilegeEnum) -> ::windows::core::Result<ISWbemPrivilege> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(iprivilege), ::core::mem::transmute(&mut result__)).from_abi::<ISWbemPrivilege>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn Add(&self, iprivilege: WbemPrivilegeEnum, bisenabled: i16) -> ::windows::core::Result<ISWbemPrivilege> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(iprivilege), ::core::mem::transmute(bisenabled), ::core::mem::transmute(&mut result__)).from_abi::<ISWbemPrivilege>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn Remove(&self, iprivilege: WbemPrivilegeEnum) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(iprivilege)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn DeleteAll(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddAsString<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, strprivilege: Param0, bisenabled: i16) -> ::windows::core::Result<ISWbemPrivilege> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), strprivilege.into_param().abi(), ::core::mem::transmute(bisenabled), ::core::mem::transmute(&mut result__)).from_abi::<ISWbemPrivilege>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ISWbemPrivilegeSet> for super::Com::IDispatch {
    fn from(value: ISWbemPrivilegeSet) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ISWbemPrivilegeSet> for super::Com::IDispatch {
    fn from(value: &ISWbemPrivilegeSet) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for ISWbemPrivilegeSet {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &ISWbemPrivilegeSet {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ISWbemPrivilegeSet> for ::windows::core::IUnknown {
    fn from(value: ISWbemPrivilegeSet) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISWbemPrivilegeSet> for ::windows::core::IUnknown {
    fn from(value: &ISWbemPrivilegeSet) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISWbemPrivilegeSet {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ISWbemPrivilegeSet {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISWbemPrivilegeSet {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISWbemPrivilegeSet {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISWbemPrivilegeSet {}
unsafe impl ::windows::core::Interface for ISWbemPrivilegeSet {
    type Vtable = ISWbemPrivilegeSetVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x26ee67bf_5804_11d2_8b4a_00600806d9b6);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISWbemPrivilegeSetVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iprivilege: WbemPrivilegeEnum, objwbemprivilege: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, icount: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iprivilege: WbemPrivilegeEnum, bisenabled: i16, objwbemprivilege: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iprivilege: WbemPrivilegeEnum) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strprivilege: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bisenabled: i16, objwbemprivilege: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_System_Wmi'*"]
#[repr(transparent)]
pub struct ISWbemProperty(::windows::core::IUnknown);
impl ISWbemProperty {
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::ITypeInfo>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Value(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetValue(&self, varvalue: *const super::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(varvalue)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn IsLocal(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Origin(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn CIMType(&self) -> ::windows::core::Result<WbemCimtypeEnum> {
        let mut result__: WbemCimtypeEnum = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<WbemCimtypeEnum>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn Qualifiers_(&self) -> ::windows::core::Result<ISWbemQualifierSet> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ISWbemQualifierSet>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn IsArray(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ISWbemProperty> for super::Com::IDispatch {
    fn from(value: ISWbemProperty) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ISWbemProperty> for super::Com::IDispatch {
    fn from(value: &ISWbemProperty) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for ISWbemProperty {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &ISWbemProperty {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ISWbemProperty> for ::windows::core::IUnknown {
    fn from(value: ISWbemProperty) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISWbemProperty> for ::windows::core::IUnknown {
    fn from(value: &ISWbemProperty) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISWbemProperty {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ISWbemProperty {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISWbemProperty {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISWbemProperty {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISWbemProperty {}
unsafe impl ::windows::core::Interface for ISWbemProperty {
    type Vtable = ISWbemPropertyVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1a388f98_d4ba_11d1_8b09_00600806d9b6);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISWbemPropertyVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varvalue: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varvalue: *const super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bislocal: *mut i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strorigin: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, icimtype: *mut WbemCimtypeEnum) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemqualifierset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bisarray: *mut i16) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_System_Wmi'*"]
#[repr(transparent)]
pub struct ISWbemPropertySet(::windows::core::IUnknown);
impl ISWbemPropertySet {
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::ITypeInfo>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Item<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, strname: Param0, iflags: i32) -> ::windows::core::Result<ISWbemProperty> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), strname.into_param().abi(), ::core::mem::transmute(iflags), ::core::mem::transmute(&mut result__)).from_abi::<ISWbemProperty>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Add<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, strname: Param0, icimtype: WbemCimtypeEnum, bisarray: i16, iflags: i32) -> ::windows::core::Result<ISWbemProperty> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), strname.into_param().abi(), ::core::mem::transmute(icimtype), ::core::mem::transmute(bisarray), ::core::mem::transmute(iflags), ::core::mem::transmute(&mut result__)).from_abi::<ISWbemProperty>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Remove<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, strname: Param0, iflags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), strname.into_param().abi(), ::core::mem::transmute(iflags)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ISWbemPropertySet> for super::Com::IDispatch {
    fn from(value: ISWbemPropertySet) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ISWbemPropertySet> for super::Com::IDispatch {
    fn from(value: &ISWbemPropertySet) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for ISWbemPropertySet {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &ISWbemPropertySet {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ISWbemPropertySet> for ::windows::core::IUnknown {
    fn from(value: ISWbemPropertySet) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISWbemPropertySet> for ::windows::core::IUnknown {
    fn from(value: &ISWbemPropertySet) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISWbemPropertySet {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ISWbemPropertySet {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISWbemPropertySet {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISWbemPropertySet {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISWbemPropertySet {}
unsafe impl ::windows::core::Interface for ISWbemPropertySet {
    type Vtable = ISWbemPropertySetVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdea0a7b2_d4ba_11d1_8b09_00600806d9b6);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISWbemPropertySetVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemproperty: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, icount: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, icimtype: WbemCimtypeEnum, bisarray: i16, iflags: i32, objwbemproperty: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_System_Wmi'*"]
#[repr(transparent)]
pub struct ISWbemQualifier(::windows::core::IUnknown);
impl ISWbemQualifier {
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::ITypeInfo>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Value(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetValue(&self, varvalue: *const super::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(varvalue)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn IsLocal(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn PropagatesToSubclass(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn SetPropagatesToSubclass(&self, bpropagatestosubclass: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(bpropagatestosubclass)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn PropagatesToInstance(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn SetPropagatesToInstance(&self, bpropagatestoinstance: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(bpropagatestoinstance)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn IsOverridable(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn SetIsOverridable(&self, bisoverridable: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(bisoverridable)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn IsAmended(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ISWbemQualifier> for super::Com::IDispatch {
    fn from(value: ISWbemQualifier) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ISWbemQualifier> for super::Com::IDispatch {
    fn from(value: &ISWbemQualifier) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for ISWbemQualifier {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &ISWbemQualifier {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ISWbemQualifier> for ::windows::core::IUnknown {
    fn from(value: ISWbemQualifier) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISWbemQualifier> for ::windows::core::IUnknown {
    fn from(value: &ISWbemQualifier) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISWbemQualifier {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ISWbemQualifier {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISWbemQualifier {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISWbemQualifier {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISWbemQualifier {}
unsafe impl ::windows::core::Interface for ISWbemQualifier {
    type Vtable = ISWbemQualifierVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x79b05932_d3b7_11d1_8b06_00600806d9b6);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISWbemQualifierVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varvalue: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varvalue: *const super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bislocal: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bpropagatestosubclass: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bpropagatestosubclass: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bpropagatestoinstance: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bpropagatestoinstance: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bisoverridable: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bisoverridable: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bisamended: *mut i16) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_System_Wmi'*"]
#[repr(transparent)]
pub struct ISWbemQualifierSet(::windows::core::IUnknown);
impl ISWbemQualifierSet {
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::ITypeInfo>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Item<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0, iflags: i32) -> ::windows::core::Result<ISWbemQualifier> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), name.into_param().abi(), ::core::mem::transmute(iflags), ::core::mem::transmute(&mut result__)).from_abi::<ISWbemQualifier>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Add<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, strname: Param0, varval: *const super::Com::VARIANT, bpropagatestosubclass: i16, bpropagatestoinstance: i16, bisoverridable: i16, iflags: i32) -> ::windows::core::Result<ISWbemQualifier> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), strname.into_param().abi(), ::core::mem::transmute(varval), ::core::mem::transmute(bpropagatestosubclass), ::core::mem::transmute(bpropagatestoinstance), ::core::mem::transmute(bisoverridable), ::core::mem::transmute(iflags), ::core::mem::transmute(&mut result__)).from_abi::<ISWbemQualifier>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Remove<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, strname: Param0, iflags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), strname.into_param().abi(), ::core::mem::transmute(iflags)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ISWbemQualifierSet> for super::Com::IDispatch {
    fn from(value: ISWbemQualifierSet) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ISWbemQualifierSet> for super::Com::IDispatch {
    fn from(value: &ISWbemQualifierSet) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for ISWbemQualifierSet {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &ISWbemQualifierSet {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ISWbemQualifierSet> for ::windows::core::IUnknown {
    fn from(value: ISWbemQualifierSet) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISWbemQualifierSet> for ::windows::core::IUnknown {
    fn from(value: &ISWbemQualifierSet) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISWbemQualifierSet {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ISWbemQualifierSet {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISWbemQualifierSet {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISWbemQualifierSet {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISWbemQualifierSet {}
unsafe impl ::windows::core::Interface for ISWbemQualifierSet {
    type Vtable = ISWbemQualifierSetVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9b16ed16_d3df_11d1_8b08_00600806d9b6);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISWbemQualifierSetVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemqualifier: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, icount: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varval: *const super::Com::VARIANT, bpropagatestosubclass: i16, bpropagatestoinstance: i16, bisoverridable: i16, iflags: i32, objwbemqualifier: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_System_Wmi'*"]
#[repr(transparent)]
pub struct ISWbemRefreshableItem(::windows::core::IUnknown);
impl ISWbemRefreshableItem {
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::ITypeInfo>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn Index(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn Refresher(&self) -> ::windows::core::Result<ISWbemRefresher> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ISWbemRefresher>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn IsSet(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn Object(&self) -> ::windows::core::Result<ISWbemObjectEx> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ISWbemObjectEx>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn ObjectSet(&self) -> ::windows::core::Result<ISWbemObjectSet> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ISWbemObjectSet>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn Remove(&self, iflags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(iflags)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ISWbemRefreshableItem> for super::Com::IDispatch {
    fn from(value: ISWbemRefreshableItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ISWbemRefreshableItem> for super::Com::IDispatch {
    fn from(value: &ISWbemRefreshableItem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for ISWbemRefreshableItem {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &ISWbemRefreshableItem {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ISWbemRefreshableItem> for ::windows::core::IUnknown {
    fn from(value: ISWbemRefreshableItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISWbemRefreshableItem> for ::windows::core::IUnknown {
    fn from(value: &ISWbemRefreshableItem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISWbemRefreshableItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ISWbemRefreshableItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISWbemRefreshableItem {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISWbemRefreshableItem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISWbemRefreshableItem {}
unsafe impl ::windows::core::Interface for ISWbemRefreshableItem {
    type Vtable = ISWbemRefreshableItemVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5ad4bf92_daab_11d3_b38f_00105a1f473a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISWbemRefreshableItemVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iindex: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemrefresher: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bisset: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemobjectset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iflags: i32) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_System_Wmi'*"]
#[repr(transparent)]
pub struct ISWbemRefresher(::windows::core::IUnknown);
impl ISWbemRefresher {
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::ITypeInfo>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn Item(&self, iindex: i32) -> ::windows::core::Result<ISWbemRefreshableItem> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(iindex), ::core::mem::transmute(&mut result__)).from_abi::<ISWbemRefreshableItem>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn Add<'a, Param0: ::windows::core::IntoParam<'a, ISWbemServicesEx>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param3: ::windows::core::IntoParam<'a, super::Com::IDispatch>>(&self, objwbemservices: Param0, bsinstancepath: Param1, iflags: i32, objwbemnamedvalueset: Param3) -> ::windows::core::Result<ISWbemRefreshableItem> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), objwbemservices.into_param().abi(), bsinstancepath.into_param().abi(), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<ISWbemRefreshableItem>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn AddEnum<'a, Param0: ::windows::core::IntoParam<'a, ISWbemServicesEx>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param3: ::windows::core::IntoParam<'a, super::Com::IDispatch>>(&self, objwbemservices: Param0, bsclassname: Param1, iflags: i32, objwbemnamedvalueset: Param3) -> ::windows::core::Result<ISWbemRefreshableItem> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), objwbemservices.into_param().abi(), bsclassname.into_param().abi(), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<ISWbemRefreshableItem>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn Remove(&self, iindex: i32, iflags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(iindex), ::core::mem::transmute(iflags)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn Refresh(&self, iflags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(iflags)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn AutoReconnect(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn SetAutoReconnect(&self, bcount: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(bcount)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn DeleteAll(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ISWbemRefresher> for super::Com::IDispatch {
    fn from(value: ISWbemRefresher) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ISWbemRefresher> for super::Com::IDispatch {
    fn from(value: &ISWbemRefresher) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for ISWbemRefresher {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &ISWbemRefresher {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ISWbemRefresher> for ::windows::core::IUnknown {
    fn from(value: ISWbemRefresher) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISWbemRefresher> for ::windows::core::IUnknown {
    fn from(value: &ISWbemRefresher) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISWbemRefresher {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ISWbemRefresher {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISWbemRefresher {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISWbemRefresher {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISWbemRefresher {}
unsafe impl ::windows::core::Interface for ISWbemRefresher {
    type Vtable = ISWbemRefresherVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x14d8250e_d9c2_11d3_b38f_00105a1f473a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISWbemRefresherVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iindex: i32, objwbemrefreshableitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, icount: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemservices: ::windows::core::RawPtr, bsinstancepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemrefreshableitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemservices: ::windows::core::RawPtr, bsclassname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemrefreshableitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iindex: i32, iflags: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iflags: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bcount: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bcount: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_System_Wmi'*"]
#[repr(transparent)]
pub struct ISWbemSecurity(::windows::core::IUnknown);
impl ISWbemSecurity {
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::ITypeInfo>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn ImpersonationLevel(&self) -> ::windows::core::Result<WbemImpersonationLevelEnum> {
        let mut result__: WbemImpersonationLevelEnum = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<WbemImpersonationLevelEnum>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn SetImpersonationLevel(&self, iimpersonationlevel: WbemImpersonationLevelEnum) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(iimpersonationlevel)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn AuthenticationLevel(&self) -> ::windows::core::Result<WbemAuthenticationLevelEnum> {
        let mut result__: WbemAuthenticationLevelEnum = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<WbemAuthenticationLevelEnum>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn SetAuthenticationLevel(&self, iauthenticationlevel: WbemAuthenticationLevelEnum) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(iauthenticationlevel)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn Privileges(&self) -> ::windows::core::Result<ISWbemPrivilegeSet> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ISWbemPrivilegeSet>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ISWbemSecurity> for super::Com::IDispatch {
    fn from(value: ISWbemSecurity) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ISWbemSecurity> for super::Com::IDispatch {
    fn from(value: &ISWbemSecurity) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for ISWbemSecurity {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &ISWbemSecurity {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ISWbemSecurity> for ::windows::core::IUnknown {
    fn from(value: ISWbemSecurity) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISWbemSecurity> for ::windows::core::IUnknown {
    fn from(value: &ISWbemSecurity) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISWbemSecurity {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ISWbemSecurity {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISWbemSecurity {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISWbemSecurity {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISWbemSecurity {}
unsafe impl ::windows::core::Interface for ISWbemSecurity {
    type Vtable = ISWbemSecurityVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb54d66e6_2287_11d2_8b33_00600806d9b6);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISWbemSecurityVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iimpersonationlevel: *mut WbemImpersonationLevelEnum) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iimpersonationlevel: WbemImpersonationLevelEnum) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iauthenticationlevel: *mut WbemAuthenticationLevelEnum) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iauthenticationlevel: WbemAuthenticationLevelEnum) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemprivilegeset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_System_Wmi'*"]
#[repr(transparent)]
pub struct ISWbemServices(::windows::core::IUnknown);
impl ISWbemServices {
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::ITypeInfo>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn Get<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::Com::IDispatch>>(&self, strobjectpath: Param0, iflags: i32, objwbemnamedvalueset: Param2) -> ::windows::core::Result<ISWbemObject> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), strobjectpath.into_param().abi(), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<ISWbemObject>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetAsync<'a, Param0: ::windows::core::IntoParam<'a, super::Com::IDispatch>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param3: ::windows::core::IntoParam<'a, super::Com::IDispatch>, Param4: ::windows::core::IntoParam<'a, super::Com::IDispatch>>(&self, objwbemsink: Param0, strobjectpath: Param1, iflags: i32, objwbemnamedvalueset: Param3, objwbemasynccontext: Param4) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), objwbemsink.into_param().abi(), strobjectpath.into_param().abi(), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi(), objwbemasynccontext.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn Delete<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::Com::IDispatch>>(&self, strobjectpath: Param0, iflags: i32, objwbemnamedvalueset: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), strobjectpath.into_param().abi(), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn DeleteAsync<'a, Param0: ::windows::core::IntoParam<'a, super::Com::IDispatch>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param3: ::windows::core::IntoParam<'a, super::Com::IDispatch>, Param4: ::windows::core::IntoParam<'a, super::Com::IDispatch>>(&self, objwbemsink: Param0, strobjectpath: Param1, iflags: i32, objwbemnamedvalueset: Param3, objwbemasynccontext: Param4) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), objwbemsink.into_param().abi(), strobjectpath.into_param().abi(), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi(), objwbemasynccontext.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn InstancesOf<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::Com::IDispatch>>(&self, strclass: Param0, iflags: i32, objwbemnamedvalueset: Param2) -> ::windows::core::Result<ISWbemObjectSet> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), strclass.into_param().abi(), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<ISWbemObjectSet>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn InstancesOfAsync<'a, Param0: ::windows::core::IntoParam<'a, super::Com::IDispatch>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param3: ::windows::core::IntoParam<'a, super::Com::IDispatch>, Param4: ::windows::core::IntoParam<'a, super::Com::IDispatch>>(&self, objwbemsink: Param0, strclass: Param1, iflags: i32, objwbemnamedvalueset: Param3, objwbemasynccontext: Param4) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), objwbemsink.into_param().abi(), strclass.into_param().abi(), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi(), objwbemasynccontext.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn SubclassesOf<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::Com::IDispatch>>(&self, strsuperclass: Param0, iflags: i32, objwbemnamedvalueset: Param2) -> ::windows::core::Result<ISWbemObjectSet> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), strsuperclass.into_param().abi(), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<ISWbemObjectSet>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn SubclassesOfAsync<'a, Param0: ::windows::core::IntoParam<'a, super::Com::IDispatch>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param3: ::windows::core::IntoParam<'a, super::Com::IDispatch>, Param4: ::windows::core::IntoParam<'a, super::Com::IDispatch>>(&self, objwbemsink: Param0, strsuperclass: Param1, iflags: i32, objwbemnamedvalueset: Param3, objwbemasynccontext: Param4) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), objwbemsink.into_param().abi(), strsuperclass.into_param().abi(), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi(), objwbemasynccontext.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn ExecQuery<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param3: ::windows::core::IntoParam<'a, super::Com::IDispatch>>(&self, strquery: Param0, strquerylanguage: Param1, iflags: i32, objwbemnamedvalueset: Param3) -> ::windows::core::Result<ISWbemObjectSet> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), strquery.into_param().abi(), strquerylanguage.into_param().abi(), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<ISWbemObjectSet>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn ExecQueryAsync<'a, Param0: ::windows::core::IntoParam<'a, super::Com::IDispatch>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param4: ::windows::core::IntoParam<'a, super::Com::IDispatch>, Param5: ::windows::core::IntoParam<'a, super::Com::IDispatch>>(&self, objwbemsink: Param0, strquery: Param1, strquerylanguage: Param2, lflags: i32, objwbemnamedvalueset: Param4, objwbemasynccontext: Param5) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), objwbemsink.into_param().abi(), strquery.into_param().abi(), strquerylanguage.into_param().abi(), ::core::mem::transmute(lflags), objwbemnamedvalueset.into_param().abi(), objwbemasynccontext.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn AssociatorsOf<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param7: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param8: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param10: ::windows::core::IntoParam<'a, super::Com::IDispatch>>(
        &self,
        strobjectpath: Param0,
        strassocclass: Param1,
        strresultclass: Param2,
        strresultrole: Param3,
        strrole: Param4,
        bclassesonly: i16,
        bschemaonly: i16,
        strrequiredassocqualifier: Param7,
        strrequiredqualifier: Param8,
        iflags: i32,
        objwbemnamedvalueset: Param10,
    ) -> ::windows::core::Result<ISWbemObjectSet> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), strobjectpath.into_param().abi(), strassocclass.into_param().abi(), strresultclass.into_param().abi(), strresultrole.into_param().abi(), strrole.into_param().abi(), ::core::mem::transmute(bclassesonly), ::core::mem::transmute(bschemaonly), strrequiredassocqualifier.into_param().abi(), strrequiredqualifier.into_param().abi(), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<ISWbemObjectSet>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn AssociatorsOfAsync<'a, Param0: ::windows::core::IntoParam<'a, super::Com::IDispatch>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param5: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param8: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param9: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param11: ::windows::core::IntoParam<'a, super::Com::IDispatch>, Param12: ::windows::core::IntoParam<'a, super::Com::IDispatch>>(
        &self,
        objwbemsink: Param0,
        strobjectpath: Param1,
        strassocclass: Param2,
        strresultclass: Param3,
        strresultrole: Param4,
        strrole: Param5,
        bclassesonly: i16,
        bschemaonly: i16,
        strrequiredassocqualifier: Param8,
        strrequiredqualifier: Param9,
        iflags: i32,
        objwbemnamedvalueset: Param11,
        objwbemasynccontext: Param12,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(
            ::core::mem::transmute_copy(self),
            objwbemsink.into_param().abi(),
            strobjectpath.into_param().abi(),
            strassocclass.into_param().abi(),
            strresultclass.into_param().abi(),
            strresultrole.into_param().abi(),
            strrole.into_param().abi(),
            ::core::mem::transmute(bclassesonly),
            ::core::mem::transmute(bschemaonly),
            strrequiredassocqualifier.into_param().abi(),
            strrequiredqualifier.into_param().abi(),
            ::core::mem::transmute(iflags),
            objwbemnamedvalueset.into_param().abi(),
            objwbemasynccontext.into_param().abi(),
        )
        .ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn ReferencesTo<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param5: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param7: ::windows::core::IntoParam<'a, super::Com::IDispatch>>(&self, strobjectpath: Param0, strresultclass: Param1, strrole: Param2, bclassesonly: i16, bschemaonly: i16, strrequiredqualifier: Param5, iflags: i32, objwbemnamedvalueset: Param7) -> ::windows::core::Result<ISWbemObjectSet> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), strobjectpath.into_param().abi(), strresultclass.into_param().abi(), strrole.into_param().abi(), ::core::mem::transmute(bclassesonly), ::core::mem::transmute(bschemaonly), strrequiredqualifier.into_param().abi(), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<ISWbemObjectSet>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn ReferencesToAsync<'a, Param0: ::windows::core::IntoParam<'a, super::Com::IDispatch>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param6: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param8: ::windows::core::IntoParam<'a, super::Com::IDispatch>, Param9: ::windows::core::IntoParam<'a, super::Com::IDispatch>>(&self, objwbemsink: Param0, strobjectpath: Param1, strresultclass: Param2, strrole: Param3, bclassesonly: i16, bschemaonly: i16, strrequiredqualifier: Param6, iflags: i32, objwbemnamedvalueset: Param8, objwbemasynccontext: Param9) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), objwbemsink.into_param().abi(), strobjectpath.into_param().abi(), strresultclass.into_param().abi(), strrole.into_param().abi(), ::core::mem::transmute(bclassesonly), ::core::mem::transmute(bschemaonly), strrequiredqualifier.into_param().abi(), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi(), objwbemasynccontext.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn ExecNotificationQuery<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param3: ::windows::core::IntoParam<'a, super::Com::IDispatch>>(&self, strquery: Param0, strquerylanguage: Param1, iflags: i32, objwbemnamedvalueset: Param3) -> ::windows::core::Result<ISWbemEventSource> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), strquery.into_param().abi(), strquerylanguage.into_param().abi(), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<ISWbemEventSource>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn ExecNotificationQueryAsync<'a, Param0: ::windows::core::IntoParam<'a, super::Com::IDispatch>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param4: ::windows::core::IntoParam<'a, super::Com::IDispatch>, Param5: ::windows::core::IntoParam<'a, super::Com::IDispatch>>(&self, objwbemsink: Param0, strquery: Param1, strquerylanguage: Param2, iflags: i32, objwbemnamedvalueset: Param4, objwbemasynccontext: Param5) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), objwbemsink.into_param().abi(), strquery.into_param().abi(), strquerylanguage.into_param().abi(), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi(), objwbemasynccontext.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn ExecMethod<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::Com::IDispatch>, Param4: ::windows::core::IntoParam<'a, super::Com::IDispatch>>(&self, strobjectpath: Param0, strmethodname: Param1, objwbeminparameters: Param2, iflags: i32, objwbemnamedvalueset: Param4) -> ::windows::core::Result<ISWbemObject> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), strobjectpath.into_param().abi(), strmethodname.into_param().abi(), objwbeminparameters.into_param().abi(), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<ISWbemObject>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn ExecMethodAsync<'a, Param0: ::windows::core::IntoParam<'a, super::Com::IDispatch>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param3: ::windows::core::IntoParam<'a, super::Com::IDispatch>, Param5: ::windows::core::IntoParam<'a, super::Com::IDispatch>, Param6: ::windows::core::IntoParam<'a, super::Com::IDispatch>>(&self, objwbemsink: Param0, strobjectpath: Param1, strmethodname: Param2, objwbeminparameters: Param3, iflags: i32, objwbemnamedvalueset: Param5, objwbemasynccontext: Param6) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), objwbemsink.into_param().abi(), strobjectpath.into_param().abi(), strmethodname.into_param().abi(), objwbeminparameters.into_param().abi(), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi(), objwbemasynccontext.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn Security_(&self) -> ::windows::core::Result<ISWbemSecurity> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ISWbemSecurity>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ISWbemServices> for super::Com::IDispatch {
    fn from(value: ISWbemServices) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ISWbemServices> for super::Com::IDispatch {
    fn from(value: &ISWbemServices) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for ISWbemServices {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &ISWbemServices {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ISWbemServices> for ::windows::core::IUnknown {
    fn from(value: ISWbemServices) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISWbemServices> for ::windows::core::IUnknown {
    fn from(value: &ISWbemServices) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISWbemServices {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ISWbemServices {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISWbemServices {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISWbemServices {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISWbemServices {}
unsafe impl ::windows::core::Interface for ISWbemServices {
    type Vtable = ISWbemServicesVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x76a6415c_cb41_11d1_8b02_00600806d9b6);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISWbemServicesVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strobjectpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemsink: ::windows::core::RawPtr, strobjectpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemasynccontext: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strobjectpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemsink: ::windows::core::RawPtr, strobjectpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemasynccontext: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemobjectset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemsink: ::windows::core::RawPtr, strclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemasynccontext: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strsuperclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemobjectset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemsink: ::windows::core::RawPtr, strsuperclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemasynccontext: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strquery: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strquerylanguage: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemobjectset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemsink: ::windows::core::RawPtr, strquery: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strquerylanguage: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemasynccontext: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub  unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strobjectpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strassocclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strresultclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strresultrole: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strrole: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bclassesonly: i16, bschemaonly: i16, strrequiredassocqualifier: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strrequiredqualifier: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemobjectset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub  unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        objwbemsink: ::windows::core::RawPtr,
        strobjectpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        strassocclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        strresultclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        strresultrole: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        strrole: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        bclassesonly: i16,
        bschemaonly: i16,
        strrequiredassocqualifier: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        strrequiredqualifier: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        iflags: i32,
        objwbemnamedvalueset: ::windows::core::RawPtr,
        objwbemasynccontext: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strobjectpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strresultclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strrole: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bclassesonly: i16, bschemaonly: i16, strrequiredqualifier: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemobjectset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemsink: ::windows::core::RawPtr, strobjectpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strresultclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strrole: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bclassesonly: i16, bschemaonly: i16, strrequiredqualifier: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemasynccontext: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strquery: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strquerylanguage: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemeventsource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemsink: ::windows::core::RawPtr, strquery: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strquerylanguage: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemasynccontext: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strobjectpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strmethodname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, objwbeminparameters: ::windows::core::RawPtr, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemoutparameters: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemsink: ::windows::core::RawPtr, strobjectpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strmethodname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, objwbeminparameters: ::windows::core::RawPtr, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemasynccontext: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemsecurity: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_System_Wmi'*"]
#[repr(transparent)]
pub struct ISWbemServicesEx(::windows::core::IUnknown);
impl ISWbemServicesEx {
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::ITypeInfo>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn Get<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::Com::IDispatch>>(&self, strobjectpath: Param0, iflags: i32, objwbemnamedvalueset: Param2) -> ::windows::core::Result<ISWbemObject> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), strobjectpath.into_param().abi(), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<ISWbemObject>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetAsync<'a, Param0: ::windows::core::IntoParam<'a, super::Com::IDispatch>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param3: ::windows::core::IntoParam<'a, super::Com::IDispatch>, Param4: ::windows::core::IntoParam<'a, super::Com::IDispatch>>(&self, objwbemsink: Param0, strobjectpath: Param1, iflags: i32, objwbemnamedvalueset: Param3, objwbemasynccontext: Param4) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), objwbemsink.into_param().abi(), strobjectpath.into_param().abi(), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi(), objwbemasynccontext.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn Delete<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::Com::IDispatch>>(&self, strobjectpath: Param0, iflags: i32, objwbemnamedvalueset: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), strobjectpath.into_param().abi(), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn DeleteAsync<'a, Param0: ::windows::core::IntoParam<'a, super::Com::IDispatch>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param3: ::windows::core::IntoParam<'a, super::Com::IDispatch>, Param4: ::windows::core::IntoParam<'a, super::Com::IDispatch>>(&self, objwbemsink: Param0, strobjectpath: Param1, iflags: i32, objwbemnamedvalueset: Param3, objwbemasynccontext: Param4) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), objwbemsink.into_param().abi(), strobjectpath.into_param().abi(), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi(), objwbemasynccontext.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn InstancesOf<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::Com::IDispatch>>(&self, strclass: Param0, iflags: i32, objwbemnamedvalueset: Param2) -> ::windows::core::Result<ISWbemObjectSet> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), strclass.into_param().abi(), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<ISWbemObjectSet>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn InstancesOfAsync<'a, Param0: ::windows::core::IntoParam<'a, super::Com::IDispatch>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param3: ::windows::core::IntoParam<'a, super::Com::IDispatch>, Param4: ::windows::core::IntoParam<'a, super::Com::IDispatch>>(&self, objwbemsink: Param0, strclass: Param1, iflags: i32, objwbemnamedvalueset: Param3, objwbemasynccontext: Param4) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), objwbemsink.into_param().abi(), strclass.into_param().abi(), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi(), objwbemasynccontext.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn SubclassesOf<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::Com::IDispatch>>(&self, strsuperclass: Param0, iflags: i32, objwbemnamedvalueset: Param2) -> ::windows::core::Result<ISWbemObjectSet> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), strsuperclass.into_param().abi(), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<ISWbemObjectSet>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn SubclassesOfAsync<'a, Param0: ::windows::core::IntoParam<'a, super::Com::IDispatch>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param3: ::windows::core::IntoParam<'a, super::Com::IDispatch>, Param4: ::windows::core::IntoParam<'a, super::Com::IDispatch>>(&self, objwbemsink: Param0, strsuperclass: Param1, iflags: i32, objwbemnamedvalueset: Param3, objwbemasynccontext: Param4) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), objwbemsink.into_param().abi(), strsuperclass.into_param().abi(), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi(), objwbemasynccontext.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn ExecQuery<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param3: ::windows::core::IntoParam<'a, super::Com::IDispatch>>(&self, strquery: Param0, strquerylanguage: Param1, iflags: i32, objwbemnamedvalueset: Param3) -> ::windows::core::Result<ISWbemObjectSet> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), strquery.into_param().abi(), strquerylanguage.into_param().abi(), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<ISWbemObjectSet>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn ExecQueryAsync<'a, Param0: ::windows::core::IntoParam<'a, super::Com::IDispatch>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param4: ::windows::core::IntoParam<'a, super::Com::IDispatch>, Param5: ::windows::core::IntoParam<'a, super::Com::IDispatch>>(&self, objwbemsink: Param0, strquery: Param1, strquerylanguage: Param2, lflags: i32, objwbemnamedvalueset: Param4, objwbemasynccontext: Param5) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), objwbemsink.into_param().abi(), strquery.into_param().abi(), strquerylanguage.into_param().abi(), ::core::mem::transmute(lflags), objwbemnamedvalueset.into_param().abi(), objwbemasynccontext.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn AssociatorsOf<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param7: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param8: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param10: ::windows::core::IntoParam<'a, super::Com::IDispatch>>(
        &self,
        strobjectpath: Param0,
        strassocclass: Param1,
        strresultclass: Param2,
        strresultrole: Param3,
        strrole: Param4,
        bclassesonly: i16,
        bschemaonly: i16,
        strrequiredassocqualifier: Param7,
        strrequiredqualifier: Param8,
        iflags: i32,
        objwbemnamedvalueset: Param10,
    ) -> ::windows::core::Result<ISWbemObjectSet> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), strobjectpath.into_param().abi(), strassocclass.into_param().abi(), strresultclass.into_param().abi(), strresultrole.into_param().abi(), strrole.into_param().abi(), ::core::mem::transmute(bclassesonly), ::core::mem::transmute(bschemaonly), strrequiredassocqualifier.into_param().abi(), strrequiredqualifier.into_param().abi(), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<ISWbemObjectSet>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn AssociatorsOfAsync<'a, Param0: ::windows::core::IntoParam<'a, super::Com::IDispatch>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param5: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param8: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param9: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param11: ::windows::core::IntoParam<'a, super::Com::IDispatch>, Param12: ::windows::core::IntoParam<'a, super::Com::IDispatch>>(
        &self,
        objwbemsink: Param0,
        strobjectpath: Param1,
        strassocclass: Param2,
        strresultclass: Param3,
        strresultrole: Param4,
        strrole: Param5,
        bclassesonly: i16,
        bschemaonly: i16,
        strrequiredassocqualifier: Param8,
        strrequiredqualifier: Param9,
        iflags: i32,
        objwbemnamedvalueset: Param11,
        objwbemasynccontext: Param12,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(
            ::core::mem::transmute_copy(self),
            objwbemsink.into_param().abi(),
            strobjectpath.into_param().abi(),
            strassocclass.into_param().abi(),
            strresultclass.into_param().abi(),
            strresultrole.into_param().abi(),
            strrole.into_param().abi(),
            ::core::mem::transmute(bclassesonly),
            ::core::mem::transmute(bschemaonly),
            strrequiredassocqualifier.into_param().abi(),
            strrequiredqualifier.into_param().abi(),
            ::core::mem::transmute(iflags),
            objwbemnamedvalueset.into_param().abi(),
            objwbemasynccontext.into_param().abi(),
        )
        .ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn ReferencesTo<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param5: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param7: ::windows::core::IntoParam<'a, super::Com::IDispatch>>(&self, strobjectpath: Param0, strresultclass: Param1, strrole: Param2, bclassesonly: i16, bschemaonly: i16, strrequiredqualifier: Param5, iflags: i32, objwbemnamedvalueset: Param7) -> ::windows::core::Result<ISWbemObjectSet> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), strobjectpath.into_param().abi(), strresultclass.into_param().abi(), strrole.into_param().abi(), ::core::mem::transmute(bclassesonly), ::core::mem::transmute(bschemaonly), strrequiredqualifier.into_param().abi(), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<ISWbemObjectSet>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn ReferencesToAsync<'a, Param0: ::windows::core::IntoParam<'a, super::Com::IDispatch>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param6: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param8: ::windows::core::IntoParam<'a, super::Com::IDispatch>, Param9: ::windows::core::IntoParam<'a, super::Com::IDispatch>>(&self, objwbemsink: Param0, strobjectpath: Param1, strresultclass: Param2, strrole: Param3, bclassesonly: i16, bschemaonly: i16, strrequiredqualifier: Param6, iflags: i32, objwbemnamedvalueset: Param8, objwbemasynccontext: Param9) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), objwbemsink.into_param().abi(), strobjectpath.into_param().abi(), strresultclass.into_param().abi(), strrole.into_param().abi(), ::core::mem::transmute(bclassesonly), ::core::mem::transmute(bschemaonly), strrequiredqualifier.into_param().abi(), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi(), objwbemasynccontext.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn ExecNotificationQuery<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param3: ::windows::core::IntoParam<'a, super::Com::IDispatch>>(&self, strquery: Param0, strquerylanguage: Param1, iflags: i32, objwbemnamedvalueset: Param3) -> ::windows::core::Result<ISWbemEventSource> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), strquery.into_param().abi(), strquerylanguage.into_param().abi(), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<ISWbemEventSource>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn ExecNotificationQueryAsync<'a, Param0: ::windows::core::IntoParam<'a, super::Com::IDispatch>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param4: ::windows::core::IntoParam<'a, super::Com::IDispatch>, Param5: ::windows::core::IntoParam<'a, super::Com::IDispatch>>(&self, objwbemsink: Param0, strquery: Param1, strquerylanguage: Param2, iflags: i32, objwbemnamedvalueset: Param4, objwbemasynccontext: Param5) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), objwbemsink.into_param().abi(), strquery.into_param().abi(), strquerylanguage.into_param().abi(), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi(), objwbemasynccontext.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn ExecMethod<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::Com::IDispatch>, Param4: ::windows::core::IntoParam<'a, super::Com::IDispatch>>(&self, strobjectpath: Param0, strmethodname: Param1, objwbeminparameters: Param2, iflags: i32, objwbemnamedvalueset: Param4) -> ::windows::core::Result<ISWbemObject> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), strobjectpath.into_param().abi(), strmethodname.into_param().abi(), objwbeminparameters.into_param().abi(), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<ISWbemObject>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn ExecMethodAsync<'a, Param0: ::windows::core::IntoParam<'a, super::Com::IDispatch>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param3: ::windows::core::IntoParam<'a, super::Com::IDispatch>, Param5: ::windows::core::IntoParam<'a, super::Com::IDispatch>, Param6: ::windows::core::IntoParam<'a, super::Com::IDispatch>>(&self, objwbemsink: Param0, strobjectpath: Param1, strmethodname: Param2, objwbeminparameters: Param3, iflags: i32, objwbemnamedvalueset: Param5, objwbemasynccontext: Param6) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), objwbemsink.into_param().abi(), strobjectpath.into_param().abi(), strmethodname.into_param().abi(), objwbeminparameters.into_param().abi(), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi(), objwbemasynccontext.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn Security_(&self) -> ::windows::core::Result<ISWbemSecurity> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ISWbemSecurity>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Put<'a, Param0: ::windows::core::IntoParam<'a, ISWbemObjectEx>, Param2: ::windows::core::IntoParam<'a, super::Com::IDispatch>>(&self, objwbemobject: Param0, iflags: i32, objwbemnamedvalueset: Param2) -> ::windows::core::Result<ISWbemObjectPath> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), objwbemobject.into_param().abi(), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<ISWbemObjectPath>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PutAsync<'a, Param0: ::windows::core::IntoParam<'a, ISWbemSink>, Param1: ::windows::core::IntoParam<'a, ISWbemObjectEx>, Param3: ::windows::core::IntoParam<'a, super::Com::IDispatch>, Param4: ::windows::core::IntoParam<'a, super::Com::IDispatch>>(&self, objwbemsink: Param0, objwbemobject: Param1, iflags: i32, objwbemnamedvalueset: Param3, objwbemasynccontext: Param4) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self), objwbemsink.into_param().abi(), objwbemobject.into_param().abi(), ::core::mem::transmute(iflags), objwbemnamedvalueset.into_param().abi(), objwbemasynccontext.into_param().abi()).ok()
    }
}
impl ::core::convert::From<ISWbemServicesEx> for ISWbemServices {
    fn from(value: ISWbemServicesEx) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISWbemServicesEx> for ISWbemServices {
    fn from(value: &ISWbemServicesEx) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ISWbemServices> for ISWbemServicesEx {
    fn into_param(self) -> ::windows::core::Param<'a, ISWbemServices> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ISWbemServices> for &ISWbemServicesEx {
    fn into_param(self) -> ::windows::core::Param<'a, ISWbemServices> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ISWbemServicesEx> for super::Com::IDispatch {
    fn from(value: ISWbemServicesEx) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ISWbemServicesEx> for super::Com::IDispatch {
    fn from(value: &ISWbemServicesEx) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for ISWbemServicesEx {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &ISWbemServicesEx {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ISWbemServicesEx> for ::windows::core::IUnknown {
    fn from(value: ISWbemServicesEx) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISWbemServicesEx> for ::windows::core::IUnknown {
    fn from(value: &ISWbemServicesEx) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISWbemServicesEx {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ISWbemServicesEx {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISWbemServicesEx {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISWbemServicesEx {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISWbemServicesEx {}
unsafe impl ::windows::core::Interface for ISWbemServicesEx {
    type Vtable = ISWbemServicesExVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd2f68443_85dc_427e_91d8_366554cc754c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISWbemServicesExVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strobjectpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemsink: ::windows::core::RawPtr, strobjectpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemasynccontext: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strobjectpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemsink: ::windows::core::RawPtr, strobjectpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemasynccontext: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemobjectset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemsink: ::windows::core::RawPtr, strclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemasynccontext: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strsuperclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemobjectset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemsink: ::windows::core::RawPtr, strsuperclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemasynccontext: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strquery: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strquerylanguage: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemobjectset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemsink: ::windows::core::RawPtr, strquery: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strquerylanguage: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemasynccontext: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub  unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strobjectpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strassocclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strresultclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strresultrole: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strrole: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bclassesonly: i16, bschemaonly: i16, strrequiredassocqualifier: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strrequiredqualifier: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemobjectset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub  unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        objwbemsink: ::windows::core::RawPtr,
        strobjectpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        strassocclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        strresultclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        strresultrole: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        strrole: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        bclassesonly: i16,
        bschemaonly: i16,
        strrequiredassocqualifier: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        strrequiredqualifier: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        iflags: i32,
        objwbemnamedvalueset: ::windows::core::RawPtr,
        objwbemasynccontext: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strobjectpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strresultclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strrole: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bclassesonly: i16, bschemaonly: i16, strrequiredqualifier: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemobjectset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemsink: ::windows::core::RawPtr, strobjectpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strresultclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strrole: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bclassesonly: i16, bschemaonly: i16, strrequiredqualifier: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemasynccontext: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strquery: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strquerylanguage: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemeventsource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemsink: ::windows::core::RawPtr, strquery: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strquerylanguage: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemasynccontext: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strobjectpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strmethodname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, objwbeminparameters: ::windows::core::RawPtr, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemoutparameters: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemsink: ::windows::core::RawPtr, strobjectpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strmethodname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, objwbeminparameters: ::windows::core::RawPtr, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemasynccontext: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemsecurity: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemobject: ::windows::core::RawPtr, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemobjectpath: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemsink: ::windows::core::RawPtr, objwbemobject: ::windows::core::RawPtr, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemasynccontext: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
);
#[doc = "*Required features: 'Win32_System_Wmi'*"]
#[repr(transparent)]
pub struct ISWbemSink(::windows::core::IUnknown);
impl ISWbemSink {
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::ITypeInfo>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn Cancel(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ISWbemSink> for super::Com::IDispatch {
    fn from(value: ISWbemSink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ISWbemSink> for super::Com::IDispatch {
    fn from(value: &ISWbemSink) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for ISWbemSink {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &ISWbemSink {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ISWbemSink> for ::windows::core::IUnknown {
    fn from(value: ISWbemSink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISWbemSink> for ::windows::core::IUnknown {
    fn from(value: &ISWbemSink) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISWbemSink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ISWbemSink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISWbemSink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISWbemSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISWbemSink {}
unsafe impl ::windows::core::Interface for ISWbemSink {
    type Vtable = ISWbemSinkVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x75718c9f_f029_11d1_a1ac_00c04fb6c223);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISWbemSinkVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_System_Wmi'*"]
#[repr(transparent)]
pub struct ISWbemSinkEvents(::windows::core::IUnknown);
impl ISWbemSinkEvents {
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::ITypeInfo>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ISWbemSinkEvents> for super::Com::IDispatch {
    fn from(value: ISWbemSinkEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ISWbemSinkEvents> for super::Com::IDispatch {
    fn from(value: &ISWbemSinkEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for ISWbemSinkEvents {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &ISWbemSinkEvents {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ISWbemSinkEvents> for ::windows::core::IUnknown {
    fn from(value: ISWbemSinkEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISWbemSinkEvents> for ::windows::core::IUnknown {
    fn from(value: &ISWbemSinkEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISWbemSinkEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ISWbemSinkEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISWbemSinkEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISWbemSinkEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISWbemSinkEvents {}
unsafe impl ::windows::core::Interface for ISWbemSinkEvents {
    type Vtable = ISWbemSinkEventsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x75718ca0_f029_11d1_a1ac_00c04fb6c223);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISWbemSinkEventsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
);
#[doc = "*Required features: 'Win32_System_Wmi'*"]
#[repr(transparent)]
pub struct IUnsecuredApartment(::windows::core::IUnknown);
impl IUnsecuredApartment {
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn CreateObjectStub<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, pobject: Param0) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pobject.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
}
impl ::core::convert::From<IUnsecuredApartment> for ::windows::core::IUnknown {
    fn from(value: IUnsecuredApartment) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUnsecuredApartment> for ::windows::core::IUnknown {
    fn from(value: &IUnsecuredApartment) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IUnsecuredApartment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IUnsecuredApartment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IUnsecuredApartment {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUnsecuredApartment {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUnsecuredApartment {}
unsafe impl ::windows::core::Interface for IUnsecuredApartment {
    type Vtable = IUnsecuredApartmentVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1cfaba8c_1523_11d1_ad79_00c04fd8fdff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUnsecuredApartmentVtbl(pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pobject: *mut ::core::ffi::c_void, ppstub: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT);
#[doc = "*Required features: 'Win32_System_Wmi'*"]
#[repr(transparent)]
pub struct IWMIExtension(::windows::core::IUnknown);
impl IWMIExtension {
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::Com::ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::ITypeInfo>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WMIObjectPath(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn GetWMIObject(&self) -> ::windows::core::Result<ISWbemObject> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ISWbemObject>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn GetWMIServices(&self) -> ::windows::core::Result<ISWbemServices> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ISWbemServices>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IWMIExtension> for super::Com::IDispatch {
    fn from(value: IWMIExtension) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IWMIExtension> for super::Com::IDispatch {
    fn from(value: &IWMIExtension) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IWMIExtension {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &IWMIExtension {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMIExtension> for ::windows::core::IUnknown {
    fn from(value: IWMIExtension) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMIExtension> for ::windows::core::IUnknown {
    fn from(value: &IWMIExtension) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMIExtension {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWMIExtension {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMIExtension {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMIExtension {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMIExtension {}
unsafe impl ::windows::core::Interface for IWMIExtension {
    type Vtable = IWMIExtensionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xadc1f06e_5c7e_11d2_8b74_00104b2afb41);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMIExtensionVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strwmiobjectpath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwmiobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwmiservices: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_System_Wmi'*"]
#[repr(transparent)]
pub struct IWbemAddressResolution(::windows::core::IUnknown);
impl IWbemAddressResolution {
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Resolve<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, wsznamespacepath: Param0, wszaddresstype: super::super::Foundation::PWSTR, pdwaddresslength: *mut u32, pabbinaryaddress: *mut *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), wsznamespacepath.into_param().abi(), ::core::mem::transmute(wszaddresstype), ::core::mem::transmute(pdwaddresslength), ::core::mem::transmute(pabbinaryaddress)).ok()
    }
}
impl ::core::convert::From<IWbemAddressResolution> for ::windows::core::IUnknown {
    fn from(value: IWbemAddressResolution) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWbemAddressResolution> for ::windows::core::IUnknown {
    fn from(value: &IWbemAddressResolution) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWbemAddressResolution {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWbemAddressResolution {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWbemAddressResolution {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWbemAddressResolution {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemAddressResolution {}
unsafe impl ::windows::core::Interface for IWbemAddressResolution {
    type Vtable = IWbemAddressResolutionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf7ce2e12_8c90_11d1_9e7b_00c04fc324a8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemAddressResolutionVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wsznamespacepath: super::super::Foundation::PWSTR, wszaddresstype: super::super::Foundation::PWSTR, pdwaddresslength: *mut u32, pabbinaryaddress: *mut *mut u8) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_System_Wmi'*"]
#[repr(transparent)]
pub struct IWbemBackupRestore(::windows::core::IUnknown);
impl IWbemBackupRestore {
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Backup<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, strbackuptofile: Param0, lflags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), strbackuptofile.into_param().abi(), ::core::mem::transmute(lflags)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Restore<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, strrestorefromfile: Param0, lflags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), strrestorefromfile.into_param().abi(), ::core::mem::transmute(lflags)).ok()
    }
}
impl ::core::convert::From<IWbemBackupRestore> for ::windows::core::IUnknown {
    fn from(value: IWbemBackupRestore) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWbemBackupRestore> for ::windows::core::IUnknown {
    fn from(value: &IWbemBackupRestore) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWbemBackupRestore {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWbemBackupRestore {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWbemBackupRestore {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWbemBackupRestore {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemBackupRestore {}
unsafe impl ::windows::core::Interface for IWbemBackupRestore {
    type Vtable = IWbemBackupRestoreVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc49e32c7_bc8b_11d2_85d4_00105a1f8304);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemBackupRestoreVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strbackuptofile: super::super::Foundation::PWSTR, lflags: i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strrestorefromfile: super::super::Foundation::PWSTR, lflags: i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_System_Wmi'*"]
#[repr(transparent)]
pub struct IWbemBackupRestoreEx(::windows::core::IUnknown);
impl IWbemBackupRestoreEx {
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Backup<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, strbackuptofile: Param0, lflags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), strbackuptofile.into_param().abi(), ::core::mem::transmute(lflags)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Restore<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, strrestorefromfile: Param0, lflags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), strrestorefromfile.into_param().abi(), ::core::mem::transmute(lflags)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn Pause(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn Resume(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<IWbemBackupRestoreEx> for IWbemBackupRestore {
    fn from(value: IWbemBackupRestoreEx) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWbemBackupRestoreEx> for IWbemBackupRestore {
    fn from(value: &IWbemBackupRestoreEx) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWbemBackupRestore> for IWbemBackupRestoreEx {
    fn into_param(self) -> ::windows::core::Param<'a, IWbemBackupRestore> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWbemBackupRestore> for &IWbemBackupRestoreEx {
    fn into_param(self) -> ::windows::core::Param<'a, IWbemBackupRestore> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWbemBackupRestoreEx> for ::windows::core::IUnknown {
    fn from(value: IWbemBackupRestoreEx) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWbemBackupRestoreEx> for ::windows::core::IUnknown {
    fn from(value: &IWbemBackupRestoreEx) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWbemBackupRestoreEx {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWbemBackupRestoreEx {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWbemBackupRestoreEx {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWbemBackupRestoreEx {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemBackupRestoreEx {}
unsafe impl ::windows::core::Interface for IWbemBackupRestoreEx {
    type Vtable = IWbemBackupRestoreExVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa359dec5_e813_4834_8a2a_ba7f1d777d76);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemBackupRestoreExVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strbackuptofile: super::super::Foundation::PWSTR, lflags: i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strrestorefromfile: super::super::Foundation::PWSTR, lflags: i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_System_Wmi'*"]
#[repr(transparent)]
pub struct IWbemCallResult(::windows::core::IUnknown);
impl IWbemCallResult {
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn GetResultObject(&self, ltimeout: i32) -> ::windows::core::Result<IWbemClassObject> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(ltimeout), ::core::mem::transmute(&mut result__)).from_abi::<IWbemClassObject>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetResultString(&self, ltimeout: i32) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(ltimeout), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn GetResultServices(&self, ltimeout: i32) -> ::windows::core::Result<IWbemServices> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(ltimeout), ::core::mem::transmute(&mut result__)).from_abi::<IWbemServices>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn GetCallStatus(&self, ltimeout: i32) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(ltimeout), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
}
impl ::core::convert::From<IWbemCallResult> for ::windows::core::IUnknown {
    fn from(value: IWbemCallResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWbemCallResult> for ::windows::core::IUnknown {
    fn from(value: &IWbemCallResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWbemCallResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWbemCallResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWbemCallResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWbemCallResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemCallResult {}
unsafe impl ::windows::core::Interface for IWbemCallResult {
    type Vtable = IWbemCallResultVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x44aca675_e8fc_11d0_a07c_00c04fb68820);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemCallResultVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ltimeout: i32, ppresultobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ltimeout: i32, pstrresultstring: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ltimeout: i32, ppservices: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ltimeout: i32, plstatus: *mut i32) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_System_Wmi'*"]
#[repr(transparent)]
pub struct IWbemClassObject(::windows::core::IUnknown);
impl IWbemClassObject {
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn GetQualifierSet(&self) -> ::windows::core::Result<IWbemQualifierSet> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IWbemQualifierSet>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Get<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, wszname: Param0, lflags: i32, pval: *mut super::Com::VARIANT, ptype: *mut i32, plflavor: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), wszname.into_param().abi(), ::core::mem::transmute(lflags), ::core::mem::transmute(pval), ::core::mem::transmute(ptype), ::core::mem::transmute(plflavor)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Put<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, wszname: Param0, lflags: i32, pval: *const super::Com::VARIANT, r#type: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), wszname.into_param().abi(), ::core::mem::transmute(lflags), ::core::mem::transmute(pval), ::core::mem::transmute(r#type)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Delete<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, wszname: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), wszname.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetNames<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, wszqualifiername: Param0, lflags: i32, pqualifierval: *const super::Com::VARIANT) -> ::windows::core::Result<*mut super::Com::SAFEARRAY> {
        let mut result__: *mut super::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), wszqualifiername.into_param().abi(), ::core::mem::transmute(lflags), ::core::mem::transmute(pqualifierval), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::Com::SAFEARRAY>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn BeginEnumeration(&self, lenumflags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(lenumflags)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Next(&self, lflags: i32, strname: *mut super::super::Foundation::BSTR, pval: *mut super::Com::VARIANT, ptype: *mut i32, plflavor: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(lflags), ::core::mem::transmute(strname), ::core::mem::transmute(pval), ::core::mem::transmute(ptype), ::core::mem::transmute(plflavor)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn EndEnumeration(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPropertyQualifierSet<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, wszproperty: Param0) -> ::windows::core::Result<IWbemQualifierSet> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), wszproperty.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IWbemQualifierSet>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IWbemClassObject> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IWbemClassObject>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetObjectText(&self, lflags: i32) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(lflags), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn SpawnDerivedClass(&self, lflags: i32) -> ::windows::core::Result<IWbemClassObject> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(lflags), ::core::mem::transmute(&mut result__)).from_abi::<IWbemClassObject>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn SpawnInstance(&self, lflags: i32) -> ::windows::core::Result<IWbemClassObject> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(lflags), ::core::mem::transmute(&mut result__)).from_abi::<IWbemClassObject>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn CompareTo<'a, Param1: ::windows::core::IntoParam<'a, IWbemClassObject>>(&self, lflags: i32, pcompareto: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(lflags), pcompareto.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPropertyOrigin<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, wszname: Param0) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), wszname.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InheritsFrom<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, strancestor: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), strancestor.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMethod<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, wszname: Param0, lflags: i32, ppinsignature: *mut ::core::option::Option<IWbemClassObject>, ppoutsignature: *mut ::core::option::Option<IWbemClassObject>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), wszname.into_param().abi(), ::core::mem::transmute(lflags), ::core::mem::transmute(ppinsignature), ::core::mem::transmute(ppoutsignature)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PutMethod<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, IWbemClassObject>, Param3: ::windows::core::IntoParam<'a, IWbemClassObject>>(&self, wszname: Param0, lflags: i32, pinsignature: Param2, poutsignature: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), wszname.into_param().abi(), ::core::mem::transmute(lflags), pinsignature.into_param().abi(), poutsignature.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DeleteMethod<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, wszname: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), wszname.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn BeginMethodEnumeration(&self, lenumflags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(lenumflags)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn NextMethod(&self, lflags: i32, pstrname: *mut super::super::Foundation::BSTR, ppinsignature: *mut ::core::option::Option<IWbemClassObject>, ppoutsignature: *mut ::core::option::Option<IWbemClassObject>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(lflags), ::core::mem::transmute(pstrname), ::core::mem::transmute(ppinsignature), ::core::mem::transmute(ppoutsignature)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn EndMethodEnumeration(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMethodQualifierSet<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, wszmethod: Param0) -> ::windows::core::Result<IWbemQualifierSet> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), wszmethod.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IWbemQualifierSet>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMethodOrigin<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, wszmethodname: Param0) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), wszmethodname.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
}
impl ::core::convert::From<IWbemClassObject> for ::windows::core::IUnknown {
    fn from(value: IWbemClassObject) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWbemClassObject> for ::windows::core::IUnknown {
    fn from(value: &IWbemClassObject) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWbemClassObject {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWbemClassObject {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWbemClassObject {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWbemClassObject {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemClassObject {}
unsafe impl ::windows::core::Interface for IWbemClassObject {
    type Vtable = IWbemClassObjectVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdc12a681_737f_11cf_884d_00aa004b2e24);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemClassObjectVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppqualset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszname: super::super::Foundation::PWSTR, lflags: i32, pval: *mut super::Com::VARIANT, ptype: *mut i32, plflavor: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszname: super::super::Foundation::PWSTR, lflags: i32, pval: *const super::Com::VARIANT, r#type: i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszqualifiername: super::super::Foundation::PWSTR, lflags: i32, pqualifierval: *const super::Com::VARIANT, pnames: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lenumflags: i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, strname: *mut super::super::Foundation::BSTR, pval: *mut super::Com::VARIANT, ptype: *mut i32, plflavor: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszproperty: super::super::Foundation::PWSTR, ppqualset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcopy: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, pstrobjecttext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, ppnewclass: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, ppnewinstance: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, pcompareto: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszname: super::super::Foundation::PWSTR, pstrclassname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strancestor: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszname: super::super::Foundation::PWSTR, lflags: i32, ppinsignature: *mut ::windows::core::RawPtr, ppoutsignature: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszname: super::super::Foundation::PWSTR, lflags: i32, pinsignature: ::windows::core::RawPtr, poutsignature: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lenumflags: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, pstrname: *mut super::super::Foundation::BSTR, ppinsignature: *mut ::windows::core::RawPtr, ppoutsignature: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszmethod: super::super::Foundation::PWSTR, ppqualset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszmethodname: super::super::Foundation::PWSTR, pstrclassname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_System_Wmi'*"]
#[repr(transparent)]
pub struct IWbemClientConnectionTransport(::windows::core::IUnknown);
impl IWbemClientConnectionTransport {
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Open<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param5: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param6: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param8: ::windows::core::IntoParam<'a, IWbemContext>>(&self, straddresstype: Param0, dwbinaryaddresslength: u32, abbinaryaddress: *const u8, strobject: Param3, struser: Param4, strpassword: Param5, strlocale: Param6, lflags: i32, pctx: Param8, riid: *const ::windows::core::GUID, pinterface: *mut *mut ::core::ffi::c_void, pcallres: *mut ::core::option::Option<IWbemCallResult>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), straddresstype.into_param().abi(), ::core::mem::transmute(dwbinaryaddresslength), ::core::mem::transmute(abbinaryaddress), strobject.into_param().abi(), struser.into_param().abi(), strpassword.into_param().abi(), strlocale.into_param().abi(), ::core::mem::transmute(lflags), pctx.into_param().abi(), ::core::mem::transmute(riid), ::core::mem::transmute(pinterface), ::core::mem::transmute(pcallres)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OpenAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param5: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param6: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param8: ::windows::core::IntoParam<'a, IWbemContext>, Param10: ::windows::core::IntoParam<'a, IWbemObjectSink>>(&self, straddresstype: Param0, dwbinaryaddresslength: u32, abbinaryaddress: *const u8, strobject: Param3, struser: Param4, strpassword: Param5, strlocale: Param6, lflags: i32, pctx: Param8, riid: *const ::windows::core::GUID, presponsehandler: Param10) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), straddresstype.into_param().abi(), ::core::mem::transmute(dwbinaryaddresslength), ::core::mem::transmute(abbinaryaddress), strobject.into_param().abi(), struser.into_param().abi(), strpassword.into_param().abi(), strlocale.into_param().abi(), ::core::mem::transmute(lflags), pctx.into_param().abi(), ::core::mem::transmute(riid), presponsehandler.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn Cancel<'a, Param1: ::windows::core::IntoParam<'a, IWbemObjectSink>>(&self, lflags: i32, phandler: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(lflags), phandler.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IWbemClientConnectionTransport> for ::windows::core::IUnknown {
    fn from(value: IWbemClientConnectionTransport) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWbemClientConnectionTransport> for ::windows::core::IUnknown {
    fn from(value: &IWbemClientConnectionTransport) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWbemClientConnectionTransport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWbemClientConnectionTransport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWbemClientConnectionTransport {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWbemClientConnectionTransport {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemClientConnectionTransport {}
unsafe impl ::windows::core::Interface for IWbemClientConnectionTransport {
    type Vtable = IWbemClientConnectionTransportVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa889c72a_fcc1_4a9e_af61_ed071333fb5b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemClientConnectionTransportVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, straddresstype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwbinaryaddresslength: u32, abbinaryaddress: *const u8, strobject: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, struser: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strlocale: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lflags: i32, pctx: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, pinterface: *mut *mut ::core::ffi::c_void, pcallres: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, straddresstype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwbinaryaddresslength: u32, abbinaryaddress: *const u8, strobject: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, struser: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strlocale: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lflags: i32, pctx: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, presponsehandler: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, phandler: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_System_Wmi'*"]
#[repr(transparent)]
pub struct IWbemClientTransport(::windows::core::IUnknown);
impl IWbemClientTransport {
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ConnectServer<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param5: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param6: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param8: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param9: ::windows::core::IntoParam<'a, IWbemContext>>(&self, straddresstype: Param0, dwbinaryaddresslength: u32, abbinaryaddress: *const u8, strnetworkresource: Param3, struser: Param4, strpassword: Param5, strlocale: Param6, lsecurityflags: i32, strauthority: Param8, pctx: Param9) -> ::windows::core::Result<IWbemServices> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), straddresstype.into_param().abi(), ::core::mem::transmute(dwbinaryaddresslength), ::core::mem::transmute(abbinaryaddress), strnetworkresource.into_param().abi(), struser.into_param().abi(), strpassword.into_param().abi(), strlocale.into_param().abi(), ::core::mem::transmute(lsecurityflags), strauthority.into_param().abi(), pctx.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IWbemServices>(result__)
    }
}
impl ::core::convert::From<IWbemClientTransport> for ::windows::core::IUnknown {
    fn from(value: IWbemClientTransport) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWbemClientTransport> for ::windows::core::IUnknown {
    fn from(value: &IWbemClientTransport) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWbemClientTransport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWbemClientTransport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWbemClientTransport {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWbemClientTransport {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemClientTransport {}
unsafe impl ::windows::core::Interface for IWbemClientTransport {
    type Vtable = IWbemClientTransportVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf7ce2e11_8c90_11d1_9e7b_00c04fc324a8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemClientTransportVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, straddresstype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwbinaryaddresslength: u32, abbinaryaddress: *const u8, strnetworkresource: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, struser: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strlocale: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lsecurityflags: i32, strauthority: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pctx: ::windows::core::RawPtr, ppnamespace: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_System_Wmi'*"]
#[repr(transparent)]
pub struct IWbemConfigureRefresher(::windows::core::IUnknown);
impl IWbemConfigureRefresher {
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddObjectByPath<'a, Param0: ::windows::core::IntoParam<'a, IWbemServices>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, IWbemContext>>(&self, pnamespace: Param0, wszpath: Param1, lflags: i32, pcontext: Param3, pprefreshable: *mut ::core::option::Option<IWbemClassObject>, plid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pnamespace.into_param().abi(), wszpath.into_param().abi(), ::core::mem::transmute(lflags), pcontext.into_param().abi(), ::core::mem::transmute(pprefreshable), ::core::mem::transmute(plid)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn AddObjectByTemplate<'a, Param0: ::windows::core::IntoParam<'a, IWbemServices>, Param1: ::windows::core::IntoParam<'a, IWbemClassObject>, Param3: ::windows::core::IntoParam<'a, IWbemContext>>(&self, pnamespace: Param0, ptemplate: Param1, lflags: i32, pcontext: Param3, pprefreshable: *mut ::core::option::Option<IWbemClassObject>, plid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pnamespace.into_param().abi(), ptemplate.into_param().abi(), ::core::mem::transmute(lflags), pcontext.into_param().abi(), ::core::mem::transmute(pprefreshable), ::core::mem::transmute(plid)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn AddRefresher<'a, Param0: ::windows::core::IntoParam<'a, IWbemRefresher>>(&self, prefresher: Param0, lflags: i32, plid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), prefresher.into_param().abi(), ::core::mem::transmute(lflags), ::core::mem::transmute(plid)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn Remove(&self, lid: i32, lflags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(lid), ::core::mem::transmute(lflags)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddEnum<'a, Param0: ::windows::core::IntoParam<'a, IWbemServices>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, IWbemContext>>(&self, pnamespace: Param0, wszclassname: Param1, lflags: i32, pcontext: Param3, ppenum: *mut ::core::option::Option<IWbemHiPerfEnum>, plid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), pnamespace.into_param().abi(), wszclassname.into_param().abi(), ::core::mem::transmute(lflags), pcontext.into_param().abi(), ::core::mem::transmute(ppenum), ::core::mem::transmute(plid)).ok()
    }
}
impl ::core::convert::From<IWbemConfigureRefresher> for ::windows::core::IUnknown {
    fn from(value: IWbemConfigureRefresher) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWbemConfigureRefresher> for ::windows::core::IUnknown {
    fn from(value: &IWbemConfigureRefresher) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWbemConfigureRefresher {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWbemConfigureRefresher {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWbemConfigureRefresher {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWbemConfigureRefresher {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemConfigureRefresher {}
unsafe impl ::windows::core::Interface for IWbemConfigureRefresher {
    type Vtable = IWbemConfigureRefresherVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x49353c92_516b_11d1_aea6_00c04fb68820);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemConfigureRefresherVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnamespace: ::windows::core::RawPtr, wszpath: super::super::Foundation::PWSTR, lflags: i32, pcontext: ::windows::core::RawPtr, pprefreshable: *mut ::windows::core::RawPtr, plid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnamespace: ::windows::core::RawPtr, ptemplate: ::windows::core::RawPtr, lflags: i32, pcontext: ::windows::core::RawPtr, pprefreshable: *mut ::windows::core::RawPtr, plid: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prefresher: ::windows::core::RawPtr, lflags: i32, plid: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lid: i32, lflags: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnamespace: ::windows::core::RawPtr, wszclassname: super::super::Foundation::PWSTR, lflags: i32, pcontext: ::windows::core::RawPtr, ppenum: *mut ::windows::core::RawPtr, plid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_System_Wmi'*"]
#[repr(transparent)]
pub struct IWbemConnectorLogin(::windows::core::IUnknown);
impl IWbemConnectorLogin {
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ConnectorLogin<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, IWbemContext>, T: ::windows::core::Interface>(&self, wsznetworkresource: Param0, wszpreferredlocale: Param1, lflags: i32, pctx: Param3) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), wsznetworkresource.into_param().abi(), wszpreferredlocale.into_param().abi(), ::core::mem::transmute(lflags), pctx.into_param().abi(), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
impl ::core::convert::From<IWbemConnectorLogin> for ::windows::core::IUnknown {
    fn from(value: IWbemConnectorLogin) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWbemConnectorLogin> for ::windows::core::IUnknown {
    fn from(value: &IWbemConnectorLogin) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWbemConnectorLogin {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWbemConnectorLogin {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWbemConnectorLogin {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWbemConnectorLogin {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemConnectorLogin {}
unsafe impl ::windows::core::Interface for IWbemConnectorLogin {
    type Vtable = IWbemConnectorLoginVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd8ec9cb1_b135_4f10_8b1b_c7188bb0d186);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemConnectorLoginVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wsznetworkresource: super::super::Foundation::PWSTR, wszpreferredlocale: super::super::Foundation::PWSTR, lflags: i32, pctx: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, pinterface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_System_Wmi'*"]
#[repr(transparent)]
pub struct IWbemConstructClassObject(::windows::core::IUnknown);
impl IWbemConstructClassObject {
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetInheritanceChain(&self, lnumantecedents: i32, awszantecedents: *const super::super::Foundation::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(lnumantecedents), ::core::mem::transmute(awszantecedents)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetPropertyOrigin<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, wszpropertyname: Param0, loriginindex: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), wszpropertyname.into_param().abi(), ::core::mem::transmute(loriginindex)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetMethodOrigin<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, wszmethodname: Param0, loriginindex: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), wszmethodname.into_param().abi(), ::core::mem::transmute(loriginindex)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetServerNamespace<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, wszserver: Param0, wsznamespace: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), wszserver.into_param().abi(), wsznamespace.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IWbemConstructClassObject> for ::windows::core::IUnknown {
    fn from(value: IWbemConstructClassObject) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWbemConstructClassObject> for ::windows::core::IUnknown {
    fn from(value: &IWbemConstructClassObject) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWbemConstructClassObject {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWbemConstructClassObject {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWbemConstructClassObject {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWbemConstructClassObject {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemConstructClassObject {}
unsafe impl ::windows::core::Interface for IWbemConstructClassObject {
    type Vtable = IWbemConstructClassObjectVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9ef76194_70d5_11d1_ad90_00c04fd8fdff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemConstructClassObjectVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lnumantecedents: i32, awszantecedents: *const super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszpropertyname: super::super::Foundation::PWSTR, loriginindex: i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszmethodname: super::super::Foundation::PWSTR, loriginindex: i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszserver: super::super::Foundation::PWSTR, wsznamespace: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_System_Wmi'*"]
#[repr(transparent)]
pub struct IWbemContext(::windows::core::IUnknown);
impl IWbemContext {
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IWbemContext> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IWbemContext>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetNames(&self, lflags: i32) -> ::windows::core::Result<*mut super::Com::SAFEARRAY> {
        let mut result__: *mut super::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(lflags), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::Com::SAFEARRAY>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn BeginEnumeration(&self, lflags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(lflags)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Next(&self, lflags: i32, pstrname: *mut super::super::Foundation::BSTR, pvalue: *mut super::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(lflags), ::core::mem::transmute(pstrname), ::core::mem::transmute(pvalue)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn EndEnumeration(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetValue<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, wszname: Param0, lflags: i32, pvalue: *const super::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), wszname.into_param().abi(), ::core::mem::transmute(lflags), ::core::mem::transmute(pvalue)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetValue<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, wszname: Param0, lflags: i32) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), wszname.into_param().abi(), ::core::mem::transmute(lflags), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DeleteValue<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, wszname: Param0, lflags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), wszname.into_param().abi(), ::core::mem::transmute(lflags)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn DeleteAll(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<IWbemContext> for ::windows::core::IUnknown {
    fn from(value: IWbemContext) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWbemContext> for ::windows::core::IUnknown {
    fn from(value: &IWbemContext) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWbemContext {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWbemContext {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWbemContext {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWbemContext {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemContext {}
unsafe impl ::windows::core::Interface for IWbemContext {
    type Vtable = IWbemContextVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x44aca674_e8fc_11d0_a07c_00c04fb68820);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemContextVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppnewcopy: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, pnames: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, pstrname: *mut super::super::Foundation::BSTR, pvalue: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszname: super::super::Foundation::PWSTR, lflags: i32, pvalue: *const super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszname: super::super::Foundation::PWSTR, lflags: i32, pvalue: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszname: super::super::Foundation::PWSTR, lflags: i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_System_Wmi'*"]
#[repr(transparent)]
pub struct IWbemDecoupledBasicEventProvider(::windows::core::IUnknown);
impl IWbemDecoupledBasicEventProvider {
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Register<'a, Param1: ::windows::core::IntoParam<'a, IWbemContext>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param5: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param6: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, a_flags: i32, a_context: Param1, a_user: Param2, a_locale: Param3, a_scope: Param4, a_registration: Param5, piunknown: Param6) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(a_flags), a_context.into_param().abi(), a_user.into_param().abi(), a_locale.into_param().abi(), a_scope.into_param().abi(), a_registration.into_param().abi(), piunknown.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn UnRegister(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn GetSink<'a, Param1: ::windows::core::IntoParam<'a, IWbemContext>>(&self, a_flags: i32, a_context: Param1) -> ::windows::core::Result<IWbemObjectSink> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(a_flags), a_context.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IWbemObjectSink>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn GetService<'a, Param1: ::windows::core::IntoParam<'a, IWbemContext>>(&self, a_flags: i32, a_context: Param1) -> ::windows::core::Result<IWbemServices> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(a_flags), a_context.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IWbemServices>(result__)
    }
}
impl ::core::convert::From<IWbemDecoupledBasicEventProvider> for IWbemDecoupledRegistrar {
    fn from(value: IWbemDecoupledBasicEventProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWbemDecoupledBasicEventProvider> for IWbemDecoupledRegistrar {
    fn from(value: &IWbemDecoupledBasicEventProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWbemDecoupledRegistrar> for IWbemDecoupledBasicEventProvider {
    fn into_param(self) -> ::windows::core::Param<'a, IWbemDecoupledRegistrar> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWbemDecoupledRegistrar> for &IWbemDecoupledBasicEventProvider {
    fn into_param(self) -> ::windows::core::Param<'a, IWbemDecoupledRegistrar> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWbemDecoupledBasicEventProvider> for ::windows::core::IUnknown {
    fn from(value: IWbemDecoupledBasicEventProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWbemDecoupledBasicEventProvider> for ::windows::core::IUnknown {
    fn from(value: &IWbemDecoupledBasicEventProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWbemDecoupledBasicEventProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWbemDecoupledBasicEventProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWbemDecoupledBasicEventProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWbemDecoupledBasicEventProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemDecoupledBasicEventProvider {}
unsafe impl ::windows::core::Interface for IWbemDecoupledBasicEventProvider {
    type Vtable = IWbemDecoupledBasicEventProviderVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x86336d20_ca11_4786_9ef1_bc8a946b42fc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemDecoupledBasicEventProviderVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, a_flags: i32, a_context: ::windows::core::RawPtr, a_user: super::super::Foundation::PWSTR, a_locale: super::super::Foundation::PWSTR, a_scope: super::super::Foundation::PWSTR, a_registration: super::super::Foundation::PWSTR, piunknown: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, a_flags: i32, a_context: ::windows::core::RawPtr, a_sink: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, a_flags: i32, a_context: ::windows::core::RawPtr, a_service: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_System_Wmi'*"]
#[repr(transparent)]
pub struct IWbemDecoupledRegistrar(::windows::core::IUnknown);
impl IWbemDecoupledRegistrar {
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Register<'a, Param1: ::windows::core::IntoParam<'a, IWbemContext>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param5: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param6: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, a_flags: i32, a_context: Param1, a_user: Param2, a_locale: Param3, a_scope: Param4, a_registration: Param5, piunknown: Param6) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(a_flags), a_context.into_param().abi(), a_user.into_param().abi(), a_locale.into_param().abi(), a_scope.into_param().abi(), a_registration.into_param().abi(), piunknown.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn UnRegister(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<IWbemDecoupledRegistrar> for ::windows::core::IUnknown {
    fn from(value: IWbemDecoupledRegistrar) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWbemDecoupledRegistrar> for ::windows::core::IUnknown {
    fn from(value: &IWbemDecoupledRegistrar) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWbemDecoupledRegistrar {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWbemDecoupledRegistrar {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWbemDecoupledRegistrar {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWbemDecoupledRegistrar {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemDecoupledRegistrar {}
unsafe impl ::windows::core::Interface for IWbemDecoupledRegistrar {
    type Vtable = IWbemDecoupledRegistrarVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1005cbcf_e64f_4646_bcd3_3a089d8a84b4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemDecoupledRegistrarVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, a_flags: i32, a_context: ::windows::core::RawPtr, a_user: super::super::Foundation::PWSTR, a_locale: super::super::Foundation::PWSTR, a_scope: super::super::Foundation::PWSTR, a_registration: super::super::Foundation::PWSTR, piunknown: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_System_Wmi'*"]
#[repr(transparent)]
pub struct IWbemEventConsumerProvider(::windows::core::IUnknown);
impl IWbemEventConsumerProvider {
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn FindConsumer<'a, Param0: ::windows::core::IntoParam<'a, IWbemClassObject>>(&self, plogicalconsumer: Param0) -> ::windows::core::Result<IWbemUnboundObjectSink> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), plogicalconsumer.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IWbemUnboundObjectSink>(result__)
    }
}
impl ::core::convert::From<IWbemEventConsumerProvider> for ::windows::core::IUnknown {
    fn from(value: IWbemEventConsumerProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWbemEventConsumerProvider> for ::windows::core::IUnknown {
    fn from(value: &IWbemEventConsumerProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWbemEventConsumerProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWbemEventConsumerProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWbemEventConsumerProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWbemEventConsumerProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemEventConsumerProvider {}
unsafe impl ::windows::core::Interface for IWbemEventConsumerProvider {
    type Vtable = IWbemEventConsumerProviderVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe246107a_b06e_11d0_ad61_00c04fd8fdff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemEventConsumerProviderVtbl(pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plogicalconsumer: ::windows::core::RawPtr, ppconsumer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT);
#[doc = "*Required features: 'Win32_System_Wmi'*"]
#[repr(transparent)]
pub struct IWbemEventProvider(::windows::core::IUnknown);
impl IWbemEventProvider {
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn ProvideEvents<'a, Param0: ::windows::core::IntoParam<'a, IWbemObjectSink>>(&self, psink: Param0, lflags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), psink.into_param().abi(), ::core::mem::transmute(lflags)).ok()
    }
}
impl ::core::convert::From<IWbemEventProvider> for ::windows::core::IUnknown {
    fn from(value: IWbemEventProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWbemEventProvider> for ::windows::core::IUnknown {
    fn from(value: &IWbemEventProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWbemEventProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWbemEventProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWbemEventProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWbemEventProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemEventProvider {}
unsafe impl ::windows::core::Interface for IWbemEventProvider {
    type Vtable = IWbemEventProviderVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe245105b_b06e_11d0_ad61_00c04fd8fdff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemEventProviderVtbl(pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psink: ::windows::core::RawPtr, lflags: i32) -> ::windows::core::HRESULT);
#[doc = "*Required features: 'Win32_System_Wmi'*"]
#[repr(transparent)]
pub struct IWbemEventProviderQuerySink(::windows::core::IUnknown);
impl IWbemEventProviderQuerySink {
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn NewQuery(&self, dwid: u32, wszquerylanguage: *const u16, wszquery: *const u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwid), ::core::mem::transmute(wszquerylanguage), ::core::mem::transmute(wszquery)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn CancelQuery(&self, dwid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwid)).ok()
    }
}
impl ::core::convert::From<IWbemEventProviderQuerySink> for ::windows::core::IUnknown {
    fn from(value: IWbemEventProviderQuerySink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWbemEventProviderQuerySink> for ::windows::core::IUnknown {
    fn from(value: &IWbemEventProviderQuerySink) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWbemEventProviderQuerySink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWbemEventProviderQuerySink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWbemEventProviderQuerySink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWbemEventProviderQuerySink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemEventProviderQuerySink {}
unsafe impl ::windows::core::Interface for IWbemEventProviderQuerySink {
    type Vtable = IWbemEventProviderQuerySinkVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x580acaf8_fa1c_11d0_ad72_00c04fd8fdff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemEventProviderQuerySinkVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwid: u32, wszquerylanguage: *const u16, wszquery: *const u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwid: u32) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_System_Wmi'*"]
#[repr(transparent)]
pub struct IWbemEventProviderSecurity(::windows::core::IUnknown);
impl IWbemEventProviderSecurity {
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn AccessCheck(&self, wszquerylanguage: *const u16, wszquery: *const u16, lsidlength: i32, psid: *const u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(wszquerylanguage), ::core::mem::transmute(wszquery), ::core::mem::transmute(lsidlength), ::core::mem::transmute(psid)).ok()
    }
}
impl ::core::convert::From<IWbemEventProviderSecurity> for ::windows::core::IUnknown {
    fn from(value: IWbemEventProviderSecurity) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWbemEventProviderSecurity> for ::windows::core::IUnknown {
    fn from(value: &IWbemEventProviderSecurity) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWbemEventProviderSecurity {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWbemEventProviderSecurity {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWbemEventProviderSecurity {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWbemEventProviderSecurity {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemEventProviderSecurity {}
unsafe impl ::windows::core::Interface for IWbemEventProviderSecurity {
    type Vtable = IWbemEventProviderSecurityVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x631f7d96_d993_11d2_b339_00105a1f4aaf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemEventProviderSecurityVtbl(pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszquerylanguage: *const u16, wszquery: *const u16, lsidlength: i32, psid: *const u8) -> ::windows::core::HRESULT);
#[doc = "*Required features: 'Win32_System_Wmi'*"]
#[repr(transparent)]
pub struct IWbemEventSink(::windows::core::IUnknown);
impl IWbemEventSink {
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn Indicate(&self, lobjectcount: i32, apobjarray: *const ::core::option::Option<IWbemClassObject>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(lobjectcount), ::core::mem::transmute(apobjarray)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetStatus<'a, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param3: ::windows::core::IntoParam<'a, IWbemClassObject>>(&self, lflags: i32, hresult: ::windows::core::HRESULT, strparam: Param2, pobjparam: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(lflags), ::core::mem::transmute(hresult), strparam.into_param().abi(), pobjparam.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn SetSinkSecurity(&self, lsdlength: i32, psd: *const u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(lsdlength), ::core::mem::transmute(psd)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn IsActive(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetRestrictedSink<'a, Param2: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, lnumqueries: i32, awszqueries: *const super::super::Foundation::PWSTR, pcallback: Param2) -> ::windows::core::Result<IWbemEventSink> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(lnumqueries), ::core::mem::transmute(awszqueries), pcallback.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IWbemEventSink>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn SetBatchingParameters(&self, lflags: i32, dwmaxbuffersize: u32, dwmaxsendlatency: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(lflags), ::core::mem::transmute(dwmaxbuffersize), ::core::mem::transmute(dwmaxsendlatency)).ok()
    }
}
impl ::core::convert::From<IWbemEventSink> for IWbemObjectSink {
    fn from(value: IWbemEventSink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWbemEventSink> for IWbemObjectSink {
    fn from(value: &IWbemEventSink) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWbemObjectSink> for IWbemEventSink {
    fn into_param(self) -> ::windows::core::Param<'a, IWbemObjectSink> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWbemObjectSink> for &IWbemEventSink {
    fn into_param(self) -> ::windows::core::Param<'a, IWbemObjectSink> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWbemEventSink> for ::windows::core::IUnknown {
    fn from(value: IWbemEventSink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWbemEventSink> for ::windows::core::IUnknown {
    fn from(value: &IWbemEventSink) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWbemEventSink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWbemEventSink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWbemEventSink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWbemEventSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemEventSink {}
unsafe impl ::windows::core::Interface for IWbemEventSink {
    type Vtable = IWbemEventSinkVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3ae0080a_7e3a_4366_bf89_0feedc931659);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemEventSinkVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lobjectcount: i32, apobjarray: *const ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, hresult: ::windows::core::HRESULT, strparam: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pobjparam: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lsdlength: i32, psd: *const u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lnumqueries: i32, awszqueries: *const super::super::Foundation::PWSTR, pcallback: *mut ::core::ffi::c_void, ppsink: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, dwmaxbuffersize: u32, dwmaxsendlatency: u32) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_System_Wmi'*"]
#[repr(transparent)]
pub struct IWbemHiPerfEnum(::windows::core::IUnknown);
impl IWbemHiPerfEnum {
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn AddObjects(&self, lflags: i32, unumobjects: u32, apids: *const i32, apobj: *const ::core::option::Option<IWbemObjectAccess>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(lflags), ::core::mem::transmute(unumobjects), ::core::mem::transmute(apids), ::core::mem::transmute(apobj)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn RemoveObjects(&self, lflags: i32, unumobjects: u32, apids: *const i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(lflags), ::core::mem::transmute(unumobjects), ::core::mem::transmute(apids)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn GetObjects(&self, lflags: i32, unumobjects: u32, apobj: *mut ::core::option::Option<IWbemObjectAccess>, pureturned: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(lflags), ::core::mem::transmute(unumobjects), ::core::mem::transmute(apobj), ::core::mem::transmute(pureturned)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn RemoveAll(&self, lflags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(lflags)).ok()
    }
}
impl ::core::convert::From<IWbemHiPerfEnum> for ::windows::core::IUnknown {
    fn from(value: IWbemHiPerfEnum) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWbemHiPerfEnum> for ::windows::core::IUnknown {
    fn from(value: &IWbemHiPerfEnum) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWbemHiPerfEnum {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWbemHiPerfEnum {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWbemHiPerfEnum {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWbemHiPerfEnum {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemHiPerfEnum {}
unsafe impl ::windows::core::Interface for IWbemHiPerfEnum {
    type Vtable = IWbemHiPerfEnumVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2705c288_79ae_11d2_b348_00105a1f8177);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemHiPerfEnumVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, unumobjects: u32, apids: *const i32, apobj: *const ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, unumobjects: u32, apids: *const i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, unumobjects: u32, apobj: *mut ::windows::core::RawPtr, pureturned: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_System_Wmi'*"]
#[repr(transparent)]
pub struct IWbemHiPerfProvider(::windows::core::IUnknown);
impl IWbemHiPerfProvider {
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn QueryInstances<'a, Param0: ::windows::core::IntoParam<'a, IWbemServices>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, IWbemContext>>(&self, pnamespace: Param0, wszclass: Param1, lflags: i32, pctx: Param3, psink: ::core::option::Option<IWbemObjectSink>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pnamespace.into_param().abi(), wszclass.into_param().abi(), ::core::mem::transmute(lflags), pctx.into_param().abi(), ::core::mem::transmute(psink)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn CreateRefresher<'a, Param0: ::windows::core::IntoParam<'a, IWbemServices>>(&self, pnamespace: Param0, lflags: i32) -> ::windows::core::Result<IWbemRefresher> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pnamespace.into_param().abi(), ::core::mem::transmute(lflags), ::core::mem::transmute(&mut result__)).from_abi::<IWbemRefresher>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn CreateRefreshableObject<'a, Param0: ::windows::core::IntoParam<'a, IWbemServices>, Param1: ::windows::core::IntoParam<'a, IWbemObjectAccess>, Param2: ::windows::core::IntoParam<'a, IWbemRefresher>, Param4: ::windows::core::IntoParam<'a, IWbemContext>>(&self, pnamespace: Param0, ptemplate: Param1, prefresher: Param2, lflags: i32, pcontext: Param4, pprefreshable: *mut ::core::option::Option<IWbemObjectAccess>, plid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), pnamespace.into_param().abi(), ptemplate.into_param().abi(), prefresher.into_param().abi(), ::core::mem::transmute(lflags), pcontext.into_param().abi(), ::core::mem::transmute(pprefreshable), ::core::mem::transmute(plid)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn StopRefreshing<'a, Param0: ::windows::core::IntoParam<'a, IWbemRefresher>>(&self, prefresher: Param0, lid: i32, lflags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), prefresher.into_param().abi(), ::core::mem::transmute(lid), ::core::mem::transmute(lflags)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateRefreshableEnum<'a, Param0: ::windows::core::IntoParam<'a, IWbemServices>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, IWbemRefresher>, Param4: ::windows::core::IntoParam<'a, IWbemContext>, Param5: ::windows::core::IntoParam<'a, IWbemHiPerfEnum>>(&self, pnamespace: Param0, wszclass: Param1, prefresher: Param2, lflags: i32, pcontext: Param4, phiperfenum: Param5) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), pnamespace.into_param().abi(), wszclass.into_param().abi(), prefresher.into_param().abi(), ::core::mem::transmute(lflags), pcontext.into_param().abi(), phiperfenum.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn GetObjects<'a, Param0: ::windows::core::IntoParam<'a, IWbemServices>, Param4: ::windows::core::IntoParam<'a, IWbemContext>>(&self, pnamespace: Param0, lnumobjects: i32, apobj: *mut ::core::option::Option<IWbemObjectAccess>, lflags: i32, pcontext: Param4) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), pnamespace.into_param().abi(), ::core::mem::transmute(lnumobjects), ::core::mem::transmute(apobj), ::core::mem::transmute(lflags), pcontext.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IWbemHiPerfProvider> for ::windows::core::IUnknown {
    fn from(value: IWbemHiPerfProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWbemHiPerfProvider> for ::windows::core::IUnknown {
    fn from(value: &IWbemHiPerfProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWbemHiPerfProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWbemHiPerfProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWbemHiPerfProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWbemHiPerfProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemHiPerfProvider {}
unsafe impl ::windows::core::Interface for IWbemHiPerfProvider {
    type Vtable = IWbemHiPerfProviderVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x49353c93_516b_11d1_aea6_00c04fb68820);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemHiPerfProviderVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnamespace: ::windows::core::RawPtr, wszclass: super::super::Foundation::PWSTR, lflags: i32, pctx: ::windows::core::RawPtr, psink: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnamespace: ::windows::core::RawPtr, lflags: i32, pprefresher: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnamespace: ::windows::core::RawPtr, ptemplate: ::windows::core::RawPtr, prefresher: ::windows::core::RawPtr, lflags: i32, pcontext: ::windows::core::RawPtr, pprefreshable: *mut ::windows::core::RawPtr, plid: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prefresher: ::windows::core::RawPtr, lid: i32, lflags: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnamespace: ::windows::core::RawPtr, wszclass: super::super::Foundation::PWSTR, prefresher: ::windows::core::RawPtr, lflags: i32, pcontext: ::windows::core::RawPtr, phiperfenum: ::windows::core::RawPtr, plid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnamespace: ::windows::core::RawPtr, lnumobjects: i32, apobj: *mut ::windows::core::RawPtr, lflags: i32, pcontext: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_System_Wmi'*"]
#[repr(transparent)]
pub struct IWbemLevel1Login(::windows::core::IUnknown);
impl IWbemLevel1Login {
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EstablishPosition<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, wszlocalelist: Param0, dwnumlocales: u32) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), wszlocalelist.into_param().abi(), ::core::mem::transmute(dwnumlocales), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RequestChallenge<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, wsznetworkresource: Param0, wszuser: Param1) -> ::windows::core::Result<u8> {
        let mut result__: u8 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), wsznetworkresource.into_param().abi(), wszuser.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<u8>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WBEMLogin<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, IWbemContext>>(&self, wszpreferredlocale: Param0, accesstoken: *const u8, lflags: i32, pctx: Param3) -> ::windows::core::Result<IWbemServices> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), wszpreferredlocale.into_param().abi(), ::core::mem::transmute(accesstoken), ::core::mem::transmute(lflags), pctx.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IWbemServices>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn NTLMLogin<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, IWbemContext>>(&self, wsznetworkresource: Param0, wszpreferredlocale: Param1, lflags: i32, pctx: Param3) -> ::windows::core::Result<IWbemServices> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), wsznetworkresource.into_param().abi(), wszpreferredlocale.into_param().abi(), ::core::mem::transmute(lflags), pctx.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IWbemServices>(result__)
    }
}
impl ::core::convert::From<IWbemLevel1Login> for ::windows::core::IUnknown {
    fn from(value: IWbemLevel1Login) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWbemLevel1Login> for ::windows::core::IUnknown {
    fn from(value: &IWbemLevel1Login) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWbemLevel1Login {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWbemLevel1Login {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWbemLevel1Login {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWbemLevel1Login {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemLevel1Login {}
unsafe impl ::windows::core::Interface for IWbemLevel1Login {
    type Vtable = IWbemLevel1LoginVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf309ad18_d86a_11d0_a075_00c04fb68820);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemLevel1LoginVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszlocalelist: super::super::Foundation::PWSTR, dwnumlocales: u32, reserved: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wsznetworkresource: super::super::Foundation::PWSTR, wszuser: super::super::Foundation::PWSTR, nonce: *mut u8) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszpreferredlocale: super::super::Foundation::PWSTR, accesstoken: *const u8, lflags: i32, pctx: ::windows::core::RawPtr, ppnamespace: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wsznetworkresource: super::super::Foundation::PWSTR, wszpreferredlocale: super::super::Foundation::PWSTR, lflags: i32, pctx: ::windows::core::RawPtr, ppnamespace: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_System_Wmi'*"]
#[repr(transparent)]
pub struct IWbemLocator(::windows::core::IUnknown);
impl IWbemLocator {
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ConnectServer<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param5: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param6: ::windows::core::IntoParam<'a, IWbemContext>>(&self, strnetworkresource: Param0, struser: Param1, strpassword: Param2, strlocale: Param3, lsecurityflags: i32, strauthority: Param5, pctx: Param6) -> ::windows::core::Result<IWbemServices> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), strnetworkresource.into_param().abi(), struser.into_param().abi(), strpassword.into_param().abi(), strlocale.into_param().abi(), ::core::mem::transmute(lsecurityflags), strauthority.into_param().abi(), pctx.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IWbemServices>(result__)
    }
}
impl ::core::convert::From<IWbemLocator> for ::windows::core::IUnknown {
    fn from(value: IWbemLocator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWbemLocator> for ::windows::core::IUnknown {
    fn from(value: &IWbemLocator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWbemLocator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWbemLocator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWbemLocator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWbemLocator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemLocator {}
unsafe impl ::windows::core::Interface for IWbemLocator {
    type Vtable = IWbemLocatorVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdc12a687_737f_11cf_884d_00aa004b2e24);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemLocatorVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strnetworkresource: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, struser: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strlocale: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lsecurityflags: i32, strauthority: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pctx: ::windows::core::RawPtr, ppnamespace: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_System_Wmi'*"]
#[repr(transparent)]
pub struct IWbemObjectAccess(::windows::core::IUnknown);
impl IWbemObjectAccess {
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn GetQualifierSet(&self) -> ::windows::core::Result<IWbemQualifierSet> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IWbemQualifierSet>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Get<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, wszname: Param0, lflags: i32, pval: *mut super::Com::VARIANT, ptype: *mut i32, plflavor: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), wszname.into_param().abi(), ::core::mem::transmute(lflags), ::core::mem::transmute(pval), ::core::mem::transmute(ptype), ::core::mem::transmute(plflavor)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Put<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, wszname: Param0, lflags: i32, pval: *const super::Com::VARIANT, r#type: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), wszname.into_param().abi(), ::core::mem::transmute(lflags), ::core::mem::transmute(pval), ::core::mem::transmute(r#type)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Delete<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, wszname: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), wszname.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetNames<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, wszqualifiername: Param0, lflags: i32, pqualifierval: *const super::Com::VARIANT) -> ::windows::core::Result<*mut super::Com::SAFEARRAY> {
        let mut result__: *mut super::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), wszqualifiername.into_param().abi(), ::core::mem::transmute(lflags), ::core::mem::transmute(pqualifierval), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::Com::SAFEARRAY>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn BeginEnumeration(&self, lenumflags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(lenumflags)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Next(&self, lflags: i32, strname: *mut super::super::Foundation::BSTR, pval: *mut super::Com::VARIANT, ptype: *mut i32, plflavor: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(lflags), ::core::mem::transmute(strname), ::core::mem::transmute(pval), ::core::mem::transmute(ptype), ::core::mem::transmute(plflavor)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn EndEnumeration(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPropertyQualifierSet<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, wszproperty: Param0) -> ::windows::core::Result<IWbemQualifierSet> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), wszproperty.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IWbemQualifierSet>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IWbemClassObject> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IWbemClassObject>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetObjectText(&self, lflags: i32) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(lflags), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn SpawnDerivedClass(&self, lflags: i32) -> ::windows::core::Result<IWbemClassObject> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(lflags), ::core::mem::transmute(&mut result__)).from_abi::<IWbemClassObject>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn SpawnInstance(&self, lflags: i32) -> ::windows::core::Result<IWbemClassObject> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(lflags), ::core::mem::transmute(&mut result__)).from_abi::<IWbemClassObject>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn CompareTo<'a, Param1: ::windows::core::IntoParam<'a, IWbemClassObject>>(&self, lflags: i32, pcompareto: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(lflags), pcompareto.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPropertyOrigin<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, wszname: Param0) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), wszname.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InheritsFrom<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, strancestor: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), strancestor.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMethod<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, wszname: Param0, lflags: i32, ppinsignature: *mut ::core::option::Option<IWbemClassObject>, ppoutsignature: *mut ::core::option::Option<IWbemClassObject>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), wszname.into_param().abi(), ::core::mem::transmute(lflags), ::core::mem::transmute(ppinsignature), ::core::mem::transmute(ppoutsignature)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PutMethod<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, IWbemClassObject>, Param3: ::windows::core::IntoParam<'a, IWbemClassObject>>(&self, wszname: Param0, lflags: i32, pinsignature: Param2, poutsignature: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), wszname.into_param().abi(), ::core::mem::transmute(lflags), pinsignature.into_param().abi(), poutsignature.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DeleteMethod<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, wszname: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), wszname.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn BeginMethodEnumeration(&self, lenumflags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(lenumflags)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn NextMethod(&self, lflags: i32, pstrname: *mut super::super::Foundation::BSTR, ppinsignature: *mut ::core::option::Option<IWbemClassObject>, ppoutsignature: *mut ::core::option::Option<IWbemClassObject>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(lflags), ::core::mem::transmute(pstrname), ::core::mem::transmute(ppinsignature), ::core::mem::transmute(ppoutsignature)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn EndMethodEnumeration(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMethodQualifierSet<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, wszmethod: Param0) -> ::windows::core::Result<IWbemQualifierSet> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), wszmethod.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IWbemQualifierSet>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMethodOrigin<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, wszmethodname: Param0) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), wszmethodname.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPropertyHandle<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, wszpropertyname: Param0, ptype: *mut i32, plhandle: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self), wszpropertyname.into_param().abi(), ::core::mem::transmute(ptype), ::core::mem::transmute(plhandle)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn WritePropertyValue(&self, lhandle: i32, lnumbytes: i32, adata: *const u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).28)(::core::mem::transmute_copy(self), ::core::mem::transmute(lhandle), ::core::mem::transmute(lnumbytes), ::core::mem::transmute(adata)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn ReadPropertyValue(&self, lhandle: i32, lbuffersize: i32, plnumbytes: *mut i32, adata: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).29)(::core::mem::transmute_copy(self), ::core::mem::transmute(lhandle), ::core::mem::transmute(lbuffersize), ::core::mem::transmute(plnumbytes), ::core::mem::transmute(adata)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn ReadDWORD(&self, lhandle: i32) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).30)(::core::mem::transmute_copy(self), ::core::mem::transmute(lhandle), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn WriteDWORD(&self, lhandle: i32, dw: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).31)(::core::mem::transmute_copy(self), ::core::mem::transmute(lhandle), ::core::mem::transmute(dw)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn ReadQWORD(&self, lhandle: i32) -> ::windows::core::Result<u64> {
        let mut result__: u64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).32)(::core::mem::transmute_copy(self), ::core::mem::transmute(lhandle), ::core::mem::transmute(&mut result__)).from_abi::<u64>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn WriteQWORD(&self, lhandle: i32, pw: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).33)(::core::mem::transmute_copy(self), ::core::mem::transmute(lhandle), ::core::mem::transmute(pw)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPropertyInfoByHandle(&self, lhandle: i32, pstrname: *mut super::super::Foundation::BSTR, ptype: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).34)(::core::mem::transmute_copy(self), ::core::mem::transmute(lhandle), ::core::mem::transmute(pstrname), ::core::mem::transmute(ptype)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn Lock(&self, lflags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).35)(::core::mem::transmute_copy(self), ::core::mem::transmute(lflags)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn Unlock(&self, lflags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).36)(::core::mem::transmute_copy(self), ::core::mem::transmute(lflags)).ok()
    }
}
impl ::core::convert::From<IWbemObjectAccess> for IWbemClassObject {
    fn from(value: IWbemObjectAccess) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWbemObjectAccess> for IWbemClassObject {
    fn from(value: &IWbemObjectAccess) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWbemClassObject> for IWbemObjectAccess {
    fn into_param(self) -> ::windows::core::Param<'a, IWbemClassObject> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWbemClassObject> for &IWbemObjectAccess {
    fn into_param(self) -> ::windows::core::Param<'a, IWbemClassObject> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWbemObjectAccess> for ::windows::core::IUnknown {
    fn from(value: IWbemObjectAccess) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWbemObjectAccess> for ::windows::core::IUnknown {
    fn from(value: &IWbemObjectAccess) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWbemObjectAccess {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWbemObjectAccess {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWbemObjectAccess {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWbemObjectAccess {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemObjectAccess {}
unsafe impl ::windows::core::Interface for IWbemObjectAccess {
    type Vtable = IWbemObjectAccessVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x49353c9a_516b_11d1_aea6_00c04fb68820);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemObjectAccessVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppqualset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszname: super::super::Foundation::PWSTR, lflags: i32, pval: *mut super::Com::VARIANT, ptype: *mut i32, plflavor: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszname: super::super::Foundation::PWSTR, lflags: i32, pval: *const super::Com::VARIANT, r#type: i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszqualifiername: super::super::Foundation::PWSTR, lflags: i32, pqualifierval: *const super::Com::VARIANT, pnames: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lenumflags: i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, strname: *mut super::super::Foundation::BSTR, pval: *mut super::Com::VARIANT, ptype: *mut i32, plflavor: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszproperty: super::super::Foundation::PWSTR, ppqualset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcopy: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, pstrobjecttext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, ppnewclass: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, ppnewinstance: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, pcompareto: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszname: super::super::Foundation::PWSTR, pstrclassname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strancestor: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszname: super::super::Foundation::PWSTR, lflags: i32, ppinsignature: *mut ::windows::core::RawPtr, ppoutsignature: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszname: super::super::Foundation::PWSTR, lflags: i32, pinsignature: ::windows::core::RawPtr, poutsignature: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lenumflags: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, pstrname: *mut super::super::Foundation::BSTR, ppinsignature: *mut ::windows::core::RawPtr, ppoutsignature: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszmethod: super::super::Foundation::PWSTR, ppqualset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszmethodname: super::super::Foundation::PWSTR, pstrclassname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszpropertyname: super::super::Foundation::PWSTR, ptype: *mut i32, plhandle: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lhandle: i32, lnumbytes: i32, adata: *const u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lhandle: i32, lbuffersize: i32, plnumbytes: *mut i32, adata: *mut u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lhandle: i32, pdw: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lhandle: i32, dw: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lhandle: i32, pqw: *mut u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lhandle: i32, pw: u64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lhandle: i32, pstrname: *mut super::super::Foundation::BSTR, ptype: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_System_Wmi'*"]
#[repr(transparent)]
pub struct IWbemObjectSink(::windows::core::IUnknown);
impl IWbemObjectSink {
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn Indicate(&self, lobjectcount: i32, apobjarray: *const ::core::option::Option<IWbemClassObject>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(lobjectcount), ::core::mem::transmute(apobjarray)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetStatus<'a, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param3: ::windows::core::IntoParam<'a, IWbemClassObject>>(&self, lflags: i32, hresult: ::windows::core::HRESULT, strparam: Param2, pobjparam: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(lflags), ::core::mem::transmute(hresult), strparam.into_param().abi(), pobjparam.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IWbemObjectSink> for ::windows::core::IUnknown {
    fn from(value: IWbemObjectSink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWbemObjectSink> for ::windows::core::IUnknown {
    fn from(value: &IWbemObjectSink) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWbemObjectSink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWbemObjectSink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWbemObjectSink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWbemObjectSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemObjectSink {}
unsafe impl ::windows::core::Interface for IWbemObjectSink {
    type Vtable = IWbemObjectSinkVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7c857801_7381_11cf_884d_00aa004b2e24);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemObjectSinkVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lobjectcount: i32, apobjarray: *const ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, hresult: ::windows::core::HRESULT, strparam: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pobjparam: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_System_Wmi'*"]
#[repr(transparent)]
pub struct IWbemObjectSinkEx(::windows::core::IUnknown);
impl IWbemObjectSinkEx {
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn Indicate(&self, lobjectcount: i32, apobjarray: *const ::core::option::Option<IWbemClassObject>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(lobjectcount), ::core::mem::transmute(apobjarray)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetStatus<'a, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param3: ::windows::core::IntoParam<'a, IWbemClassObject>>(&self, lflags: i32, hresult: ::windows::core::HRESULT, strparam: Param2, pobjparam: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(lflags), ::core::mem::transmute(hresult), strparam.into_param().abi(), pobjparam.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WriteMessage<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, uchannel: u32, strmessage: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(uchannel), strmessage.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn WriteError<'a, Param0: ::windows::core::IntoParam<'a, IWbemClassObject>>(&self, pobjerror: Param0) -> ::windows::core::Result<u8> {
        let mut result__: u8 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), pobjerror.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<u8>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PromptUser<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, strmessage: Param0, uprompttype: u8) -> ::windows::core::Result<u8> {
        let mut result__: u8 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), strmessage.into_param().abi(), ::core::mem::transmute(uprompttype), ::core::mem::transmute(&mut result__)).from_abi::<u8>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WriteProgress<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, stractivity: Param0, strcurrentoperation: Param1, strstatusdescription: Param2, upercentcomplete: u32, usecondsremaining: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), stractivity.into_param().abi(), strcurrentoperation.into_param().abi(), strstatusdescription.into_param().abi(), ::core::mem::transmute(upercentcomplete), ::core::mem::transmute(usecondsremaining)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn WriteStreamParameter<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, strname: Param0, vtvalue: *const super::Com::VARIANT, ultype: u32, ulflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), strname.into_param().abi(), ::core::mem::transmute(vtvalue), ::core::mem::transmute(ultype), ::core::mem::transmute(ulflags)).ok()
    }
}
impl ::core::convert::From<IWbemObjectSinkEx> for IWbemObjectSink {
    fn from(value: IWbemObjectSinkEx) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWbemObjectSinkEx> for IWbemObjectSink {
    fn from(value: &IWbemObjectSinkEx) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWbemObjectSink> for IWbemObjectSinkEx {
    fn into_param(self) -> ::windows::core::Param<'a, IWbemObjectSink> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWbemObjectSink> for &IWbemObjectSinkEx {
    fn into_param(self) -> ::windows::core::Param<'a, IWbemObjectSink> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWbemObjectSinkEx> for ::windows::core::IUnknown {
    fn from(value: IWbemObjectSinkEx) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWbemObjectSinkEx> for ::windows::core::IUnknown {
    fn from(value: &IWbemObjectSinkEx) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWbemObjectSinkEx {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWbemObjectSinkEx {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWbemObjectSinkEx {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWbemObjectSinkEx {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemObjectSinkEx {}
unsafe impl ::windows::core::Interface for IWbemObjectSinkEx {
    type Vtable = IWbemObjectSinkExVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe7d35cfa_348b_485e_b524_252725d697ca);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemObjectSinkExVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lobjectcount: i32, apobjarray: *const ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, hresult: ::windows::core::HRESULT, strparam: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pobjparam: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uchannel: u32, strmessage: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pobjerror: ::windows::core::RawPtr, pureturned: *mut u8) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strmessage: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, uprompttype: u8, pureturned: *mut u8) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stractivity: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strcurrentoperation: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strstatusdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, upercentcomplete: u32, usecondsremaining: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, vtvalue: *const super::Com::VARIANT, ultype: u32, ulflags: u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
);
#[doc = "*Required features: 'Win32_System_Wmi'*"]
#[repr(transparent)]
pub struct IWbemObjectTextSrc(::windows::core::IUnknown);
impl IWbemObjectTextSrc {
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetText<'a, Param1: ::windows::core::IntoParam<'a, IWbemClassObject>, Param3: ::windows::core::IntoParam<'a, IWbemContext>>(&self, lflags: i32, pobj: Param1, uobjtextformat: u32, pctx: Param3) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(lflags), pobj.into_param().abi(), ::core::mem::transmute(uobjtextformat), pctx.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateFromText<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param3: ::windows::core::IntoParam<'a, IWbemContext>>(&self, lflags: i32, strtext: Param1, uobjtextformat: u32, pctx: Param3) -> ::windows::core::Result<IWbemClassObject> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(lflags), strtext.into_param().abi(), ::core::mem::transmute(uobjtextformat), pctx.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IWbemClassObject>(result__)
    }
}
impl ::core::convert::From<IWbemObjectTextSrc> for ::windows::core::IUnknown {
    fn from(value: IWbemObjectTextSrc) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWbemObjectTextSrc> for ::windows::core::IUnknown {
    fn from(value: &IWbemObjectTextSrc) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWbemObjectTextSrc {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWbemObjectTextSrc {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWbemObjectTextSrc {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWbemObjectTextSrc {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemObjectTextSrc {}
unsafe impl ::windows::core::Interface for IWbemObjectTextSrc {
    type Vtable = IWbemObjectTextSrcVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbfbf883a_cad7_11d3_a11b_00105a1f515a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemObjectTextSrcVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, pobj: ::windows::core::RawPtr, uobjtextformat: u32, pctx: ::windows::core::RawPtr, strtext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, strtext: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, uobjtextformat: u32, pctx: ::windows::core::RawPtr, pnewobj: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_System_Wmi'*"]
#[repr(transparent)]
pub struct IWbemPath(::windows::core::IUnknown);
impl IWbemPath {
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetText<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, umode: u32, pszpath: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(umode), pszpath.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetText(&self, lflags: i32, pubufflength: *mut u32, psztext: super::super::Foundation::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(lflags), ::core::mem::transmute(pubufflength), ::core::mem::transmute(psztext)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn GetInfo(&self, urequestedinfo: u32) -> ::windows::core::Result<u64> {
        let mut result__: u64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(urequestedinfo), ::core::mem::transmute(&mut result__)).from_abi::<u64>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetServer<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, name: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetServer(&self, punamebuflength: *mut u32, pname: super::super::Foundation::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(punamebuflength), ::core::mem::transmute(pname)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn GetNamespaceCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetNamespaceAt<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, uindex: u32, pszname: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(uindex), pszname.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetNamespaceAt(&self, uindex: u32, punamebuflength: *mut u32, pname: super::super::Foundation::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(uindex), ::core::mem::transmute(punamebuflength), ::core::mem::transmute(pname)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn RemoveNamespaceAt(&self, uindex: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(uindex)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn RemoveAllNamespaces(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn GetScopeCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetScope<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, uindex: u32, pszclass: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(uindex), pszclass.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetScopeFromText<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, uindex: u32, psztext: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(uindex), psztext.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetScope(&self, uindex: u32, puclassnamebufsize: *mut u32, pszclass: super::super::Foundation::PWSTR, pkeylist: *mut ::core::option::Option<IWbemPathKeyList>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(uindex), ::core::mem::transmute(puclassnamebufsize), ::core::mem::transmute(pszclass), ::core::mem::transmute(pkeylist)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetScopeAsText(&self, uindex: u32, putextbufsize: *mut u32, psztext: super::super::Foundation::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(uindex), ::core::mem::transmute(putextbufsize), ::core::mem::transmute(psztext)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn RemoveScope(&self, uindex: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(uindex)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn RemoveAllScopes(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetClassName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, name: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetClassName(&self, pubufflength: *mut u32, pszname: super::super::Foundation::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(pubufflength), ::core::mem::transmute(pszname)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn GetKeyList(&self) -> ::windows::core::Result<IWbemPathKeyList> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IWbemPathKeyList>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateClassPart<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, lflags: i32, name: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(lflags), name.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn DeleteClassPart(&self, lflags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), ::core::mem::transmute(lflags)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsRelative<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, wszmachine: Param0, wsznamespace: Param1) -> super::super::Foundation::BOOL {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), wszmachine.into_param().abi(), wsznamespace.into_param().abi()))
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsRelativeOrChild<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, wszmachine: Param0, wsznamespace: Param1, lflags: i32) -> super::super::Foundation::BOOL {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), wszmachine.into_param().abi(), wsznamespace.into_param().abi(), ::core::mem::transmute(lflags)))
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsLocal<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, wszmachine: Param0) -> super::super::Foundation::BOOL {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self), wszmachine.into_param().abi()))
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsSameClassName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, wszclass: Param0) -> super::super::Foundation::BOOL {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).28)(::core::mem::transmute_copy(self), wszclass.into_param().abi()))
    }
}
impl ::core::convert::From<IWbemPath> for ::windows::core::IUnknown {
    fn from(value: IWbemPath) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWbemPath> for ::windows::core::IUnknown {
    fn from(value: &IWbemPath) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWbemPath {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWbemPath {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWbemPath {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWbemPath {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemPath {}
unsafe impl ::windows::core::Interface for IWbemPath {
    type Vtable = IWbemPathVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3bc15af2_736c_477e_9e51_238af8667dcc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemPathVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, umode: u32, pszpath: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, pubufflength: *mut u32, psztext: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, urequestedinfo: u32, puresponse: *mut u64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punamebuflength: *mut u32, pname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pucount: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uindex: u32, pszname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uindex: u32, punamebuflength: *mut u32, pname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uindex: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pucount: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uindex: u32, pszclass: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uindex: u32, psztext: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uindex: u32, puclassnamebufsize: *mut u32, pszclass: super::super::Foundation::PWSTR, pkeylist: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uindex: u32, putextbufsize: *mut u32, psztext: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uindex: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pubufflength: *mut u32, pszname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pout: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, name: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszmachine: super::super::Foundation::PWSTR, wsznamespace: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszmachine: super::super::Foundation::PWSTR, wsznamespace: super::super::Foundation::PWSTR, lflags: i32) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszmachine: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszclass: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_System_Wmi'*"]
#[repr(transparent)]
pub struct IWbemPathKeyList(::windows::core::IUnknown);
impl IWbemPathKeyList {
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetKey<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, wszname: Param0, uflags: u32, ucimtype: u32, pkeyval: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), wszname.into_param().abi(), ::core::mem::transmute(uflags), ::core::mem::transmute(ucimtype), ::core::mem::transmute(pkeyval)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetKey2<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, wszname: Param0, uflags: u32, ucimtype: u32, pkeyval: *const super::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), wszname.into_param().abi(), ::core::mem::transmute(uflags), ::core::mem::transmute(ucimtype), ::core::mem::transmute(pkeyval)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetKey(&self, ukeyix: u32, uflags: u32, punamebufsize: *mut u32, pszkeyname: super::super::Foundation::PWSTR, pukeyvalbufsize: *mut u32, pkeyval: *mut ::core::ffi::c_void, puapparentcimtype: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(ukeyix), ::core::mem::transmute(uflags), ::core::mem::transmute(punamebufsize), ::core::mem::transmute(pszkeyname), ::core::mem::transmute(pukeyvalbufsize), ::core::mem::transmute(pkeyval), ::core::mem::transmute(puapparentcimtype)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetKey2(&self, ukeyix: u32, uflags: u32, punamebufsize: *mut u32, pszkeyname: super::super::Foundation::PWSTR, pkeyvalue: *mut super::Com::VARIANT, puapparentcimtype: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(ukeyix), ::core::mem::transmute(uflags), ::core::mem::transmute(punamebufsize), ::core::mem::transmute(pszkeyname), ::core::mem::transmute(pkeyvalue), ::core::mem::transmute(puapparentcimtype)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RemoveKey<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, wszname: Param0, uflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), wszname.into_param().abi(), ::core::mem::transmute(uflags)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn RemoveAllKeys(&self, uflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(uflags)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn MakeSingleton(&self, bset: u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(bset)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn GetInfo(&self, urequestedinfo: u32) -> ::windows::core::Result<u64> {
        let mut result__: u64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(urequestedinfo), ::core::mem::transmute(&mut result__)).from_abi::<u64>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetText(&self, lflags: i32, pubufflength: *mut u32, psztext: super::super::Foundation::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(lflags), ::core::mem::transmute(pubufflength), ::core::mem::transmute(psztext)).ok()
    }
}
impl ::core::convert::From<IWbemPathKeyList> for ::windows::core::IUnknown {
    fn from(value: IWbemPathKeyList) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWbemPathKeyList> for ::windows::core::IUnknown {
    fn from(value: &IWbemPathKeyList) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWbemPathKeyList {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWbemPathKeyList {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWbemPathKeyList {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWbemPathKeyList {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemPathKeyList {}
unsafe impl ::windows::core::Interface for IWbemPathKeyList {
    type Vtable = IWbemPathKeyListVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9ae62877_7544_4bb0_aa26_a13824659ed6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemPathKeyListVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pukeycount: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszname: super::super::Foundation::PWSTR, uflags: u32, ucimtype: u32, pkeyval: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszname: super::super::Foundation::PWSTR, uflags: u32, ucimtype: u32, pkeyval: *const super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ukeyix: u32, uflags: u32, punamebufsize: *mut u32, pszkeyname: super::super::Foundation::PWSTR, pukeyvalbufsize: *mut u32, pkeyval: *mut ::core::ffi::c_void, puapparentcimtype: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ukeyix: u32, uflags: u32, punamebufsize: *mut u32, pszkeyname: super::super::Foundation::PWSTR, pkeyvalue: *mut super::Com::VARIANT, puapparentcimtype: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszname: super::super::Foundation::PWSTR, uflags: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uflags: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bset: u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, urequestedinfo: u32, puresponse: *mut u64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, pubufflength: *mut u32, psztext: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_System_Wmi'*"]
#[repr(transparent)]
pub struct IWbemPropertyProvider(::windows::core::IUnknown);
impl IWbemPropertyProvider {
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetProperty<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, lflags: i32, strlocale: Param1, strclassmapping: Param2, strinstmapping: Param3, strpropmapping: Param4) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(lflags), strlocale.into_param().abi(), strclassmapping.into_param().abi(), strinstmapping.into_param().abi(), strpropmapping.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PutProperty<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, lflags: i32, strlocale: Param1, strclassmapping: Param2, strinstmapping: Param3, strpropmapping: Param4, pvvalue: *const super::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(lflags), strlocale.into_param().abi(), strclassmapping.into_param().abi(), strinstmapping.into_param().abi(), strpropmapping.into_param().abi(), ::core::mem::transmute(pvvalue)).ok()
    }
}
impl ::core::convert::From<IWbemPropertyProvider> for ::windows::core::IUnknown {
    fn from(value: IWbemPropertyProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWbemPropertyProvider> for ::windows::core::IUnknown {
    fn from(value: &IWbemPropertyProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWbemPropertyProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWbemPropertyProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWbemPropertyProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWbemPropertyProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemPropertyProvider {}
unsafe impl ::windows::core::Interface for IWbemPropertyProvider {
    type Vtable = IWbemPropertyProviderVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xce61e841_65bc_11d0_b6bd_00aa003240c7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemPropertyProviderVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, strlocale: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strclassmapping: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strinstmapping: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strpropmapping: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvvalue: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, strlocale: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strclassmapping: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strinstmapping: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strpropmapping: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvvalue: *const super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
);
#[doc = "*Required features: 'Win32_System_Wmi'*"]
#[repr(transparent)]
pub struct IWbemProviderIdentity(::windows::core::IUnknown);
impl IWbemProviderIdentity {
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn SetRegistrationObject<'a, Param1: ::windows::core::IntoParam<'a, IWbemClassObject>>(&self, lflags: i32, pprovreg: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(lflags), pprovreg.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IWbemProviderIdentity> for ::windows::core::IUnknown {
    fn from(value: IWbemProviderIdentity) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWbemProviderIdentity> for ::windows::core::IUnknown {
    fn from(value: &IWbemProviderIdentity) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWbemProviderIdentity {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWbemProviderIdentity {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWbemProviderIdentity {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWbemProviderIdentity {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemProviderIdentity {}
unsafe impl ::windows::core::Interface for IWbemProviderIdentity {
    type Vtable = IWbemProviderIdentityVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x631f7d97_d993_11d2_b339_00105a1f4aaf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemProviderIdentityVtbl(pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, pprovreg: ::windows::core::RawPtr) -> ::windows::core::HRESULT);
#[doc = "*Required features: 'Win32_System_Wmi'*"]
#[repr(transparent)]
pub struct IWbemProviderInit(::windows::core::IUnknown);
impl IWbemProviderInit {
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Initialize<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param4: ::windows::core::IntoParam<'a, IWbemServices>, Param5: ::windows::core::IntoParam<'a, IWbemContext>, Param6: ::windows::core::IntoParam<'a, IWbemProviderInitSink>>(&self, wszuser: Param0, lflags: i32, wsznamespace: Param2, wszlocale: Param3, pnamespace: Param4, pctx: Param5, pinitsink: Param6) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), wszuser.into_param().abi(), ::core::mem::transmute(lflags), wsznamespace.into_param().abi(), wszlocale.into_param().abi(), pnamespace.into_param().abi(), pctx.into_param().abi(), pinitsink.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IWbemProviderInit> for ::windows::core::IUnknown {
    fn from(value: IWbemProviderInit) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWbemProviderInit> for ::windows::core::IUnknown {
    fn from(value: &IWbemProviderInit) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWbemProviderInit {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWbemProviderInit {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWbemProviderInit {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWbemProviderInit {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemProviderInit {}
unsafe impl ::windows::core::Interface for IWbemProviderInit {
    type Vtable = IWbemProviderInitVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1be41572_91dd_11d1_aeb2_00c04fb68820);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemProviderInitVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszuser: super::super::Foundation::PWSTR, lflags: i32, wsznamespace: super::super::Foundation::PWSTR, wszlocale: super::super::Foundation::PWSTR, pnamespace: ::windows::core::RawPtr, pctx: ::windows::core::RawPtr, pinitsink: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_System_Wmi'*"]
#[repr(transparent)]
pub struct IWbemProviderInitSink(::windows::core::IUnknown);
impl IWbemProviderInitSink {
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn SetStatus(&self, lstatus: i32, lflags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(lstatus), ::core::mem::transmute(lflags)).ok()
    }
}
impl ::core::convert::From<IWbemProviderInitSink> for ::windows::core::IUnknown {
    fn from(value: IWbemProviderInitSink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWbemProviderInitSink> for ::windows::core::IUnknown {
    fn from(value: &IWbemProviderInitSink) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWbemProviderInitSink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWbemProviderInitSink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWbemProviderInitSink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWbemProviderInitSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemProviderInitSink {}
unsafe impl ::windows::core::Interface for IWbemProviderInitSink {
    type Vtable = IWbemProviderInitSinkVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1be41571_91dd_11d1_aeb2_00c04fb68820);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemProviderInitSinkVtbl(pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lstatus: i32, lflags: i32) -> ::windows::core::HRESULT);
#[doc = "*Required features: 'Win32_System_Wmi'*"]
#[repr(transparent)]
pub struct IWbemQualifierSet(::windows::core::IUnknown);
impl IWbemQualifierSet {
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Get<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, wszname: Param0, lflags: i32, pval: *mut super::Com::VARIANT, plflavor: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), wszname.into_param().abi(), ::core::mem::transmute(lflags), ::core::mem::transmute(pval), ::core::mem::transmute(plflavor)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Put<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, wszname: Param0, pval: *const super::Com::VARIANT, lflavor: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), wszname.into_param().abi(), ::core::mem::transmute(pval), ::core::mem::transmute(lflavor)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Delete<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, wszname: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), wszname.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetNames(&self, lflags: i32) -> ::windows::core::Result<*mut super::Com::SAFEARRAY> {
        let mut result__: *mut super::Com::SAFEARRAY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(lflags), ::core::mem::transmute(&mut result__)).from_abi::<*mut super::Com::SAFEARRAY>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn BeginEnumeration(&self, lflags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(lflags)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Next(&self, lflags: i32, pstrname: *mut super::super::Foundation::BSTR, pval: *mut super::Com::VARIANT, plflavor: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(lflags), ::core::mem::transmute(pstrname), ::core::mem::transmute(pval), ::core::mem::transmute(plflavor)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn EndEnumeration(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<IWbemQualifierSet> for ::windows::core::IUnknown {
    fn from(value: IWbemQualifierSet) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWbemQualifierSet> for ::windows::core::IUnknown {
    fn from(value: &IWbemQualifierSet) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWbemQualifierSet {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWbemQualifierSet {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWbemQualifierSet {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWbemQualifierSet {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemQualifierSet {}
unsafe impl ::windows::core::Interface for IWbemQualifierSet {
    type Vtable = IWbemQualifierSetVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdc12a680_737f_11cf_884d_00aa004b2e24);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemQualifierSetVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszname: super::super::Foundation::PWSTR, lflags: i32, pval: *mut super::Com::VARIANT, plflavor: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszname: super::super::Foundation::PWSTR, pval: *const super::Com::VARIANT, lflavor: i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, pnames: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, pstrname: *mut super::super::Foundation::BSTR, pval: *mut super::Com::VARIANT, plflavor: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_System_Wmi'*"]
#[repr(transparent)]
pub struct IWbemQuery(::windows::core::IUnknown);
impl IWbemQuery {
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn Empty(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn SetLanguageFeatures(&self, uflags: u32, uarraysize: u32, pufeatures: *const u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(uflags), ::core::mem::transmute(uarraysize), ::core::mem::transmute(pufeatures)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn TestLanguageFeatures(&self, uflags: u32, uarraysize: *mut u32, pufeatures: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(uflags), ::core::mem::transmute(uarraysize), ::core::mem::transmute(pufeatures)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Parse<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszlang: Param0, pszquery: Param1, uflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), pszlang.into_param().abi(), pszquery.into_param().abi(), ::core::mem::transmute(uflags)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn GetAnalysis(&self, uanalysistype: u32, uflags: u32, panalysis: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(uanalysistype), ::core::mem::transmute(uflags), ::core::mem::transmute(panalysis)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn FreeMemory(&self, pmem: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmem)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn GetQueryInfo(&self, uanalysistype: u32, uinfoid: u32, ubufsize: u32, pdestbuf: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(uanalysistype), ::core::mem::transmute(uinfoid), ::core::mem::transmute(ubufsize), ::core::mem::transmute(pdestbuf)).ok()
    }
}
impl ::core::convert::From<IWbemQuery> for ::windows::core::IUnknown {
    fn from(value: IWbemQuery) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWbemQuery> for ::windows::core::IUnknown {
    fn from(value: &IWbemQuery) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWbemQuery {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWbemQuery {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWbemQuery {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWbemQuery {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemQuery {}
unsafe impl ::windows::core::Interface for IWbemQuery {
    type Vtable = IWbemQueryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x81166f58_dd98_11d3_a120_00105a1f515a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemQueryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uflags: u32, uarraysize: u32, pufeatures: *const u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uflags: u32, uarraysize: *mut u32, pufeatures: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszlang: super::super::Foundation::PWSTR, pszquery: super::super::Foundation::PWSTR, uflags: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uanalysistype: u32, uflags: u32, panalysis: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmem: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uanalysistype: u32, uinfoid: u32, ubufsize: u32, pdestbuf: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_System_Wmi'*"]
#[repr(transparent)]
pub struct IWbemRefresher(::windows::core::IUnknown);
impl IWbemRefresher {
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn Refresh(&self, lflags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(lflags)).ok()
    }
}
impl ::core::convert::From<IWbemRefresher> for ::windows::core::IUnknown {
    fn from(value: IWbemRefresher) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWbemRefresher> for ::windows::core::IUnknown {
    fn from(value: &IWbemRefresher) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWbemRefresher {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWbemRefresher {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWbemRefresher {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWbemRefresher {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemRefresher {}
unsafe impl ::windows::core::Interface for IWbemRefresher {
    type Vtable = IWbemRefresherVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x49353c99_516b_11d1_aea6_00c04fb68820);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemRefresherVtbl(pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32) -> ::windows::core::HRESULT);
#[doc = "*Required features: 'Win32_System_Wmi'*"]
#[repr(transparent)]
pub struct IWbemServices(::windows::core::IUnknown);
impl IWbemServices {
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OpenNamespace<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, IWbemContext>>(&self, strnamespace: Param0, lflags: i32, pctx: Param2, ppworkingnamespace: *mut ::core::option::Option<IWbemServices>, ppresult: *mut ::core::option::Option<IWbemCallResult>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), strnamespace.into_param().abi(), ::core::mem::transmute(lflags), pctx.into_param().abi(), ::core::mem::transmute(ppworkingnamespace), ::core::mem::transmute(ppresult)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn CancelAsyncCall<'a, Param0: ::windows::core::IntoParam<'a, IWbemObjectSink>>(&self, psink: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), psink.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn QueryObjectSink(&self, lflags: i32) -> ::windows::core::Result<IWbemObjectSink> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(lflags), ::core::mem::transmute(&mut result__)).from_abi::<IWbemObjectSink>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetObject<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, IWbemContext>>(&self, strobjectpath: Param0, lflags: i32, pctx: Param2, ppobject: *mut ::core::option::Option<IWbemClassObject>, ppcallresult: *mut ::core::option::Option<IWbemCallResult>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), strobjectpath.into_param().abi(), ::core::mem::transmute(lflags), pctx.into_param().abi(), ::core::mem::transmute(ppobject), ::core::mem::transmute(ppcallresult)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetObjectAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, IWbemContext>, Param3: ::windows::core::IntoParam<'a, IWbemObjectSink>>(&self, strobjectpath: Param0, lflags: i32, pctx: Param2, presponsehandler: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), strobjectpath.into_param().abi(), ::core::mem::transmute(lflags), pctx.into_param().abi(), presponsehandler.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn PutClass<'a, Param0: ::windows::core::IntoParam<'a, IWbemClassObject>, Param2: ::windows::core::IntoParam<'a, IWbemContext>>(&self, pobject: Param0, lflags: i32, pctx: Param2, ppcallresult: *mut ::core::option::Option<IWbemCallResult>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), pobject.into_param().abi(), ::core::mem::transmute(lflags), pctx.into_param().abi(), ::core::mem::transmute(ppcallresult)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn PutClassAsync<'a, Param0: ::windows::core::IntoParam<'a, IWbemClassObject>, Param2: ::windows::core::IntoParam<'a, IWbemContext>, Param3: ::windows::core::IntoParam<'a, IWbemObjectSink>>(&self, pobject: Param0, lflags: i32, pctx: Param2, presponsehandler: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), pobject.into_param().abi(), ::core::mem::transmute(lflags), pctx.into_param().abi(), presponsehandler.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DeleteClass<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, IWbemContext>>(&self, strclass: Param0, lflags: i32, pctx: Param2, ppcallresult: *mut ::core::option::Option<IWbemCallResult>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), strclass.into_param().abi(), ::core::mem::transmute(lflags), pctx.into_param().abi(), ::core::mem::transmute(ppcallresult)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DeleteClassAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, IWbemContext>, Param3: ::windows::core::IntoParam<'a, IWbemObjectSink>>(&self, strclass: Param0, lflags: i32, pctx: Param2, presponsehandler: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), strclass.into_param().abi(), ::core::mem::transmute(lflags), pctx.into_param().abi(), presponsehandler.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateClassEnum<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, IWbemContext>>(&self, strsuperclass: Param0, lflags: i32, pctx: Param2) -> ::windows::core::Result<IEnumWbemClassObject> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), strsuperclass.into_param().abi(), ::core::mem::transmute(lflags), pctx.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IEnumWbemClassObject>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateClassEnumAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, IWbemContext>, Param3: ::windows::core::IntoParam<'a, IWbemObjectSink>>(&self, strsuperclass: Param0, lflags: i32, pctx: Param2, presponsehandler: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), strsuperclass.into_param().abi(), ::core::mem::transmute(lflags), pctx.into_param().abi(), presponsehandler.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn PutInstance<'a, Param0: ::windows::core::IntoParam<'a, IWbemClassObject>, Param2: ::windows::core::IntoParam<'a, IWbemContext>>(&self, pinst: Param0, lflags: i32, pctx: Param2, ppcallresult: *mut ::core::option::Option<IWbemCallResult>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), pinst.into_param().abi(), ::core::mem::transmute(lflags), pctx.into_param().abi(), ::core::mem::transmute(ppcallresult)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn PutInstanceAsync<'a, Param0: ::windows::core::IntoParam<'a, IWbemClassObject>, Param2: ::windows::core::IntoParam<'a, IWbemContext>, Param3: ::windows::core::IntoParam<'a, IWbemObjectSink>>(&self, pinst: Param0, lflags: i32, pctx: Param2, presponsehandler: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), pinst.into_param().abi(), ::core::mem::transmute(lflags), pctx.into_param().abi(), presponsehandler.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DeleteInstance<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, IWbemContext>>(&self, strobjectpath: Param0, lflags: i32, pctx: Param2, ppcallresult: *mut ::core::option::Option<IWbemCallResult>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), strobjectpath.into_param().abi(), ::core::mem::transmute(lflags), pctx.into_param().abi(), ::core::mem::transmute(ppcallresult)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DeleteInstanceAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, IWbemContext>, Param3: ::windows::core::IntoParam<'a, IWbemObjectSink>>(&self, strobjectpath: Param0, lflags: i32, pctx: Param2, presponsehandler: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), strobjectpath.into_param().abi(), ::core::mem::transmute(lflags), pctx.into_param().abi(), presponsehandler.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateInstanceEnum<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, IWbemContext>>(&self, strfilter: Param0, lflags: i32, pctx: Param2) -> ::windows::core::Result<IEnumWbemClassObject> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), strfilter.into_param().abi(), ::core::mem::transmute(lflags), pctx.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IEnumWbemClassObject>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateInstanceEnumAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, IWbemContext>, Param3: ::windows::core::IntoParam<'a, IWbemObjectSink>>(&self, strfilter: Param0, lflags: i32, pctx: Param2, presponsehandler: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), strfilter.into_param().abi(), ::core::mem::transmute(lflags), pctx.into_param().abi(), presponsehandler.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ExecQuery<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param3: ::windows::core::IntoParam<'a, IWbemContext>>(&self, strquerylanguage: Param0, strquery: Param1, lflags: i32, pctx: Param3) -> ::windows::core::Result<IEnumWbemClassObject> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), strquerylanguage.into_param().abi(), strquery.into_param().abi(), ::core::mem::transmute(lflags), pctx.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IEnumWbemClassObject>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ExecQueryAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param3: ::windows::core::IntoParam<'a, IWbemContext>, Param4: ::windows::core::IntoParam<'a, IWbemObjectSink>>(&self, strquerylanguage: Param0, strquery: Param1, lflags: i32, pctx: Param3, presponsehandler: Param4) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), strquerylanguage.into_param().abi(), strquery.into_param().abi(), ::core::mem::transmute(lflags), pctx.into_param().abi(), presponsehandler.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ExecNotificationQuery<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param3: ::windows::core::IntoParam<'a, IWbemContext>>(&self, strquerylanguage: Param0, strquery: Param1, lflags: i32, pctx: Param3) -> ::windows::core::Result<IEnumWbemClassObject> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), strquerylanguage.into_param().abi(), strquery.into_param().abi(), ::core::mem::transmute(lflags), pctx.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IEnumWbemClassObject>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ExecNotificationQueryAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param3: ::windows::core::IntoParam<'a, IWbemContext>, Param4: ::windows::core::IntoParam<'a, IWbemObjectSink>>(&self, strquerylanguage: Param0, strquery: Param1, lflags: i32, pctx: Param3, presponsehandler: Param4) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), strquerylanguage.into_param().abi(), strquery.into_param().abi(), ::core::mem::transmute(lflags), pctx.into_param().abi(), presponsehandler.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ExecMethod<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param3: ::windows::core::IntoParam<'a, IWbemContext>, Param4: ::windows::core::IntoParam<'a, IWbemClassObject>>(&self, strobjectpath: Param0, strmethodname: Param1, lflags: i32, pctx: Param3, pinparams: Param4, ppoutparams: *mut ::core::option::Option<IWbemClassObject>, ppcallresult: *mut ::core::option::Option<IWbemCallResult>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), strobjectpath.into_param().abi(), strmethodname.into_param().abi(), ::core::mem::transmute(lflags), pctx.into_param().abi(), pinparams.into_param().abi(), ::core::mem::transmute(ppoutparams), ::core::mem::transmute(ppcallresult)).ok()
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ExecMethodAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param3: ::windows::core::IntoParam<'a, IWbemContext>, Param4: ::windows::core::IntoParam<'a, IWbemClassObject>, Param5: ::windows::core::IntoParam<'a, IWbemObjectSink>>(&self, strobjectpath: Param0, strmethodname: Param1, lflags: i32, pctx: Param3, pinparams: Param4, presponsehandler: Param5) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), strobjectpath.into_param().abi(), strmethodname.into_param().abi(), ::core::mem::transmute(lflags), pctx.into_param().abi(), pinparams.into_param().abi(), presponsehandler.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IWbemServices> for ::windows::core::IUnknown {
    fn from(value: IWbemServices) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWbemServices> for ::windows::core::IUnknown {
    fn from(value: &IWbemServices) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWbemServices {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWbemServices {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWbemServices {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWbemServices {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemServices {}
unsafe impl ::windows::core::Interface for IWbemServices {
    type Vtable = IWbemServicesVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9556dc99_828c_11cf_a37e_00aa003240c7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemServicesVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strnamespace: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lflags: i32, pctx: ::windows::core::RawPtr, ppworkingnamespace: *mut ::windows::core::RawPtr, ppresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psink: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, ppresponsehandler: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strobjectpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lflags: i32, pctx: ::windows::core::RawPtr, ppobject: *mut ::windows::core::RawPtr, ppcallresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strobjectpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lflags: i32, pctx: ::windows::core::RawPtr, presponsehandler: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pobject: ::windows::core::RawPtr, lflags: i32, pctx: ::windows::core::RawPtr, ppcallresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pobject: ::windows::core::RawPtr, lflags: i32, pctx: ::windows::core::RawPtr, presponsehandler: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lflags: i32, pctx: ::windows::core::RawPtr, ppcallresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lflags: i32, pctx: ::windows::core::RawPtr, presponsehandler: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strsuperclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lflags: i32, pctx: ::windows::core::RawPtr, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strsuperclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lflags: i32, pctx: ::windows::core::RawPtr, presponsehandler: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinst: ::windows::core::RawPtr, lflags: i32, pctx: ::windows::core::RawPtr, ppcallresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinst: ::windows::core::RawPtr, lflags: i32, pctx: ::windows::core::RawPtr, presponsehandler: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strobjectpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lflags: i32, pctx: ::windows::core::RawPtr, ppcallresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strobjectpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lflags: i32, pctx: ::windows::core::RawPtr, presponsehandler: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strfilter: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lflags: i32, pctx: ::windows::core::RawPtr, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strfilter: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lflags: i32, pctx: ::windows::core::RawPtr, presponsehandler: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strquerylanguage: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strquery: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lflags: i32, pctx: ::windows::core::RawPtr, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strquerylanguage: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strquery: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lflags: i32, pctx: ::windows::core::RawPtr, presponsehandler: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strquerylanguage: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strquery: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lflags: i32, pctx: ::windows::core::RawPtr, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strquerylanguage: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strquery: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lflags: i32, pctx: ::windows::core::RawPtr, presponsehandler: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strobjectpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strmethodname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lflags: i32, pctx: ::windows::core::RawPtr, pinparams: ::windows::core::RawPtr, ppoutparams: *mut ::windows::core::RawPtr, ppcallresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strobjectpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strmethodname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lflags: i32, pctx: ::windows::core::RawPtr, pinparams: ::windows::core::RawPtr, presponsehandler: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_System_Wmi'*"]
#[repr(transparent)]
pub struct IWbemShutdown(::windows::core::IUnknown);
impl IWbemShutdown {
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn Shutdown<'a, Param2: ::windows::core::IntoParam<'a, IWbemContext>>(&self, ureason: i32, umaxmilliseconds: u32, pctx: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(ureason), ::core::mem::transmute(umaxmilliseconds), pctx.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IWbemShutdown> for ::windows::core::IUnknown {
    fn from(value: IWbemShutdown) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWbemShutdown> for ::windows::core::IUnknown {
    fn from(value: &IWbemShutdown) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWbemShutdown {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWbemShutdown {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWbemShutdown {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWbemShutdown {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemShutdown {}
unsafe impl ::windows::core::Interface for IWbemShutdown {
    type Vtable = IWbemShutdownVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb7b31df9_d515_11d3_a11c_00105a1f515a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemShutdownVtbl(pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ureason: i32, umaxmilliseconds: u32, pctx: ::windows::core::RawPtr) -> ::windows::core::HRESULT);
#[doc = "*Required features: 'Win32_System_Wmi'*"]
#[repr(transparent)]
pub struct IWbemStatusCodeText(::windows::core::IUnknown);
impl IWbemStatusCodeText {
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetErrorCodeText(&self, hres: ::windows::core::HRESULT, localeid: u32, lflags: i32) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(hres), ::core::mem::transmute(localeid), ::core::mem::transmute(lflags), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFacilityCodeText(&self, hres: ::windows::core::HRESULT, localeid: u32, lflags: i32) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(hres), ::core::mem::transmute(localeid), ::core::mem::transmute(lflags), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
}
impl ::core::convert::From<IWbemStatusCodeText> for ::windows::core::IUnknown {
    fn from(value: IWbemStatusCodeText) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWbemStatusCodeText> for ::windows::core::IUnknown {
    fn from(value: &IWbemStatusCodeText) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWbemStatusCodeText {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWbemStatusCodeText {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWbemStatusCodeText {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWbemStatusCodeText {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemStatusCodeText {}
unsafe impl ::windows::core::Interface for IWbemStatusCodeText {
    type Vtable = IWbemStatusCodeTextVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeb87e1bc_3233_11d2_aec9_00c04fb68820);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemStatusCodeTextVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hres: ::windows::core::HRESULT, localeid: u32, lflags: i32, messagetext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hres: ::windows::core::HRESULT, localeid: u32, lflags: i32, messagetext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_System_Wmi'*"]
#[repr(transparent)]
pub struct IWbemTransport(::windows::core::IUnknown);
impl IWbemTransport {
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn Initialize(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<IWbemTransport> for ::windows::core::IUnknown {
    fn from(value: IWbemTransport) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWbemTransport> for ::windows::core::IUnknown {
    fn from(value: &IWbemTransport) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWbemTransport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWbemTransport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWbemTransport {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWbemTransport {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemTransport {}
unsafe impl ::windows::core::Interface for IWbemTransport {
    type Vtable = IWbemTransportVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x553fe584_2156_11d0_b6ae_00aa003240c7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemTransportVtbl(pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT);
#[doc = "*Required features: 'Win32_System_Wmi'*"]
#[repr(transparent)]
pub struct IWbemUnboundObjectSink(::windows::core::IUnknown);
impl IWbemUnboundObjectSink {
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn IndicateToConsumer<'a, Param0: ::windows::core::IntoParam<'a, IWbemClassObject>>(&self, plogicalconsumer: Param0, lnumobjects: i32, apobjects: *const ::core::option::Option<IWbemClassObject>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), plogicalconsumer.into_param().abi(), ::core::mem::transmute(lnumobjects), ::core::mem::transmute(apobjects)).ok()
    }
}
impl ::core::convert::From<IWbemUnboundObjectSink> for ::windows::core::IUnknown {
    fn from(value: IWbemUnboundObjectSink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWbemUnboundObjectSink> for ::windows::core::IUnknown {
    fn from(value: &IWbemUnboundObjectSink) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWbemUnboundObjectSink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWbemUnboundObjectSink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWbemUnboundObjectSink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWbemUnboundObjectSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemUnboundObjectSink {}
unsafe impl ::windows::core::Interface for IWbemUnboundObjectSink {
    type Vtable = IWbemUnboundObjectSinkVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe246107b_b06e_11d0_ad61_00c04fd8fdff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemUnboundObjectSinkVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plogicalconsumer: ::windows::core::RawPtr, lnumobjects: i32, apobjects: *const ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_System_Wmi'*"]
#[repr(transparent)]
pub struct IWbemUnsecuredApartment(::windows::core::IUnknown);
impl IWbemUnsecuredApartment {
    #[doc = "*Required features: 'Win32_System_Wmi'*"]
    pub unsafe fn CreateObjectStub<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, pobject: Param0) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pobject.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateSinkStub<'a, Param0: ::windows::core::IntoParam<'a, IWbemObjectSink>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, psink: Param0, dwflags: u32, wszreserved: Param2) -> ::windows::core::Result<IWbemObjectSink> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), psink.into_param().abi(), ::core::mem::transmute(dwflags), wszreserved.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IWbemObjectSink>(result__)
    }
}
impl ::core::convert::From<IWbemUnsecuredApartment> for IUnsecuredApartment {
    fn from(value: IWbemUnsecuredApartment) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWbemUnsecuredApartment> for IUnsecuredApartment {
    fn from(value: &IWbemUnsecuredApartment) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IUnsecuredApartment> for IWbemUnsecuredApartment {
    fn into_param(self) -> ::windows::core::Param<'a, IUnsecuredApartment> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IUnsecuredApartment> for &IWbemUnsecuredApartment {
    fn into_param(self) -> ::windows::core::Param<'a, IUnsecuredApartment> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWbemUnsecuredApartment> for ::windows::core::IUnknown {
    fn from(value: IWbemUnsecuredApartment) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWbemUnsecuredApartment> for ::windows::core::IUnknown {
    fn from(value: &IWbemUnsecuredApartment) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWbemUnsecuredApartment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWbemUnsecuredApartment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWbemUnsecuredApartment {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWbemUnsecuredApartment {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemUnsecuredApartment {}
unsafe impl ::windows::core::Interface for IWbemUnsecuredApartment {
    type Vtable = IWbemUnsecuredApartmentVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x31739d04_3471_4cf4_9a7c_57a44ae71956);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemUnsecuredApartmentVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pobject: *mut ::core::ffi::c_void, ppstub: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psink: ::windows::core::RawPtr, dwflags: u32, wszreserved: super::super::Foundation::PWSTR, ppstub: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_Application {
    pub reserved1: u64,
    pub reserved2: isize,
    pub ft: *mut MI_ApplicationFT,
}
impl ::core::marker::Copy for MI_Application {}
impl ::core::clone::Clone for MI_Application {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_Application {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_Application {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_Application>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_Application {}
impl ::core::default::Default for MI_Application {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_ApplicationFT {
    pub Close: isize,
    pub NewSession: isize,
    pub NewHostedProvider: isize,
    pub NewInstance: isize,
    pub NewDestinationOptions: isize,
    pub NewOperationOptions: isize,
    pub NewSubscriptionDeliveryOptions: isize,
    pub NewSerializer: isize,
    pub NewDeserializer: isize,
    pub NewInstanceFromClass: isize,
    pub NewClass: isize,
}
impl ::core::marker::Copy for MI_ApplicationFT {}
impl ::core::clone::Clone for MI_ApplicationFT {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_ApplicationFT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_ApplicationFT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_ApplicationFT>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_ApplicationFT {}
impl ::core::default::Default for MI_ApplicationFT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_System_Wmi'*"]
#[inline]
pub unsafe fn MI_Application_InitializeV1(flags: u32, applicationid: *const u16, extendederror: *mut *mut MI_Instance, application: *mut MI_Application) -> MI_Result {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MI_Application_InitializeV1(flags: u32, applicationid: *const u16, extendederror: *mut *mut MI_Instance, application: *mut MI_Application) -> MI_Result;
        }
        ::core::mem::transmute(MI_Application_InitializeV1(::core::mem::transmute(flags), ::core::mem::transmute(applicationid), ::core::mem::transmute(extendederror), ::core::mem::transmute(application)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_Array {
    pub data: *mut ::core::ffi::c_void,
    pub size: u32,
}
impl ::core::marker::Copy for MI_Array {}
impl ::core::clone::Clone for MI_Array {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_Array {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_Array {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_Array>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_Array {}
impl ::core::default::Default for MI_Array {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_ArrayField {
    pub value: MI_Array,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_ArrayField {}
impl ::core::clone::Clone for MI_ArrayField {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_ArrayField {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_ArrayField {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_ArrayField>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_ArrayField {}
impl ::core::default::Default for MI_ArrayField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_BooleanA {
    pub data: *mut u8,
    pub size: u32,
}
impl ::core::marker::Copy for MI_BooleanA {}
impl ::core::clone::Clone for MI_BooleanA {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_BooleanA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_BooleanA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_BooleanA>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_BooleanA {}
impl ::core::default::Default for MI_BooleanA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_BooleanAField {
    pub value: MI_BooleanA,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_BooleanAField {}
impl ::core::clone::Clone for MI_BooleanAField {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_BooleanAField {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_BooleanAField {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_BooleanAField>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_BooleanAField {}
impl ::core::default::Default for MI_BooleanAField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_BooleanField {
    pub value: u8,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_BooleanField {}
impl ::core::clone::Clone for MI_BooleanField {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_BooleanField {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_BooleanField {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_BooleanField>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_BooleanField {}
impl ::core::default::Default for MI_BooleanField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_CALL_VERSION: u32 = 1u32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_CHAR_TYPE: u32 = 2u32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub type MI_CallbackMode = i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_CALLBACKMODE_REPORT: MI_CallbackMode = 0i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_CALLBACKMODE_INQUIRE: MI_CallbackMode = 1i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_CALLBACKMODE_IGNORE: MI_CallbackMode = 2i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub type MI_CancelCallback = ::core::option::Option<unsafe extern "system" fn(reason: MI_CancellationReason, callbackdata: *const ::core::ffi::c_void)>;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub type MI_CancellationReason = i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_REASON_NONE: MI_CancellationReason = 0i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_REASON_TIMEOUT: MI_CancellationReason = 1i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_REASON_SHUTDOWN: MI_CancellationReason = 2i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_REASON_SERVICESTOP: MI_CancellationReason = 3i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_Char16A {
    pub data: *mut u16,
    pub size: u32,
}
impl ::core::marker::Copy for MI_Char16A {}
impl ::core::clone::Clone for MI_Char16A {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_Char16A {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_Char16A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_Char16A>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_Char16A {}
impl ::core::default::Default for MI_Char16A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_Char16AField {
    pub value: MI_Char16A,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_Char16AField {}
impl ::core::clone::Clone for MI_Char16AField {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_Char16AField {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_Char16AField {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_Char16AField>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_Char16AField {}
impl ::core::default::Default for MI_Char16AField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_Char16Field {
    pub value: u16,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_Char16Field {}
impl ::core::clone::Clone for MI_Char16Field {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_Char16Field {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_Char16Field {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_Char16Field>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_Char16Field {}
impl ::core::default::Default for MI_Char16Field {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_Class {
    pub ft: *mut MI_ClassFT,
    pub classDecl: *mut MI_ClassDecl,
    pub namespaceName: *mut u16,
    pub serverName: *mut u16,
    pub reserved: [isize; 4],
}
impl ::core::marker::Copy for MI_Class {}
impl ::core::clone::Clone for MI_Class {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_Class {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_Class {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_Class>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_Class {}
impl ::core::default::Default for MI_Class {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_ClassDecl {
    pub flags: u32,
    pub code: u32,
    pub name: *mut u16,
    pub qualifiers: *mut *mut MI_Qualifier,
    pub numQualifiers: u32,
    pub properties: *mut *mut MI_PropertyDecl,
    pub numProperties: u32,
    pub size: u32,
    pub superClass: *mut u16,
    pub superClassDecl: *mut MI_ClassDecl,
    pub methods: *mut *mut MI_MethodDecl,
    pub numMethods: u32,
    pub schema: *mut MI_SchemaDecl,
    pub providerFT: *mut MI_ProviderFT,
    pub owningClass: *mut MI_Class,
}
impl ::core::marker::Copy for MI_ClassDecl {}
impl ::core::clone::Clone for MI_ClassDecl {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_ClassDecl {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_ClassDecl {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_ClassDecl>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_ClassDecl {}
impl ::core::default::Default for MI_ClassDecl {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_ClassFT {
    pub GetClassNameA: isize,
    pub GetNameSpace: isize,
    pub GetServerName: isize,
    pub GetElementCount: isize,
    pub GetElement: isize,
    pub GetElementAt: isize,
    pub GetClassQualifierSet: isize,
    pub GetMethodCount: isize,
    pub GetMethodAt: isize,
    pub GetMethod: isize,
    pub GetParentClassName: isize,
    pub GetParentClass: isize,
    pub Delete: isize,
    pub Clone: isize,
}
impl ::core::marker::Copy for MI_ClassFT {}
impl ::core::clone::Clone for MI_ClassFT {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_ClassFT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_ClassFT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_ClassFT>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_ClassFT {}
impl ::core::default::Default for MI_ClassFT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_ClientFT_V1 {
    pub applicationFT: *mut MI_ApplicationFT,
    pub sessionFT: *mut MI_SessionFT,
    pub operationFT: *mut MI_OperationFT,
    pub hostedProviderFT: *mut MI_HostedProviderFT,
    pub serializerFT: *mut MI_SerializerFT,
    pub deserializerFT: *mut MI_DeserializerFT,
    pub subscribeDeliveryOptionsFT: *mut MI_SubscriptionDeliveryOptionsFT,
    pub destinationOptionsFT: *mut MI_DestinationOptionsFT,
    pub operationOptionsFT: *mut MI_OperationOptionsFT,
    pub utilitiesFT: *mut MI_UtilitiesFT,
}
impl ::core::marker::Copy for MI_ClientFT_V1 {}
impl ::core::clone::Clone for MI_ClientFT_V1 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_ClientFT_V1 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_ClientFT_V1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_ClientFT_V1>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_ClientFT_V1 {}
impl ::core::default::Default for MI_ClientFT_V1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_ConstBooleanA {
    pub data: *mut u8,
    pub size: u32,
}
impl ::core::marker::Copy for MI_ConstBooleanA {}
impl ::core::clone::Clone for MI_ConstBooleanA {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_ConstBooleanA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_ConstBooleanA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_ConstBooleanA>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_ConstBooleanA {}
impl ::core::default::Default for MI_ConstBooleanA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_ConstBooleanAField {
    pub value: MI_ConstBooleanA,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_ConstBooleanAField {}
impl ::core::clone::Clone for MI_ConstBooleanAField {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_ConstBooleanAField {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_ConstBooleanAField {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_ConstBooleanAField>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_ConstBooleanAField {}
impl ::core::default::Default for MI_ConstBooleanAField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_ConstBooleanField {
    pub value: u8,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_ConstBooleanField {}
impl ::core::clone::Clone for MI_ConstBooleanField {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_ConstBooleanField {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_ConstBooleanField {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_ConstBooleanField>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_ConstBooleanField {}
impl ::core::default::Default for MI_ConstBooleanField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_ConstChar16A {
    pub data: *mut u16,
    pub size: u32,
}
impl ::core::marker::Copy for MI_ConstChar16A {}
impl ::core::clone::Clone for MI_ConstChar16A {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_ConstChar16A {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_ConstChar16A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_ConstChar16A>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_ConstChar16A {}
impl ::core::default::Default for MI_ConstChar16A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_ConstChar16AField {
    pub value: MI_ConstChar16A,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_ConstChar16AField {}
impl ::core::clone::Clone for MI_ConstChar16AField {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_ConstChar16AField {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_ConstChar16AField {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_ConstChar16AField>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_ConstChar16AField {}
impl ::core::default::Default for MI_ConstChar16AField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_ConstChar16Field {
    pub value: u16,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_ConstChar16Field {}
impl ::core::clone::Clone for MI_ConstChar16Field {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_ConstChar16Field {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_ConstChar16Field {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_ConstChar16Field>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_ConstChar16Field {}
impl ::core::default::Default for MI_ConstChar16Field {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_ConstDatetimeA {
    pub data: *mut MI_Datetime,
    pub size: u32,
}
impl ::core::marker::Copy for MI_ConstDatetimeA {}
impl ::core::clone::Clone for MI_ConstDatetimeA {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_ConstDatetimeA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_ConstDatetimeA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_ConstDatetimeA>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_ConstDatetimeA {}
impl ::core::default::Default for MI_ConstDatetimeA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_ConstDatetimeAField {
    pub value: MI_ConstDatetimeA,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_ConstDatetimeAField {}
impl ::core::clone::Clone for MI_ConstDatetimeAField {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_ConstDatetimeAField {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_ConstDatetimeAField {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_ConstDatetimeAField>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_ConstDatetimeAField {}
impl ::core::default::Default for MI_ConstDatetimeAField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_ConstDatetimeField {
    pub value: MI_Datetime,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_ConstDatetimeField {}
impl ::core::clone::Clone for MI_ConstDatetimeField {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_ConstDatetimeField {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_ConstDatetimeField {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_ConstDatetimeField>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_ConstDatetimeField {}
impl ::core::default::Default for MI_ConstDatetimeField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_ConstInstanceA {
    pub data: *mut *mut MI_Instance,
    pub size: u32,
}
impl ::core::marker::Copy for MI_ConstInstanceA {}
impl ::core::clone::Clone for MI_ConstInstanceA {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_ConstInstanceA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_ConstInstanceA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_ConstInstanceA>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_ConstInstanceA {}
impl ::core::default::Default for MI_ConstInstanceA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_ConstInstanceAField {
    pub value: MI_ConstInstanceA,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_ConstInstanceAField {}
impl ::core::clone::Clone for MI_ConstInstanceAField {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_ConstInstanceAField {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_ConstInstanceAField {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_ConstInstanceAField>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_ConstInstanceAField {}
impl ::core::default::Default for MI_ConstInstanceAField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_ConstInstanceField {
    pub value: *mut MI_Instance,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_ConstInstanceField {}
impl ::core::clone::Clone for MI_ConstInstanceField {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_ConstInstanceField {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_ConstInstanceField {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_ConstInstanceField>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_ConstInstanceField {}
impl ::core::default::Default for MI_ConstInstanceField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_ConstReal32A {
    pub data: *mut f32,
    pub size: u32,
}
impl ::core::marker::Copy for MI_ConstReal32A {}
impl ::core::clone::Clone for MI_ConstReal32A {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_ConstReal32A {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_ConstReal32A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_ConstReal32A>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_ConstReal32A {}
impl ::core::default::Default for MI_ConstReal32A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_ConstReal32AField {
    pub value: MI_ConstReal32A,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_ConstReal32AField {}
impl ::core::clone::Clone for MI_ConstReal32AField {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_ConstReal32AField {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_ConstReal32AField {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_ConstReal32AField>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_ConstReal32AField {}
impl ::core::default::Default for MI_ConstReal32AField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_ConstReal32Field {
    pub value: f32,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_ConstReal32Field {}
impl ::core::clone::Clone for MI_ConstReal32Field {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_ConstReal32Field {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_ConstReal32Field {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_ConstReal32Field>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_ConstReal32Field {}
impl ::core::default::Default for MI_ConstReal32Field {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_ConstReal64A {
    pub data: *mut f64,
    pub size: u32,
}
impl ::core::marker::Copy for MI_ConstReal64A {}
impl ::core::clone::Clone for MI_ConstReal64A {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_ConstReal64A {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_ConstReal64A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_ConstReal64A>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_ConstReal64A {}
impl ::core::default::Default for MI_ConstReal64A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_ConstReal64AField {
    pub value: MI_ConstReal64A,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_ConstReal64AField {}
impl ::core::clone::Clone for MI_ConstReal64AField {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_ConstReal64AField {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_ConstReal64AField {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_ConstReal64AField>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_ConstReal64AField {}
impl ::core::default::Default for MI_ConstReal64AField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_ConstReal64Field {
    pub value: f64,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_ConstReal64Field {}
impl ::core::clone::Clone for MI_ConstReal64Field {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_ConstReal64Field {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_ConstReal64Field {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_ConstReal64Field>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_ConstReal64Field {}
impl ::core::default::Default for MI_ConstReal64Field {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_ConstReferenceA {
    pub data: *mut *mut MI_Instance,
    pub size: u32,
}
impl ::core::marker::Copy for MI_ConstReferenceA {}
impl ::core::clone::Clone for MI_ConstReferenceA {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_ConstReferenceA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_ConstReferenceA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_ConstReferenceA>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_ConstReferenceA {}
impl ::core::default::Default for MI_ConstReferenceA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_ConstReferenceAField {
    pub value: MI_ConstReferenceA,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_ConstReferenceAField {}
impl ::core::clone::Clone for MI_ConstReferenceAField {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_ConstReferenceAField {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_ConstReferenceAField {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_ConstReferenceAField>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_ConstReferenceAField {}
impl ::core::default::Default for MI_ConstReferenceAField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_ConstReferenceField {
    pub value: *mut MI_Instance,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_ConstReferenceField {}
impl ::core::clone::Clone for MI_ConstReferenceField {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_ConstReferenceField {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_ConstReferenceField {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_ConstReferenceField>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_ConstReferenceField {}
impl ::core::default::Default for MI_ConstReferenceField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_ConstSint16A {
    pub data: *mut i16,
    pub size: u32,
}
impl ::core::marker::Copy for MI_ConstSint16A {}
impl ::core::clone::Clone for MI_ConstSint16A {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_ConstSint16A {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_ConstSint16A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_ConstSint16A>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_ConstSint16A {}
impl ::core::default::Default for MI_ConstSint16A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_ConstSint16AField {
    pub value: MI_ConstSint16A,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_ConstSint16AField {}
impl ::core::clone::Clone for MI_ConstSint16AField {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_ConstSint16AField {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_ConstSint16AField {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_ConstSint16AField>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_ConstSint16AField {}
impl ::core::default::Default for MI_ConstSint16AField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_ConstSint16Field {
    pub value: i16,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_ConstSint16Field {}
impl ::core::clone::Clone for MI_ConstSint16Field {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_ConstSint16Field {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_ConstSint16Field {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_ConstSint16Field>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_ConstSint16Field {}
impl ::core::default::Default for MI_ConstSint16Field {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_ConstSint32A {
    pub data: *mut i32,
    pub size: u32,
}
impl ::core::marker::Copy for MI_ConstSint32A {}
impl ::core::clone::Clone for MI_ConstSint32A {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_ConstSint32A {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_ConstSint32A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_ConstSint32A>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_ConstSint32A {}
impl ::core::default::Default for MI_ConstSint32A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_ConstSint32AField {
    pub value: MI_ConstSint32A,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_ConstSint32AField {}
impl ::core::clone::Clone for MI_ConstSint32AField {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_ConstSint32AField {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_ConstSint32AField {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_ConstSint32AField>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_ConstSint32AField {}
impl ::core::default::Default for MI_ConstSint32AField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_ConstSint32Field {
    pub value: i32,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_ConstSint32Field {}
impl ::core::clone::Clone for MI_ConstSint32Field {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_ConstSint32Field {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_ConstSint32Field {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_ConstSint32Field>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_ConstSint32Field {}
impl ::core::default::Default for MI_ConstSint32Field {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_ConstSint64A {
    pub data: *mut i64,
    pub size: u32,
}
impl ::core::marker::Copy for MI_ConstSint64A {}
impl ::core::clone::Clone for MI_ConstSint64A {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_ConstSint64A {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_ConstSint64A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_ConstSint64A>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_ConstSint64A {}
impl ::core::default::Default for MI_ConstSint64A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_ConstSint64AField {
    pub value: MI_ConstSint64A,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_ConstSint64AField {}
impl ::core::clone::Clone for MI_ConstSint64AField {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_ConstSint64AField {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_ConstSint64AField {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_ConstSint64AField>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_ConstSint64AField {}
impl ::core::default::Default for MI_ConstSint64AField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_ConstSint64Field {
    pub value: i64,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_ConstSint64Field {}
impl ::core::clone::Clone for MI_ConstSint64Field {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_ConstSint64Field {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_ConstSint64Field {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_ConstSint64Field>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_ConstSint64Field {}
impl ::core::default::Default for MI_ConstSint64Field {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_ConstSint8A {
    pub data: *mut i8,
    pub size: u32,
}
impl ::core::marker::Copy for MI_ConstSint8A {}
impl ::core::clone::Clone for MI_ConstSint8A {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_ConstSint8A {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_ConstSint8A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_ConstSint8A>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_ConstSint8A {}
impl ::core::default::Default for MI_ConstSint8A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_ConstSint8AField {
    pub value: MI_ConstSint8A,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_ConstSint8AField {}
impl ::core::clone::Clone for MI_ConstSint8AField {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_ConstSint8AField {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_ConstSint8AField {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_ConstSint8AField>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_ConstSint8AField {}
impl ::core::default::Default for MI_ConstSint8AField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_ConstSint8Field {
    pub value: i8,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_ConstSint8Field {}
impl ::core::clone::Clone for MI_ConstSint8Field {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_ConstSint8Field {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_ConstSint8Field {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_ConstSint8Field>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_ConstSint8Field {}
impl ::core::default::Default for MI_ConstSint8Field {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_ConstStringA {
    pub data: *mut *mut u16,
    pub size: u32,
}
impl ::core::marker::Copy for MI_ConstStringA {}
impl ::core::clone::Clone for MI_ConstStringA {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_ConstStringA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_ConstStringA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_ConstStringA>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_ConstStringA {}
impl ::core::default::Default for MI_ConstStringA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_ConstStringAField {
    pub value: MI_ConstStringA,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_ConstStringAField {}
impl ::core::clone::Clone for MI_ConstStringAField {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_ConstStringAField {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_ConstStringAField {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_ConstStringAField>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_ConstStringAField {}
impl ::core::default::Default for MI_ConstStringAField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_ConstStringField {
    pub value: *mut u16,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_ConstStringField {}
impl ::core::clone::Clone for MI_ConstStringField {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_ConstStringField {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_ConstStringField {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_ConstStringField>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_ConstStringField {}
impl ::core::default::Default for MI_ConstStringField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_ConstUint16A {
    pub data: *mut u16,
    pub size: u32,
}
impl ::core::marker::Copy for MI_ConstUint16A {}
impl ::core::clone::Clone for MI_ConstUint16A {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_ConstUint16A {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_ConstUint16A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_ConstUint16A>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_ConstUint16A {}
impl ::core::default::Default for MI_ConstUint16A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_ConstUint16AField {
    pub value: MI_ConstUint16A,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_ConstUint16AField {}
impl ::core::clone::Clone for MI_ConstUint16AField {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_ConstUint16AField {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_ConstUint16AField {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_ConstUint16AField>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_ConstUint16AField {}
impl ::core::default::Default for MI_ConstUint16AField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_ConstUint16Field {
    pub value: u16,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_ConstUint16Field {}
impl ::core::clone::Clone for MI_ConstUint16Field {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_ConstUint16Field {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_ConstUint16Field {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_ConstUint16Field>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_ConstUint16Field {}
impl ::core::default::Default for MI_ConstUint16Field {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_ConstUint32A {
    pub data: *mut u32,
    pub size: u32,
}
impl ::core::marker::Copy for MI_ConstUint32A {}
impl ::core::clone::Clone for MI_ConstUint32A {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_ConstUint32A {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_ConstUint32A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_ConstUint32A>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_ConstUint32A {}
impl ::core::default::Default for MI_ConstUint32A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_ConstUint32AField {
    pub value: MI_ConstUint32A,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_ConstUint32AField {}
impl ::core::clone::Clone for MI_ConstUint32AField {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_ConstUint32AField {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_ConstUint32AField {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_ConstUint32AField>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_ConstUint32AField {}
impl ::core::default::Default for MI_ConstUint32AField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_ConstUint32Field {
    pub value: u32,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_ConstUint32Field {}
impl ::core::clone::Clone for MI_ConstUint32Field {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_ConstUint32Field {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_ConstUint32Field {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_ConstUint32Field>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_ConstUint32Field {}
impl ::core::default::Default for MI_ConstUint32Field {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_ConstUint64A {
    pub data: *mut u64,
    pub size: u32,
}
impl ::core::marker::Copy for MI_ConstUint64A {}
impl ::core::clone::Clone for MI_ConstUint64A {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_ConstUint64A {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_ConstUint64A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_ConstUint64A>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_ConstUint64A {}
impl ::core::default::Default for MI_ConstUint64A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_ConstUint64AField {
    pub value: MI_ConstUint64A,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_ConstUint64AField {}
impl ::core::clone::Clone for MI_ConstUint64AField {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_ConstUint64AField {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_ConstUint64AField {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_ConstUint64AField>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_ConstUint64AField {}
impl ::core::default::Default for MI_ConstUint64AField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_ConstUint64Field {
    pub value: u64,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_ConstUint64Field {}
impl ::core::clone::Clone for MI_ConstUint64Field {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_ConstUint64Field {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_ConstUint64Field {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_ConstUint64Field>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_ConstUint64Field {}
impl ::core::default::Default for MI_ConstUint64Field {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_ConstUint8A {
    pub data: *mut u8,
    pub size: u32,
}
impl ::core::marker::Copy for MI_ConstUint8A {}
impl ::core::clone::Clone for MI_ConstUint8A {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_ConstUint8A {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_ConstUint8A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_ConstUint8A>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_ConstUint8A {}
impl ::core::default::Default for MI_ConstUint8A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_ConstUint8AField {
    pub value: MI_ConstUint8A,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_ConstUint8AField {}
impl ::core::clone::Clone for MI_ConstUint8AField {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_ConstUint8AField {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_ConstUint8AField {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_ConstUint8AField>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_ConstUint8AField {}
impl ::core::default::Default for MI_ConstUint8AField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_ConstUint8Field {
    pub value: u8,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_ConstUint8Field {}
impl ::core::clone::Clone for MI_ConstUint8Field {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_ConstUint8Field {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_ConstUint8Field {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_ConstUint8Field>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_ConstUint8Field {}
impl ::core::default::Default for MI_ConstUint8Field {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_Context {
    pub ft: *mut MI_ContextFT,
    pub reserved: [isize; 3],
}
impl ::core::marker::Copy for MI_Context {}
impl ::core::clone::Clone for MI_Context {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_Context {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_Context {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_Context>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_Context {}
impl ::core::default::Default for MI_Context {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_ContextFT {
    pub PostResult: isize,
    pub PostInstance: isize,
    pub PostIndication: isize,
    pub ConstructInstance: isize,
    pub ConstructParameters: isize,
    pub NewInstance: isize,
    pub NewDynamicInstance: isize,
    pub NewParameters: isize,
    pub Canceled: isize,
    pub GetLocale: isize,
    pub RegisterCancel: isize,
    pub RequestUnload: isize,
    pub RefuseUnload: isize,
    pub GetLocalSession: isize,
    pub SetStringOption: isize,
    pub GetStringOption: isize,
    pub GetNumberOption: isize,
    pub GetCustomOption: isize,
    pub GetCustomOptionCount: isize,
    pub GetCustomOptionAt: isize,
    pub WriteMessage: isize,
    pub WriteProgress: isize,
    pub WriteStreamParameter: isize,
    pub WriteCimError: isize,
    pub PromptUser: isize,
    pub ShouldProcess: isize,
    pub ShouldContinue: isize,
    pub PostError: isize,
    pub PostCimError: isize,
    pub WriteError: isize,
}
impl ::core::marker::Copy for MI_ContextFT {}
impl ::core::clone::Clone for MI_ContextFT {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_ContextFT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_ContextFT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_ContextFT>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_ContextFT {}
impl ::core::default::Default for MI_ContextFT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_Datetime {
    pub isTimestamp: u32,
    pub u: MI_Datetime_0,
}
impl ::core::marker::Copy for MI_Datetime {}
impl ::core::clone::Clone for MI_Datetime {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_Datetime {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_Datetime {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_Datetime>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_Datetime {}
impl ::core::default::Default for MI_Datetime {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub union MI_Datetime_0 {
    pub timestamp: MI_Timestamp,
    pub interval: MI_Interval,
}
impl ::core::marker::Copy for MI_Datetime_0 {}
impl ::core::clone::Clone for MI_Datetime_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_Datetime_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_Datetime_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_Datetime_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_Datetime_0 {}
impl ::core::default::Default for MI_Datetime_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_DatetimeA {
    pub data: *mut MI_Datetime,
    pub size: u32,
}
impl ::core::marker::Copy for MI_DatetimeA {}
impl ::core::clone::Clone for MI_DatetimeA {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_DatetimeA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_DatetimeA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_DatetimeA>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_DatetimeA {}
impl ::core::default::Default for MI_DatetimeA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_DatetimeAField {
    pub value: MI_DatetimeA,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_DatetimeAField {}
impl ::core::clone::Clone for MI_DatetimeAField {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_DatetimeAField {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_DatetimeAField {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_DatetimeAField>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_DatetimeAField {}
impl ::core::default::Default for MI_DatetimeAField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_DatetimeField {
    pub value: MI_Datetime,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_DatetimeField {}
impl ::core::clone::Clone for MI_DatetimeField {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_DatetimeField {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_DatetimeField {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_DatetimeField>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_DatetimeField {}
impl ::core::default::Default for MI_DatetimeField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_Deserializer {
    pub reserved1: u64,
    pub reserved2: isize,
}
impl ::core::marker::Copy for MI_Deserializer {}
impl ::core::clone::Clone for MI_Deserializer {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_Deserializer {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_Deserializer {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_Deserializer>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_Deserializer {}
impl ::core::default::Default for MI_Deserializer {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_DeserializerFT {
    pub Close: isize,
    pub DeserializeClass: isize,
    pub Class_GetClassName: isize,
    pub Class_GetParentClassName: isize,
    pub DeserializeInstance: isize,
    pub Instance_GetClassName: isize,
}
impl ::core::marker::Copy for MI_DeserializerFT {}
impl ::core::clone::Clone for MI_DeserializerFT {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_DeserializerFT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_DeserializerFT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_DeserializerFT>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_DeserializerFT {}
impl ::core::default::Default for MI_DeserializerFT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub type MI_Deserializer_ClassObjectNeeded = ::core::option::Option<unsafe extern "system" fn(context: *const ::core::ffi::c_void, servername: *const u16, namespacename: *const u16, classname: *const u16, requestedclassobject: *mut *mut MI_Class) -> MI_Result>;
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_DestinationOptions {
    pub reserved1: u64,
    pub reserved2: isize,
    pub ft: *mut MI_DestinationOptionsFT,
}
impl ::core::marker::Copy for MI_DestinationOptions {}
impl ::core::clone::Clone for MI_DestinationOptions {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_DestinationOptions {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_DestinationOptions {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_DestinationOptions>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_DestinationOptions {}
impl ::core::default::Default for MI_DestinationOptions {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_DestinationOptionsFT {
    pub Delete: isize,
    pub SetString: isize,
    pub SetNumber: isize,
    pub AddCredentials: isize,
    pub GetString: isize,
    pub GetNumber: isize,
    pub GetOptionCount: isize,
    pub GetOptionAt: isize,
    pub GetOption: isize,
    pub GetCredentialsCount: isize,
    pub GetCredentialsAt: isize,
    pub GetCredentialsPasswordAt: isize,
    pub Clone: isize,
    pub SetInterval: isize,
    pub GetInterval: isize,
}
impl ::core::marker::Copy for MI_DestinationOptionsFT {}
impl ::core::clone::Clone for MI_DestinationOptionsFT {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_DestinationOptionsFT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_DestinationOptionsFT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_DestinationOptionsFT>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_DestinationOptionsFT {}
impl ::core::default::Default for MI_DestinationOptionsFT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub type MI_DestinationOptions_ImpersonationType = i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_DestinationOptions_ImpersonationType_Default: MI_DestinationOptions_ImpersonationType = 0i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_DestinationOptions_ImpersonationType_None: MI_DestinationOptions_ImpersonationType = 1i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_DestinationOptions_ImpersonationType_Identify: MI_DestinationOptions_ImpersonationType = 2i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_DestinationOptions_ImpersonationType_Impersonate: MI_DestinationOptions_ImpersonationType = 3i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_DestinationOptions_ImpersonationType_Delegate: MI_DestinationOptions_ImpersonationType = 4i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub type MI_ErrorCategory = i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_ERRORCATEGORY_NOT_SPECIFIED: MI_ErrorCategory = 0i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_ERRORCATEGORY_OPEN_ERROR: MI_ErrorCategory = 1i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_ERRORCATEGORY_CLOS_EERROR: MI_ErrorCategory = 2i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_ERRORCATEGORY_DEVICE_ERROR: MI_ErrorCategory = 3i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_ERRORCATEGORY_DEADLOCK_DETECTED: MI_ErrorCategory = 4i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_ERRORCATEGORY_INVALID_ARGUMENT: MI_ErrorCategory = 5i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_ERRORCATEGORY_INVALID_DATA: MI_ErrorCategory = 6i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_ERRORCATEGORY_INVALID_OPERATION: MI_ErrorCategory = 7i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_ERRORCATEGORY_INVALID_RESULT: MI_ErrorCategory = 8i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_ERRORCATEGORY_INVALID_TYPE: MI_ErrorCategory = 9i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_ERRORCATEGORY_METADATA_ERROR: MI_ErrorCategory = 10i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_ERRORCATEGORY_NOT_IMPLEMENTED: MI_ErrorCategory = 11i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_ERRORCATEGORY_NOT_INSTALLED: MI_ErrorCategory = 12i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_ERRORCATEGORY_OBJECT_NOT_FOUND: MI_ErrorCategory = 13i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_ERRORCATEGORY_OPERATION_STOPPED: MI_ErrorCategory = 14i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_ERRORCATEGORY_OPERATION_TIMEOUT: MI_ErrorCategory = 15i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_ERRORCATEGORY_SYNTAX_ERROR: MI_ErrorCategory = 16i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_ERRORCATEGORY_PARSER_ERROR: MI_ErrorCategory = 17i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_ERRORCATEGORY_ACCESS_DENIED: MI_ErrorCategory = 18i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_ERRORCATEGORY_RESOURCE_BUSY: MI_ErrorCategory = 19i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_ERRORCATEGORY_RESOURCE_EXISTS: MI_ErrorCategory = 20i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_ERRORCATEGORY_RESOURCE_UNAVAILABLE: MI_ErrorCategory = 21i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_ERRORCATEGORY_READ_ERROR: MI_ErrorCategory = 22i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_ERRORCATEGORY_WRITE_ERROR: MI_ErrorCategory = 23i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_ERRORCATEGORY_FROM_STDERR: MI_ErrorCategory = 24i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_ERRORCATEGORY_SECURITY_ERROR: MI_ErrorCategory = 25i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_ERRORCATEGORY_PROTOCOL_ERROR: MI_ErrorCategory = 26i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_ERRORCATEGORY_CONNECTION_ERROR: MI_ErrorCategory = 27i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_ERRORCATEGORY_AUTHENTICATION_ERROR: MI_ErrorCategory = 28i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_ERRORCATEGORY_LIMITS_EXCEEDED: MI_ErrorCategory = 29i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_ERRORCATEGORY_QUOTA_EXCEEDED: MI_ErrorCategory = 30i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_ERRORCATEGORY_NOT_ENABLED: MI_ErrorCategory = 31i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_FLAG_ABSTRACT: u32 = 131072u32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_FLAG_ADOPT: u32 = 2147483648u32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_FLAG_ANY: u32 = 127u32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_FLAG_ASSOCIATION: u32 = 16u32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_FLAG_BORROW: u32 = 1073741824u32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_FLAG_CLASS: u32 = 1u32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_FLAG_DISABLEOVERRIDE: u32 = 256u32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_FLAG_ENABLEOVERRIDE: u32 = 128u32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_FLAG_EXPENSIVE: u32 = 524288u32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_FLAG_EXTENDED: u32 = 4096u32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_FLAG_IN: u32 = 8192u32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_FLAG_INDICATION: u32 = 32u32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_FLAG_KEY: u32 = 4096u32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_FLAG_METHOD: u32 = 2u32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_FLAG_NOT_MODIFIED: u32 = 33554432u32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_FLAG_NULL: u32 = 536870912u32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_FLAG_OUT: u32 = 16384u32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_FLAG_PARAMETER: u32 = 8u32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_FLAG_PROPERTY: u32 = 4u32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_FLAG_READONLY: u32 = 2097152u32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_FLAG_REFERENCE: u32 = 64u32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_FLAG_REQUIRED: u32 = 32768u32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_FLAG_RESTRICTED: u32 = 512u32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_FLAG_STATIC: u32 = 65536u32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_FLAG_STREAM: u32 = 1048576u32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_FLAG_TERMINAL: u32 = 262144u32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_FLAG_TOSUBCLASS: u32 = 1024u32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_FLAG_TRANSLATABLE: u32 = 2048u32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_FLAG_VERSION: u32 = 469762048u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_FeatureDecl {
    pub flags: u32,
    pub code: u32,
    pub name: *mut u16,
    pub qualifiers: *mut *mut MI_Qualifier,
    pub numQualifiers: u32,
}
impl ::core::marker::Copy for MI_FeatureDecl {}
impl ::core::clone::Clone for MI_FeatureDecl {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_FeatureDecl {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_FeatureDecl {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_FeatureDecl>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_FeatureDecl {}
impl ::core::default::Default for MI_FeatureDecl {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_Filter {
    pub ft: *mut MI_FilterFT,
    pub reserved: [isize; 3],
}
impl ::core::marker::Copy for MI_Filter {}
impl ::core::clone::Clone for MI_Filter {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_Filter {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_Filter {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_Filter>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_Filter {}
impl ::core::default::Default for MI_Filter {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_FilterFT {
    pub Evaluate: isize,
    pub GetExpression: isize,
}
impl ::core::marker::Copy for MI_FilterFT {}
impl ::core::clone::Clone for MI_FilterFT {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_FilterFT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_FilterFT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_FilterFT>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_FilterFT {}
impl ::core::default::Default for MI_FilterFT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_HostedProvider {
    pub reserved1: u64,
    pub reserved2: isize,
    pub ft: *mut MI_HostedProviderFT,
}
impl ::core::marker::Copy for MI_HostedProvider {}
impl ::core::clone::Clone for MI_HostedProvider {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_HostedProvider {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_HostedProvider {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_HostedProvider>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_HostedProvider {}
impl ::core::default::Default for MI_HostedProvider {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_HostedProviderFT {
    pub Close: isize,
    pub GetApplication: isize,
}
impl ::core::marker::Copy for MI_HostedProviderFT {}
impl ::core::clone::Clone for MI_HostedProviderFT {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_HostedProviderFT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_HostedProviderFT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_HostedProviderFT>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_HostedProviderFT {}
impl ::core::default::Default for MI_HostedProviderFT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_Instance {
    pub ft: *mut MI_InstanceFT,
    pub classDecl: *mut MI_ClassDecl,
    pub serverName: *mut u16,
    pub nameSpace: *mut u16,
    pub reserved: [isize; 4],
}
impl ::core::marker::Copy for MI_Instance {}
impl ::core::clone::Clone for MI_Instance {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_Instance {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_Instance {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_Instance>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_Instance {}
impl ::core::default::Default for MI_Instance {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_InstanceA {
    pub data: *mut *mut MI_Instance,
    pub size: u32,
}
impl ::core::marker::Copy for MI_InstanceA {}
impl ::core::clone::Clone for MI_InstanceA {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_InstanceA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_InstanceA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_InstanceA>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_InstanceA {}
impl ::core::default::Default for MI_InstanceA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_InstanceAField {
    pub value: MI_InstanceA,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_InstanceAField {}
impl ::core::clone::Clone for MI_InstanceAField {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_InstanceAField {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_InstanceAField {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_InstanceAField>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_InstanceAField {}
impl ::core::default::Default for MI_InstanceAField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_InstanceExFT {
    pub parent: MI_InstanceFT,
    pub Normalize: isize,
}
impl ::core::marker::Copy for MI_InstanceExFT {}
impl ::core::clone::Clone for MI_InstanceExFT {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_InstanceExFT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_InstanceExFT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_InstanceExFT>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_InstanceExFT {}
impl ::core::default::Default for MI_InstanceExFT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_InstanceFT {
    pub Clone: isize,
    pub Destruct: isize,
    pub Delete: isize,
    pub IsA: isize,
    pub GetClassNameA: isize,
    pub SetNameSpace: isize,
    pub GetNameSpace: isize,
    pub GetElementCount: isize,
    pub AddElement: isize,
    pub SetElement: isize,
    pub SetElementAt: isize,
    pub GetElement: isize,
    pub GetElementAt: isize,
    pub ClearElement: isize,
    pub ClearElementAt: isize,
    pub GetServerName: isize,
    pub SetServerName: isize,
    pub GetClass: isize,
}
impl ::core::marker::Copy for MI_InstanceFT {}
impl ::core::clone::Clone for MI_InstanceFT {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_InstanceFT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_InstanceFT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_InstanceFT>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_InstanceFT {}
impl ::core::default::Default for MI_InstanceFT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_InstanceField {
    pub value: *mut MI_Instance,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_InstanceField {}
impl ::core::clone::Clone for MI_InstanceField {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_InstanceField {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_InstanceField {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_InstanceField>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_InstanceField {}
impl ::core::default::Default for MI_InstanceField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_Interval {
    pub days: u32,
    pub hours: u32,
    pub minutes: u32,
    pub seconds: u32,
    pub microseconds: u32,
    pub __padding1: u32,
    pub __padding2: u32,
    pub __padding3: u32,
}
impl ::core::marker::Copy for MI_Interval {}
impl ::core::clone::Clone for MI_Interval {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_Interval {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_Interval {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_Interval>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_Interval {}
impl ::core::default::Default for MI_Interval {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub type MI_LocaleType = i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_LOCALE_TYPE_REQUESTED_UI: MI_LocaleType = 0i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_LOCALE_TYPE_REQUESTED_DATA: MI_LocaleType = 1i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_LOCALE_TYPE_CLOSEST_UI: MI_LocaleType = 2i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_LOCALE_TYPE_CLOSEST_DATA: MI_LocaleType = 3i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_MAX_LOCALE_SIZE: u32 = 128u32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_MODULE_FLAG_BOOLEANS: u32 = 16u32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_MODULE_FLAG_CPLUSPLUS: u32 = 32u32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_MODULE_FLAG_DESCRIPTIONS: u32 = 2u32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_MODULE_FLAG_FILTER_SUPPORT: u32 = 128u32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_MODULE_FLAG_LOCALIZED: u32 = 64u32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_MODULE_FLAG_MAPPING_STRINGS: u32 = 8u32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_MODULE_FLAG_STANDARD_QUALIFIERS: u32 = 1u32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_MODULE_FLAG_VALUES: u32 = 4u32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub type MI_MainFunction = ::core::option::Option<unsafe extern "system" fn(server: *mut MI_Server) -> *mut MI_Module>;
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_MethodDecl {
    pub flags: u32,
    pub code: u32,
    pub name: *mut u16,
    pub qualifiers: *mut *mut MI_Qualifier,
    pub numQualifiers: u32,
    pub parameters: *mut *mut MI_ParameterDecl,
    pub numParameters: u32,
    pub size: u32,
    pub returnType: u32,
    pub origin: *mut u16,
    pub propagator: *mut u16,
    pub schema: *mut MI_SchemaDecl,
    pub function: MI_MethodDecl_Invoke,
}
impl ::core::marker::Copy for MI_MethodDecl {}
impl ::core::clone::Clone for MI_MethodDecl {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_MethodDecl {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_MethodDecl {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_MethodDecl>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_MethodDecl {}
impl ::core::default::Default for MI_MethodDecl {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub type MI_MethodDecl_Invoke = ::core::option::Option<unsafe extern "system" fn(self_: *const ::core::ffi::c_void, context: *const MI_Context, namespace: *const u16, classname: *const u16, methodname: *const u16, instancename: *const MI_Instance, parameters: *const MI_Instance)>;
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_Module {
    pub version: u32,
    pub generatorVersion: u32,
    pub flags: u32,
    pub charSize: u32,
    pub schemaDecl: *mut MI_SchemaDecl,
    pub Load: MI_Module_Load,
    pub Unload: MI_Module_Unload,
    pub dynamicProviderFT: *mut MI_ProviderFT,
}
impl ::core::marker::Copy for MI_Module {}
impl ::core::clone::Clone for MI_Module {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_Module {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_Module {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_Module>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_Module {}
impl ::core::default::Default for MI_Module {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub type MI_Module_Load = ::core::option::Option<unsafe extern "system" fn(self_: *mut *mut MI_Module_Self, context: *const MI_Context)>;
#[repr(C)]
pub struct MI_Module_Self(pub u8);
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub type MI_Module_Unload = ::core::option::Option<unsafe extern "system" fn(self_: *const MI_Module_Self, context: *const MI_Context)>;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_OPERATIONFLAGS_BASIC_RTTI: u32 = 2u32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_OPERATIONFLAGS_DEFAULT_RTTI: u32 = 0u32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_OPERATIONFLAGS_EXPENSIVE_PROPERTIES: u32 = 64u32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_OPERATIONFLAGS_FULL_RTTI: u32 = 4u32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_OPERATIONFLAGS_LOCALIZED_QUALIFIERS: u32 = 8u32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_OPERATIONFLAGS_MANUAL_ACK_RESULTS: u32 = 1u32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_OPERATIONFLAGS_NO_RTTI: u32 = 1024u32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_OPERATIONFLAGS_POLYMORPHISM_DEEP_BASE_PROPS_ONLY: u32 = 384u32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_OPERATIONFLAGS_POLYMORPHISM_SHALLOW: u32 = 128u32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_OPERATIONFLAGS_REPORT_OPERATION_STARTED: u32 = 512u32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_OPERATIONFLAGS_STANDARD_RTTI: u32 = 2048u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_ObjectDecl {
    pub flags: u32,
    pub code: u32,
    pub name: *mut u16,
    pub qualifiers: *mut *mut MI_Qualifier,
    pub numQualifiers: u32,
    pub properties: *mut *mut MI_PropertyDecl,
    pub numProperties: u32,
    pub size: u32,
}
impl ::core::marker::Copy for MI_ObjectDecl {}
impl ::core::clone::Clone for MI_ObjectDecl {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_ObjectDecl {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_ObjectDecl {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_ObjectDecl>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_ObjectDecl {}
impl ::core::default::Default for MI_ObjectDecl {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_Operation {
    pub reserved1: u64,
    pub reserved2: isize,
    pub ft: *mut MI_OperationFT,
}
impl ::core::marker::Copy for MI_Operation {}
impl ::core::clone::Clone for MI_Operation {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_Operation {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_Operation {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_Operation>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_Operation {}
impl ::core::default::Default for MI_Operation {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub type MI_OperationCallback_Class = ::core::option::Option<unsafe extern "system" fn(operation: *const MI_Operation, callbackcontext: *const ::core::ffi::c_void, classresult: *const MI_Class, moreresults: u8, resultcode: MI_Result, errorstring: *const u16, errordetails: *const MI_Instance, resultacknowledgement: isize)>;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub type MI_OperationCallback_Indication = ::core::option::Option<unsafe extern "system" fn(operation: *const MI_Operation, callbackcontext: *const ::core::ffi::c_void, instance: *const MI_Instance, bookmark: *const u16, machineid: *const u16, moreresults: u8, resultcode: MI_Result, errorstring: *const u16, errordetails: *const MI_Instance, resultacknowledgement: isize)>;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub type MI_OperationCallback_Instance = ::core::option::Option<unsafe extern "system" fn(operation: *const MI_Operation, callbackcontext: *const ::core::ffi::c_void, instance: *const MI_Instance, moreresults: u8, resultcode: MI_Result, errorstring: *const u16, errordetails: *const MI_Instance, resultacknowledgement: isize)>;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub type MI_OperationCallback_PromptUser = ::core::option::Option<unsafe extern "system" fn(operation: *const MI_Operation, callbackcontext: *const ::core::ffi::c_void, message: *const u16, prompttype: MI_PromptType, promptuserresult: isize)>;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub type MI_OperationCallback_ResponseType = i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_OperationCallback_ResponseType_No: MI_OperationCallback_ResponseType = 0i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_OperationCallback_ResponseType_Yes: MI_OperationCallback_ResponseType = 1i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_OperationCallback_ResponseType_NoToAll: MI_OperationCallback_ResponseType = 2i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_OperationCallback_ResponseType_YesToAll: MI_OperationCallback_ResponseType = 3i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub type MI_OperationCallback_StreamedParameter = ::core::option::Option<unsafe extern "system" fn(operation: *const MI_Operation, callbackcontext: *const ::core::ffi::c_void, parametername: *const u16, resulttype: MI_Type, result: *const MI_Value, resultacknowledgement: isize)>;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub type MI_OperationCallback_WriteError = ::core::option::Option<unsafe extern "system" fn(operation: *const MI_Operation, callbackcontext: *const ::core::ffi::c_void, instance: *const MI_Instance, writeerrorresult: isize)>;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub type MI_OperationCallback_WriteMessage = ::core::option::Option<unsafe extern "system" fn(operation: *const MI_Operation, callbackcontext: *const ::core::ffi::c_void, channel: u32, message: *const u16)>;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub type MI_OperationCallback_WriteProgress = ::core::option::Option<unsafe extern "system" fn(operation: *const MI_Operation, callbackcontext: *const ::core::ffi::c_void, activity: *const u16, currentoperation: *const u16, statusdescription: *const u16, percentagecomplete: u32, secondsremaining: u32)>;
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_OperationCallbacks {
    pub callbackContext: *mut ::core::ffi::c_void,
    pub promptUser: MI_OperationCallback_PromptUser,
    pub writeError: MI_OperationCallback_WriteError,
    pub writeMessage: MI_OperationCallback_WriteMessage,
    pub writeProgress: MI_OperationCallback_WriteProgress,
    pub instanceResult: MI_OperationCallback_Instance,
    pub indicationResult: MI_OperationCallback_Indication,
    pub classResult: MI_OperationCallback_Class,
    pub streamedParameterResult: MI_OperationCallback_StreamedParameter,
}
impl ::core::marker::Copy for MI_OperationCallbacks {}
impl ::core::clone::Clone for MI_OperationCallbacks {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_OperationCallbacks {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_OperationCallbacks {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_OperationCallbacks>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_OperationCallbacks {}
impl ::core::default::Default for MI_OperationCallbacks {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_OperationFT {
    pub Close: isize,
    pub Cancel: isize,
    pub GetSession: isize,
    pub GetInstance: isize,
    pub GetIndication: isize,
    pub GetClass: isize,
}
impl ::core::marker::Copy for MI_OperationFT {}
impl ::core::clone::Clone for MI_OperationFT {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_OperationFT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_OperationFT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_OperationFT>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_OperationFT {}
impl ::core::default::Default for MI_OperationFT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_OperationOptions {
    pub reserved1: u64,
    pub reserved2: isize,
    pub ft: *mut MI_OperationOptionsFT,
}
impl ::core::marker::Copy for MI_OperationOptions {}
impl ::core::clone::Clone for MI_OperationOptions {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_OperationOptions {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_OperationOptions {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_OperationOptions>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_OperationOptions {}
impl ::core::default::Default for MI_OperationOptions {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_OperationOptionsFT {
    pub Delete: isize,
    pub SetString: isize,
    pub SetNumber: isize,
    pub SetCustomOption: isize,
    pub GetString: isize,
    pub GetNumber: isize,
    pub GetOptionCount: isize,
    pub GetOptionAt: isize,
    pub GetOption: isize,
    pub GetEnabledChannels: isize,
    pub Clone: isize,
    pub SetInterval: isize,
    pub GetInterval: isize,
}
impl ::core::marker::Copy for MI_OperationOptionsFT {}
impl ::core::clone::Clone for MI_OperationOptionsFT {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_OperationOptionsFT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_OperationOptionsFT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_OperationOptionsFT>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_OperationOptionsFT {}
impl ::core::default::Default for MI_OperationOptionsFT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_ParameterDecl {
    pub flags: u32,
    pub code: u32,
    pub name: *mut u16,
    pub qualifiers: *mut *mut MI_Qualifier,
    pub numQualifiers: u32,
    pub r#type: u32,
    pub className: *mut u16,
    pub subscript: u32,
    pub offset: u32,
}
impl ::core::marker::Copy for MI_ParameterDecl {}
impl ::core::clone::Clone for MI_ParameterDecl {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_ParameterDecl {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_ParameterDecl {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_ParameterDecl>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_ParameterDecl {}
impl ::core::default::Default for MI_ParameterDecl {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_ParameterSet {
    pub reserved1: u64,
    pub reserved2: isize,
    pub ft: *mut MI_ParameterSetFT,
}
impl ::core::marker::Copy for MI_ParameterSet {}
impl ::core::clone::Clone for MI_ParameterSet {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_ParameterSet {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_ParameterSet {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_ParameterSet>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_ParameterSet {}
impl ::core::default::Default for MI_ParameterSet {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_ParameterSetFT {
    pub GetMethodReturnType: isize,
    pub GetParameterCount: isize,
    pub GetParameterAt: isize,
    pub GetParameter: isize,
}
impl ::core::marker::Copy for MI_ParameterSetFT {}
impl ::core::clone::Clone for MI_ParameterSetFT {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_ParameterSetFT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_ParameterSetFT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_ParameterSetFT>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_ParameterSetFT {}
impl ::core::default::Default for MI_ParameterSetFT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub type MI_PromptType = i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_PROMPTTYPE_NORMAL: MI_PromptType = 0i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_PROMPTTYPE_CRITICAL: MI_PromptType = 1i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_PropertyDecl {
    pub flags: u32,
    pub code: u32,
    pub name: *mut u16,
    pub qualifiers: *mut *mut MI_Qualifier,
    pub numQualifiers: u32,
    pub r#type: u32,
    pub className: *mut u16,
    pub subscript: u32,
    pub offset: u32,
    pub origin: *mut u16,
    pub propagator: *mut u16,
    pub value: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for MI_PropertyDecl {}
impl ::core::clone::Clone for MI_PropertyDecl {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_PropertyDecl {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_PropertyDecl {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_PropertyDecl>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_PropertyDecl {}
impl ::core::default::Default for MI_PropertyDecl {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_PropertySet {
    pub ft: *mut MI_PropertySetFT,
    pub reserved: [isize; 3],
}
impl ::core::marker::Copy for MI_PropertySet {}
impl ::core::clone::Clone for MI_PropertySet {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_PropertySet {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_PropertySet {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_PropertySet>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_PropertySet {}
impl ::core::default::Default for MI_PropertySet {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_PropertySetFT {
    pub GetElementCount: isize,
    pub ContainsElement: isize,
    pub AddElement: isize,
    pub GetElementAt: isize,
    pub Clear: isize,
    pub Destruct: isize,
    pub Delete: isize,
    pub Clone: isize,
}
impl ::core::marker::Copy for MI_PropertySetFT {}
impl ::core::clone::Clone for MI_PropertySetFT {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_PropertySetFT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_PropertySetFT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_PropertySetFT>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_PropertySetFT {}
impl ::core::default::Default for MI_PropertySetFT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub type MI_ProviderArchitecture = i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_PROVIDER_ARCHITECTURE_32BIT: MI_ProviderArchitecture = 0i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_PROVIDER_ARCHITECTURE_64BIT: MI_ProviderArchitecture = 1i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_ProviderFT {
    pub Load: MI_ProviderFT_Load,
    pub Unload: MI_ProviderFT_Unload,
    pub GetInstance: MI_ProviderFT_GetInstance,
    pub EnumerateInstances: MI_ProviderFT_EnumerateInstances,
    pub CreateInstance: MI_ProviderFT_CreateInstance,
    pub ModifyInstance: MI_ProviderFT_ModifyInstance,
    pub DeleteInstance: MI_ProviderFT_DeleteInstance,
    pub AssociatorInstances: MI_ProviderFT_AssociatorInstances,
    pub ReferenceInstances: MI_ProviderFT_ReferenceInstances,
    pub EnableIndications: MI_ProviderFT_EnableIndications,
    pub DisableIndications: MI_ProviderFT_DisableIndications,
    pub Subscribe: MI_ProviderFT_Subscribe,
    pub Unsubscribe: MI_ProviderFT_Unsubscribe,
    pub Invoke: MI_ProviderFT_Invoke,
}
impl ::core::marker::Copy for MI_ProviderFT {}
impl ::core::clone::Clone for MI_ProviderFT {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_ProviderFT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_ProviderFT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_ProviderFT>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_ProviderFT {}
impl ::core::default::Default for MI_ProviderFT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub type MI_ProviderFT_AssociatorInstances = ::core::option::Option<unsafe extern "system" fn(self_: *const ::core::ffi::c_void, context: *const MI_Context, namespace: *const u16, classname: *const u16, instancename: *const MI_Instance, resultclass: *const u16, role: *const u16, resultrole: *const u16, propertyset: *const MI_PropertySet, keysonly: u8, filter: *const MI_Filter)>;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub type MI_ProviderFT_CreateInstance = ::core::option::Option<unsafe extern "system" fn(self_: *const ::core::ffi::c_void, context: *const MI_Context, namespace: *const u16, classname: *const u16, newinstance: *const MI_Instance)>;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub type MI_ProviderFT_DeleteInstance = ::core::option::Option<unsafe extern "system" fn(self_: *const ::core::ffi::c_void, context: *const MI_Context, namespace: *const u16, classname: *const u16, instancename: *const MI_Instance)>;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub type MI_ProviderFT_DisableIndications = ::core::option::Option<unsafe extern "system" fn(self_: *const ::core::ffi::c_void, indicationscontext: *const MI_Context, namespace: *const u16, classname: *const u16)>;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub type MI_ProviderFT_EnableIndications = ::core::option::Option<unsafe extern "system" fn(self_: *const ::core::ffi::c_void, indicationscontext: *const MI_Context, namespace: *const u16, classname: *const u16)>;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub type MI_ProviderFT_EnumerateInstances = ::core::option::Option<unsafe extern "system" fn(self_: *const ::core::ffi::c_void, context: *const MI_Context, namespace: *const u16, classname: *const u16, propertyset: *const MI_PropertySet, keysonly: u8, filter: *const MI_Filter)>;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub type MI_ProviderFT_GetInstance = ::core::option::Option<unsafe extern "system" fn(self_: *const ::core::ffi::c_void, context: *const MI_Context, namespace: *const u16, classname: *const u16, instancename: *const MI_Instance, propertyset: *const MI_PropertySet)>;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub type MI_ProviderFT_Invoke = ::core::option::Option<unsafe extern "system" fn(self_: *const ::core::ffi::c_void, context: *const MI_Context, namespace: *const u16, classname: *const u16, methodname: *const u16, instancename: *const MI_Instance, inputparameters: *const MI_Instance)>;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub type MI_ProviderFT_Load = ::core::option::Option<unsafe extern "system" fn(self_: *mut *mut ::core::ffi::c_void, selfmodule: *const MI_Module_Self, context: *const MI_Context)>;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub type MI_ProviderFT_ModifyInstance = ::core::option::Option<unsafe extern "system" fn(self_: *mut ::core::ffi::c_void, context: *mut MI_Context, namespace: *const u16, classname: *const u16, modifiedinstance: *const MI_Instance, propertyset: *const MI_PropertySet)>;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub type MI_ProviderFT_ReferenceInstances = ::core::option::Option<unsafe extern "system" fn(self_: *const ::core::ffi::c_void, context: *const MI_Context, namespace: *const u16, classname: *const u16, instancename: *const MI_Instance, role: *const u16, propertyset: *const MI_PropertySet, keysonly: u8, filter: *const MI_Filter)>;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub type MI_ProviderFT_Subscribe = ::core::option::Option<unsafe extern "system" fn(self_: *const ::core::ffi::c_void, context: *const MI_Context, namespace: *const u16, classname: *const u16, filter: *const MI_Filter, bookmark: *const u16, subscriptionid: u64, subscriptionself: *mut *mut ::core::ffi::c_void)>;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub type MI_ProviderFT_Unload = ::core::option::Option<unsafe extern "system" fn(self_: *const ::core::ffi::c_void, context: *const MI_Context)>;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub type MI_ProviderFT_Unsubscribe = ::core::option::Option<unsafe extern "system" fn(self_: *const ::core::ffi::c_void, context: *const MI_Context, namespace: *const u16, classname: *const u16, subscriptionid: u64, subscriptionself: *const ::core::ffi::c_void)>;
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_Qualifier {
    pub name: *mut u16,
    pub r#type: u32,
    pub flavor: u32,
    pub value: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for MI_Qualifier {}
impl ::core::clone::Clone for MI_Qualifier {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_Qualifier {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_Qualifier {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_Qualifier>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_Qualifier {}
impl ::core::default::Default for MI_Qualifier {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_QualifierDecl {
    pub name: *mut u16,
    pub r#type: u32,
    pub scope: u32,
    pub flavor: u32,
    pub subscript: u32,
    pub value: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for MI_QualifierDecl {}
impl ::core::clone::Clone for MI_QualifierDecl {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_QualifierDecl {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_QualifierDecl {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_QualifierDecl>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_QualifierDecl {}
impl ::core::default::Default for MI_QualifierDecl {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_QualifierSet {
    pub reserved1: u64,
    pub reserved2: isize,
    pub ft: *mut MI_QualifierSetFT,
}
impl ::core::marker::Copy for MI_QualifierSet {}
impl ::core::clone::Clone for MI_QualifierSet {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_QualifierSet {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_QualifierSet {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_QualifierSet>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_QualifierSet {}
impl ::core::default::Default for MI_QualifierSet {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_QualifierSetFT {
    pub GetQualifierCount: isize,
    pub GetQualifierAt: isize,
    pub GetQualifier: isize,
}
impl ::core::marker::Copy for MI_QualifierSetFT {}
impl ::core::clone::Clone for MI_QualifierSetFT {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_QualifierSetFT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_QualifierSetFT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_QualifierSetFT>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_QualifierSetFT {}
impl ::core::default::Default for MI_QualifierSetFT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_Real32A {
    pub data: *mut f32,
    pub size: u32,
}
impl ::core::marker::Copy for MI_Real32A {}
impl ::core::clone::Clone for MI_Real32A {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_Real32A {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_Real32A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_Real32A>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_Real32A {}
impl ::core::default::Default for MI_Real32A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_Real32AField {
    pub value: MI_Real32A,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_Real32AField {}
impl ::core::clone::Clone for MI_Real32AField {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_Real32AField {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_Real32AField {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_Real32AField>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_Real32AField {}
impl ::core::default::Default for MI_Real32AField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_Real32Field {
    pub value: f32,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_Real32Field {}
impl ::core::clone::Clone for MI_Real32Field {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_Real32Field {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_Real32Field {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_Real32Field>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_Real32Field {}
impl ::core::default::Default for MI_Real32Field {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_Real64A {
    pub data: *mut f64,
    pub size: u32,
}
impl ::core::marker::Copy for MI_Real64A {}
impl ::core::clone::Clone for MI_Real64A {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_Real64A {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_Real64A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_Real64A>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_Real64A {}
impl ::core::default::Default for MI_Real64A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_Real64AField {
    pub value: MI_Real64A,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_Real64AField {}
impl ::core::clone::Clone for MI_Real64AField {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_Real64AField {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_Real64AField {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_Real64AField>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_Real64AField {}
impl ::core::default::Default for MI_Real64AField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_Real64Field {
    pub value: f64,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_Real64Field {}
impl ::core::clone::Clone for MI_Real64Field {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_Real64Field {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_Real64Field {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_Real64Field>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_Real64Field {}
impl ::core::default::Default for MI_Real64Field {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_ReferenceA {
    pub data: *mut *mut MI_Instance,
    pub size: u32,
}
impl ::core::marker::Copy for MI_ReferenceA {}
impl ::core::clone::Clone for MI_ReferenceA {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_ReferenceA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_ReferenceA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_ReferenceA>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_ReferenceA {}
impl ::core::default::Default for MI_ReferenceA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_ReferenceAField {
    pub value: MI_ReferenceA,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_ReferenceAField {}
impl ::core::clone::Clone for MI_ReferenceAField {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_ReferenceAField {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_ReferenceAField {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_ReferenceAField>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_ReferenceAField {}
impl ::core::default::Default for MI_ReferenceAField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_ReferenceField {
    pub value: *mut MI_Instance,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_ReferenceField {}
impl ::core::clone::Clone for MI_ReferenceField {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_ReferenceField {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_ReferenceField {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_ReferenceField>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_ReferenceField {}
impl ::core::default::Default for MI_ReferenceField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub type MI_Result = i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_RESULT_OK: MI_Result = 0i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_RESULT_FAILED: MI_Result = 1i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_RESULT_ACCESS_DENIED: MI_Result = 2i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_RESULT_INVALID_NAMESPACE: MI_Result = 3i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_RESULT_INVALID_PARAMETER: MI_Result = 4i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_RESULT_INVALID_CLASS: MI_Result = 5i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_RESULT_NOT_FOUND: MI_Result = 6i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_RESULT_NOT_SUPPORTED: MI_Result = 7i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_RESULT_CLASS_HAS_CHILDREN: MI_Result = 8i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_RESULT_CLASS_HAS_INSTANCES: MI_Result = 9i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_RESULT_INVALID_SUPERCLASS: MI_Result = 10i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_RESULT_ALREADY_EXISTS: MI_Result = 11i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_RESULT_NO_SUCH_PROPERTY: MI_Result = 12i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_RESULT_TYPE_MISMATCH: MI_Result = 13i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_RESULT_QUERY_LANGUAGE_NOT_SUPPORTED: MI_Result = 14i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_RESULT_INVALID_QUERY: MI_Result = 15i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_RESULT_METHOD_NOT_AVAILABLE: MI_Result = 16i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_RESULT_METHOD_NOT_FOUND: MI_Result = 17i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_RESULT_NAMESPACE_NOT_EMPTY: MI_Result = 20i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_RESULT_INVALID_ENUMERATION_CONTEXT: MI_Result = 21i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_RESULT_INVALID_OPERATION_TIMEOUT: MI_Result = 22i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_RESULT_PULL_HAS_BEEN_ABANDONED: MI_Result = 23i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_RESULT_PULL_CANNOT_BE_ABANDONED: MI_Result = 24i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_RESULT_FILTERED_ENUMERATION_NOT_SUPPORTED: MI_Result = 25i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_RESULT_CONTINUATION_ON_ERROR_NOT_SUPPORTED: MI_Result = 26i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_RESULT_SERVER_LIMITS_EXCEEDED: MI_Result = 27i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_RESULT_SERVER_IS_SHUTTING_DOWN: MI_Result = 28i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_SERIALIZER_FLAGS_CLASS_DEEP: u32 = 1u32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_SERIALIZER_FLAGS_INSTANCE_WITH_CLASS: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_SchemaDecl {
    pub qualifierDecls: *mut *mut MI_QualifierDecl,
    pub numQualifierDecls: u32,
    pub classDecls: *mut *mut MI_ClassDecl,
    pub numClassDecls: u32,
}
impl ::core::marker::Copy for MI_SchemaDecl {}
impl ::core::clone::Clone for MI_SchemaDecl {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_SchemaDecl {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_SchemaDecl {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_SchemaDecl>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_SchemaDecl {}
impl ::core::default::Default for MI_SchemaDecl {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_Serializer {
    pub reserved1: u64,
    pub reserved2: isize,
}
impl ::core::marker::Copy for MI_Serializer {}
impl ::core::clone::Clone for MI_Serializer {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_Serializer {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_Serializer {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_Serializer>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_Serializer {}
impl ::core::default::Default for MI_Serializer {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_SerializerFT {
    pub Close: isize,
    pub SerializeClass: isize,
    pub SerializeInstance: isize,
}
impl ::core::marker::Copy for MI_SerializerFT {}
impl ::core::clone::Clone for MI_SerializerFT {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_SerializerFT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_SerializerFT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_SerializerFT>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_SerializerFT {}
impl ::core::default::Default for MI_SerializerFT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_Server {
    pub serverFT: *mut MI_ServerFT,
    pub contextFT: *mut MI_ContextFT,
    pub instanceFT: *mut MI_InstanceFT,
    pub propertySetFT: *mut MI_PropertySetFT,
    pub filterFT: *mut MI_FilterFT,
}
impl ::core::marker::Copy for MI_Server {}
impl ::core::clone::Clone for MI_Server {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_Server {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_Server {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_Server>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_Server {}
impl ::core::default::Default for MI_Server {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_ServerFT {
    pub GetVersion: isize,
    pub GetSystemName: isize,
}
impl ::core::marker::Copy for MI_ServerFT {}
impl ::core::clone::Clone for MI_ServerFT {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_ServerFT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_ServerFT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_ServerFT>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_ServerFT {}
impl ::core::default::Default for MI_ServerFT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_Session {
    pub reserved1: u64,
    pub reserved2: isize,
    pub ft: *mut MI_SessionFT,
}
impl ::core::marker::Copy for MI_Session {}
impl ::core::clone::Clone for MI_Session {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_Session {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_Session {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_Session>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_Session {}
impl ::core::default::Default for MI_Session {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_SessionCallbacks {
    pub callbackContext: *mut ::core::ffi::c_void,
    pub writeMessage: isize,
    pub writeError: isize,
}
impl ::core::marker::Copy for MI_SessionCallbacks {}
impl ::core::clone::Clone for MI_SessionCallbacks {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_SessionCallbacks {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_SessionCallbacks {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_SessionCallbacks>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_SessionCallbacks {}
impl ::core::default::Default for MI_SessionCallbacks {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_SessionFT {
    pub Close: isize,
    pub GetApplication: isize,
    pub GetInstance: isize,
    pub ModifyInstance: isize,
    pub CreateInstance: isize,
    pub DeleteInstance: isize,
    pub Invoke: isize,
    pub EnumerateInstances: isize,
    pub QueryInstances: isize,
    pub AssociatorInstances: isize,
    pub ReferenceInstances: isize,
    pub Subscribe: isize,
    pub GetClass: isize,
    pub EnumerateClasses: isize,
    pub TestConnection: isize,
}
impl ::core::marker::Copy for MI_SessionFT {}
impl ::core::clone::Clone for MI_SessionFT {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_SessionFT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_SessionFT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_SessionFT>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_SessionFT {}
impl ::core::default::Default for MI_SessionFT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_Sint16A {
    pub data: *mut i16,
    pub size: u32,
}
impl ::core::marker::Copy for MI_Sint16A {}
impl ::core::clone::Clone for MI_Sint16A {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_Sint16A {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_Sint16A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_Sint16A>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_Sint16A {}
impl ::core::default::Default for MI_Sint16A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_Sint16AField {
    pub value: MI_Sint16A,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_Sint16AField {}
impl ::core::clone::Clone for MI_Sint16AField {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_Sint16AField {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_Sint16AField {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_Sint16AField>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_Sint16AField {}
impl ::core::default::Default for MI_Sint16AField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_Sint16Field {
    pub value: i16,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_Sint16Field {}
impl ::core::clone::Clone for MI_Sint16Field {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_Sint16Field {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_Sint16Field {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_Sint16Field>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_Sint16Field {}
impl ::core::default::Default for MI_Sint16Field {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_Sint32A {
    pub data: *mut i32,
    pub size: u32,
}
impl ::core::marker::Copy for MI_Sint32A {}
impl ::core::clone::Clone for MI_Sint32A {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_Sint32A {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_Sint32A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_Sint32A>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_Sint32A {}
impl ::core::default::Default for MI_Sint32A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_Sint32AField {
    pub value: MI_Sint32A,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_Sint32AField {}
impl ::core::clone::Clone for MI_Sint32AField {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_Sint32AField {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_Sint32AField {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_Sint32AField>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_Sint32AField {}
impl ::core::default::Default for MI_Sint32AField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_Sint32Field {
    pub value: i32,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_Sint32Field {}
impl ::core::clone::Clone for MI_Sint32Field {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_Sint32Field {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_Sint32Field {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_Sint32Field>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_Sint32Field {}
impl ::core::default::Default for MI_Sint32Field {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_Sint64A {
    pub data: *mut i64,
    pub size: u32,
}
impl ::core::marker::Copy for MI_Sint64A {}
impl ::core::clone::Clone for MI_Sint64A {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_Sint64A {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_Sint64A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_Sint64A>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_Sint64A {}
impl ::core::default::Default for MI_Sint64A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_Sint64AField {
    pub value: MI_Sint64A,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_Sint64AField {}
impl ::core::clone::Clone for MI_Sint64AField {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_Sint64AField {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_Sint64AField {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_Sint64AField>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_Sint64AField {}
impl ::core::default::Default for MI_Sint64AField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_Sint64Field {
    pub value: i64,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_Sint64Field {}
impl ::core::clone::Clone for MI_Sint64Field {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_Sint64Field {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_Sint64Field {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_Sint64Field>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_Sint64Field {}
impl ::core::default::Default for MI_Sint64Field {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_Sint8A {
    pub data: *mut i8,
    pub size: u32,
}
impl ::core::marker::Copy for MI_Sint8A {}
impl ::core::clone::Clone for MI_Sint8A {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_Sint8A {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_Sint8A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_Sint8A>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_Sint8A {}
impl ::core::default::Default for MI_Sint8A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_Sint8AField {
    pub value: MI_Sint8A,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_Sint8AField {}
impl ::core::clone::Clone for MI_Sint8AField {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_Sint8AField {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_Sint8AField {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_Sint8AField>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_Sint8AField {}
impl ::core::default::Default for MI_Sint8AField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_Sint8Field {
    pub value: i8,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_Sint8Field {}
impl ::core::clone::Clone for MI_Sint8Field {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_Sint8Field {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_Sint8Field {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_Sint8Field>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_Sint8Field {}
impl ::core::default::Default for MI_Sint8Field {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_StringA {
    pub data: *mut *mut u16,
    pub size: u32,
}
impl ::core::marker::Copy for MI_StringA {}
impl ::core::clone::Clone for MI_StringA {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_StringA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_StringA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_StringA>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_StringA {}
impl ::core::default::Default for MI_StringA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_StringAField {
    pub value: MI_StringA,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_StringAField {}
impl ::core::clone::Clone for MI_StringAField {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_StringAField {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_StringAField {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_StringAField>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_StringAField {}
impl ::core::default::Default for MI_StringAField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_StringField {
    pub value: *mut u16,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_StringField {}
impl ::core::clone::Clone for MI_StringField {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_StringField {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_StringField {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_StringField>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_StringField {}
impl ::core::default::Default for MI_StringField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_SubscriptionDeliveryOptions {
    pub reserved1: u64,
    pub reserved2: isize,
    pub ft: *mut MI_SubscriptionDeliveryOptionsFT,
}
impl ::core::marker::Copy for MI_SubscriptionDeliveryOptions {}
impl ::core::clone::Clone for MI_SubscriptionDeliveryOptions {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_SubscriptionDeliveryOptions {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_SubscriptionDeliveryOptions {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_SubscriptionDeliveryOptions>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_SubscriptionDeliveryOptions {}
impl ::core::default::Default for MI_SubscriptionDeliveryOptions {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_SubscriptionDeliveryOptionsFT {
    pub SetString: isize,
    pub SetNumber: isize,
    pub SetDateTime: isize,
    pub SetInterval: isize,
    pub AddCredentials: isize,
    pub Delete: isize,
    pub GetString: isize,
    pub GetNumber: isize,
    pub GetDateTime: isize,
    pub GetInterval: isize,
    pub GetOptionCount: isize,
    pub GetOptionAt: isize,
    pub GetOption: isize,
    pub GetCredentialsCount: isize,
    pub GetCredentialsAt: isize,
    pub GetCredentialsPasswordAt: isize,
    pub Clone: isize,
}
impl ::core::marker::Copy for MI_SubscriptionDeliveryOptionsFT {}
impl ::core::clone::Clone for MI_SubscriptionDeliveryOptionsFT {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_SubscriptionDeliveryOptionsFT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_SubscriptionDeliveryOptionsFT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_SubscriptionDeliveryOptionsFT>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_SubscriptionDeliveryOptionsFT {}
impl ::core::default::Default for MI_SubscriptionDeliveryOptionsFT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub type MI_SubscriptionDeliveryType = i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_SubscriptionDeliveryType_Pull: MI_SubscriptionDeliveryType = 1i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_SubscriptionDeliveryType_Push: MI_SubscriptionDeliveryType = 2i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_Timestamp {
    pub year: u32,
    pub month: u32,
    pub day: u32,
    pub hour: u32,
    pub minute: u32,
    pub second: u32,
    pub microseconds: u32,
    pub utc: i32,
}
impl ::core::marker::Copy for MI_Timestamp {}
impl ::core::clone::Clone for MI_Timestamp {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_Timestamp {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_Timestamp {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_Timestamp>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_Timestamp {}
impl ::core::default::Default for MI_Timestamp {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub type MI_Type = i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_BOOLEAN: MI_Type = 0i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_UINT8: MI_Type = 1i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_SINT8: MI_Type = 2i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_UINT16: MI_Type = 3i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_SINT16: MI_Type = 4i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_UINT32: MI_Type = 5i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_SINT32: MI_Type = 6i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_UINT64: MI_Type = 7i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_SINT64: MI_Type = 8i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_REAL32: MI_Type = 9i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_REAL64: MI_Type = 10i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_CHAR16: MI_Type = 11i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_DATETIME: MI_Type = 12i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_STRING: MI_Type = 13i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_REFERENCE: MI_Type = 14i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_INSTANCE: MI_Type = 15i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_BOOLEANA: MI_Type = 16i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_UINT8A: MI_Type = 17i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_SINT8A: MI_Type = 18i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_UINT16A: MI_Type = 19i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_SINT16A: MI_Type = 20i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_UINT32A: MI_Type = 21i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_SINT32A: MI_Type = 22i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_UINT64A: MI_Type = 23i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_SINT64A: MI_Type = 24i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_REAL32A: MI_Type = 25i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_REAL64A: MI_Type = 26i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_CHAR16A: MI_Type = 27i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_DATETIMEA: MI_Type = 28i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_STRINGA: MI_Type = 29i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_REFERENCEA: MI_Type = 30i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_INSTANCEA: MI_Type = 31i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_ARRAY: MI_Type = 16i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_Uint16A {
    pub data: *mut u16,
    pub size: u32,
}
impl ::core::marker::Copy for MI_Uint16A {}
impl ::core::clone::Clone for MI_Uint16A {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_Uint16A {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_Uint16A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_Uint16A>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_Uint16A {}
impl ::core::default::Default for MI_Uint16A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_Uint16AField {
    pub value: MI_Uint16A,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_Uint16AField {}
impl ::core::clone::Clone for MI_Uint16AField {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_Uint16AField {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_Uint16AField {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_Uint16AField>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_Uint16AField {}
impl ::core::default::Default for MI_Uint16AField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_Uint16Field {
    pub value: u16,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_Uint16Field {}
impl ::core::clone::Clone for MI_Uint16Field {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_Uint16Field {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_Uint16Field {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_Uint16Field>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_Uint16Field {}
impl ::core::default::Default for MI_Uint16Field {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_Uint32A {
    pub data: *mut u32,
    pub size: u32,
}
impl ::core::marker::Copy for MI_Uint32A {}
impl ::core::clone::Clone for MI_Uint32A {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_Uint32A {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_Uint32A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_Uint32A>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_Uint32A {}
impl ::core::default::Default for MI_Uint32A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_Uint32AField {
    pub value: MI_Uint32A,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_Uint32AField {}
impl ::core::clone::Clone for MI_Uint32AField {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_Uint32AField {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_Uint32AField {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_Uint32AField>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_Uint32AField {}
impl ::core::default::Default for MI_Uint32AField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_Uint32Field {
    pub value: u32,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_Uint32Field {}
impl ::core::clone::Clone for MI_Uint32Field {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_Uint32Field {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_Uint32Field {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_Uint32Field>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_Uint32Field {}
impl ::core::default::Default for MI_Uint32Field {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_Uint64A {
    pub data: *mut u64,
    pub size: u32,
}
impl ::core::marker::Copy for MI_Uint64A {}
impl ::core::clone::Clone for MI_Uint64A {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_Uint64A {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_Uint64A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_Uint64A>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_Uint64A {}
impl ::core::default::Default for MI_Uint64A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_Uint64AField {
    pub value: MI_Uint64A,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_Uint64AField {}
impl ::core::clone::Clone for MI_Uint64AField {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_Uint64AField {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_Uint64AField {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_Uint64AField>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_Uint64AField {}
impl ::core::default::Default for MI_Uint64AField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_Uint64Field {
    pub value: u64,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_Uint64Field {}
impl ::core::clone::Clone for MI_Uint64Field {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_Uint64Field {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_Uint64Field {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_Uint64Field>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_Uint64Field {}
impl ::core::default::Default for MI_Uint64Field {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_Uint8A {
    pub data: *mut u8,
    pub size: u32,
}
impl ::core::marker::Copy for MI_Uint8A {}
impl ::core::clone::Clone for MI_Uint8A {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_Uint8A {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_Uint8A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_Uint8A>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_Uint8A {}
impl ::core::default::Default for MI_Uint8A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_Uint8AField {
    pub value: MI_Uint8A,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_Uint8AField {}
impl ::core::clone::Clone for MI_Uint8AField {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_Uint8AField {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_Uint8AField {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_Uint8AField>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_Uint8AField {}
impl ::core::default::Default for MI_Uint8AField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_Uint8Field {
    pub value: u8,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_Uint8Field {}
impl ::core::clone::Clone for MI_Uint8Field {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_Uint8Field {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_Uint8Field {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_Uint8Field>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_Uint8Field {}
impl ::core::default::Default for MI_Uint8Field {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_UserCredentials {
    pub authenticationType: *mut u16,
    pub credentials: MI_UserCredentials_0,
}
impl ::core::marker::Copy for MI_UserCredentials {}
impl ::core::clone::Clone for MI_UserCredentials {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_UserCredentials {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_UserCredentials {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_UserCredentials>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_UserCredentials {}
impl ::core::default::Default for MI_UserCredentials {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub union MI_UserCredentials_0 {
    pub usernamePassword: MI_UsernamePasswordCreds,
    pub certificateThumbprint: *mut u16,
}
impl ::core::marker::Copy for MI_UserCredentials_0 {}
impl ::core::clone::Clone for MI_UserCredentials_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_UserCredentials_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_UserCredentials_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_UserCredentials_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_UserCredentials_0 {}
impl ::core::default::Default for MI_UserCredentials_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_UsernamePasswordCreds {
    pub domain: *mut u16,
    pub username: *mut u16,
    pub password: *mut u16,
}
impl ::core::marker::Copy for MI_UsernamePasswordCreds {}
impl ::core::clone::Clone for MI_UsernamePasswordCreds {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_UsernamePasswordCreds {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_UsernamePasswordCreds {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_UsernamePasswordCreds>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_UsernamePasswordCreds {}
impl ::core::default::Default for MI_UsernamePasswordCreds {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct MI_UtilitiesFT {
    pub MapErrorToMiErrorCategory: isize,
    pub CimErrorFromErrorCode: isize,
}
impl ::core::marker::Copy for MI_UtilitiesFT {}
impl ::core::clone::Clone for MI_UtilitiesFT {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_UtilitiesFT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_UtilitiesFT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_UtilitiesFT>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_UtilitiesFT {}
impl ::core::default::Default for MI_UtilitiesFT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub union MI_Value {
    pub boolean: u8,
    pub uint8: u8,
    pub sint8: i8,
    pub uint16: u16,
    pub sint16: i16,
    pub uint32: u32,
    pub sint32: i32,
    pub uint64: u64,
    pub sint64: i64,
    pub real32: f32,
    pub real64: f64,
    pub char16: u16,
    pub datetime: MI_Datetime,
    pub string: *mut u16,
    pub instance: *mut MI_Instance,
    pub reference: *mut MI_Instance,
    pub booleana: MI_BooleanA,
    pub uint8a: MI_Uint8A,
    pub sint8a: MI_Sint8A,
    pub uint16a: MI_Uint16A,
    pub sint16a: MI_Sint16A,
    pub uint32a: MI_Uint32A,
    pub sint32a: MI_Sint32A,
    pub uint64a: MI_Uint64A,
    pub sint64a: MI_Sint64A,
    pub real32a: MI_Real32A,
    pub real64a: MI_Real64A,
    pub char16a: MI_Char16A,
    pub datetimea: MI_DatetimeA,
    pub stringa: MI_StringA,
    pub referencea: MI_ReferenceA,
    pub instancea: MI_InstanceA,
    pub array: MI_Array,
}
impl ::core::marker::Copy for MI_Value {}
impl ::core::clone::Clone for MI_Value {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MI_Value {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MI_Value {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MI_Value>()) == 0 }
    }
}
impl ::core::cmp::Eq for MI_Value {}
impl ::core::default::Default for MI_Value {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_WRITEMESSAGE_CHANNEL_DEBUG: u32 = 2u32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_WRITEMESSAGE_CHANNEL_VERBOSE: u32 = 1u32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const MI_WRITEMESSAGE_CHANNEL_WARNING: u32 = 0u32;
pub const MofCompiler: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6daf9757_2e37_11d2_aec9_00c04fb68820);
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SWbemAnalysisMatrix {
    pub m_uVersion: u32,
    pub m_uMatrixType: u32,
    pub m_pszProperty: super::super::Foundation::PWSTR,
    pub m_uPropertyType: u32,
    pub m_uEntries: u32,
    pub m_pValues: *mut *mut ::core::ffi::c_void,
    pub m_pbTruthTable: *mut super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SWbemAnalysisMatrix {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SWbemAnalysisMatrix {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SWbemAnalysisMatrix {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SWbemAnalysisMatrix {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SWbemAnalysisMatrix>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SWbemAnalysisMatrix {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SWbemAnalysisMatrix {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SWbemAnalysisMatrixList {
    pub m_uVersion: u32,
    pub m_uMatrixType: u32,
    pub m_uNumMatrices: u32,
    pub m_pMatrices: *mut SWbemAnalysisMatrix,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SWbemAnalysisMatrixList {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SWbemAnalysisMatrixList {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SWbemAnalysisMatrixList {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SWbemAnalysisMatrixList {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SWbemAnalysisMatrixList>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SWbemAnalysisMatrixList {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SWbemAnalysisMatrixList {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SWbemAssocQueryInf {
    pub m_uVersion: u32,
    pub m_uAnalysisType: u32,
    pub m_uFeatureMask: u32,
    pub m_pPath: ::core::option::Option<IWbemPath>,
    pub m_pszPath: super::super::Foundation::PWSTR,
    pub m_pszQueryText: super::super::Foundation::PWSTR,
    pub m_pszResultClass: super::super::Foundation::PWSTR,
    pub m_pszAssocClass: super::super::Foundation::PWSTR,
    pub m_pszRole: super::super::Foundation::PWSTR,
    pub m_pszResultRole: super::super::Foundation::PWSTR,
    pub m_pszRequiredQualifier: super::super::Foundation::PWSTR,
    pub m_pszRequiredAssocQualifier: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SWbemAssocQueryInf {
    fn clone(&self) -> Self {
        Self {
            m_uVersion: self.m_uVersion,
            m_uAnalysisType: self.m_uAnalysisType,
            m_uFeatureMask: self.m_uFeatureMask,
            m_pPath: self.m_pPath.clone(),
            m_pszPath: self.m_pszPath,
            m_pszQueryText: self.m_pszQueryText,
            m_pszResultClass: self.m_pszResultClass,
            m_pszAssocClass: self.m_pszAssocClass,
            m_pszRole: self.m_pszRole,
            m_pszResultRole: self.m_pszResultRole,
            m_pszRequiredQualifier: self.m_pszRequiredQualifier,
            m_pszRequiredAssocQualifier: self.m_pszRequiredAssocQualifier,
        }
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SWbemAssocQueryInf {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SWbemAssocQueryInf {
    fn eq(&self, other: &Self) -> bool {
        self.m_uVersion == other.m_uVersion && self.m_uAnalysisType == other.m_uAnalysisType && self.m_uFeatureMask == other.m_uFeatureMask && self.m_pPath == other.m_pPath && self.m_pszPath == other.m_pszPath && self.m_pszQueryText == other.m_pszQueryText && self.m_pszResultClass == other.m_pszResultClass && self.m_pszAssocClass == other.m_pszAssocClass && self.m_pszRole == other.m_pszRole && self.m_pszResultRole == other.m_pszResultRole && self.m_pszRequiredQualifier == other.m_pszRequiredQualifier && self.m_pszRequiredAssocQualifier == other.m_pszRequiredAssocQualifier
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SWbemAssocQueryInf {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SWbemAssocQueryInf {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const SWbemDateTime: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x47dfbe54_cf76_11d3_b38f_00105a1f473a);
pub const SWbemEventSource: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x04b83d58_21ae_11d2_8b33_00600806d9b6);
pub const SWbemLastError: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc2feeeac_cfcd_11d1_8b05_00600806d9b6);
pub const SWbemLocator: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x76a64158_cb41_11d1_8b02_00600806d9b6);
pub const SWbemMethod: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x04b83d5b_21ae_11d2_8b33_00600806d9b6);
pub const SWbemMethodSet: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x04b83d5a_21ae_11d2_8b33_00600806d9b6);
pub const SWbemNamedValue: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x04b83d60_21ae_11d2_8b33_00600806d9b6);
pub const SWbemNamedValueSet: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9aed384e_ce8b_11d1_8b05_00600806d9b6);
pub const SWbemObject: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x04b83d62_21ae_11d2_8b33_00600806d9b6);
pub const SWbemObjectEx: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd6bdafb2_9435_491f_bb87_6aa0f0bc31a2);
pub const SWbemObjectPath: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5791bc26_ce9c_11d1_97bf_0000f81e849c);
pub const SWbemObjectSet: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x04b83d61_21ae_11d2_8b33_00600806d9b6);
pub const SWbemPrivilege: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x26ee67bc_5804_11d2_8b4a_00600806d9b6);
pub const SWbemPrivilegeSet: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x26ee67be_5804_11d2_8b4a_00600806d9b6);
pub const SWbemProperty: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x04b83d5d_21ae_11d2_8b33_00600806d9b6);
pub const SWbemPropertySet: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x04b83d5c_21ae_11d2_8b33_00600806d9b6);
pub const SWbemQualifier: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x04b83d5f_21ae_11d2_8b33_00600806d9b6);
pub const SWbemQualifierSet: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x04b83d5e_21ae_11d2_8b33_00600806d9b6);
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SWbemQueryQualifiedName {
    pub m_uVersion: u32,
    pub m_uTokenType: u32,
    pub m_uNameListSize: u32,
    pub m_ppszNameList: *mut super::super::Foundation::PWSTR,
    pub m_bArraysUsed: super::super::Foundation::BOOL,
    pub m_pbArrayElUsed: *mut super::super::Foundation::BOOL,
    pub m_puArrayIndex: *mut u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SWbemQueryQualifiedName {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SWbemQueryQualifiedName {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SWbemQueryQualifiedName {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SWbemQueryQualifiedName {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SWbemQueryQualifiedName>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SWbemQueryQualifiedName {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SWbemQueryQualifiedName {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const SWbemRefreshableItem: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8c6854bc_de4b_11d3_b390_00105a1f473a);
pub const SWbemRefresher: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd269bf5c_d9c1_11d3_b38f_00105a1f473a);
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub union SWbemRpnConst {
    pub m_pszStrVal: super::super::Foundation::PWSTR,
    pub m_bBoolVal: super::super::Foundation::BOOL,
    pub m_lLongVal: i32,
    pub m_uLongVal: u32,
    pub m_dblVal: f64,
    pub m_lVal64: i64,
    pub m_uVal64: i64,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SWbemRpnConst {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SWbemRpnConst {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SWbemRpnConst {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SWbemRpnConst {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SWbemRpnConst>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SWbemRpnConst {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SWbemRpnConst {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SWbemRpnEncodedQuery {
    pub m_uVersion: u32,
    pub m_uTokenType: u32,
    pub m_uParsedFeatureMask: u64,
    pub m_uDetectedArraySize: u32,
    pub m_puDetectedFeatures: *mut u32,
    pub m_uSelectListSize: u32,
    pub m_ppSelectList: *mut *mut SWbemQueryQualifiedName,
    pub m_uFromTargetType: u32,
    pub m_pszOptionalFromPath: super::super::Foundation::PWSTR,
    pub m_uFromListSize: u32,
    pub m_ppszFromList: *mut super::super::Foundation::PWSTR,
    pub m_uWhereClauseSize: u32,
    pub m_ppRpnWhereClause: *mut *mut SWbemRpnQueryToken,
    pub m_dblWithinPolling: f64,
    pub m_dblWithinWindow: f64,
    pub m_uOrderByListSize: u32,
    pub m_ppszOrderByList: *mut super::super::Foundation::PWSTR,
    pub m_uOrderDirectionEl: *mut u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SWbemRpnEncodedQuery {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SWbemRpnEncodedQuery {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SWbemRpnEncodedQuery {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SWbemRpnEncodedQuery {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SWbemRpnEncodedQuery>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SWbemRpnEncodedQuery {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SWbemRpnEncodedQuery {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SWbemRpnQueryToken {
    pub m_uVersion: u32,
    pub m_uTokenType: u32,
    pub m_uSubexpressionShape: u32,
    pub m_uOperator: u32,
    pub m_pRightIdent: *mut SWbemQueryQualifiedName,
    pub m_pLeftIdent: *mut SWbemQueryQualifiedName,
    pub m_uConstApparentType: u32,
    pub m_Const: SWbemRpnConst,
    pub m_uConst2ApparentType: u32,
    pub m_Const2: SWbemRpnConst,
    pub m_pszRightFunc: super::super::Foundation::PWSTR,
    pub m_pszLeftFunc: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SWbemRpnQueryToken {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SWbemRpnQueryToken {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SWbemRpnQueryToken {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SWbemRpnQueryToken {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SWbemRpnQueryToken>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SWbemRpnQueryToken {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SWbemRpnQueryToken {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct SWbemRpnTokenList {
    pub m_uVersion: u32,
    pub m_uTokenType: u32,
    pub m_uNumTokens: u32,
}
impl ::core::marker::Copy for SWbemRpnTokenList {}
impl ::core::clone::Clone for SWbemRpnTokenList {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SWbemRpnTokenList {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SWbemRpnTokenList {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SWbemRpnTokenList>()) == 0 }
    }
}
impl ::core::cmp::Eq for SWbemRpnTokenList {}
impl ::core::default::Default for SWbemRpnTokenList {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const SWbemSecurity: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb54d66e9_2287_11d2_8b33_00600806d9b6);
pub const SWbemServices: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x04b83d63_21ae_11d2_8b33_00600806d9b6);
pub const SWbemServicesEx: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x62e522dc_8cf3_40a8_8b2e_37d595651e40);
pub const SWbemSink: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x75718c9a_f029_11d1_a1ac_00c04fb6c223);
pub const UnsecuredApartment: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x49bd2028_1523_11d1_ad79_00c04fd8fdff);
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub type WBEMSTATUS = i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_NO_ERROR: WBEMSTATUS = 0i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_S_NO_ERROR: WBEMSTATUS = 0i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_S_SAME: WBEMSTATUS = 0i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_S_FALSE: WBEMSTATUS = 1i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_S_ALREADY_EXISTS: WBEMSTATUS = 262145i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_S_RESET_TO_DEFAULT: WBEMSTATUS = 262146i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_S_DIFFERENT: WBEMSTATUS = 262147i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_S_TIMEDOUT: WBEMSTATUS = 262148i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_S_NO_MORE_DATA: WBEMSTATUS = 262149i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_S_OPERATION_CANCELLED: WBEMSTATUS = 262150i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_S_PENDING: WBEMSTATUS = 262151i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_S_DUPLICATE_OBJECTS: WBEMSTATUS = 262152i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_S_ACCESS_DENIED: WBEMSTATUS = 262153i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_S_PARTIAL_RESULTS: WBEMSTATUS = 262160i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_S_SOURCE_NOT_AVAILABLE: WBEMSTATUS = 262167i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_E_FAILED: WBEMSTATUS = -2147217407i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_E_NOT_FOUND: WBEMSTATUS = -2147217406i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_E_ACCESS_DENIED: WBEMSTATUS = -2147217405i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_E_PROVIDER_FAILURE: WBEMSTATUS = -2147217404i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_E_TYPE_MISMATCH: WBEMSTATUS = -2147217403i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_E_OUT_OF_MEMORY: WBEMSTATUS = -2147217402i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_E_INVALID_CONTEXT: WBEMSTATUS = -2147217401i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_E_INVALID_PARAMETER: WBEMSTATUS = -2147217400i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_E_NOT_AVAILABLE: WBEMSTATUS = -2147217399i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_E_CRITICAL_ERROR: WBEMSTATUS = -2147217398i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_E_INVALID_STREAM: WBEMSTATUS = -2147217397i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_E_NOT_SUPPORTED: WBEMSTATUS = -2147217396i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_E_INVALID_SUPERCLASS: WBEMSTATUS = -2147217395i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_E_INVALID_NAMESPACE: WBEMSTATUS = -2147217394i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_E_INVALID_OBJECT: WBEMSTATUS = -2147217393i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_E_INVALID_CLASS: WBEMSTATUS = -2147217392i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_E_PROVIDER_NOT_FOUND: WBEMSTATUS = -2147217391i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_E_INVALID_PROVIDER_REGISTRATION: WBEMSTATUS = -2147217390i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_E_PROVIDER_LOAD_FAILURE: WBEMSTATUS = -2147217389i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_E_INITIALIZATION_FAILURE: WBEMSTATUS = -2147217388i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_E_TRANSPORT_FAILURE: WBEMSTATUS = -2147217387i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_E_INVALID_OPERATION: WBEMSTATUS = -2147217386i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_E_INVALID_QUERY: WBEMSTATUS = -2147217385i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_E_INVALID_QUERY_TYPE: WBEMSTATUS = -2147217384i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_E_ALREADY_EXISTS: WBEMSTATUS = -2147217383i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_E_OVERRIDE_NOT_ALLOWED: WBEMSTATUS = -2147217382i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_E_PROPAGATED_QUALIFIER: WBEMSTATUS = -2147217381i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_E_PROPAGATED_PROPERTY: WBEMSTATUS = -2147217380i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_E_UNEXPECTED: WBEMSTATUS = -2147217379i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_E_ILLEGAL_OPERATION: WBEMSTATUS = -2147217378i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_E_CANNOT_BE_KEY: WBEMSTATUS = -2147217377i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_E_INCOMPLETE_CLASS: WBEMSTATUS = -2147217376i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_E_INVALID_SYNTAX: WBEMSTATUS = -2147217375i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_E_NONDECORATED_OBJECT: WBEMSTATUS = -2147217374i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_E_READ_ONLY: WBEMSTATUS = -2147217373i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_E_PROVIDER_NOT_CAPABLE: WBEMSTATUS = -2147217372i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_E_CLASS_HAS_CHILDREN: WBEMSTATUS = -2147217371i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_E_CLASS_HAS_INSTANCES: WBEMSTATUS = -2147217370i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_E_QUERY_NOT_IMPLEMENTED: WBEMSTATUS = -2147217369i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_E_ILLEGAL_NULL: WBEMSTATUS = -2147217368i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_E_INVALID_QUALIFIER_TYPE: WBEMSTATUS = -2147217367i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_E_INVALID_PROPERTY_TYPE: WBEMSTATUS = -2147217366i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_E_VALUE_OUT_OF_RANGE: WBEMSTATUS = -2147217365i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_E_CANNOT_BE_SINGLETON: WBEMSTATUS = -2147217364i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_E_INVALID_CIM_TYPE: WBEMSTATUS = -2147217363i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_E_INVALID_METHOD: WBEMSTATUS = -2147217362i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_E_INVALID_METHOD_PARAMETERS: WBEMSTATUS = -2147217361i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_E_SYSTEM_PROPERTY: WBEMSTATUS = -2147217360i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_E_INVALID_PROPERTY: WBEMSTATUS = -2147217359i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_E_CALL_CANCELLED: WBEMSTATUS = -2147217358i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_E_SHUTTING_DOWN: WBEMSTATUS = -2147217357i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_E_PROPAGATED_METHOD: WBEMSTATUS = -2147217356i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_E_UNSUPPORTED_PARAMETER: WBEMSTATUS = -2147217355i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_E_MISSING_PARAMETER_ID: WBEMSTATUS = -2147217354i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_E_INVALID_PARAMETER_ID: WBEMSTATUS = -2147217353i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_E_NONCONSECUTIVE_PARAMETER_IDS: WBEMSTATUS = -2147217352i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_E_PARAMETER_ID_ON_RETVAL: WBEMSTATUS = -2147217351i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_E_INVALID_OBJECT_PATH: WBEMSTATUS = -2147217350i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_E_OUT_OF_DISK_SPACE: WBEMSTATUS = -2147217349i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_E_BUFFER_TOO_SMALL: WBEMSTATUS = -2147217348i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_E_UNSUPPORTED_PUT_EXTENSION: WBEMSTATUS = -2147217347i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_E_UNKNOWN_OBJECT_TYPE: WBEMSTATUS = -2147217346i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_E_UNKNOWN_PACKET_TYPE: WBEMSTATUS = -2147217345i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_E_MARSHAL_VERSION_MISMATCH: WBEMSTATUS = -2147217344i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_E_MARSHAL_INVALID_SIGNATURE: WBEMSTATUS = -2147217343i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_E_INVALID_QUALIFIER: WBEMSTATUS = -2147217342i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_E_INVALID_DUPLICATE_PARAMETER: WBEMSTATUS = -2147217341i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_E_TOO_MUCH_DATA: WBEMSTATUS = -2147217340i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_E_SERVER_TOO_BUSY: WBEMSTATUS = -2147217339i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_E_INVALID_FLAVOR: WBEMSTATUS = -2147217338i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_E_CIRCULAR_REFERENCE: WBEMSTATUS = -2147217337i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_E_UNSUPPORTED_CLASS_UPDATE: WBEMSTATUS = -2147217336i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_E_CANNOT_CHANGE_KEY_INHERITANCE: WBEMSTATUS = -2147217335i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_E_CANNOT_CHANGE_INDEX_INHERITANCE: WBEMSTATUS = -2147217328i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_E_TOO_MANY_PROPERTIES: WBEMSTATUS = -2147217327i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_E_UPDATE_TYPE_MISMATCH: WBEMSTATUS = -2147217326i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_E_UPDATE_OVERRIDE_NOT_ALLOWED: WBEMSTATUS = -2147217325i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_E_UPDATE_PROPAGATED_METHOD: WBEMSTATUS = -2147217324i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_E_METHOD_NOT_IMPLEMENTED: WBEMSTATUS = -2147217323i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_E_METHOD_DISABLED: WBEMSTATUS = -2147217322i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_E_REFRESHER_BUSY: WBEMSTATUS = -2147217321i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_E_UNPARSABLE_QUERY: WBEMSTATUS = -2147217320i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_E_NOT_EVENT_CLASS: WBEMSTATUS = -2147217319i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_E_MISSING_GROUP_WITHIN: WBEMSTATUS = -2147217318i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_E_MISSING_AGGREGATION_LIST: WBEMSTATUS = -2147217317i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_E_PROPERTY_NOT_AN_OBJECT: WBEMSTATUS = -2147217316i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_E_AGGREGATING_BY_OBJECT: WBEMSTATUS = -2147217315i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_E_UNINTERPRETABLE_PROVIDER_QUERY: WBEMSTATUS = -2147217313i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_E_BACKUP_RESTORE_WINMGMT_RUNNING: WBEMSTATUS = -2147217312i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_E_QUEUE_OVERFLOW: WBEMSTATUS = -2147217311i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_E_PRIVILEGE_NOT_HELD: WBEMSTATUS = -2147217310i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_E_INVALID_OPERATOR: WBEMSTATUS = -2147217309i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_E_LOCAL_CREDENTIALS: WBEMSTATUS = -2147217308i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_E_CANNOT_BE_ABSTRACT: WBEMSTATUS = -2147217307i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_E_AMENDED_OBJECT: WBEMSTATUS = -2147217306i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_E_CLIENT_TOO_SLOW: WBEMSTATUS = -2147217305i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_E_NULL_SECURITY_DESCRIPTOR: WBEMSTATUS = -2147217304i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_E_TIMED_OUT: WBEMSTATUS = -2147217303i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_E_INVALID_ASSOCIATION: WBEMSTATUS = -2147217302i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_E_AMBIGUOUS_OPERATION: WBEMSTATUS = -2147217301i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_E_QUOTA_VIOLATION: WBEMSTATUS = -2147217300i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_E_RESERVED_001: WBEMSTATUS = -2147217299i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_E_RESERVED_002: WBEMSTATUS = -2147217298i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_E_UNSUPPORTED_LOCALE: WBEMSTATUS = -2147217297i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_E_HANDLE_OUT_OF_DATE: WBEMSTATUS = -2147217296i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_E_CONNECTION_FAILED: WBEMSTATUS = -2147217295i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_E_INVALID_HANDLE_REQUEST: WBEMSTATUS = -2147217294i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_E_PROPERTY_NAME_TOO_WIDE: WBEMSTATUS = -2147217293i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_E_CLASS_NAME_TOO_WIDE: WBEMSTATUS = -2147217292i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_E_METHOD_NAME_TOO_WIDE: WBEMSTATUS = -2147217291i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_E_QUALIFIER_NAME_TOO_WIDE: WBEMSTATUS = -2147217290i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_E_RERUN_COMMAND: WBEMSTATUS = -2147217289i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_E_DATABASE_VER_MISMATCH: WBEMSTATUS = -2147217288i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_E_VETO_DELETE: WBEMSTATUS = -2147217287i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_E_VETO_PUT: WBEMSTATUS = -2147217286i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_E_INVALID_LOCALE: WBEMSTATUS = -2147217280i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_E_PROVIDER_SUSPENDED: WBEMSTATUS = -2147217279i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_E_SYNCHRONIZATION_REQUIRED: WBEMSTATUS = -2147217278i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_E_NO_SCHEMA: WBEMSTATUS = -2147217277i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_E_PROVIDER_ALREADY_REGISTERED: WBEMSTATUS = -2147217276i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_E_PROVIDER_NOT_REGISTERED: WBEMSTATUS = -2147217275i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_E_FATAL_TRANSPORT_ERROR: WBEMSTATUS = -2147217274i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_E_ENCRYPTED_CONNECTION_REQUIRED: WBEMSTATUS = -2147217273i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_E_PROVIDER_TIMED_OUT: WBEMSTATUS = -2147217272i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_E_NO_KEY: WBEMSTATUS = -2147217271i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_E_PROVIDER_DISABLED: WBEMSTATUS = -2147217270i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEMESS_E_REGISTRATION_TOO_BROAD: WBEMSTATUS = -2147213311i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEMESS_E_REGISTRATION_TOO_PRECISE: WBEMSTATUS = -2147213310i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEMESS_E_AUTHZ_NOT_PRIVILEGED: WBEMSTATUS = -2147213309i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEMMOF_E_EXPECTED_QUALIFIER_NAME: WBEMSTATUS = -2147205119i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEMMOF_E_EXPECTED_SEMI: WBEMSTATUS = -2147205118i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEMMOF_E_EXPECTED_OPEN_BRACE: WBEMSTATUS = -2147205117i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEMMOF_E_EXPECTED_CLOSE_BRACE: WBEMSTATUS = -2147205116i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEMMOF_E_EXPECTED_CLOSE_BRACKET: WBEMSTATUS = -2147205115i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEMMOF_E_EXPECTED_CLOSE_PAREN: WBEMSTATUS = -2147205114i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEMMOF_E_ILLEGAL_CONSTANT_VALUE: WBEMSTATUS = -2147205113i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEMMOF_E_EXPECTED_TYPE_IDENTIFIER: WBEMSTATUS = -2147205112i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEMMOF_E_EXPECTED_OPEN_PAREN: WBEMSTATUS = -2147205111i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEMMOF_E_UNRECOGNIZED_TOKEN: WBEMSTATUS = -2147205110i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEMMOF_E_UNRECOGNIZED_TYPE: WBEMSTATUS = -2147205109i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEMMOF_E_EXPECTED_PROPERTY_NAME: WBEMSTATUS = -2147205108i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEMMOF_E_TYPEDEF_NOT_SUPPORTED: WBEMSTATUS = -2147205107i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEMMOF_E_UNEXPECTED_ALIAS: WBEMSTATUS = -2147205106i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEMMOF_E_UNEXPECTED_ARRAY_INIT: WBEMSTATUS = -2147205105i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEMMOF_E_INVALID_AMENDMENT_SYNTAX: WBEMSTATUS = -2147205104i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEMMOF_E_INVALID_DUPLICATE_AMENDMENT: WBEMSTATUS = -2147205103i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEMMOF_E_INVALID_PRAGMA: WBEMSTATUS = -2147205102i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEMMOF_E_INVALID_NAMESPACE_SYNTAX: WBEMSTATUS = -2147205101i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEMMOF_E_EXPECTED_CLASS_NAME: WBEMSTATUS = -2147205100i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEMMOF_E_TYPE_MISMATCH: WBEMSTATUS = -2147205099i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEMMOF_E_EXPECTED_ALIAS_NAME: WBEMSTATUS = -2147205098i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEMMOF_E_INVALID_CLASS_DECLARATION: WBEMSTATUS = -2147205097i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEMMOF_E_INVALID_INSTANCE_DECLARATION: WBEMSTATUS = -2147205096i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEMMOF_E_EXPECTED_DOLLAR: WBEMSTATUS = -2147205095i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEMMOF_E_CIMTYPE_QUALIFIER: WBEMSTATUS = -2147205094i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEMMOF_E_DUPLICATE_PROPERTY: WBEMSTATUS = -2147205093i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEMMOF_E_INVALID_NAMESPACE_SPECIFICATION: WBEMSTATUS = -2147205092i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEMMOF_E_OUT_OF_RANGE: WBEMSTATUS = -2147205091i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEMMOF_E_INVALID_FILE: WBEMSTATUS = -2147205090i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEMMOF_E_ALIASES_IN_EMBEDDED: WBEMSTATUS = -2147205089i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEMMOF_E_NULL_ARRAY_ELEM: WBEMSTATUS = -2147205088i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEMMOF_E_DUPLICATE_QUALIFIER: WBEMSTATUS = -2147205087i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEMMOF_E_EXPECTED_FLAVOR_TYPE: WBEMSTATUS = -2147205086i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEMMOF_E_INCOMPATIBLE_FLAVOR_TYPES: WBEMSTATUS = -2147205085i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEMMOF_E_MULTIPLE_ALIASES: WBEMSTATUS = -2147205084i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEMMOF_E_INCOMPATIBLE_FLAVOR_TYPES2: WBEMSTATUS = -2147205083i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEMMOF_E_NO_ARRAYS_RETURNED: WBEMSTATUS = -2147205082i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEMMOF_E_MUST_BE_IN_OR_OUT: WBEMSTATUS = -2147205081i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEMMOF_E_INVALID_FLAGS_SYNTAX: WBEMSTATUS = -2147205080i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEMMOF_E_EXPECTED_BRACE_OR_BAD_TYPE: WBEMSTATUS = -2147205079i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEMMOF_E_UNSUPPORTED_CIMV22_QUAL_VALUE: WBEMSTATUS = -2147205078i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEMMOF_E_UNSUPPORTED_CIMV22_DATA_TYPE: WBEMSTATUS = -2147205077i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEMMOF_E_INVALID_DELETEINSTANCE_SYNTAX: WBEMSTATUS = -2147205076i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEMMOF_E_INVALID_QUALIFIER_SYNTAX: WBEMSTATUS = -2147205075i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEMMOF_E_QUALIFIER_USED_OUTSIDE_SCOPE: WBEMSTATUS = -2147205074i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEMMOF_E_ERROR_CREATING_TEMP_FILE: WBEMSTATUS = -2147205073i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEMMOF_E_ERROR_INVALID_INCLUDE_FILE: WBEMSTATUS = -2147205072i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEMMOF_E_INVALID_DELETECLASS_SYNTAX: WBEMSTATUS = -2147205071i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub type WBEMSTATUS_FORMAT = i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEMSTATUS_FORMAT_NEWLINE: WBEMSTATUS_FORMAT = 0i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEMSTATUS_FORMAT_NO_NEWLINE: WBEMSTATUS_FORMAT = 1i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEMS_DISPID_COMPLETED: u32 = 2u32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEMS_DISPID_CONNECTION_READY: u32 = 5u32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEMS_DISPID_DERIVATION: u32 = 23u32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEMS_DISPID_OBJECT_PUT: u32 = 4u32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEMS_DISPID_OBJECT_READY: u32 = 1u32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEMS_DISPID_PROGRESS: u32 = 3u32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub type WBEM_BACKUP_RESTORE_FLAGS = i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_FLAG_BACKUP_RESTORE_DEFAULT: WBEM_BACKUP_RESTORE_FLAGS = 0i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_FLAG_BACKUP_RESTORE_FORCE_SHUTDOWN: WBEM_BACKUP_RESTORE_FLAGS = 1i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub type WBEM_BATCH_TYPE = i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_FLAG_BATCH_IF_NEEDED: WBEM_BATCH_TYPE = 0i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_FLAG_MUST_BATCH: WBEM_BATCH_TYPE = 1i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_FLAG_MUST_NOT_BATCH: WBEM_BATCH_TYPE = 2i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub type WBEM_CHANGE_FLAG_TYPE = i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_FLAG_CREATE_OR_UPDATE: WBEM_CHANGE_FLAG_TYPE = 0i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_FLAG_UPDATE_ONLY: WBEM_CHANGE_FLAG_TYPE = 1i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_FLAG_CREATE_ONLY: WBEM_CHANGE_FLAG_TYPE = 2i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_FLAG_UPDATE_COMPATIBLE: WBEM_CHANGE_FLAG_TYPE = 0i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_FLAG_UPDATE_SAFE_MODE: WBEM_CHANGE_FLAG_TYPE = 32i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_FLAG_UPDATE_FORCE_MODE: WBEM_CHANGE_FLAG_TYPE = 64i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_MASK_UPDATE_MODE: WBEM_CHANGE_FLAG_TYPE = 96i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_FLAG_ADVISORY: WBEM_CHANGE_FLAG_TYPE = 65536i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub type WBEM_COMPARISON_FLAG = i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_COMPARISON_INCLUDE_ALL: WBEM_COMPARISON_FLAG = 0i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_FLAG_IGNORE_QUALIFIERS: WBEM_COMPARISON_FLAG = 1i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_FLAG_IGNORE_OBJECT_SOURCE: WBEM_COMPARISON_FLAG = 2i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_FLAG_IGNORE_DEFAULT_VALUES: WBEM_COMPARISON_FLAG = 4i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_FLAG_IGNORE_CLASS: WBEM_COMPARISON_FLAG = 8i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_FLAG_IGNORE_CASE: WBEM_COMPARISON_FLAG = 16i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_FLAG_IGNORE_FLAVOR: WBEM_COMPARISON_FLAG = 32i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub type WBEM_COMPILER_OPTIONS = i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_FLAG_CHECK_ONLY: WBEM_COMPILER_OPTIONS = 1i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_FLAG_AUTORECOVER: WBEM_COMPILER_OPTIONS = 2i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_FLAG_WMI_CHECK: WBEM_COMPILER_OPTIONS = 4i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_FLAG_CONSOLE_PRINT: WBEM_COMPILER_OPTIONS = 8i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_FLAG_DONT_ADD_TO_LIST: WBEM_COMPILER_OPTIONS = 16i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_FLAG_SPLIT_FILES: WBEM_COMPILER_OPTIONS = 32i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_FLAG_STORE_FILE: WBEM_COMPILER_OPTIONS = 256i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub struct WBEM_COMPILE_STATUS_INFO {
    pub lPhaseError: i32,
    pub hRes: ::windows::core::HRESULT,
    pub ObjectNum: i32,
    pub FirstLine: i32,
    pub LastLine: i32,
    pub dwOutFlags: u32,
}
impl ::core::marker::Copy for WBEM_COMPILE_STATUS_INFO {}
impl ::core::clone::Clone for WBEM_COMPILE_STATUS_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WBEM_COMPILE_STATUS_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WBEM_COMPILE_STATUS_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WBEM_COMPILE_STATUS_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for WBEM_COMPILE_STATUS_INFO {}
impl ::core::default::Default for WBEM_COMPILE_STATUS_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub type WBEM_CONDITION_FLAG_TYPE = i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_FLAG_ALWAYS: WBEM_CONDITION_FLAG_TYPE = 0i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_FLAG_ONLY_IF_TRUE: WBEM_CONDITION_FLAG_TYPE = 1i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_FLAG_ONLY_IF_FALSE: WBEM_CONDITION_FLAG_TYPE = 2i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_FLAG_ONLY_IF_IDENTICAL: WBEM_CONDITION_FLAG_TYPE = 3i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_MASK_PRIMARY_CONDITION: WBEM_CONDITION_FLAG_TYPE = 3i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_FLAG_KEYS_ONLY: WBEM_CONDITION_FLAG_TYPE = 4i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_FLAG_REFS_ONLY: WBEM_CONDITION_FLAG_TYPE = 8i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_FLAG_LOCAL_ONLY: WBEM_CONDITION_FLAG_TYPE = 16i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_FLAG_PROPAGATED_ONLY: WBEM_CONDITION_FLAG_TYPE = 32i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_FLAG_SYSTEM_ONLY: WBEM_CONDITION_FLAG_TYPE = 48i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_FLAG_NONSYSTEM_ONLY: WBEM_CONDITION_FLAG_TYPE = 64i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_MASK_CONDITION_ORIGIN: WBEM_CONDITION_FLAG_TYPE = 112i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_FLAG_CLASS_OVERRIDES_ONLY: WBEM_CONDITION_FLAG_TYPE = 256i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_FLAG_CLASS_LOCAL_AND_OVERRIDES: WBEM_CONDITION_FLAG_TYPE = 512i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_MASK_CLASS_CONDITION: WBEM_CONDITION_FLAG_TYPE = 768i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub type WBEM_CONNECT_OPTIONS = i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_FLAG_CONNECT_REPOSITORY_ONLY: WBEM_CONNECT_OPTIONS = 64i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_FLAG_CONNECT_USE_MAX_WAIT: WBEM_CONNECT_OPTIONS = 128i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_FLAG_CONNECT_PROVIDERS: WBEM_CONNECT_OPTIONS = 256i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub type WBEM_EXTRA_RETURN_CODES = i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_S_INITIALIZED: WBEM_EXTRA_RETURN_CODES = 0i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_S_LIMITED_SERVICE: WBEM_EXTRA_RETURN_CODES = 274433i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_S_INDIRECTLY_UPDATED: WBEM_EXTRA_RETURN_CODES = 274434i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_S_SUBJECT_TO_SDS: WBEM_EXTRA_RETURN_CODES = 274435i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_E_RETRY_LATER: WBEM_EXTRA_RETURN_CODES = -2147209215i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_E_RESOURCE_CONTENTION: WBEM_EXTRA_RETURN_CODES = -2147209214i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub type WBEM_FLAVOR_TYPE = i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_FLAVOR_DONT_PROPAGATE: WBEM_FLAVOR_TYPE = 0i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_FLAVOR_FLAG_PROPAGATE_TO_INSTANCE: WBEM_FLAVOR_TYPE = 1i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_FLAVOR_FLAG_PROPAGATE_TO_DERIVED_CLASS: WBEM_FLAVOR_TYPE = 2i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_FLAVOR_MASK_PROPAGATION: WBEM_FLAVOR_TYPE = 15i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_FLAVOR_OVERRIDABLE: WBEM_FLAVOR_TYPE = 0i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_FLAVOR_NOT_OVERRIDABLE: WBEM_FLAVOR_TYPE = 16i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_FLAVOR_MASK_PERMISSIONS: WBEM_FLAVOR_TYPE = 16i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_FLAVOR_ORIGIN_LOCAL: WBEM_FLAVOR_TYPE = 0i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_FLAVOR_ORIGIN_PROPAGATED: WBEM_FLAVOR_TYPE = 32i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_FLAVOR_ORIGIN_SYSTEM: WBEM_FLAVOR_TYPE = 64i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_FLAVOR_MASK_ORIGIN: WBEM_FLAVOR_TYPE = 96i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_FLAVOR_NOT_AMENDED: WBEM_FLAVOR_TYPE = 0i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_FLAVOR_AMENDED: WBEM_FLAVOR_TYPE = 128i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_FLAVOR_MASK_AMENDED: WBEM_FLAVOR_TYPE = 128i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub type WBEM_GENERIC_FLAG_TYPE = i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_FLAG_RETURN_IMMEDIATELY: WBEM_GENERIC_FLAG_TYPE = 16i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_FLAG_RETURN_WBEM_COMPLETE: WBEM_GENERIC_FLAG_TYPE = 0i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_FLAG_BIDIRECTIONAL: WBEM_GENERIC_FLAG_TYPE = 0i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_FLAG_FORWARD_ONLY: WBEM_GENERIC_FLAG_TYPE = 32i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_FLAG_NO_ERROR_OBJECT: WBEM_GENERIC_FLAG_TYPE = 64i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_FLAG_RETURN_ERROR_OBJECT: WBEM_GENERIC_FLAG_TYPE = 0i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_FLAG_SEND_STATUS: WBEM_GENERIC_FLAG_TYPE = 128i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_FLAG_DONT_SEND_STATUS: WBEM_GENERIC_FLAG_TYPE = 0i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_FLAG_ENSURE_LOCATABLE: WBEM_GENERIC_FLAG_TYPE = 256i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_FLAG_DIRECT_READ: WBEM_GENERIC_FLAG_TYPE = 512i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_FLAG_SEND_ONLY_SELECTED: WBEM_GENERIC_FLAG_TYPE = 0i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_RETURN_WHEN_COMPLETE: WBEM_GENERIC_FLAG_TYPE = 0i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_RETURN_IMMEDIATELY: WBEM_GENERIC_FLAG_TYPE = 16i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_MASK_RESERVED_FLAGS: WBEM_GENERIC_FLAG_TYPE = 126976i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_FLAG_USE_AMENDED_QUALIFIERS: WBEM_GENERIC_FLAG_TYPE = 131072i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_FLAG_STRONG_VALIDATION: WBEM_GENERIC_FLAG_TYPE = 1048576i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub type WBEM_GENUS_TYPE = i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_GENUS_CLASS: WBEM_GENUS_TYPE = 1i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_GENUS_INSTANCE: WBEM_GENUS_TYPE = 2i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub type WBEM_GET_KEY_FLAGS = i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEMPATH_TEXT: WBEM_GET_KEY_FLAGS = 1i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEMPATH_QUOTEDTEXT: WBEM_GET_KEY_FLAGS = 2i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub type WBEM_GET_TEXT_FLAGS = i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEMPATH_COMPRESSED: WBEM_GET_TEXT_FLAGS = 1i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEMPATH_GET_RELATIVE_ONLY: WBEM_GET_TEXT_FLAGS = 2i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEMPATH_GET_SERVER_TOO: WBEM_GET_TEXT_FLAGS = 4i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEMPATH_GET_SERVER_AND_NAMESPACE_ONLY: WBEM_GET_TEXT_FLAGS = 8i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEMPATH_GET_NAMESPACE_ONLY: WBEM_GET_TEXT_FLAGS = 16i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEMPATH_GET_ORIGINAL: WBEM_GET_TEXT_FLAGS = 32i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub type WBEM_INFORMATION_FLAG_TYPE = i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_FLAG_SHORT_NAME: WBEM_INFORMATION_FLAG_TYPE = 1i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_FLAG_LONG_NAME: WBEM_INFORMATION_FLAG_TYPE = 2i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub type WBEM_LIMITATION_FLAG_TYPE = i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_FLAG_EXCLUDE_OBJECT_QUALIFIERS: WBEM_LIMITATION_FLAG_TYPE = 16i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_FLAG_EXCLUDE_PROPERTY_QUALIFIERS: WBEM_LIMITATION_FLAG_TYPE = 32i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub type WBEM_LIMITS = i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_MAX_IDENTIFIER: WBEM_LIMITS = 4096i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_MAX_QUERY: WBEM_LIMITS = 16384i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_MAX_PATH: WBEM_LIMITS = 8192i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_MAX_OBJECT_NESTING: WBEM_LIMITS = 64i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_MAX_USER_PROPERTIES: WBEM_LIMITS = 1024i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub type WBEM_LOCKING = i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_FLAG_ALLOW_READ: WBEM_LOCKING = 1i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub type WBEM_PATH_CREATE_FLAG = i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEMPATH_CREATE_ACCEPT_RELATIVE: WBEM_PATH_CREATE_FLAG = 1i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEMPATH_CREATE_ACCEPT_ABSOLUTE: WBEM_PATH_CREATE_FLAG = 2i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEMPATH_CREATE_ACCEPT_ALL: WBEM_PATH_CREATE_FLAG = 4i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEMPATH_TREAT_SINGLE_IDENT_AS_NS: WBEM_PATH_CREATE_FLAG = 8i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub type WBEM_PATH_STATUS_FLAG = i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEMPATH_INFO_ANON_LOCAL_MACHINE: WBEM_PATH_STATUS_FLAG = 1i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEMPATH_INFO_HAS_MACHINE_NAME: WBEM_PATH_STATUS_FLAG = 2i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEMPATH_INFO_IS_CLASS_REF: WBEM_PATH_STATUS_FLAG = 4i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEMPATH_INFO_IS_INST_REF: WBEM_PATH_STATUS_FLAG = 8i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEMPATH_INFO_HAS_SUBSCOPES: WBEM_PATH_STATUS_FLAG = 16i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEMPATH_INFO_IS_COMPOUND: WBEM_PATH_STATUS_FLAG = 32i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEMPATH_INFO_HAS_V2_REF_PATHS: WBEM_PATH_STATUS_FLAG = 64i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEMPATH_INFO_HAS_IMPLIED_KEY: WBEM_PATH_STATUS_FLAG = 128i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEMPATH_INFO_CONTAINS_SINGLETON: WBEM_PATH_STATUS_FLAG = 256i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEMPATH_INFO_V1_COMPLIANT: WBEM_PATH_STATUS_FLAG = 512i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEMPATH_INFO_V2_COMPLIANT: WBEM_PATH_STATUS_FLAG = 1024i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEMPATH_INFO_CIM_COMPLIANT: WBEM_PATH_STATUS_FLAG = 2048i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEMPATH_INFO_IS_SINGLETON: WBEM_PATH_STATUS_FLAG = 4096i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEMPATH_INFO_IS_PARENT: WBEM_PATH_STATUS_FLAG = 8192i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEMPATH_INFO_SERVER_NAMESPACE_ONLY: WBEM_PATH_STATUS_FLAG = 16384i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEMPATH_INFO_NATIVE_PATH: WBEM_PATH_STATUS_FLAG = 32768i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEMPATH_INFO_WMI_PATH: WBEM_PATH_STATUS_FLAG = 65536i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEMPATH_INFO_PATH_HAD_SERVER: WBEM_PATH_STATUS_FLAG = 131072i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub type WBEM_PROVIDER_FLAGS = i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_FLAG_OWNER_UPDATE: WBEM_PROVIDER_FLAGS = 65536i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub type WBEM_PROVIDER_REQUIREMENTS_TYPE = i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_REQUIREMENTS_START_POSTFILTER: WBEM_PROVIDER_REQUIREMENTS_TYPE = 0i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_REQUIREMENTS_STOP_POSTFILTER: WBEM_PROVIDER_REQUIREMENTS_TYPE = 1i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_REQUIREMENTS_RECHECK_SUBSCRIPTIONS: WBEM_PROVIDER_REQUIREMENTS_TYPE = 2i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub type WBEM_QUERY_FLAG_TYPE = i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_FLAG_DEEP: WBEM_QUERY_FLAG_TYPE = 0i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_FLAG_SHALLOW: WBEM_QUERY_FLAG_TYPE = 1i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_FLAG_PROTOTYPE: WBEM_QUERY_FLAG_TYPE = 2i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub type WBEM_REFRESHER_FLAGS = i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_FLAG_REFRESH_AUTO_RECONNECT: WBEM_REFRESHER_FLAGS = 0i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_FLAG_REFRESH_NO_AUTO_RECONNECT: WBEM_REFRESHER_FLAGS = 1i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub type WBEM_SECURITY_FLAGS = i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_ENABLE: WBEM_SECURITY_FLAGS = 1i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_METHOD_EXECUTE: WBEM_SECURITY_FLAGS = 2i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_FULL_WRITE_REP: WBEM_SECURITY_FLAGS = 4i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_PARTIAL_WRITE_REP: WBEM_SECURITY_FLAGS = 8i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_WRITE_PROVIDER: WBEM_SECURITY_FLAGS = 16i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_REMOTE_ACCESS: WBEM_SECURITY_FLAGS = 32i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_RIGHT_SUBSCRIBE: WBEM_SECURITY_FLAGS = 64i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_RIGHT_PUBLISH: WBEM_SECURITY_FLAGS = 128i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub type WBEM_SHUTDOWN_FLAGS = i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_SHUTDOWN_UNLOAD_COMPONENT: WBEM_SHUTDOWN_FLAGS = 1i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_SHUTDOWN_WMI: WBEM_SHUTDOWN_FLAGS = 2i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_SHUTDOWN_OS: WBEM_SHUTDOWN_FLAGS = 3i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub type WBEM_STATUS_TYPE = i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_STATUS_COMPLETE: WBEM_STATUS_TYPE = 0i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_STATUS_REQUIREMENTS: WBEM_STATUS_TYPE = 1i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_STATUS_PROGRESS: WBEM_STATUS_TYPE = 2i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_STATUS_LOGGING_INFORMATION: WBEM_STATUS_TYPE = 256i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_STATUS_LOGGING_INFORMATION_PROVIDER: WBEM_STATUS_TYPE = 512i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_STATUS_LOGGING_INFORMATION_HOST: WBEM_STATUS_TYPE = 1024i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_STATUS_LOGGING_INFORMATION_REPOSITORY: WBEM_STATUS_TYPE = 2048i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_STATUS_LOGGING_INFORMATION_ESS: WBEM_STATUS_TYPE = 4096i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub type WBEM_TEXT_FLAG_TYPE = i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_FLAG_NO_FLAVORS: WBEM_TEXT_FLAG_TYPE = 1i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub type WBEM_TIMEOUT_TYPE = i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_NO_WAIT: WBEM_TIMEOUT_TYPE = 0i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_INFINITE: WBEM_TIMEOUT_TYPE = -1i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub type WBEM_UNSECAPP_FLAG_TYPE = i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_FLAG_UNSECAPP_DEFAULT_CHECK_ACCESS: WBEM_UNSECAPP_FLAG_TYPE = 0i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_FLAG_UNSECAPP_CHECK_ACCESS: WBEM_UNSECAPP_FLAG_TYPE = 1i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_FLAG_UNSECAPP_DONT_CHECK_ACCESS: WBEM_UNSECAPP_FLAG_TYPE = 2i32;
pub const WMIExtension: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf0975afe_5c7f_11d2_8b74_00104b2afb41);
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub type WMIQ_ANALYSIS_TYPE = i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WMIQ_ANALYSIS_RPN_SEQUENCE: WMIQ_ANALYSIS_TYPE = 1i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WMIQ_ANALYSIS_ASSOC_QUERY: WMIQ_ANALYSIS_TYPE = 2i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WMIQ_ANALYSIS_PROP_ANALYSIS_MATRIX: WMIQ_ANALYSIS_TYPE = 3i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WMIQ_ANALYSIS_QUERY_TEXT: WMIQ_ANALYSIS_TYPE = 4i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WMIQ_ANALYSIS_RESERVED: WMIQ_ANALYSIS_TYPE = 134217728i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub type WMIQ_ASSOCQ_FLAGS = i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WMIQ_ASSOCQ_ASSOCIATORS: WMIQ_ASSOCQ_FLAGS = 1i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WMIQ_ASSOCQ_REFERENCES: WMIQ_ASSOCQ_FLAGS = 2i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WMIQ_ASSOCQ_RESULTCLASS: WMIQ_ASSOCQ_FLAGS = 4i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WMIQ_ASSOCQ_ASSOCCLASS: WMIQ_ASSOCQ_FLAGS = 8i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WMIQ_ASSOCQ_ROLE: WMIQ_ASSOCQ_FLAGS = 16i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WMIQ_ASSOCQ_RESULTROLE: WMIQ_ASSOCQ_FLAGS = 32i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WMIQ_ASSOCQ_REQUIREDQUALIFIER: WMIQ_ASSOCQ_FLAGS = 64i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WMIQ_ASSOCQ_REQUIREDASSOCQUALIFIER: WMIQ_ASSOCQ_FLAGS = 128i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WMIQ_ASSOCQ_CLASSDEFSONLY: WMIQ_ASSOCQ_FLAGS = 256i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WMIQ_ASSOCQ_KEYSONLY: WMIQ_ASSOCQ_FLAGS = 512i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WMIQ_ASSOCQ_SCHEMAONLY: WMIQ_ASSOCQ_FLAGS = 1024i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WMIQ_ASSOCQ_CLASSREFSONLY: WMIQ_ASSOCQ_FLAGS = 2048i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub type WMIQ_LANGUAGE_FEATURES = i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WMIQ_LF1_BASIC_SELECT: WMIQ_LANGUAGE_FEATURES = 1i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WMIQ_LF2_CLASS_NAME_IN_QUERY: WMIQ_LANGUAGE_FEATURES = 2i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WMIQ_LF3_STRING_CASE_FUNCTIONS: WMIQ_LANGUAGE_FEATURES = 3i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WMIQ_LF4_PROP_TO_PROP_TESTS: WMIQ_LANGUAGE_FEATURES = 4i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WMIQ_LF5_COUNT_STAR: WMIQ_LANGUAGE_FEATURES = 5i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WMIQ_LF6_ORDER_BY: WMIQ_LANGUAGE_FEATURES = 6i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WMIQ_LF7_DISTINCT: WMIQ_LANGUAGE_FEATURES = 7i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WMIQ_LF8_ISA: WMIQ_LANGUAGE_FEATURES = 8i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WMIQ_LF9_THIS: WMIQ_LANGUAGE_FEATURES = 9i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WMIQ_LF10_COMPEX_SUBEXPRESSIONS: WMIQ_LANGUAGE_FEATURES = 10i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WMIQ_LF11_ALIASING: WMIQ_LANGUAGE_FEATURES = 11i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WMIQ_LF12_GROUP_BY_HAVING: WMIQ_LANGUAGE_FEATURES = 12i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WMIQ_LF13_WMI_WITHIN: WMIQ_LANGUAGE_FEATURES = 13i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WMIQ_LF14_SQL_WRITE_OPERATIONS: WMIQ_LANGUAGE_FEATURES = 14i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WMIQ_LF15_GO: WMIQ_LANGUAGE_FEATURES = 15i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WMIQ_LF16_SINGLE_LEVEL_TRANSACTIONS: WMIQ_LANGUAGE_FEATURES = 16i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WMIQ_LF17_QUALIFIED_NAMES: WMIQ_LANGUAGE_FEATURES = 17i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WMIQ_LF18_ASSOCIATONS: WMIQ_LANGUAGE_FEATURES = 18i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WMIQ_LF19_SYSTEM_PROPERTIES: WMIQ_LANGUAGE_FEATURES = 19i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WMIQ_LF20_EXTENDED_SYSTEM_PROPERTIES: WMIQ_LANGUAGE_FEATURES = 20i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WMIQ_LF21_SQL89_JOINS: WMIQ_LANGUAGE_FEATURES = 21i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WMIQ_LF22_SQL92_JOINS: WMIQ_LANGUAGE_FEATURES = 22i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WMIQ_LF23_SUBSELECTS: WMIQ_LANGUAGE_FEATURES = 23i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WMIQ_LF24_UMI_EXTENSIONS: WMIQ_LANGUAGE_FEATURES = 24i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WMIQ_LF25_DATEPART: WMIQ_LANGUAGE_FEATURES = 25i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WMIQ_LF26_LIKE: WMIQ_LANGUAGE_FEATURES = 26i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WMIQ_LF27_CIM_TEMPORAL_CONSTRUCTS: WMIQ_LANGUAGE_FEATURES = 27i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WMIQ_LF28_STANDARD_AGGREGATES: WMIQ_LANGUAGE_FEATURES = 28i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WMIQ_LF29_MULTI_LEVEL_ORDER_BY: WMIQ_LANGUAGE_FEATURES = 29i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WMIQ_LF30_WMI_PRAGMAS: WMIQ_LANGUAGE_FEATURES = 30i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WMIQ_LF31_QUALIFIER_TESTS: WMIQ_LANGUAGE_FEATURES = 31i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WMIQ_LF32_SP_EXECUTE: WMIQ_LANGUAGE_FEATURES = 32i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WMIQ_LF33_ARRAY_ACCESS: WMIQ_LANGUAGE_FEATURES = 33i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WMIQ_LF34_UNION: WMIQ_LANGUAGE_FEATURES = 34i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WMIQ_LF35_COMPLEX_SELECT_TARGET: WMIQ_LANGUAGE_FEATURES = 35i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WMIQ_LF36_REFERENCE_TESTS: WMIQ_LANGUAGE_FEATURES = 36i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WMIQ_LF37_SELECT_INTO: WMIQ_LANGUAGE_FEATURES = 37i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WMIQ_LF38_BASIC_DATETIME_TESTS: WMIQ_LANGUAGE_FEATURES = 38i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WMIQ_LF39_COUNT_COLUMN: WMIQ_LANGUAGE_FEATURES = 39i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WMIQ_LF40_BETWEEN: WMIQ_LANGUAGE_FEATURES = 40i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WMIQ_LF_LAST: WMIQ_LANGUAGE_FEATURES = 40i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub type WMIQ_RPNQ_FEATURE = i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WMIQ_RPNF_WHERE_CLAUSE_PRESENT: WMIQ_RPNQ_FEATURE = 1i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WMIQ_RPNF_QUERY_IS_CONJUNCTIVE: WMIQ_RPNQ_FEATURE = 2i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WMIQ_RPNF_QUERY_IS_DISJUNCTIVE: WMIQ_RPNQ_FEATURE = 4i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WMIQ_RPNF_PROJECTION: WMIQ_RPNQ_FEATURE = 8i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WMIQ_RPNF_FEATURE_SELECT_STAR: WMIQ_RPNQ_FEATURE = 16i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WMIQ_RPNF_EQUALITY_TESTS_ONLY: WMIQ_RPNQ_FEATURE = 32i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WMIQ_RPNF_COUNT_STAR: WMIQ_RPNQ_FEATURE = 64i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WMIQ_RPNF_QUALIFIED_NAMES_USED: WMIQ_RPNQ_FEATURE = 128i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WMIQ_RPNF_SYSPROP_CLASS_USED: WMIQ_RPNQ_FEATURE = 256i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WMIQ_RPNF_PROP_TO_PROP_TESTS: WMIQ_RPNQ_FEATURE = 512i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WMIQ_RPNF_ORDER_BY: WMIQ_RPNQ_FEATURE = 1024i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WMIQ_RPNF_ISA_USED: WMIQ_RPNQ_FEATURE = 2048i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WMIQ_RPNF_GROUP_BY_HAVING: WMIQ_RPNQ_FEATURE = 4096i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WMIQ_RPNF_ARRAY_ACCESS_USED: WMIQ_RPNQ_FEATURE = 8192i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub type WMIQ_RPN_TOKEN_FLAGS = i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WMIQ_RPN_TOKEN_EXPRESSION: WMIQ_RPN_TOKEN_FLAGS = 1i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WMIQ_RPN_TOKEN_AND: WMIQ_RPN_TOKEN_FLAGS = 2i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WMIQ_RPN_TOKEN_OR: WMIQ_RPN_TOKEN_FLAGS = 3i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WMIQ_RPN_TOKEN_NOT: WMIQ_RPN_TOKEN_FLAGS = 4i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WMIQ_RPN_OP_UNDEFINED: WMIQ_RPN_TOKEN_FLAGS = 0i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WMIQ_RPN_OP_EQ: WMIQ_RPN_TOKEN_FLAGS = 1i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WMIQ_RPN_OP_NE: WMIQ_RPN_TOKEN_FLAGS = 2i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WMIQ_RPN_OP_GE: WMIQ_RPN_TOKEN_FLAGS = 3i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WMIQ_RPN_OP_LE: WMIQ_RPN_TOKEN_FLAGS = 4i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WMIQ_RPN_OP_LT: WMIQ_RPN_TOKEN_FLAGS = 5i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WMIQ_RPN_OP_GT: WMIQ_RPN_TOKEN_FLAGS = 6i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WMIQ_RPN_OP_LIKE: WMIQ_RPN_TOKEN_FLAGS = 7i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WMIQ_RPN_OP_ISA: WMIQ_RPN_TOKEN_FLAGS = 8i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WMIQ_RPN_OP_ISNOTA: WMIQ_RPN_TOKEN_FLAGS = 9i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WMIQ_RPN_OP_ISNULL: WMIQ_RPN_TOKEN_FLAGS = 10i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WMIQ_RPN_OP_ISNOTNULL: WMIQ_RPN_TOKEN_FLAGS = 11i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WMIQ_RPN_LEFT_PROPERTY_NAME: WMIQ_RPN_TOKEN_FLAGS = 1i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WMIQ_RPN_RIGHT_PROPERTY_NAME: WMIQ_RPN_TOKEN_FLAGS = 2i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WMIQ_RPN_CONST2: WMIQ_RPN_TOKEN_FLAGS = 4i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WMIQ_RPN_CONST: WMIQ_RPN_TOKEN_FLAGS = 8i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WMIQ_RPN_RELOP: WMIQ_RPN_TOKEN_FLAGS = 16i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WMIQ_RPN_LEFT_FUNCTION: WMIQ_RPN_TOKEN_FLAGS = 32i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WMIQ_RPN_RIGHT_FUNCTION: WMIQ_RPN_TOKEN_FLAGS = 64i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WMIQ_RPN_GET_TOKEN_TYPE: WMIQ_RPN_TOKEN_FLAGS = 1i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WMIQ_RPN_GET_EXPR_SHAPE: WMIQ_RPN_TOKEN_FLAGS = 2i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WMIQ_RPN_GET_LEFT_FUNCTION: WMIQ_RPN_TOKEN_FLAGS = 3i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WMIQ_RPN_GET_RIGHT_FUNCTION: WMIQ_RPN_TOKEN_FLAGS = 4i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WMIQ_RPN_GET_RELOP: WMIQ_RPN_TOKEN_FLAGS = 5i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WMIQ_RPN_NEXT_TOKEN: WMIQ_RPN_TOKEN_FLAGS = 1i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WMIQ_RPN_FROM_UNARY: WMIQ_RPN_TOKEN_FLAGS = 1i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WMIQ_RPN_FROM_PATH: WMIQ_RPN_TOKEN_FLAGS = 2i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WMIQ_RPN_FROM_CLASS_LIST: WMIQ_RPN_TOKEN_FLAGS = 4i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WMIQ_RPN_FROM_MULTIPLE: WMIQ_RPN_TOKEN_FLAGS = 8i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub type WMI_OBJ_TEXT = i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WMI_OBJ_TEXT_CIM_DTD_2_0: WMI_OBJ_TEXT = 1i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WMI_OBJ_TEXT_WMI_DTD_2_0: WMI_OBJ_TEXT = 2i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WMI_OBJ_TEXT_WMI_EXT1: WMI_OBJ_TEXT = 3i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WMI_OBJ_TEXT_WMI_EXT2: WMI_OBJ_TEXT = 4i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WMI_OBJ_TEXT_WMI_EXT3: WMI_OBJ_TEXT = 5i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WMI_OBJ_TEXT_WMI_EXT4: WMI_OBJ_TEXT = 6i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WMI_OBJ_TEXT_WMI_EXT5: WMI_OBJ_TEXT = 7i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WMI_OBJ_TEXT_WMI_EXT6: WMI_OBJ_TEXT = 8i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WMI_OBJ_TEXT_WMI_EXT7: WMI_OBJ_TEXT = 9i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WMI_OBJ_TEXT_WMI_EXT8: WMI_OBJ_TEXT = 10i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WMI_OBJ_TEXT_WMI_EXT9: WMI_OBJ_TEXT = 11i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WMI_OBJ_TEXT_WMI_EXT10: WMI_OBJ_TEXT = 12i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WMI_OBJ_TEXT_LAST: WMI_OBJ_TEXT = 13i32;
pub const WbemAdministrativeLocator: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcb8555cc_9128_11d1_ad9b_00c04fd8fdff);
pub const WbemAuthenticatedLocator: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcd184336_9128_11d1_ad9b_00c04fd8fdff);
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub type WbemAuthenticationLevelEnum = i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemAuthenticationLevelDefault: WbemAuthenticationLevelEnum = 0i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemAuthenticationLevelNone: WbemAuthenticationLevelEnum = 1i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemAuthenticationLevelConnect: WbemAuthenticationLevelEnum = 2i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemAuthenticationLevelCall: WbemAuthenticationLevelEnum = 3i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemAuthenticationLevelPkt: WbemAuthenticationLevelEnum = 4i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemAuthenticationLevelPktIntegrity: WbemAuthenticationLevelEnum = 5i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemAuthenticationLevelPktPrivacy: WbemAuthenticationLevelEnum = 6i32;
pub const WbemBackupRestore: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc49e32c6_bc8b_11d2_85d4_00105a1f8304);
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub type WbemChangeFlagEnum = i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemChangeFlagCreateOrUpdate: WbemChangeFlagEnum = 0i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemChangeFlagUpdateOnly: WbemChangeFlagEnum = 1i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemChangeFlagCreateOnly: WbemChangeFlagEnum = 2i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemChangeFlagUpdateCompatible: WbemChangeFlagEnum = 0i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemChangeFlagUpdateSafeMode: WbemChangeFlagEnum = 32i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemChangeFlagUpdateForceMode: WbemChangeFlagEnum = 64i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemChangeFlagStrongValidation: WbemChangeFlagEnum = 128i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemChangeFlagAdvisory: WbemChangeFlagEnum = 65536i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub type WbemCimtypeEnum = i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemCimtypeSint8: WbemCimtypeEnum = 16i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemCimtypeUint8: WbemCimtypeEnum = 17i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemCimtypeSint16: WbemCimtypeEnum = 2i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemCimtypeUint16: WbemCimtypeEnum = 18i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemCimtypeSint32: WbemCimtypeEnum = 3i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemCimtypeUint32: WbemCimtypeEnum = 19i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemCimtypeSint64: WbemCimtypeEnum = 20i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemCimtypeUint64: WbemCimtypeEnum = 21i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemCimtypeReal32: WbemCimtypeEnum = 4i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemCimtypeReal64: WbemCimtypeEnum = 5i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemCimtypeBoolean: WbemCimtypeEnum = 11i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemCimtypeString: WbemCimtypeEnum = 8i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemCimtypeDatetime: WbemCimtypeEnum = 101i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemCimtypeReference: WbemCimtypeEnum = 102i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemCimtypeChar16: WbemCimtypeEnum = 103i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemCimtypeObject: WbemCimtypeEnum = 13i32;
pub const WbemClassObject: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9a653086_174f_11d2_b5f9_00104b703efd);
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub type WbemComparisonFlagEnum = i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemComparisonFlagIncludeAll: WbemComparisonFlagEnum = 0i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemComparisonFlagIgnoreQualifiers: WbemComparisonFlagEnum = 1i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemComparisonFlagIgnoreObjectSource: WbemComparisonFlagEnum = 2i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemComparisonFlagIgnoreDefaultValues: WbemComparisonFlagEnum = 4i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemComparisonFlagIgnoreClass: WbemComparisonFlagEnum = 8i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemComparisonFlagIgnoreCase: WbemComparisonFlagEnum = 16i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemComparisonFlagIgnoreFlavor: WbemComparisonFlagEnum = 32i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub type WbemConnectOptionsEnum = i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemConnectFlagUseMaxWait: WbemConnectOptionsEnum = 128i32;
pub const WbemContext: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x674b6698_ee92_11d0_ad71_00c04fd8fdff);
pub const WbemDCOMTransport: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf7ce2e13_8c90_11d1_9e7b_00c04fc324a8);
pub const WbemDecoupledBasicEventProvider: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf5f75737_2843_4f22_933d_c76a97cda62f);
pub const WbemDecoupledRegistrar: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4cfc7932_0f9d_4bef_9c32_8ea2a6b56fcb);
pub const WbemDefPath: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcf4cc405_e2c5_4ddd_b3ce_5e7582d8c9fa);
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub type WbemErrorEnum = i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemNoErr: WbemErrorEnum = 0i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemErrFailed: WbemErrorEnum = -2147217407i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemErrNotFound: WbemErrorEnum = -2147217406i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemErrAccessDenied: WbemErrorEnum = -2147217405i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemErrProviderFailure: WbemErrorEnum = -2147217404i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemErrTypeMismatch: WbemErrorEnum = -2147217403i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemErrOutOfMemory: WbemErrorEnum = -2147217402i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemErrInvalidContext: WbemErrorEnum = -2147217401i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemErrInvalidParameter: WbemErrorEnum = -2147217400i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemErrNotAvailable: WbemErrorEnum = -2147217399i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemErrCriticalError: WbemErrorEnum = -2147217398i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemErrInvalidStream: WbemErrorEnum = -2147217397i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemErrNotSupported: WbemErrorEnum = -2147217396i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemErrInvalidSuperclass: WbemErrorEnum = -2147217395i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemErrInvalidNamespace: WbemErrorEnum = -2147217394i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemErrInvalidObject: WbemErrorEnum = -2147217393i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemErrInvalidClass: WbemErrorEnum = -2147217392i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemErrProviderNotFound: WbemErrorEnum = -2147217391i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemErrInvalidProviderRegistration: WbemErrorEnum = -2147217390i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemErrProviderLoadFailure: WbemErrorEnum = -2147217389i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemErrInitializationFailure: WbemErrorEnum = -2147217388i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemErrTransportFailure: WbemErrorEnum = -2147217387i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemErrInvalidOperation: WbemErrorEnum = -2147217386i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemErrInvalidQuery: WbemErrorEnum = -2147217385i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemErrInvalidQueryType: WbemErrorEnum = -2147217384i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemErrAlreadyExists: WbemErrorEnum = -2147217383i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemErrOverrideNotAllowed: WbemErrorEnum = -2147217382i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemErrPropagatedQualifier: WbemErrorEnum = -2147217381i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemErrPropagatedProperty: WbemErrorEnum = -2147217380i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemErrUnexpected: WbemErrorEnum = -2147217379i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemErrIllegalOperation: WbemErrorEnum = -2147217378i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemErrCannotBeKey: WbemErrorEnum = -2147217377i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemErrIncompleteClass: WbemErrorEnum = -2147217376i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemErrInvalidSyntax: WbemErrorEnum = -2147217375i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemErrNondecoratedObject: WbemErrorEnum = -2147217374i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemErrReadOnly: WbemErrorEnum = -2147217373i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemErrProviderNotCapable: WbemErrorEnum = -2147217372i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemErrClassHasChildren: WbemErrorEnum = -2147217371i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemErrClassHasInstances: WbemErrorEnum = -2147217370i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemErrQueryNotImplemented: WbemErrorEnum = -2147217369i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemErrIllegalNull: WbemErrorEnum = -2147217368i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemErrInvalidQualifierType: WbemErrorEnum = -2147217367i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemErrInvalidPropertyType: WbemErrorEnum = -2147217366i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemErrValueOutOfRange: WbemErrorEnum = -2147217365i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemErrCannotBeSingleton: WbemErrorEnum = -2147217364i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemErrInvalidCimType: WbemErrorEnum = -2147217363i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemErrInvalidMethod: WbemErrorEnum = -2147217362i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemErrInvalidMethodParameters: WbemErrorEnum = -2147217361i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemErrSystemProperty: WbemErrorEnum = -2147217360i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemErrInvalidProperty: WbemErrorEnum = -2147217359i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemErrCallCancelled: WbemErrorEnum = -2147217358i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemErrShuttingDown: WbemErrorEnum = -2147217357i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemErrPropagatedMethod: WbemErrorEnum = -2147217356i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemErrUnsupportedParameter: WbemErrorEnum = -2147217355i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemErrMissingParameter: WbemErrorEnum = -2147217354i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemErrInvalidParameterId: WbemErrorEnum = -2147217353i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemErrNonConsecutiveParameterIds: WbemErrorEnum = -2147217352i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemErrParameterIdOnRetval: WbemErrorEnum = -2147217351i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemErrInvalidObjectPath: WbemErrorEnum = -2147217350i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemErrOutOfDiskSpace: WbemErrorEnum = -2147217349i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemErrBufferTooSmall: WbemErrorEnum = -2147217348i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemErrUnsupportedPutExtension: WbemErrorEnum = -2147217347i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemErrUnknownObjectType: WbemErrorEnum = -2147217346i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemErrUnknownPacketType: WbemErrorEnum = -2147217345i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemErrMarshalVersionMismatch: WbemErrorEnum = -2147217344i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemErrMarshalInvalidSignature: WbemErrorEnum = -2147217343i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemErrInvalidQualifier: WbemErrorEnum = -2147217342i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemErrInvalidDuplicateParameter: WbemErrorEnum = -2147217341i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemErrTooMuchData: WbemErrorEnum = -2147217340i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemErrServerTooBusy: WbemErrorEnum = -2147217339i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemErrInvalidFlavor: WbemErrorEnum = -2147217338i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemErrCircularReference: WbemErrorEnum = -2147217337i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemErrUnsupportedClassUpdate: WbemErrorEnum = -2147217336i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemErrCannotChangeKeyInheritance: WbemErrorEnum = -2147217335i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemErrCannotChangeIndexInheritance: WbemErrorEnum = -2147217328i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemErrTooManyProperties: WbemErrorEnum = -2147217327i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemErrUpdateTypeMismatch: WbemErrorEnum = -2147217326i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemErrUpdateOverrideNotAllowed: WbemErrorEnum = -2147217325i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemErrUpdatePropagatedMethod: WbemErrorEnum = -2147217324i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemErrMethodNotImplemented: WbemErrorEnum = -2147217323i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemErrMethodDisabled: WbemErrorEnum = -2147217322i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemErrRefresherBusy: WbemErrorEnum = -2147217321i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemErrUnparsableQuery: WbemErrorEnum = -2147217320i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemErrNotEventClass: WbemErrorEnum = -2147217319i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemErrMissingGroupWithin: WbemErrorEnum = -2147217318i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemErrMissingAggregationList: WbemErrorEnum = -2147217317i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemErrPropertyNotAnObject: WbemErrorEnum = -2147217316i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemErrAggregatingByObject: WbemErrorEnum = -2147217315i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemErrUninterpretableProviderQuery: WbemErrorEnum = -2147217313i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemErrBackupRestoreWinmgmtRunning: WbemErrorEnum = -2147217312i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemErrQueueOverflow: WbemErrorEnum = -2147217311i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemErrPrivilegeNotHeld: WbemErrorEnum = -2147217310i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemErrInvalidOperator: WbemErrorEnum = -2147217309i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemErrLocalCredentials: WbemErrorEnum = -2147217308i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemErrCannotBeAbstract: WbemErrorEnum = -2147217307i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemErrAmendedObject: WbemErrorEnum = -2147217306i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemErrClientTooSlow: WbemErrorEnum = -2147217305i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemErrNullSecurityDescriptor: WbemErrorEnum = -2147217304i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemErrTimeout: WbemErrorEnum = -2147217303i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemErrInvalidAssociation: WbemErrorEnum = -2147217302i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemErrAmbiguousOperation: WbemErrorEnum = -2147217301i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemErrQuotaViolation: WbemErrorEnum = -2147217300i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemErrTransactionConflict: WbemErrorEnum = -2147217299i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemErrForcedRollback: WbemErrorEnum = -2147217298i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemErrUnsupportedLocale: WbemErrorEnum = -2147217297i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemErrHandleOutOfDate: WbemErrorEnum = -2147217296i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemErrConnectionFailed: WbemErrorEnum = -2147217295i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemErrInvalidHandleRequest: WbemErrorEnum = -2147217294i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemErrPropertyNameTooWide: WbemErrorEnum = -2147217293i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemErrClassNameTooWide: WbemErrorEnum = -2147217292i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemErrMethodNameTooWide: WbemErrorEnum = -2147217291i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemErrQualifierNameTooWide: WbemErrorEnum = -2147217290i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemErrRerunCommand: WbemErrorEnum = -2147217289i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemErrDatabaseVerMismatch: WbemErrorEnum = -2147217288i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemErrVetoPut: WbemErrorEnum = -2147217287i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemErrVetoDelete: WbemErrorEnum = -2147217286i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemErrInvalidLocale: WbemErrorEnum = -2147217280i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemErrProviderSuspended: WbemErrorEnum = -2147217279i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemErrSynchronizationRequired: WbemErrorEnum = -2147217278i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemErrNoSchema: WbemErrorEnum = -2147217277i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemErrProviderAlreadyRegistered: WbemErrorEnum = -2147217276i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemErrProviderNotRegistered: WbemErrorEnum = -2147217275i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemErrFatalTransportError: WbemErrorEnum = -2147217274i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemErrEncryptedConnectionRequired: WbemErrorEnum = -2147217273i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemErrRegistrationTooBroad: WbemErrorEnum = -2147213311i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemErrRegistrationTooPrecise: WbemErrorEnum = -2147213310i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemErrTimedout: WbemErrorEnum = -2147209215i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemErrResetToDefault: WbemErrorEnum = -2147209214i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub type WbemFlagEnum = i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemFlagReturnImmediately: WbemFlagEnum = 16i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemFlagReturnWhenComplete: WbemFlagEnum = 0i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemFlagBidirectional: WbemFlagEnum = 0i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemFlagForwardOnly: WbemFlagEnum = 32i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemFlagNoErrorObject: WbemFlagEnum = 64i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemFlagReturnErrorObject: WbemFlagEnum = 0i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemFlagSendStatus: WbemFlagEnum = 128i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemFlagDontSendStatus: WbemFlagEnum = 0i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemFlagEnsureLocatable: WbemFlagEnum = 256i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemFlagDirectRead: WbemFlagEnum = 512i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemFlagSendOnlySelected: WbemFlagEnum = 0i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemFlagUseAmendedQualifiers: WbemFlagEnum = 131072i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemFlagGetDefault: WbemFlagEnum = 0i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemFlagSpawnInstance: WbemFlagEnum = 1i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemFlagUseCurrentTime: WbemFlagEnum = 1i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub type WbemImpersonationLevelEnum = i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemImpersonationLevelAnonymous: WbemImpersonationLevelEnum = 1i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemImpersonationLevelIdentify: WbemImpersonationLevelEnum = 2i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemImpersonationLevelImpersonate: WbemImpersonationLevelEnum = 3i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemImpersonationLevelDelegate: WbemImpersonationLevelEnum = 4i32;
pub const WbemLevel1Login: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8bc3f05e_d86b_11d0_a075_00c04fb68820);
pub const WbemLocalAddrRes: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa1044801_8f7e_11d1_9e7c_00c04fc324a8);
pub const WbemLocator: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4590f811_1d3a_11d0_891f_00aa004b2e24);
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub type WbemObjectTextFormatEnum = i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemObjectTextFormatCIMDTD20: WbemObjectTextFormatEnum = 1i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemObjectTextFormatWMIDTD20: WbemObjectTextFormatEnum = 2i32;
pub const WbemObjectTextSrc: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8d1c559d_84f0_4bb3_a7d5_56a7435a9ba6);
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub type WbemPrivilegeEnum = i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemPrivilegeCreateToken: WbemPrivilegeEnum = 1i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemPrivilegePrimaryToken: WbemPrivilegeEnum = 2i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemPrivilegeLockMemory: WbemPrivilegeEnum = 3i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemPrivilegeIncreaseQuota: WbemPrivilegeEnum = 4i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemPrivilegeMachineAccount: WbemPrivilegeEnum = 5i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemPrivilegeTcb: WbemPrivilegeEnum = 6i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemPrivilegeSecurity: WbemPrivilegeEnum = 7i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemPrivilegeTakeOwnership: WbemPrivilegeEnum = 8i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemPrivilegeLoadDriver: WbemPrivilegeEnum = 9i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemPrivilegeSystemProfile: WbemPrivilegeEnum = 10i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemPrivilegeSystemtime: WbemPrivilegeEnum = 11i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemPrivilegeProfileSingleProcess: WbemPrivilegeEnum = 12i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemPrivilegeIncreaseBasePriority: WbemPrivilegeEnum = 13i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemPrivilegeCreatePagefile: WbemPrivilegeEnum = 14i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemPrivilegeCreatePermanent: WbemPrivilegeEnum = 15i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemPrivilegeBackup: WbemPrivilegeEnum = 16i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemPrivilegeRestore: WbemPrivilegeEnum = 17i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemPrivilegeShutdown: WbemPrivilegeEnum = 18i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemPrivilegeDebug: WbemPrivilegeEnum = 19i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemPrivilegeAudit: WbemPrivilegeEnum = 20i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemPrivilegeSystemEnvironment: WbemPrivilegeEnum = 21i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemPrivilegeChangeNotify: WbemPrivilegeEnum = 22i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemPrivilegeRemoteShutdown: WbemPrivilegeEnum = 23i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemPrivilegeUndock: WbemPrivilegeEnum = 24i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemPrivilegeSyncAgent: WbemPrivilegeEnum = 25i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemPrivilegeEnableDelegation: WbemPrivilegeEnum = 26i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemPrivilegeManageVolume: WbemPrivilegeEnum = 27i32;
pub const WbemQuery: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeac8a024_21e2_4523_ad73_a71a0aa2f56a);
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub type WbemQueryFlagEnum = i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemQueryFlagDeep: WbemQueryFlagEnum = 0i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemQueryFlagShallow: WbemQueryFlagEnum = 1i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemQueryFlagPrototype: WbemQueryFlagEnum = 2i32;
pub const WbemRefresher: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc71566f2_561e_11d1_ad87_00c04fd8fdff);
pub const WbemStatusCodeText: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeb87e1bd_3233_11d2_aec9_00c04fb68820);
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub type WbemTextFlagEnum = i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemTextFlagNoFlavors: WbemTextFlagEnum = 1i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub type WbemTimeout = i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const wbemTimeoutInfinite: WbemTimeout = -1i32;
pub const WbemUnauthenticatedLocator: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x443e7b79_de31_11d2_b340_00104bcc4b4a);
pub const WbemUninitializedClassObject: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7a0227f6_7108_11d1_ad90_00c04fd8fdff);
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub type tag_WBEM_LOGIN_TYPE = i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_FLAG_INPROC_LOGIN: tag_WBEM_LOGIN_TYPE = 0i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_FLAG_LOCAL_LOGIN: tag_WBEM_LOGIN_TYPE = 1i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_FLAG_REMOTE_LOGIN: tag_WBEM_LOGIN_TYPE = 2i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_AUTHENTICATION_METHOD_MASK: tag_WBEM_LOGIN_TYPE = 15i32;
#[doc = "*Required features: 'Win32_System_Wmi'*"]
pub const WBEM_FLAG_USE_MULTIPLE_CHALLENGES: tag_WBEM_LOGIN_TYPE = 16i32;
