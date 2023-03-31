#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
#[inline]
pub unsafe fn MI_Application_InitializeV1(flags: u32, applicationid: ::core::option::Option<*const u16>, extendederror: ::core::option::Option<*mut *mut MI_Instance>, application: *mut MI_Application) -> MI_Result {
    ::windows_targets::link ! ( "mi.dll""cdecl" fn MI_Application_InitializeV1 ( flags : u32 , applicationid : *const u16 , extendederror : *mut *mut MI_Instance , application : *mut MI_Application ) -> MI_Result );
    MI_Application_InitializeV1(flags, ::core::mem::transmute(applicationid.unwrap_or(::std::ptr::null())), ::core::mem::transmute(extendederror.unwrap_or(::std::ptr::null_mut())), application)
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
#[repr(transparent)]
pub struct IEnumWbemClassObject(::windows::core::IUnknown);
impl IEnumWbemClassObject {
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Next(&self, ltimeout: i32, apobjects: &mut [::core::option::Option<IWbemClassObject>], pureturned: *mut u32) -> ::windows::core::HRESULT {
        (::windows::core::Interface::vtable(self).Next)(::windows::core::Interface::as_raw(self), ltimeout, apobjects.len() as _, ::core::mem::transmute(apobjects.as_ptr()), pureturned)
    }
    pub unsafe fn NextAsync<P0>(&self, ucount: u32, psink: P0) -> ::windows::core::HRESULT
    where
        P0: ::windows::core::IntoParam<IWbemObjectSink>,
    {
        (::windows::core::Interface::vtable(self).NextAsync)(::windows::core::Interface::as_raw(self), ucount, psink.into_param().abi())
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumWbemClassObject> {
        let mut result__ = ::windows::core::zeroed::<IEnumWbemClassObject>();
        (::windows::core::Interface::vtable(self).Clone)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Skip(&self, ltimeout: i32, ncount: u32) -> ::windows::core::HRESULT {
        (::windows::core::Interface::vtable(self).Skip)(::windows::core::Interface::as_raw(self), ltimeout, ncount)
    }
}
::windows::imp::interface_hierarchy!(IEnumWbemClassObject, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IEnumWbemClassObject {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumWbemClassObject {}
impl ::core::fmt::Debug for IEnumWbemClassObject {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumWbemClassObject").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IEnumWbemClassObject {
    type Vtable = IEnumWbemClassObject_Vtbl;
}
impl ::core::clone::Clone for IEnumWbemClassObject {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IEnumWbemClassObject {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x027947e1_d731_11ce_a357_000000000001);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumWbemClassObject_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ltimeout: i32, ucount: u32, apobjects: *mut *mut ::core::ffi::c_void, pureturned: *mut u32) -> ::windows::core::HRESULT,
    pub NextAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ucount: u32, psink: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ltimeout: i32, ncount: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
#[repr(transparent)]
pub struct IMofCompiler(::windows::core::IUnknown);
impl IMofCompiler {
    pub unsafe fn CompileFile<P0, P1, P2, P3, P4>(&self, filename: P0, serverandnamespace: P1, user: P2, authority: P3, password: P4, loptionflags: i32, lclassflags: i32, linstanceflags: i32, pinfo: *mut WBEM_COMPILE_STATUS_INFO) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P3: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P4: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).CompileFile)(::windows::core::Interface::as_raw(self), filename.into_param().abi(), serverandnamespace.into_param().abi(), user.into_param().abi(), authority.into_param().abi(), password.into_param().abi(), loptionflags, lclassflags, linstanceflags, pinfo).ok()
    }
    pub unsafe fn CompileBuffer<P0, P1, P2, P3>(&self, pbuffer: &[u8], serverandnamespace: P0, user: P1, authority: P2, password: P3, loptionflags: i32, lclassflags: i32, linstanceflags: i32, pinfo: *mut WBEM_COMPILE_STATUS_INFO) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P3: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).CompileBuffer)(::windows::core::Interface::as_raw(self), pbuffer.len() as _, ::core::mem::transmute(pbuffer.as_ptr()), serverandnamespace.into_param().abi(), user.into_param().abi(), authority.into_param().abi(), password.into_param().abi(), loptionflags, lclassflags, linstanceflags, pinfo).ok()
    }
    pub unsafe fn CreateBMOF<P0, P1, P2>(&self, textfilename: P0, bmoffilename: P1, serverandnamespace: P2, loptionflags: i32, lclassflags: i32, linstanceflags: i32, pinfo: *mut WBEM_COMPILE_STATUS_INFO) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).CreateBMOF)(::windows::core::Interface::as_raw(self), textfilename.into_param().abi(), bmoffilename.into_param().abi(), serverandnamespace.into_param().abi(), loptionflags, lclassflags, linstanceflags, pinfo).ok()
    }
}
::windows::imp::interface_hierarchy!(IMofCompiler, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IMofCompiler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMofCompiler {}
impl ::core::fmt::Debug for IMofCompiler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMofCompiler").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMofCompiler {
    type Vtable = IMofCompiler_Vtbl;
}
impl ::core::clone::Clone for IMofCompiler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMofCompiler {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6daf974e_2e37_11d2_aec9_00c04fb68820);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMofCompiler_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub CompileFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filename: ::windows::core::PCWSTR, serverandnamespace: ::windows::core::PCWSTR, user: ::windows::core::PCWSTR, authority: ::windows::core::PCWSTR, password: ::windows::core::PCWSTR, loptionflags: i32, lclassflags: i32, linstanceflags: i32, pinfo: *mut WBEM_COMPILE_STATUS_INFO) -> ::windows::core::HRESULT,
    pub CompileBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buffsize: i32, pbuffer: *const u8, serverandnamespace: ::windows::core::PCWSTR, user: ::windows::core::PCWSTR, authority: ::windows::core::PCWSTR, password: ::windows::core::PCWSTR, loptionflags: i32, lclassflags: i32, linstanceflags: i32, pinfo: *mut WBEM_COMPILE_STATUS_INFO) -> ::windows::core::HRESULT,
    pub CreateBMOF: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, textfilename: ::windows::core::PCWSTR, bmoffilename: ::windows::core::PCWSTR, serverandnamespace: ::windows::core::PCWSTR, loptionflags: i32, lclassflags: i32, linstanceflags: i32, pinfo: *mut WBEM_COMPILE_STATUS_INFO) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISWbemDateTime(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISWbemDateTime {
    pub unsafe fn Value(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).Value)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetValue<P0>(&self, strvalue: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).SetValue)(::windows::core::Interface::as_raw(self), strvalue.into_param().abi()).ok()
    }
    pub unsafe fn Year(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).Year)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetYear(&self, iyear: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetYear)(::windows::core::Interface::as_raw(self), iyear).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn YearSpecified(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::VARIANT_BOOL>();
        (::windows::core::Interface::vtable(self).YearSpecified)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetYearSpecified<P0>(&self, byearspecified: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetYearSpecified)(::windows::core::Interface::as_raw(self), byearspecified.into_param().abi()).ok()
    }
    pub unsafe fn Month(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).Month)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetMonth(&self, imonth: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetMonth)(::windows::core::Interface::as_raw(self), imonth).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MonthSpecified(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::VARIANT_BOOL>();
        (::windows::core::Interface::vtable(self).MonthSpecified)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetMonthSpecified<P0>(&self, bmonthspecified: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetMonthSpecified)(::windows::core::Interface::as_raw(self), bmonthspecified.into_param().abi()).ok()
    }
    pub unsafe fn Day(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).Day)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetDay(&self, iday: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDay)(::windows::core::Interface::as_raw(self), iday).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DaySpecified(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::VARIANT_BOOL>();
        (::windows::core::Interface::vtable(self).DaySpecified)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDaySpecified<P0>(&self, bdayspecified: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetDaySpecified)(::windows::core::Interface::as_raw(self), bdayspecified.into_param().abi()).ok()
    }
    pub unsafe fn Hours(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).Hours)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetHours(&self, ihours: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetHours)(::windows::core::Interface::as_raw(self), ihours).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HoursSpecified(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::VARIANT_BOOL>();
        (::windows::core::Interface::vtable(self).HoursSpecified)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetHoursSpecified<P0>(&self, bhoursspecified: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetHoursSpecified)(::windows::core::Interface::as_raw(self), bhoursspecified.into_param().abi()).ok()
    }
    pub unsafe fn Minutes(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).Minutes)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetMinutes(&self, iminutes: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetMinutes)(::windows::core::Interface::as_raw(self), iminutes).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MinutesSpecified(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::VARIANT_BOOL>();
        (::windows::core::Interface::vtable(self).MinutesSpecified)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetMinutesSpecified<P0>(&self, bminutesspecified: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetMinutesSpecified)(::windows::core::Interface::as_raw(self), bminutesspecified.into_param().abi()).ok()
    }
    pub unsafe fn Seconds(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).Seconds)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetSeconds(&self, iseconds: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSeconds)(::windows::core::Interface::as_raw(self), iseconds).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SecondsSpecified(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::VARIANT_BOOL>();
        (::windows::core::Interface::vtable(self).SecondsSpecified)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSecondsSpecified<P0>(&self, bsecondsspecified: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetSecondsSpecified)(::windows::core::Interface::as_raw(self), bsecondsspecified.into_param().abi()).ok()
    }
    pub unsafe fn Microseconds(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).Microseconds)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetMicroseconds(&self, imicroseconds: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetMicroseconds)(::windows::core::Interface::as_raw(self), imicroseconds).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MicrosecondsSpecified(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::VARIANT_BOOL>();
        (::windows::core::Interface::vtable(self).MicrosecondsSpecified)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetMicrosecondsSpecified<P0>(&self, bmicrosecondsspecified: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetMicrosecondsSpecified)(::windows::core::Interface::as_raw(self), bmicrosecondsspecified.into_param().abi()).ok()
    }
    pub unsafe fn UTC(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).UTC)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetUTC(&self, iutc: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetUTC)(::windows::core::Interface::as_raw(self), iutc).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UTCSpecified(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::VARIANT_BOOL>();
        (::windows::core::Interface::vtable(self).UTCSpecified)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetUTCSpecified<P0>(&self, butcspecified: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetUTCSpecified)(::windows::core::Interface::as_raw(self), butcspecified.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsInterval(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::VARIANT_BOOL>();
        (::windows::core::Interface::vtable(self).IsInterval)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetIsInterval<P0>(&self, bisinterval: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetIsInterval)(::windows::core::Interface::as_raw(self), bisinterval.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetVarDate<P0>(&self, bislocal: P0) -> ::windows::core::Result<f64>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        let mut result__ = ::windows::core::zeroed::<f64>();
        (::windows::core::Interface::vtable(self).GetVarDate)(::windows::core::Interface::as_raw(self), bislocal.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetVarDate<P0>(&self, dvardate: f64, bislocal: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetVarDate)(::windows::core::Interface::as_raw(self), dvardate, bislocal.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFileTime<P0>(&self, bislocal: P0) -> ::windows::core::Result<::windows::core::BSTR>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).GetFileTime)(::windows::core::Interface::as_raw(self), bislocal.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetFileTime<P0, P1>(&self, strfiletime: P0, bislocal: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetFileTime)(::windows::core::Interface::as_raw(self), strfiletime.into_param().abi(), bislocal.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(ISWbemDateTime, ::windows::core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISWbemDateTime {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISWbemDateTime {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISWbemDateTime {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISWbemDateTime").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISWbemDateTime {
    type Vtable = ISWbemDateTime_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISWbemDateTime {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for ISWbemDateTime {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5e97458a_cf77_11d3_b38f_00105a1f473a);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISWbemDateTime_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strvalue: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub SetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strvalue: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub Year: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iyear: *mut i32) -> ::windows::core::HRESULT,
    pub SetYear: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iyear: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub YearSpecified: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, byearspecified: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    YearSpecified: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetYearSpecified: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, byearspecified: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetYearSpecified: usize,
    pub Month: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, imonth: *mut i32) -> ::windows::core::HRESULT,
    pub SetMonth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, imonth: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub MonthSpecified: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bmonthspecified: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MonthSpecified: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetMonthSpecified: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bmonthspecified: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetMonthSpecified: usize,
    pub Day: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iday: *mut i32) -> ::windows::core::HRESULT,
    pub SetDay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iday: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub DaySpecified: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bdayspecified: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DaySpecified: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetDaySpecified: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bdayspecified: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetDaySpecified: usize,
    pub Hours: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ihours: *mut i32) -> ::windows::core::HRESULT,
    pub SetHours: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ihours: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub HoursSpecified: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bhoursspecified: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    HoursSpecified: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetHoursSpecified: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bhoursspecified: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetHoursSpecified: usize,
    pub Minutes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iminutes: *mut i32) -> ::windows::core::HRESULT,
    pub SetMinutes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iminutes: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub MinutesSpecified: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bminutesspecified: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MinutesSpecified: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetMinutesSpecified: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bminutesspecified: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetMinutesSpecified: usize,
    pub Seconds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iseconds: *mut i32) -> ::windows::core::HRESULT,
    pub SetSeconds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iseconds: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SecondsSpecified: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bsecondsspecified: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SecondsSpecified: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetSecondsSpecified: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bsecondsspecified: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetSecondsSpecified: usize,
    pub Microseconds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, imicroseconds: *mut i32) -> ::windows::core::HRESULT,
    pub SetMicroseconds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, imicroseconds: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub MicrosecondsSpecified: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bmicrosecondsspecified: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MicrosecondsSpecified: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetMicrosecondsSpecified: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bmicrosecondsspecified: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetMicrosecondsSpecified: usize,
    pub UTC: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iutc: *mut i32) -> ::windows::core::HRESULT,
    pub SetUTC: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iutc: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub UTCSpecified: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, butcspecified: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    UTCSpecified: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetUTCSpecified: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, butcspecified: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetUTCSpecified: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bisinterval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsInterval: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetIsInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bisinterval: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetIsInterval: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetVarDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bislocal: super::super::Foundation::VARIANT_BOOL, dvardate: *mut f64) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetVarDate: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetVarDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dvardate: f64, bislocal: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetVarDate: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetFileTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bislocal: super::super::Foundation::VARIANT_BOOL, strfiletime: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetFileTime: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetFileTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strfiletime: ::std::mem::MaybeUninit<::windows::core::BSTR>, bislocal: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetFileTime: usize,
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISWbemEventSource(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISWbemEventSource {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn NextEvent(&self, itimeoutms: i32) -> ::windows::core::Result<ISWbemObject> {
        let mut result__ = ::windows::core::zeroed::<ISWbemObject>();
        (::windows::core::Interface::vtable(self).NextEvent)(::windows::core::Interface::as_raw(self), itimeoutms, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Security_(&self) -> ::windows::core::Result<ISWbemSecurity> {
        let mut result__ = ::windows::core::zeroed::<ISWbemSecurity>();
        (::windows::core::Interface::vtable(self).Security_)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(ISWbemEventSource, ::windows::core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISWbemEventSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISWbemEventSource {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISWbemEventSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISWbemEventSource").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISWbemEventSource {
    type Vtable = ISWbemEventSource_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISWbemEventSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for ISWbemEventSource {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x27d54d92_0ebe_11d2_8b22_00600806d9b6);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISWbemEventSource_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub NextEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itimeoutms: i32, objwbemobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    NextEvent: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Security_: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemsecurity: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Security_: usize,
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISWbemLastError(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISWbemLastError {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Put_<P0>(&self, iflags: i32, objwbemnamedvalueset: P0) -> ::windows::core::Result<ISWbemObjectPath>
    where
        P0: ::windows::core::IntoParam<super::Com::IDispatch>,
    {
        let mut result__ = ::windows::core::zeroed::<ISWbemObjectPath>();
        (::windows::core::Interface::vtable(self).base__.Put_)(::windows::core::Interface::as_raw(self), iflags, objwbemnamedvalueset.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PutAsync_<P0, P1, P2>(&self, objwbemsink: P0, iflags: i32, objwbemnamedvalueset: P1, objwbemasynccontext: P2) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::Com::IDispatch>,
        P1: ::windows::core::IntoParam<super::Com::IDispatch>,
        P2: ::windows::core::IntoParam<super::Com::IDispatch>,
    {
        (::windows::core::Interface::vtable(self).base__.PutAsync_)(::windows::core::Interface::as_raw(self), objwbemsink.into_param().abi(), iflags, objwbemnamedvalueset.into_param().abi(), objwbemasynccontext.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Delete_<P0>(&self, iflags: i32, objwbemnamedvalueset: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::Com::IDispatch>,
    {
        (::windows::core::Interface::vtable(self).base__.Delete_)(::windows::core::Interface::as_raw(self), iflags, objwbemnamedvalueset.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn DeleteAsync_<P0, P1, P2>(&self, objwbemsink: P0, iflags: i32, objwbemnamedvalueset: P1, objwbemasynccontext: P2) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::Com::IDispatch>,
        P1: ::windows::core::IntoParam<super::Com::IDispatch>,
        P2: ::windows::core::IntoParam<super::Com::IDispatch>,
    {
        (::windows::core::Interface::vtable(self).base__.DeleteAsync_)(::windows::core::Interface::as_raw(self), objwbemsink.into_param().abi(), iflags, objwbemnamedvalueset.into_param().abi(), objwbemasynccontext.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Instances_<P0>(&self, iflags: i32, objwbemnamedvalueset: P0) -> ::windows::core::Result<ISWbemObjectSet>
    where
        P0: ::windows::core::IntoParam<super::Com::IDispatch>,
    {
        let mut result__ = ::windows::core::zeroed::<ISWbemObjectSet>();
        (::windows::core::Interface::vtable(self).base__.Instances_)(::windows::core::Interface::as_raw(self), iflags, objwbemnamedvalueset.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InstancesAsync_<P0, P1, P2>(&self, objwbemsink: P0, iflags: i32, objwbemnamedvalueset: P1, objwbemasynccontext: P2) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::Com::IDispatch>,
        P1: ::windows::core::IntoParam<super::Com::IDispatch>,
        P2: ::windows::core::IntoParam<super::Com::IDispatch>,
    {
        (::windows::core::Interface::vtable(self).base__.InstancesAsync_)(::windows::core::Interface::as_raw(self), objwbemsink.into_param().abi(), iflags, objwbemnamedvalueset.into_param().abi(), objwbemasynccontext.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Subclasses_<P0>(&self, iflags: i32, objwbemnamedvalueset: P0) -> ::windows::core::Result<ISWbemObjectSet>
    where
        P0: ::windows::core::IntoParam<super::Com::IDispatch>,
    {
        let mut result__ = ::windows::core::zeroed::<ISWbemObjectSet>();
        (::windows::core::Interface::vtable(self).base__.Subclasses_)(::windows::core::Interface::as_raw(self), iflags, objwbemnamedvalueset.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SubclassesAsync_<P0, P1, P2>(&self, objwbemsink: P0, iflags: i32, objwbemnamedvalueset: P1, objwbemasynccontext: P2) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::Com::IDispatch>,
        P1: ::windows::core::IntoParam<super::Com::IDispatch>,
        P2: ::windows::core::IntoParam<super::Com::IDispatch>,
    {
        (::windows::core::Interface::vtable(self).base__.SubclassesAsync_)(::windows::core::Interface::as_raw(self), objwbemsink.into_param().abi(), iflags, objwbemnamedvalueset.into_param().abi(), objwbemasynccontext.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn Associators_<P0, P1, P2, P3, P4, P5, P6, P7, P8>(&self, strassocclass: P0, strresultclass: P1, strresultrole: P2, strrole: P3, bclassesonly: P4, bschemaonly: P5, strrequiredassocqualifier: P6, strrequiredqualifier: P7, iflags: i32, objwbemnamedvalueset: P8) -> ::windows::core::Result<ISWbemObjectSet>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
        P2: ::windows::core::IntoParam<::windows::core::BSTR>,
        P3: ::windows::core::IntoParam<::windows::core::BSTR>,
        P4: ::windows::core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
        P5: ::windows::core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
        P6: ::windows::core::IntoParam<::windows::core::BSTR>,
        P7: ::windows::core::IntoParam<::windows::core::BSTR>,
        P8: ::windows::core::IntoParam<super::Com::IDispatch>,
    {
        let mut result__ = ::windows::core::zeroed::<ISWbemObjectSet>();
        (::windows::core::Interface::vtable(self).base__.Associators_)(::windows::core::Interface::as_raw(self), strassocclass.into_param().abi(), strresultclass.into_param().abi(), strresultrole.into_param().abi(), strrole.into_param().abi(), bclassesonly.into_param().abi(), bschemaonly.into_param().abi(), strrequiredassocqualifier.into_param().abi(), strrequiredqualifier.into_param().abi(), iflags, objwbemnamedvalueset.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn AssociatorsAsync_<P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10>(&self, objwbemsink: P0, strassocclass: P1, strresultclass: P2, strresultrole: P3, strrole: P4, bclassesonly: P5, bschemaonly: P6, strrequiredassocqualifier: P7, strrequiredqualifier: P8, iflags: i32, objwbemnamedvalueset: P9, objwbemasynccontext: P10) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::Com::IDispatch>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
        P2: ::windows::core::IntoParam<::windows::core::BSTR>,
        P3: ::windows::core::IntoParam<::windows::core::BSTR>,
        P4: ::windows::core::IntoParam<::windows::core::BSTR>,
        P5: ::windows::core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
        P6: ::windows::core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
        P7: ::windows::core::IntoParam<::windows::core::BSTR>,
        P8: ::windows::core::IntoParam<::windows::core::BSTR>,
        P9: ::windows::core::IntoParam<super::Com::IDispatch>,
        P10: ::windows::core::IntoParam<super::Com::IDispatch>,
    {
        (::windows::core::Interface::vtable(self).base__.AssociatorsAsync_)(::windows::core::Interface::as_raw(self), objwbemsink.into_param().abi(), strassocclass.into_param().abi(), strresultclass.into_param().abi(), strresultrole.into_param().abi(), strrole.into_param().abi(), bclassesonly.into_param().abi(), bschemaonly.into_param().abi(), strrequiredassocqualifier.into_param().abi(), strrequiredqualifier.into_param().abi(), iflags, objwbemnamedvalueset.into_param().abi(), objwbemasynccontext.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn References_<P0, P1, P2, P3, P4, P5>(&self, strresultclass: P0, strrole: P1, bclassesonly: P2, bschemaonly: P3, strrequiredqualifier: P4, iflags: i32, objwbemnamedvalueset: P5) -> ::windows::core::Result<ISWbemObjectSet>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
        P2: ::windows::core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
        P3: ::windows::core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
        P4: ::windows::core::IntoParam<::windows::core::BSTR>,
        P5: ::windows::core::IntoParam<super::Com::IDispatch>,
    {
        let mut result__ = ::windows::core::zeroed::<ISWbemObjectSet>();
        (::windows::core::Interface::vtable(self).base__.References_)(::windows::core::Interface::as_raw(self), strresultclass.into_param().abi(), strrole.into_param().abi(), bclassesonly.into_param().abi(), bschemaonly.into_param().abi(), strrequiredqualifier.into_param().abi(), iflags, objwbemnamedvalueset.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn ReferencesAsync_<P0, P1, P2, P3, P4, P5, P6, P7>(&self, objwbemsink: P0, strresultclass: P1, strrole: P2, bclassesonly: P3, bschemaonly: P4, strrequiredqualifier: P5, iflags: i32, objwbemnamedvalueset: P6, objwbemasynccontext: P7) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::Com::IDispatch>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
        P2: ::windows::core::IntoParam<::windows::core::BSTR>,
        P3: ::windows::core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
        P4: ::windows::core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
        P5: ::windows::core::IntoParam<::windows::core::BSTR>,
        P6: ::windows::core::IntoParam<super::Com::IDispatch>,
        P7: ::windows::core::IntoParam<super::Com::IDispatch>,
    {
        (::windows::core::Interface::vtable(self).base__.ReferencesAsync_)(::windows::core::Interface::as_raw(self), objwbemsink.into_param().abi(), strresultclass.into_param().abi(), strrole.into_param().abi(), bclassesonly.into_param().abi(), bschemaonly.into_param().abi(), strrequiredqualifier.into_param().abi(), iflags, objwbemnamedvalueset.into_param().abi(), objwbemasynccontext.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ExecMethod_<P0, P1, P2>(&self, strmethodname: P0, objwbeminparameters: P1, iflags: i32, objwbemnamedvalueset: P2) -> ::windows::core::Result<ISWbemObject>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<super::Com::IDispatch>,
        P2: ::windows::core::IntoParam<super::Com::IDispatch>,
    {
        let mut result__ = ::windows::core::zeroed::<ISWbemObject>();
        (::windows::core::Interface::vtable(self).base__.ExecMethod_)(::windows::core::Interface::as_raw(self), strmethodname.into_param().abi(), objwbeminparameters.into_param().abi(), iflags, objwbemnamedvalueset.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ExecMethodAsync_<P0, P1, P2, P3, P4>(&self, objwbemsink: P0, strmethodname: P1, objwbeminparameters: P2, iflags: i32, objwbemnamedvalueset: P3, objwbemasynccontext: P4) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::Com::IDispatch>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
        P2: ::windows::core::IntoParam<super::Com::IDispatch>,
        P3: ::windows::core::IntoParam<super::Com::IDispatch>,
        P4: ::windows::core::IntoParam<super::Com::IDispatch>,
    {
        (::windows::core::Interface::vtable(self).base__.ExecMethodAsync_)(::windows::core::Interface::as_raw(self), objwbemsink.into_param().abi(), strmethodname.into_param().abi(), objwbeminparameters.into_param().abi(), iflags, objwbemnamedvalueset.into_param().abi(), objwbemasynccontext.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Clone_(&self) -> ::windows::core::Result<ISWbemObject> {
        let mut result__ = ::windows::core::zeroed::<ISWbemObject>();
        (::windows::core::Interface::vtable(self).base__.Clone_)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetObjectText_(&self, iflags: i32) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.GetObjectText_)(::windows::core::Interface::as_raw(self), iflags, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SpawnDerivedClass_(&self, iflags: i32) -> ::windows::core::Result<ISWbemObject> {
        let mut result__ = ::windows::core::zeroed::<ISWbemObject>();
        (::windows::core::Interface::vtable(self).base__.SpawnDerivedClass_)(::windows::core::Interface::as_raw(self), iflags, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SpawnInstance_(&self, iflags: i32) -> ::windows::core::Result<ISWbemObject> {
        let mut result__ = ::windows::core::zeroed::<ISWbemObject>();
        (::windows::core::Interface::vtable(self).base__.SpawnInstance_)(::windows::core::Interface::as_raw(self), iflags, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn CompareTo_<P0>(&self, objwbemobject: P0, iflags: i32) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL>
    where
        P0: ::windows::core::IntoParam<super::Com::IDispatch>,
    {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::VARIANT_BOOL>();
        (::windows::core::Interface::vtable(self).base__.CompareTo_)(::windows::core::Interface::as_raw(self), objwbemobject.into_param().abi(), iflags, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Qualifiers_(&self) -> ::windows::core::Result<ISWbemQualifierSet> {
        let mut result__ = ::windows::core::zeroed::<ISWbemQualifierSet>();
        (::windows::core::Interface::vtable(self).base__.Qualifiers_)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Properties_(&self) -> ::windows::core::Result<ISWbemPropertySet> {
        let mut result__ = ::windows::core::zeroed::<ISWbemPropertySet>();
        (::windows::core::Interface::vtable(self).base__.Properties_)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Methods_(&self) -> ::windows::core::Result<ISWbemMethodSet> {
        let mut result__ = ::windows::core::zeroed::<ISWbemMethodSet>();
        (::windows::core::Interface::vtable(self).base__.Methods_)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Derivation_(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).base__.Derivation_)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Path_(&self) -> ::windows::core::Result<ISWbemObjectPath> {
        let mut result__ = ::windows::core::zeroed::<ISWbemObjectPath>();
        (::windows::core::Interface::vtable(self).base__.Path_)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Security_(&self) -> ::windows::core::Result<ISWbemSecurity> {
        let mut result__ = ::windows::core::zeroed::<ISWbemSecurity>();
        (::windows::core::Interface::vtable(self).base__.Security_)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(ISWbemLastError, ::windows::core::IUnknown, super::Com::IDispatch, ISWbemObject);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISWbemLastError {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISWbemLastError {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISWbemLastError {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISWbemLastError").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISWbemLastError {
    type Vtable = ISWbemLastError_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISWbemLastError {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for ISWbemLastError {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd962db84_d4bb_11d1_8b09_00600806d9b6);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISWbemLastError_Vtbl {
    pub base__: ISWbemObject_Vtbl,
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISWbemLocator(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISWbemLocator {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ConnectServer<P0, P1, P2, P3, P4, P5, P6>(&self, strserver: P0, strnamespace: P1, struser: P2, strpassword: P3, strlocale: P4, strauthority: P5, isecurityflags: i32, objwbemnamedvalueset: P6) -> ::windows::core::Result<ISWbemServices>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
        P2: ::windows::core::IntoParam<::windows::core::BSTR>,
        P3: ::windows::core::IntoParam<::windows::core::BSTR>,
        P4: ::windows::core::IntoParam<::windows::core::BSTR>,
        P5: ::windows::core::IntoParam<::windows::core::BSTR>,
        P6: ::windows::core::IntoParam<super::Com::IDispatch>,
    {
        let mut result__ = ::windows::core::zeroed::<ISWbemServices>();
        (::windows::core::Interface::vtable(self).ConnectServer)(::windows::core::Interface::as_raw(self), strserver.into_param().abi(), strnamespace.into_param().abi(), struser.into_param().abi(), strpassword.into_param().abi(), strlocale.into_param().abi(), strauthority.into_param().abi(), isecurityflags, objwbemnamedvalueset.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Security_(&self) -> ::windows::core::Result<ISWbemSecurity> {
        let mut result__ = ::windows::core::zeroed::<ISWbemSecurity>();
        (::windows::core::Interface::vtable(self).Security_)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(ISWbemLocator, ::windows::core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISWbemLocator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISWbemLocator {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISWbemLocator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISWbemLocator").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISWbemLocator {
    type Vtable = ISWbemLocator_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISWbemLocator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for ISWbemLocator {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x76a6415b_cb41_11d1_8b02_00600806d9b6);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISWbemLocator_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub ConnectServer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strserver: ::std::mem::MaybeUninit<::windows::core::BSTR>, strnamespace: ::std::mem::MaybeUninit<::windows::core::BSTR>, struser: ::std::mem::MaybeUninit<::windows::core::BSTR>, strpassword: ::std::mem::MaybeUninit<::windows::core::BSTR>, strlocale: ::std::mem::MaybeUninit<::windows::core::BSTR>, strauthority: ::std::mem::MaybeUninit<::windows::core::BSTR>, isecurityflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void, objwbemservices: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ConnectServer: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Security_: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemsecurity: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Security_: usize,
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISWbemMethod(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISWbemMethod {
    pub unsafe fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).Name)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Origin(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).Origin)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InParameters(&self) -> ::windows::core::Result<ISWbemObject> {
        let mut result__ = ::windows::core::zeroed::<ISWbemObject>();
        (::windows::core::Interface::vtable(self).InParameters)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OutParameters(&self) -> ::windows::core::Result<ISWbemObject> {
        let mut result__ = ::windows::core::zeroed::<ISWbemObject>();
        (::windows::core::Interface::vtable(self).OutParameters)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Qualifiers_(&self) -> ::windows::core::Result<ISWbemQualifierSet> {
        let mut result__ = ::windows::core::zeroed::<ISWbemQualifierSet>();
        (::windows::core::Interface::vtable(self).Qualifiers_)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(ISWbemMethod, ::windows::core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISWbemMethod {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISWbemMethod {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISWbemMethod {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISWbemMethod").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISWbemMethod {
    type Vtable = ISWbemMethod_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISWbemMethod {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for ISWbemMethod {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x422e8e90_d955_11d1_8b09_00600806d9b6);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISWbemMethod_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub Origin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strorigin: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub InParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbeminparameters: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    InParameters: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OutParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemoutparameters: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OutParameters: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Qualifiers_: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemqualifierset: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Qualifiers_: usize,
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISWbemMethodSet(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISWbemMethodSet {
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::IUnknown>();
        (::windows::core::Interface::vtable(self)._NewEnum)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Item<P0>(&self, strname: P0, iflags: i32) -> ::windows::core::Result<ISWbemMethod>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<ISWbemMethod>();
        (::windows::core::Interface::vtable(self).Item)(::windows::core::Interface::as_raw(self), strname.into_param().abi(), iflags, &mut result__).from_abi(result__)
    }
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).Count)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(ISWbemMethodSet, ::windows::core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISWbemMethodSet {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISWbemMethodSet {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISWbemMethodSet {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISWbemMethodSet").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISWbemMethodSet {
    type Vtable = ISWbemMethodSet_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISWbemMethodSet {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for ISWbemMethodSet {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc93ba292_d955_11d1_8b09_00600806d9b6);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISWbemMethodSet_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strname: ::std::mem::MaybeUninit<::windows::core::BSTR>, iflags: i32, objwbemmethod: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Item: usize,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, icount: *mut i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISWbemNamedValue(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISWbemNamedValue {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Value(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).Value)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetValue(&self, varvalue: *const super::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetValue)(::windows::core::Interface::as_raw(self), varvalue).ok()
    }
    pub unsafe fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).Name)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(ISWbemNamedValue, ::windows::core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISWbemNamedValue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISWbemNamedValue {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISWbemNamedValue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISWbemNamedValue").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISWbemNamedValue {
    type Vtable = ISWbemNamedValue_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISWbemNamedValue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for ISWbemNamedValue {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x76a64164_cb41_11d1_8b02_00600806d9b6);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISWbemNamedValue_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varvalue: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Value: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varvalue: *const super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetValue: usize,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISWbemNamedValueSet(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISWbemNamedValueSet {
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::IUnknown>();
        (::windows::core::Interface::vtable(self)._NewEnum)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Item<P0>(&self, strname: P0, iflags: i32) -> ::windows::core::Result<ISWbemNamedValue>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<ISWbemNamedValue>();
        (::windows::core::Interface::vtable(self).Item)(::windows::core::Interface::as_raw(self), strname.into_param().abi(), iflags, &mut result__).from_abi(result__)
    }
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).Count)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Add<P0>(&self, strname: P0, varvalue: *const super::Com::VARIANT, iflags: i32) -> ::windows::core::Result<ISWbemNamedValue>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<ISWbemNamedValue>();
        (::windows::core::Interface::vtable(self).Add)(::windows::core::Interface::as_raw(self), strname.into_param().abi(), varvalue, iflags, &mut result__).from_abi(result__)
    }
    pub unsafe fn Remove<P0>(&self, strname: P0, iflags: i32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).Remove)(::windows::core::Interface::as_raw(self), strname.into_param().abi(), iflags).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Clone(&self) -> ::windows::core::Result<ISWbemNamedValueSet> {
        let mut result__ = ::windows::core::zeroed::<ISWbemNamedValueSet>();
        (::windows::core::Interface::vtable(self).Clone)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn DeleteAll(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DeleteAll)(::windows::core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(ISWbemNamedValueSet, ::windows::core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISWbemNamedValueSet {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISWbemNamedValueSet {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISWbemNamedValueSet {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISWbemNamedValueSet").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISWbemNamedValueSet {
    type Vtable = ISWbemNamedValueSet_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISWbemNamedValueSet {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for ISWbemNamedValueSet {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcf2376ea_ce8c_11d1_8b05_00600806d9b6);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISWbemNamedValueSet_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strname: ::std::mem::MaybeUninit<::windows::core::BSTR>, iflags: i32, objwbemnamedvalue: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Item: usize,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, icount: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Add: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strname: ::std::mem::MaybeUninit<::windows::core::BSTR>, varvalue: *const super::Com::VARIANT, iflags: i32, objwbemnamedvalue: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Add: usize,
    pub Remove: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strname: ::std::mem::MaybeUninit<::windows::core::BSTR>, iflags: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemnamedvalueset: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Clone: usize,
    pub DeleteAll: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISWbemObject(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISWbemObject {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Put_<P0>(&self, iflags: i32, objwbemnamedvalueset: P0) -> ::windows::core::Result<ISWbemObjectPath>
    where
        P0: ::windows::core::IntoParam<super::Com::IDispatch>,
    {
        let mut result__ = ::windows::core::zeroed::<ISWbemObjectPath>();
        (::windows::core::Interface::vtable(self).Put_)(::windows::core::Interface::as_raw(self), iflags, objwbemnamedvalueset.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PutAsync_<P0, P1, P2>(&self, objwbemsink: P0, iflags: i32, objwbemnamedvalueset: P1, objwbemasynccontext: P2) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::Com::IDispatch>,
        P1: ::windows::core::IntoParam<super::Com::IDispatch>,
        P2: ::windows::core::IntoParam<super::Com::IDispatch>,
    {
        (::windows::core::Interface::vtable(self).PutAsync_)(::windows::core::Interface::as_raw(self), objwbemsink.into_param().abi(), iflags, objwbemnamedvalueset.into_param().abi(), objwbemasynccontext.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Delete_<P0>(&self, iflags: i32, objwbemnamedvalueset: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::Com::IDispatch>,
    {
        (::windows::core::Interface::vtable(self).Delete_)(::windows::core::Interface::as_raw(self), iflags, objwbemnamedvalueset.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn DeleteAsync_<P0, P1, P2>(&self, objwbemsink: P0, iflags: i32, objwbemnamedvalueset: P1, objwbemasynccontext: P2) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::Com::IDispatch>,
        P1: ::windows::core::IntoParam<super::Com::IDispatch>,
        P2: ::windows::core::IntoParam<super::Com::IDispatch>,
    {
        (::windows::core::Interface::vtable(self).DeleteAsync_)(::windows::core::Interface::as_raw(self), objwbemsink.into_param().abi(), iflags, objwbemnamedvalueset.into_param().abi(), objwbemasynccontext.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Instances_<P0>(&self, iflags: i32, objwbemnamedvalueset: P0) -> ::windows::core::Result<ISWbemObjectSet>
    where
        P0: ::windows::core::IntoParam<super::Com::IDispatch>,
    {
        let mut result__ = ::windows::core::zeroed::<ISWbemObjectSet>();
        (::windows::core::Interface::vtable(self).Instances_)(::windows::core::Interface::as_raw(self), iflags, objwbemnamedvalueset.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InstancesAsync_<P0, P1, P2>(&self, objwbemsink: P0, iflags: i32, objwbemnamedvalueset: P1, objwbemasynccontext: P2) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::Com::IDispatch>,
        P1: ::windows::core::IntoParam<super::Com::IDispatch>,
        P2: ::windows::core::IntoParam<super::Com::IDispatch>,
    {
        (::windows::core::Interface::vtable(self).InstancesAsync_)(::windows::core::Interface::as_raw(self), objwbemsink.into_param().abi(), iflags, objwbemnamedvalueset.into_param().abi(), objwbemasynccontext.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Subclasses_<P0>(&self, iflags: i32, objwbemnamedvalueset: P0) -> ::windows::core::Result<ISWbemObjectSet>
    where
        P0: ::windows::core::IntoParam<super::Com::IDispatch>,
    {
        let mut result__ = ::windows::core::zeroed::<ISWbemObjectSet>();
        (::windows::core::Interface::vtable(self).Subclasses_)(::windows::core::Interface::as_raw(self), iflags, objwbemnamedvalueset.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SubclassesAsync_<P0, P1, P2>(&self, objwbemsink: P0, iflags: i32, objwbemnamedvalueset: P1, objwbemasynccontext: P2) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::Com::IDispatch>,
        P1: ::windows::core::IntoParam<super::Com::IDispatch>,
        P2: ::windows::core::IntoParam<super::Com::IDispatch>,
    {
        (::windows::core::Interface::vtable(self).SubclassesAsync_)(::windows::core::Interface::as_raw(self), objwbemsink.into_param().abi(), iflags, objwbemnamedvalueset.into_param().abi(), objwbemasynccontext.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn Associators_<P0, P1, P2, P3, P4, P5, P6, P7, P8>(&self, strassocclass: P0, strresultclass: P1, strresultrole: P2, strrole: P3, bclassesonly: P4, bschemaonly: P5, strrequiredassocqualifier: P6, strrequiredqualifier: P7, iflags: i32, objwbemnamedvalueset: P8) -> ::windows::core::Result<ISWbemObjectSet>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
        P2: ::windows::core::IntoParam<::windows::core::BSTR>,
        P3: ::windows::core::IntoParam<::windows::core::BSTR>,
        P4: ::windows::core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
        P5: ::windows::core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
        P6: ::windows::core::IntoParam<::windows::core::BSTR>,
        P7: ::windows::core::IntoParam<::windows::core::BSTR>,
        P8: ::windows::core::IntoParam<super::Com::IDispatch>,
    {
        let mut result__ = ::windows::core::zeroed::<ISWbemObjectSet>();
        (::windows::core::Interface::vtable(self).Associators_)(::windows::core::Interface::as_raw(self), strassocclass.into_param().abi(), strresultclass.into_param().abi(), strresultrole.into_param().abi(), strrole.into_param().abi(), bclassesonly.into_param().abi(), bschemaonly.into_param().abi(), strrequiredassocqualifier.into_param().abi(), strrequiredqualifier.into_param().abi(), iflags, objwbemnamedvalueset.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn AssociatorsAsync_<P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10>(&self, objwbemsink: P0, strassocclass: P1, strresultclass: P2, strresultrole: P3, strrole: P4, bclassesonly: P5, bschemaonly: P6, strrequiredassocqualifier: P7, strrequiredqualifier: P8, iflags: i32, objwbemnamedvalueset: P9, objwbemasynccontext: P10) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::Com::IDispatch>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
        P2: ::windows::core::IntoParam<::windows::core::BSTR>,
        P3: ::windows::core::IntoParam<::windows::core::BSTR>,
        P4: ::windows::core::IntoParam<::windows::core::BSTR>,
        P5: ::windows::core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
        P6: ::windows::core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
        P7: ::windows::core::IntoParam<::windows::core::BSTR>,
        P8: ::windows::core::IntoParam<::windows::core::BSTR>,
        P9: ::windows::core::IntoParam<super::Com::IDispatch>,
        P10: ::windows::core::IntoParam<super::Com::IDispatch>,
    {
        (::windows::core::Interface::vtable(self).AssociatorsAsync_)(::windows::core::Interface::as_raw(self), objwbemsink.into_param().abi(), strassocclass.into_param().abi(), strresultclass.into_param().abi(), strresultrole.into_param().abi(), strrole.into_param().abi(), bclassesonly.into_param().abi(), bschemaonly.into_param().abi(), strrequiredassocqualifier.into_param().abi(), strrequiredqualifier.into_param().abi(), iflags, objwbemnamedvalueset.into_param().abi(), objwbemasynccontext.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn References_<P0, P1, P2, P3, P4, P5>(&self, strresultclass: P0, strrole: P1, bclassesonly: P2, bschemaonly: P3, strrequiredqualifier: P4, iflags: i32, objwbemnamedvalueset: P5) -> ::windows::core::Result<ISWbemObjectSet>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
        P2: ::windows::core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
        P3: ::windows::core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
        P4: ::windows::core::IntoParam<::windows::core::BSTR>,
        P5: ::windows::core::IntoParam<super::Com::IDispatch>,
    {
        let mut result__ = ::windows::core::zeroed::<ISWbemObjectSet>();
        (::windows::core::Interface::vtable(self).References_)(::windows::core::Interface::as_raw(self), strresultclass.into_param().abi(), strrole.into_param().abi(), bclassesonly.into_param().abi(), bschemaonly.into_param().abi(), strrequiredqualifier.into_param().abi(), iflags, objwbemnamedvalueset.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn ReferencesAsync_<P0, P1, P2, P3, P4, P5, P6, P7>(&self, objwbemsink: P0, strresultclass: P1, strrole: P2, bclassesonly: P3, bschemaonly: P4, strrequiredqualifier: P5, iflags: i32, objwbemnamedvalueset: P6, objwbemasynccontext: P7) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::Com::IDispatch>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
        P2: ::windows::core::IntoParam<::windows::core::BSTR>,
        P3: ::windows::core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
        P4: ::windows::core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
        P5: ::windows::core::IntoParam<::windows::core::BSTR>,
        P6: ::windows::core::IntoParam<super::Com::IDispatch>,
        P7: ::windows::core::IntoParam<super::Com::IDispatch>,
    {
        (::windows::core::Interface::vtable(self).ReferencesAsync_)(::windows::core::Interface::as_raw(self), objwbemsink.into_param().abi(), strresultclass.into_param().abi(), strrole.into_param().abi(), bclassesonly.into_param().abi(), bschemaonly.into_param().abi(), strrequiredqualifier.into_param().abi(), iflags, objwbemnamedvalueset.into_param().abi(), objwbemasynccontext.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ExecMethod_<P0, P1, P2>(&self, strmethodname: P0, objwbeminparameters: P1, iflags: i32, objwbemnamedvalueset: P2) -> ::windows::core::Result<ISWbemObject>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<super::Com::IDispatch>,
        P2: ::windows::core::IntoParam<super::Com::IDispatch>,
    {
        let mut result__ = ::windows::core::zeroed::<ISWbemObject>();
        (::windows::core::Interface::vtable(self).ExecMethod_)(::windows::core::Interface::as_raw(self), strmethodname.into_param().abi(), objwbeminparameters.into_param().abi(), iflags, objwbemnamedvalueset.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ExecMethodAsync_<P0, P1, P2, P3, P4>(&self, objwbemsink: P0, strmethodname: P1, objwbeminparameters: P2, iflags: i32, objwbemnamedvalueset: P3, objwbemasynccontext: P4) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::Com::IDispatch>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
        P2: ::windows::core::IntoParam<super::Com::IDispatch>,
        P3: ::windows::core::IntoParam<super::Com::IDispatch>,
        P4: ::windows::core::IntoParam<super::Com::IDispatch>,
    {
        (::windows::core::Interface::vtable(self).ExecMethodAsync_)(::windows::core::Interface::as_raw(self), objwbemsink.into_param().abi(), strmethodname.into_param().abi(), objwbeminparameters.into_param().abi(), iflags, objwbemnamedvalueset.into_param().abi(), objwbemasynccontext.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Clone_(&self) -> ::windows::core::Result<ISWbemObject> {
        let mut result__ = ::windows::core::zeroed::<ISWbemObject>();
        (::windows::core::Interface::vtable(self).Clone_)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetObjectText_(&self, iflags: i32) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).GetObjectText_)(::windows::core::Interface::as_raw(self), iflags, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SpawnDerivedClass_(&self, iflags: i32) -> ::windows::core::Result<ISWbemObject> {
        let mut result__ = ::windows::core::zeroed::<ISWbemObject>();
        (::windows::core::Interface::vtable(self).SpawnDerivedClass_)(::windows::core::Interface::as_raw(self), iflags, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SpawnInstance_(&self, iflags: i32) -> ::windows::core::Result<ISWbemObject> {
        let mut result__ = ::windows::core::zeroed::<ISWbemObject>();
        (::windows::core::Interface::vtable(self).SpawnInstance_)(::windows::core::Interface::as_raw(self), iflags, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn CompareTo_<P0>(&self, objwbemobject: P0, iflags: i32) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL>
    where
        P0: ::windows::core::IntoParam<super::Com::IDispatch>,
    {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::VARIANT_BOOL>();
        (::windows::core::Interface::vtable(self).CompareTo_)(::windows::core::Interface::as_raw(self), objwbemobject.into_param().abi(), iflags, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Qualifiers_(&self) -> ::windows::core::Result<ISWbemQualifierSet> {
        let mut result__ = ::windows::core::zeroed::<ISWbemQualifierSet>();
        (::windows::core::Interface::vtable(self).Qualifiers_)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Properties_(&self) -> ::windows::core::Result<ISWbemPropertySet> {
        let mut result__ = ::windows::core::zeroed::<ISWbemPropertySet>();
        (::windows::core::Interface::vtable(self).Properties_)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Methods_(&self) -> ::windows::core::Result<ISWbemMethodSet> {
        let mut result__ = ::windows::core::zeroed::<ISWbemMethodSet>();
        (::windows::core::Interface::vtable(self).Methods_)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Derivation_(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).Derivation_)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Path_(&self) -> ::windows::core::Result<ISWbemObjectPath> {
        let mut result__ = ::windows::core::zeroed::<ISWbemObjectPath>();
        (::windows::core::Interface::vtable(self).Path_)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Security_(&self) -> ::windows::core::Result<ISWbemSecurity> {
        let mut result__ = ::windows::core::zeroed::<ISWbemSecurity>();
        (::windows::core::Interface::vtable(self).Security_)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(ISWbemObject, ::windows::core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISWbemObject {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISWbemObject {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISWbemObject {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISWbemObject").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISWbemObject {
    type Vtable = ISWbemObject_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISWbemObject {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for ISWbemObject {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x76a6415a_cb41_11d1_8b02_00600806d9b6);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISWbemObject_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Put_: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void, objwbemobjectpath: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Put_: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub PutAsync_: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemsink: *mut ::core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void, objwbemasynccontext: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PutAsync_: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Delete_: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Delete_: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub DeleteAsync_: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemsink: *mut ::core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void, objwbemasynccontext: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    DeleteAsync_: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Instances_: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void, objwbemobjectset: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Instances_: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub InstancesAsync_: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemsink: *mut ::core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void, objwbemasynccontext: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    InstancesAsync_: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Subclasses_: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void, objwbemobjectset: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Subclasses_: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SubclassesAsync_: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemsink: *mut ::core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void, objwbemasynccontext: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SubclassesAsync_: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub Associators_: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strassocclass: ::std::mem::MaybeUninit<::windows::core::BSTR>, strresultclass: ::std::mem::MaybeUninit<::windows::core::BSTR>, strresultrole: ::std::mem::MaybeUninit<::windows::core::BSTR>, strrole: ::std::mem::MaybeUninit<::windows::core::BSTR>, bclassesonly: super::super::Foundation::VARIANT_BOOL, bschemaonly: super::super::Foundation::VARIANT_BOOL, strrequiredassocqualifier: ::std::mem::MaybeUninit<::windows::core::BSTR>, strrequiredqualifier: ::std::mem::MaybeUninit<::windows::core::BSTR>, iflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void, objwbemobjectset: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    Associators_: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub AssociatorsAsync_: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemsink: *mut ::core::ffi::c_void, strassocclass: ::std::mem::MaybeUninit<::windows::core::BSTR>, strresultclass: ::std::mem::MaybeUninit<::windows::core::BSTR>, strresultrole: ::std::mem::MaybeUninit<::windows::core::BSTR>, strrole: ::std::mem::MaybeUninit<::windows::core::BSTR>, bclassesonly: super::super::Foundation::VARIANT_BOOL, bschemaonly: super::super::Foundation::VARIANT_BOOL, strrequiredassocqualifier: ::std::mem::MaybeUninit<::windows::core::BSTR>, strrequiredqualifier: ::std::mem::MaybeUninit<::windows::core::BSTR>, iflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void, objwbemasynccontext: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    AssociatorsAsync_: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub References_: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strresultclass: ::std::mem::MaybeUninit<::windows::core::BSTR>, strrole: ::std::mem::MaybeUninit<::windows::core::BSTR>, bclassesonly: super::super::Foundation::VARIANT_BOOL, bschemaonly: super::super::Foundation::VARIANT_BOOL, strrequiredqualifier: ::std::mem::MaybeUninit<::windows::core::BSTR>, iflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void, objwbemobjectset: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    References_: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub ReferencesAsync_: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemsink: *mut ::core::ffi::c_void, strresultclass: ::std::mem::MaybeUninit<::windows::core::BSTR>, strrole: ::std::mem::MaybeUninit<::windows::core::BSTR>, bclassesonly: super::super::Foundation::VARIANT_BOOL, bschemaonly: super::super::Foundation::VARIANT_BOOL, strrequiredqualifier: ::std::mem::MaybeUninit<::windows::core::BSTR>, iflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void, objwbemasynccontext: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    ReferencesAsync_: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ExecMethod_: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strmethodname: ::std::mem::MaybeUninit<::windows::core::BSTR>, objwbeminparameters: *mut ::core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void, objwbemoutparameters: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ExecMethod_: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ExecMethodAsync_: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemsink: *mut ::core::ffi::c_void, strmethodname: ::std::mem::MaybeUninit<::windows::core::BSTR>, objwbeminparameters: *mut ::core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void, objwbemasynccontext: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ExecMethodAsync_: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Clone_: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Clone_: usize,
    pub GetObjectText_: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iflags: i32, strobjecttext: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub SpawnDerivedClass_: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iflags: i32, objwbemobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SpawnDerivedClass_: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SpawnInstance_: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iflags: i32, objwbemobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SpawnInstance_: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub CompareTo_: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemobject: *mut ::core::ffi::c_void, iflags: i32, bresult: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    CompareTo_: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Qualifiers_: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemqualifierset: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Qualifiers_: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Properties_: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbempropertyset: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Properties_: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Methods_: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemmethodset: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Methods_: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Derivation_: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strclassnamearray: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Derivation_: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Path_: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemobjectpath: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Path_: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Security_: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemsecurity: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Security_: usize,
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISWbemObjectEx(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISWbemObjectEx {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Put_<P0>(&self, iflags: i32, objwbemnamedvalueset: P0) -> ::windows::core::Result<ISWbemObjectPath>
    where
        P0: ::windows::core::IntoParam<super::Com::IDispatch>,
    {
        let mut result__ = ::windows::core::zeroed::<ISWbemObjectPath>();
        (::windows::core::Interface::vtable(self).base__.Put_)(::windows::core::Interface::as_raw(self), iflags, objwbemnamedvalueset.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PutAsync_<P0, P1, P2>(&self, objwbemsink: P0, iflags: i32, objwbemnamedvalueset: P1, objwbemasynccontext: P2) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::Com::IDispatch>,
        P1: ::windows::core::IntoParam<super::Com::IDispatch>,
        P2: ::windows::core::IntoParam<super::Com::IDispatch>,
    {
        (::windows::core::Interface::vtable(self).base__.PutAsync_)(::windows::core::Interface::as_raw(self), objwbemsink.into_param().abi(), iflags, objwbemnamedvalueset.into_param().abi(), objwbemasynccontext.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Delete_<P0>(&self, iflags: i32, objwbemnamedvalueset: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::Com::IDispatch>,
    {
        (::windows::core::Interface::vtable(self).base__.Delete_)(::windows::core::Interface::as_raw(self), iflags, objwbemnamedvalueset.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn DeleteAsync_<P0, P1, P2>(&self, objwbemsink: P0, iflags: i32, objwbemnamedvalueset: P1, objwbemasynccontext: P2) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::Com::IDispatch>,
        P1: ::windows::core::IntoParam<super::Com::IDispatch>,
        P2: ::windows::core::IntoParam<super::Com::IDispatch>,
    {
        (::windows::core::Interface::vtable(self).base__.DeleteAsync_)(::windows::core::Interface::as_raw(self), objwbemsink.into_param().abi(), iflags, objwbemnamedvalueset.into_param().abi(), objwbemasynccontext.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Instances_<P0>(&self, iflags: i32, objwbemnamedvalueset: P0) -> ::windows::core::Result<ISWbemObjectSet>
    where
        P0: ::windows::core::IntoParam<super::Com::IDispatch>,
    {
        let mut result__ = ::windows::core::zeroed::<ISWbemObjectSet>();
        (::windows::core::Interface::vtable(self).base__.Instances_)(::windows::core::Interface::as_raw(self), iflags, objwbemnamedvalueset.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InstancesAsync_<P0, P1, P2>(&self, objwbemsink: P0, iflags: i32, objwbemnamedvalueset: P1, objwbemasynccontext: P2) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::Com::IDispatch>,
        P1: ::windows::core::IntoParam<super::Com::IDispatch>,
        P2: ::windows::core::IntoParam<super::Com::IDispatch>,
    {
        (::windows::core::Interface::vtable(self).base__.InstancesAsync_)(::windows::core::Interface::as_raw(self), objwbemsink.into_param().abi(), iflags, objwbemnamedvalueset.into_param().abi(), objwbemasynccontext.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Subclasses_<P0>(&self, iflags: i32, objwbemnamedvalueset: P0) -> ::windows::core::Result<ISWbemObjectSet>
    where
        P0: ::windows::core::IntoParam<super::Com::IDispatch>,
    {
        let mut result__ = ::windows::core::zeroed::<ISWbemObjectSet>();
        (::windows::core::Interface::vtable(self).base__.Subclasses_)(::windows::core::Interface::as_raw(self), iflags, objwbemnamedvalueset.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SubclassesAsync_<P0, P1, P2>(&self, objwbemsink: P0, iflags: i32, objwbemnamedvalueset: P1, objwbemasynccontext: P2) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::Com::IDispatch>,
        P1: ::windows::core::IntoParam<super::Com::IDispatch>,
        P2: ::windows::core::IntoParam<super::Com::IDispatch>,
    {
        (::windows::core::Interface::vtable(self).base__.SubclassesAsync_)(::windows::core::Interface::as_raw(self), objwbemsink.into_param().abi(), iflags, objwbemnamedvalueset.into_param().abi(), objwbemasynccontext.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn Associators_<P0, P1, P2, P3, P4, P5, P6, P7, P8>(&self, strassocclass: P0, strresultclass: P1, strresultrole: P2, strrole: P3, bclassesonly: P4, bschemaonly: P5, strrequiredassocqualifier: P6, strrequiredqualifier: P7, iflags: i32, objwbemnamedvalueset: P8) -> ::windows::core::Result<ISWbemObjectSet>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
        P2: ::windows::core::IntoParam<::windows::core::BSTR>,
        P3: ::windows::core::IntoParam<::windows::core::BSTR>,
        P4: ::windows::core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
        P5: ::windows::core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
        P6: ::windows::core::IntoParam<::windows::core::BSTR>,
        P7: ::windows::core::IntoParam<::windows::core::BSTR>,
        P8: ::windows::core::IntoParam<super::Com::IDispatch>,
    {
        let mut result__ = ::windows::core::zeroed::<ISWbemObjectSet>();
        (::windows::core::Interface::vtable(self).base__.Associators_)(::windows::core::Interface::as_raw(self), strassocclass.into_param().abi(), strresultclass.into_param().abi(), strresultrole.into_param().abi(), strrole.into_param().abi(), bclassesonly.into_param().abi(), bschemaonly.into_param().abi(), strrequiredassocqualifier.into_param().abi(), strrequiredqualifier.into_param().abi(), iflags, objwbemnamedvalueset.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn AssociatorsAsync_<P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10>(&self, objwbemsink: P0, strassocclass: P1, strresultclass: P2, strresultrole: P3, strrole: P4, bclassesonly: P5, bschemaonly: P6, strrequiredassocqualifier: P7, strrequiredqualifier: P8, iflags: i32, objwbemnamedvalueset: P9, objwbemasynccontext: P10) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::Com::IDispatch>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
        P2: ::windows::core::IntoParam<::windows::core::BSTR>,
        P3: ::windows::core::IntoParam<::windows::core::BSTR>,
        P4: ::windows::core::IntoParam<::windows::core::BSTR>,
        P5: ::windows::core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
        P6: ::windows::core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
        P7: ::windows::core::IntoParam<::windows::core::BSTR>,
        P8: ::windows::core::IntoParam<::windows::core::BSTR>,
        P9: ::windows::core::IntoParam<super::Com::IDispatch>,
        P10: ::windows::core::IntoParam<super::Com::IDispatch>,
    {
        (::windows::core::Interface::vtable(self).base__.AssociatorsAsync_)(::windows::core::Interface::as_raw(self), objwbemsink.into_param().abi(), strassocclass.into_param().abi(), strresultclass.into_param().abi(), strresultrole.into_param().abi(), strrole.into_param().abi(), bclassesonly.into_param().abi(), bschemaonly.into_param().abi(), strrequiredassocqualifier.into_param().abi(), strrequiredqualifier.into_param().abi(), iflags, objwbemnamedvalueset.into_param().abi(), objwbemasynccontext.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn References_<P0, P1, P2, P3, P4, P5>(&self, strresultclass: P0, strrole: P1, bclassesonly: P2, bschemaonly: P3, strrequiredqualifier: P4, iflags: i32, objwbemnamedvalueset: P5) -> ::windows::core::Result<ISWbemObjectSet>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
        P2: ::windows::core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
        P3: ::windows::core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
        P4: ::windows::core::IntoParam<::windows::core::BSTR>,
        P5: ::windows::core::IntoParam<super::Com::IDispatch>,
    {
        let mut result__ = ::windows::core::zeroed::<ISWbemObjectSet>();
        (::windows::core::Interface::vtable(self).base__.References_)(::windows::core::Interface::as_raw(self), strresultclass.into_param().abi(), strrole.into_param().abi(), bclassesonly.into_param().abi(), bschemaonly.into_param().abi(), strrequiredqualifier.into_param().abi(), iflags, objwbemnamedvalueset.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn ReferencesAsync_<P0, P1, P2, P3, P4, P5, P6, P7>(&self, objwbemsink: P0, strresultclass: P1, strrole: P2, bclassesonly: P3, bschemaonly: P4, strrequiredqualifier: P5, iflags: i32, objwbemnamedvalueset: P6, objwbemasynccontext: P7) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::Com::IDispatch>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
        P2: ::windows::core::IntoParam<::windows::core::BSTR>,
        P3: ::windows::core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
        P4: ::windows::core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
        P5: ::windows::core::IntoParam<::windows::core::BSTR>,
        P6: ::windows::core::IntoParam<super::Com::IDispatch>,
        P7: ::windows::core::IntoParam<super::Com::IDispatch>,
    {
        (::windows::core::Interface::vtable(self).base__.ReferencesAsync_)(::windows::core::Interface::as_raw(self), objwbemsink.into_param().abi(), strresultclass.into_param().abi(), strrole.into_param().abi(), bclassesonly.into_param().abi(), bschemaonly.into_param().abi(), strrequiredqualifier.into_param().abi(), iflags, objwbemnamedvalueset.into_param().abi(), objwbemasynccontext.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ExecMethod_<P0, P1, P2>(&self, strmethodname: P0, objwbeminparameters: P1, iflags: i32, objwbemnamedvalueset: P2) -> ::windows::core::Result<ISWbemObject>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<super::Com::IDispatch>,
        P2: ::windows::core::IntoParam<super::Com::IDispatch>,
    {
        let mut result__ = ::windows::core::zeroed::<ISWbemObject>();
        (::windows::core::Interface::vtable(self).base__.ExecMethod_)(::windows::core::Interface::as_raw(self), strmethodname.into_param().abi(), objwbeminparameters.into_param().abi(), iflags, objwbemnamedvalueset.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ExecMethodAsync_<P0, P1, P2, P3, P4>(&self, objwbemsink: P0, strmethodname: P1, objwbeminparameters: P2, iflags: i32, objwbemnamedvalueset: P3, objwbemasynccontext: P4) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::Com::IDispatch>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
        P2: ::windows::core::IntoParam<super::Com::IDispatch>,
        P3: ::windows::core::IntoParam<super::Com::IDispatch>,
        P4: ::windows::core::IntoParam<super::Com::IDispatch>,
    {
        (::windows::core::Interface::vtable(self).base__.ExecMethodAsync_)(::windows::core::Interface::as_raw(self), objwbemsink.into_param().abi(), strmethodname.into_param().abi(), objwbeminparameters.into_param().abi(), iflags, objwbemnamedvalueset.into_param().abi(), objwbemasynccontext.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Clone_(&self) -> ::windows::core::Result<ISWbemObject> {
        let mut result__ = ::windows::core::zeroed::<ISWbemObject>();
        (::windows::core::Interface::vtable(self).base__.Clone_)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetObjectText_(&self, iflags: i32) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.GetObjectText_)(::windows::core::Interface::as_raw(self), iflags, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SpawnDerivedClass_(&self, iflags: i32) -> ::windows::core::Result<ISWbemObject> {
        let mut result__ = ::windows::core::zeroed::<ISWbemObject>();
        (::windows::core::Interface::vtable(self).base__.SpawnDerivedClass_)(::windows::core::Interface::as_raw(self), iflags, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SpawnInstance_(&self, iflags: i32) -> ::windows::core::Result<ISWbemObject> {
        let mut result__ = ::windows::core::zeroed::<ISWbemObject>();
        (::windows::core::Interface::vtable(self).base__.SpawnInstance_)(::windows::core::Interface::as_raw(self), iflags, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn CompareTo_<P0>(&self, objwbemobject: P0, iflags: i32) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL>
    where
        P0: ::windows::core::IntoParam<super::Com::IDispatch>,
    {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::VARIANT_BOOL>();
        (::windows::core::Interface::vtable(self).base__.CompareTo_)(::windows::core::Interface::as_raw(self), objwbemobject.into_param().abi(), iflags, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Qualifiers_(&self) -> ::windows::core::Result<ISWbemQualifierSet> {
        let mut result__ = ::windows::core::zeroed::<ISWbemQualifierSet>();
        (::windows::core::Interface::vtable(self).base__.Qualifiers_)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Properties_(&self) -> ::windows::core::Result<ISWbemPropertySet> {
        let mut result__ = ::windows::core::zeroed::<ISWbemPropertySet>();
        (::windows::core::Interface::vtable(self).base__.Properties_)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Methods_(&self) -> ::windows::core::Result<ISWbemMethodSet> {
        let mut result__ = ::windows::core::zeroed::<ISWbemMethodSet>();
        (::windows::core::Interface::vtable(self).base__.Methods_)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Derivation_(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).base__.Derivation_)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Path_(&self) -> ::windows::core::Result<ISWbemObjectPath> {
        let mut result__ = ::windows::core::zeroed::<ISWbemObjectPath>();
        (::windows::core::Interface::vtable(self).base__.Path_)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Security_(&self) -> ::windows::core::Result<ISWbemSecurity> {
        let mut result__ = ::windows::core::zeroed::<ISWbemSecurity>();
        (::windows::core::Interface::vtable(self).base__.Security_)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Refresh_<P0>(&self, iflags: i32, objwbemnamedvalueset: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::Com::IDispatch>,
    {
        (::windows::core::Interface::vtable(self).Refresh_)(::windows::core::Interface::as_raw(self), iflags, objwbemnamedvalueset.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SystemProperties_(&self) -> ::windows::core::Result<ISWbemPropertySet> {
        let mut result__ = ::windows::core::zeroed::<ISWbemPropertySet>();
        (::windows::core::Interface::vtable(self).SystemProperties_)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetText_<P0>(&self, iobjecttextformat: WbemObjectTextFormatEnum, iflags: i32, objwbemnamedvalueset: P0) -> ::windows::core::Result<::windows::core::BSTR>
    where
        P0: ::windows::core::IntoParam<super::Com::IDispatch>,
    {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).GetText_)(::windows::core::Interface::as_raw(self), iobjecttextformat, iflags, objwbemnamedvalueset.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetFromText_<P0, P1>(&self, bstext: P0, iobjecttextformat: WbemObjectTextFormatEnum, iflags: i32, objwbemnamedvalueset: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<super::Com::IDispatch>,
    {
        (::windows::core::Interface::vtable(self).SetFromText_)(::windows::core::Interface::as_raw(self), bstext.into_param().abi(), iobjecttextformat, iflags, objwbemnamedvalueset.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(ISWbemObjectEx, ::windows::core::IUnknown, super::Com::IDispatch, ISWbemObject);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISWbemObjectEx {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISWbemObjectEx {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISWbemObjectEx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISWbemObjectEx").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISWbemObjectEx {
    type Vtable = ISWbemObjectEx_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISWbemObjectEx {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for ISWbemObjectEx {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x269ad56a_8a67_4129_bc8c_0506dcfe9880);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISWbemObjectEx_Vtbl {
    pub base__: ISWbemObject_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Refresh_: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Refresh_: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SystemProperties_: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbempropertyset: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SystemProperties_: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetText_: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iobjecttextformat: WbemObjectTextFormatEnum, iflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void, bstext: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetText_: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetFromText_: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstext: ::std::mem::MaybeUninit<::windows::core::BSTR>, iobjecttextformat: WbemObjectTextFormatEnum, iflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetFromText_: usize,
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISWbemObjectPath(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISWbemObjectPath {
    pub unsafe fn Path(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).Path)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetPath<P0>(&self, strpath: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).SetPath)(::windows::core::Interface::as_raw(self), strpath.into_param().abi()).ok()
    }
    pub unsafe fn RelPath(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).RelPath)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetRelPath<P0>(&self, strrelpath: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).SetRelPath)(::windows::core::Interface::as_raw(self), strrelpath.into_param().abi()).ok()
    }
    pub unsafe fn Server(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).Server)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetServer<P0>(&self, strserver: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).SetServer)(::windows::core::Interface::as_raw(self), strserver.into_param().abi()).ok()
    }
    pub unsafe fn Namespace(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).Namespace)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetNamespace<P0>(&self, strnamespace: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).SetNamespace)(::windows::core::Interface::as_raw(self), strnamespace.into_param().abi()).ok()
    }
    pub unsafe fn ParentNamespace(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).ParentNamespace)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn DisplayName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).DisplayName)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetDisplayName<P0>(&self, strdisplayname: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).SetDisplayName)(::windows::core::Interface::as_raw(self), strdisplayname.into_param().abi()).ok()
    }
    pub unsafe fn Class(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).Class)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetClass<P0>(&self, strclass: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).SetClass)(::windows::core::Interface::as_raw(self), strclass.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsClass(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::VARIANT_BOOL>();
        (::windows::core::Interface::vtable(self).IsClass)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetAsClass(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAsClass)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsSingleton(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::VARIANT_BOOL>();
        (::windows::core::Interface::vtable(self).IsSingleton)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetAsSingleton(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAsSingleton)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Keys(&self) -> ::windows::core::Result<ISWbemNamedValueSet> {
        let mut result__ = ::windows::core::zeroed::<ISWbemNamedValueSet>();
        (::windows::core::Interface::vtable(self).Keys)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Security_(&self) -> ::windows::core::Result<ISWbemSecurity> {
        let mut result__ = ::windows::core::zeroed::<ISWbemSecurity>();
        (::windows::core::Interface::vtable(self).Security_)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Locale(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).Locale)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetLocale<P0>(&self, strlocale: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).SetLocale)(::windows::core::Interface::as_raw(self), strlocale.into_param().abi()).ok()
    }
    pub unsafe fn Authority(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).Authority)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetAuthority<P0>(&self, strauthority: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).SetAuthority)(::windows::core::Interface::as_raw(self), strauthority.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(ISWbemObjectPath, ::windows::core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISWbemObjectPath {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISWbemObjectPath {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISWbemObjectPath {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISWbemObjectPath").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISWbemObjectPath {
    type Vtable = ISWbemObjectPath_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISWbemObjectPath {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for ISWbemObjectPath {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5791bc27_ce9c_11d1_97bf_0000f81e849c);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISWbemObjectPath_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Path: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strpath: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub SetPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strpath: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub RelPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strrelpath: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub SetRelPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strrelpath: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub Server: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strserver: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub SetServer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strserver: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub Namespace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strnamespace: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub SetNamespace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strnamespace: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub ParentNamespace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strparentnamespace: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strdisplayname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub SetDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strdisplayname: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub Class: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strclass: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub SetClass: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strclass: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsClass: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bisclass: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsClass: usize,
    pub SetAsClass: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsSingleton: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bissingleton: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsSingleton: usize,
    pub SetAsSingleton: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Keys: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemnamedvalueset: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Keys: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Security_: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemsecurity: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Security_: usize,
    pub Locale: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strlocale: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub SetLocale: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strlocale: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub Authority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strauthority: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub SetAuthority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strauthority: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISWbemObjectSet(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISWbemObjectSet {
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::IUnknown>();
        (::windows::core::Interface::vtable(self)._NewEnum)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Item<P0>(&self, strobjectpath: P0, iflags: i32) -> ::windows::core::Result<ISWbemObject>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<ISWbemObject>();
        (::windows::core::Interface::vtable(self).Item)(::windows::core::Interface::as_raw(self), strobjectpath.into_param().abi(), iflags, &mut result__).from_abi(result__)
    }
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).Count)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Security_(&self) -> ::windows::core::Result<ISWbemSecurity> {
        let mut result__ = ::windows::core::zeroed::<ISWbemSecurity>();
        (::windows::core::Interface::vtable(self).Security_)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ItemIndex(&self, lindex: i32) -> ::windows::core::Result<ISWbemObject> {
        let mut result__ = ::windows::core::zeroed::<ISWbemObject>();
        (::windows::core::Interface::vtable(self).ItemIndex)(::windows::core::Interface::as_raw(self), lindex, &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(ISWbemObjectSet, ::windows::core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISWbemObjectSet {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISWbemObjectSet {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISWbemObjectSet {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISWbemObjectSet").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISWbemObjectSet {
    type Vtable = ISWbemObjectSet_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISWbemObjectSet {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for ISWbemObjectSet {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x76a6415f_cb41_11d1_8b02_00600806d9b6);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISWbemObjectSet_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strobjectpath: ::std::mem::MaybeUninit<::windows::core::BSTR>, iflags: i32, objwbemobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Item: usize,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, icount: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Security_: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemsecurity: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Security_: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ItemIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lindex: i32, objwbemobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ItemIndex: usize,
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISWbemPrivilege(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISWbemPrivilege {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsEnabled(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::VARIANT_BOOL>();
        (::windows::core::Interface::vtable(self).IsEnabled)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetIsEnabled<P0>(&self, bisenabled: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetIsEnabled)(::windows::core::Interface::as_raw(self), bisenabled.into_param().abi()).ok()
    }
    pub unsafe fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).Name)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn DisplayName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).DisplayName)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Identifier(&self) -> ::windows::core::Result<WbemPrivilegeEnum> {
        let mut result__ = ::windows::core::zeroed::<WbemPrivilegeEnum>();
        (::windows::core::Interface::vtable(self).Identifier)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(ISWbemPrivilege, ::windows::core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISWbemPrivilege {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISWbemPrivilege {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISWbemPrivilege {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISWbemPrivilege").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISWbemPrivilege {
    type Vtable = ISWbemPrivilege_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISWbemPrivilege {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for ISWbemPrivilege {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x26ee67bd_5804_11d2_8b4a_00600806d9b6);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISWbemPrivilege_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub IsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bisenabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsEnabled: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetIsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bisenabled: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetIsEnabled: usize,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strdisplayname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strdisplayname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub Identifier: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iprivilege: *mut WbemPrivilegeEnum) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISWbemPrivilegeSet(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISWbemPrivilegeSet {
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::IUnknown>();
        (::windows::core::Interface::vtable(self)._NewEnum)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Item(&self, iprivilege: WbemPrivilegeEnum) -> ::windows::core::Result<ISWbemPrivilege> {
        let mut result__ = ::windows::core::zeroed::<ISWbemPrivilege>();
        (::windows::core::Interface::vtable(self).Item)(::windows::core::Interface::as_raw(self), iprivilege, &mut result__).from_abi(result__)
    }
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).Count)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn Add<P0>(&self, iprivilege: WbemPrivilegeEnum, bisenabled: P0) -> ::windows::core::Result<ISWbemPrivilege>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        let mut result__ = ::windows::core::zeroed::<ISWbemPrivilege>();
        (::windows::core::Interface::vtable(self).Add)(::windows::core::Interface::as_raw(self), iprivilege, bisenabled.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn Remove(&self, iprivilege: WbemPrivilegeEnum) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Remove)(::windows::core::Interface::as_raw(self), iprivilege).ok()
    }
    pub unsafe fn DeleteAll(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DeleteAll)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn AddAsString<P0, P1>(&self, strprivilege: P0, bisenabled: P1) -> ::windows::core::Result<ISWbemPrivilege>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        let mut result__ = ::windows::core::zeroed::<ISWbemPrivilege>();
        (::windows::core::Interface::vtable(self).AddAsString)(::windows::core::Interface::as_raw(self), strprivilege.into_param().abi(), bisenabled.into_param().abi(), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(ISWbemPrivilegeSet, ::windows::core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISWbemPrivilegeSet {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISWbemPrivilegeSet {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISWbemPrivilegeSet {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISWbemPrivilegeSet").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISWbemPrivilegeSet {
    type Vtable = ISWbemPrivilegeSet_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISWbemPrivilegeSet {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for ISWbemPrivilegeSet {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x26ee67bf_5804_11d2_8b4a_00600806d9b6);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISWbemPrivilegeSet_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iprivilege: WbemPrivilegeEnum, objwbemprivilege: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Item: usize,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, icount: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub Add: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iprivilege: WbemPrivilegeEnum, bisenabled: super::super::Foundation::VARIANT_BOOL, objwbemprivilege: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    Add: usize,
    pub Remove: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iprivilege: WbemPrivilegeEnum) -> ::windows::core::HRESULT,
    pub DeleteAll: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub AddAsString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strprivilege: ::std::mem::MaybeUninit<::windows::core::BSTR>, bisenabled: super::super::Foundation::VARIANT_BOOL, objwbemprivilege: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    AddAsString: usize,
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISWbemProperty(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISWbemProperty {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Value(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).Value)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetValue(&self, varvalue: *const super::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetValue)(::windows::core::Interface::as_raw(self), varvalue).ok()
    }
    pub unsafe fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).Name)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsLocal(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::VARIANT_BOOL>();
        (::windows::core::Interface::vtable(self).IsLocal)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Origin(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).Origin)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CIMType(&self) -> ::windows::core::Result<WbemCimtypeEnum> {
        let mut result__ = ::windows::core::zeroed::<WbemCimtypeEnum>();
        (::windows::core::Interface::vtable(self).CIMType)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Qualifiers_(&self) -> ::windows::core::Result<ISWbemQualifierSet> {
        let mut result__ = ::windows::core::zeroed::<ISWbemQualifierSet>();
        (::windows::core::Interface::vtable(self).Qualifiers_)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsArray(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::VARIANT_BOOL>();
        (::windows::core::Interface::vtable(self).IsArray)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(ISWbemProperty, ::windows::core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISWbemProperty {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISWbemProperty {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISWbemProperty {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISWbemProperty").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISWbemProperty {
    type Vtable = ISWbemProperty_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISWbemProperty {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for ISWbemProperty {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1a388f98_d4ba_11d1_8b09_00600806d9b6);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISWbemProperty_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varvalue: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Value: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varvalue: *const super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetValue: usize,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsLocal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bislocal: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsLocal: usize,
    pub Origin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strorigin: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub CIMType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, icimtype: *mut WbemCimtypeEnum) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Qualifiers_: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemqualifierset: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Qualifiers_: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bisarray: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsArray: usize,
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISWbemPropertySet(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISWbemPropertySet {
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::IUnknown>();
        (::windows::core::Interface::vtable(self)._NewEnum)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Item<P0>(&self, strname: P0, iflags: i32) -> ::windows::core::Result<ISWbemProperty>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<ISWbemProperty>();
        (::windows::core::Interface::vtable(self).Item)(::windows::core::Interface::as_raw(self), strname.into_param().abi(), iflags, &mut result__).from_abi(result__)
    }
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).Count)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn Add<P0, P1>(&self, strname: P0, icimtype: WbemCimtypeEnum, bisarray: P1, iflags: i32) -> ::windows::core::Result<ISWbemProperty>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        let mut result__ = ::windows::core::zeroed::<ISWbemProperty>();
        (::windows::core::Interface::vtable(self).Add)(::windows::core::Interface::as_raw(self), strname.into_param().abi(), icimtype, bisarray.into_param().abi(), iflags, &mut result__).from_abi(result__)
    }
    pub unsafe fn Remove<P0>(&self, strname: P0, iflags: i32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).Remove)(::windows::core::Interface::as_raw(self), strname.into_param().abi(), iflags).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(ISWbemPropertySet, ::windows::core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISWbemPropertySet {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISWbemPropertySet {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISWbemPropertySet {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISWbemPropertySet").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISWbemPropertySet {
    type Vtable = ISWbemPropertySet_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISWbemPropertySet {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for ISWbemPropertySet {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdea0a7b2_d4ba_11d1_8b09_00600806d9b6);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISWbemPropertySet_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strname: ::std::mem::MaybeUninit<::windows::core::BSTR>, iflags: i32, objwbemproperty: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Item: usize,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, icount: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub Add: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strname: ::std::mem::MaybeUninit<::windows::core::BSTR>, icimtype: WbemCimtypeEnum, bisarray: super::super::Foundation::VARIANT_BOOL, iflags: i32, objwbemproperty: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    Add: usize,
    pub Remove: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strname: ::std::mem::MaybeUninit<::windows::core::BSTR>, iflags: i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISWbemQualifier(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISWbemQualifier {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Value(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).Value)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetValue(&self, varvalue: *const super::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetValue)(::windows::core::Interface::as_raw(self), varvalue).ok()
    }
    pub unsafe fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).Name)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsLocal(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::VARIANT_BOOL>();
        (::windows::core::Interface::vtable(self).IsLocal)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PropagatesToSubclass(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::VARIANT_BOOL>();
        (::windows::core::Interface::vtable(self).PropagatesToSubclass)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetPropagatesToSubclass<P0>(&self, bpropagatestosubclass: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetPropagatesToSubclass)(::windows::core::Interface::as_raw(self), bpropagatestosubclass.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PropagatesToInstance(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::VARIANT_BOOL>();
        (::windows::core::Interface::vtable(self).PropagatesToInstance)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetPropagatesToInstance<P0>(&self, bpropagatestoinstance: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetPropagatesToInstance)(::windows::core::Interface::as_raw(self), bpropagatestoinstance.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsOverridable(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::VARIANT_BOOL>();
        (::windows::core::Interface::vtable(self).IsOverridable)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetIsOverridable<P0>(&self, bisoverridable: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetIsOverridable)(::windows::core::Interface::as_raw(self), bisoverridable.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsAmended(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::VARIANT_BOOL>();
        (::windows::core::Interface::vtable(self).IsAmended)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(ISWbemQualifier, ::windows::core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISWbemQualifier {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISWbemQualifier {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISWbemQualifier {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISWbemQualifier").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISWbemQualifier {
    type Vtable = ISWbemQualifier_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISWbemQualifier {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for ISWbemQualifier {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x79b05932_d3b7_11d1_8b06_00600806d9b6);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISWbemQualifier_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varvalue: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Value: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varvalue: *const super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetValue: usize,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsLocal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bislocal: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsLocal: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub PropagatesToSubclass: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bpropagatestosubclass: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    PropagatesToSubclass: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetPropagatesToSubclass: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bpropagatestosubclass: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetPropagatesToSubclass: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub PropagatesToInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bpropagatestoinstance: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    PropagatesToInstance: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetPropagatesToInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bpropagatestoinstance: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetPropagatesToInstance: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsOverridable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bisoverridable: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsOverridable: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetIsOverridable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bisoverridable: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetIsOverridable: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsAmended: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bisamended: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsAmended: usize,
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISWbemQualifierSet(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISWbemQualifierSet {
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::IUnknown>();
        (::windows::core::Interface::vtable(self)._NewEnum)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Item<P0>(&self, name: P0, iflags: i32) -> ::windows::core::Result<ISWbemQualifier>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<ISWbemQualifier>();
        (::windows::core::Interface::vtable(self).Item)(::windows::core::Interface::as_raw(self), name.into_param().abi(), iflags, &mut result__).from_abi(result__)
    }
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).Count)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Add<P0, P1, P2, P3>(&self, strname: P0, varval: *const super::Com::VARIANT, bpropagatestosubclass: P1, bpropagatestoinstance: P2, bisoverridable: P3, iflags: i32) -> ::windows::core::Result<ISWbemQualifier>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
        P2: ::windows::core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
        P3: ::windows::core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        let mut result__ = ::windows::core::zeroed::<ISWbemQualifier>();
        (::windows::core::Interface::vtable(self).Add)(::windows::core::Interface::as_raw(self), strname.into_param().abi(), varval, bpropagatestosubclass.into_param().abi(), bpropagatestoinstance.into_param().abi(), bisoverridable.into_param().abi(), iflags, &mut result__).from_abi(result__)
    }
    pub unsafe fn Remove<P0>(&self, strname: P0, iflags: i32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).Remove)(::windows::core::Interface::as_raw(self), strname.into_param().abi(), iflags).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(ISWbemQualifierSet, ::windows::core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISWbemQualifierSet {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISWbemQualifierSet {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISWbemQualifierSet {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISWbemQualifierSet").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISWbemQualifierSet {
    type Vtable = ISWbemQualifierSet_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISWbemQualifierSet {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for ISWbemQualifierSet {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9b16ed16_d3df_11d1_8b08_00600806d9b6);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISWbemQualifierSet_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::BSTR>, iflags: i32, objwbemqualifier: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Item: usize,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, icount: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Add: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strname: ::std::mem::MaybeUninit<::windows::core::BSTR>, varval: *const super::Com::VARIANT, bpropagatestosubclass: super::super::Foundation::VARIANT_BOOL, bpropagatestoinstance: super::super::Foundation::VARIANT_BOOL, bisoverridable: super::super::Foundation::VARIANT_BOOL, iflags: i32, objwbemqualifier: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Add: usize,
    pub Remove: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strname: ::std::mem::MaybeUninit<::windows::core::BSTR>, iflags: i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISWbemRefreshableItem(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISWbemRefreshableItem {
    pub unsafe fn Index(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).Index)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Refresher(&self) -> ::windows::core::Result<ISWbemRefresher> {
        let mut result__ = ::windows::core::zeroed::<ISWbemRefresher>();
        (::windows::core::Interface::vtable(self).Refresher)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsSet(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::VARIANT_BOOL>();
        (::windows::core::Interface::vtable(self).IsSet)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Object(&self) -> ::windows::core::Result<ISWbemObjectEx> {
        let mut result__ = ::windows::core::zeroed::<ISWbemObjectEx>();
        (::windows::core::Interface::vtable(self).Object)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ObjectSet(&self) -> ::windows::core::Result<ISWbemObjectSet> {
        let mut result__ = ::windows::core::zeroed::<ISWbemObjectSet>();
        (::windows::core::Interface::vtable(self).ObjectSet)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Remove(&self, iflags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Remove)(::windows::core::Interface::as_raw(self), iflags).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(ISWbemRefreshableItem, ::windows::core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISWbemRefreshableItem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISWbemRefreshableItem {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISWbemRefreshableItem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISWbemRefreshableItem").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISWbemRefreshableItem {
    type Vtable = ISWbemRefreshableItem_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISWbemRefreshableItem {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for ISWbemRefreshableItem {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5ad4bf92_daab_11d3_b38f_00105a1f473a);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISWbemRefreshableItem_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Index: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iindex: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Refresher: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemrefresher: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Refresher: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsSet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bisset: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsSet: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Object: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Object: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ObjectSet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemobjectset: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ObjectSet: usize,
    pub Remove: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iflags: i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISWbemRefresher(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISWbemRefresher {
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::IUnknown>();
        (::windows::core::Interface::vtable(self)._NewEnum)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Item(&self, iindex: i32) -> ::windows::core::Result<ISWbemRefreshableItem> {
        let mut result__ = ::windows::core::zeroed::<ISWbemRefreshableItem>();
        (::windows::core::Interface::vtable(self).Item)(::windows::core::Interface::as_raw(self), iindex, &mut result__).from_abi(result__)
    }
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).Count)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Add<P0, P1, P2>(&self, objwbemservices: P0, bsinstancepath: P1, iflags: i32, objwbemnamedvalueset: P2) -> ::windows::core::Result<ISWbemRefreshableItem>
    where
        P0: ::windows::core::IntoParam<ISWbemServicesEx>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
        P2: ::windows::core::IntoParam<super::Com::IDispatch>,
    {
        let mut result__ = ::windows::core::zeroed::<ISWbemRefreshableItem>();
        (::windows::core::Interface::vtable(self).Add)(::windows::core::Interface::as_raw(self), objwbemservices.into_param().abi(), bsinstancepath.into_param().abi(), iflags, objwbemnamedvalueset.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddEnum<P0, P1, P2>(&self, objwbemservices: P0, bsclassname: P1, iflags: i32, objwbemnamedvalueset: P2) -> ::windows::core::Result<ISWbemRefreshableItem>
    where
        P0: ::windows::core::IntoParam<ISWbemServicesEx>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
        P2: ::windows::core::IntoParam<super::Com::IDispatch>,
    {
        let mut result__ = ::windows::core::zeroed::<ISWbemRefreshableItem>();
        (::windows::core::Interface::vtable(self).AddEnum)(::windows::core::Interface::as_raw(self), objwbemservices.into_param().abi(), bsclassname.into_param().abi(), iflags, objwbemnamedvalueset.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn Remove(&self, iindex: i32, iflags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Remove)(::windows::core::Interface::as_raw(self), iindex, iflags).ok()
    }
    pub unsafe fn Refresh(&self, iflags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Refresh)(::windows::core::Interface::as_raw(self), iflags).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AutoReconnect(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::VARIANT_BOOL>();
        (::windows::core::Interface::vtable(self).AutoReconnect)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAutoReconnect<P0>(&self, bcount: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetAutoReconnect)(::windows::core::Interface::as_raw(self), bcount.into_param().abi()).ok()
    }
    pub unsafe fn DeleteAll(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DeleteAll)(::windows::core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(ISWbemRefresher, ::windows::core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISWbemRefresher {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISWbemRefresher {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISWbemRefresher {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISWbemRefresher").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISWbemRefresher {
    type Vtable = ISWbemRefresher_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISWbemRefresher {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for ISWbemRefresher {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x14d8250e_d9c2_11d3_b38f_00105a1f473a);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISWbemRefresher_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iindex: i32, objwbemrefreshableitem: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Item: usize,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, icount: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Add: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemservices: *mut ::core::ffi::c_void, bsinstancepath: ::std::mem::MaybeUninit<::windows::core::BSTR>, iflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void, objwbemrefreshableitem: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Add: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub AddEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemservices: *mut ::core::ffi::c_void, bsclassname: ::std::mem::MaybeUninit<::windows::core::BSTR>, iflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void, objwbemrefreshableitem: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddEnum: usize,
    pub Remove: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iindex: i32, iflags: i32) -> ::windows::core::HRESULT,
    pub Refresh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iflags: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub AutoReconnect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bcount: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AutoReconnect: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetAutoReconnect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bcount: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetAutoReconnect: usize,
    pub DeleteAll: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISWbemSecurity(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISWbemSecurity {
    pub unsafe fn ImpersonationLevel(&self) -> ::windows::core::Result<WbemImpersonationLevelEnum> {
        let mut result__ = ::windows::core::zeroed::<WbemImpersonationLevelEnum>();
        (::windows::core::Interface::vtable(self).ImpersonationLevel)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetImpersonationLevel(&self, iimpersonationlevel: WbemImpersonationLevelEnum) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetImpersonationLevel)(::windows::core::Interface::as_raw(self), iimpersonationlevel).ok()
    }
    pub unsafe fn AuthenticationLevel(&self) -> ::windows::core::Result<WbemAuthenticationLevelEnum> {
        let mut result__ = ::windows::core::zeroed::<WbemAuthenticationLevelEnum>();
        (::windows::core::Interface::vtable(self).AuthenticationLevel)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetAuthenticationLevel(&self, iauthenticationlevel: WbemAuthenticationLevelEnum) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAuthenticationLevel)(::windows::core::Interface::as_raw(self), iauthenticationlevel).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Privileges(&self) -> ::windows::core::Result<ISWbemPrivilegeSet> {
        let mut result__ = ::windows::core::zeroed::<ISWbemPrivilegeSet>();
        (::windows::core::Interface::vtable(self).Privileges)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(ISWbemSecurity, ::windows::core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISWbemSecurity {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISWbemSecurity {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISWbemSecurity {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISWbemSecurity").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISWbemSecurity {
    type Vtable = ISWbemSecurity_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISWbemSecurity {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for ISWbemSecurity {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb54d66e6_2287_11d2_8b33_00600806d9b6);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISWbemSecurity_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub ImpersonationLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iimpersonationlevel: *mut WbemImpersonationLevelEnum) -> ::windows::core::HRESULT,
    pub SetImpersonationLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iimpersonationlevel: WbemImpersonationLevelEnum) -> ::windows::core::HRESULT,
    pub AuthenticationLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iauthenticationlevel: *mut WbemAuthenticationLevelEnum) -> ::windows::core::HRESULT,
    pub SetAuthenticationLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iauthenticationlevel: WbemAuthenticationLevelEnum) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Privileges: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemprivilegeset: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Privileges: usize,
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISWbemServices(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISWbemServices {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Get<P0, P1>(&self, strobjectpath: P0, iflags: i32, objwbemnamedvalueset: P1) -> ::windows::core::Result<ISWbemObject>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<super::Com::IDispatch>,
    {
        let mut result__ = ::windows::core::zeroed::<ISWbemObject>();
        (::windows::core::Interface::vtable(self).Get)(::windows::core::Interface::as_raw(self), strobjectpath.into_param().abi(), iflags, objwbemnamedvalueset.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetAsync<P0, P1, P2, P3>(&self, objwbemsink: P0, strobjectpath: P1, iflags: i32, objwbemnamedvalueset: P2, objwbemasynccontext: P3) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::Com::IDispatch>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
        P2: ::windows::core::IntoParam<super::Com::IDispatch>,
        P3: ::windows::core::IntoParam<super::Com::IDispatch>,
    {
        (::windows::core::Interface::vtable(self).GetAsync)(::windows::core::Interface::as_raw(self), objwbemsink.into_param().abi(), strobjectpath.into_param().abi(), iflags, objwbemnamedvalueset.into_param().abi(), objwbemasynccontext.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Delete<P0, P1>(&self, strobjectpath: P0, iflags: i32, objwbemnamedvalueset: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<super::Com::IDispatch>,
    {
        (::windows::core::Interface::vtable(self).Delete)(::windows::core::Interface::as_raw(self), strobjectpath.into_param().abi(), iflags, objwbemnamedvalueset.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn DeleteAsync<P0, P1, P2, P3>(&self, objwbemsink: P0, strobjectpath: P1, iflags: i32, objwbemnamedvalueset: P2, objwbemasynccontext: P3) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::Com::IDispatch>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
        P2: ::windows::core::IntoParam<super::Com::IDispatch>,
        P3: ::windows::core::IntoParam<super::Com::IDispatch>,
    {
        (::windows::core::Interface::vtable(self).DeleteAsync)(::windows::core::Interface::as_raw(self), objwbemsink.into_param().abi(), strobjectpath.into_param().abi(), iflags, objwbemnamedvalueset.into_param().abi(), objwbemasynccontext.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InstancesOf<P0, P1>(&self, strclass: P0, iflags: i32, objwbemnamedvalueset: P1) -> ::windows::core::Result<ISWbemObjectSet>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<super::Com::IDispatch>,
    {
        let mut result__ = ::windows::core::zeroed::<ISWbemObjectSet>();
        (::windows::core::Interface::vtable(self).InstancesOf)(::windows::core::Interface::as_raw(self), strclass.into_param().abi(), iflags, objwbemnamedvalueset.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InstancesOfAsync<P0, P1, P2, P3>(&self, objwbemsink: P0, strclass: P1, iflags: i32, objwbemnamedvalueset: P2, objwbemasynccontext: P3) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::Com::IDispatch>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
        P2: ::windows::core::IntoParam<super::Com::IDispatch>,
        P3: ::windows::core::IntoParam<super::Com::IDispatch>,
    {
        (::windows::core::Interface::vtable(self).InstancesOfAsync)(::windows::core::Interface::as_raw(self), objwbemsink.into_param().abi(), strclass.into_param().abi(), iflags, objwbemnamedvalueset.into_param().abi(), objwbemasynccontext.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SubclassesOf<P0, P1>(&self, strsuperclass: P0, iflags: i32, objwbemnamedvalueset: P1) -> ::windows::core::Result<ISWbemObjectSet>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<super::Com::IDispatch>,
    {
        let mut result__ = ::windows::core::zeroed::<ISWbemObjectSet>();
        (::windows::core::Interface::vtable(self).SubclassesOf)(::windows::core::Interface::as_raw(self), strsuperclass.into_param().abi(), iflags, objwbemnamedvalueset.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SubclassesOfAsync<P0, P1, P2, P3>(&self, objwbemsink: P0, strsuperclass: P1, iflags: i32, objwbemnamedvalueset: P2, objwbemasynccontext: P3) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::Com::IDispatch>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
        P2: ::windows::core::IntoParam<super::Com::IDispatch>,
        P3: ::windows::core::IntoParam<super::Com::IDispatch>,
    {
        (::windows::core::Interface::vtable(self).SubclassesOfAsync)(::windows::core::Interface::as_raw(self), objwbemsink.into_param().abi(), strsuperclass.into_param().abi(), iflags, objwbemnamedvalueset.into_param().abi(), objwbemasynccontext.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ExecQuery<P0, P1, P2>(&self, strquery: P0, strquerylanguage: P1, iflags: i32, objwbemnamedvalueset: P2) -> ::windows::core::Result<ISWbemObjectSet>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
        P2: ::windows::core::IntoParam<super::Com::IDispatch>,
    {
        let mut result__ = ::windows::core::zeroed::<ISWbemObjectSet>();
        (::windows::core::Interface::vtable(self).ExecQuery)(::windows::core::Interface::as_raw(self), strquery.into_param().abi(), strquerylanguage.into_param().abi(), iflags, objwbemnamedvalueset.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ExecQueryAsync<P0, P1, P2, P3, P4>(&self, objwbemsink: P0, strquery: P1, strquerylanguage: P2, lflags: i32, objwbemnamedvalueset: P3, objwbemasynccontext: P4) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::Com::IDispatch>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
        P2: ::windows::core::IntoParam<::windows::core::BSTR>,
        P3: ::windows::core::IntoParam<super::Com::IDispatch>,
        P4: ::windows::core::IntoParam<super::Com::IDispatch>,
    {
        (::windows::core::Interface::vtable(self).ExecQueryAsync)(::windows::core::Interface::as_raw(self), objwbemsink.into_param().abi(), strquery.into_param().abi(), strquerylanguage.into_param().abi(), lflags, objwbemnamedvalueset.into_param().abi(), objwbemasynccontext.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn AssociatorsOf<P0, P1, P2, P3, P4, P5, P6, P7, P8, P9>(&self, strobjectpath: P0, strassocclass: P1, strresultclass: P2, strresultrole: P3, strrole: P4, bclassesonly: P5, bschemaonly: P6, strrequiredassocqualifier: P7, strrequiredqualifier: P8, iflags: i32, objwbemnamedvalueset: P9) -> ::windows::core::Result<ISWbemObjectSet>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
        P2: ::windows::core::IntoParam<::windows::core::BSTR>,
        P3: ::windows::core::IntoParam<::windows::core::BSTR>,
        P4: ::windows::core::IntoParam<::windows::core::BSTR>,
        P5: ::windows::core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
        P6: ::windows::core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
        P7: ::windows::core::IntoParam<::windows::core::BSTR>,
        P8: ::windows::core::IntoParam<::windows::core::BSTR>,
        P9: ::windows::core::IntoParam<super::Com::IDispatch>,
    {
        let mut result__ = ::windows::core::zeroed::<ISWbemObjectSet>();
        (::windows::core::Interface::vtable(self).AssociatorsOf)(::windows::core::Interface::as_raw(self), strobjectpath.into_param().abi(), strassocclass.into_param().abi(), strresultclass.into_param().abi(), strresultrole.into_param().abi(), strrole.into_param().abi(), bclassesonly.into_param().abi(), bschemaonly.into_param().abi(), strrequiredassocqualifier.into_param().abi(), strrequiredqualifier.into_param().abi(), iflags, objwbemnamedvalueset.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn AssociatorsOfAsync<P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11>(&self, objwbemsink: P0, strobjectpath: P1, strassocclass: P2, strresultclass: P3, strresultrole: P4, strrole: P5, bclassesonly: P6, bschemaonly: P7, strrequiredassocqualifier: P8, strrequiredqualifier: P9, iflags: i32, objwbemnamedvalueset: P10, objwbemasynccontext: P11) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::Com::IDispatch>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
        P2: ::windows::core::IntoParam<::windows::core::BSTR>,
        P3: ::windows::core::IntoParam<::windows::core::BSTR>,
        P4: ::windows::core::IntoParam<::windows::core::BSTR>,
        P5: ::windows::core::IntoParam<::windows::core::BSTR>,
        P6: ::windows::core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
        P7: ::windows::core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
        P8: ::windows::core::IntoParam<::windows::core::BSTR>,
        P9: ::windows::core::IntoParam<::windows::core::BSTR>,
        P10: ::windows::core::IntoParam<super::Com::IDispatch>,
        P11: ::windows::core::IntoParam<super::Com::IDispatch>,
    {
        (::windows::core::Interface::vtable(self).AssociatorsOfAsync)(::windows::core::Interface::as_raw(self), objwbemsink.into_param().abi(), strobjectpath.into_param().abi(), strassocclass.into_param().abi(), strresultclass.into_param().abi(), strresultrole.into_param().abi(), strrole.into_param().abi(), bclassesonly.into_param().abi(), bschemaonly.into_param().abi(), strrequiredassocqualifier.into_param().abi(), strrequiredqualifier.into_param().abi(), iflags, objwbemnamedvalueset.into_param().abi(), objwbemasynccontext.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn ReferencesTo<P0, P1, P2, P3, P4, P5, P6>(&self, strobjectpath: P0, strresultclass: P1, strrole: P2, bclassesonly: P3, bschemaonly: P4, strrequiredqualifier: P5, iflags: i32, objwbemnamedvalueset: P6) -> ::windows::core::Result<ISWbemObjectSet>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
        P2: ::windows::core::IntoParam<::windows::core::BSTR>,
        P3: ::windows::core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
        P4: ::windows::core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
        P5: ::windows::core::IntoParam<::windows::core::BSTR>,
        P6: ::windows::core::IntoParam<super::Com::IDispatch>,
    {
        let mut result__ = ::windows::core::zeroed::<ISWbemObjectSet>();
        (::windows::core::Interface::vtable(self).ReferencesTo)(::windows::core::Interface::as_raw(self), strobjectpath.into_param().abi(), strresultclass.into_param().abi(), strrole.into_param().abi(), bclassesonly.into_param().abi(), bschemaonly.into_param().abi(), strrequiredqualifier.into_param().abi(), iflags, objwbemnamedvalueset.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn ReferencesToAsync<P0, P1, P2, P3, P4, P5, P6, P7, P8>(&self, objwbemsink: P0, strobjectpath: P1, strresultclass: P2, strrole: P3, bclassesonly: P4, bschemaonly: P5, strrequiredqualifier: P6, iflags: i32, objwbemnamedvalueset: P7, objwbemasynccontext: P8) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::Com::IDispatch>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
        P2: ::windows::core::IntoParam<::windows::core::BSTR>,
        P3: ::windows::core::IntoParam<::windows::core::BSTR>,
        P4: ::windows::core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
        P5: ::windows::core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
        P6: ::windows::core::IntoParam<::windows::core::BSTR>,
        P7: ::windows::core::IntoParam<super::Com::IDispatch>,
        P8: ::windows::core::IntoParam<super::Com::IDispatch>,
    {
        (::windows::core::Interface::vtable(self).ReferencesToAsync)(::windows::core::Interface::as_raw(self), objwbemsink.into_param().abi(), strobjectpath.into_param().abi(), strresultclass.into_param().abi(), strrole.into_param().abi(), bclassesonly.into_param().abi(), bschemaonly.into_param().abi(), strrequiredqualifier.into_param().abi(), iflags, objwbemnamedvalueset.into_param().abi(), objwbemasynccontext.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ExecNotificationQuery<P0, P1, P2>(&self, strquery: P0, strquerylanguage: P1, iflags: i32, objwbemnamedvalueset: P2) -> ::windows::core::Result<ISWbemEventSource>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
        P2: ::windows::core::IntoParam<super::Com::IDispatch>,
    {
        let mut result__ = ::windows::core::zeroed::<ISWbemEventSource>();
        (::windows::core::Interface::vtable(self).ExecNotificationQuery)(::windows::core::Interface::as_raw(self), strquery.into_param().abi(), strquerylanguage.into_param().abi(), iflags, objwbemnamedvalueset.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ExecNotificationQueryAsync<P0, P1, P2, P3, P4>(&self, objwbemsink: P0, strquery: P1, strquerylanguage: P2, iflags: i32, objwbemnamedvalueset: P3, objwbemasynccontext: P4) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::Com::IDispatch>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
        P2: ::windows::core::IntoParam<::windows::core::BSTR>,
        P3: ::windows::core::IntoParam<super::Com::IDispatch>,
        P4: ::windows::core::IntoParam<super::Com::IDispatch>,
    {
        (::windows::core::Interface::vtable(self).ExecNotificationQueryAsync)(::windows::core::Interface::as_raw(self), objwbemsink.into_param().abi(), strquery.into_param().abi(), strquerylanguage.into_param().abi(), iflags, objwbemnamedvalueset.into_param().abi(), objwbemasynccontext.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ExecMethod<P0, P1, P2, P3>(&self, strobjectpath: P0, strmethodname: P1, objwbeminparameters: P2, iflags: i32, objwbemnamedvalueset: P3) -> ::windows::core::Result<ISWbemObject>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
        P2: ::windows::core::IntoParam<super::Com::IDispatch>,
        P3: ::windows::core::IntoParam<super::Com::IDispatch>,
    {
        let mut result__ = ::windows::core::zeroed::<ISWbemObject>();
        (::windows::core::Interface::vtable(self).ExecMethod)(::windows::core::Interface::as_raw(self), strobjectpath.into_param().abi(), strmethodname.into_param().abi(), objwbeminparameters.into_param().abi(), iflags, objwbemnamedvalueset.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ExecMethodAsync<P0, P1, P2, P3, P4, P5>(&self, objwbemsink: P0, strobjectpath: P1, strmethodname: P2, objwbeminparameters: P3, iflags: i32, objwbemnamedvalueset: P4, objwbemasynccontext: P5) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::Com::IDispatch>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
        P2: ::windows::core::IntoParam<::windows::core::BSTR>,
        P3: ::windows::core::IntoParam<super::Com::IDispatch>,
        P4: ::windows::core::IntoParam<super::Com::IDispatch>,
        P5: ::windows::core::IntoParam<super::Com::IDispatch>,
    {
        (::windows::core::Interface::vtable(self).ExecMethodAsync)(::windows::core::Interface::as_raw(self), objwbemsink.into_param().abi(), strobjectpath.into_param().abi(), strmethodname.into_param().abi(), objwbeminparameters.into_param().abi(), iflags, objwbemnamedvalueset.into_param().abi(), objwbemasynccontext.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Security_(&self) -> ::windows::core::Result<ISWbemSecurity> {
        let mut result__ = ::windows::core::zeroed::<ISWbemSecurity>();
        (::windows::core::Interface::vtable(self).Security_)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(ISWbemServices, ::windows::core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISWbemServices {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISWbemServices {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISWbemServices {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISWbemServices").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISWbemServices {
    type Vtable = ISWbemServices_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISWbemServices {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for ISWbemServices {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x76a6415c_cb41_11d1_8b02_00600806d9b6);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISWbemServices_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Get: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strobjectpath: ::std::mem::MaybeUninit<::windows::core::BSTR>, iflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void, objwbemobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Get: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemsink: *mut ::core::ffi::c_void, strobjectpath: ::std::mem::MaybeUninit<::windows::core::BSTR>, iflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void, objwbemasynccontext: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetAsync: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Delete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strobjectpath: ::std::mem::MaybeUninit<::windows::core::BSTR>, iflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Delete: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub DeleteAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemsink: *mut ::core::ffi::c_void, strobjectpath: ::std::mem::MaybeUninit<::windows::core::BSTR>, iflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void, objwbemasynccontext: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    DeleteAsync: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub InstancesOf: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strclass: ::std::mem::MaybeUninit<::windows::core::BSTR>, iflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void, objwbemobjectset: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    InstancesOf: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub InstancesOfAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemsink: *mut ::core::ffi::c_void, strclass: ::std::mem::MaybeUninit<::windows::core::BSTR>, iflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void, objwbemasynccontext: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    InstancesOfAsync: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SubclassesOf: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strsuperclass: ::std::mem::MaybeUninit<::windows::core::BSTR>, iflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void, objwbemobjectset: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SubclassesOf: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SubclassesOfAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemsink: *mut ::core::ffi::c_void, strsuperclass: ::std::mem::MaybeUninit<::windows::core::BSTR>, iflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void, objwbemasynccontext: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SubclassesOfAsync: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ExecQuery: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strquery: ::std::mem::MaybeUninit<::windows::core::BSTR>, strquerylanguage: ::std::mem::MaybeUninit<::windows::core::BSTR>, iflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void, objwbemobjectset: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ExecQuery: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ExecQueryAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemsink: *mut ::core::ffi::c_void, strquery: ::std::mem::MaybeUninit<::windows::core::BSTR>, strquerylanguage: ::std::mem::MaybeUninit<::windows::core::BSTR>, lflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void, objwbemasynccontext: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ExecQueryAsync: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub AssociatorsOf: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strobjectpath: ::std::mem::MaybeUninit<::windows::core::BSTR>, strassocclass: ::std::mem::MaybeUninit<::windows::core::BSTR>, strresultclass: ::std::mem::MaybeUninit<::windows::core::BSTR>, strresultrole: ::std::mem::MaybeUninit<::windows::core::BSTR>, strrole: ::std::mem::MaybeUninit<::windows::core::BSTR>, bclassesonly: super::super::Foundation::VARIANT_BOOL, bschemaonly: super::super::Foundation::VARIANT_BOOL, strrequiredassocqualifier: ::std::mem::MaybeUninit<::windows::core::BSTR>, strrequiredqualifier: ::std::mem::MaybeUninit<::windows::core::BSTR>, iflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void, objwbemobjectset: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    AssociatorsOf: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub AssociatorsOfAsync: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        objwbemsink: *mut ::core::ffi::c_void,
        strobjectpath: ::std::mem::MaybeUninit<::windows::core::BSTR>,
        strassocclass: ::std::mem::MaybeUninit<::windows::core::BSTR>,
        strresultclass: ::std::mem::MaybeUninit<::windows::core::BSTR>,
        strresultrole: ::std::mem::MaybeUninit<::windows::core::BSTR>,
        strrole: ::std::mem::MaybeUninit<::windows::core::BSTR>,
        bclassesonly: super::super::Foundation::VARIANT_BOOL,
        bschemaonly: super::super::Foundation::VARIANT_BOOL,
        strrequiredassocqualifier: ::std::mem::MaybeUninit<::windows::core::BSTR>,
        strrequiredqualifier: ::std::mem::MaybeUninit<::windows::core::BSTR>,
        iflags: i32,
        objwbemnamedvalueset: *mut ::core::ffi::c_void,
        objwbemasynccontext: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    AssociatorsOfAsync: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub ReferencesTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strobjectpath: ::std::mem::MaybeUninit<::windows::core::BSTR>, strresultclass: ::std::mem::MaybeUninit<::windows::core::BSTR>, strrole: ::std::mem::MaybeUninit<::windows::core::BSTR>, bclassesonly: super::super::Foundation::VARIANT_BOOL, bschemaonly: super::super::Foundation::VARIANT_BOOL, strrequiredqualifier: ::std::mem::MaybeUninit<::windows::core::BSTR>, iflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void, objwbemobjectset: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    ReferencesTo: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub ReferencesToAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemsink: *mut ::core::ffi::c_void, strobjectpath: ::std::mem::MaybeUninit<::windows::core::BSTR>, strresultclass: ::std::mem::MaybeUninit<::windows::core::BSTR>, strrole: ::std::mem::MaybeUninit<::windows::core::BSTR>, bclassesonly: super::super::Foundation::VARIANT_BOOL, bschemaonly: super::super::Foundation::VARIANT_BOOL, strrequiredqualifier: ::std::mem::MaybeUninit<::windows::core::BSTR>, iflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void, objwbemasynccontext: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    ReferencesToAsync: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ExecNotificationQuery: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strquery: ::std::mem::MaybeUninit<::windows::core::BSTR>, strquerylanguage: ::std::mem::MaybeUninit<::windows::core::BSTR>, iflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void, objwbemeventsource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ExecNotificationQuery: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ExecNotificationQueryAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemsink: *mut ::core::ffi::c_void, strquery: ::std::mem::MaybeUninit<::windows::core::BSTR>, strquerylanguage: ::std::mem::MaybeUninit<::windows::core::BSTR>, iflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void, objwbemasynccontext: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ExecNotificationQueryAsync: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ExecMethod: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strobjectpath: ::std::mem::MaybeUninit<::windows::core::BSTR>, strmethodname: ::std::mem::MaybeUninit<::windows::core::BSTR>, objwbeminparameters: *mut ::core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void, objwbemoutparameters: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ExecMethod: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ExecMethodAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemsink: *mut ::core::ffi::c_void, strobjectpath: ::std::mem::MaybeUninit<::windows::core::BSTR>, strmethodname: ::std::mem::MaybeUninit<::windows::core::BSTR>, objwbeminparameters: *mut ::core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void, objwbemasynccontext: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ExecMethodAsync: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Security_: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemsecurity: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Security_: usize,
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISWbemServicesEx(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISWbemServicesEx {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Get<P0, P1>(&self, strobjectpath: P0, iflags: i32, objwbemnamedvalueset: P1) -> ::windows::core::Result<ISWbemObject>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<super::Com::IDispatch>,
    {
        let mut result__ = ::windows::core::zeroed::<ISWbemObject>();
        (::windows::core::Interface::vtable(self).base__.Get)(::windows::core::Interface::as_raw(self), strobjectpath.into_param().abi(), iflags, objwbemnamedvalueset.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetAsync<P0, P1, P2, P3>(&self, objwbemsink: P0, strobjectpath: P1, iflags: i32, objwbemnamedvalueset: P2, objwbemasynccontext: P3) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::Com::IDispatch>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
        P2: ::windows::core::IntoParam<super::Com::IDispatch>,
        P3: ::windows::core::IntoParam<super::Com::IDispatch>,
    {
        (::windows::core::Interface::vtable(self).base__.GetAsync)(::windows::core::Interface::as_raw(self), objwbemsink.into_param().abi(), strobjectpath.into_param().abi(), iflags, objwbemnamedvalueset.into_param().abi(), objwbemasynccontext.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Delete<P0, P1>(&self, strobjectpath: P0, iflags: i32, objwbemnamedvalueset: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<super::Com::IDispatch>,
    {
        (::windows::core::Interface::vtable(self).base__.Delete)(::windows::core::Interface::as_raw(self), strobjectpath.into_param().abi(), iflags, objwbemnamedvalueset.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn DeleteAsync<P0, P1, P2, P3>(&self, objwbemsink: P0, strobjectpath: P1, iflags: i32, objwbemnamedvalueset: P2, objwbemasynccontext: P3) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::Com::IDispatch>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
        P2: ::windows::core::IntoParam<super::Com::IDispatch>,
        P3: ::windows::core::IntoParam<super::Com::IDispatch>,
    {
        (::windows::core::Interface::vtable(self).base__.DeleteAsync)(::windows::core::Interface::as_raw(self), objwbemsink.into_param().abi(), strobjectpath.into_param().abi(), iflags, objwbemnamedvalueset.into_param().abi(), objwbemasynccontext.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InstancesOf<P0, P1>(&self, strclass: P0, iflags: i32, objwbemnamedvalueset: P1) -> ::windows::core::Result<ISWbemObjectSet>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<super::Com::IDispatch>,
    {
        let mut result__ = ::windows::core::zeroed::<ISWbemObjectSet>();
        (::windows::core::Interface::vtable(self).base__.InstancesOf)(::windows::core::Interface::as_raw(self), strclass.into_param().abi(), iflags, objwbemnamedvalueset.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InstancesOfAsync<P0, P1, P2, P3>(&self, objwbemsink: P0, strclass: P1, iflags: i32, objwbemnamedvalueset: P2, objwbemasynccontext: P3) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::Com::IDispatch>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
        P2: ::windows::core::IntoParam<super::Com::IDispatch>,
        P3: ::windows::core::IntoParam<super::Com::IDispatch>,
    {
        (::windows::core::Interface::vtable(self).base__.InstancesOfAsync)(::windows::core::Interface::as_raw(self), objwbemsink.into_param().abi(), strclass.into_param().abi(), iflags, objwbemnamedvalueset.into_param().abi(), objwbemasynccontext.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SubclassesOf<P0, P1>(&self, strsuperclass: P0, iflags: i32, objwbemnamedvalueset: P1) -> ::windows::core::Result<ISWbemObjectSet>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<super::Com::IDispatch>,
    {
        let mut result__ = ::windows::core::zeroed::<ISWbemObjectSet>();
        (::windows::core::Interface::vtable(self).base__.SubclassesOf)(::windows::core::Interface::as_raw(self), strsuperclass.into_param().abi(), iflags, objwbemnamedvalueset.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SubclassesOfAsync<P0, P1, P2, P3>(&self, objwbemsink: P0, strsuperclass: P1, iflags: i32, objwbemnamedvalueset: P2, objwbemasynccontext: P3) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::Com::IDispatch>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
        P2: ::windows::core::IntoParam<super::Com::IDispatch>,
        P3: ::windows::core::IntoParam<super::Com::IDispatch>,
    {
        (::windows::core::Interface::vtable(self).base__.SubclassesOfAsync)(::windows::core::Interface::as_raw(self), objwbemsink.into_param().abi(), strsuperclass.into_param().abi(), iflags, objwbemnamedvalueset.into_param().abi(), objwbemasynccontext.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ExecQuery<P0, P1, P2>(&self, strquery: P0, strquerylanguage: P1, iflags: i32, objwbemnamedvalueset: P2) -> ::windows::core::Result<ISWbemObjectSet>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
        P2: ::windows::core::IntoParam<super::Com::IDispatch>,
    {
        let mut result__ = ::windows::core::zeroed::<ISWbemObjectSet>();
        (::windows::core::Interface::vtable(self).base__.ExecQuery)(::windows::core::Interface::as_raw(self), strquery.into_param().abi(), strquerylanguage.into_param().abi(), iflags, objwbemnamedvalueset.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ExecQueryAsync<P0, P1, P2, P3, P4>(&self, objwbemsink: P0, strquery: P1, strquerylanguage: P2, lflags: i32, objwbemnamedvalueset: P3, objwbemasynccontext: P4) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::Com::IDispatch>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
        P2: ::windows::core::IntoParam<::windows::core::BSTR>,
        P3: ::windows::core::IntoParam<super::Com::IDispatch>,
        P4: ::windows::core::IntoParam<super::Com::IDispatch>,
    {
        (::windows::core::Interface::vtable(self).base__.ExecQueryAsync)(::windows::core::Interface::as_raw(self), objwbemsink.into_param().abi(), strquery.into_param().abi(), strquerylanguage.into_param().abi(), lflags, objwbemnamedvalueset.into_param().abi(), objwbemasynccontext.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn AssociatorsOf<P0, P1, P2, P3, P4, P5, P6, P7, P8, P9>(&self, strobjectpath: P0, strassocclass: P1, strresultclass: P2, strresultrole: P3, strrole: P4, bclassesonly: P5, bschemaonly: P6, strrequiredassocqualifier: P7, strrequiredqualifier: P8, iflags: i32, objwbemnamedvalueset: P9) -> ::windows::core::Result<ISWbemObjectSet>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
        P2: ::windows::core::IntoParam<::windows::core::BSTR>,
        P3: ::windows::core::IntoParam<::windows::core::BSTR>,
        P4: ::windows::core::IntoParam<::windows::core::BSTR>,
        P5: ::windows::core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
        P6: ::windows::core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
        P7: ::windows::core::IntoParam<::windows::core::BSTR>,
        P8: ::windows::core::IntoParam<::windows::core::BSTR>,
        P9: ::windows::core::IntoParam<super::Com::IDispatch>,
    {
        let mut result__ = ::windows::core::zeroed::<ISWbemObjectSet>();
        (::windows::core::Interface::vtable(self).base__.AssociatorsOf)(::windows::core::Interface::as_raw(self), strobjectpath.into_param().abi(), strassocclass.into_param().abi(), strresultclass.into_param().abi(), strresultrole.into_param().abi(), strrole.into_param().abi(), bclassesonly.into_param().abi(), bschemaonly.into_param().abi(), strrequiredassocqualifier.into_param().abi(), strrequiredqualifier.into_param().abi(), iflags, objwbemnamedvalueset.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn AssociatorsOfAsync<P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11>(&self, objwbemsink: P0, strobjectpath: P1, strassocclass: P2, strresultclass: P3, strresultrole: P4, strrole: P5, bclassesonly: P6, bschemaonly: P7, strrequiredassocqualifier: P8, strrequiredqualifier: P9, iflags: i32, objwbemnamedvalueset: P10, objwbemasynccontext: P11) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::Com::IDispatch>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
        P2: ::windows::core::IntoParam<::windows::core::BSTR>,
        P3: ::windows::core::IntoParam<::windows::core::BSTR>,
        P4: ::windows::core::IntoParam<::windows::core::BSTR>,
        P5: ::windows::core::IntoParam<::windows::core::BSTR>,
        P6: ::windows::core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
        P7: ::windows::core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
        P8: ::windows::core::IntoParam<::windows::core::BSTR>,
        P9: ::windows::core::IntoParam<::windows::core::BSTR>,
        P10: ::windows::core::IntoParam<super::Com::IDispatch>,
        P11: ::windows::core::IntoParam<super::Com::IDispatch>,
    {
        (::windows::core::Interface::vtable(self).base__.AssociatorsOfAsync)(::windows::core::Interface::as_raw(self), objwbemsink.into_param().abi(), strobjectpath.into_param().abi(), strassocclass.into_param().abi(), strresultclass.into_param().abi(), strresultrole.into_param().abi(), strrole.into_param().abi(), bclassesonly.into_param().abi(), bschemaonly.into_param().abi(), strrequiredassocqualifier.into_param().abi(), strrequiredqualifier.into_param().abi(), iflags, objwbemnamedvalueset.into_param().abi(), objwbemasynccontext.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn ReferencesTo<P0, P1, P2, P3, P4, P5, P6>(&self, strobjectpath: P0, strresultclass: P1, strrole: P2, bclassesonly: P3, bschemaonly: P4, strrequiredqualifier: P5, iflags: i32, objwbemnamedvalueset: P6) -> ::windows::core::Result<ISWbemObjectSet>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
        P2: ::windows::core::IntoParam<::windows::core::BSTR>,
        P3: ::windows::core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
        P4: ::windows::core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
        P5: ::windows::core::IntoParam<::windows::core::BSTR>,
        P6: ::windows::core::IntoParam<super::Com::IDispatch>,
    {
        let mut result__ = ::windows::core::zeroed::<ISWbemObjectSet>();
        (::windows::core::Interface::vtable(self).base__.ReferencesTo)(::windows::core::Interface::as_raw(self), strobjectpath.into_param().abi(), strresultclass.into_param().abi(), strrole.into_param().abi(), bclassesonly.into_param().abi(), bschemaonly.into_param().abi(), strrequiredqualifier.into_param().abi(), iflags, objwbemnamedvalueset.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn ReferencesToAsync<P0, P1, P2, P3, P4, P5, P6, P7, P8>(&self, objwbemsink: P0, strobjectpath: P1, strresultclass: P2, strrole: P3, bclassesonly: P4, bschemaonly: P5, strrequiredqualifier: P6, iflags: i32, objwbemnamedvalueset: P7, objwbemasynccontext: P8) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::Com::IDispatch>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
        P2: ::windows::core::IntoParam<::windows::core::BSTR>,
        P3: ::windows::core::IntoParam<::windows::core::BSTR>,
        P4: ::windows::core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
        P5: ::windows::core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
        P6: ::windows::core::IntoParam<::windows::core::BSTR>,
        P7: ::windows::core::IntoParam<super::Com::IDispatch>,
        P8: ::windows::core::IntoParam<super::Com::IDispatch>,
    {
        (::windows::core::Interface::vtable(self).base__.ReferencesToAsync)(::windows::core::Interface::as_raw(self), objwbemsink.into_param().abi(), strobjectpath.into_param().abi(), strresultclass.into_param().abi(), strrole.into_param().abi(), bclassesonly.into_param().abi(), bschemaonly.into_param().abi(), strrequiredqualifier.into_param().abi(), iflags, objwbemnamedvalueset.into_param().abi(), objwbemasynccontext.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ExecNotificationQuery<P0, P1, P2>(&self, strquery: P0, strquerylanguage: P1, iflags: i32, objwbemnamedvalueset: P2) -> ::windows::core::Result<ISWbemEventSource>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
        P2: ::windows::core::IntoParam<super::Com::IDispatch>,
    {
        let mut result__ = ::windows::core::zeroed::<ISWbemEventSource>();
        (::windows::core::Interface::vtable(self).base__.ExecNotificationQuery)(::windows::core::Interface::as_raw(self), strquery.into_param().abi(), strquerylanguage.into_param().abi(), iflags, objwbemnamedvalueset.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ExecNotificationQueryAsync<P0, P1, P2, P3, P4>(&self, objwbemsink: P0, strquery: P1, strquerylanguage: P2, iflags: i32, objwbemnamedvalueset: P3, objwbemasynccontext: P4) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::Com::IDispatch>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
        P2: ::windows::core::IntoParam<::windows::core::BSTR>,
        P3: ::windows::core::IntoParam<super::Com::IDispatch>,
        P4: ::windows::core::IntoParam<super::Com::IDispatch>,
    {
        (::windows::core::Interface::vtable(self).base__.ExecNotificationQueryAsync)(::windows::core::Interface::as_raw(self), objwbemsink.into_param().abi(), strquery.into_param().abi(), strquerylanguage.into_param().abi(), iflags, objwbemnamedvalueset.into_param().abi(), objwbemasynccontext.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ExecMethod<P0, P1, P2, P3>(&self, strobjectpath: P0, strmethodname: P1, objwbeminparameters: P2, iflags: i32, objwbemnamedvalueset: P3) -> ::windows::core::Result<ISWbemObject>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
        P2: ::windows::core::IntoParam<super::Com::IDispatch>,
        P3: ::windows::core::IntoParam<super::Com::IDispatch>,
    {
        let mut result__ = ::windows::core::zeroed::<ISWbemObject>();
        (::windows::core::Interface::vtable(self).base__.ExecMethod)(::windows::core::Interface::as_raw(self), strobjectpath.into_param().abi(), strmethodname.into_param().abi(), objwbeminparameters.into_param().abi(), iflags, objwbemnamedvalueset.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ExecMethodAsync<P0, P1, P2, P3, P4, P5>(&self, objwbemsink: P0, strobjectpath: P1, strmethodname: P2, objwbeminparameters: P3, iflags: i32, objwbemnamedvalueset: P4, objwbemasynccontext: P5) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::Com::IDispatch>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
        P2: ::windows::core::IntoParam<::windows::core::BSTR>,
        P3: ::windows::core::IntoParam<super::Com::IDispatch>,
        P4: ::windows::core::IntoParam<super::Com::IDispatch>,
        P5: ::windows::core::IntoParam<super::Com::IDispatch>,
    {
        (::windows::core::Interface::vtable(self).base__.ExecMethodAsync)(::windows::core::Interface::as_raw(self), objwbemsink.into_param().abi(), strobjectpath.into_param().abi(), strmethodname.into_param().abi(), objwbeminparameters.into_param().abi(), iflags, objwbemnamedvalueset.into_param().abi(), objwbemasynccontext.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Security_(&self) -> ::windows::core::Result<ISWbemSecurity> {
        let mut result__ = ::windows::core::zeroed::<ISWbemSecurity>();
        (::windows::core::Interface::vtable(self).base__.Security_)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Put<P0, P1>(&self, objwbemobject: P0, iflags: i32, objwbemnamedvalueset: P1) -> ::windows::core::Result<ISWbemObjectPath>
    where
        P0: ::windows::core::IntoParam<ISWbemObjectEx>,
        P1: ::windows::core::IntoParam<super::Com::IDispatch>,
    {
        let mut result__ = ::windows::core::zeroed::<ISWbemObjectPath>();
        (::windows::core::Interface::vtable(self).Put)(::windows::core::Interface::as_raw(self), objwbemobject.into_param().abi(), iflags, objwbemnamedvalueset.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PutAsync<P0, P1, P2, P3>(&self, objwbemsink: P0, objwbemobject: P1, iflags: i32, objwbemnamedvalueset: P2, objwbemasynccontext: P3) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ISWbemSink>,
        P1: ::windows::core::IntoParam<ISWbemObjectEx>,
        P2: ::windows::core::IntoParam<super::Com::IDispatch>,
        P3: ::windows::core::IntoParam<super::Com::IDispatch>,
    {
        (::windows::core::Interface::vtable(self).PutAsync)(::windows::core::Interface::as_raw(self), objwbemsink.into_param().abi(), objwbemobject.into_param().abi(), iflags, objwbemnamedvalueset.into_param().abi(), objwbemasynccontext.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(ISWbemServicesEx, ::windows::core::IUnknown, super::Com::IDispatch, ISWbemServices);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISWbemServicesEx {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISWbemServicesEx {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISWbemServicesEx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISWbemServicesEx").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISWbemServicesEx {
    type Vtable = ISWbemServicesEx_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISWbemServicesEx {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for ISWbemServicesEx {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd2f68443_85dc_427e_91d8_366554cc754c);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISWbemServicesEx_Vtbl {
    pub base__: ISWbemServices_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Put: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemobject: *mut ::core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void, objwbemobjectpath: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Put: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub PutAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwbemsink: *mut ::core::ffi::c_void, objwbemobject: *mut ::core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void, objwbemasynccontext: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PutAsync: usize,
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISWbemSink(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISWbemSink {
    pub unsafe fn Cancel(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Cancel)(::windows::core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(ISWbemSink, ::windows::core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISWbemSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISWbemSink {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISWbemSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISWbemSink").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISWbemSink {
    type Vtable = ISWbemSink_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISWbemSink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for ISWbemSink {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x75718c9f_f029_11d1_a1ac_00c04fb6c223);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISWbemSink_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Cancel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISWbemSinkEvents(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISWbemSinkEvents {}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(ISWbemSinkEvents, ::windows::core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISWbemSinkEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISWbemSinkEvents {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISWbemSinkEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISWbemSinkEvents").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISWbemSinkEvents {
    type Vtable = ISWbemSinkEvents_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISWbemSinkEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for ISWbemSinkEvents {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x75718ca0_f029_11d1_a1ac_00c04fb6c223);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISWbemSinkEvents_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
#[repr(transparent)]
pub struct IUnsecuredApartment(::windows::core::IUnknown);
impl IUnsecuredApartment {
    pub unsafe fn CreateObjectStub<P0>(&self, pobject: P0) -> ::windows::core::Result<::windows::core::IUnknown>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        let mut result__ = ::windows::core::zeroed::<::windows::core::IUnknown>();
        (::windows::core::Interface::vtable(self).CreateObjectStub)(::windows::core::Interface::as_raw(self), pobject.into_param().abi(), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IUnsecuredApartment, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IUnsecuredApartment {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUnsecuredApartment {}
impl ::core::fmt::Debug for IUnsecuredApartment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUnsecuredApartment").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IUnsecuredApartment {
    type Vtable = IUnsecuredApartment_Vtbl;
}
impl ::core::clone::Clone for IUnsecuredApartment {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IUnsecuredApartment {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1cfaba8c_1523_11d1_ad79_00c04fd8fdff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUnsecuredApartment_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub CreateObjectStub: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pobject: *mut ::core::ffi::c_void, ppstub: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IWMIExtension(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWMIExtension {
    pub unsafe fn WMIObjectPath(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).WMIObjectPath)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetWMIObject(&self) -> ::windows::core::Result<ISWbemObject> {
        let mut result__ = ::windows::core::zeroed::<ISWbemObject>();
        (::windows::core::Interface::vtable(self).GetWMIObject)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetWMIServices(&self) -> ::windows::core::Result<ISWbemServices> {
        let mut result__ = ::windows::core::zeroed::<ISWbemServices>();
        (::windows::core::Interface::vtable(self).GetWMIServices)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IWMIExtension, ::windows::core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IWMIExtension {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IWMIExtension {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IWMIExtension {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMIExtension").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IWMIExtension {
    type Vtable = IWMIExtension_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IWMIExtension {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IWMIExtension {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xadc1f06e_5c7e_11d2_8b74_00104b2afb41);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWMIExtension_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub WMIObjectPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strwmiobjectpath: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetWMIObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwmiobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetWMIObject: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetWMIServices: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objwmiservices: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetWMIServices: usize,
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
#[repr(transparent)]
pub struct IWbemAddressResolution(::windows::core::IUnknown);
impl IWbemAddressResolution {
    pub unsafe fn Resolve<P0>(&self, wsznamespacepath: P0, wszaddresstype: ::windows::core::PWSTR, pdwaddresslength: *mut u32, pabbinaryaddress: *mut *mut u8) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).Resolve)(::windows::core::Interface::as_raw(self), wsznamespacepath.into_param().abi(), ::core::mem::transmute(wszaddresstype), pdwaddresslength, pabbinaryaddress).ok()
    }
}
::windows::imp::interface_hierarchy!(IWbemAddressResolution, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IWbemAddressResolution {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemAddressResolution {}
impl ::core::fmt::Debug for IWbemAddressResolution {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWbemAddressResolution").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWbemAddressResolution {
    type Vtable = IWbemAddressResolution_Vtbl;
}
impl ::core::clone::Clone for IWbemAddressResolution {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWbemAddressResolution {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf7ce2e12_8c90_11d1_9e7b_00c04fc324a8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemAddressResolution_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Resolve: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wsznamespacepath: ::windows::core::PCWSTR, wszaddresstype: ::windows::core::PWSTR, pdwaddresslength: *mut u32, pabbinaryaddress: *mut *mut u8) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
#[repr(transparent)]
pub struct IWbemBackupRestore(::windows::core::IUnknown);
impl IWbemBackupRestore {
    pub unsafe fn Backup<P0>(&self, strbackuptofile: P0, lflags: i32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).Backup)(::windows::core::Interface::as_raw(self), strbackuptofile.into_param().abi(), lflags).ok()
    }
    pub unsafe fn Restore<P0>(&self, strrestorefromfile: P0, lflags: i32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).Restore)(::windows::core::Interface::as_raw(self), strrestorefromfile.into_param().abi(), lflags).ok()
    }
}
::windows::imp::interface_hierarchy!(IWbemBackupRestore, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IWbemBackupRestore {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemBackupRestore {}
impl ::core::fmt::Debug for IWbemBackupRestore {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWbemBackupRestore").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWbemBackupRestore {
    type Vtable = IWbemBackupRestore_Vtbl;
}
impl ::core::clone::Clone for IWbemBackupRestore {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWbemBackupRestore {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc49e32c7_bc8b_11d2_85d4_00105a1f8304);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemBackupRestore_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Backup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strbackuptofile: ::windows::core::PCWSTR, lflags: i32) -> ::windows::core::HRESULT,
    pub Restore: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strrestorefromfile: ::windows::core::PCWSTR, lflags: i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
#[repr(transparent)]
pub struct IWbemBackupRestoreEx(::windows::core::IUnknown);
impl IWbemBackupRestoreEx {
    pub unsafe fn Backup<P0>(&self, strbackuptofile: P0, lflags: i32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.Backup)(::windows::core::Interface::as_raw(self), strbackuptofile.into_param().abi(), lflags).ok()
    }
    pub unsafe fn Restore<P0>(&self, strrestorefromfile: P0, lflags: i32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.Restore)(::windows::core::Interface::as_raw(self), strrestorefromfile.into_param().abi(), lflags).ok()
    }
    pub unsafe fn Pause(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Pause)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Resume(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Resume)(::windows::core::Interface::as_raw(self)).ok()
    }
}
::windows::imp::interface_hierarchy!(IWbemBackupRestoreEx, ::windows::core::IUnknown, IWbemBackupRestore);
impl ::core::cmp::PartialEq for IWbemBackupRestoreEx {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemBackupRestoreEx {}
impl ::core::fmt::Debug for IWbemBackupRestoreEx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWbemBackupRestoreEx").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWbemBackupRestoreEx {
    type Vtable = IWbemBackupRestoreEx_Vtbl;
}
impl ::core::clone::Clone for IWbemBackupRestoreEx {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWbemBackupRestoreEx {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa359dec5_e813_4834_8a2a_ba7f1d777d76);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemBackupRestoreEx_Vtbl {
    pub base__: IWbemBackupRestore_Vtbl,
    pub Pause: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Resume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
#[repr(transparent)]
pub struct IWbemCallResult(::windows::core::IUnknown);
impl IWbemCallResult {
    pub unsafe fn GetResultObject(&self, ltimeout: i32) -> ::windows::core::Result<IWbemClassObject> {
        let mut result__ = ::windows::core::zeroed::<IWbemClassObject>();
        (::windows::core::Interface::vtable(self).GetResultObject)(::windows::core::Interface::as_raw(self), ltimeout, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetResultString(&self, ltimeout: i32) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).GetResultString)(::windows::core::Interface::as_raw(self), ltimeout, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetResultServices(&self, ltimeout: i32) -> ::windows::core::Result<IWbemServices> {
        let mut result__ = ::windows::core::zeroed::<IWbemServices>();
        (::windows::core::Interface::vtable(self).GetResultServices)(::windows::core::Interface::as_raw(self), ltimeout, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetCallStatus(&self, ltimeout: i32) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).GetCallStatus)(::windows::core::Interface::as_raw(self), ltimeout, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IWbemCallResult, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IWbemCallResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemCallResult {}
impl ::core::fmt::Debug for IWbemCallResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWbemCallResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWbemCallResult {
    type Vtable = IWbemCallResult_Vtbl;
}
impl ::core::clone::Clone for IWbemCallResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWbemCallResult {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x44aca675_e8fc_11d0_a07c_00c04fb68820);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemCallResult_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetResultObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ltimeout: i32, ppresultobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetResultString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ltimeout: i32, pstrresultstring: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub GetResultServices: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ltimeout: i32, ppservices: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetCallStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ltimeout: i32, plstatus: *mut i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
#[repr(transparent)]
pub struct IWbemClassObject(::windows::core::IUnknown);
impl IWbemClassObject {
    pub unsafe fn GetQualifierSet(&self) -> ::windows::core::Result<IWbemQualifierSet> {
        let mut result__ = ::windows::core::zeroed::<IWbemQualifierSet>();
        (::windows::core::Interface::vtable(self).GetQualifierSet)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Get<P0>(&self, wszname: P0, lflags: i32, pval: *mut super::Com::VARIANT, ptype: ::core::option::Option<*mut i32>, plflavor: ::core::option::Option<*mut i32>) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).Get)(::windows::core::Interface::as_raw(self), wszname.into_param().abi(), lflags, pval, ::core::mem::transmute(ptype.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(plflavor.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Put<P0>(&self, wszname: P0, lflags: i32, pval: *const super::Com::VARIANT, r#type: i32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).Put)(::windows::core::Interface::as_raw(self), wszname.into_param().abi(), lflags, pval, r#type).ok()
    }
    pub unsafe fn Delete<P0>(&self, wszname: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).Delete)(::windows::core::Interface::as_raw(self), wszname.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetNames<P0>(&self, wszqualifiername: P0, lflags: i32, pqualifierval: *const super::Com::VARIANT) -> ::windows::core::Result<*mut super::Com::SAFEARRAY>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<*mut super::Com::SAFEARRAY>();
        (::windows::core::Interface::vtable(self).GetNames)(::windows::core::Interface::as_raw(self), wszqualifiername.into_param().abi(), lflags, pqualifierval, &mut result__).from_abi(result__)
    }
    pub unsafe fn BeginEnumeration(&self, lenumflags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).BeginEnumeration)(::windows::core::Interface::as_raw(self), lenumflags).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Next(&self, lflags: i32, strname: *mut ::windows::core::BSTR, pval: *mut super::Com::VARIANT, ptype: *mut i32, plflavor: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Next)(::windows::core::Interface::as_raw(self), lflags, ::core::mem::transmute(strname), pval, ptype, plflavor).ok()
    }
    pub unsafe fn EndEnumeration(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EndEnumeration)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetPropertyQualifierSet<P0>(&self, wszproperty: P0) -> ::windows::core::Result<IWbemQualifierSet>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IWbemQualifierSet>();
        (::windows::core::Interface::vtable(self).GetPropertyQualifierSet)(::windows::core::Interface::as_raw(self), wszproperty.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IWbemClassObject> {
        let mut result__ = ::windows::core::zeroed::<IWbemClassObject>();
        (::windows::core::Interface::vtable(self).Clone)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetObjectText(&self, lflags: i32) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).GetObjectText)(::windows::core::Interface::as_raw(self), lflags, &mut result__).from_abi(result__)
    }
    pub unsafe fn SpawnDerivedClass(&self, lflags: i32) -> ::windows::core::Result<IWbemClassObject> {
        let mut result__ = ::windows::core::zeroed::<IWbemClassObject>();
        (::windows::core::Interface::vtable(self).SpawnDerivedClass)(::windows::core::Interface::as_raw(self), lflags, &mut result__).from_abi(result__)
    }
    pub unsafe fn SpawnInstance(&self, lflags: i32) -> ::windows::core::Result<IWbemClassObject> {
        let mut result__ = ::windows::core::zeroed::<IWbemClassObject>();
        (::windows::core::Interface::vtable(self).SpawnInstance)(::windows::core::Interface::as_raw(self), lflags, &mut result__).from_abi(result__)
    }
    pub unsafe fn CompareTo<P0>(&self, lflags: i32, pcompareto: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IWbemClassObject>,
    {
        (::windows::core::Interface::vtable(self).CompareTo)(::windows::core::Interface::as_raw(self), lflags, pcompareto.into_param().abi()).ok()
    }
    pub unsafe fn GetPropertyOrigin<P0>(&self, wszname: P0) -> ::windows::core::Result<::windows::core::BSTR>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).GetPropertyOrigin)(::windows::core::Interface::as_raw(self), wszname.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn InheritsFrom<P0>(&self, strancestor: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).InheritsFrom)(::windows::core::Interface::as_raw(self), strancestor.into_param().abi()).ok()
    }
    pub unsafe fn GetMethod<P0>(&self, wszname: P0, lflags: i32, ppinsignature: *mut ::core::option::Option<IWbemClassObject>, ppoutsignature: *mut ::core::option::Option<IWbemClassObject>) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).GetMethod)(::windows::core::Interface::as_raw(self), wszname.into_param().abi(), lflags, ::core::mem::transmute(ppinsignature), ::core::mem::transmute(ppoutsignature)).ok()
    }
    pub unsafe fn PutMethod<P0, P1, P2>(&self, wszname: P0, lflags: i32, pinsignature: P1, poutsignature: P2) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<IWbemClassObject>,
        P2: ::windows::core::IntoParam<IWbemClassObject>,
    {
        (::windows::core::Interface::vtable(self).PutMethod)(::windows::core::Interface::as_raw(self), wszname.into_param().abi(), lflags, pinsignature.into_param().abi(), poutsignature.into_param().abi()).ok()
    }
    pub unsafe fn DeleteMethod<P0>(&self, wszname: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).DeleteMethod)(::windows::core::Interface::as_raw(self), wszname.into_param().abi()).ok()
    }
    pub unsafe fn BeginMethodEnumeration(&self, lenumflags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).BeginMethodEnumeration)(::windows::core::Interface::as_raw(self), lenumflags).ok()
    }
    pub unsafe fn NextMethod(&self, lflags: i32, pstrname: *mut ::windows::core::BSTR, ppinsignature: *mut ::core::option::Option<IWbemClassObject>, ppoutsignature: *mut ::core::option::Option<IWbemClassObject>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).NextMethod)(::windows::core::Interface::as_raw(self), lflags, ::core::mem::transmute(pstrname), ::core::mem::transmute(ppinsignature), ::core::mem::transmute(ppoutsignature)).ok()
    }
    pub unsafe fn EndMethodEnumeration(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EndMethodEnumeration)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetMethodQualifierSet<P0>(&self, wszmethod: P0) -> ::windows::core::Result<IWbemQualifierSet>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IWbemQualifierSet>();
        (::windows::core::Interface::vtable(self).GetMethodQualifierSet)(::windows::core::Interface::as_raw(self), wszmethod.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetMethodOrigin<P0>(&self, wszmethodname: P0) -> ::windows::core::Result<::windows::core::BSTR>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).GetMethodOrigin)(::windows::core::Interface::as_raw(self), wszmethodname.into_param().abi(), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IWbemClassObject, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IWbemClassObject {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemClassObject {}
impl ::core::fmt::Debug for IWbemClassObject {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWbemClassObject").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWbemClassObject {
    type Vtable = IWbemClassObject_Vtbl;
}
impl ::core::clone::Clone for IWbemClassObject {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWbemClassObject {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdc12a681_737f_11cf_884d_00aa004b2e24);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemClassObject_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetQualifierSet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppqualset: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Get: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszname: ::windows::core::PCWSTR, lflags: i32, pval: *mut super::Com::VARIANT, ptype: *mut i32, plflavor: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Get: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Put: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszname: ::windows::core::PCWSTR, lflags: i32, pval: *const super::Com::VARIANT, r#type: i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Put: usize,
    pub Delete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetNames: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszqualifiername: ::windows::core::PCWSTR, lflags: i32, pqualifierval: *const super::Com::VARIANT, pnames: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetNames: usize,
    pub BeginEnumeration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lenumflags: i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, strname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, pval: *mut super::Com::VARIANT, ptype: *mut i32, plflavor: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Next: usize,
    pub EndEnumeration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetPropertyQualifierSet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszproperty: ::windows::core::PCWSTR, ppqualset: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcopy: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetObjectText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, pstrobjecttext: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub SpawnDerivedClass: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, ppnewclass: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SpawnInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, ppnewinstance: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CompareTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, pcompareto: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetPropertyOrigin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszname: ::windows::core::PCWSTR, pstrclassname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub InheritsFrom: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strancestor: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub GetMethod: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszname: ::windows::core::PCWSTR, lflags: i32, ppinsignature: *mut *mut ::core::ffi::c_void, ppoutsignature: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub PutMethod: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszname: ::windows::core::PCWSTR, lflags: i32, pinsignature: *mut ::core::ffi::c_void, poutsignature: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DeleteMethod: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub BeginMethodEnumeration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lenumflags: i32) -> ::windows::core::HRESULT,
    pub NextMethod: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, pstrname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, ppinsignature: *mut *mut ::core::ffi::c_void, ppoutsignature: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub EndMethodEnumeration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetMethodQualifierSet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszmethod: ::windows::core::PCWSTR, ppqualset: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetMethodOrigin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszmethodname: ::windows::core::PCWSTR, pstrclassname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
#[repr(transparent)]
pub struct IWbemClientConnectionTransport(::windows::core::IUnknown);
impl IWbemClientConnectionTransport {
    pub unsafe fn Open<P0, P1, P2, P3, P4, P5, T>(&self, straddresstype: P0, abbinaryaddress: &[u8], strobject: P1, struser: P2, strpassword: P3, strlocale: P4, lflags: i32, pctx: P5, pcallres: *mut ::core::option::Option<IWbemCallResult>) -> ::windows::core::Result<T>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
        P2: ::windows::core::IntoParam<::windows::core::BSTR>,
        P3: ::windows::core::IntoParam<::windows::core::BSTR>,
        P4: ::windows::core::IntoParam<::windows::core::BSTR>,
        P5: ::windows::core::IntoParam<IWbemContext>,
        T: ::windows::core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows::core::Interface::vtable(self).Open)(::windows::core::Interface::as_raw(self), straddresstype.into_param().abi(), abbinaryaddress.len() as _, ::core::mem::transmute(abbinaryaddress.as_ptr()), strobject.into_param().abi(), struser.into_param().abi(), strpassword.into_param().abi(), strlocale.into_param().abi(), lflags, pctx.into_param().abi(), &<T as ::windows::core::ComInterface>::IID, &mut result__, ::core::mem::transmute(pcallres)).from_abi(result__)
    }
    pub unsafe fn OpenAsync<P0, P1, P2, P3, P4, P5, P6>(&self, straddresstype: P0, abbinaryaddress: &[u8], strobject: P1, struser: P2, strpassword: P3, strlocale: P4, lflags: i32, pctx: P5, riid: *const ::windows::core::GUID, presponsehandler: P6) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
        P2: ::windows::core::IntoParam<::windows::core::BSTR>,
        P3: ::windows::core::IntoParam<::windows::core::BSTR>,
        P4: ::windows::core::IntoParam<::windows::core::BSTR>,
        P5: ::windows::core::IntoParam<IWbemContext>,
        P6: ::windows::core::IntoParam<IWbemObjectSink>,
    {
        (::windows::core::Interface::vtable(self).OpenAsync)(::windows::core::Interface::as_raw(self), straddresstype.into_param().abi(), abbinaryaddress.len() as _, ::core::mem::transmute(abbinaryaddress.as_ptr()), strobject.into_param().abi(), struser.into_param().abi(), strpassword.into_param().abi(), strlocale.into_param().abi(), lflags, pctx.into_param().abi(), riid, presponsehandler.into_param().abi()).ok()
    }
    pub unsafe fn Cancel<P0>(&self, lflags: i32, phandler: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IWbemObjectSink>,
    {
        (::windows::core::Interface::vtable(self).Cancel)(::windows::core::Interface::as_raw(self), lflags, phandler.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(IWbemClientConnectionTransport, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IWbemClientConnectionTransport {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemClientConnectionTransport {}
impl ::core::fmt::Debug for IWbemClientConnectionTransport {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWbemClientConnectionTransport").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWbemClientConnectionTransport {
    type Vtable = IWbemClientConnectionTransport_Vtbl;
}
impl ::core::clone::Clone for IWbemClientConnectionTransport {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWbemClientConnectionTransport {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa889c72a_fcc1_4a9e_af61_ed071333fb5b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemClientConnectionTransport_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Open: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, straddresstype: ::std::mem::MaybeUninit<::windows::core::BSTR>, dwbinaryaddresslength: u32, abbinaryaddress: *const u8, strobject: ::std::mem::MaybeUninit<::windows::core::BSTR>, struser: ::std::mem::MaybeUninit<::windows::core::BSTR>, strpassword: ::std::mem::MaybeUninit<::windows::core::BSTR>, strlocale: ::std::mem::MaybeUninit<::windows::core::BSTR>, lflags: i32, pctx: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, pinterface: *mut *mut ::core::ffi::c_void, pcallres: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub OpenAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, straddresstype: ::std::mem::MaybeUninit<::windows::core::BSTR>, dwbinaryaddresslength: u32, abbinaryaddress: *const u8, strobject: ::std::mem::MaybeUninit<::windows::core::BSTR>, struser: ::std::mem::MaybeUninit<::windows::core::BSTR>, strpassword: ::std::mem::MaybeUninit<::windows::core::BSTR>, strlocale: ::std::mem::MaybeUninit<::windows::core::BSTR>, lflags: i32, pctx: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, presponsehandler: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Cancel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, phandler: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
#[repr(transparent)]
pub struct IWbemClientTransport(::windows::core::IUnknown);
impl IWbemClientTransport {
    pub unsafe fn ConnectServer<P0, P1, P2, P3, P4, P5, P6>(&self, straddresstype: P0, abbinaryaddress: &[u8], strnetworkresource: P1, struser: P2, strpassword: P3, strlocale: P4, lsecurityflags: i32, strauthority: P5, pctx: P6) -> ::windows::core::Result<IWbemServices>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
        P2: ::windows::core::IntoParam<::windows::core::BSTR>,
        P3: ::windows::core::IntoParam<::windows::core::BSTR>,
        P4: ::windows::core::IntoParam<::windows::core::BSTR>,
        P5: ::windows::core::IntoParam<::windows::core::BSTR>,
        P6: ::windows::core::IntoParam<IWbemContext>,
    {
        let mut result__ = ::windows::core::zeroed::<IWbemServices>();
        (::windows::core::Interface::vtable(self).ConnectServer)(::windows::core::Interface::as_raw(self), straddresstype.into_param().abi(), abbinaryaddress.len() as _, ::core::mem::transmute(abbinaryaddress.as_ptr()), strnetworkresource.into_param().abi(), struser.into_param().abi(), strpassword.into_param().abi(), strlocale.into_param().abi(), lsecurityflags, strauthority.into_param().abi(), pctx.into_param().abi(), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IWbemClientTransport, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IWbemClientTransport {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemClientTransport {}
impl ::core::fmt::Debug for IWbemClientTransport {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWbemClientTransport").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWbemClientTransport {
    type Vtable = IWbemClientTransport_Vtbl;
}
impl ::core::clone::Clone for IWbemClientTransport {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWbemClientTransport {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf7ce2e11_8c90_11d1_9e7b_00c04fc324a8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemClientTransport_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub ConnectServer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, straddresstype: ::std::mem::MaybeUninit<::windows::core::BSTR>, dwbinaryaddresslength: u32, abbinaryaddress: *const u8, strnetworkresource: ::std::mem::MaybeUninit<::windows::core::BSTR>, struser: ::std::mem::MaybeUninit<::windows::core::BSTR>, strpassword: ::std::mem::MaybeUninit<::windows::core::BSTR>, strlocale: ::std::mem::MaybeUninit<::windows::core::BSTR>, lsecurityflags: i32, strauthority: ::std::mem::MaybeUninit<::windows::core::BSTR>, pctx: *mut ::core::ffi::c_void, ppnamespace: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
#[repr(transparent)]
pub struct IWbemConfigureRefresher(::windows::core::IUnknown);
impl IWbemConfigureRefresher {
    pub unsafe fn AddObjectByPath<P0, P1, P2>(&self, pnamespace: P0, wszpath: P1, lflags: i32, pcontext: P2, pprefreshable: *mut ::core::option::Option<IWbemClassObject>, plid: *mut i32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IWbemServices>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P2: ::windows::core::IntoParam<IWbemContext>,
    {
        (::windows::core::Interface::vtable(self).AddObjectByPath)(::windows::core::Interface::as_raw(self), pnamespace.into_param().abi(), wszpath.into_param().abi(), lflags, pcontext.into_param().abi(), ::core::mem::transmute(pprefreshable), plid).ok()
    }
    pub unsafe fn AddObjectByTemplate<P0, P1, P2>(&self, pnamespace: P0, ptemplate: P1, lflags: i32, pcontext: P2, pprefreshable: *mut ::core::option::Option<IWbemClassObject>, plid: *mut i32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IWbemServices>,
        P1: ::windows::core::IntoParam<IWbemClassObject>,
        P2: ::windows::core::IntoParam<IWbemContext>,
    {
        (::windows::core::Interface::vtable(self).AddObjectByTemplate)(::windows::core::Interface::as_raw(self), pnamespace.into_param().abi(), ptemplate.into_param().abi(), lflags, pcontext.into_param().abi(), ::core::mem::transmute(pprefreshable), plid).ok()
    }
    pub unsafe fn AddRefresher<P0>(&self, prefresher: P0, lflags: i32, plid: *mut i32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IWbemRefresher>,
    {
        (::windows::core::Interface::vtable(self).AddRefresher)(::windows::core::Interface::as_raw(self), prefresher.into_param().abi(), lflags, plid).ok()
    }
    pub unsafe fn Remove(&self, lid: i32, lflags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Remove)(::windows::core::Interface::as_raw(self), lid, lflags).ok()
    }
    pub unsafe fn AddEnum<P0, P1, P2>(&self, pnamespace: P0, wszclassname: P1, lflags: i32, pcontext: P2, ppenum: *mut ::core::option::Option<IWbemHiPerfEnum>, plid: *mut i32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IWbemServices>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P2: ::windows::core::IntoParam<IWbemContext>,
    {
        (::windows::core::Interface::vtable(self).AddEnum)(::windows::core::Interface::as_raw(self), pnamespace.into_param().abi(), wszclassname.into_param().abi(), lflags, pcontext.into_param().abi(), ::core::mem::transmute(ppenum), plid).ok()
    }
}
::windows::imp::interface_hierarchy!(IWbemConfigureRefresher, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IWbemConfigureRefresher {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemConfigureRefresher {}
impl ::core::fmt::Debug for IWbemConfigureRefresher {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWbemConfigureRefresher").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWbemConfigureRefresher {
    type Vtable = IWbemConfigureRefresher_Vtbl;
}
impl ::core::clone::Clone for IWbemConfigureRefresher {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWbemConfigureRefresher {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x49353c92_516b_11d1_aea6_00c04fb68820);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemConfigureRefresher_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub AddObjectByPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnamespace: *mut ::core::ffi::c_void, wszpath: ::windows::core::PCWSTR, lflags: i32, pcontext: *mut ::core::ffi::c_void, pprefreshable: *mut *mut ::core::ffi::c_void, plid: *mut i32) -> ::windows::core::HRESULT,
    pub AddObjectByTemplate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnamespace: *mut ::core::ffi::c_void, ptemplate: *mut ::core::ffi::c_void, lflags: i32, pcontext: *mut ::core::ffi::c_void, pprefreshable: *mut *mut ::core::ffi::c_void, plid: *mut i32) -> ::windows::core::HRESULT,
    pub AddRefresher: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prefresher: *mut ::core::ffi::c_void, lflags: i32, plid: *mut i32) -> ::windows::core::HRESULT,
    pub Remove: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lid: i32, lflags: i32) -> ::windows::core::HRESULT,
    pub AddEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnamespace: *mut ::core::ffi::c_void, wszclassname: ::windows::core::PCWSTR, lflags: i32, pcontext: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void, plid: *mut i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
#[repr(transparent)]
pub struct IWbemConnectorLogin(::windows::core::IUnknown);
impl IWbemConnectorLogin {
    pub unsafe fn ConnectorLogin<P0, P1, P2, T>(&self, wsznetworkresource: P0, wszpreferredlocale: P1, lflags: i32, pctx: P2) -> ::windows::core::Result<T>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P2: ::windows::core::IntoParam<IWbemContext>,
        T: ::windows::core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows::core::Interface::vtable(self).ConnectorLogin)(::windows::core::Interface::as_raw(self), wsznetworkresource.into_param().abi(), wszpreferredlocale.into_param().abi(), lflags, pctx.into_param().abi(), &<T as ::windows::core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IWbemConnectorLogin, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IWbemConnectorLogin {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemConnectorLogin {}
impl ::core::fmt::Debug for IWbemConnectorLogin {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWbemConnectorLogin").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWbemConnectorLogin {
    type Vtable = IWbemConnectorLogin_Vtbl;
}
impl ::core::clone::Clone for IWbemConnectorLogin {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWbemConnectorLogin {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd8ec9cb1_b135_4f10_8b1b_c7188bb0d186);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemConnectorLogin_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub ConnectorLogin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wsznetworkresource: ::windows::core::PCWSTR, wszpreferredlocale: ::windows::core::PCWSTR, lflags: i32, pctx: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, pinterface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
#[repr(transparent)]
pub struct IWbemConstructClassObject(::windows::core::IUnknown);
impl IWbemConstructClassObject {
    pub unsafe fn SetInheritanceChain(&self, lnumantecedents: i32, awszantecedents: *const ::windows::core::PCWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetInheritanceChain)(::windows::core::Interface::as_raw(self), lnumantecedents, awszantecedents).ok()
    }
    pub unsafe fn SetPropertyOrigin<P0>(&self, wszpropertyname: P0, loriginindex: i32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetPropertyOrigin)(::windows::core::Interface::as_raw(self), wszpropertyname.into_param().abi(), loriginindex).ok()
    }
    pub unsafe fn SetMethodOrigin<P0>(&self, wszmethodname: P0, loriginindex: i32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetMethodOrigin)(::windows::core::Interface::as_raw(self), wszmethodname.into_param().abi(), loriginindex).ok()
    }
    pub unsafe fn SetServerNamespace<P0, P1>(&self, wszserver: P0, wsznamespace: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetServerNamespace)(::windows::core::Interface::as_raw(self), wszserver.into_param().abi(), wsznamespace.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(IWbemConstructClassObject, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IWbemConstructClassObject {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemConstructClassObject {}
impl ::core::fmt::Debug for IWbemConstructClassObject {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWbemConstructClassObject").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWbemConstructClassObject {
    type Vtable = IWbemConstructClassObject_Vtbl;
}
impl ::core::clone::Clone for IWbemConstructClassObject {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWbemConstructClassObject {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9ef76194_70d5_11d1_ad90_00c04fd8fdff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemConstructClassObject_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub SetInheritanceChain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lnumantecedents: i32, awszantecedents: *const ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub SetPropertyOrigin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszpropertyname: ::windows::core::PCWSTR, loriginindex: i32) -> ::windows::core::HRESULT,
    pub SetMethodOrigin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszmethodname: ::windows::core::PCWSTR, loriginindex: i32) -> ::windows::core::HRESULT,
    pub SetServerNamespace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszserver: ::windows::core::PCWSTR, wsznamespace: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
#[repr(transparent)]
pub struct IWbemContext(::windows::core::IUnknown);
impl IWbemContext {
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IWbemContext> {
        let mut result__ = ::windows::core::zeroed::<IWbemContext>();
        (::windows::core::Interface::vtable(self).Clone)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetNames(&self, lflags: i32) -> ::windows::core::Result<*mut super::Com::SAFEARRAY> {
        let mut result__ = ::windows::core::zeroed::<*mut super::Com::SAFEARRAY>();
        (::windows::core::Interface::vtable(self).GetNames)(::windows::core::Interface::as_raw(self), lflags, &mut result__).from_abi(result__)
    }
    pub unsafe fn BeginEnumeration(&self, lflags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).BeginEnumeration)(::windows::core::Interface::as_raw(self), lflags).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Next(&self, lflags: i32, pstrname: *mut ::windows::core::BSTR, pvalue: *mut super::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Next)(::windows::core::Interface::as_raw(self), lflags, ::core::mem::transmute(pstrname), pvalue).ok()
    }
    pub unsafe fn EndEnumeration(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EndEnumeration)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetValue<P0>(&self, wszname: P0, lflags: i32, pvalue: *const super::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetValue)(::windows::core::Interface::as_raw(self), wszname.into_param().abi(), lflags, pvalue).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetValue<P0>(&self, wszname: P0, lflags: i32) -> ::windows::core::Result<super::Com::VARIANT>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<super::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).GetValue)(::windows::core::Interface::as_raw(self), wszname.into_param().abi(), lflags, &mut result__).from_abi(result__)
    }
    pub unsafe fn DeleteValue<P0>(&self, wszname: P0, lflags: i32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).DeleteValue)(::windows::core::Interface::as_raw(self), wszname.into_param().abi(), lflags).ok()
    }
    pub unsafe fn DeleteAll(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DeleteAll)(::windows::core::Interface::as_raw(self)).ok()
    }
}
::windows::imp::interface_hierarchy!(IWbemContext, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IWbemContext {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemContext {}
impl ::core::fmt::Debug for IWbemContext {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWbemContext").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWbemContext {
    type Vtable = IWbemContext_Vtbl;
}
impl ::core::clone::Clone for IWbemContext {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWbemContext {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x44aca674_e8fc_11d0_a07c_00c04fb68820);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemContext_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppnewcopy: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetNames: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, pnames: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetNames: usize,
    pub BeginEnumeration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, pstrname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, pvalue: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Next: usize,
    pub EndEnumeration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszname: ::windows::core::PCWSTR, lflags: i32, pvalue: *const super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetValue: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszname: ::windows::core::PCWSTR, lflags: i32, pvalue: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetValue: usize,
    pub DeleteValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszname: ::windows::core::PCWSTR, lflags: i32) -> ::windows::core::HRESULT,
    pub DeleteAll: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
#[repr(transparent)]
pub struct IWbemDecoupledBasicEventProvider(::windows::core::IUnknown);
impl IWbemDecoupledBasicEventProvider {
    pub unsafe fn Register<P0, P1, P2, P3, P4, P5>(&self, a_flags: i32, a_context: P0, a_user: P1, a_locale: P2, a_scope: P3, a_registration: P4, piunknown: P5) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IWbemContext>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P3: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P4: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P5: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).base__.Register)(::windows::core::Interface::as_raw(self), a_flags, a_context.into_param().abi(), a_user.into_param().abi(), a_locale.into_param().abi(), a_scope.into_param().abi(), a_registration.into_param().abi(), piunknown.into_param().abi()).ok()
    }
    pub unsafe fn UnRegister(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.UnRegister)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetSink<P0>(&self, a_flags: i32, a_context: P0) -> ::windows::core::Result<IWbemObjectSink>
    where
        P0: ::windows::core::IntoParam<IWbemContext>,
    {
        let mut result__ = ::windows::core::zeroed::<IWbemObjectSink>();
        (::windows::core::Interface::vtable(self).GetSink)(::windows::core::Interface::as_raw(self), a_flags, a_context.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetService<P0>(&self, a_flags: i32, a_context: P0) -> ::windows::core::Result<IWbemServices>
    where
        P0: ::windows::core::IntoParam<IWbemContext>,
    {
        let mut result__ = ::windows::core::zeroed::<IWbemServices>();
        (::windows::core::Interface::vtable(self).GetService)(::windows::core::Interface::as_raw(self), a_flags, a_context.into_param().abi(), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IWbemDecoupledBasicEventProvider, ::windows::core::IUnknown, IWbemDecoupledRegistrar);
impl ::core::cmp::PartialEq for IWbemDecoupledBasicEventProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemDecoupledBasicEventProvider {}
impl ::core::fmt::Debug for IWbemDecoupledBasicEventProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWbemDecoupledBasicEventProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWbemDecoupledBasicEventProvider {
    type Vtable = IWbemDecoupledBasicEventProvider_Vtbl;
}
impl ::core::clone::Clone for IWbemDecoupledBasicEventProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWbemDecoupledBasicEventProvider {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x86336d20_ca11_4786_9ef1_bc8a946b42fc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemDecoupledBasicEventProvider_Vtbl {
    pub base__: IWbemDecoupledRegistrar_Vtbl,
    pub GetSink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, a_flags: i32, a_context: *mut ::core::ffi::c_void, a_sink: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetService: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, a_flags: i32, a_context: *mut ::core::ffi::c_void, a_service: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
#[repr(transparent)]
pub struct IWbemDecoupledRegistrar(::windows::core::IUnknown);
impl IWbemDecoupledRegistrar {
    pub unsafe fn Register<P0, P1, P2, P3, P4, P5>(&self, a_flags: i32, a_context: P0, a_user: P1, a_locale: P2, a_scope: P3, a_registration: P4, piunknown: P5) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IWbemContext>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P3: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P4: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P5: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).Register)(::windows::core::Interface::as_raw(self), a_flags, a_context.into_param().abi(), a_user.into_param().abi(), a_locale.into_param().abi(), a_scope.into_param().abi(), a_registration.into_param().abi(), piunknown.into_param().abi()).ok()
    }
    pub unsafe fn UnRegister(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).UnRegister)(::windows::core::Interface::as_raw(self)).ok()
    }
}
::windows::imp::interface_hierarchy!(IWbemDecoupledRegistrar, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IWbemDecoupledRegistrar {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemDecoupledRegistrar {}
impl ::core::fmt::Debug for IWbemDecoupledRegistrar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWbemDecoupledRegistrar").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWbemDecoupledRegistrar {
    type Vtable = IWbemDecoupledRegistrar_Vtbl;
}
impl ::core::clone::Clone for IWbemDecoupledRegistrar {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWbemDecoupledRegistrar {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1005cbcf_e64f_4646_bcd3_3a089d8a84b4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemDecoupledRegistrar_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Register: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, a_flags: i32, a_context: *mut ::core::ffi::c_void, a_user: ::windows::core::PCWSTR, a_locale: ::windows::core::PCWSTR, a_scope: ::windows::core::PCWSTR, a_registration: ::windows::core::PCWSTR, piunknown: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub UnRegister: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
#[repr(transparent)]
pub struct IWbemEventConsumerProvider(::windows::core::IUnknown);
impl IWbemEventConsumerProvider {
    pub unsafe fn FindConsumer<P0>(&self, plogicalconsumer: P0) -> ::windows::core::Result<IWbemUnboundObjectSink>
    where
        P0: ::windows::core::IntoParam<IWbemClassObject>,
    {
        let mut result__ = ::windows::core::zeroed::<IWbemUnboundObjectSink>();
        (::windows::core::Interface::vtable(self).FindConsumer)(::windows::core::Interface::as_raw(self), plogicalconsumer.into_param().abi(), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IWbemEventConsumerProvider, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IWbemEventConsumerProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemEventConsumerProvider {}
impl ::core::fmt::Debug for IWbemEventConsumerProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWbemEventConsumerProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWbemEventConsumerProvider {
    type Vtable = IWbemEventConsumerProvider_Vtbl;
}
impl ::core::clone::Clone for IWbemEventConsumerProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWbemEventConsumerProvider {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe246107a_b06e_11d0_ad61_00c04fd8fdff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemEventConsumerProvider_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub FindConsumer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plogicalconsumer: *mut ::core::ffi::c_void, ppconsumer: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
#[repr(transparent)]
pub struct IWbemEventProvider(::windows::core::IUnknown);
impl IWbemEventProvider {
    pub unsafe fn ProvideEvents<P0>(&self, psink: P0, lflags: i32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IWbemObjectSink>,
    {
        (::windows::core::Interface::vtable(self).ProvideEvents)(::windows::core::Interface::as_raw(self), psink.into_param().abi(), lflags).ok()
    }
}
::windows::imp::interface_hierarchy!(IWbemEventProvider, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IWbemEventProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemEventProvider {}
impl ::core::fmt::Debug for IWbemEventProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWbemEventProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWbemEventProvider {
    type Vtable = IWbemEventProvider_Vtbl;
}
impl ::core::clone::Clone for IWbemEventProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWbemEventProvider {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe245105b_b06e_11d0_ad61_00c04fd8fdff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemEventProvider_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub ProvideEvents: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psink: *mut ::core::ffi::c_void, lflags: i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
#[repr(transparent)]
pub struct IWbemEventProviderQuerySink(::windows::core::IUnknown);
impl IWbemEventProviderQuerySink {
    pub unsafe fn NewQuery(&self, dwid: u32, wszquerylanguage: *const u16, wszquery: *const u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).NewQuery)(::windows::core::Interface::as_raw(self), dwid, wszquerylanguage, wszquery).ok()
    }
    pub unsafe fn CancelQuery(&self, dwid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CancelQuery)(::windows::core::Interface::as_raw(self), dwid).ok()
    }
}
::windows::imp::interface_hierarchy!(IWbemEventProviderQuerySink, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IWbemEventProviderQuerySink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemEventProviderQuerySink {}
impl ::core::fmt::Debug for IWbemEventProviderQuerySink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWbemEventProviderQuerySink").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWbemEventProviderQuerySink {
    type Vtable = IWbemEventProviderQuerySink_Vtbl;
}
impl ::core::clone::Clone for IWbemEventProviderQuerySink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWbemEventProviderQuerySink {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x580acaf8_fa1c_11d0_ad72_00c04fd8fdff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemEventProviderQuerySink_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub NewQuery: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwid: u32, wszquerylanguage: *const u16, wszquery: *const u16) -> ::windows::core::HRESULT,
    pub CancelQuery: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwid: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
#[repr(transparent)]
pub struct IWbemEventProviderSecurity(::windows::core::IUnknown);
impl IWbemEventProviderSecurity {
    pub unsafe fn AccessCheck(&self, wszquerylanguage: *const u16, wszquery: *const u16, psid: &[u8]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AccessCheck)(::windows::core::Interface::as_raw(self), wszquerylanguage, wszquery, psid.len() as _, ::core::mem::transmute(psid.as_ptr())).ok()
    }
}
::windows::imp::interface_hierarchy!(IWbemEventProviderSecurity, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IWbemEventProviderSecurity {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemEventProviderSecurity {}
impl ::core::fmt::Debug for IWbemEventProviderSecurity {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWbemEventProviderSecurity").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWbemEventProviderSecurity {
    type Vtable = IWbemEventProviderSecurity_Vtbl;
}
impl ::core::clone::Clone for IWbemEventProviderSecurity {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWbemEventProviderSecurity {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x631f7d96_d993_11d2_b339_00105a1f4aaf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemEventProviderSecurity_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub AccessCheck: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszquerylanguage: *const u16, wszquery: *const u16, lsidlength: i32, psid: *const u8) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
#[repr(transparent)]
pub struct IWbemEventSink(::windows::core::IUnknown);
impl IWbemEventSink {
    pub unsafe fn Indicate(&self, apobjarray: &[::core::option::Option<IWbemClassObject>]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Indicate)(::windows::core::Interface::as_raw(self), apobjarray.len() as _, ::core::mem::transmute(apobjarray.as_ptr())).ok()
    }
    pub unsafe fn SetStatus<P0, P1>(&self, lflags: i32, hresult: ::windows::core::HRESULT, strparam: P0, pobjparam: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<IWbemClassObject>,
    {
        (::windows::core::Interface::vtable(self).base__.SetStatus)(::windows::core::Interface::as_raw(self), lflags, hresult, strparam.into_param().abi(), pobjparam.into_param().abi()).ok()
    }
    pub unsafe fn SetSinkSecurity(&self, psd: &[u8]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSinkSecurity)(::windows::core::Interface::as_raw(self), psd.len() as _, ::core::mem::transmute(psd.as_ptr())).ok()
    }
    pub unsafe fn IsActive(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).IsActive)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetRestrictedSink<P0>(&self, awszqueries: &[::windows::core::PCWSTR], pcallback: P0) -> ::windows::core::Result<IWbemEventSink>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        let mut result__ = ::windows::core::zeroed::<IWbemEventSink>();
        (::windows::core::Interface::vtable(self).GetRestrictedSink)(::windows::core::Interface::as_raw(self), awszqueries.len() as _, ::core::mem::transmute(awszqueries.as_ptr()), pcallback.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetBatchingParameters(&self, lflags: i32, dwmaxbuffersize: u32, dwmaxsendlatency: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetBatchingParameters)(::windows::core::Interface::as_raw(self), lflags, dwmaxbuffersize, dwmaxsendlatency).ok()
    }
}
::windows::imp::interface_hierarchy!(IWbemEventSink, ::windows::core::IUnknown, IWbemObjectSink);
impl ::core::cmp::PartialEq for IWbemEventSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemEventSink {}
impl ::core::fmt::Debug for IWbemEventSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWbemEventSink").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWbemEventSink {
    type Vtable = IWbemEventSink_Vtbl;
}
impl ::core::clone::Clone for IWbemEventSink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWbemEventSink {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3ae0080a_7e3a_4366_bf89_0feedc931659);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemEventSink_Vtbl {
    pub base__: IWbemObjectSink_Vtbl,
    pub SetSinkSecurity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lsdlength: i32, psd: *const u8) -> ::windows::core::HRESULT,
    pub IsActive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetRestrictedSink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lnumqueries: i32, awszqueries: *const ::windows::core::PCWSTR, pcallback: *mut ::core::ffi::c_void, ppsink: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetBatchingParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, dwmaxbuffersize: u32, dwmaxsendlatency: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
#[repr(transparent)]
pub struct IWbemHiPerfEnum(::windows::core::IUnknown);
impl IWbemHiPerfEnum {
    pub unsafe fn AddObjects(&self, lflags: i32, unumobjects: u32, apids: *const i32, apobj: *const ::core::option::Option<IWbemObjectAccess>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AddObjects)(::windows::core::Interface::as_raw(self), lflags, unumobjects, apids, ::core::mem::transmute(apobj)).ok()
    }
    pub unsafe fn RemoveObjects(&self, lflags: i32, apids: &[i32]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RemoveObjects)(::windows::core::Interface::as_raw(self), lflags, apids.len() as _, ::core::mem::transmute(apids.as_ptr())).ok()
    }
    pub unsafe fn GetObjects(&self, lflags: i32, apobj: &mut [::core::option::Option<IWbemObjectAccess>], pureturned: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetObjects)(::windows::core::Interface::as_raw(self), lflags, apobj.len() as _, ::core::mem::transmute(apobj.as_ptr()), pureturned).ok()
    }
    pub unsafe fn RemoveAll(&self, lflags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RemoveAll)(::windows::core::Interface::as_raw(self), lflags).ok()
    }
}
::windows::imp::interface_hierarchy!(IWbemHiPerfEnum, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IWbemHiPerfEnum {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemHiPerfEnum {}
impl ::core::fmt::Debug for IWbemHiPerfEnum {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWbemHiPerfEnum").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWbemHiPerfEnum {
    type Vtable = IWbemHiPerfEnum_Vtbl;
}
impl ::core::clone::Clone for IWbemHiPerfEnum {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWbemHiPerfEnum {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2705c288_79ae_11d2_b348_00105a1f8177);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemHiPerfEnum_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub AddObjects: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, unumobjects: u32, apids: *const i32, apobj: *const *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RemoveObjects: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, unumobjects: u32, apids: *const i32) -> ::windows::core::HRESULT,
    pub GetObjects: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, unumobjects: u32, apobj: *mut *mut ::core::ffi::c_void, pureturned: *mut u32) -> ::windows::core::HRESULT,
    pub RemoveAll: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
#[repr(transparent)]
pub struct IWbemHiPerfProvider(::windows::core::IUnknown);
impl IWbemHiPerfProvider {
    pub unsafe fn QueryInstances<P0, P1, P2, P3>(&self, pnamespace: P0, wszclass: P1, lflags: i32, pctx: P2, psink: P3) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IWbemServices>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P2: ::windows::core::IntoParam<IWbemContext>,
        P3: ::windows::core::IntoParam<IWbemObjectSink>,
    {
        (::windows::core::Interface::vtable(self).QueryInstances)(::windows::core::Interface::as_raw(self), pnamespace.into_param().abi(), wszclass.into_param().abi(), lflags, pctx.into_param().abi(), psink.into_param().abi()).ok()
    }
    pub unsafe fn CreateRefresher<P0>(&self, pnamespace: P0, lflags: i32) -> ::windows::core::Result<IWbemRefresher>
    where
        P0: ::windows::core::IntoParam<IWbemServices>,
    {
        let mut result__ = ::windows::core::zeroed::<IWbemRefresher>();
        (::windows::core::Interface::vtable(self).CreateRefresher)(::windows::core::Interface::as_raw(self), pnamespace.into_param().abi(), lflags, &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateRefreshableObject<P0, P1, P2, P3>(&self, pnamespace: P0, ptemplate: P1, prefresher: P2, lflags: i32, pcontext: P3, pprefreshable: *mut ::core::option::Option<IWbemObjectAccess>, plid: *mut i32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IWbemServices>,
        P1: ::windows::core::IntoParam<IWbemObjectAccess>,
        P2: ::windows::core::IntoParam<IWbemRefresher>,
        P3: ::windows::core::IntoParam<IWbemContext>,
    {
        (::windows::core::Interface::vtable(self).CreateRefreshableObject)(::windows::core::Interface::as_raw(self), pnamespace.into_param().abi(), ptemplate.into_param().abi(), prefresher.into_param().abi(), lflags, pcontext.into_param().abi(), ::core::mem::transmute(pprefreshable), plid).ok()
    }
    pub unsafe fn StopRefreshing<P0>(&self, prefresher: P0, lid: i32, lflags: i32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IWbemRefresher>,
    {
        (::windows::core::Interface::vtable(self).StopRefreshing)(::windows::core::Interface::as_raw(self), prefresher.into_param().abi(), lid, lflags).ok()
    }
    pub unsafe fn CreateRefreshableEnum<P0, P1, P2, P3, P4>(&self, pnamespace: P0, wszclass: P1, prefresher: P2, lflags: i32, pcontext: P3, phiperfenum: P4) -> ::windows::core::Result<i32>
    where
        P0: ::windows::core::IntoParam<IWbemServices>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P2: ::windows::core::IntoParam<IWbemRefresher>,
        P3: ::windows::core::IntoParam<IWbemContext>,
        P4: ::windows::core::IntoParam<IWbemHiPerfEnum>,
    {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).CreateRefreshableEnum)(::windows::core::Interface::as_raw(self), pnamespace.into_param().abi(), wszclass.into_param().abi(), prefresher.into_param().abi(), lflags, pcontext.into_param().abi(), phiperfenum.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetObjects<P0, P1>(&self, pnamespace: P0, apobj: &mut [::core::option::Option<IWbemObjectAccess>], lflags: i32, pcontext: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IWbemServices>,
        P1: ::windows::core::IntoParam<IWbemContext>,
    {
        (::windows::core::Interface::vtable(self).GetObjects)(::windows::core::Interface::as_raw(self), pnamespace.into_param().abi(), apobj.len() as _, ::core::mem::transmute(apobj.as_ptr()), lflags, pcontext.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(IWbemHiPerfProvider, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IWbemHiPerfProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemHiPerfProvider {}
impl ::core::fmt::Debug for IWbemHiPerfProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWbemHiPerfProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWbemHiPerfProvider {
    type Vtable = IWbemHiPerfProvider_Vtbl;
}
impl ::core::clone::Clone for IWbemHiPerfProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWbemHiPerfProvider {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x49353c93_516b_11d1_aea6_00c04fb68820);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemHiPerfProvider_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub QueryInstances: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnamespace: *mut ::core::ffi::c_void, wszclass: ::windows::core::PCWSTR, lflags: i32, pctx: *mut ::core::ffi::c_void, psink: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateRefresher: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnamespace: *mut ::core::ffi::c_void, lflags: i32, pprefresher: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateRefreshableObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnamespace: *mut ::core::ffi::c_void, ptemplate: *mut ::core::ffi::c_void, prefresher: *mut ::core::ffi::c_void, lflags: i32, pcontext: *mut ::core::ffi::c_void, pprefreshable: *mut *mut ::core::ffi::c_void, plid: *mut i32) -> ::windows::core::HRESULT,
    pub StopRefreshing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prefresher: *mut ::core::ffi::c_void, lid: i32, lflags: i32) -> ::windows::core::HRESULT,
    pub CreateRefreshableEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnamespace: *mut ::core::ffi::c_void, wszclass: ::windows::core::PCWSTR, prefresher: *mut ::core::ffi::c_void, lflags: i32, pcontext: *mut ::core::ffi::c_void, phiperfenum: *mut ::core::ffi::c_void, plid: *mut i32) -> ::windows::core::HRESULT,
    pub GetObjects: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnamespace: *mut ::core::ffi::c_void, lnumobjects: i32, apobj: *mut *mut ::core::ffi::c_void, lflags: i32, pcontext: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
#[repr(transparent)]
pub struct IWbemLevel1Login(::windows::core::IUnknown);
impl IWbemLevel1Login {
    pub unsafe fn EstablishPosition<P0>(&self, wszlocalelist: P0, dwnumlocales: u32) -> ::windows::core::Result<u32>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).EstablishPosition)(::windows::core::Interface::as_raw(self), wszlocalelist.into_param().abi(), dwnumlocales, &mut result__).from_abi(result__)
    }
    pub unsafe fn RequestChallenge<P0, P1>(&self, wsznetworkresource: P0, wszuser: P1) -> ::windows::core::Result<u8>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<u8>();
        (::windows::core::Interface::vtable(self).RequestChallenge)(::windows::core::Interface::as_raw(self), wsznetworkresource.into_param().abi(), wszuser.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn WBEMLogin<P0, P1>(&self, wszpreferredlocale: P0, accesstoken: *const u8, lflags: i32, pctx: P1) -> ::windows::core::Result<IWbemServices>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<IWbemContext>,
    {
        let mut result__ = ::windows::core::zeroed::<IWbemServices>();
        (::windows::core::Interface::vtable(self).WBEMLogin)(::windows::core::Interface::as_raw(self), wszpreferredlocale.into_param().abi(), accesstoken, lflags, pctx.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn NTLMLogin<P0, P1, P2>(&self, wsznetworkresource: P0, wszpreferredlocale: P1, lflags: i32, pctx: P2) -> ::windows::core::Result<IWbemServices>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P2: ::windows::core::IntoParam<IWbemContext>,
    {
        let mut result__ = ::windows::core::zeroed::<IWbemServices>();
        (::windows::core::Interface::vtable(self).NTLMLogin)(::windows::core::Interface::as_raw(self), wsznetworkresource.into_param().abi(), wszpreferredlocale.into_param().abi(), lflags, pctx.into_param().abi(), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IWbemLevel1Login, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IWbemLevel1Login {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemLevel1Login {}
impl ::core::fmt::Debug for IWbemLevel1Login {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWbemLevel1Login").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWbemLevel1Login {
    type Vtable = IWbemLevel1Login_Vtbl;
}
impl ::core::clone::Clone for IWbemLevel1Login {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWbemLevel1Login {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf309ad18_d86a_11d0_a075_00c04fb68820);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemLevel1Login_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub EstablishPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszlocalelist: ::windows::core::PCWSTR, dwnumlocales: u32, reserved: *mut u32) -> ::windows::core::HRESULT,
    pub RequestChallenge: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wsznetworkresource: ::windows::core::PCWSTR, wszuser: ::windows::core::PCWSTR, nonce: *mut u8) -> ::windows::core::HRESULT,
    pub WBEMLogin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszpreferredlocale: ::windows::core::PCWSTR, accesstoken: *const u8, lflags: i32, pctx: *mut ::core::ffi::c_void, ppnamespace: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub NTLMLogin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wsznetworkresource: ::windows::core::PCWSTR, wszpreferredlocale: ::windows::core::PCWSTR, lflags: i32, pctx: *mut ::core::ffi::c_void, ppnamespace: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
#[repr(transparent)]
pub struct IWbemLocator(::windows::core::IUnknown);
impl IWbemLocator {
    pub unsafe fn ConnectServer<P0, P1, P2, P3, P4, P5>(&self, strnetworkresource: P0, struser: P1, strpassword: P2, strlocale: P3, lsecurityflags: i32, strauthority: P4, pctx: P5) -> ::windows::core::Result<IWbemServices>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
        P2: ::windows::core::IntoParam<::windows::core::BSTR>,
        P3: ::windows::core::IntoParam<::windows::core::BSTR>,
        P4: ::windows::core::IntoParam<::windows::core::BSTR>,
        P5: ::windows::core::IntoParam<IWbemContext>,
    {
        let mut result__ = ::windows::core::zeroed::<IWbemServices>();
        (::windows::core::Interface::vtable(self).ConnectServer)(::windows::core::Interface::as_raw(self), strnetworkresource.into_param().abi(), struser.into_param().abi(), strpassword.into_param().abi(), strlocale.into_param().abi(), lsecurityflags, strauthority.into_param().abi(), pctx.into_param().abi(), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IWbemLocator, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IWbemLocator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemLocator {}
impl ::core::fmt::Debug for IWbemLocator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWbemLocator").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWbemLocator {
    type Vtable = IWbemLocator_Vtbl;
}
impl ::core::clone::Clone for IWbemLocator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWbemLocator {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdc12a687_737f_11cf_884d_00aa004b2e24);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemLocator_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub ConnectServer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strnetworkresource: ::std::mem::MaybeUninit<::windows::core::BSTR>, struser: ::std::mem::MaybeUninit<::windows::core::BSTR>, strpassword: ::std::mem::MaybeUninit<::windows::core::BSTR>, strlocale: ::std::mem::MaybeUninit<::windows::core::BSTR>, lsecurityflags: i32, strauthority: ::std::mem::MaybeUninit<::windows::core::BSTR>, pctx: *mut ::core::ffi::c_void, ppnamespace: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
#[repr(transparent)]
pub struct IWbemObjectAccess(::windows::core::IUnknown);
impl IWbemObjectAccess {
    pub unsafe fn GetQualifierSet(&self) -> ::windows::core::Result<IWbemQualifierSet> {
        let mut result__ = ::windows::core::zeroed::<IWbemQualifierSet>();
        (::windows::core::Interface::vtable(self).base__.GetQualifierSet)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Get<P0>(&self, wszname: P0, lflags: i32, pval: *mut super::Com::VARIANT, ptype: ::core::option::Option<*mut i32>, plflavor: ::core::option::Option<*mut i32>) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.Get)(::windows::core::Interface::as_raw(self), wszname.into_param().abi(), lflags, pval, ::core::mem::transmute(ptype.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(plflavor.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Put<P0>(&self, wszname: P0, lflags: i32, pval: *const super::Com::VARIANT, r#type: i32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.Put)(::windows::core::Interface::as_raw(self), wszname.into_param().abi(), lflags, pval, r#type).ok()
    }
    pub unsafe fn Delete<P0>(&self, wszname: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.Delete)(::windows::core::Interface::as_raw(self), wszname.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetNames<P0>(&self, wszqualifiername: P0, lflags: i32, pqualifierval: *const super::Com::VARIANT) -> ::windows::core::Result<*mut super::Com::SAFEARRAY>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<*mut super::Com::SAFEARRAY>();
        (::windows::core::Interface::vtable(self).base__.GetNames)(::windows::core::Interface::as_raw(self), wszqualifiername.into_param().abi(), lflags, pqualifierval, &mut result__).from_abi(result__)
    }
    pub unsafe fn BeginEnumeration(&self, lenumflags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.BeginEnumeration)(::windows::core::Interface::as_raw(self), lenumflags).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Next(&self, lflags: i32, strname: *mut ::windows::core::BSTR, pval: *mut super::Com::VARIANT, ptype: *mut i32, plflavor: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Next)(::windows::core::Interface::as_raw(self), lflags, ::core::mem::transmute(strname), pval, ptype, plflavor).ok()
    }
    pub unsafe fn EndEnumeration(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.EndEnumeration)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetPropertyQualifierSet<P0>(&self, wszproperty: P0) -> ::windows::core::Result<IWbemQualifierSet>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IWbemQualifierSet>();
        (::windows::core::Interface::vtable(self).base__.GetPropertyQualifierSet)(::windows::core::Interface::as_raw(self), wszproperty.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IWbemClassObject> {
        let mut result__ = ::windows::core::zeroed::<IWbemClassObject>();
        (::windows::core::Interface::vtable(self).base__.Clone)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetObjectText(&self, lflags: i32) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.GetObjectText)(::windows::core::Interface::as_raw(self), lflags, &mut result__).from_abi(result__)
    }
    pub unsafe fn SpawnDerivedClass(&self, lflags: i32) -> ::windows::core::Result<IWbemClassObject> {
        let mut result__ = ::windows::core::zeroed::<IWbemClassObject>();
        (::windows::core::Interface::vtable(self).base__.SpawnDerivedClass)(::windows::core::Interface::as_raw(self), lflags, &mut result__).from_abi(result__)
    }
    pub unsafe fn SpawnInstance(&self, lflags: i32) -> ::windows::core::Result<IWbemClassObject> {
        let mut result__ = ::windows::core::zeroed::<IWbemClassObject>();
        (::windows::core::Interface::vtable(self).base__.SpawnInstance)(::windows::core::Interface::as_raw(self), lflags, &mut result__).from_abi(result__)
    }
    pub unsafe fn CompareTo<P0>(&self, lflags: i32, pcompareto: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IWbemClassObject>,
    {
        (::windows::core::Interface::vtable(self).base__.CompareTo)(::windows::core::Interface::as_raw(self), lflags, pcompareto.into_param().abi()).ok()
    }
    pub unsafe fn GetPropertyOrigin<P0>(&self, wszname: P0) -> ::windows::core::Result<::windows::core::BSTR>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.GetPropertyOrigin)(::windows::core::Interface::as_raw(self), wszname.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn InheritsFrom<P0>(&self, strancestor: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.InheritsFrom)(::windows::core::Interface::as_raw(self), strancestor.into_param().abi()).ok()
    }
    pub unsafe fn GetMethod<P0>(&self, wszname: P0, lflags: i32, ppinsignature: *mut ::core::option::Option<IWbemClassObject>, ppoutsignature: *mut ::core::option::Option<IWbemClassObject>) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.GetMethod)(::windows::core::Interface::as_raw(self), wszname.into_param().abi(), lflags, ::core::mem::transmute(ppinsignature), ::core::mem::transmute(ppoutsignature)).ok()
    }
    pub unsafe fn PutMethod<P0, P1, P2>(&self, wszname: P0, lflags: i32, pinsignature: P1, poutsignature: P2) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<IWbemClassObject>,
        P2: ::windows::core::IntoParam<IWbemClassObject>,
    {
        (::windows::core::Interface::vtable(self).base__.PutMethod)(::windows::core::Interface::as_raw(self), wszname.into_param().abi(), lflags, pinsignature.into_param().abi(), poutsignature.into_param().abi()).ok()
    }
    pub unsafe fn DeleteMethod<P0>(&self, wszname: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.DeleteMethod)(::windows::core::Interface::as_raw(self), wszname.into_param().abi()).ok()
    }
    pub unsafe fn BeginMethodEnumeration(&self, lenumflags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.BeginMethodEnumeration)(::windows::core::Interface::as_raw(self), lenumflags).ok()
    }
    pub unsafe fn NextMethod(&self, lflags: i32, pstrname: *mut ::windows::core::BSTR, ppinsignature: *mut ::core::option::Option<IWbemClassObject>, ppoutsignature: *mut ::core::option::Option<IWbemClassObject>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.NextMethod)(::windows::core::Interface::as_raw(self), lflags, ::core::mem::transmute(pstrname), ::core::mem::transmute(ppinsignature), ::core::mem::transmute(ppoutsignature)).ok()
    }
    pub unsafe fn EndMethodEnumeration(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.EndMethodEnumeration)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetMethodQualifierSet<P0>(&self, wszmethod: P0) -> ::windows::core::Result<IWbemQualifierSet>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IWbemQualifierSet>();
        (::windows::core::Interface::vtable(self).base__.GetMethodQualifierSet)(::windows::core::Interface::as_raw(self), wszmethod.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetMethodOrigin<P0>(&self, wszmethodname: P0) -> ::windows::core::Result<::windows::core::BSTR>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.GetMethodOrigin)(::windows::core::Interface::as_raw(self), wszmethodname.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetPropertyHandle<P0>(&self, wszpropertyname: P0, ptype: *mut i32, plhandle: *mut i32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).GetPropertyHandle)(::windows::core::Interface::as_raw(self), wszpropertyname.into_param().abi(), ptype, plhandle).ok()
    }
    pub unsafe fn WritePropertyValue(&self, lhandle: i32, adata: &[u8]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).WritePropertyValue)(::windows::core::Interface::as_raw(self), lhandle, adata.len() as _, ::core::mem::transmute(adata.as_ptr())).ok()
    }
    pub unsafe fn ReadPropertyValue(&self, lhandle: i32, plnumbytes: *mut i32, adata: &mut [u8]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ReadPropertyValue)(::windows::core::Interface::as_raw(self), lhandle, adata.len() as _, plnumbytes, ::core::mem::transmute(adata.as_ptr())).ok()
    }
    pub unsafe fn ReadDWORD(&self, lhandle: i32) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).ReadDWORD)(::windows::core::Interface::as_raw(self), lhandle, &mut result__).from_abi(result__)
    }
    pub unsafe fn WriteDWORD(&self, lhandle: i32, dw: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).WriteDWORD)(::windows::core::Interface::as_raw(self), lhandle, dw).ok()
    }
    pub unsafe fn ReadQWORD(&self, lhandle: i32) -> ::windows::core::Result<u64> {
        let mut result__ = ::windows::core::zeroed::<u64>();
        (::windows::core::Interface::vtable(self).ReadQWORD)(::windows::core::Interface::as_raw(self), lhandle, &mut result__).from_abi(result__)
    }
    pub unsafe fn WriteQWORD(&self, lhandle: i32, pw: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).WriteQWORD)(::windows::core::Interface::as_raw(self), lhandle, pw).ok()
    }
    pub unsafe fn GetPropertyInfoByHandle(&self, lhandle: i32, pstrname: *mut ::windows::core::BSTR, ptype: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetPropertyInfoByHandle)(::windows::core::Interface::as_raw(self), lhandle, ::core::mem::transmute(pstrname), ptype).ok()
    }
    pub unsafe fn Lock(&self, lflags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Lock)(::windows::core::Interface::as_raw(self), lflags).ok()
    }
    pub unsafe fn Unlock(&self, lflags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Unlock)(::windows::core::Interface::as_raw(self), lflags).ok()
    }
}
::windows::imp::interface_hierarchy!(IWbemObjectAccess, ::windows::core::IUnknown, IWbemClassObject);
impl ::core::cmp::PartialEq for IWbemObjectAccess {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemObjectAccess {}
impl ::core::fmt::Debug for IWbemObjectAccess {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWbemObjectAccess").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWbemObjectAccess {
    type Vtable = IWbemObjectAccess_Vtbl;
}
impl ::core::clone::Clone for IWbemObjectAccess {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWbemObjectAccess {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x49353c9a_516b_11d1_aea6_00c04fb68820);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemObjectAccess_Vtbl {
    pub base__: IWbemClassObject_Vtbl,
    pub GetPropertyHandle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszpropertyname: ::windows::core::PCWSTR, ptype: *mut i32, plhandle: *mut i32) -> ::windows::core::HRESULT,
    pub WritePropertyValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lhandle: i32, lnumbytes: i32, adata: *const u8) -> ::windows::core::HRESULT,
    pub ReadPropertyValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lhandle: i32, lbuffersize: i32, plnumbytes: *mut i32, adata: *mut u8) -> ::windows::core::HRESULT,
    pub ReadDWORD: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lhandle: i32, pdw: *mut u32) -> ::windows::core::HRESULT,
    pub WriteDWORD: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lhandle: i32, dw: u32) -> ::windows::core::HRESULT,
    pub ReadQWORD: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lhandle: i32, pqw: *mut u64) -> ::windows::core::HRESULT,
    pub WriteQWORD: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lhandle: i32, pw: u64) -> ::windows::core::HRESULT,
    pub GetPropertyInfoByHandle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lhandle: i32, pstrname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, ptype: *mut i32) -> ::windows::core::HRESULT,
    pub Lock: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32) -> ::windows::core::HRESULT,
    pub Unlock: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
#[repr(transparent)]
pub struct IWbemObjectSink(::windows::core::IUnknown);
impl IWbemObjectSink {
    pub unsafe fn Indicate(&self, apobjarray: &[::core::option::Option<IWbemClassObject>]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Indicate)(::windows::core::Interface::as_raw(self), apobjarray.len() as _, ::core::mem::transmute(apobjarray.as_ptr())).ok()
    }
    pub unsafe fn SetStatus<P0, P1>(&self, lflags: i32, hresult: ::windows::core::HRESULT, strparam: P0, pobjparam: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<IWbemClassObject>,
    {
        (::windows::core::Interface::vtable(self).SetStatus)(::windows::core::Interface::as_raw(self), lflags, hresult, strparam.into_param().abi(), pobjparam.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(IWbemObjectSink, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IWbemObjectSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemObjectSink {}
impl ::core::fmt::Debug for IWbemObjectSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWbemObjectSink").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWbemObjectSink {
    type Vtable = IWbemObjectSink_Vtbl;
}
impl ::core::clone::Clone for IWbemObjectSink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWbemObjectSink {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7c857801_7381_11cf_884d_00aa004b2e24);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemObjectSink_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Indicate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lobjectcount: i32, apobjarray: *const *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, hresult: ::windows::core::HRESULT, strparam: ::std::mem::MaybeUninit<::windows::core::BSTR>, pobjparam: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
#[repr(transparent)]
pub struct IWbemObjectSinkEx(::windows::core::IUnknown);
impl IWbemObjectSinkEx {
    pub unsafe fn Indicate(&self, apobjarray: &[::core::option::Option<IWbemClassObject>]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Indicate)(::windows::core::Interface::as_raw(self), apobjarray.len() as _, ::core::mem::transmute(apobjarray.as_ptr())).ok()
    }
    pub unsafe fn SetStatus<P0, P1>(&self, lflags: i32, hresult: ::windows::core::HRESULT, strparam: P0, pobjparam: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<IWbemClassObject>,
    {
        (::windows::core::Interface::vtable(self).base__.SetStatus)(::windows::core::Interface::as_raw(self), lflags, hresult, strparam.into_param().abi(), pobjparam.into_param().abi()).ok()
    }
    pub unsafe fn WriteMessage<P0>(&self, uchannel: u32, strmessage: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).WriteMessage)(::windows::core::Interface::as_raw(self), uchannel, strmessage.into_param().abi()).ok()
    }
    pub unsafe fn WriteError<P0>(&self, pobjerror: P0) -> ::windows::core::Result<u8>
    where
        P0: ::windows::core::IntoParam<IWbemClassObject>,
    {
        let mut result__ = ::windows::core::zeroed::<u8>();
        (::windows::core::Interface::vtable(self).WriteError)(::windows::core::Interface::as_raw(self), pobjerror.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn PromptUser<P0>(&self, strmessage: P0, uprompttype: u8) -> ::windows::core::Result<u8>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<u8>();
        (::windows::core::Interface::vtable(self).PromptUser)(::windows::core::Interface::as_raw(self), strmessage.into_param().abi(), uprompttype, &mut result__).from_abi(result__)
    }
    pub unsafe fn WriteProgress<P0, P1, P2>(&self, stractivity: P0, strcurrentoperation: P1, strstatusdescription: P2, upercentcomplete: u32, usecondsremaining: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
        P2: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).WriteProgress)(::windows::core::Interface::as_raw(self), stractivity.into_param().abi(), strcurrentoperation.into_param().abi(), strstatusdescription.into_param().abi(), upercentcomplete, usecondsremaining).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn WriteStreamParameter<P0>(&self, strname: P0, vtvalue: *const super::Com::VARIANT, ultype: u32, ulflags: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).WriteStreamParameter)(::windows::core::Interface::as_raw(self), strname.into_param().abi(), vtvalue, ultype, ulflags).ok()
    }
}
::windows::imp::interface_hierarchy!(IWbemObjectSinkEx, ::windows::core::IUnknown, IWbemObjectSink);
impl ::core::cmp::PartialEq for IWbemObjectSinkEx {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemObjectSinkEx {}
impl ::core::fmt::Debug for IWbemObjectSinkEx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWbemObjectSinkEx").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWbemObjectSinkEx {
    type Vtable = IWbemObjectSinkEx_Vtbl;
}
impl ::core::clone::Clone for IWbemObjectSinkEx {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWbemObjectSinkEx {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe7d35cfa_348b_485e_b524_252725d697ca);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemObjectSinkEx_Vtbl {
    pub base__: IWbemObjectSink_Vtbl,
    pub WriteMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uchannel: u32, strmessage: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub WriteError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pobjerror: *mut ::core::ffi::c_void, pureturned: *mut u8) -> ::windows::core::HRESULT,
    pub PromptUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strmessage: ::std::mem::MaybeUninit<::windows::core::BSTR>, uprompttype: u8, pureturned: *mut u8) -> ::windows::core::HRESULT,
    pub WriteProgress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stractivity: ::std::mem::MaybeUninit<::windows::core::BSTR>, strcurrentoperation: ::std::mem::MaybeUninit<::windows::core::BSTR>, strstatusdescription: ::std::mem::MaybeUninit<::windows::core::BSTR>, upercentcomplete: u32, usecondsremaining: u32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub WriteStreamParameter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strname: ::std::mem::MaybeUninit<::windows::core::BSTR>, vtvalue: *const super::Com::VARIANT, ultype: u32, ulflags: u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    WriteStreamParameter: usize,
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
#[repr(transparent)]
pub struct IWbemObjectTextSrc(::windows::core::IUnknown);
impl IWbemObjectTextSrc {
    pub unsafe fn GetText<P0, P1>(&self, lflags: i32, pobj: P0, uobjtextformat: u32, pctx: P1) -> ::windows::core::Result<::windows::core::BSTR>
    where
        P0: ::windows::core::IntoParam<IWbemClassObject>,
        P1: ::windows::core::IntoParam<IWbemContext>,
    {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).GetText)(::windows::core::Interface::as_raw(self), lflags, pobj.into_param().abi(), uobjtextformat, pctx.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateFromText<P0, P1>(&self, lflags: i32, strtext: P0, uobjtextformat: u32, pctx: P1) -> ::windows::core::Result<IWbemClassObject>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<IWbemContext>,
    {
        let mut result__ = ::windows::core::zeroed::<IWbemClassObject>();
        (::windows::core::Interface::vtable(self).CreateFromText)(::windows::core::Interface::as_raw(self), lflags, strtext.into_param().abi(), uobjtextformat, pctx.into_param().abi(), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IWbemObjectTextSrc, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IWbemObjectTextSrc {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemObjectTextSrc {}
impl ::core::fmt::Debug for IWbemObjectTextSrc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWbemObjectTextSrc").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWbemObjectTextSrc {
    type Vtable = IWbemObjectTextSrc_Vtbl;
}
impl ::core::clone::Clone for IWbemObjectTextSrc {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWbemObjectTextSrc {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbfbf883a_cad7_11d3_a11b_00105a1f515a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemObjectTextSrc_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, pobj: *mut ::core::ffi::c_void, uobjtextformat: u32, pctx: *mut ::core::ffi::c_void, strtext: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub CreateFromText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, strtext: ::std::mem::MaybeUninit<::windows::core::BSTR>, uobjtextformat: u32, pctx: *mut ::core::ffi::c_void, pnewobj: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
#[repr(transparent)]
pub struct IWbemPath(::windows::core::IUnknown);
impl IWbemPath {
    pub unsafe fn SetText<P0>(&self, umode: u32, pszpath: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetText)(::windows::core::Interface::as_raw(self), umode, pszpath.into_param().abi()).ok()
    }
    pub unsafe fn GetText(&self, lflags: i32, pubufflength: *mut u32, psztext: ::windows::core::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetText)(::windows::core::Interface::as_raw(self), lflags, pubufflength, ::core::mem::transmute(psztext)).ok()
    }
    pub unsafe fn GetInfo(&self, urequestedinfo: u32) -> ::windows::core::Result<u64> {
        let mut result__ = ::windows::core::zeroed::<u64>();
        (::windows::core::Interface::vtable(self).GetInfo)(::windows::core::Interface::as_raw(self), urequestedinfo, &mut result__).from_abi(result__)
    }
    pub unsafe fn SetServer<P0>(&self, name: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetServer)(::windows::core::Interface::as_raw(self), name.into_param().abi()).ok()
    }
    pub unsafe fn GetServer(&self, punamebuflength: *mut u32, pname: ::windows::core::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetServer)(::windows::core::Interface::as_raw(self), punamebuflength, ::core::mem::transmute(pname)).ok()
    }
    pub unsafe fn GetNamespaceCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetNamespaceCount)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetNamespaceAt<P0>(&self, uindex: u32, pszname: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetNamespaceAt)(::windows::core::Interface::as_raw(self), uindex, pszname.into_param().abi()).ok()
    }
    pub unsafe fn GetNamespaceAt(&self, uindex: u32, punamebuflength: *mut u32, pname: ::windows::core::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetNamespaceAt)(::windows::core::Interface::as_raw(self), uindex, punamebuflength, ::core::mem::transmute(pname)).ok()
    }
    pub unsafe fn RemoveNamespaceAt(&self, uindex: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RemoveNamespaceAt)(::windows::core::Interface::as_raw(self), uindex).ok()
    }
    pub unsafe fn RemoveAllNamespaces(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RemoveAllNamespaces)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetScopeCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetScopeCount)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetScope<P0>(&self, uindex: u32, pszclass: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetScope)(::windows::core::Interface::as_raw(self), uindex, pszclass.into_param().abi()).ok()
    }
    pub unsafe fn SetScopeFromText<P0>(&self, uindex: u32, psztext: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetScopeFromText)(::windows::core::Interface::as_raw(self), uindex, psztext.into_param().abi()).ok()
    }
    pub unsafe fn GetScope(&self, uindex: u32, puclassnamebufsize: *mut u32, pszclass: ::windows::core::PWSTR, pkeylist: *mut ::core::option::Option<IWbemPathKeyList>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetScope)(::windows::core::Interface::as_raw(self), uindex, puclassnamebufsize, ::core::mem::transmute(pszclass), ::core::mem::transmute(pkeylist)).ok()
    }
    pub unsafe fn GetScopeAsText(&self, uindex: u32, putextbufsize: *mut u32, psztext: ::windows::core::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetScopeAsText)(::windows::core::Interface::as_raw(self), uindex, putextbufsize, ::core::mem::transmute(psztext)).ok()
    }
    pub unsafe fn RemoveScope(&self, uindex: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RemoveScope)(::windows::core::Interface::as_raw(self), uindex).ok()
    }
    pub unsafe fn RemoveAllScopes(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RemoveAllScopes)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetClassName<P0>(&self, name: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetClassName)(::windows::core::Interface::as_raw(self), name.into_param().abi()).ok()
    }
    pub unsafe fn GetClassName(&self, pubufflength: *mut u32, pszname: ::windows::core::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetClassName)(::windows::core::Interface::as_raw(self), pubufflength, ::core::mem::transmute(pszname)).ok()
    }
    pub unsafe fn GetKeyList(&self) -> ::windows::core::Result<IWbemPathKeyList> {
        let mut result__ = ::windows::core::zeroed::<IWbemPathKeyList>();
        (::windows::core::Interface::vtable(self).GetKeyList)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateClassPart<P0>(&self, lflags: i32, name: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).CreateClassPart)(::windows::core::Interface::as_raw(self), lflags, name.into_param().abi()).ok()
    }
    pub unsafe fn DeleteClassPart(&self, lflags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DeleteClassPart)(::windows::core::Interface::as_raw(self), lflags).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsRelative<P0, P1>(&self, wszmachine: P0, wsznamespace: P1) -> super::super::Foundation::BOOL
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).IsRelative)(::windows::core::Interface::as_raw(self), wszmachine.into_param().abi(), wsznamespace.into_param().abi())
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsRelativeOrChild<P0, P1>(&self, wszmachine: P0, wsznamespace: P1, lflags: i32) -> super::super::Foundation::BOOL
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).IsRelativeOrChild)(::windows::core::Interface::as_raw(self), wszmachine.into_param().abi(), wsznamespace.into_param().abi(), lflags)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsLocal<P0>(&self, wszmachine: P0) -> super::super::Foundation::BOOL
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).IsLocal)(::windows::core::Interface::as_raw(self), wszmachine.into_param().abi())
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsSameClassName<P0>(&self, wszclass: P0) -> super::super::Foundation::BOOL
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).IsSameClassName)(::windows::core::Interface::as_raw(self), wszclass.into_param().abi())
    }
}
::windows::imp::interface_hierarchy!(IWbemPath, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IWbemPath {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemPath {}
impl ::core::fmt::Debug for IWbemPath {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWbemPath").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWbemPath {
    type Vtable = IWbemPath_Vtbl;
}
impl ::core::clone::Clone for IWbemPath {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWbemPath {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3bc15af2_736c_477e_9e51_238af8667dcc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemPath_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub SetText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, umode: u32, pszpath: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub GetText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, pubufflength: *mut u32, psztext: ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub GetInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, urequestedinfo: u32, puresponse: *mut u64) -> ::windows::core::HRESULT,
    pub SetServer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub GetServer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punamebuflength: *mut u32, pname: ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub GetNamespaceCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pucount: *mut u32) -> ::windows::core::HRESULT,
    pub SetNamespaceAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uindex: u32, pszname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub GetNamespaceAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uindex: u32, punamebuflength: *mut u32, pname: ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub RemoveNamespaceAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uindex: u32) -> ::windows::core::HRESULT,
    pub RemoveAllNamespaces: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetScopeCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pucount: *mut u32) -> ::windows::core::HRESULT,
    pub SetScope: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uindex: u32, pszclass: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub SetScopeFromText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uindex: u32, psztext: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub GetScope: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uindex: u32, puclassnamebufsize: *mut u32, pszclass: ::windows::core::PWSTR, pkeylist: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetScopeAsText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uindex: u32, putextbufsize: *mut u32, psztext: ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub RemoveScope: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uindex: u32) -> ::windows::core::HRESULT,
    pub RemoveAllScopes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetClassName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub GetClassName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pubufflength: *mut u32, pszname: ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub GetKeyList: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pout: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateClassPart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, name: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub DeleteClassPart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsRelative: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszmachine: ::windows::core::PCWSTR, wsznamespace: ::windows::core::PCWSTR) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsRelative: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsRelativeOrChild: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszmachine: ::windows::core::PCWSTR, wsznamespace: ::windows::core::PCWSTR, lflags: i32) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsRelativeOrChild: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsLocal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszmachine: ::windows::core::PCWSTR) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsLocal: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsSameClassName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszclass: ::windows::core::PCWSTR) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsSameClassName: usize,
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
#[repr(transparent)]
pub struct IWbemPathKeyList(::windows::core::IUnknown);
impl IWbemPathKeyList {
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetCount)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetKey<P0>(&self, wszname: P0, uflags: u32, ucimtype: u32, pkeyval: *const ::core::ffi::c_void) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetKey)(::windows::core::Interface::as_raw(self), wszname.into_param().abi(), uflags, ucimtype, pkeyval).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetKey2<P0>(&self, wszname: P0, uflags: u32, ucimtype: u32, pkeyval: *const super::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetKey2)(::windows::core::Interface::as_raw(self), wszname.into_param().abi(), uflags, ucimtype, pkeyval).ok()
    }
    pub unsafe fn GetKey(&self, ukeyix: u32, uflags: u32, punamebufsize: *mut u32, pszkeyname: ::windows::core::PWSTR, pukeyvalbufsize: *mut u32, pkeyval: *mut ::core::ffi::c_void, puapparentcimtype: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetKey)(::windows::core::Interface::as_raw(self), ukeyix, uflags, punamebufsize, ::core::mem::transmute(pszkeyname), pukeyvalbufsize, pkeyval, puapparentcimtype).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetKey2(&self, ukeyix: u32, uflags: u32, punamebufsize: *mut u32, pszkeyname: ::windows::core::PWSTR, pkeyvalue: *mut super::Com::VARIANT, puapparentcimtype: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetKey2)(::windows::core::Interface::as_raw(self), ukeyix, uflags, punamebufsize, ::core::mem::transmute(pszkeyname), pkeyvalue, puapparentcimtype).ok()
    }
    pub unsafe fn RemoveKey<P0>(&self, wszname: P0, uflags: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).RemoveKey)(::windows::core::Interface::as_raw(self), wszname.into_param().abi(), uflags).ok()
    }
    pub unsafe fn RemoveAllKeys(&self, uflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RemoveAllKeys)(::windows::core::Interface::as_raw(self), uflags).ok()
    }
    pub unsafe fn MakeSingleton(&self, bset: u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).MakeSingleton)(::windows::core::Interface::as_raw(self), bset).ok()
    }
    pub unsafe fn GetInfo(&self, urequestedinfo: u32) -> ::windows::core::Result<u64> {
        let mut result__ = ::windows::core::zeroed::<u64>();
        (::windows::core::Interface::vtable(self).GetInfo)(::windows::core::Interface::as_raw(self), urequestedinfo, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetText(&self, lflags: i32, pubufflength: *mut u32, psztext: ::windows::core::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetText)(::windows::core::Interface::as_raw(self), lflags, pubufflength, ::core::mem::transmute(psztext)).ok()
    }
}
::windows::imp::interface_hierarchy!(IWbemPathKeyList, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IWbemPathKeyList {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemPathKeyList {}
impl ::core::fmt::Debug for IWbemPathKeyList {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWbemPathKeyList").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWbemPathKeyList {
    type Vtable = IWbemPathKeyList_Vtbl;
}
impl ::core::clone::Clone for IWbemPathKeyList {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWbemPathKeyList {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9ae62877_7544_4bb0_aa26_a13824659ed6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemPathKeyList_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pukeycount: *mut u32) -> ::windows::core::HRESULT,
    pub SetKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszname: ::windows::core::PCWSTR, uflags: u32, ucimtype: u32, pkeyval: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetKey2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszname: ::windows::core::PCWSTR, uflags: u32, ucimtype: u32, pkeyval: *const super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetKey2: usize,
    pub GetKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ukeyix: u32, uflags: u32, punamebufsize: *mut u32, pszkeyname: ::windows::core::PWSTR, pukeyvalbufsize: *mut u32, pkeyval: *mut ::core::ffi::c_void, puapparentcimtype: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetKey2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ukeyix: u32, uflags: u32, punamebufsize: *mut u32, pszkeyname: ::windows::core::PWSTR, pkeyvalue: *mut super::Com::VARIANT, puapparentcimtype: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetKey2: usize,
    pub RemoveKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszname: ::windows::core::PCWSTR, uflags: u32) -> ::windows::core::HRESULT,
    pub RemoveAllKeys: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uflags: u32) -> ::windows::core::HRESULT,
    pub MakeSingleton: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bset: u8) -> ::windows::core::HRESULT,
    pub GetInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, urequestedinfo: u32, puresponse: *mut u64) -> ::windows::core::HRESULT,
    pub GetText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, pubufflength: *mut u32, psztext: ::windows::core::PWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
#[repr(transparent)]
pub struct IWbemPropertyProvider(::windows::core::IUnknown);
impl IWbemPropertyProvider {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetProperty<P0, P1, P2, P3>(&self, lflags: i32, strlocale: P0, strclassmapping: P1, strinstmapping: P2, strpropmapping: P3) -> ::windows::core::Result<super::Com::VARIANT>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
        P2: ::windows::core::IntoParam<::windows::core::BSTR>,
        P3: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<super::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).GetProperty)(::windows::core::Interface::as_raw(self), lflags, strlocale.into_param().abi(), strclassmapping.into_param().abi(), strinstmapping.into_param().abi(), strpropmapping.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PutProperty<P0, P1, P2, P3>(&self, lflags: i32, strlocale: P0, strclassmapping: P1, strinstmapping: P2, strpropmapping: P3, pvvalue: *const super::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
        P2: ::windows::core::IntoParam<::windows::core::BSTR>,
        P3: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).PutProperty)(::windows::core::Interface::as_raw(self), lflags, strlocale.into_param().abi(), strclassmapping.into_param().abi(), strinstmapping.into_param().abi(), strpropmapping.into_param().abi(), pvvalue).ok()
    }
}
::windows::imp::interface_hierarchy!(IWbemPropertyProvider, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IWbemPropertyProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemPropertyProvider {}
impl ::core::fmt::Debug for IWbemPropertyProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWbemPropertyProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWbemPropertyProvider {
    type Vtable = IWbemPropertyProvider_Vtbl;
}
impl ::core::clone::Clone for IWbemPropertyProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWbemPropertyProvider {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xce61e841_65bc_11d0_b6bd_00aa003240c7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemPropertyProvider_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, strlocale: ::std::mem::MaybeUninit<::windows::core::BSTR>, strclassmapping: ::std::mem::MaybeUninit<::windows::core::BSTR>, strinstmapping: ::std::mem::MaybeUninit<::windows::core::BSTR>, strpropmapping: ::std::mem::MaybeUninit<::windows::core::BSTR>, pvvalue: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetProperty: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub PutProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, strlocale: ::std::mem::MaybeUninit<::windows::core::BSTR>, strclassmapping: ::std::mem::MaybeUninit<::windows::core::BSTR>, strinstmapping: ::std::mem::MaybeUninit<::windows::core::BSTR>, strpropmapping: ::std::mem::MaybeUninit<::windows::core::BSTR>, pvvalue: *const super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    PutProperty: usize,
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
#[repr(transparent)]
pub struct IWbemProviderIdentity(::windows::core::IUnknown);
impl IWbemProviderIdentity {
    pub unsafe fn SetRegistrationObject<P0>(&self, lflags: i32, pprovreg: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IWbemClassObject>,
    {
        (::windows::core::Interface::vtable(self).SetRegistrationObject)(::windows::core::Interface::as_raw(self), lflags, pprovreg.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(IWbemProviderIdentity, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IWbemProviderIdentity {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemProviderIdentity {}
impl ::core::fmt::Debug for IWbemProviderIdentity {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWbemProviderIdentity").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWbemProviderIdentity {
    type Vtable = IWbemProviderIdentity_Vtbl;
}
impl ::core::clone::Clone for IWbemProviderIdentity {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWbemProviderIdentity {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x631f7d97_d993_11d2_b339_00105a1f4aaf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemProviderIdentity_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub SetRegistrationObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, pprovreg: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
#[repr(transparent)]
pub struct IWbemProviderInit(::windows::core::IUnknown);
impl IWbemProviderInit {
    pub unsafe fn Initialize<P0, P1, P2, P3, P4, P5>(&self, wszuser: P0, lflags: i32, wsznamespace: P1, wszlocale: P2, pnamespace: P3, pctx: P4, pinitsink: P5) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P3: ::windows::core::IntoParam<IWbemServices>,
        P4: ::windows::core::IntoParam<IWbemContext>,
        P5: ::windows::core::IntoParam<IWbemProviderInitSink>,
    {
        (::windows::core::Interface::vtable(self).Initialize)(::windows::core::Interface::as_raw(self), wszuser.into_param().abi(), lflags, wsznamespace.into_param().abi(), wszlocale.into_param().abi(), pnamespace.into_param().abi(), pctx.into_param().abi(), pinitsink.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(IWbemProviderInit, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IWbemProviderInit {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemProviderInit {}
impl ::core::fmt::Debug for IWbemProviderInit {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWbemProviderInit").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWbemProviderInit {
    type Vtable = IWbemProviderInit_Vtbl;
}
impl ::core::clone::Clone for IWbemProviderInit {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWbemProviderInit {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1be41572_91dd_11d1_aeb2_00c04fb68820);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemProviderInit_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszuser: ::windows::core::PCWSTR, lflags: i32, wsznamespace: ::windows::core::PCWSTR, wszlocale: ::windows::core::PCWSTR, pnamespace: *mut ::core::ffi::c_void, pctx: *mut ::core::ffi::c_void, pinitsink: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
#[repr(transparent)]
pub struct IWbemProviderInitSink(::windows::core::IUnknown);
impl IWbemProviderInitSink {
    pub unsafe fn SetStatus(&self, lstatus: i32, lflags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetStatus)(::windows::core::Interface::as_raw(self), lstatus, lflags).ok()
    }
}
::windows::imp::interface_hierarchy!(IWbemProviderInitSink, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IWbemProviderInitSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemProviderInitSink {}
impl ::core::fmt::Debug for IWbemProviderInitSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWbemProviderInitSink").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWbemProviderInitSink {
    type Vtable = IWbemProviderInitSink_Vtbl;
}
impl ::core::clone::Clone for IWbemProviderInitSink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWbemProviderInitSink {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1be41571_91dd_11d1_aeb2_00c04fb68820);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemProviderInitSink_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub SetStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lstatus: i32, lflags: i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
#[repr(transparent)]
pub struct IWbemQualifierSet(::windows::core::IUnknown);
impl IWbemQualifierSet {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Get<P0>(&self, wszname: P0, lflags: i32, pval: *mut super::Com::VARIANT, plflavor: *mut i32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).Get)(::windows::core::Interface::as_raw(self), wszname.into_param().abi(), lflags, pval, plflavor).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Put<P0>(&self, wszname: P0, pval: *const super::Com::VARIANT, lflavor: i32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).Put)(::windows::core::Interface::as_raw(self), wszname.into_param().abi(), pval, lflavor).ok()
    }
    pub unsafe fn Delete<P0>(&self, wszname: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).Delete)(::windows::core::Interface::as_raw(self), wszname.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetNames(&self, lflags: i32) -> ::windows::core::Result<*mut super::Com::SAFEARRAY> {
        let mut result__ = ::windows::core::zeroed::<*mut super::Com::SAFEARRAY>();
        (::windows::core::Interface::vtable(self).GetNames)(::windows::core::Interface::as_raw(self), lflags, &mut result__).from_abi(result__)
    }
    pub unsafe fn BeginEnumeration(&self, lflags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).BeginEnumeration)(::windows::core::Interface::as_raw(self), lflags).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Next(&self, lflags: i32, pstrname: *mut ::windows::core::BSTR, pval: *mut super::Com::VARIANT, plflavor: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Next)(::windows::core::Interface::as_raw(self), lflags, ::core::mem::transmute(pstrname), pval, plflavor).ok()
    }
    pub unsafe fn EndEnumeration(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EndEnumeration)(::windows::core::Interface::as_raw(self)).ok()
    }
}
::windows::imp::interface_hierarchy!(IWbemQualifierSet, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IWbemQualifierSet {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemQualifierSet {}
impl ::core::fmt::Debug for IWbemQualifierSet {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWbemQualifierSet").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWbemQualifierSet {
    type Vtable = IWbemQualifierSet_Vtbl;
}
impl ::core::clone::Clone for IWbemQualifierSet {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWbemQualifierSet {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdc12a680_737f_11cf_884d_00aa004b2e24);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemQualifierSet_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Get: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszname: ::windows::core::PCWSTR, lflags: i32, pval: *mut super::Com::VARIANT, plflavor: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Get: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Put: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszname: ::windows::core::PCWSTR, pval: *const super::Com::VARIANT, lflavor: i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Put: usize,
    pub Delete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetNames: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, pnames: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetNames: usize,
    pub BeginEnumeration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, pstrname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, pval: *mut super::Com::VARIANT, plflavor: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Next: usize,
    pub EndEnumeration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
#[repr(transparent)]
pub struct IWbemQuery(::windows::core::IUnknown);
impl IWbemQuery {
    pub unsafe fn Empty(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Empty)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetLanguageFeatures(&self, uflags: u32, uarraysize: u32, pufeatures: *const u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetLanguageFeatures)(::windows::core::Interface::as_raw(self), uflags, uarraysize, pufeatures).ok()
    }
    pub unsafe fn TestLanguageFeatures(&self, uflags: u32, uarraysize: *mut u32, pufeatures: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).TestLanguageFeatures)(::windows::core::Interface::as_raw(self), uflags, uarraysize, pufeatures).ok()
    }
    pub unsafe fn Parse<P0, P1>(&self, pszlang: P0, pszquery: P1, uflags: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).Parse)(::windows::core::Interface::as_raw(self), pszlang.into_param().abi(), pszquery.into_param().abi(), uflags).ok()
    }
    pub unsafe fn GetAnalysis(&self, uanalysistype: u32, uflags: u32, panalysis: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetAnalysis)(::windows::core::Interface::as_raw(self), uanalysistype, uflags, panalysis).ok()
    }
    pub unsafe fn FreeMemory(&self, pmem: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).FreeMemory)(::windows::core::Interface::as_raw(self), pmem).ok()
    }
    pub unsafe fn GetQueryInfo(&self, uanalysistype: u32, uinfoid: u32, ubufsize: u32, pdestbuf: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetQueryInfo)(::windows::core::Interface::as_raw(self), uanalysistype, uinfoid, ubufsize, pdestbuf).ok()
    }
}
::windows::imp::interface_hierarchy!(IWbemQuery, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IWbemQuery {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemQuery {}
impl ::core::fmt::Debug for IWbemQuery {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWbemQuery").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWbemQuery {
    type Vtable = IWbemQuery_Vtbl;
}
impl ::core::clone::Clone for IWbemQuery {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWbemQuery {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x81166f58_dd98_11d3_a120_00105a1f515a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemQuery_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Empty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetLanguageFeatures: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uflags: u32, uarraysize: u32, pufeatures: *const u32) -> ::windows::core::HRESULT,
    pub TestLanguageFeatures: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uflags: u32, uarraysize: *mut u32, pufeatures: *mut u32) -> ::windows::core::HRESULT,
    pub Parse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszlang: ::windows::core::PCWSTR, pszquery: ::windows::core::PCWSTR, uflags: u32) -> ::windows::core::HRESULT,
    pub GetAnalysis: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uanalysistype: u32, uflags: u32, panalysis: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub FreeMemory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmem: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetQueryInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uanalysistype: u32, uinfoid: u32, ubufsize: u32, pdestbuf: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
#[repr(transparent)]
pub struct IWbemRefresher(::windows::core::IUnknown);
impl IWbemRefresher {
    pub unsafe fn Refresh(&self, lflags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Refresh)(::windows::core::Interface::as_raw(self), lflags).ok()
    }
}
::windows::imp::interface_hierarchy!(IWbemRefresher, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IWbemRefresher {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemRefresher {}
impl ::core::fmt::Debug for IWbemRefresher {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWbemRefresher").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWbemRefresher {
    type Vtable = IWbemRefresher_Vtbl;
}
impl ::core::clone::Clone for IWbemRefresher {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWbemRefresher {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x49353c99_516b_11d1_aea6_00c04fb68820);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemRefresher_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Refresh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
#[repr(transparent)]
pub struct IWbemServices(::windows::core::IUnknown);
impl IWbemServices {
    pub unsafe fn OpenNamespace<P0, P1>(&self, strnamespace: P0, lflags: i32, pctx: P1, ppworkingnamespace: ::core::option::Option<*mut ::core::option::Option<IWbemServices>>, ppresult: ::core::option::Option<*mut ::core::option::Option<IWbemCallResult>>) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<IWbemContext>,
    {
        (::windows::core::Interface::vtable(self).OpenNamespace)(::windows::core::Interface::as_raw(self), strnamespace.into_param().abi(), lflags, pctx.into_param().abi(), ::core::mem::transmute(ppworkingnamespace.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(ppresult.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn CancelAsyncCall<P0>(&self, psink: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IWbemObjectSink>,
    {
        (::windows::core::Interface::vtable(self).CancelAsyncCall)(::windows::core::Interface::as_raw(self), psink.into_param().abi()).ok()
    }
    pub unsafe fn QueryObjectSink(&self, lflags: i32) -> ::windows::core::Result<IWbemObjectSink> {
        let mut result__ = ::windows::core::zeroed::<IWbemObjectSink>();
        (::windows::core::Interface::vtable(self).QueryObjectSink)(::windows::core::Interface::as_raw(self), lflags, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetObject<P0, P1>(&self, strobjectpath: P0, lflags: i32, pctx: P1, ppobject: ::core::option::Option<*mut ::core::option::Option<IWbemClassObject>>, ppcallresult: ::core::option::Option<*mut ::core::option::Option<IWbemCallResult>>) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<IWbemContext>,
    {
        (::windows::core::Interface::vtable(self).GetObject)(::windows::core::Interface::as_raw(self), strobjectpath.into_param().abi(), lflags, pctx.into_param().abi(), ::core::mem::transmute(ppobject.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(ppcallresult.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetObjectAsync<P0, P1, P2>(&self, strobjectpath: P0, lflags: i32, pctx: P1, presponsehandler: P2) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<IWbemContext>,
        P2: ::windows::core::IntoParam<IWbemObjectSink>,
    {
        (::windows::core::Interface::vtable(self).GetObjectAsync)(::windows::core::Interface::as_raw(self), strobjectpath.into_param().abi(), lflags, pctx.into_param().abi(), presponsehandler.into_param().abi()).ok()
    }
    pub unsafe fn PutClass<P0, P1>(&self, pobject: P0, lflags: i32, pctx: P1, ppcallresult: ::core::option::Option<*mut ::core::option::Option<IWbemCallResult>>) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IWbemClassObject>,
        P1: ::windows::core::IntoParam<IWbemContext>,
    {
        (::windows::core::Interface::vtable(self).PutClass)(::windows::core::Interface::as_raw(self), pobject.into_param().abi(), lflags, pctx.into_param().abi(), ::core::mem::transmute(ppcallresult.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn PutClassAsync<P0, P1, P2>(&self, pobject: P0, lflags: i32, pctx: P1, presponsehandler: P2) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IWbemClassObject>,
        P1: ::windows::core::IntoParam<IWbemContext>,
        P2: ::windows::core::IntoParam<IWbemObjectSink>,
    {
        (::windows::core::Interface::vtable(self).PutClassAsync)(::windows::core::Interface::as_raw(self), pobject.into_param().abi(), lflags, pctx.into_param().abi(), presponsehandler.into_param().abi()).ok()
    }
    pub unsafe fn DeleteClass<P0, P1>(&self, strclass: P0, lflags: i32, pctx: P1, ppcallresult: ::core::option::Option<*mut ::core::option::Option<IWbemCallResult>>) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<IWbemContext>,
    {
        (::windows::core::Interface::vtable(self).DeleteClass)(::windows::core::Interface::as_raw(self), strclass.into_param().abi(), lflags, pctx.into_param().abi(), ::core::mem::transmute(ppcallresult.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn DeleteClassAsync<P0, P1, P2>(&self, strclass: P0, lflags: i32, pctx: P1, presponsehandler: P2) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<IWbemContext>,
        P2: ::windows::core::IntoParam<IWbemObjectSink>,
    {
        (::windows::core::Interface::vtable(self).DeleteClassAsync)(::windows::core::Interface::as_raw(self), strclass.into_param().abi(), lflags, pctx.into_param().abi(), presponsehandler.into_param().abi()).ok()
    }
    pub unsafe fn CreateClassEnum<P0, P1>(&self, strsuperclass: P0, lflags: i32, pctx: P1) -> ::windows::core::Result<IEnumWbemClassObject>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<IWbemContext>,
    {
        let mut result__ = ::windows::core::zeroed::<IEnumWbemClassObject>();
        (::windows::core::Interface::vtable(self).CreateClassEnum)(::windows::core::Interface::as_raw(self), strsuperclass.into_param().abi(), lflags, pctx.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateClassEnumAsync<P0, P1, P2>(&self, strsuperclass: P0, lflags: i32, pctx: P1, presponsehandler: P2) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<IWbemContext>,
        P2: ::windows::core::IntoParam<IWbemObjectSink>,
    {
        (::windows::core::Interface::vtable(self).CreateClassEnumAsync)(::windows::core::Interface::as_raw(self), strsuperclass.into_param().abi(), lflags, pctx.into_param().abi(), presponsehandler.into_param().abi()).ok()
    }
    pub unsafe fn PutInstance<P0, P1>(&self, pinst: P0, lflags: i32, pctx: P1, ppcallresult: ::core::option::Option<*mut ::core::option::Option<IWbemCallResult>>) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IWbemClassObject>,
        P1: ::windows::core::IntoParam<IWbemContext>,
    {
        (::windows::core::Interface::vtable(self).PutInstance)(::windows::core::Interface::as_raw(self), pinst.into_param().abi(), lflags, pctx.into_param().abi(), ::core::mem::transmute(ppcallresult.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn PutInstanceAsync<P0, P1, P2>(&self, pinst: P0, lflags: i32, pctx: P1, presponsehandler: P2) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IWbemClassObject>,
        P1: ::windows::core::IntoParam<IWbemContext>,
        P2: ::windows::core::IntoParam<IWbemObjectSink>,
    {
        (::windows::core::Interface::vtable(self).PutInstanceAsync)(::windows::core::Interface::as_raw(self), pinst.into_param().abi(), lflags, pctx.into_param().abi(), presponsehandler.into_param().abi()).ok()
    }
    pub unsafe fn DeleteInstance<P0, P1>(&self, strobjectpath: P0, lflags: i32, pctx: P1, ppcallresult: ::core::option::Option<*mut ::core::option::Option<IWbemCallResult>>) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<IWbemContext>,
    {
        (::windows::core::Interface::vtable(self).DeleteInstance)(::windows::core::Interface::as_raw(self), strobjectpath.into_param().abi(), lflags, pctx.into_param().abi(), ::core::mem::transmute(ppcallresult.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn DeleteInstanceAsync<P0, P1, P2>(&self, strobjectpath: P0, lflags: i32, pctx: P1, presponsehandler: P2) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<IWbemContext>,
        P2: ::windows::core::IntoParam<IWbemObjectSink>,
    {
        (::windows::core::Interface::vtable(self).DeleteInstanceAsync)(::windows::core::Interface::as_raw(self), strobjectpath.into_param().abi(), lflags, pctx.into_param().abi(), presponsehandler.into_param().abi()).ok()
    }
    pub unsafe fn CreateInstanceEnum<P0, P1>(&self, strfilter: P0, lflags: i32, pctx: P1) -> ::windows::core::Result<IEnumWbemClassObject>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<IWbemContext>,
    {
        let mut result__ = ::windows::core::zeroed::<IEnumWbemClassObject>();
        (::windows::core::Interface::vtable(self).CreateInstanceEnum)(::windows::core::Interface::as_raw(self), strfilter.into_param().abi(), lflags, pctx.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateInstanceEnumAsync<P0, P1, P2>(&self, strfilter: P0, lflags: i32, pctx: P1, presponsehandler: P2) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<IWbemContext>,
        P2: ::windows::core::IntoParam<IWbemObjectSink>,
    {
        (::windows::core::Interface::vtable(self).CreateInstanceEnumAsync)(::windows::core::Interface::as_raw(self), strfilter.into_param().abi(), lflags, pctx.into_param().abi(), presponsehandler.into_param().abi()).ok()
    }
    pub unsafe fn ExecQuery<P0, P1, P2>(&self, strquerylanguage: P0, strquery: P1, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: P2) -> ::windows::core::Result<IEnumWbemClassObject>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
        P2: ::windows::core::IntoParam<IWbemContext>,
    {
        let mut result__ = ::windows::core::zeroed::<IEnumWbemClassObject>();
        (::windows::core::Interface::vtable(self).ExecQuery)(::windows::core::Interface::as_raw(self), strquerylanguage.into_param().abi(), strquery.into_param().abi(), lflags, pctx.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn ExecQueryAsync<P0, P1, P2, P3>(&self, strquerylanguage: P0, strquery: P1, lflags: i32, pctx: P2, presponsehandler: P3) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
        P2: ::windows::core::IntoParam<IWbemContext>,
        P3: ::windows::core::IntoParam<IWbemObjectSink>,
    {
        (::windows::core::Interface::vtable(self).ExecQueryAsync)(::windows::core::Interface::as_raw(self), strquerylanguage.into_param().abi(), strquery.into_param().abi(), lflags, pctx.into_param().abi(), presponsehandler.into_param().abi()).ok()
    }
    pub unsafe fn ExecNotificationQuery<P0, P1, P2>(&self, strquerylanguage: P0, strquery: P1, lflags: i32, pctx: P2) -> ::windows::core::Result<IEnumWbemClassObject>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
        P2: ::windows::core::IntoParam<IWbemContext>,
    {
        let mut result__ = ::windows::core::zeroed::<IEnumWbemClassObject>();
        (::windows::core::Interface::vtable(self).ExecNotificationQuery)(::windows::core::Interface::as_raw(self), strquerylanguage.into_param().abi(), strquery.into_param().abi(), lflags, pctx.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn ExecNotificationQueryAsync<P0, P1, P2, P3>(&self, strquerylanguage: P0, strquery: P1, lflags: i32, pctx: P2, presponsehandler: P3) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
        P2: ::windows::core::IntoParam<IWbemContext>,
        P3: ::windows::core::IntoParam<IWbemObjectSink>,
    {
        (::windows::core::Interface::vtable(self).ExecNotificationQueryAsync)(::windows::core::Interface::as_raw(self), strquerylanguage.into_param().abi(), strquery.into_param().abi(), lflags, pctx.into_param().abi(), presponsehandler.into_param().abi()).ok()
    }
    pub unsafe fn ExecMethod<P0, P1, P2, P3>(&self, strobjectpath: P0, strmethodname: P1, lflags: i32, pctx: P2, pinparams: P3, ppoutparams: ::core::option::Option<*mut ::core::option::Option<IWbemClassObject>>, ppcallresult: ::core::option::Option<*mut ::core::option::Option<IWbemCallResult>>) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
        P2: ::windows::core::IntoParam<IWbemContext>,
        P3: ::windows::core::IntoParam<IWbemClassObject>,
    {
        (::windows::core::Interface::vtable(self).ExecMethod)(::windows::core::Interface::as_raw(self), strobjectpath.into_param().abi(), strmethodname.into_param().abi(), lflags, pctx.into_param().abi(), pinparams.into_param().abi(), ::core::mem::transmute(ppoutparams.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(ppcallresult.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn ExecMethodAsync<P0, P1, P2, P3, P4>(&self, strobjectpath: P0, strmethodname: P1, lflags: i32, pctx: P2, pinparams: P3, presponsehandler: P4) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
        P2: ::windows::core::IntoParam<IWbemContext>,
        P3: ::windows::core::IntoParam<IWbemClassObject>,
        P4: ::windows::core::IntoParam<IWbemObjectSink>,
    {
        (::windows::core::Interface::vtable(self).ExecMethodAsync)(::windows::core::Interface::as_raw(self), strobjectpath.into_param().abi(), strmethodname.into_param().abi(), lflags, pctx.into_param().abi(), pinparams.into_param().abi(), presponsehandler.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(IWbemServices, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IWbemServices {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemServices {}
impl ::core::fmt::Debug for IWbemServices {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWbemServices").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWbemServices {
    type Vtable = IWbemServices_Vtbl;
}
impl ::core::clone::Clone for IWbemServices {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWbemServices {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9556dc99_828c_11cf_a37e_00aa003240c7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemServices_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub OpenNamespace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strnamespace: ::std::mem::MaybeUninit<::windows::core::BSTR>, lflags: i32, pctx: *mut ::core::ffi::c_void, ppworkingnamespace: *mut *mut ::core::ffi::c_void, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CancelAsyncCall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psink: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub QueryObjectSink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, ppresponsehandler: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strobjectpath: ::std::mem::MaybeUninit<::windows::core::BSTR>, lflags: i32, pctx: *mut ::core::ffi::c_void, ppobject: *mut *mut ::core::ffi::c_void, ppcallresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetObjectAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strobjectpath: ::std::mem::MaybeUninit<::windows::core::BSTR>, lflags: i32, pctx: *mut ::core::ffi::c_void, presponsehandler: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub PutClass: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pobject: *mut ::core::ffi::c_void, lflags: i32, pctx: *mut ::core::ffi::c_void, ppcallresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub PutClassAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pobject: *mut ::core::ffi::c_void, lflags: i32, pctx: *mut ::core::ffi::c_void, presponsehandler: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DeleteClass: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strclass: ::std::mem::MaybeUninit<::windows::core::BSTR>, lflags: i32, pctx: *mut ::core::ffi::c_void, ppcallresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DeleteClassAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strclass: ::std::mem::MaybeUninit<::windows::core::BSTR>, lflags: i32, pctx: *mut ::core::ffi::c_void, presponsehandler: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateClassEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strsuperclass: ::std::mem::MaybeUninit<::windows::core::BSTR>, lflags: i32, pctx: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateClassEnumAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strsuperclass: ::std::mem::MaybeUninit<::windows::core::BSTR>, lflags: i32, pctx: *mut ::core::ffi::c_void, presponsehandler: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub PutInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinst: *mut ::core::ffi::c_void, lflags: i32, pctx: *mut ::core::ffi::c_void, ppcallresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub PutInstanceAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinst: *mut ::core::ffi::c_void, lflags: i32, pctx: *mut ::core::ffi::c_void, presponsehandler: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DeleteInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strobjectpath: ::std::mem::MaybeUninit<::windows::core::BSTR>, lflags: i32, pctx: *mut ::core::ffi::c_void, ppcallresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DeleteInstanceAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strobjectpath: ::std::mem::MaybeUninit<::windows::core::BSTR>, lflags: i32, pctx: *mut ::core::ffi::c_void, presponsehandler: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateInstanceEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strfilter: ::std::mem::MaybeUninit<::windows::core::BSTR>, lflags: i32, pctx: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateInstanceEnumAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strfilter: ::std::mem::MaybeUninit<::windows::core::BSTR>, lflags: i32, pctx: *mut ::core::ffi::c_void, presponsehandler: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ExecQuery: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strquerylanguage: ::std::mem::MaybeUninit<::windows::core::BSTR>, strquery: ::std::mem::MaybeUninit<::windows::core::BSTR>, lflags: WBEM_GENERIC_FLAG_TYPE, pctx: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ExecQueryAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strquerylanguage: ::std::mem::MaybeUninit<::windows::core::BSTR>, strquery: ::std::mem::MaybeUninit<::windows::core::BSTR>, lflags: i32, pctx: *mut ::core::ffi::c_void, presponsehandler: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ExecNotificationQuery: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strquerylanguage: ::std::mem::MaybeUninit<::windows::core::BSTR>, strquery: ::std::mem::MaybeUninit<::windows::core::BSTR>, lflags: i32, pctx: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ExecNotificationQueryAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strquerylanguage: ::std::mem::MaybeUninit<::windows::core::BSTR>, strquery: ::std::mem::MaybeUninit<::windows::core::BSTR>, lflags: i32, pctx: *mut ::core::ffi::c_void, presponsehandler: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ExecMethod: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strobjectpath: ::std::mem::MaybeUninit<::windows::core::BSTR>, strmethodname: ::std::mem::MaybeUninit<::windows::core::BSTR>, lflags: i32, pctx: *mut ::core::ffi::c_void, pinparams: *mut ::core::ffi::c_void, ppoutparams: *mut *mut ::core::ffi::c_void, ppcallresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ExecMethodAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strobjectpath: ::std::mem::MaybeUninit<::windows::core::BSTR>, strmethodname: ::std::mem::MaybeUninit<::windows::core::BSTR>, lflags: i32, pctx: *mut ::core::ffi::c_void, pinparams: *mut ::core::ffi::c_void, presponsehandler: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
#[repr(transparent)]
pub struct IWbemShutdown(::windows::core::IUnknown);
impl IWbemShutdown {
    pub unsafe fn Shutdown<P0>(&self, ureason: i32, umaxmilliseconds: u32, pctx: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IWbemContext>,
    {
        (::windows::core::Interface::vtable(self).Shutdown)(::windows::core::Interface::as_raw(self), ureason, umaxmilliseconds, pctx.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(IWbemShutdown, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IWbemShutdown {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemShutdown {}
impl ::core::fmt::Debug for IWbemShutdown {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWbemShutdown").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWbemShutdown {
    type Vtable = IWbemShutdown_Vtbl;
}
impl ::core::clone::Clone for IWbemShutdown {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWbemShutdown {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb7b31df9_d515_11d3_a11c_00105a1f515a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemShutdown_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Shutdown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ureason: i32, umaxmilliseconds: u32, pctx: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
#[repr(transparent)]
pub struct IWbemStatusCodeText(::windows::core::IUnknown);
impl IWbemStatusCodeText {
    pub unsafe fn GetErrorCodeText(&self, hres: ::windows::core::HRESULT, localeid: u32, lflags: i32) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).GetErrorCodeText)(::windows::core::Interface::as_raw(self), hres, localeid, lflags, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetFacilityCodeText(&self, hres: ::windows::core::HRESULT, localeid: u32, lflags: i32) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).GetFacilityCodeText)(::windows::core::Interface::as_raw(self), hres, localeid, lflags, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IWbemStatusCodeText, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IWbemStatusCodeText {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemStatusCodeText {}
impl ::core::fmt::Debug for IWbemStatusCodeText {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWbemStatusCodeText").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWbemStatusCodeText {
    type Vtable = IWbemStatusCodeText_Vtbl;
}
impl ::core::clone::Clone for IWbemStatusCodeText {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWbemStatusCodeText {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeb87e1bc_3233_11d2_aec9_00c04fb68820);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemStatusCodeText_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetErrorCodeText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hres: ::windows::core::HRESULT, localeid: u32, lflags: i32, messagetext: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub GetFacilityCodeText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hres: ::windows::core::HRESULT, localeid: u32, lflags: i32, messagetext: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
#[repr(transparent)]
pub struct IWbemTransport(::windows::core::IUnknown);
impl IWbemTransport {
    pub unsafe fn Initialize(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Initialize)(::windows::core::Interface::as_raw(self)).ok()
    }
}
::windows::imp::interface_hierarchy!(IWbemTransport, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IWbemTransport {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemTransport {}
impl ::core::fmt::Debug for IWbemTransport {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWbemTransport").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWbemTransport {
    type Vtable = IWbemTransport_Vtbl;
}
impl ::core::clone::Clone for IWbemTransport {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWbemTransport {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x553fe584_2156_11d0_b6ae_00aa003240c7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemTransport_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
#[repr(transparent)]
pub struct IWbemUnboundObjectSink(::windows::core::IUnknown);
impl IWbemUnboundObjectSink {
    pub unsafe fn IndicateToConsumer<P0>(&self, plogicalconsumer: P0, apobjects: &[::core::option::Option<IWbemClassObject>]) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IWbemClassObject>,
    {
        (::windows::core::Interface::vtable(self).IndicateToConsumer)(::windows::core::Interface::as_raw(self), plogicalconsumer.into_param().abi(), apobjects.len() as _, ::core::mem::transmute(apobjects.as_ptr())).ok()
    }
}
::windows::imp::interface_hierarchy!(IWbemUnboundObjectSink, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IWbemUnboundObjectSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemUnboundObjectSink {}
impl ::core::fmt::Debug for IWbemUnboundObjectSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWbemUnboundObjectSink").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWbemUnboundObjectSink {
    type Vtable = IWbemUnboundObjectSink_Vtbl;
}
impl ::core::clone::Clone for IWbemUnboundObjectSink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWbemUnboundObjectSink {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe246107b_b06e_11d0_ad61_00c04fd8fdff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemUnboundObjectSink_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub IndicateToConsumer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plogicalconsumer: *mut ::core::ffi::c_void, lnumobjects: i32, apobjects: *const *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
#[repr(transparent)]
pub struct IWbemUnsecuredApartment(::windows::core::IUnknown);
impl IWbemUnsecuredApartment {
    pub unsafe fn CreateObjectStub<P0>(&self, pobject: P0) -> ::windows::core::Result<::windows::core::IUnknown>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        let mut result__ = ::windows::core::zeroed::<::windows::core::IUnknown>();
        (::windows::core::Interface::vtable(self).base__.CreateObjectStub)(::windows::core::Interface::as_raw(self), pobject.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateSinkStub<P0, P1>(&self, psink: P0, dwflags: u32, wszreserved: P1) -> ::windows::core::Result<IWbemObjectSink>
    where
        P0: ::windows::core::IntoParam<IWbemObjectSink>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IWbemObjectSink>();
        (::windows::core::Interface::vtable(self).CreateSinkStub)(::windows::core::Interface::as_raw(self), psink.into_param().abi(), dwflags, wszreserved.into_param().abi(), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IWbemUnsecuredApartment, ::windows::core::IUnknown, IUnsecuredApartment);
impl ::core::cmp::PartialEq for IWbemUnsecuredApartment {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemUnsecuredApartment {}
impl ::core::fmt::Debug for IWbemUnsecuredApartment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWbemUnsecuredApartment").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWbemUnsecuredApartment {
    type Vtable = IWbemUnsecuredApartment_Vtbl;
}
impl ::core::clone::Clone for IWbemUnsecuredApartment {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWbemUnsecuredApartment {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x31739d04_3471_4cf4_9a7c_57a44ae71956);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemUnsecuredApartment_Vtbl {
    pub base__: IUnsecuredApartment_Vtbl,
    pub CreateSinkStub: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psink: *mut ::core::ffi::c_void, dwflags: u32, wszreserved: ::windows::core::PCWSTR, ppstub: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_CALL_VERSION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_CHAR_TYPE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_FLAG_ABSTRACT: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_FLAG_ADOPT: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_FLAG_ANY: u32 = 127u32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_FLAG_ASSOCIATION: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_FLAG_BORROW: u32 = 1073741824u32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_FLAG_CLASS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_FLAG_DISABLEOVERRIDE: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_FLAG_ENABLEOVERRIDE: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_FLAG_EXPENSIVE: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_FLAG_EXTENDED: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_FLAG_IN: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_FLAG_INDICATION: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_FLAG_KEY: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_FLAG_METHOD: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_FLAG_NOT_MODIFIED: u32 = 33554432u32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_FLAG_NULL: u32 = 536870912u32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_FLAG_OUT: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_FLAG_PARAMETER: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_FLAG_PROPERTY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_FLAG_READONLY: u32 = 2097152u32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_FLAG_REFERENCE: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_FLAG_REQUIRED: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_FLAG_RESTRICTED: u32 = 512u32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_FLAG_STATIC: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_FLAG_STREAM: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_FLAG_TERMINAL: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_FLAG_TOSUBCLASS: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_FLAG_TRANSLATABLE: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_FLAG_VERSION: u32 = 469762048u32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_MAX_LOCALE_SIZE: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_MODULE_FLAG_BOOLEANS: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_MODULE_FLAG_CPLUSPLUS: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_MODULE_FLAG_DESCRIPTIONS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_MODULE_FLAG_FILTER_SUPPORT: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_MODULE_FLAG_LOCALIZED: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_MODULE_FLAG_MAPPING_STRINGS: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_MODULE_FLAG_STANDARD_QUALIFIERS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_MODULE_FLAG_VALUES: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_OPERATIONFLAGS_BASIC_RTTI: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_OPERATIONFLAGS_DEFAULT_RTTI: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_OPERATIONFLAGS_EXPENSIVE_PROPERTIES: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_OPERATIONFLAGS_FULL_RTTI: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_OPERATIONFLAGS_LOCALIZED_QUALIFIERS: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_OPERATIONFLAGS_MANUAL_ACK_RESULTS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_OPERATIONFLAGS_NO_RTTI: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_OPERATIONFLAGS_POLYMORPHISM_DEEP_BASE_PROPS_ONLY: u32 = 384u32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_OPERATIONFLAGS_POLYMORPHISM_SHALLOW: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_OPERATIONFLAGS_REPORT_OPERATION_STARTED: u32 = 512u32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_OPERATIONFLAGS_STANDARD_RTTI: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_SERIALIZER_FLAGS_CLASS_DEEP: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_SERIALIZER_FLAGS_INSTANCE_WITH_CLASS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_SUBSCRIBE_BOOKMARK_NEWEST: ::windows::core::PCWSTR = ::windows::core::w!("MI_SUBSCRIBE_BOOKMARK_NEWEST");
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_SUBSCRIBE_BOOKMARK_OLDEST: ::windows::core::PCWSTR = ::windows::core::w!("MI_SUBSCRIBE_BOOKMARK_OLDEST");
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_WRITEMESSAGE_CHANNEL_DEBUG: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_WRITEMESSAGE_CHANNEL_VERBOSE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_WRITEMESSAGE_CHANNEL_WARNING: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MofCompiler: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6daf9757_2e37_11d2_aec9_00c04fb68820);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const SWbemDateTime: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x47dfbe54_cf76_11d3_b38f_00105a1f473a);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const SWbemEventSource: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x04b83d58_21ae_11d2_8b33_00600806d9b6);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const SWbemLastError: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc2feeeac_cfcd_11d1_8b05_00600806d9b6);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const SWbemLocator: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x76a64158_cb41_11d1_8b02_00600806d9b6);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const SWbemMethod: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x04b83d5b_21ae_11d2_8b33_00600806d9b6);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const SWbemMethodSet: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x04b83d5a_21ae_11d2_8b33_00600806d9b6);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const SWbemNamedValue: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x04b83d60_21ae_11d2_8b33_00600806d9b6);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const SWbemNamedValueSet: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9aed384e_ce8b_11d1_8b05_00600806d9b6);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const SWbemObject: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x04b83d62_21ae_11d2_8b33_00600806d9b6);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const SWbemObjectEx: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd6bdafb2_9435_491f_bb87_6aa0f0bc31a2);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const SWbemObjectPath: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5791bc26_ce9c_11d1_97bf_0000f81e849c);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const SWbemObjectSet: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x04b83d61_21ae_11d2_8b33_00600806d9b6);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const SWbemPrivilege: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x26ee67bc_5804_11d2_8b4a_00600806d9b6);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const SWbemPrivilegeSet: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x26ee67be_5804_11d2_8b4a_00600806d9b6);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const SWbemProperty: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x04b83d5d_21ae_11d2_8b33_00600806d9b6);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const SWbemPropertySet: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x04b83d5c_21ae_11d2_8b33_00600806d9b6);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const SWbemQualifier: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x04b83d5f_21ae_11d2_8b33_00600806d9b6);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const SWbemQualifierSet: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x04b83d5e_21ae_11d2_8b33_00600806d9b6);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const SWbemRefreshableItem: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8c6854bc_de4b_11d3_b390_00105a1f473a);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const SWbemRefresher: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd269bf5c_d9c1_11d3_b38f_00105a1f473a);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const SWbemSecurity: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb54d66e9_2287_11d2_8b33_00600806d9b6);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const SWbemServices: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x04b83d63_21ae_11d2_8b33_00600806d9b6);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const SWbemServicesEx: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x62e522dc_8cf3_40a8_8b2e_37d595651e40);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const SWbemSink: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x75718c9a_f029_11d1_a1ac_00c04fb6c223);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const UnsecuredApartment: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x49bd2028_1523_11d1_ad79_00c04fd8fdff);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMS_DISPID_COMPLETED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMS_DISPID_CONNECTION_READY: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMS_DISPID_DERIVATION: u32 = 23u32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMS_DISPID_OBJECT_PUT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMS_DISPID_OBJECT_READY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMS_DISPID_PROGRESS: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_INFINITE: i32 = -1i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_NO_WAIT: i32 = 0i32;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIExtension: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf0975afe_5c7f_11d2_8b74_00104b2afb41);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WbemAdministrativeLocator: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcb8555cc_9128_11d1_ad9b_00c04fd8fdff);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WbemAuthenticatedLocator: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcd184336_9128_11d1_ad9b_00c04fd8fdff);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WbemBackupRestore: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc49e32c6_bc8b_11d2_85d4_00105a1f8304);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WbemClassObject: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9a653086_174f_11d2_b5f9_00104b703efd);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WbemContext: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x674b6698_ee92_11d0_ad71_00c04fd8fdff);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WbemDCOMTransport: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf7ce2e13_8c90_11d1_9e7b_00c04fc324a8);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WbemDecoupledBasicEventProvider: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf5f75737_2843_4f22_933d_c76a97cda62f);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WbemDecoupledRegistrar: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4cfc7932_0f9d_4bef_9c32_8ea2a6b56fcb);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WbemDefPath: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcf4cc405_e2c5_4ddd_b3ce_5e7582d8c9fa);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WbemLevel1Login: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8bc3f05e_d86b_11d0_a075_00c04fb68820);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WbemLocalAddrRes: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa1044801_8f7e_11d1_9e7c_00c04fc324a8);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WbemLocator: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4590f811_1d3a_11d0_891f_00aa004b2e24);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WbemObjectTextSrc: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8d1c559d_84f0_4bb3_a7d5_56a7435a9ba6);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WbemQuery: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeac8a024_21e2_4523_ad73_a71a0aa2f56a);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WbemRefresher: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc71566f2_561e_11d1_ad87_00c04fd8fdff);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WbemStatusCodeText: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeb87e1bd_3233_11d2_aec9_00c04fb68820);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WbemUnauthenticatedLocator: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x443e7b79_de31_11d2_b340_00104bcc4b4a);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WbemUninitializedClassObject: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7a0227f6_7108_11d1_ad90_00c04fd8fdff);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CIMTYPE_ENUMERATION(pub i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const CIM_ILLEGAL: CIMTYPE_ENUMERATION = CIMTYPE_ENUMERATION(4095i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const CIM_EMPTY: CIMTYPE_ENUMERATION = CIMTYPE_ENUMERATION(0i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const CIM_SINT8: CIMTYPE_ENUMERATION = CIMTYPE_ENUMERATION(16i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const CIM_UINT8: CIMTYPE_ENUMERATION = CIMTYPE_ENUMERATION(17i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const CIM_SINT16: CIMTYPE_ENUMERATION = CIMTYPE_ENUMERATION(2i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const CIM_UINT16: CIMTYPE_ENUMERATION = CIMTYPE_ENUMERATION(18i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const CIM_SINT32: CIMTYPE_ENUMERATION = CIMTYPE_ENUMERATION(3i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const CIM_UINT32: CIMTYPE_ENUMERATION = CIMTYPE_ENUMERATION(19i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const CIM_SINT64: CIMTYPE_ENUMERATION = CIMTYPE_ENUMERATION(20i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const CIM_UINT64: CIMTYPE_ENUMERATION = CIMTYPE_ENUMERATION(21i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const CIM_REAL32: CIMTYPE_ENUMERATION = CIMTYPE_ENUMERATION(4i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const CIM_REAL64: CIMTYPE_ENUMERATION = CIMTYPE_ENUMERATION(5i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const CIM_BOOLEAN: CIMTYPE_ENUMERATION = CIMTYPE_ENUMERATION(11i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const CIM_STRING: CIMTYPE_ENUMERATION = CIMTYPE_ENUMERATION(8i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const CIM_DATETIME: CIMTYPE_ENUMERATION = CIMTYPE_ENUMERATION(101i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const CIM_REFERENCE: CIMTYPE_ENUMERATION = CIMTYPE_ENUMERATION(102i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const CIM_CHAR16: CIMTYPE_ENUMERATION = CIMTYPE_ENUMERATION(103i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const CIM_OBJECT: CIMTYPE_ENUMERATION = CIMTYPE_ENUMERATION(13i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const CIM_FLAG_ARRAY: CIMTYPE_ENUMERATION = CIMTYPE_ENUMERATION(8192i32);
impl ::core::marker::Copy for CIMTYPE_ENUMERATION {}
impl ::core::clone::Clone for CIMTYPE_ENUMERATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CIMTYPE_ENUMERATION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for CIMTYPE_ENUMERATION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for CIMTYPE_ENUMERATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CIMTYPE_ENUMERATION").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MI_CallbackMode(pub i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_CALLBACKMODE_REPORT: MI_CallbackMode = MI_CallbackMode(0i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_CALLBACKMODE_INQUIRE: MI_CallbackMode = MI_CallbackMode(1i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_CALLBACKMODE_IGNORE: MI_CallbackMode = MI_CallbackMode(2i32);
impl ::core::marker::Copy for MI_CallbackMode {}
impl ::core::clone::Clone for MI_CallbackMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MI_CallbackMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for MI_CallbackMode {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for MI_CallbackMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MI_CallbackMode").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MI_CancellationReason(pub i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_REASON_NONE: MI_CancellationReason = MI_CancellationReason(0i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_REASON_TIMEOUT: MI_CancellationReason = MI_CancellationReason(1i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_REASON_SHUTDOWN: MI_CancellationReason = MI_CancellationReason(2i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_REASON_SERVICESTOP: MI_CancellationReason = MI_CancellationReason(3i32);
impl ::core::marker::Copy for MI_CancellationReason {}
impl ::core::clone::Clone for MI_CancellationReason {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MI_CancellationReason {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for MI_CancellationReason {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for MI_CancellationReason {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MI_CancellationReason").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MI_DestinationOptions_ImpersonationType(pub i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_DestinationOptions_ImpersonationType_Default: MI_DestinationOptions_ImpersonationType = MI_DestinationOptions_ImpersonationType(0i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_DestinationOptions_ImpersonationType_None: MI_DestinationOptions_ImpersonationType = MI_DestinationOptions_ImpersonationType(1i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_DestinationOptions_ImpersonationType_Identify: MI_DestinationOptions_ImpersonationType = MI_DestinationOptions_ImpersonationType(2i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_DestinationOptions_ImpersonationType_Impersonate: MI_DestinationOptions_ImpersonationType = MI_DestinationOptions_ImpersonationType(3i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_DestinationOptions_ImpersonationType_Delegate: MI_DestinationOptions_ImpersonationType = MI_DestinationOptions_ImpersonationType(4i32);
impl ::core::marker::Copy for MI_DestinationOptions_ImpersonationType {}
impl ::core::clone::Clone for MI_DestinationOptions_ImpersonationType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MI_DestinationOptions_ImpersonationType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for MI_DestinationOptions_ImpersonationType {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for MI_DestinationOptions_ImpersonationType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MI_DestinationOptions_ImpersonationType").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MI_ErrorCategory(pub i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_ERRORCATEGORY_NOT_SPECIFIED: MI_ErrorCategory = MI_ErrorCategory(0i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_ERRORCATEGORY_OPEN_ERROR: MI_ErrorCategory = MI_ErrorCategory(1i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_ERRORCATEGORY_CLOS_EERROR: MI_ErrorCategory = MI_ErrorCategory(2i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_ERRORCATEGORY_DEVICE_ERROR: MI_ErrorCategory = MI_ErrorCategory(3i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_ERRORCATEGORY_DEADLOCK_DETECTED: MI_ErrorCategory = MI_ErrorCategory(4i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_ERRORCATEGORY_INVALID_ARGUMENT: MI_ErrorCategory = MI_ErrorCategory(5i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_ERRORCATEGORY_INVALID_DATA: MI_ErrorCategory = MI_ErrorCategory(6i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_ERRORCATEGORY_INVALID_OPERATION: MI_ErrorCategory = MI_ErrorCategory(7i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_ERRORCATEGORY_INVALID_RESULT: MI_ErrorCategory = MI_ErrorCategory(8i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_ERRORCATEGORY_INVALID_TYPE: MI_ErrorCategory = MI_ErrorCategory(9i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_ERRORCATEGORY_METADATA_ERROR: MI_ErrorCategory = MI_ErrorCategory(10i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_ERRORCATEGORY_NOT_IMPLEMENTED: MI_ErrorCategory = MI_ErrorCategory(11i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_ERRORCATEGORY_NOT_INSTALLED: MI_ErrorCategory = MI_ErrorCategory(12i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_ERRORCATEGORY_OBJECT_NOT_FOUND: MI_ErrorCategory = MI_ErrorCategory(13i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_ERRORCATEGORY_OPERATION_STOPPED: MI_ErrorCategory = MI_ErrorCategory(14i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_ERRORCATEGORY_OPERATION_TIMEOUT: MI_ErrorCategory = MI_ErrorCategory(15i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_ERRORCATEGORY_SYNTAX_ERROR: MI_ErrorCategory = MI_ErrorCategory(16i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_ERRORCATEGORY_PARSER_ERROR: MI_ErrorCategory = MI_ErrorCategory(17i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_ERRORCATEGORY_ACCESS_DENIED: MI_ErrorCategory = MI_ErrorCategory(18i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_ERRORCATEGORY_RESOURCE_BUSY: MI_ErrorCategory = MI_ErrorCategory(19i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_ERRORCATEGORY_RESOURCE_EXISTS: MI_ErrorCategory = MI_ErrorCategory(20i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_ERRORCATEGORY_RESOURCE_UNAVAILABLE: MI_ErrorCategory = MI_ErrorCategory(21i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_ERRORCATEGORY_READ_ERROR: MI_ErrorCategory = MI_ErrorCategory(22i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_ERRORCATEGORY_WRITE_ERROR: MI_ErrorCategory = MI_ErrorCategory(23i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_ERRORCATEGORY_FROM_STDERR: MI_ErrorCategory = MI_ErrorCategory(24i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_ERRORCATEGORY_SECURITY_ERROR: MI_ErrorCategory = MI_ErrorCategory(25i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_ERRORCATEGORY_PROTOCOL_ERROR: MI_ErrorCategory = MI_ErrorCategory(26i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_ERRORCATEGORY_CONNECTION_ERROR: MI_ErrorCategory = MI_ErrorCategory(27i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_ERRORCATEGORY_AUTHENTICATION_ERROR: MI_ErrorCategory = MI_ErrorCategory(28i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_ERRORCATEGORY_LIMITS_EXCEEDED: MI_ErrorCategory = MI_ErrorCategory(29i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_ERRORCATEGORY_QUOTA_EXCEEDED: MI_ErrorCategory = MI_ErrorCategory(30i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_ERRORCATEGORY_NOT_ENABLED: MI_ErrorCategory = MI_ErrorCategory(31i32);
impl ::core::marker::Copy for MI_ErrorCategory {}
impl ::core::clone::Clone for MI_ErrorCategory {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MI_ErrorCategory {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for MI_ErrorCategory {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for MI_ErrorCategory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MI_ErrorCategory").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MI_LocaleType(pub i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_LOCALE_TYPE_REQUESTED_UI: MI_LocaleType = MI_LocaleType(0i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_LOCALE_TYPE_REQUESTED_DATA: MI_LocaleType = MI_LocaleType(1i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_LOCALE_TYPE_CLOSEST_UI: MI_LocaleType = MI_LocaleType(2i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_LOCALE_TYPE_CLOSEST_DATA: MI_LocaleType = MI_LocaleType(3i32);
impl ::core::marker::Copy for MI_LocaleType {}
impl ::core::clone::Clone for MI_LocaleType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MI_LocaleType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for MI_LocaleType {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for MI_LocaleType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MI_LocaleType").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MI_OperationCallback_ResponseType(pub i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_OperationCallback_ResponseType_No: MI_OperationCallback_ResponseType = MI_OperationCallback_ResponseType(0i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_OperationCallback_ResponseType_Yes: MI_OperationCallback_ResponseType = MI_OperationCallback_ResponseType(1i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_OperationCallback_ResponseType_NoToAll: MI_OperationCallback_ResponseType = MI_OperationCallback_ResponseType(2i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_OperationCallback_ResponseType_YesToAll: MI_OperationCallback_ResponseType = MI_OperationCallback_ResponseType(3i32);
impl ::core::marker::Copy for MI_OperationCallback_ResponseType {}
impl ::core::clone::Clone for MI_OperationCallback_ResponseType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MI_OperationCallback_ResponseType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for MI_OperationCallback_ResponseType {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for MI_OperationCallback_ResponseType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MI_OperationCallback_ResponseType").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MI_PromptType(pub i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_PROMPTTYPE_NORMAL: MI_PromptType = MI_PromptType(0i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_PROMPTTYPE_CRITICAL: MI_PromptType = MI_PromptType(1i32);
impl ::core::marker::Copy for MI_PromptType {}
impl ::core::clone::Clone for MI_PromptType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MI_PromptType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for MI_PromptType {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for MI_PromptType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MI_PromptType").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MI_ProviderArchitecture(pub i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_PROVIDER_ARCHITECTURE_32BIT: MI_ProviderArchitecture = MI_ProviderArchitecture(0i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_PROVIDER_ARCHITECTURE_64BIT: MI_ProviderArchitecture = MI_ProviderArchitecture(1i32);
impl ::core::marker::Copy for MI_ProviderArchitecture {}
impl ::core::clone::Clone for MI_ProviderArchitecture {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MI_ProviderArchitecture {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for MI_ProviderArchitecture {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for MI_ProviderArchitecture {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MI_ProviderArchitecture").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MI_Result(pub i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_RESULT_OK: MI_Result = MI_Result(0i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_RESULT_FAILED: MI_Result = MI_Result(1i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_RESULT_ACCESS_DENIED: MI_Result = MI_Result(2i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_RESULT_INVALID_NAMESPACE: MI_Result = MI_Result(3i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_RESULT_INVALID_PARAMETER: MI_Result = MI_Result(4i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_RESULT_INVALID_CLASS: MI_Result = MI_Result(5i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_RESULT_NOT_FOUND: MI_Result = MI_Result(6i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_RESULT_NOT_SUPPORTED: MI_Result = MI_Result(7i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_RESULT_CLASS_HAS_CHILDREN: MI_Result = MI_Result(8i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_RESULT_CLASS_HAS_INSTANCES: MI_Result = MI_Result(9i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_RESULT_INVALID_SUPERCLASS: MI_Result = MI_Result(10i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_RESULT_ALREADY_EXISTS: MI_Result = MI_Result(11i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_RESULT_NO_SUCH_PROPERTY: MI_Result = MI_Result(12i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_RESULT_TYPE_MISMATCH: MI_Result = MI_Result(13i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_RESULT_QUERY_LANGUAGE_NOT_SUPPORTED: MI_Result = MI_Result(14i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_RESULT_INVALID_QUERY: MI_Result = MI_Result(15i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_RESULT_METHOD_NOT_AVAILABLE: MI_Result = MI_Result(16i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_RESULT_METHOD_NOT_FOUND: MI_Result = MI_Result(17i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_RESULT_NAMESPACE_NOT_EMPTY: MI_Result = MI_Result(20i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_RESULT_INVALID_ENUMERATION_CONTEXT: MI_Result = MI_Result(21i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_RESULT_INVALID_OPERATION_TIMEOUT: MI_Result = MI_Result(22i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_RESULT_PULL_HAS_BEEN_ABANDONED: MI_Result = MI_Result(23i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_RESULT_PULL_CANNOT_BE_ABANDONED: MI_Result = MI_Result(24i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_RESULT_FILTERED_ENUMERATION_NOT_SUPPORTED: MI_Result = MI_Result(25i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_RESULT_CONTINUATION_ON_ERROR_NOT_SUPPORTED: MI_Result = MI_Result(26i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_RESULT_SERVER_LIMITS_EXCEEDED: MI_Result = MI_Result(27i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_RESULT_SERVER_IS_SHUTTING_DOWN: MI_Result = MI_Result(28i32);
impl ::core::marker::Copy for MI_Result {}
impl ::core::clone::Clone for MI_Result {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MI_Result {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for MI_Result {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for MI_Result {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MI_Result").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MI_SubscriptionDeliveryType(pub i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_SubscriptionDeliveryType_Pull: MI_SubscriptionDeliveryType = MI_SubscriptionDeliveryType(1i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_SubscriptionDeliveryType_Push: MI_SubscriptionDeliveryType = MI_SubscriptionDeliveryType(2i32);
impl ::core::marker::Copy for MI_SubscriptionDeliveryType {}
impl ::core::clone::Clone for MI_SubscriptionDeliveryType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MI_SubscriptionDeliveryType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for MI_SubscriptionDeliveryType {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for MI_SubscriptionDeliveryType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MI_SubscriptionDeliveryType").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MI_Type(pub i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_BOOLEAN: MI_Type = MI_Type(0i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_UINT8: MI_Type = MI_Type(1i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_SINT8: MI_Type = MI_Type(2i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_UINT16: MI_Type = MI_Type(3i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_SINT16: MI_Type = MI_Type(4i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_UINT32: MI_Type = MI_Type(5i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_SINT32: MI_Type = MI_Type(6i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_UINT64: MI_Type = MI_Type(7i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_SINT64: MI_Type = MI_Type(8i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_REAL32: MI_Type = MI_Type(9i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_REAL64: MI_Type = MI_Type(10i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_CHAR16: MI_Type = MI_Type(11i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_DATETIME: MI_Type = MI_Type(12i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_STRING: MI_Type = MI_Type(13i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_REFERENCE: MI_Type = MI_Type(14i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_INSTANCE: MI_Type = MI_Type(15i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_BOOLEANA: MI_Type = MI_Type(16i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_UINT8A: MI_Type = MI_Type(17i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_SINT8A: MI_Type = MI_Type(18i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_UINT16A: MI_Type = MI_Type(19i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_SINT16A: MI_Type = MI_Type(20i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_UINT32A: MI_Type = MI_Type(21i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_SINT32A: MI_Type = MI_Type(22i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_UINT64A: MI_Type = MI_Type(23i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_SINT64A: MI_Type = MI_Type(24i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_REAL32A: MI_Type = MI_Type(25i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_REAL64A: MI_Type = MI_Type(26i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_CHAR16A: MI_Type = MI_Type(27i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_DATETIMEA: MI_Type = MI_Type(28i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_STRINGA: MI_Type = MI_Type(29i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_REFERENCEA: MI_Type = MI_Type(30i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_INSTANCEA: MI_Type = MI_Type(31i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const MI_ARRAY: MI_Type = MI_Type(16i32);
impl ::core::marker::Copy for MI_Type {}
impl ::core::clone::Clone for MI_Type {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MI_Type {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for MI_Type {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for MI_Type {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MI_Type").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WBEMSTATUS(pub i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_NO_ERROR: WBEMSTATUS = WBEMSTATUS(0i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_S_NO_ERROR: WBEMSTATUS = WBEMSTATUS(0i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_S_SAME: WBEMSTATUS = WBEMSTATUS(0i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_S_FALSE: WBEMSTATUS = WBEMSTATUS(1i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_S_ALREADY_EXISTS: WBEMSTATUS = WBEMSTATUS(262145i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_S_RESET_TO_DEFAULT: WBEMSTATUS = WBEMSTATUS(262146i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_S_DIFFERENT: WBEMSTATUS = WBEMSTATUS(262147i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_S_TIMEDOUT: WBEMSTATUS = WBEMSTATUS(262148i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_S_NO_MORE_DATA: WBEMSTATUS = WBEMSTATUS(262149i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_S_OPERATION_CANCELLED: WBEMSTATUS = WBEMSTATUS(262150i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_S_PENDING: WBEMSTATUS = WBEMSTATUS(262151i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_S_DUPLICATE_OBJECTS: WBEMSTATUS = WBEMSTATUS(262152i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_S_ACCESS_DENIED: WBEMSTATUS = WBEMSTATUS(262153i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_S_PARTIAL_RESULTS: WBEMSTATUS = WBEMSTATUS(262160i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_S_SOURCE_NOT_AVAILABLE: WBEMSTATUS = WBEMSTATUS(262167i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_FAILED: WBEMSTATUS = WBEMSTATUS(-2147217407i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_NOT_FOUND: WBEMSTATUS = WBEMSTATUS(-2147217406i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_ACCESS_DENIED: WBEMSTATUS = WBEMSTATUS(-2147217405i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_PROVIDER_FAILURE: WBEMSTATUS = WBEMSTATUS(-2147217404i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_TYPE_MISMATCH: WBEMSTATUS = WBEMSTATUS(-2147217403i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_OUT_OF_MEMORY: WBEMSTATUS = WBEMSTATUS(-2147217402i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_INVALID_CONTEXT: WBEMSTATUS = WBEMSTATUS(-2147217401i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_INVALID_PARAMETER: WBEMSTATUS = WBEMSTATUS(-2147217400i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_NOT_AVAILABLE: WBEMSTATUS = WBEMSTATUS(-2147217399i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_CRITICAL_ERROR: WBEMSTATUS = WBEMSTATUS(-2147217398i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_INVALID_STREAM: WBEMSTATUS = WBEMSTATUS(-2147217397i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_NOT_SUPPORTED: WBEMSTATUS = WBEMSTATUS(-2147217396i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_INVALID_SUPERCLASS: WBEMSTATUS = WBEMSTATUS(-2147217395i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_INVALID_NAMESPACE: WBEMSTATUS = WBEMSTATUS(-2147217394i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_INVALID_OBJECT: WBEMSTATUS = WBEMSTATUS(-2147217393i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_INVALID_CLASS: WBEMSTATUS = WBEMSTATUS(-2147217392i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_PROVIDER_NOT_FOUND: WBEMSTATUS = WBEMSTATUS(-2147217391i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_INVALID_PROVIDER_REGISTRATION: WBEMSTATUS = WBEMSTATUS(-2147217390i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_PROVIDER_LOAD_FAILURE: WBEMSTATUS = WBEMSTATUS(-2147217389i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_INITIALIZATION_FAILURE: WBEMSTATUS = WBEMSTATUS(-2147217388i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_TRANSPORT_FAILURE: WBEMSTATUS = WBEMSTATUS(-2147217387i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_INVALID_OPERATION: WBEMSTATUS = WBEMSTATUS(-2147217386i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_INVALID_QUERY: WBEMSTATUS = WBEMSTATUS(-2147217385i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_INVALID_QUERY_TYPE: WBEMSTATUS = WBEMSTATUS(-2147217384i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_ALREADY_EXISTS: WBEMSTATUS = WBEMSTATUS(-2147217383i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_OVERRIDE_NOT_ALLOWED: WBEMSTATUS = WBEMSTATUS(-2147217382i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_PROPAGATED_QUALIFIER: WBEMSTATUS = WBEMSTATUS(-2147217381i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_PROPAGATED_PROPERTY: WBEMSTATUS = WBEMSTATUS(-2147217380i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_UNEXPECTED: WBEMSTATUS = WBEMSTATUS(-2147217379i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_ILLEGAL_OPERATION: WBEMSTATUS = WBEMSTATUS(-2147217378i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_CANNOT_BE_KEY: WBEMSTATUS = WBEMSTATUS(-2147217377i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_INCOMPLETE_CLASS: WBEMSTATUS = WBEMSTATUS(-2147217376i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_INVALID_SYNTAX: WBEMSTATUS = WBEMSTATUS(-2147217375i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_NONDECORATED_OBJECT: WBEMSTATUS = WBEMSTATUS(-2147217374i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_READ_ONLY: WBEMSTATUS = WBEMSTATUS(-2147217373i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_PROVIDER_NOT_CAPABLE: WBEMSTATUS = WBEMSTATUS(-2147217372i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_CLASS_HAS_CHILDREN: WBEMSTATUS = WBEMSTATUS(-2147217371i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_CLASS_HAS_INSTANCES: WBEMSTATUS = WBEMSTATUS(-2147217370i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_QUERY_NOT_IMPLEMENTED: WBEMSTATUS = WBEMSTATUS(-2147217369i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_ILLEGAL_NULL: WBEMSTATUS = WBEMSTATUS(-2147217368i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_INVALID_QUALIFIER_TYPE: WBEMSTATUS = WBEMSTATUS(-2147217367i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_INVALID_PROPERTY_TYPE: WBEMSTATUS = WBEMSTATUS(-2147217366i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_VALUE_OUT_OF_RANGE: WBEMSTATUS = WBEMSTATUS(-2147217365i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_CANNOT_BE_SINGLETON: WBEMSTATUS = WBEMSTATUS(-2147217364i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_INVALID_CIM_TYPE: WBEMSTATUS = WBEMSTATUS(-2147217363i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_INVALID_METHOD: WBEMSTATUS = WBEMSTATUS(-2147217362i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_INVALID_METHOD_PARAMETERS: WBEMSTATUS = WBEMSTATUS(-2147217361i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_SYSTEM_PROPERTY: WBEMSTATUS = WBEMSTATUS(-2147217360i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_INVALID_PROPERTY: WBEMSTATUS = WBEMSTATUS(-2147217359i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_CALL_CANCELLED: WBEMSTATUS = WBEMSTATUS(-2147217358i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_SHUTTING_DOWN: WBEMSTATUS = WBEMSTATUS(-2147217357i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_PROPAGATED_METHOD: WBEMSTATUS = WBEMSTATUS(-2147217356i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_UNSUPPORTED_PARAMETER: WBEMSTATUS = WBEMSTATUS(-2147217355i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_MISSING_PARAMETER_ID: WBEMSTATUS = WBEMSTATUS(-2147217354i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_INVALID_PARAMETER_ID: WBEMSTATUS = WBEMSTATUS(-2147217353i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_NONCONSECUTIVE_PARAMETER_IDS: WBEMSTATUS = WBEMSTATUS(-2147217352i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_PARAMETER_ID_ON_RETVAL: WBEMSTATUS = WBEMSTATUS(-2147217351i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_INVALID_OBJECT_PATH: WBEMSTATUS = WBEMSTATUS(-2147217350i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_OUT_OF_DISK_SPACE: WBEMSTATUS = WBEMSTATUS(-2147217349i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_BUFFER_TOO_SMALL: WBEMSTATUS = WBEMSTATUS(-2147217348i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_UNSUPPORTED_PUT_EXTENSION: WBEMSTATUS = WBEMSTATUS(-2147217347i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_UNKNOWN_OBJECT_TYPE: WBEMSTATUS = WBEMSTATUS(-2147217346i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_UNKNOWN_PACKET_TYPE: WBEMSTATUS = WBEMSTATUS(-2147217345i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_MARSHAL_VERSION_MISMATCH: WBEMSTATUS = WBEMSTATUS(-2147217344i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_MARSHAL_INVALID_SIGNATURE: WBEMSTATUS = WBEMSTATUS(-2147217343i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_INVALID_QUALIFIER: WBEMSTATUS = WBEMSTATUS(-2147217342i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_INVALID_DUPLICATE_PARAMETER: WBEMSTATUS = WBEMSTATUS(-2147217341i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_TOO_MUCH_DATA: WBEMSTATUS = WBEMSTATUS(-2147217340i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_SERVER_TOO_BUSY: WBEMSTATUS = WBEMSTATUS(-2147217339i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_INVALID_FLAVOR: WBEMSTATUS = WBEMSTATUS(-2147217338i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_CIRCULAR_REFERENCE: WBEMSTATUS = WBEMSTATUS(-2147217337i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_UNSUPPORTED_CLASS_UPDATE: WBEMSTATUS = WBEMSTATUS(-2147217336i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_CANNOT_CHANGE_KEY_INHERITANCE: WBEMSTATUS = WBEMSTATUS(-2147217335i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_CANNOT_CHANGE_INDEX_INHERITANCE: WBEMSTATUS = WBEMSTATUS(-2147217328i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_TOO_MANY_PROPERTIES: WBEMSTATUS = WBEMSTATUS(-2147217327i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_UPDATE_TYPE_MISMATCH: WBEMSTATUS = WBEMSTATUS(-2147217326i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_UPDATE_OVERRIDE_NOT_ALLOWED: WBEMSTATUS = WBEMSTATUS(-2147217325i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_UPDATE_PROPAGATED_METHOD: WBEMSTATUS = WBEMSTATUS(-2147217324i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_METHOD_NOT_IMPLEMENTED: WBEMSTATUS = WBEMSTATUS(-2147217323i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_METHOD_DISABLED: WBEMSTATUS = WBEMSTATUS(-2147217322i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_REFRESHER_BUSY: WBEMSTATUS = WBEMSTATUS(-2147217321i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_UNPARSABLE_QUERY: WBEMSTATUS = WBEMSTATUS(-2147217320i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_NOT_EVENT_CLASS: WBEMSTATUS = WBEMSTATUS(-2147217319i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_MISSING_GROUP_WITHIN: WBEMSTATUS = WBEMSTATUS(-2147217318i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_MISSING_AGGREGATION_LIST: WBEMSTATUS = WBEMSTATUS(-2147217317i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_PROPERTY_NOT_AN_OBJECT: WBEMSTATUS = WBEMSTATUS(-2147217316i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_AGGREGATING_BY_OBJECT: WBEMSTATUS = WBEMSTATUS(-2147217315i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_UNINTERPRETABLE_PROVIDER_QUERY: WBEMSTATUS = WBEMSTATUS(-2147217313i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_BACKUP_RESTORE_WINMGMT_RUNNING: WBEMSTATUS = WBEMSTATUS(-2147217312i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_QUEUE_OVERFLOW: WBEMSTATUS = WBEMSTATUS(-2147217311i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_PRIVILEGE_NOT_HELD: WBEMSTATUS = WBEMSTATUS(-2147217310i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_INVALID_OPERATOR: WBEMSTATUS = WBEMSTATUS(-2147217309i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_LOCAL_CREDENTIALS: WBEMSTATUS = WBEMSTATUS(-2147217308i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_CANNOT_BE_ABSTRACT: WBEMSTATUS = WBEMSTATUS(-2147217307i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_AMENDED_OBJECT: WBEMSTATUS = WBEMSTATUS(-2147217306i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_CLIENT_TOO_SLOW: WBEMSTATUS = WBEMSTATUS(-2147217305i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_NULL_SECURITY_DESCRIPTOR: WBEMSTATUS = WBEMSTATUS(-2147217304i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_TIMED_OUT: WBEMSTATUS = WBEMSTATUS(-2147217303i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_INVALID_ASSOCIATION: WBEMSTATUS = WBEMSTATUS(-2147217302i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_AMBIGUOUS_OPERATION: WBEMSTATUS = WBEMSTATUS(-2147217301i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_QUOTA_VIOLATION: WBEMSTATUS = WBEMSTATUS(-2147217300i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_RESERVED_001: WBEMSTATUS = WBEMSTATUS(-2147217299i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_RESERVED_002: WBEMSTATUS = WBEMSTATUS(-2147217298i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_UNSUPPORTED_LOCALE: WBEMSTATUS = WBEMSTATUS(-2147217297i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_HANDLE_OUT_OF_DATE: WBEMSTATUS = WBEMSTATUS(-2147217296i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_CONNECTION_FAILED: WBEMSTATUS = WBEMSTATUS(-2147217295i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_INVALID_HANDLE_REQUEST: WBEMSTATUS = WBEMSTATUS(-2147217294i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_PROPERTY_NAME_TOO_WIDE: WBEMSTATUS = WBEMSTATUS(-2147217293i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_CLASS_NAME_TOO_WIDE: WBEMSTATUS = WBEMSTATUS(-2147217292i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_METHOD_NAME_TOO_WIDE: WBEMSTATUS = WBEMSTATUS(-2147217291i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_QUALIFIER_NAME_TOO_WIDE: WBEMSTATUS = WBEMSTATUS(-2147217290i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_RERUN_COMMAND: WBEMSTATUS = WBEMSTATUS(-2147217289i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_DATABASE_VER_MISMATCH: WBEMSTATUS = WBEMSTATUS(-2147217288i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_VETO_DELETE: WBEMSTATUS = WBEMSTATUS(-2147217287i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_VETO_PUT: WBEMSTATUS = WBEMSTATUS(-2147217286i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_INVALID_LOCALE: WBEMSTATUS = WBEMSTATUS(-2147217280i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_PROVIDER_SUSPENDED: WBEMSTATUS = WBEMSTATUS(-2147217279i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_SYNCHRONIZATION_REQUIRED: WBEMSTATUS = WBEMSTATUS(-2147217278i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_NO_SCHEMA: WBEMSTATUS = WBEMSTATUS(-2147217277i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_PROVIDER_ALREADY_REGISTERED: WBEMSTATUS = WBEMSTATUS(-2147217276i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_PROVIDER_NOT_REGISTERED: WBEMSTATUS = WBEMSTATUS(-2147217275i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_FATAL_TRANSPORT_ERROR: WBEMSTATUS = WBEMSTATUS(-2147217274i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_ENCRYPTED_CONNECTION_REQUIRED: WBEMSTATUS = WBEMSTATUS(-2147217273i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_PROVIDER_TIMED_OUT: WBEMSTATUS = WBEMSTATUS(-2147217272i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_NO_KEY: WBEMSTATUS = WBEMSTATUS(-2147217271i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_PROVIDER_DISABLED: WBEMSTATUS = WBEMSTATUS(-2147217270i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMESS_E_REGISTRATION_TOO_BROAD: WBEMSTATUS = WBEMSTATUS(-2147213311i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMESS_E_REGISTRATION_TOO_PRECISE: WBEMSTATUS = WBEMSTATUS(-2147213310i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMESS_E_AUTHZ_NOT_PRIVILEGED: WBEMSTATUS = WBEMSTATUS(-2147213309i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMMOF_E_EXPECTED_QUALIFIER_NAME: WBEMSTATUS = WBEMSTATUS(-2147205119i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMMOF_E_EXPECTED_SEMI: WBEMSTATUS = WBEMSTATUS(-2147205118i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMMOF_E_EXPECTED_OPEN_BRACE: WBEMSTATUS = WBEMSTATUS(-2147205117i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMMOF_E_EXPECTED_CLOSE_BRACE: WBEMSTATUS = WBEMSTATUS(-2147205116i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMMOF_E_EXPECTED_CLOSE_BRACKET: WBEMSTATUS = WBEMSTATUS(-2147205115i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMMOF_E_EXPECTED_CLOSE_PAREN: WBEMSTATUS = WBEMSTATUS(-2147205114i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMMOF_E_ILLEGAL_CONSTANT_VALUE: WBEMSTATUS = WBEMSTATUS(-2147205113i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMMOF_E_EXPECTED_TYPE_IDENTIFIER: WBEMSTATUS = WBEMSTATUS(-2147205112i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMMOF_E_EXPECTED_OPEN_PAREN: WBEMSTATUS = WBEMSTATUS(-2147205111i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMMOF_E_UNRECOGNIZED_TOKEN: WBEMSTATUS = WBEMSTATUS(-2147205110i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMMOF_E_UNRECOGNIZED_TYPE: WBEMSTATUS = WBEMSTATUS(-2147205109i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMMOF_E_EXPECTED_PROPERTY_NAME: WBEMSTATUS = WBEMSTATUS(-2147205108i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMMOF_E_TYPEDEF_NOT_SUPPORTED: WBEMSTATUS = WBEMSTATUS(-2147205107i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMMOF_E_UNEXPECTED_ALIAS: WBEMSTATUS = WBEMSTATUS(-2147205106i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMMOF_E_UNEXPECTED_ARRAY_INIT: WBEMSTATUS = WBEMSTATUS(-2147205105i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMMOF_E_INVALID_AMENDMENT_SYNTAX: WBEMSTATUS = WBEMSTATUS(-2147205104i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMMOF_E_INVALID_DUPLICATE_AMENDMENT: WBEMSTATUS = WBEMSTATUS(-2147205103i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMMOF_E_INVALID_PRAGMA: WBEMSTATUS = WBEMSTATUS(-2147205102i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMMOF_E_INVALID_NAMESPACE_SYNTAX: WBEMSTATUS = WBEMSTATUS(-2147205101i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMMOF_E_EXPECTED_CLASS_NAME: WBEMSTATUS = WBEMSTATUS(-2147205100i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMMOF_E_TYPE_MISMATCH: WBEMSTATUS = WBEMSTATUS(-2147205099i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMMOF_E_EXPECTED_ALIAS_NAME: WBEMSTATUS = WBEMSTATUS(-2147205098i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMMOF_E_INVALID_CLASS_DECLARATION: WBEMSTATUS = WBEMSTATUS(-2147205097i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMMOF_E_INVALID_INSTANCE_DECLARATION: WBEMSTATUS = WBEMSTATUS(-2147205096i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMMOF_E_EXPECTED_DOLLAR: WBEMSTATUS = WBEMSTATUS(-2147205095i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMMOF_E_CIMTYPE_QUALIFIER: WBEMSTATUS = WBEMSTATUS(-2147205094i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMMOF_E_DUPLICATE_PROPERTY: WBEMSTATUS = WBEMSTATUS(-2147205093i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMMOF_E_INVALID_NAMESPACE_SPECIFICATION: WBEMSTATUS = WBEMSTATUS(-2147205092i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMMOF_E_OUT_OF_RANGE: WBEMSTATUS = WBEMSTATUS(-2147205091i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMMOF_E_INVALID_FILE: WBEMSTATUS = WBEMSTATUS(-2147205090i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMMOF_E_ALIASES_IN_EMBEDDED: WBEMSTATUS = WBEMSTATUS(-2147205089i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMMOF_E_NULL_ARRAY_ELEM: WBEMSTATUS = WBEMSTATUS(-2147205088i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMMOF_E_DUPLICATE_QUALIFIER: WBEMSTATUS = WBEMSTATUS(-2147205087i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMMOF_E_EXPECTED_FLAVOR_TYPE: WBEMSTATUS = WBEMSTATUS(-2147205086i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMMOF_E_INCOMPATIBLE_FLAVOR_TYPES: WBEMSTATUS = WBEMSTATUS(-2147205085i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMMOF_E_MULTIPLE_ALIASES: WBEMSTATUS = WBEMSTATUS(-2147205084i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMMOF_E_INCOMPATIBLE_FLAVOR_TYPES2: WBEMSTATUS = WBEMSTATUS(-2147205083i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMMOF_E_NO_ARRAYS_RETURNED: WBEMSTATUS = WBEMSTATUS(-2147205082i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMMOF_E_MUST_BE_IN_OR_OUT: WBEMSTATUS = WBEMSTATUS(-2147205081i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMMOF_E_INVALID_FLAGS_SYNTAX: WBEMSTATUS = WBEMSTATUS(-2147205080i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMMOF_E_EXPECTED_BRACE_OR_BAD_TYPE: WBEMSTATUS = WBEMSTATUS(-2147205079i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMMOF_E_UNSUPPORTED_CIMV22_QUAL_VALUE: WBEMSTATUS = WBEMSTATUS(-2147205078i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMMOF_E_UNSUPPORTED_CIMV22_DATA_TYPE: WBEMSTATUS = WBEMSTATUS(-2147205077i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMMOF_E_INVALID_DELETEINSTANCE_SYNTAX: WBEMSTATUS = WBEMSTATUS(-2147205076i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMMOF_E_INVALID_QUALIFIER_SYNTAX: WBEMSTATUS = WBEMSTATUS(-2147205075i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMMOF_E_QUALIFIER_USED_OUTSIDE_SCOPE: WBEMSTATUS = WBEMSTATUS(-2147205074i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMMOF_E_ERROR_CREATING_TEMP_FILE: WBEMSTATUS = WBEMSTATUS(-2147205073i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMMOF_E_ERROR_INVALID_INCLUDE_FILE: WBEMSTATUS = WBEMSTATUS(-2147205072i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMMOF_E_INVALID_DELETECLASS_SYNTAX: WBEMSTATUS = WBEMSTATUS(-2147205071i32);
impl ::core::marker::Copy for WBEMSTATUS {}
impl ::core::clone::Clone for WBEMSTATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WBEMSTATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WBEMSTATUS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WBEMSTATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WBEMSTATUS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WBEMSTATUS_FORMAT(pub i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMSTATUS_FORMAT_NEWLINE: WBEMSTATUS_FORMAT = WBEMSTATUS_FORMAT(0i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMSTATUS_FORMAT_NO_NEWLINE: WBEMSTATUS_FORMAT = WBEMSTATUS_FORMAT(1i32);
impl ::core::marker::Copy for WBEMSTATUS_FORMAT {}
impl ::core::clone::Clone for WBEMSTATUS_FORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WBEMSTATUS_FORMAT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WBEMSTATUS_FORMAT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WBEMSTATUS_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WBEMSTATUS_FORMAT").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WBEM_BACKUP_RESTORE_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAG_BACKUP_RESTORE_DEFAULT: WBEM_BACKUP_RESTORE_FLAGS = WBEM_BACKUP_RESTORE_FLAGS(0i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAG_BACKUP_RESTORE_FORCE_SHUTDOWN: WBEM_BACKUP_RESTORE_FLAGS = WBEM_BACKUP_RESTORE_FLAGS(1i32);
impl ::core::marker::Copy for WBEM_BACKUP_RESTORE_FLAGS {}
impl ::core::clone::Clone for WBEM_BACKUP_RESTORE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WBEM_BACKUP_RESTORE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WBEM_BACKUP_RESTORE_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WBEM_BACKUP_RESTORE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WBEM_BACKUP_RESTORE_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WBEM_BATCH_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAG_BATCH_IF_NEEDED: WBEM_BATCH_TYPE = WBEM_BATCH_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAG_MUST_BATCH: WBEM_BATCH_TYPE = WBEM_BATCH_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAG_MUST_NOT_BATCH: WBEM_BATCH_TYPE = WBEM_BATCH_TYPE(2i32);
impl ::core::marker::Copy for WBEM_BATCH_TYPE {}
impl ::core::clone::Clone for WBEM_BATCH_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WBEM_BATCH_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WBEM_BATCH_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WBEM_BATCH_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WBEM_BATCH_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WBEM_CHANGE_FLAG_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAG_CREATE_OR_UPDATE: WBEM_CHANGE_FLAG_TYPE = WBEM_CHANGE_FLAG_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAG_UPDATE_ONLY: WBEM_CHANGE_FLAG_TYPE = WBEM_CHANGE_FLAG_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAG_CREATE_ONLY: WBEM_CHANGE_FLAG_TYPE = WBEM_CHANGE_FLAG_TYPE(2i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAG_UPDATE_COMPATIBLE: WBEM_CHANGE_FLAG_TYPE = WBEM_CHANGE_FLAG_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAG_UPDATE_SAFE_MODE: WBEM_CHANGE_FLAG_TYPE = WBEM_CHANGE_FLAG_TYPE(32i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAG_UPDATE_FORCE_MODE: WBEM_CHANGE_FLAG_TYPE = WBEM_CHANGE_FLAG_TYPE(64i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_MASK_UPDATE_MODE: WBEM_CHANGE_FLAG_TYPE = WBEM_CHANGE_FLAG_TYPE(96i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAG_ADVISORY: WBEM_CHANGE_FLAG_TYPE = WBEM_CHANGE_FLAG_TYPE(65536i32);
impl ::core::marker::Copy for WBEM_CHANGE_FLAG_TYPE {}
impl ::core::clone::Clone for WBEM_CHANGE_FLAG_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WBEM_CHANGE_FLAG_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WBEM_CHANGE_FLAG_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WBEM_CHANGE_FLAG_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WBEM_CHANGE_FLAG_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WBEM_COMPARISON_FLAG(pub i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_COMPARISON_INCLUDE_ALL: WBEM_COMPARISON_FLAG = WBEM_COMPARISON_FLAG(0i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAG_IGNORE_QUALIFIERS: WBEM_COMPARISON_FLAG = WBEM_COMPARISON_FLAG(1i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAG_IGNORE_OBJECT_SOURCE: WBEM_COMPARISON_FLAG = WBEM_COMPARISON_FLAG(2i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAG_IGNORE_DEFAULT_VALUES: WBEM_COMPARISON_FLAG = WBEM_COMPARISON_FLAG(4i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAG_IGNORE_CLASS: WBEM_COMPARISON_FLAG = WBEM_COMPARISON_FLAG(8i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAG_IGNORE_CASE: WBEM_COMPARISON_FLAG = WBEM_COMPARISON_FLAG(16i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAG_IGNORE_FLAVOR: WBEM_COMPARISON_FLAG = WBEM_COMPARISON_FLAG(32i32);
impl ::core::marker::Copy for WBEM_COMPARISON_FLAG {}
impl ::core::clone::Clone for WBEM_COMPARISON_FLAG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WBEM_COMPARISON_FLAG {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WBEM_COMPARISON_FLAG {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WBEM_COMPARISON_FLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WBEM_COMPARISON_FLAG").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WBEM_COMPILER_OPTIONS(pub i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAG_CHECK_ONLY: WBEM_COMPILER_OPTIONS = WBEM_COMPILER_OPTIONS(1i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAG_AUTORECOVER: WBEM_COMPILER_OPTIONS = WBEM_COMPILER_OPTIONS(2i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAG_WMI_CHECK: WBEM_COMPILER_OPTIONS = WBEM_COMPILER_OPTIONS(4i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAG_CONSOLE_PRINT: WBEM_COMPILER_OPTIONS = WBEM_COMPILER_OPTIONS(8i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAG_DONT_ADD_TO_LIST: WBEM_COMPILER_OPTIONS = WBEM_COMPILER_OPTIONS(16i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAG_SPLIT_FILES: WBEM_COMPILER_OPTIONS = WBEM_COMPILER_OPTIONS(32i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAG_STORE_FILE: WBEM_COMPILER_OPTIONS = WBEM_COMPILER_OPTIONS(256i32);
impl ::core::marker::Copy for WBEM_COMPILER_OPTIONS {}
impl ::core::clone::Clone for WBEM_COMPILER_OPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WBEM_COMPILER_OPTIONS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WBEM_COMPILER_OPTIONS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WBEM_COMPILER_OPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WBEM_COMPILER_OPTIONS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WBEM_CONDITION_FLAG_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAG_ALWAYS: WBEM_CONDITION_FLAG_TYPE = WBEM_CONDITION_FLAG_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAG_ONLY_IF_TRUE: WBEM_CONDITION_FLAG_TYPE = WBEM_CONDITION_FLAG_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAG_ONLY_IF_FALSE: WBEM_CONDITION_FLAG_TYPE = WBEM_CONDITION_FLAG_TYPE(2i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAG_ONLY_IF_IDENTICAL: WBEM_CONDITION_FLAG_TYPE = WBEM_CONDITION_FLAG_TYPE(3i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_MASK_PRIMARY_CONDITION: WBEM_CONDITION_FLAG_TYPE = WBEM_CONDITION_FLAG_TYPE(3i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAG_KEYS_ONLY: WBEM_CONDITION_FLAG_TYPE = WBEM_CONDITION_FLAG_TYPE(4i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAG_REFS_ONLY: WBEM_CONDITION_FLAG_TYPE = WBEM_CONDITION_FLAG_TYPE(8i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAG_LOCAL_ONLY: WBEM_CONDITION_FLAG_TYPE = WBEM_CONDITION_FLAG_TYPE(16i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAG_PROPAGATED_ONLY: WBEM_CONDITION_FLAG_TYPE = WBEM_CONDITION_FLAG_TYPE(32i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAG_SYSTEM_ONLY: WBEM_CONDITION_FLAG_TYPE = WBEM_CONDITION_FLAG_TYPE(48i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAG_NONSYSTEM_ONLY: WBEM_CONDITION_FLAG_TYPE = WBEM_CONDITION_FLAG_TYPE(64i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_MASK_CONDITION_ORIGIN: WBEM_CONDITION_FLAG_TYPE = WBEM_CONDITION_FLAG_TYPE(112i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAG_CLASS_OVERRIDES_ONLY: WBEM_CONDITION_FLAG_TYPE = WBEM_CONDITION_FLAG_TYPE(256i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAG_CLASS_LOCAL_AND_OVERRIDES: WBEM_CONDITION_FLAG_TYPE = WBEM_CONDITION_FLAG_TYPE(512i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_MASK_CLASS_CONDITION: WBEM_CONDITION_FLAG_TYPE = WBEM_CONDITION_FLAG_TYPE(768i32);
impl ::core::marker::Copy for WBEM_CONDITION_FLAG_TYPE {}
impl ::core::clone::Clone for WBEM_CONDITION_FLAG_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WBEM_CONDITION_FLAG_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WBEM_CONDITION_FLAG_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WBEM_CONDITION_FLAG_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WBEM_CONDITION_FLAG_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WBEM_CONNECT_OPTIONS(pub i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAG_CONNECT_REPOSITORY_ONLY: WBEM_CONNECT_OPTIONS = WBEM_CONNECT_OPTIONS(64i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAG_CONNECT_USE_MAX_WAIT: WBEM_CONNECT_OPTIONS = WBEM_CONNECT_OPTIONS(128i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAG_CONNECT_PROVIDERS: WBEM_CONNECT_OPTIONS = WBEM_CONNECT_OPTIONS(256i32);
impl ::core::marker::Copy for WBEM_CONNECT_OPTIONS {}
impl ::core::clone::Clone for WBEM_CONNECT_OPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WBEM_CONNECT_OPTIONS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WBEM_CONNECT_OPTIONS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WBEM_CONNECT_OPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WBEM_CONNECT_OPTIONS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WBEM_EXTRA_RETURN_CODES(pub i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_S_INITIALIZED: WBEM_EXTRA_RETURN_CODES = WBEM_EXTRA_RETURN_CODES(0i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_S_LIMITED_SERVICE: WBEM_EXTRA_RETURN_CODES = WBEM_EXTRA_RETURN_CODES(274433i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_S_INDIRECTLY_UPDATED: WBEM_EXTRA_RETURN_CODES = WBEM_EXTRA_RETURN_CODES(274434i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_S_SUBJECT_TO_SDS: WBEM_EXTRA_RETURN_CODES = WBEM_EXTRA_RETURN_CODES(274435i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_RETRY_LATER: WBEM_EXTRA_RETURN_CODES = WBEM_EXTRA_RETURN_CODES(-2147209215i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_E_RESOURCE_CONTENTION: WBEM_EXTRA_RETURN_CODES = WBEM_EXTRA_RETURN_CODES(-2147209214i32);
impl ::core::marker::Copy for WBEM_EXTRA_RETURN_CODES {}
impl ::core::clone::Clone for WBEM_EXTRA_RETURN_CODES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WBEM_EXTRA_RETURN_CODES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WBEM_EXTRA_RETURN_CODES {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WBEM_EXTRA_RETURN_CODES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WBEM_EXTRA_RETURN_CODES").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WBEM_FLAVOR_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAVOR_DONT_PROPAGATE: WBEM_FLAVOR_TYPE = WBEM_FLAVOR_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAVOR_FLAG_PROPAGATE_TO_INSTANCE: WBEM_FLAVOR_TYPE = WBEM_FLAVOR_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAVOR_FLAG_PROPAGATE_TO_DERIVED_CLASS: WBEM_FLAVOR_TYPE = WBEM_FLAVOR_TYPE(2i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAVOR_MASK_PROPAGATION: WBEM_FLAVOR_TYPE = WBEM_FLAVOR_TYPE(15i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAVOR_OVERRIDABLE: WBEM_FLAVOR_TYPE = WBEM_FLAVOR_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAVOR_NOT_OVERRIDABLE: WBEM_FLAVOR_TYPE = WBEM_FLAVOR_TYPE(16i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAVOR_MASK_PERMISSIONS: WBEM_FLAVOR_TYPE = WBEM_FLAVOR_TYPE(16i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAVOR_ORIGIN_LOCAL: WBEM_FLAVOR_TYPE = WBEM_FLAVOR_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAVOR_ORIGIN_PROPAGATED: WBEM_FLAVOR_TYPE = WBEM_FLAVOR_TYPE(32i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAVOR_ORIGIN_SYSTEM: WBEM_FLAVOR_TYPE = WBEM_FLAVOR_TYPE(64i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAVOR_MASK_ORIGIN: WBEM_FLAVOR_TYPE = WBEM_FLAVOR_TYPE(96i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAVOR_NOT_AMENDED: WBEM_FLAVOR_TYPE = WBEM_FLAVOR_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAVOR_AMENDED: WBEM_FLAVOR_TYPE = WBEM_FLAVOR_TYPE(128i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAVOR_MASK_AMENDED: WBEM_FLAVOR_TYPE = WBEM_FLAVOR_TYPE(128i32);
impl ::core::marker::Copy for WBEM_FLAVOR_TYPE {}
impl ::core::clone::Clone for WBEM_FLAVOR_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WBEM_FLAVOR_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WBEM_FLAVOR_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WBEM_FLAVOR_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WBEM_FLAVOR_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WBEM_GENERIC_FLAG_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAG_RETURN_IMMEDIATELY: WBEM_GENERIC_FLAG_TYPE = WBEM_GENERIC_FLAG_TYPE(16i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAG_RETURN_WBEM_COMPLETE: WBEM_GENERIC_FLAG_TYPE = WBEM_GENERIC_FLAG_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAG_BIDIRECTIONAL: WBEM_GENERIC_FLAG_TYPE = WBEM_GENERIC_FLAG_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAG_FORWARD_ONLY: WBEM_GENERIC_FLAG_TYPE = WBEM_GENERIC_FLAG_TYPE(32i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAG_NO_ERROR_OBJECT: WBEM_GENERIC_FLAG_TYPE = WBEM_GENERIC_FLAG_TYPE(64i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAG_RETURN_ERROR_OBJECT: WBEM_GENERIC_FLAG_TYPE = WBEM_GENERIC_FLAG_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAG_SEND_STATUS: WBEM_GENERIC_FLAG_TYPE = WBEM_GENERIC_FLAG_TYPE(128i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAG_DONT_SEND_STATUS: WBEM_GENERIC_FLAG_TYPE = WBEM_GENERIC_FLAG_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAG_ENSURE_LOCATABLE: WBEM_GENERIC_FLAG_TYPE = WBEM_GENERIC_FLAG_TYPE(256i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAG_DIRECT_READ: WBEM_GENERIC_FLAG_TYPE = WBEM_GENERIC_FLAG_TYPE(512i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAG_SEND_ONLY_SELECTED: WBEM_GENERIC_FLAG_TYPE = WBEM_GENERIC_FLAG_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_RETURN_WHEN_COMPLETE: WBEM_GENERIC_FLAG_TYPE = WBEM_GENERIC_FLAG_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_RETURN_IMMEDIATELY: WBEM_GENERIC_FLAG_TYPE = WBEM_GENERIC_FLAG_TYPE(16i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_MASK_RESERVED_FLAGS: WBEM_GENERIC_FLAG_TYPE = WBEM_GENERIC_FLAG_TYPE(126976i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAG_USE_AMENDED_QUALIFIERS: WBEM_GENERIC_FLAG_TYPE = WBEM_GENERIC_FLAG_TYPE(131072i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAG_STRONG_VALIDATION: WBEM_GENERIC_FLAG_TYPE = WBEM_GENERIC_FLAG_TYPE(1048576i32);
impl ::core::marker::Copy for WBEM_GENERIC_FLAG_TYPE {}
impl ::core::clone::Clone for WBEM_GENERIC_FLAG_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WBEM_GENERIC_FLAG_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WBEM_GENERIC_FLAG_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WBEM_GENERIC_FLAG_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WBEM_GENERIC_FLAG_TYPE").field(&self.0).finish()
    }
}
impl WBEM_GENERIC_FLAG_TYPE {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for WBEM_GENERIC_FLAG_TYPE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for WBEM_GENERIC_FLAG_TYPE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for WBEM_GENERIC_FLAG_TYPE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for WBEM_GENERIC_FLAG_TYPE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for WBEM_GENERIC_FLAG_TYPE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WBEM_GENUS_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_GENUS_CLASS: WBEM_GENUS_TYPE = WBEM_GENUS_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_GENUS_INSTANCE: WBEM_GENUS_TYPE = WBEM_GENUS_TYPE(2i32);
impl ::core::marker::Copy for WBEM_GENUS_TYPE {}
impl ::core::clone::Clone for WBEM_GENUS_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WBEM_GENUS_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WBEM_GENUS_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WBEM_GENUS_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WBEM_GENUS_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WBEM_GET_KEY_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMPATH_TEXT: WBEM_GET_KEY_FLAGS = WBEM_GET_KEY_FLAGS(1i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMPATH_QUOTEDTEXT: WBEM_GET_KEY_FLAGS = WBEM_GET_KEY_FLAGS(2i32);
impl ::core::marker::Copy for WBEM_GET_KEY_FLAGS {}
impl ::core::clone::Clone for WBEM_GET_KEY_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WBEM_GET_KEY_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WBEM_GET_KEY_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WBEM_GET_KEY_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WBEM_GET_KEY_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WBEM_GET_TEXT_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMPATH_COMPRESSED: WBEM_GET_TEXT_FLAGS = WBEM_GET_TEXT_FLAGS(1i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMPATH_GET_RELATIVE_ONLY: WBEM_GET_TEXT_FLAGS = WBEM_GET_TEXT_FLAGS(2i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMPATH_GET_SERVER_TOO: WBEM_GET_TEXT_FLAGS = WBEM_GET_TEXT_FLAGS(4i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMPATH_GET_SERVER_AND_NAMESPACE_ONLY: WBEM_GET_TEXT_FLAGS = WBEM_GET_TEXT_FLAGS(8i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMPATH_GET_NAMESPACE_ONLY: WBEM_GET_TEXT_FLAGS = WBEM_GET_TEXT_FLAGS(16i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMPATH_GET_ORIGINAL: WBEM_GET_TEXT_FLAGS = WBEM_GET_TEXT_FLAGS(32i32);
impl ::core::marker::Copy for WBEM_GET_TEXT_FLAGS {}
impl ::core::clone::Clone for WBEM_GET_TEXT_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WBEM_GET_TEXT_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WBEM_GET_TEXT_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WBEM_GET_TEXT_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WBEM_GET_TEXT_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WBEM_INFORMATION_FLAG_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAG_SHORT_NAME: WBEM_INFORMATION_FLAG_TYPE = WBEM_INFORMATION_FLAG_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAG_LONG_NAME: WBEM_INFORMATION_FLAG_TYPE = WBEM_INFORMATION_FLAG_TYPE(2i32);
impl ::core::marker::Copy for WBEM_INFORMATION_FLAG_TYPE {}
impl ::core::clone::Clone for WBEM_INFORMATION_FLAG_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WBEM_INFORMATION_FLAG_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WBEM_INFORMATION_FLAG_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WBEM_INFORMATION_FLAG_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WBEM_INFORMATION_FLAG_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WBEM_LIMITATION_FLAG_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAG_EXCLUDE_OBJECT_QUALIFIERS: WBEM_LIMITATION_FLAG_TYPE = WBEM_LIMITATION_FLAG_TYPE(16i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAG_EXCLUDE_PROPERTY_QUALIFIERS: WBEM_LIMITATION_FLAG_TYPE = WBEM_LIMITATION_FLAG_TYPE(32i32);
impl ::core::marker::Copy for WBEM_LIMITATION_FLAG_TYPE {}
impl ::core::clone::Clone for WBEM_LIMITATION_FLAG_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WBEM_LIMITATION_FLAG_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WBEM_LIMITATION_FLAG_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WBEM_LIMITATION_FLAG_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WBEM_LIMITATION_FLAG_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WBEM_LIMITS(pub i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_MAX_IDENTIFIER: WBEM_LIMITS = WBEM_LIMITS(4096i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_MAX_QUERY: WBEM_LIMITS = WBEM_LIMITS(16384i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_MAX_PATH: WBEM_LIMITS = WBEM_LIMITS(8192i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_MAX_OBJECT_NESTING: WBEM_LIMITS = WBEM_LIMITS(64i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_MAX_USER_PROPERTIES: WBEM_LIMITS = WBEM_LIMITS(1024i32);
impl ::core::marker::Copy for WBEM_LIMITS {}
impl ::core::clone::Clone for WBEM_LIMITS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WBEM_LIMITS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WBEM_LIMITS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WBEM_LIMITS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WBEM_LIMITS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WBEM_LOCKING_FLAG_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAG_ALLOW_READ: WBEM_LOCKING_FLAG_TYPE = WBEM_LOCKING_FLAG_TYPE(1i32);
impl ::core::marker::Copy for WBEM_LOCKING_FLAG_TYPE {}
impl ::core::clone::Clone for WBEM_LOCKING_FLAG_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WBEM_LOCKING_FLAG_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WBEM_LOCKING_FLAG_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WBEM_LOCKING_FLAG_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WBEM_LOCKING_FLAG_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WBEM_LOGIN_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAG_INPROC_LOGIN: WBEM_LOGIN_TYPE = WBEM_LOGIN_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAG_LOCAL_LOGIN: WBEM_LOGIN_TYPE = WBEM_LOGIN_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAG_REMOTE_LOGIN: WBEM_LOGIN_TYPE = WBEM_LOGIN_TYPE(2i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_AUTHENTICATION_METHOD_MASK: WBEM_LOGIN_TYPE = WBEM_LOGIN_TYPE(15i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAG_USE_MULTIPLE_CHALLENGES: WBEM_LOGIN_TYPE = WBEM_LOGIN_TYPE(16i32);
impl ::core::marker::Copy for WBEM_LOGIN_TYPE {}
impl ::core::clone::Clone for WBEM_LOGIN_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WBEM_LOGIN_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WBEM_LOGIN_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WBEM_LOGIN_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WBEM_LOGIN_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WBEM_PATH_CREATE_FLAG(pub i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMPATH_CREATE_ACCEPT_RELATIVE: WBEM_PATH_CREATE_FLAG = WBEM_PATH_CREATE_FLAG(1i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMPATH_CREATE_ACCEPT_ABSOLUTE: WBEM_PATH_CREATE_FLAG = WBEM_PATH_CREATE_FLAG(2i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMPATH_CREATE_ACCEPT_ALL: WBEM_PATH_CREATE_FLAG = WBEM_PATH_CREATE_FLAG(4i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMPATH_TREAT_SINGLE_IDENT_AS_NS: WBEM_PATH_CREATE_FLAG = WBEM_PATH_CREATE_FLAG(8i32);
impl ::core::marker::Copy for WBEM_PATH_CREATE_FLAG {}
impl ::core::clone::Clone for WBEM_PATH_CREATE_FLAG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WBEM_PATH_CREATE_FLAG {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WBEM_PATH_CREATE_FLAG {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WBEM_PATH_CREATE_FLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WBEM_PATH_CREATE_FLAG").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WBEM_PATH_STATUS_FLAG(pub i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMPATH_INFO_ANON_LOCAL_MACHINE: WBEM_PATH_STATUS_FLAG = WBEM_PATH_STATUS_FLAG(1i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMPATH_INFO_HAS_MACHINE_NAME: WBEM_PATH_STATUS_FLAG = WBEM_PATH_STATUS_FLAG(2i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMPATH_INFO_IS_CLASS_REF: WBEM_PATH_STATUS_FLAG = WBEM_PATH_STATUS_FLAG(4i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMPATH_INFO_IS_INST_REF: WBEM_PATH_STATUS_FLAG = WBEM_PATH_STATUS_FLAG(8i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMPATH_INFO_HAS_SUBSCOPES: WBEM_PATH_STATUS_FLAG = WBEM_PATH_STATUS_FLAG(16i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMPATH_INFO_IS_COMPOUND: WBEM_PATH_STATUS_FLAG = WBEM_PATH_STATUS_FLAG(32i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMPATH_INFO_HAS_V2_REF_PATHS: WBEM_PATH_STATUS_FLAG = WBEM_PATH_STATUS_FLAG(64i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMPATH_INFO_HAS_IMPLIED_KEY: WBEM_PATH_STATUS_FLAG = WBEM_PATH_STATUS_FLAG(128i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMPATH_INFO_CONTAINS_SINGLETON: WBEM_PATH_STATUS_FLAG = WBEM_PATH_STATUS_FLAG(256i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMPATH_INFO_V1_COMPLIANT: WBEM_PATH_STATUS_FLAG = WBEM_PATH_STATUS_FLAG(512i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMPATH_INFO_V2_COMPLIANT: WBEM_PATH_STATUS_FLAG = WBEM_PATH_STATUS_FLAG(1024i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMPATH_INFO_CIM_COMPLIANT: WBEM_PATH_STATUS_FLAG = WBEM_PATH_STATUS_FLAG(2048i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMPATH_INFO_IS_SINGLETON: WBEM_PATH_STATUS_FLAG = WBEM_PATH_STATUS_FLAG(4096i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMPATH_INFO_IS_PARENT: WBEM_PATH_STATUS_FLAG = WBEM_PATH_STATUS_FLAG(8192i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMPATH_INFO_SERVER_NAMESPACE_ONLY: WBEM_PATH_STATUS_FLAG = WBEM_PATH_STATUS_FLAG(16384i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMPATH_INFO_NATIVE_PATH: WBEM_PATH_STATUS_FLAG = WBEM_PATH_STATUS_FLAG(32768i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMPATH_INFO_WMI_PATH: WBEM_PATH_STATUS_FLAG = WBEM_PATH_STATUS_FLAG(65536i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEMPATH_INFO_PATH_HAD_SERVER: WBEM_PATH_STATUS_FLAG = WBEM_PATH_STATUS_FLAG(131072i32);
impl ::core::marker::Copy for WBEM_PATH_STATUS_FLAG {}
impl ::core::clone::Clone for WBEM_PATH_STATUS_FLAG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WBEM_PATH_STATUS_FLAG {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WBEM_PATH_STATUS_FLAG {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WBEM_PATH_STATUS_FLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WBEM_PATH_STATUS_FLAG").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WBEM_PROVIDER_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAG_OWNER_UPDATE: WBEM_PROVIDER_FLAGS = WBEM_PROVIDER_FLAGS(65536i32);
impl ::core::marker::Copy for WBEM_PROVIDER_FLAGS {}
impl ::core::clone::Clone for WBEM_PROVIDER_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WBEM_PROVIDER_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WBEM_PROVIDER_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WBEM_PROVIDER_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WBEM_PROVIDER_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WBEM_PROVIDER_REQUIREMENTS_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_REQUIREMENTS_START_POSTFILTER: WBEM_PROVIDER_REQUIREMENTS_TYPE = WBEM_PROVIDER_REQUIREMENTS_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_REQUIREMENTS_STOP_POSTFILTER: WBEM_PROVIDER_REQUIREMENTS_TYPE = WBEM_PROVIDER_REQUIREMENTS_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_REQUIREMENTS_RECHECK_SUBSCRIPTIONS: WBEM_PROVIDER_REQUIREMENTS_TYPE = WBEM_PROVIDER_REQUIREMENTS_TYPE(2i32);
impl ::core::marker::Copy for WBEM_PROVIDER_REQUIREMENTS_TYPE {}
impl ::core::clone::Clone for WBEM_PROVIDER_REQUIREMENTS_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WBEM_PROVIDER_REQUIREMENTS_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WBEM_PROVIDER_REQUIREMENTS_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WBEM_PROVIDER_REQUIREMENTS_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WBEM_PROVIDER_REQUIREMENTS_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WBEM_QUERY_FLAG_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAG_DEEP: WBEM_QUERY_FLAG_TYPE = WBEM_QUERY_FLAG_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAG_SHALLOW: WBEM_QUERY_FLAG_TYPE = WBEM_QUERY_FLAG_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAG_PROTOTYPE: WBEM_QUERY_FLAG_TYPE = WBEM_QUERY_FLAG_TYPE(2i32);
impl ::core::marker::Copy for WBEM_QUERY_FLAG_TYPE {}
impl ::core::clone::Clone for WBEM_QUERY_FLAG_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WBEM_QUERY_FLAG_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WBEM_QUERY_FLAG_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WBEM_QUERY_FLAG_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WBEM_QUERY_FLAG_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WBEM_REFRESHER_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAG_REFRESH_AUTO_RECONNECT: WBEM_REFRESHER_FLAGS = WBEM_REFRESHER_FLAGS(0i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAG_REFRESH_NO_AUTO_RECONNECT: WBEM_REFRESHER_FLAGS = WBEM_REFRESHER_FLAGS(1i32);
impl ::core::marker::Copy for WBEM_REFRESHER_FLAGS {}
impl ::core::clone::Clone for WBEM_REFRESHER_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WBEM_REFRESHER_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WBEM_REFRESHER_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WBEM_REFRESHER_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WBEM_REFRESHER_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WBEM_SECURITY_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_ENABLE: WBEM_SECURITY_FLAGS = WBEM_SECURITY_FLAGS(1i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_METHOD_EXECUTE: WBEM_SECURITY_FLAGS = WBEM_SECURITY_FLAGS(2i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FULL_WRITE_REP: WBEM_SECURITY_FLAGS = WBEM_SECURITY_FLAGS(4i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_PARTIAL_WRITE_REP: WBEM_SECURITY_FLAGS = WBEM_SECURITY_FLAGS(8i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_WRITE_PROVIDER: WBEM_SECURITY_FLAGS = WBEM_SECURITY_FLAGS(16i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_REMOTE_ACCESS: WBEM_SECURITY_FLAGS = WBEM_SECURITY_FLAGS(32i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_RIGHT_SUBSCRIBE: WBEM_SECURITY_FLAGS = WBEM_SECURITY_FLAGS(64i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_RIGHT_PUBLISH: WBEM_SECURITY_FLAGS = WBEM_SECURITY_FLAGS(128i32);
impl ::core::marker::Copy for WBEM_SECURITY_FLAGS {}
impl ::core::clone::Clone for WBEM_SECURITY_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WBEM_SECURITY_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WBEM_SECURITY_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WBEM_SECURITY_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WBEM_SECURITY_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WBEM_SHUTDOWN_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_SHUTDOWN_UNLOAD_COMPONENT: WBEM_SHUTDOWN_FLAGS = WBEM_SHUTDOWN_FLAGS(1i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_SHUTDOWN_WMI: WBEM_SHUTDOWN_FLAGS = WBEM_SHUTDOWN_FLAGS(2i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_SHUTDOWN_OS: WBEM_SHUTDOWN_FLAGS = WBEM_SHUTDOWN_FLAGS(3i32);
impl ::core::marker::Copy for WBEM_SHUTDOWN_FLAGS {}
impl ::core::clone::Clone for WBEM_SHUTDOWN_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WBEM_SHUTDOWN_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WBEM_SHUTDOWN_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WBEM_SHUTDOWN_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WBEM_SHUTDOWN_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WBEM_STATUS_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_STATUS_COMPLETE: WBEM_STATUS_TYPE = WBEM_STATUS_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_STATUS_REQUIREMENTS: WBEM_STATUS_TYPE = WBEM_STATUS_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_STATUS_PROGRESS: WBEM_STATUS_TYPE = WBEM_STATUS_TYPE(2i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_STATUS_LOGGING_INFORMATION: WBEM_STATUS_TYPE = WBEM_STATUS_TYPE(256i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_STATUS_LOGGING_INFORMATION_PROVIDER: WBEM_STATUS_TYPE = WBEM_STATUS_TYPE(512i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_STATUS_LOGGING_INFORMATION_HOST: WBEM_STATUS_TYPE = WBEM_STATUS_TYPE(1024i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_STATUS_LOGGING_INFORMATION_REPOSITORY: WBEM_STATUS_TYPE = WBEM_STATUS_TYPE(2048i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_STATUS_LOGGING_INFORMATION_ESS: WBEM_STATUS_TYPE = WBEM_STATUS_TYPE(4096i32);
impl ::core::marker::Copy for WBEM_STATUS_TYPE {}
impl ::core::clone::Clone for WBEM_STATUS_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WBEM_STATUS_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WBEM_STATUS_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WBEM_STATUS_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WBEM_STATUS_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WBEM_TEXT_FLAG_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAG_NO_FLAVORS: WBEM_TEXT_FLAG_TYPE = WBEM_TEXT_FLAG_TYPE(1i32);
impl ::core::marker::Copy for WBEM_TEXT_FLAG_TYPE {}
impl ::core::clone::Clone for WBEM_TEXT_FLAG_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WBEM_TEXT_FLAG_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WBEM_TEXT_FLAG_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WBEM_TEXT_FLAG_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WBEM_TEXT_FLAG_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WBEM_UNSECAPP_FLAG_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAG_UNSECAPP_DEFAULT_CHECK_ACCESS: WBEM_UNSECAPP_FLAG_TYPE = WBEM_UNSECAPP_FLAG_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAG_UNSECAPP_CHECK_ACCESS: WBEM_UNSECAPP_FLAG_TYPE = WBEM_UNSECAPP_FLAG_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WBEM_FLAG_UNSECAPP_DONT_CHECK_ACCESS: WBEM_UNSECAPP_FLAG_TYPE = WBEM_UNSECAPP_FLAG_TYPE(2i32);
impl ::core::marker::Copy for WBEM_UNSECAPP_FLAG_TYPE {}
impl ::core::clone::Clone for WBEM_UNSECAPP_FLAG_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WBEM_UNSECAPP_FLAG_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WBEM_UNSECAPP_FLAG_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WBEM_UNSECAPP_FLAG_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WBEM_UNSECAPP_FLAG_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WMIQ_ANALYSIS_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_ANALYSIS_RPN_SEQUENCE: WMIQ_ANALYSIS_TYPE = WMIQ_ANALYSIS_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_ANALYSIS_ASSOC_QUERY: WMIQ_ANALYSIS_TYPE = WMIQ_ANALYSIS_TYPE(2i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_ANALYSIS_PROP_ANALYSIS_MATRIX: WMIQ_ANALYSIS_TYPE = WMIQ_ANALYSIS_TYPE(3i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_ANALYSIS_QUERY_TEXT: WMIQ_ANALYSIS_TYPE = WMIQ_ANALYSIS_TYPE(4i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_ANALYSIS_RESERVED: WMIQ_ANALYSIS_TYPE = WMIQ_ANALYSIS_TYPE(134217728i32);
impl ::core::marker::Copy for WMIQ_ANALYSIS_TYPE {}
impl ::core::clone::Clone for WMIQ_ANALYSIS_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WMIQ_ANALYSIS_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WMIQ_ANALYSIS_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WMIQ_ANALYSIS_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMIQ_ANALYSIS_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WMIQ_ASSOCQ_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_ASSOCQ_ASSOCIATORS: WMIQ_ASSOCQ_FLAGS = WMIQ_ASSOCQ_FLAGS(1i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_ASSOCQ_REFERENCES: WMIQ_ASSOCQ_FLAGS = WMIQ_ASSOCQ_FLAGS(2i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_ASSOCQ_RESULTCLASS: WMIQ_ASSOCQ_FLAGS = WMIQ_ASSOCQ_FLAGS(4i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_ASSOCQ_ASSOCCLASS: WMIQ_ASSOCQ_FLAGS = WMIQ_ASSOCQ_FLAGS(8i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_ASSOCQ_ROLE: WMIQ_ASSOCQ_FLAGS = WMIQ_ASSOCQ_FLAGS(16i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_ASSOCQ_RESULTROLE: WMIQ_ASSOCQ_FLAGS = WMIQ_ASSOCQ_FLAGS(32i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_ASSOCQ_REQUIREDQUALIFIER: WMIQ_ASSOCQ_FLAGS = WMIQ_ASSOCQ_FLAGS(64i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_ASSOCQ_REQUIREDASSOCQUALIFIER: WMIQ_ASSOCQ_FLAGS = WMIQ_ASSOCQ_FLAGS(128i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_ASSOCQ_CLASSDEFSONLY: WMIQ_ASSOCQ_FLAGS = WMIQ_ASSOCQ_FLAGS(256i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_ASSOCQ_KEYSONLY: WMIQ_ASSOCQ_FLAGS = WMIQ_ASSOCQ_FLAGS(512i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_ASSOCQ_SCHEMAONLY: WMIQ_ASSOCQ_FLAGS = WMIQ_ASSOCQ_FLAGS(1024i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_ASSOCQ_CLASSREFSONLY: WMIQ_ASSOCQ_FLAGS = WMIQ_ASSOCQ_FLAGS(2048i32);
impl ::core::marker::Copy for WMIQ_ASSOCQ_FLAGS {}
impl ::core::clone::Clone for WMIQ_ASSOCQ_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WMIQ_ASSOCQ_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WMIQ_ASSOCQ_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WMIQ_ASSOCQ_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMIQ_ASSOCQ_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WMIQ_LANGUAGE_FEATURES(pub i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_LF1_BASIC_SELECT: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(1i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_LF2_CLASS_NAME_IN_QUERY: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(2i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_LF3_STRING_CASE_FUNCTIONS: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(3i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_LF4_PROP_TO_PROP_TESTS: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(4i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_LF5_COUNT_STAR: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(5i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_LF6_ORDER_BY: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(6i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_LF7_DISTINCT: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(7i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_LF8_ISA: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(8i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_LF9_THIS: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(9i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_LF10_COMPEX_SUBEXPRESSIONS: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(10i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_LF11_ALIASING: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(11i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_LF12_GROUP_BY_HAVING: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(12i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_LF13_WMI_WITHIN: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(13i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_LF14_SQL_WRITE_OPERATIONS: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(14i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_LF15_GO: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(15i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_LF16_SINGLE_LEVEL_TRANSACTIONS: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(16i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_LF17_QUALIFIED_NAMES: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(17i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_LF18_ASSOCIATONS: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(18i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_LF19_SYSTEM_PROPERTIES: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(19i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_LF20_EXTENDED_SYSTEM_PROPERTIES: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(20i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_LF21_SQL89_JOINS: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(21i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_LF22_SQL92_JOINS: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(22i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_LF23_SUBSELECTS: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(23i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_LF24_UMI_EXTENSIONS: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(24i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_LF25_DATEPART: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(25i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_LF26_LIKE: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(26i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_LF27_CIM_TEMPORAL_CONSTRUCTS: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(27i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_LF28_STANDARD_AGGREGATES: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(28i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_LF29_MULTI_LEVEL_ORDER_BY: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(29i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_LF30_WMI_PRAGMAS: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(30i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_LF31_QUALIFIER_TESTS: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(31i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_LF32_SP_EXECUTE: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(32i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_LF33_ARRAY_ACCESS: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(33i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_LF34_UNION: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(34i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_LF35_COMPLEX_SELECT_TARGET: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(35i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_LF36_REFERENCE_TESTS: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(36i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_LF37_SELECT_INTO: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(37i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_LF38_BASIC_DATETIME_TESTS: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(38i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_LF39_COUNT_COLUMN: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(39i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_LF40_BETWEEN: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(40i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_LF_LAST: WMIQ_LANGUAGE_FEATURES = WMIQ_LANGUAGE_FEATURES(40i32);
impl ::core::marker::Copy for WMIQ_LANGUAGE_FEATURES {}
impl ::core::clone::Clone for WMIQ_LANGUAGE_FEATURES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WMIQ_LANGUAGE_FEATURES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WMIQ_LANGUAGE_FEATURES {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WMIQ_LANGUAGE_FEATURES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMIQ_LANGUAGE_FEATURES").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WMIQ_RPNF_FEATURE(pub i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_RPNF_WHERE_CLAUSE_PRESENT: WMIQ_RPNF_FEATURE = WMIQ_RPNF_FEATURE(1i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_RPNF_QUERY_IS_CONJUNCTIVE: WMIQ_RPNF_FEATURE = WMIQ_RPNF_FEATURE(2i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_RPNF_QUERY_IS_DISJUNCTIVE: WMIQ_RPNF_FEATURE = WMIQ_RPNF_FEATURE(4i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_RPNF_PROJECTION: WMIQ_RPNF_FEATURE = WMIQ_RPNF_FEATURE(8i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_RPNF_FEATURE_SELECT_STAR: WMIQ_RPNF_FEATURE = WMIQ_RPNF_FEATURE(16i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_RPNF_EQUALITY_TESTS_ONLY: WMIQ_RPNF_FEATURE = WMIQ_RPNF_FEATURE(32i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_RPNF_COUNT_STAR: WMIQ_RPNF_FEATURE = WMIQ_RPNF_FEATURE(64i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_RPNF_QUALIFIED_NAMES_USED: WMIQ_RPNF_FEATURE = WMIQ_RPNF_FEATURE(128i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_RPNF_SYSPROP_CLASS_USED: WMIQ_RPNF_FEATURE = WMIQ_RPNF_FEATURE(256i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_RPNF_PROP_TO_PROP_TESTS: WMIQ_RPNF_FEATURE = WMIQ_RPNF_FEATURE(512i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_RPNF_ORDER_BY: WMIQ_RPNF_FEATURE = WMIQ_RPNF_FEATURE(1024i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_RPNF_ISA_USED: WMIQ_RPNF_FEATURE = WMIQ_RPNF_FEATURE(2048i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_RPNF_GROUP_BY_HAVING: WMIQ_RPNF_FEATURE = WMIQ_RPNF_FEATURE(4096i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_RPNF_ARRAY_ACCESS_USED: WMIQ_RPNF_FEATURE = WMIQ_RPNF_FEATURE(8192i32);
impl ::core::marker::Copy for WMIQ_RPNF_FEATURE {}
impl ::core::clone::Clone for WMIQ_RPNF_FEATURE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WMIQ_RPNF_FEATURE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WMIQ_RPNF_FEATURE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WMIQ_RPNF_FEATURE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMIQ_RPNF_FEATURE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WMIQ_RPN_TOKEN_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_RPN_TOKEN_EXPRESSION: WMIQ_RPN_TOKEN_FLAGS = WMIQ_RPN_TOKEN_FLAGS(1i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_RPN_TOKEN_AND: WMIQ_RPN_TOKEN_FLAGS = WMIQ_RPN_TOKEN_FLAGS(2i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_RPN_TOKEN_OR: WMIQ_RPN_TOKEN_FLAGS = WMIQ_RPN_TOKEN_FLAGS(3i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_RPN_TOKEN_NOT: WMIQ_RPN_TOKEN_FLAGS = WMIQ_RPN_TOKEN_FLAGS(4i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_RPN_OP_UNDEFINED: WMIQ_RPN_TOKEN_FLAGS = WMIQ_RPN_TOKEN_FLAGS(0i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_RPN_OP_EQ: WMIQ_RPN_TOKEN_FLAGS = WMIQ_RPN_TOKEN_FLAGS(1i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_RPN_OP_NE: WMIQ_RPN_TOKEN_FLAGS = WMIQ_RPN_TOKEN_FLAGS(2i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_RPN_OP_GE: WMIQ_RPN_TOKEN_FLAGS = WMIQ_RPN_TOKEN_FLAGS(3i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_RPN_OP_LE: WMIQ_RPN_TOKEN_FLAGS = WMIQ_RPN_TOKEN_FLAGS(4i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_RPN_OP_LT: WMIQ_RPN_TOKEN_FLAGS = WMIQ_RPN_TOKEN_FLAGS(5i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_RPN_OP_GT: WMIQ_RPN_TOKEN_FLAGS = WMIQ_RPN_TOKEN_FLAGS(6i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_RPN_OP_LIKE: WMIQ_RPN_TOKEN_FLAGS = WMIQ_RPN_TOKEN_FLAGS(7i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_RPN_OP_ISA: WMIQ_RPN_TOKEN_FLAGS = WMIQ_RPN_TOKEN_FLAGS(8i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_RPN_OP_ISNOTA: WMIQ_RPN_TOKEN_FLAGS = WMIQ_RPN_TOKEN_FLAGS(9i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_RPN_OP_ISNULL: WMIQ_RPN_TOKEN_FLAGS = WMIQ_RPN_TOKEN_FLAGS(10i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_RPN_OP_ISNOTNULL: WMIQ_RPN_TOKEN_FLAGS = WMIQ_RPN_TOKEN_FLAGS(11i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_RPN_LEFT_PROPERTY_NAME: WMIQ_RPN_TOKEN_FLAGS = WMIQ_RPN_TOKEN_FLAGS(1i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_RPN_RIGHT_PROPERTY_NAME: WMIQ_RPN_TOKEN_FLAGS = WMIQ_RPN_TOKEN_FLAGS(2i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_RPN_CONST2: WMIQ_RPN_TOKEN_FLAGS = WMIQ_RPN_TOKEN_FLAGS(4i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_RPN_CONST: WMIQ_RPN_TOKEN_FLAGS = WMIQ_RPN_TOKEN_FLAGS(8i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_RPN_RELOP: WMIQ_RPN_TOKEN_FLAGS = WMIQ_RPN_TOKEN_FLAGS(16i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_RPN_LEFT_FUNCTION: WMIQ_RPN_TOKEN_FLAGS = WMIQ_RPN_TOKEN_FLAGS(32i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_RPN_RIGHT_FUNCTION: WMIQ_RPN_TOKEN_FLAGS = WMIQ_RPN_TOKEN_FLAGS(64i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_RPN_GET_TOKEN_TYPE: WMIQ_RPN_TOKEN_FLAGS = WMIQ_RPN_TOKEN_FLAGS(1i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_RPN_GET_EXPR_SHAPE: WMIQ_RPN_TOKEN_FLAGS = WMIQ_RPN_TOKEN_FLAGS(2i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_RPN_GET_LEFT_FUNCTION: WMIQ_RPN_TOKEN_FLAGS = WMIQ_RPN_TOKEN_FLAGS(3i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_RPN_GET_RIGHT_FUNCTION: WMIQ_RPN_TOKEN_FLAGS = WMIQ_RPN_TOKEN_FLAGS(4i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_RPN_GET_RELOP: WMIQ_RPN_TOKEN_FLAGS = WMIQ_RPN_TOKEN_FLAGS(5i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_RPN_NEXT_TOKEN: WMIQ_RPN_TOKEN_FLAGS = WMIQ_RPN_TOKEN_FLAGS(1i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_RPN_FROM_UNARY: WMIQ_RPN_TOKEN_FLAGS = WMIQ_RPN_TOKEN_FLAGS(1i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_RPN_FROM_PATH: WMIQ_RPN_TOKEN_FLAGS = WMIQ_RPN_TOKEN_FLAGS(2i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_RPN_FROM_CLASS_LIST: WMIQ_RPN_TOKEN_FLAGS = WMIQ_RPN_TOKEN_FLAGS(4i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMIQ_RPN_FROM_MULTIPLE: WMIQ_RPN_TOKEN_FLAGS = WMIQ_RPN_TOKEN_FLAGS(8i32);
impl ::core::marker::Copy for WMIQ_RPN_TOKEN_FLAGS {}
impl ::core::clone::Clone for WMIQ_RPN_TOKEN_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WMIQ_RPN_TOKEN_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WMIQ_RPN_TOKEN_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WMIQ_RPN_TOKEN_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMIQ_RPN_TOKEN_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WMI_OBJ_TEXT(pub i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMI_OBJ_TEXT_CIM_DTD_2_0: WMI_OBJ_TEXT = WMI_OBJ_TEXT(1i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMI_OBJ_TEXT_WMI_DTD_2_0: WMI_OBJ_TEXT = WMI_OBJ_TEXT(2i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMI_OBJ_TEXT_WMI_EXT1: WMI_OBJ_TEXT = WMI_OBJ_TEXT(3i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMI_OBJ_TEXT_WMI_EXT2: WMI_OBJ_TEXT = WMI_OBJ_TEXT(4i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMI_OBJ_TEXT_WMI_EXT3: WMI_OBJ_TEXT = WMI_OBJ_TEXT(5i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMI_OBJ_TEXT_WMI_EXT4: WMI_OBJ_TEXT = WMI_OBJ_TEXT(6i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMI_OBJ_TEXT_WMI_EXT5: WMI_OBJ_TEXT = WMI_OBJ_TEXT(7i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMI_OBJ_TEXT_WMI_EXT6: WMI_OBJ_TEXT = WMI_OBJ_TEXT(8i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMI_OBJ_TEXT_WMI_EXT7: WMI_OBJ_TEXT = WMI_OBJ_TEXT(9i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMI_OBJ_TEXT_WMI_EXT8: WMI_OBJ_TEXT = WMI_OBJ_TEXT(10i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMI_OBJ_TEXT_WMI_EXT9: WMI_OBJ_TEXT = WMI_OBJ_TEXT(11i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMI_OBJ_TEXT_WMI_EXT10: WMI_OBJ_TEXT = WMI_OBJ_TEXT(12i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const WMI_OBJ_TEXT_LAST: WMI_OBJ_TEXT = WMI_OBJ_TEXT(13i32);
impl ::core::marker::Copy for WMI_OBJ_TEXT {}
impl ::core::clone::Clone for WMI_OBJ_TEXT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WMI_OBJ_TEXT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WMI_OBJ_TEXT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WMI_OBJ_TEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMI_OBJ_TEXT").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WbemAuthenticationLevelEnum(pub i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemAuthenticationLevelDefault: WbemAuthenticationLevelEnum = WbemAuthenticationLevelEnum(0i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemAuthenticationLevelNone: WbemAuthenticationLevelEnum = WbemAuthenticationLevelEnum(1i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemAuthenticationLevelConnect: WbemAuthenticationLevelEnum = WbemAuthenticationLevelEnum(2i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemAuthenticationLevelCall: WbemAuthenticationLevelEnum = WbemAuthenticationLevelEnum(3i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemAuthenticationLevelPkt: WbemAuthenticationLevelEnum = WbemAuthenticationLevelEnum(4i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemAuthenticationLevelPktIntegrity: WbemAuthenticationLevelEnum = WbemAuthenticationLevelEnum(5i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemAuthenticationLevelPktPrivacy: WbemAuthenticationLevelEnum = WbemAuthenticationLevelEnum(6i32);
impl ::core::marker::Copy for WbemAuthenticationLevelEnum {}
impl ::core::clone::Clone for WbemAuthenticationLevelEnum {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WbemAuthenticationLevelEnum {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WbemAuthenticationLevelEnum {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WbemAuthenticationLevelEnum {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WbemAuthenticationLevelEnum").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WbemChangeFlagEnum(pub i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemChangeFlagCreateOrUpdate: WbemChangeFlagEnum = WbemChangeFlagEnum(0i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemChangeFlagUpdateOnly: WbemChangeFlagEnum = WbemChangeFlagEnum(1i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemChangeFlagCreateOnly: WbemChangeFlagEnum = WbemChangeFlagEnum(2i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemChangeFlagUpdateCompatible: WbemChangeFlagEnum = WbemChangeFlagEnum(0i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemChangeFlagUpdateSafeMode: WbemChangeFlagEnum = WbemChangeFlagEnum(32i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemChangeFlagUpdateForceMode: WbemChangeFlagEnum = WbemChangeFlagEnum(64i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemChangeFlagStrongValidation: WbemChangeFlagEnum = WbemChangeFlagEnum(128i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemChangeFlagAdvisory: WbemChangeFlagEnum = WbemChangeFlagEnum(65536i32);
impl ::core::marker::Copy for WbemChangeFlagEnum {}
impl ::core::clone::Clone for WbemChangeFlagEnum {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WbemChangeFlagEnum {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WbemChangeFlagEnum {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WbemChangeFlagEnum {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WbemChangeFlagEnum").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WbemCimtypeEnum(pub i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemCimtypeSint8: WbemCimtypeEnum = WbemCimtypeEnum(16i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemCimtypeUint8: WbemCimtypeEnum = WbemCimtypeEnum(17i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemCimtypeSint16: WbemCimtypeEnum = WbemCimtypeEnum(2i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemCimtypeUint16: WbemCimtypeEnum = WbemCimtypeEnum(18i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemCimtypeSint32: WbemCimtypeEnum = WbemCimtypeEnum(3i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemCimtypeUint32: WbemCimtypeEnum = WbemCimtypeEnum(19i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemCimtypeSint64: WbemCimtypeEnum = WbemCimtypeEnum(20i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemCimtypeUint64: WbemCimtypeEnum = WbemCimtypeEnum(21i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemCimtypeReal32: WbemCimtypeEnum = WbemCimtypeEnum(4i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemCimtypeReal64: WbemCimtypeEnum = WbemCimtypeEnum(5i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemCimtypeBoolean: WbemCimtypeEnum = WbemCimtypeEnum(11i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemCimtypeString: WbemCimtypeEnum = WbemCimtypeEnum(8i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemCimtypeDatetime: WbemCimtypeEnum = WbemCimtypeEnum(101i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemCimtypeReference: WbemCimtypeEnum = WbemCimtypeEnum(102i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemCimtypeChar16: WbemCimtypeEnum = WbemCimtypeEnum(103i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemCimtypeObject: WbemCimtypeEnum = WbemCimtypeEnum(13i32);
impl ::core::marker::Copy for WbemCimtypeEnum {}
impl ::core::clone::Clone for WbemCimtypeEnum {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WbemCimtypeEnum {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WbemCimtypeEnum {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WbemCimtypeEnum {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WbemCimtypeEnum").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WbemComparisonFlagEnum(pub i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemComparisonFlagIncludeAll: WbemComparisonFlagEnum = WbemComparisonFlagEnum(0i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemComparisonFlagIgnoreQualifiers: WbemComparisonFlagEnum = WbemComparisonFlagEnum(1i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemComparisonFlagIgnoreObjectSource: WbemComparisonFlagEnum = WbemComparisonFlagEnum(2i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemComparisonFlagIgnoreDefaultValues: WbemComparisonFlagEnum = WbemComparisonFlagEnum(4i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemComparisonFlagIgnoreClass: WbemComparisonFlagEnum = WbemComparisonFlagEnum(8i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemComparisonFlagIgnoreCase: WbemComparisonFlagEnum = WbemComparisonFlagEnum(16i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemComparisonFlagIgnoreFlavor: WbemComparisonFlagEnum = WbemComparisonFlagEnum(32i32);
impl ::core::marker::Copy for WbemComparisonFlagEnum {}
impl ::core::clone::Clone for WbemComparisonFlagEnum {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WbemComparisonFlagEnum {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WbemComparisonFlagEnum {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WbemComparisonFlagEnum {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WbemComparisonFlagEnum").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WbemConnectOptionsEnum(pub i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemConnectFlagUseMaxWait: WbemConnectOptionsEnum = WbemConnectOptionsEnum(128i32);
impl ::core::marker::Copy for WbemConnectOptionsEnum {}
impl ::core::clone::Clone for WbemConnectOptionsEnum {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WbemConnectOptionsEnum {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WbemConnectOptionsEnum {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WbemConnectOptionsEnum {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WbemConnectOptionsEnum").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WbemErrorEnum(pub i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemNoErr: WbemErrorEnum = WbemErrorEnum(0i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrFailed: WbemErrorEnum = WbemErrorEnum(-2147217407i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrNotFound: WbemErrorEnum = WbemErrorEnum(-2147217406i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrAccessDenied: WbemErrorEnum = WbemErrorEnum(-2147217405i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrProviderFailure: WbemErrorEnum = WbemErrorEnum(-2147217404i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrTypeMismatch: WbemErrorEnum = WbemErrorEnum(-2147217403i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrOutOfMemory: WbemErrorEnum = WbemErrorEnum(-2147217402i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrInvalidContext: WbemErrorEnum = WbemErrorEnum(-2147217401i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrInvalidParameter: WbemErrorEnum = WbemErrorEnum(-2147217400i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrNotAvailable: WbemErrorEnum = WbemErrorEnum(-2147217399i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrCriticalError: WbemErrorEnum = WbemErrorEnum(-2147217398i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrInvalidStream: WbemErrorEnum = WbemErrorEnum(-2147217397i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrNotSupported: WbemErrorEnum = WbemErrorEnum(-2147217396i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrInvalidSuperclass: WbemErrorEnum = WbemErrorEnum(-2147217395i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrInvalidNamespace: WbemErrorEnum = WbemErrorEnum(-2147217394i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrInvalidObject: WbemErrorEnum = WbemErrorEnum(-2147217393i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrInvalidClass: WbemErrorEnum = WbemErrorEnum(-2147217392i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrProviderNotFound: WbemErrorEnum = WbemErrorEnum(-2147217391i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrInvalidProviderRegistration: WbemErrorEnum = WbemErrorEnum(-2147217390i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrProviderLoadFailure: WbemErrorEnum = WbemErrorEnum(-2147217389i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrInitializationFailure: WbemErrorEnum = WbemErrorEnum(-2147217388i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrTransportFailure: WbemErrorEnum = WbemErrorEnum(-2147217387i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrInvalidOperation: WbemErrorEnum = WbemErrorEnum(-2147217386i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrInvalidQuery: WbemErrorEnum = WbemErrorEnum(-2147217385i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrInvalidQueryType: WbemErrorEnum = WbemErrorEnum(-2147217384i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrAlreadyExists: WbemErrorEnum = WbemErrorEnum(-2147217383i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrOverrideNotAllowed: WbemErrorEnum = WbemErrorEnum(-2147217382i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrPropagatedQualifier: WbemErrorEnum = WbemErrorEnum(-2147217381i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrPropagatedProperty: WbemErrorEnum = WbemErrorEnum(-2147217380i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrUnexpected: WbemErrorEnum = WbemErrorEnum(-2147217379i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrIllegalOperation: WbemErrorEnum = WbemErrorEnum(-2147217378i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrCannotBeKey: WbemErrorEnum = WbemErrorEnum(-2147217377i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrIncompleteClass: WbemErrorEnum = WbemErrorEnum(-2147217376i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrInvalidSyntax: WbemErrorEnum = WbemErrorEnum(-2147217375i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrNondecoratedObject: WbemErrorEnum = WbemErrorEnum(-2147217374i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrReadOnly: WbemErrorEnum = WbemErrorEnum(-2147217373i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrProviderNotCapable: WbemErrorEnum = WbemErrorEnum(-2147217372i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrClassHasChildren: WbemErrorEnum = WbemErrorEnum(-2147217371i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrClassHasInstances: WbemErrorEnum = WbemErrorEnum(-2147217370i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrQueryNotImplemented: WbemErrorEnum = WbemErrorEnum(-2147217369i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrIllegalNull: WbemErrorEnum = WbemErrorEnum(-2147217368i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrInvalidQualifierType: WbemErrorEnum = WbemErrorEnum(-2147217367i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrInvalidPropertyType: WbemErrorEnum = WbemErrorEnum(-2147217366i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrValueOutOfRange: WbemErrorEnum = WbemErrorEnum(-2147217365i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrCannotBeSingleton: WbemErrorEnum = WbemErrorEnum(-2147217364i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrInvalidCimType: WbemErrorEnum = WbemErrorEnum(-2147217363i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrInvalidMethod: WbemErrorEnum = WbemErrorEnum(-2147217362i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrInvalidMethodParameters: WbemErrorEnum = WbemErrorEnum(-2147217361i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrSystemProperty: WbemErrorEnum = WbemErrorEnum(-2147217360i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrInvalidProperty: WbemErrorEnum = WbemErrorEnum(-2147217359i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrCallCancelled: WbemErrorEnum = WbemErrorEnum(-2147217358i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrShuttingDown: WbemErrorEnum = WbemErrorEnum(-2147217357i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrPropagatedMethod: WbemErrorEnum = WbemErrorEnum(-2147217356i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrUnsupportedParameter: WbemErrorEnum = WbemErrorEnum(-2147217355i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrMissingParameter: WbemErrorEnum = WbemErrorEnum(-2147217354i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrInvalidParameterId: WbemErrorEnum = WbemErrorEnum(-2147217353i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrNonConsecutiveParameterIds: WbemErrorEnum = WbemErrorEnum(-2147217352i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrParameterIdOnRetval: WbemErrorEnum = WbemErrorEnum(-2147217351i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrInvalidObjectPath: WbemErrorEnum = WbemErrorEnum(-2147217350i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrOutOfDiskSpace: WbemErrorEnum = WbemErrorEnum(-2147217349i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrBufferTooSmall: WbemErrorEnum = WbemErrorEnum(-2147217348i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrUnsupportedPutExtension: WbemErrorEnum = WbemErrorEnum(-2147217347i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrUnknownObjectType: WbemErrorEnum = WbemErrorEnum(-2147217346i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrUnknownPacketType: WbemErrorEnum = WbemErrorEnum(-2147217345i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrMarshalVersionMismatch: WbemErrorEnum = WbemErrorEnum(-2147217344i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrMarshalInvalidSignature: WbemErrorEnum = WbemErrorEnum(-2147217343i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrInvalidQualifier: WbemErrorEnum = WbemErrorEnum(-2147217342i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrInvalidDuplicateParameter: WbemErrorEnum = WbemErrorEnum(-2147217341i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrTooMuchData: WbemErrorEnum = WbemErrorEnum(-2147217340i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrServerTooBusy: WbemErrorEnum = WbemErrorEnum(-2147217339i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrInvalidFlavor: WbemErrorEnum = WbemErrorEnum(-2147217338i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrCircularReference: WbemErrorEnum = WbemErrorEnum(-2147217337i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrUnsupportedClassUpdate: WbemErrorEnum = WbemErrorEnum(-2147217336i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrCannotChangeKeyInheritance: WbemErrorEnum = WbemErrorEnum(-2147217335i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrCannotChangeIndexInheritance: WbemErrorEnum = WbemErrorEnum(-2147217328i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrTooManyProperties: WbemErrorEnum = WbemErrorEnum(-2147217327i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrUpdateTypeMismatch: WbemErrorEnum = WbemErrorEnum(-2147217326i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrUpdateOverrideNotAllowed: WbemErrorEnum = WbemErrorEnum(-2147217325i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrUpdatePropagatedMethod: WbemErrorEnum = WbemErrorEnum(-2147217324i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrMethodNotImplemented: WbemErrorEnum = WbemErrorEnum(-2147217323i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrMethodDisabled: WbemErrorEnum = WbemErrorEnum(-2147217322i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrRefresherBusy: WbemErrorEnum = WbemErrorEnum(-2147217321i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrUnparsableQuery: WbemErrorEnum = WbemErrorEnum(-2147217320i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrNotEventClass: WbemErrorEnum = WbemErrorEnum(-2147217319i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrMissingGroupWithin: WbemErrorEnum = WbemErrorEnum(-2147217318i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrMissingAggregationList: WbemErrorEnum = WbemErrorEnum(-2147217317i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrPropertyNotAnObject: WbemErrorEnum = WbemErrorEnum(-2147217316i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrAggregatingByObject: WbemErrorEnum = WbemErrorEnum(-2147217315i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrUninterpretableProviderQuery: WbemErrorEnum = WbemErrorEnum(-2147217313i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrBackupRestoreWinmgmtRunning: WbemErrorEnum = WbemErrorEnum(-2147217312i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrQueueOverflow: WbemErrorEnum = WbemErrorEnum(-2147217311i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrPrivilegeNotHeld: WbemErrorEnum = WbemErrorEnum(-2147217310i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrInvalidOperator: WbemErrorEnum = WbemErrorEnum(-2147217309i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrLocalCredentials: WbemErrorEnum = WbemErrorEnum(-2147217308i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrCannotBeAbstract: WbemErrorEnum = WbemErrorEnum(-2147217307i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrAmendedObject: WbemErrorEnum = WbemErrorEnum(-2147217306i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrClientTooSlow: WbemErrorEnum = WbemErrorEnum(-2147217305i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrNullSecurityDescriptor: WbemErrorEnum = WbemErrorEnum(-2147217304i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrTimeout: WbemErrorEnum = WbemErrorEnum(-2147217303i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrInvalidAssociation: WbemErrorEnum = WbemErrorEnum(-2147217302i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrAmbiguousOperation: WbemErrorEnum = WbemErrorEnum(-2147217301i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrQuotaViolation: WbemErrorEnum = WbemErrorEnum(-2147217300i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrTransactionConflict: WbemErrorEnum = WbemErrorEnum(-2147217299i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrForcedRollback: WbemErrorEnum = WbemErrorEnum(-2147217298i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrUnsupportedLocale: WbemErrorEnum = WbemErrorEnum(-2147217297i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrHandleOutOfDate: WbemErrorEnum = WbemErrorEnum(-2147217296i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrConnectionFailed: WbemErrorEnum = WbemErrorEnum(-2147217295i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrInvalidHandleRequest: WbemErrorEnum = WbemErrorEnum(-2147217294i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrPropertyNameTooWide: WbemErrorEnum = WbemErrorEnum(-2147217293i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrClassNameTooWide: WbemErrorEnum = WbemErrorEnum(-2147217292i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrMethodNameTooWide: WbemErrorEnum = WbemErrorEnum(-2147217291i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrQualifierNameTooWide: WbemErrorEnum = WbemErrorEnum(-2147217290i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrRerunCommand: WbemErrorEnum = WbemErrorEnum(-2147217289i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrDatabaseVerMismatch: WbemErrorEnum = WbemErrorEnum(-2147217288i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrVetoPut: WbemErrorEnum = WbemErrorEnum(-2147217287i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrVetoDelete: WbemErrorEnum = WbemErrorEnum(-2147217286i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrInvalidLocale: WbemErrorEnum = WbemErrorEnum(-2147217280i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrProviderSuspended: WbemErrorEnum = WbemErrorEnum(-2147217279i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrSynchronizationRequired: WbemErrorEnum = WbemErrorEnum(-2147217278i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrNoSchema: WbemErrorEnum = WbemErrorEnum(-2147217277i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrProviderAlreadyRegistered: WbemErrorEnum = WbemErrorEnum(-2147217276i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrProviderNotRegistered: WbemErrorEnum = WbemErrorEnum(-2147217275i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrFatalTransportError: WbemErrorEnum = WbemErrorEnum(-2147217274i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrEncryptedConnectionRequired: WbemErrorEnum = WbemErrorEnum(-2147217273i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrRegistrationTooBroad: WbemErrorEnum = WbemErrorEnum(-2147213311i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrRegistrationTooPrecise: WbemErrorEnum = WbemErrorEnum(-2147213310i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrTimedout: WbemErrorEnum = WbemErrorEnum(-2147209215i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemErrResetToDefault: WbemErrorEnum = WbemErrorEnum(-2147209214i32);
impl ::core::marker::Copy for WbemErrorEnum {}
impl ::core::clone::Clone for WbemErrorEnum {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WbemErrorEnum {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WbemErrorEnum {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WbemErrorEnum {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WbemErrorEnum").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WbemFlagEnum(pub i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemFlagReturnImmediately: WbemFlagEnum = WbemFlagEnum(16i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemFlagReturnWhenComplete: WbemFlagEnum = WbemFlagEnum(0i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemFlagBidirectional: WbemFlagEnum = WbemFlagEnum(0i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemFlagForwardOnly: WbemFlagEnum = WbemFlagEnum(32i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemFlagNoErrorObject: WbemFlagEnum = WbemFlagEnum(64i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemFlagReturnErrorObject: WbemFlagEnum = WbemFlagEnum(0i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemFlagSendStatus: WbemFlagEnum = WbemFlagEnum(128i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemFlagDontSendStatus: WbemFlagEnum = WbemFlagEnum(0i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemFlagEnsureLocatable: WbemFlagEnum = WbemFlagEnum(256i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemFlagDirectRead: WbemFlagEnum = WbemFlagEnum(512i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemFlagSendOnlySelected: WbemFlagEnum = WbemFlagEnum(0i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemFlagUseAmendedQualifiers: WbemFlagEnum = WbemFlagEnum(131072i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemFlagGetDefault: WbemFlagEnum = WbemFlagEnum(0i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemFlagSpawnInstance: WbemFlagEnum = WbemFlagEnum(1i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemFlagUseCurrentTime: WbemFlagEnum = WbemFlagEnum(1i32);
impl ::core::marker::Copy for WbemFlagEnum {}
impl ::core::clone::Clone for WbemFlagEnum {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WbemFlagEnum {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WbemFlagEnum {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WbemFlagEnum {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WbemFlagEnum").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WbemImpersonationLevelEnum(pub i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemImpersonationLevelAnonymous: WbemImpersonationLevelEnum = WbemImpersonationLevelEnum(1i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemImpersonationLevelIdentify: WbemImpersonationLevelEnum = WbemImpersonationLevelEnum(2i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemImpersonationLevelImpersonate: WbemImpersonationLevelEnum = WbemImpersonationLevelEnum(3i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemImpersonationLevelDelegate: WbemImpersonationLevelEnum = WbemImpersonationLevelEnum(4i32);
impl ::core::marker::Copy for WbemImpersonationLevelEnum {}
impl ::core::clone::Clone for WbemImpersonationLevelEnum {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WbemImpersonationLevelEnum {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WbemImpersonationLevelEnum {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WbemImpersonationLevelEnum {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WbemImpersonationLevelEnum").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WbemObjectTextFormatEnum(pub i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemObjectTextFormatCIMDTD20: WbemObjectTextFormatEnum = WbemObjectTextFormatEnum(1i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemObjectTextFormatWMIDTD20: WbemObjectTextFormatEnum = WbemObjectTextFormatEnum(2i32);
impl ::core::marker::Copy for WbemObjectTextFormatEnum {}
impl ::core::clone::Clone for WbemObjectTextFormatEnum {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WbemObjectTextFormatEnum {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WbemObjectTextFormatEnum {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WbemObjectTextFormatEnum {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WbemObjectTextFormatEnum").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WbemPrivilegeEnum(pub i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemPrivilegeCreateToken: WbemPrivilegeEnum = WbemPrivilegeEnum(1i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemPrivilegePrimaryToken: WbemPrivilegeEnum = WbemPrivilegeEnum(2i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemPrivilegeLockMemory: WbemPrivilegeEnum = WbemPrivilegeEnum(3i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemPrivilegeIncreaseQuota: WbemPrivilegeEnum = WbemPrivilegeEnum(4i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemPrivilegeMachineAccount: WbemPrivilegeEnum = WbemPrivilegeEnum(5i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemPrivilegeTcb: WbemPrivilegeEnum = WbemPrivilegeEnum(6i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemPrivilegeSecurity: WbemPrivilegeEnum = WbemPrivilegeEnum(7i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemPrivilegeTakeOwnership: WbemPrivilegeEnum = WbemPrivilegeEnum(8i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemPrivilegeLoadDriver: WbemPrivilegeEnum = WbemPrivilegeEnum(9i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemPrivilegeSystemProfile: WbemPrivilegeEnum = WbemPrivilegeEnum(10i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemPrivilegeSystemtime: WbemPrivilegeEnum = WbemPrivilegeEnum(11i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemPrivilegeProfileSingleProcess: WbemPrivilegeEnum = WbemPrivilegeEnum(12i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemPrivilegeIncreaseBasePriority: WbemPrivilegeEnum = WbemPrivilegeEnum(13i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemPrivilegeCreatePagefile: WbemPrivilegeEnum = WbemPrivilegeEnum(14i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemPrivilegeCreatePermanent: WbemPrivilegeEnum = WbemPrivilegeEnum(15i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemPrivilegeBackup: WbemPrivilegeEnum = WbemPrivilegeEnum(16i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemPrivilegeRestore: WbemPrivilegeEnum = WbemPrivilegeEnum(17i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemPrivilegeShutdown: WbemPrivilegeEnum = WbemPrivilegeEnum(18i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemPrivilegeDebug: WbemPrivilegeEnum = WbemPrivilegeEnum(19i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemPrivilegeAudit: WbemPrivilegeEnum = WbemPrivilegeEnum(20i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemPrivilegeSystemEnvironment: WbemPrivilegeEnum = WbemPrivilegeEnum(21i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemPrivilegeChangeNotify: WbemPrivilegeEnum = WbemPrivilegeEnum(22i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemPrivilegeRemoteShutdown: WbemPrivilegeEnum = WbemPrivilegeEnum(23i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemPrivilegeUndock: WbemPrivilegeEnum = WbemPrivilegeEnum(24i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemPrivilegeSyncAgent: WbemPrivilegeEnum = WbemPrivilegeEnum(25i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemPrivilegeEnableDelegation: WbemPrivilegeEnum = WbemPrivilegeEnum(26i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemPrivilegeManageVolume: WbemPrivilegeEnum = WbemPrivilegeEnum(27i32);
impl ::core::marker::Copy for WbemPrivilegeEnum {}
impl ::core::clone::Clone for WbemPrivilegeEnum {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WbemPrivilegeEnum {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WbemPrivilegeEnum {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WbemPrivilegeEnum {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WbemPrivilegeEnum").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WbemQueryFlagEnum(pub i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemQueryFlagDeep: WbemQueryFlagEnum = WbemQueryFlagEnum(0i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemQueryFlagShallow: WbemQueryFlagEnum = WbemQueryFlagEnum(1i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemQueryFlagPrototype: WbemQueryFlagEnum = WbemQueryFlagEnum(2i32);
impl ::core::marker::Copy for WbemQueryFlagEnum {}
impl ::core::clone::Clone for WbemQueryFlagEnum {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WbemQueryFlagEnum {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WbemQueryFlagEnum {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WbemQueryFlagEnum {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WbemQueryFlagEnum").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WbemTextFlagEnum(pub i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemTextFlagNoFlavors: WbemTextFlagEnum = WbemTextFlagEnum(1i32);
impl ::core::marker::Copy for WbemTextFlagEnum {}
impl ::core::clone::Clone for WbemTextFlagEnum {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WbemTextFlagEnum {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WbemTextFlagEnum {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WbemTextFlagEnum {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WbemTextFlagEnum").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WbemTimeout(pub i32);
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub const wbemTimeoutInfinite: WbemTimeout = WbemTimeout(-1i32);
impl ::core::marker::Copy for WbemTimeout {}
impl ::core::clone::Clone for WbemTimeout {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WbemTimeout {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WbemTimeout {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WbemTimeout {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WbemTimeout").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_Application {
    pub reserved1: u64,
    pub reserved2: isize,
    pub ft: *const MI_ApplicationFT,
}
impl ::core::marker::Copy for MI_Application {}
impl ::core::clone::Clone for MI_Application {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_Application {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_Application").field("reserved1", &self.reserved1).field("reserved2", &self.reserved2).field("ft", &self.ft).finish()
    }
}
impl ::windows::core::TypeKind for MI_Application {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MI_Application {
    fn eq(&self, other: &Self) -> bool {
        self.reserved1 == other.reserved1 && self.reserved2 == other.reserved2 && self.ft == other.ft
    }
}
impl ::core::cmp::Eq for MI_Application {}
impl ::core::default::Default for MI_Application {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
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
impl ::core::fmt::Debug for MI_ApplicationFT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ApplicationFT")
            .field("Close", &self.Close)
            .field("NewSession", &self.NewSession)
            .field("NewHostedProvider", &self.NewHostedProvider)
            .field("NewInstance", &self.NewInstance)
            .field("NewDestinationOptions", &self.NewDestinationOptions)
            .field("NewOperationOptions", &self.NewOperationOptions)
            .field("NewSubscriptionDeliveryOptions", &self.NewSubscriptionDeliveryOptions)
            .field("NewSerializer", &self.NewSerializer)
            .field("NewDeserializer", &self.NewDeserializer)
            .field("NewInstanceFromClass", &self.NewInstanceFromClass)
            .field("NewClass", &self.NewClass)
            .finish()
    }
}
impl ::windows::core::TypeKind for MI_ApplicationFT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MI_ApplicationFT {
    fn eq(&self, other: &Self) -> bool {
        self.Close == other.Close && self.NewSession == other.NewSession && self.NewHostedProvider == other.NewHostedProvider && self.NewInstance == other.NewInstance && self.NewDestinationOptions == other.NewDestinationOptions && self.NewOperationOptions == other.NewOperationOptions && self.NewSubscriptionDeliveryOptions == other.NewSubscriptionDeliveryOptions && self.NewSerializer == other.NewSerializer && self.NewDeserializer == other.NewDeserializer && self.NewInstanceFromClass == other.NewInstanceFromClass && self.NewClass == other.NewClass
    }
}
impl ::core::cmp::Eq for MI_ApplicationFT {}
impl ::core::default::Default for MI_ApplicationFT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
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
impl ::core::fmt::Debug for MI_Array {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_Array").field("data", &self.data).field("size", &self.size).finish()
    }
}
impl ::windows::core::TypeKind for MI_Array {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MI_Array {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data && self.size == other.size
    }
}
impl ::core::cmp::Eq for MI_Array {}
impl ::core::default::Default for MI_Array {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
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
impl ::core::fmt::Debug for MI_ArrayField {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ArrayField").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
impl ::windows::core::TypeKind for MI_ArrayField {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MI_ArrayField {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.exists == other.exists && self.flags == other.flags
    }
}
impl ::core::cmp::Eq for MI_ArrayField {}
impl ::core::default::Default for MI_ArrayField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
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
impl ::core::fmt::Debug for MI_BooleanA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_BooleanA").field("data", &self.data).field("size", &self.size).finish()
    }
}
impl ::windows::core::TypeKind for MI_BooleanA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MI_BooleanA {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data && self.size == other.size
    }
}
impl ::core::cmp::Eq for MI_BooleanA {}
impl ::core::default::Default for MI_BooleanA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
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
impl ::core::fmt::Debug for MI_BooleanAField {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_BooleanAField").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
impl ::windows::core::TypeKind for MI_BooleanAField {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MI_BooleanAField {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.exists == other.exists && self.flags == other.flags
    }
}
impl ::core::cmp::Eq for MI_BooleanAField {}
impl ::core::default::Default for MI_BooleanAField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
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
impl ::core::fmt::Debug for MI_BooleanField {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_BooleanField").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
impl ::windows::core::TypeKind for MI_BooleanField {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MI_BooleanField {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.exists == other.exists && self.flags == other.flags
    }
}
impl ::core::cmp::Eq for MI_BooleanField {}
impl ::core::default::Default for MI_BooleanField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
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
impl ::core::fmt::Debug for MI_Char16A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_Char16A").field("data", &self.data).field("size", &self.size).finish()
    }
}
impl ::windows::core::TypeKind for MI_Char16A {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MI_Char16A {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data && self.size == other.size
    }
}
impl ::core::cmp::Eq for MI_Char16A {}
impl ::core::default::Default for MI_Char16A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
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
impl ::core::fmt::Debug for MI_Char16AField {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_Char16AField").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
impl ::windows::core::TypeKind for MI_Char16AField {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MI_Char16AField {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.exists == other.exists && self.flags == other.flags
    }
}
impl ::core::cmp::Eq for MI_Char16AField {}
impl ::core::default::Default for MI_Char16AField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
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
impl ::core::fmt::Debug for MI_Char16Field {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_Char16Field").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
impl ::windows::core::TypeKind for MI_Char16Field {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MI_Char16Field {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.exists == other.exists && self.flags == other.flags
    }
}
impl ::core::cmp::Eq for MI_Char16Field {}
impl ::core::default::Default for MI_Char16Field {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_Class {
    pub ft: *const MI_ClassFT,
    pub classDecl: *const MI_ClassDecl,
    pub namespaceName: *const u16,
    pub serverName: *const u16,
    pub reserved: [isize; 4],
}
impl ::core::marker::Copy for MI_Class {}
impl ::core::clone::Clone for MI_Class {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_Class {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_Class").field("ft", &self.ft).field("classDecl", &self.classDecl).field("namespaceName", &self.namespaceName).field("serverName", &self.serverName).field("reserved", &self.reserved).finish()
    }
}
impl ::windows::core::TypeKind for MI_Class {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MI_Class {
    fn eq(&self, other: &Self) -> bool {
        self.ft == other.ft && self.classDecl == other.classDecl && self.namespaceName == other.namespaceName && self.serverName == other.serverName && self.reserved == other.reserved
    }
}
impl ::core::cmp::Eq for MI_Class {}
impl ::core::default::Default for MI_Class {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_ClassDecl {
    pub flags: u32,
    pub code: u32,
    pub name: *const u16,
    pub qualifiers: *const *const MI_Qualifier,
    pub numQualifiers: u32,
    pub properties: *const *const MI_PropertyDecl,
    pub numProperties: u32,
    pub size: u32,
    pub superClass: *const u16,
    pub superClassDecl: *const MI_ClassDecl,
    pub methods: *const *const MI_MethodDecl,
    pub numMethods: u32,
    pub schema: *const MI_SchemaDecl,
    pub providerFT: *const MI_ProviderFT,
    pub owningClass: *mut MI_Class,
}
impl ::core::marker::Copy for MI_ClassDecl {}
impl ::core::clone::Clone for MI_ClassDecl {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_ClassDecl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ClassDecl")
            .field("flags", &self.flags)
            .field("code", &self.code)
            .field("name", &self.name)
            .field("qualifiers", &self.qualifiers)
            .field("numQualifiers", &self.numQualifiers)
            .field("properties", &self.properties)
            .field("numProperties", &self.numProperties)
            .field("size", &self.size)
            .field("superClass", &self.superClass)
            .field("superClassDecl", &self.superClassDecl)
            .field("methods", &self.methods)
            .field("numMethods", &self.numMethods)
            .field("schema", &self.schema)
            .field("providerFT", &self.providerFT)
            .field("owningClass", &self.owningClass)
            .finish()
    }
}
impl ::windows::core::TypeKind for MI_ClassDecl {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MI_ClassDecl {
    fn eq(&self, other: &Self) -> bool {
        self.flags == other.flags && self.code == other.code && self.name == other.name && self.qualifiers == other.qualifiers && self.numQualifiers == other.numQualifiers && self.properties == other.properties && self.numProperties == other.numProperties && self.size == other.size && self.superClass == other.superClass && self.superClassDecl == other.superClassDecl && self.methods == other.methods && self.numMethods == other.numMethods && self.schema == other.schema && self.providerFT == other.providerFT && self.owningClass == other.owningClass
    }
}
impl ::core::cmp::Eq for MI_ClassDecl {}
impl ::core::default::Default for MI_ClassDecl {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
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
impl ::core::fmt::Debug for MI_ClassFT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ClassFT")
            .field("GetClassNameA", &self.GetClassNameA)
            .field("GetNameSpace", &self.GetNameSpace)
            .field("GetServerName", &self.GetServerName)
            .field("GetElementCount", &self.GetElementCount)
            .field("GetElement", &self.GetElement)
            .field("GetElementAt", &self.GetElementAt)
            .field("GetClassQualifierSet", &self.GetClassQualifierSet)
            .field("GetMethodCount", &self.GetMethodCount)
            .field("GetMethodAt", &self.GetMethodAt)
            .field("GetMethod", &self.GetMethod)
            .field("GetParentClassName", &self.GetParentClassName)
            .field("GetParentClass", &self.GetParentClass)
            .field("Delete", &self.Delete)
            .field("Clone", &self.Clone)
            .finish()
    }
}
impl ::windows::core::TypeKind for MI_ClassFT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MI_ClassFT {
    fn eq(&self, other: &Self) -> bool {
        self.GetClassNameA == other.GetClassNameA && self.GetNameSpace == other.GetNameSpace && self.GetServerName == other.GetServerName && self.GetElementCount == other.GetElementCount && self.GetElement == other.GetElement && self.GetElementAt == other.GetElementAt && self.GetClassQualifierSet == other.GetClassQualifierSet && self.GetMethodCount == other.GetMethodCount && self.GetMethodAt == other.GetMethodAt && self.GetMethod == other.GetMethod && self.GetParentClassName == other.GetParentClassName && self.GetParentClass == other.GetParentClass && self.Delete == other.Delete && self.Clone == other.Clone
    }
}
impl ::core::cmp::Eq for MI_ClassFT {}
impl ::core::default::Default for MI_ClassFT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_ClientFT_V1 {
    pub applicationFT: *const MI_ApplicationFT,
    pub sessionFT: *const MI_SessionFT,
    pub operationFT: *const MI_OperationFT,
    pub hostedProviderFT: *const MI_HostedProviderFT,
    pub serializerFT: *const MI_SerializerFT,
    pub deserializerFT: *const MI_DeserializerFT,
    pub subscribeDeliveryOptionsFT: *const MI_SubscriptionDeliveryOptionsFT,
    pub destinationOptionsFT: *const MI_DestinationOptionsFT,
    pub operationOptionsFT: *const MI_OperationOptionsFT,
    pub utilitiesFT: *const MI_UtilitiesFT,
}
impl ::core::marker::Copy for MI_ClientFT_V1 {}
impl ::core::clone::Clone for MI_ClientFT_V1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_ClientFT_V1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ClientFT_V1")
            .field("applicationFT", &self.applicationFT)
            .field("sessionFT", &self.sessionFT)
            .field("operationFT", &self.operationFT)
            .field("hostedProviderFT", &self.hostedProviderFT)
            .field("serializerFT", &self.serializerFT)
            .field("deserializerFT", &self.deserializerFT)
            .field("subscribeDeliveryOptionsFT", &self.subscribeDeliveryOptionsFT)
            .field("destinationOptionsFT", &self.destinationOptionsFT)
            .field("operationOptionsFT", &self.operationOptionsFT)
            .field("utilitiesFT", &self.utilitiesFT)
            .finish()
    }
}
impl ::windows::core::TypeKind for MI_ClientFT_V1 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MI_ClientFT_V1 {
    fn eq(&self, other: &Self) -> bool {
        self.applicationFT == other.applicationFT && self.sessionFT == other.sessionFT && self.operationFT == other.operationFT && self.hostedProviderFT == other.hostedProviderFT && self.serializerFT == other.serializerFT && self.deserializerFT == other.deserializerFT && self.subscribeDeliveryOptionsFT == other.subscribeDeliveryOptionsFT && self.destinationOptionsFT == other.destinationOptionsFT && self.operationOptionsFT == other.operationOptionsFT && self.utilitiesFT == other.utilitiesFT
    }
}
impl ::core::cmp::Eq for MI_ClientFT_V1 {}
impl ::core::default::Default for MI_ClientFT_V1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_ConstBooleanA {
    pub data: *const u8,
    pub size: u32,
}
impl ::core::marker::Copy for MI_ConstBooleanA {}
impl ::core::clone::Clone for MI_ConstBooleanA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_ConstBooleanA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ConstBooleanA").field("data", &self.data).field("size", &self.size).finish()
    }
}
impl ::windows::core::TypeKind for MI_ConstBooleanA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MI_ConstBooleanA {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data && self.size == other.size
    }
}
impl ::core::cmp::Eq for MI_ConstBooleanA {}
impl ::core::default::Default for MI_ConstBooleanA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
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
impl ::core::fmt::Debug for MI_ConstBooleanAField {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ConstBooleanAField").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
impl ::windows::core::TypeKind for MI_ConstBooleanAField {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MI_ConstBooleanAField {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.exists == other.exists && self.flags == other.flags
    }
}
impl ::core::cmp::Eq for MI_ConstBooleanAField {}
impl ::core::default::Default for MI_ConstBooleanAField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
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
impl ::core::fmt::Debug for MI_ConstBooleanField {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ConstBooleanField").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
impl ::windows::core::TypeKind for MI_ConstBooleanField {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MI_ConstBooleanField {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.exists == other.exists && self.flags == other.flags
    }
}
impl ::core::cmp::Eq for MI_ConstBooleanField {}
impl ::core::default::Default for MI_ConstBooleanField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_ConstChar16A {
    pub data: *const u16,
    pub size: u32,
}
impl ::core::marker::Copy for MI_ConstChar16A {}
impl ::core::clone::Clone for MI_ConstChar16A {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_ConstChar16A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ConstChar16A").field("data", &self.data).field("size", &self.size).finish()
    }
}
impl ::windows::core::TypeKind for MI_ConstChar16A {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MI_ConstChar16A {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data && self.size == other.size
    }
}
impl ::core::cmp::Eq for MI_ConstChar16A {}
impl ::core::default::Default for MI_ConstChar16A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
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
impl ::core::fmt::Debug for MI_ConstChar16AField {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ConstChar16AField").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
impl ::windows::core::TypeKind for MI_ConstChar16AField {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MI_ConstChar16AField {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.exists == other.exists && self.flags == other.flags
    }
}
impl ::core::cmp::Eq for MI_ConstChar16AField {}
impl ::core::default::Default for MI_ConstChar16AField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
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
impl ::core::fmt::Debug for MI_ConstChar16Field {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ConstChar16Field").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
impl ::windows::core::TypeKind for MI_ConstChar16Field {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MI_ConstChar16Field {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.exists == other.exists && self.flags == other.flags
    }
}
impl ::core::cmp::Eq for MI_ConstChar16Field {}
impl ::core::default::Default for MI_ConstChar16Field {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_ConstDatetimeA {
    pub data: *const MI_Datetime,
    pub size: u32,
}
impl ::core::marker::Copy for MI_ConstDatetimeA {}
impl ::core::clone::Clone for MI_ConstDatetimeA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_ConstDatetimeA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ConstDatetimeA").field("data", &self.data).field("size", &self.size).finish()
    }
}
impl ::windows::core::TypeKind for MI_ConstDatetimeA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MI_ConstDatetimeA {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data && self.size == other.size
    }
}
impl ::core::cmp::Eq for MI_ConstDatetimeA {}
impl ::core::default::Default for MI_ConstDatetimeA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
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
impl ::core::fmt::Debug for MI_ConstDatetimeAField {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ConstDatetimeAField").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
impl ::windows::core::TypeKind for MI_ConstDatetimeAField {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MI_ConstDatetimeAField {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.exists == other.exists && self.flags == other.flags
    }
}
impl ::core::cmp::Eq for MI_ConstDatetimeAField {}
impl ::core::default::Default for MI_ConstDatetimeAField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
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
impl ::windows::core::TypeKind for MI_ConstDatetimeField {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for MI_ConstDatetimeField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_ConstInstanceA {
    pub data: *const *const MI_Instance,
    pub size: u32,
}
impl ::core::marker::Copy for MI_ConstInstanceA {}
impl ::core::clone::Clone for MI_ConstInstanceA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_ConstInstanceA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ConstInstanceA").field("data", &self.data).field("size", &self.size).finish()
    }
}
impl ::windows::core::TypeKind for MI_ConstInstanceA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MI_ConstInstanceA {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data && self.size == other.size
    }
}
impl ::core::cmp::Eq for MI_ConstInstanceA {}
impl ::core::default::Default for MI_ConstInstanceA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
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
impl ::core::fmt::Debug for MI_ConstInstanceAField {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ConstInstanceAField").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
impl ::windows::core::TypeKind for MI_ConstInstanceAField {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MI_ConstInstanceAField {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.exists == other.exists && self.flags == other.flags
    }
}
impl ::core::cmp::Eq for MI_ConstInstanceAField {}
impl ::core::default::Default for MI_ConstInstanceAField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_ConstInstanceField {
    pub value: *const MI_Instance,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_ConstInstanceField {}
impl ::core::clone::Clone for MI_ConstInstanceField {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_ConstInstanceField {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ConstInstanceField").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
impl ::windows::core::TypeKind for MI_ConstInstanceField {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MI_ConstInstanceField {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.exists == other.exists && self.flags == other.flags
    }
}
impl ::core::cmp::Eq for MI_ConstInstanceField {}
impl ::core::default::Default for MI_ConstInstanceField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_ConstReal32A {
    pub data: *const f32,
    pub size: u32,
}
impl ::core::marker::Copy for MI_ConstReal32A {}
impl ::core::clone::Clone for MI_ConstReal32A {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_ConstReal32A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ConstReal32A").field("data", &self.data).field("size", &self.size).finish()
    }
}
impl ::windows::core::TypeKind for MI_ConstReal32A {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MI_ConstReal32A {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data && self.size == other.size
    }
}
impl ::core::cmp::Eq for MI_ConstReal32A {}
impl ::core::default::Default for MI_ConstReal32A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
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
impl ::core::fmt::Debug for MI_ConstReal32AField {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ConstReal32AField").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
impl ::windows::core::TypeKind for MI_ConstReal32AField {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MI_ConstReal32AField {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.exists == other.exists && self.flags == other.flags
    }
}
impl ::core::cmp::Eq for MI_ConstReal32AField {}
impl ::core::default::Default for MI_ConstReal32AField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
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
impl ::core::fmt::Debug for MI_ConstReal32Field {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ConstReal32Field").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
impl ::windows::core::TypeKind for MI_ConstReal32Field {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MI_ConstReal32Field {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.exists == other.exists && self.flags == other.flags
    }
}
impl ::core::cmp::Eq for MI_ConstReal32Field {}
impl ::core::default::Default for MI_ConstReal32Field {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_ConstReal64A {
    pub data: *const f64,
    pub size: u32,
}
impl ::core::marker::Copy for MI_ConstReal64A {}
impl ::core::clone::Clone for MI_ConstReal64A {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_ConstReal64A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ConstReal64A").field("data", &self.data).field("size", &self.size).finish()
    }
}
impl ::windows::core::TypeKind for MI_ConstReal64A {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MI_ConstReal64A {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data && self.size == other.size
    }
}
impl ::core::cmp::Eq for MI_ConstReal64A {}
impl ::core::default::Default for MI_ConstReal64A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
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
impl ::core::fmt::Debug for MI_ConstReal64AField {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ConstReal64AField").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
impl ::windows::core::TypeKind for MI_ConstReal64AField {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MI_ConstReal64AField {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.exists == other.exists && self.flags == other.flags
    }
}
impl ::core::cmp::Eq for MI_ConstReal64AField {}
impl ::core::default::Default for MI_ConstReal64AField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
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
impl ::core::fmt::Debug for MI_ConstReal64Field {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ConstReal64Field").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
impl ::windows::core::TypeKind for MI_ConstReal64Field {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MI_ConstReal64Field {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.exists == other.exists && self.flags == other.flags
    }
}
impl ::core::cmp::Eq for MI_ConstReal64Field {}
impl ::core::default::Default for MI_ConstReal64Field {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_ConstReferenceA {
    pub data: *const *const MI_Instance,
    pub size: u32,
}
impl ::core::marker::Copy for MI_ConstReferenceA {}
impl ::core::clone::Clone for MI_ConstReferenceA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_ConstReferenceA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ConstReferenceA").field("data", &self.data).field("size", &self.size).finish()
    }
}
impl ::windows::core::TypeKind for MI_ConstReferenceA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MI_ConstReferenceA {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data && self.size == other.size
    }
}
impl ::core::cmp::Eq for MI_ConstReferenceA {}
impl ::core::default::Default for MI_ConstReferenceA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
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
impl ::core::fmt::Debug for MI_ConstReferenceAField {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ConstReferenceAField").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
impl ::windows::core::TypeKind for MI_ConstReferenceAField {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MI_ConstReferenceAField {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.exists == other.exists && self.flags == other.flags
    }
}
impl ::core::cmp::Eq for MI_ConstReferenceAField {}
impl ::core::default::Default for MI_ConstReferenceAField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_ConstReferenceField {
    pub value: *const MI_Instance,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_ConstReferenceField {}
impl ::core::clone::Clone for MI_ConstReferenceField {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_ConstReferenceField {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ConstReferenceField").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
impl ::windows::core::TypeKind for MI_ConstReferenceField {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MI_ConstReferenceField {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.exists == other.exists && self.flags == other.flags
    }
}
impl ::core::cmp::Eq for MI_ConstReferenceField {}
impl ::core::default::Default for MI_ConstReferenceField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_ConstSint16A {
    pub data: *const i16,
    pub size: u32,
}
impl ::core::marker::Copy for MI_ConstSint16A {}
impl ::core::clone::Clone for MI_ConstSint16A {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_ConstSint16A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ConstSint16A").field("data", &self.data).field("size", &self.size).finish()
    }
}
impl ::windows::core::TypeKind for MI_ConstSint16A {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MI_ConstSint16A {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data && self.size == other.size
    }
}
impl ::core::cmp::Eq for MI_ConstSint16A {}
impl ::core::default::Default for MI_ConstSint16A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
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
impl ::core::fmt::Debug for MI_ConstSint16AField {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ConstSint16AField").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
impl ::windows::core::TypeKind for MI_ConstSint16AField {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MI_ConstSint16AField {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.exists == other.exists && self.flags == other.flags
    }
}
impl ::core::cmp::Eq for MI_ConstSint16AField {}
impl ::core::default::Default for MI_ConstSint16AField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
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
impl ::core::fmt::Debug for MI_ConstSint16Field {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ConstSint16Field").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
impl ::windows::core::TypeKind for MI_ConstSint16Field {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MI_ConstSint16Field {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.exists == other.exists && self.flags == other.flags
    }
}
impl ::core::cmp::Eq for MI_ConstSint16Field {}
impl ::core::default::Default for MI_ConstSint16Field {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_ConstSint32A {
    pub data: *const i32,
    pub size: u32,
}
impl ::core::marker::Copy for MI_ConstSint32A {}
impl ::core::clone::Clone for MI_ConstSint32A {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_ConstSint32A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ConstSint32A").field("data", &self.data).field("size", &self.size).finish()
    }
}
impl ::windows::core::TypeKind for MI_ConstSint32A {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MI_ConstSint32A {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data && self.size == other.size
    }
}
impl ::core::cmp::Eq for MI_ConstSint32A {}
impl ::core::default::Default for MI_ConstSint32A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
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
impl ::core::fmt::Debug for MI_ConstSint32AField {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ConstSint32AField").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
impl ::windows::core::TypeKind for MI_ConstSint32AField {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MI_ConstSint32AField {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.exists == other.exists && self.flags == other.flags
    }
}
impl ::core::cmp::Eq for MI_ConstSint32AField {}
impl ::core::default::Default for MI_ConstSint32AField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
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
impl ::core::fmt::Debug for MI_ConstSint32Field {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ConstSint32Field").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
impl ::windows::core::TypeKind for MI_ConstSint32Field {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MI_ConstSint32Field {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.exists == other.exists && self.flags == other.flags
    }
}
impl ::core::cmp::Eq for MI_ConstSint32Field {}
impl ::core::default::Default for MI_ConstSint32Field {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_ConstSint64A {
    pub data: *const i64,
    pub size: u32,
}
impl ::core::marker::Copy for MI_ConstSint64A {}
impl ::core::clone::Clone for MI_ConstSint64A {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_ConstSint64A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ConstSint64A").field("data", &self.data).field("size", &self.size).finish()
    }
}
impl ::windows::core::TypeKind for MI_ConstSint64A {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MI_ConstSint64A {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data && self.size == other.size
    }
}
impl ::core::cmp::Eq for MI_ConstSint64A {}
impl ::core::default::Default for MI_ConstSint64A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
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
impl ::core::fmt::Debug for MI_ConstSint64AField {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ConstSint64AField").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
impl ::windows::core::TypeKind for MI_ConstSint64AField {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MI_ConstSint64AField {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.exists == other.exists && self.flags == other.flags
    }
}
impl ::core::cmp::Eq for MI_ConstSint64AField {}
impl ::core::default::Default for MI_ConstSint64AField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
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
impl ::core::fmt::Debug for MI_ConstSint64Field {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ConstSint64Field").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
impl ::windows::core::TypeKind for MI_ConstSint64Field {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MI_ConstSint64Field {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.exists == other.exists && self.flags == other.flags
    }
}
impl ::core::cmp::Eq for MI_ConstSint64Field {}
impl ::core::default::Default for MI_ConstSint64Field {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_ConstSint8A {
    pub data: *const i8,
    pub size: u32,
}
impl ::core::marker::Copy for MI_ConstSint8A {}
impl ::core::clone::Clone for MI_ConstSint8A {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_ConstSint8A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ConstSint8A").field("data", &self.data).field("size", &self.size).finish()
    }
}
impl ::windows::core::TypeKind for MI_ConstSint8A {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MI_ConstSint8A {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data && self.size == other.size
    }
}
impl ::core::cmp::Eq for MI_ConstSint8A {}
impl ::core::default::Default for MI_ConstSint8A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
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
impl ::core::fmt::Debug for MI_ConstSint8AField {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ConstSint8AField").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
impl ::windows::core::TypeKind for MI_ConstSint8AField {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MI_ConstSint8AField {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.exists == other.exists && self.flags == other.flags
    }
}
impl ::core::cmp::Eq for MI_ConstSint8AField {}
impl ::core::default::Default for MI_ConstSint8AField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
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
impl ::core::fmt::Debug for MI_ConstSint8Field {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ConstSint8Field").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
impl ::windows::core::TypeKind for MI_ConstSint8Field {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MI_ConstSint8Field {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.exists == other.exists && self.flags == other.flags
    }
}
impl ::core::cmp::Eq for MI_ConstSint8Field {}
impl ::core::default::Default for MI_ConstSint8Field {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_ConstStringA {
    pub data: *const *const u16,
    pub size: u32,
}
impl ::core::marker::Copy for MI_ConstStringA {}
impl ::core::clone::Clone for MI_ConstStringA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_ConstStringA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ConstStringA").field("data", &self.data).field("size", &self.size).finish()
    }
}
impl ::windows::core::TypeKind for MI_ConstStringA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MI_ConstStringA {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data && self.size == other.size
    }
}
impl ::core::cmp::Eq for MI_ConstStringA {}
impl ::core::default::Default for MI_ConstStringA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
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
impl ::core::fmt::Debug for MI_ConstStringAField {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ConstStringAField").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
impl ::windows::core::TypeKind for MI_ConstStringAField {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MI_ConstStringAField {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.exists == other.exists && self.flags == other.flags
    }
}
impl ::core::cmp::Eq for MI_ConstStringAField {}
impl ::core::default::Default for MI_ConstStringAField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_ConstStringField {
    pub value: *const u16,
    pub exists: u8,
    pub flags: u8,
}
impl ::core::marker::Copy for MI_ConstStringField {}
impl ::core::clone::Clone for MI_ConstStringField {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_ConstStringField {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ConstStringField").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
impl ::windows::core::TypeKind for MI_ConstStringField {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MI_ConstStringField {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.exists == other.exists && self.flags == other.flags
    }
}
impl ::core::cmp::Eq for MI_ConstStringField {}
impl ::core::default::Default for MI_ConstStringField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_ConstUint16A {
    pub data: *const u16,
    pub size: u32,
}
impl ::core::marker::Copy for MI_ConstUint16A {}
impl ::core::clone::Clone for MI_ConstUint16A {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_ConstUint16A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ConstUint16A").field("data", &self.data).field("size", &self.size).finish()
    }
}
impl ::windows::core::TypeKind for MI_ConstUint16A {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MI_ConstUint16A {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data && self.size == other.size
    }
}
impl ::core::cmp::Eq for MI_ConstUint16A {}
impl ::core::default::Default for MI_ConstUint16A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
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
impl ::core::fmt::Debug for MI_ConstUint16AField {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ConstUint16AField").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
impl ::windows::core::TypeKind for MI_ConstUint16AField {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MI_ConstUint16AField {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.exists == other.exists && self.flags == other.flags
    }
}
impl ::core::cmp::Eq for MI_ConstUint16AField {}
impl ::core::default::Default for MI_ConstUint16AField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
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
impl ::core::fmt::Debug for MI_ConstUint16Field {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ConstUint16Field").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
impl ::windows::core::TypeKind for MI_ConstUint16Field {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MI_ConstUint16Field {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.exists == other.exists && self.flags == other.flags
    }
}
impl ::core::cmp::Eq for MI_ConstUint16Field {}
impl ::core::default::Default for MI_ConstUint16Field {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_ConstUint32A {
    pub data: *const u32,
    pub size: u32,
}
impl ::core::marker::Copy for MI_ConstUint32A {}
impl ::core::clone::Clone for MI_ConstUint32A {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_ConstUint32A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ConstUint32A").field("data", &self.data).field("size", &self.size).finish()
    }
}
impl ::windows::core::TypeKind for MI_ConstUint32A {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MI_ConstUint32A {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data && self.size == other.size
    }
}
impl ::core::cmp::Eq for MI_ConstUint32A {}
impl ::core::default::Default for MI_ConstUint32A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
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
impl ::core::fmt::Debug for MI_ConstUint32AField {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ConstUint32AField").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
impl ::windows::core::TypeKind for MI_ConstUint32AField {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MI_ConstUint32AField {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.exists == other.exists && self.flags == other.flags
    }
}
impl ::core::cmp::Eq for MI_ConstUint32AField {}
impl ::core::default::Default for MI_ConstUint32AField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
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
impl ::core::fmt::Debug for MI_ConstUint32Field {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ConstUint32Field").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
impl ::windows::core::TypeKind for MI_ConstUint32Field {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MI_ConstUint32Field {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.exists == other.exists && self.flags == other.flags
    }
}
impl ::core::cmp::Eq for MI_ConstUint32Field {}
impl ::core::default::Default for MI_ConstUint32Field {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_ConstUint64A {
    pub data: *const u64,
    pub size: u32,
}
impl ::core::marker::Copy for MI_ConstUint64A {}
impl ::core::clone::Clone for MI_ConstUint64A {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_ConstUint64A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ConstUint64A").field("data", &self.data).field("size", &self.size).finish()
    }
}
impl ::windows::core::TypeKind for MI_ConstUint64A {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MI_ConstUint64A {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data && self.size == other.size
    }
}
impl ::core::cmp::Eq for MI_ConstUint64A {}
impl ::core::default::Default for MI_ConstUint64A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
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
impl ::core::fmt::Debug for MI_ConstUint64AField {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ConstUint64AField").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
impl ::windows::core::TypeKind for MI_ConstUint64AField {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MI_ConstUint64AField {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.exists == other.exists && self.flags == other.flags
    }
}
impl ::core::cmp::Eq for MI_ConstUint64AField {}
impl ::core::default::Default for MI_ConstUint64AField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
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
impl ::core::fmt::Debug for MI_ConstUint64Field {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ConstUint64Field").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
impl ::windows::core::TypeKind for MI_ConstUint64Field {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MI_ConstUint64Field {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.exists == other.exists && self.flags == other.flags
    }
}
impl ::core::cmp::Eq for MI_ConstUint64Field {}
impl ::core::default::Default for MI_ConstUint64Field {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_ConstUint8A {
    pub data: *const u8,
    pub size: u32,
}
impl ::core::marker::Copy for MI_ConstUint8A {}
impl ::core::clone::Clone for MI_ConstUint8A {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_ConstUint8A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ConstUint8A").field("data", &self.data).field("size", &self.size).finish()
    }
}
impl ::windows::core::TypeKind for MI_ConstUint8A {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MI_ConstUint8A {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data && self.size == other.size
    }
}
impl ::core::cmp::Eq for MI_ConstUint8A {}
impl ::core::default::Default for MI_ConstUint8A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
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
impl ::core::fmt::Debug for MI_ConstUint8AField {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ConstUint8AField").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
impl ::windows::core::TypeKind for MI_ConstUint8AField {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MI_ConstUint8AField {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.exists == other.exists && self.flags == other.flags
    }
}
impl ::core::cmp::Eq for MI_ConstUint8AField {}
impl ::core::default::Default for MI_ConstUint8AField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
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
impl ::core::fmt::Debug for MI_ConstUint8Field {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ConstUint8Field").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
impl ::windows::core::TypeKind for MI_ConstUint8Field {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MI_ConstUint8Field {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.exists == other.exists && self.flags == other.flags
    }
}
impl ::core::cmp::Eq for MI_ConstUint8Field {}
impl ::core::default::Default for MI_ConstUint8Field {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_Context {
    pub ft: *const MI_ContextFT,
    pub reserved: [isize; 3],
}
impl ::core::marker::Copy for MI_Context {}
impl ::core::clone::Clone for MI_Context {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_Context {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_Context").field("ft", &self.ft).field("reserved", &self.reserved).finish()
    }
}
impl ::windows::core::TypeKind for MI_Context {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MI_Context {
    fn eq(&self, other: &Self) -> bool {
        self.ft == other.ft && self.reserved == other.reserved
    }
}
impl ::core::cmp::Eq for MI_Context {}
impl ::core::default::Default for MI_Context {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
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
impl ::core::fmt::Debug for MI_ContextFT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ContextFT")
            .field("PostResult", &self.PostResult)
            .field("PostInstance", &self.PostInstance)
            .field("PostIndication", &self.PostIndication)
            .field("ConstructInstance", &self.ConstructInstance)
            .field("ConstructParameters", &self.ConstructParameters)
            .field("NewInstance", &self.NewInstance)
            .field("NewDynamicInstance", &self.NewDynamicInstance)
            .field("NewParameters", &self.NewParameters)
            .field("Canceled", &self.Canceled)
            .field("GetLocale", &self.GetLocale)
            .field("RegisterCancel", &self.RegisterCancel)
            .field("RequestUnload", &self.RequestUnload)
            .field("RefuseUnload", &self.RefuseUnload)
            .field("GetLocalSession", &self.GetLocalSession)
            .field("SetStringOption", &self.SetStringOption)
            .field("GetStringOption", &self.GetStringOption)
            .field("GetNumberOption", &self.GetNumberOption)
            .field("GetCustomOption", &self.GetCustomOption)
            .field("GetCustomOptionCount", &self.GetCustomOptionCount)
            .field("GetCustomOptionAt", &self.GetCustomOptionAt)
            .field("WriteMessage", &self.WriteMessage)
            .field("WriteProgress", &self.WriteProgress)
            .field("WriteStreamParameter", &self.WriteStreamParameter)
            .field("WriteCimError", &self.WriteCimError)
            .field("PromptUser", &self.PromptUser)
            .field("ShouldProcess", &self.ShouldProcess)
            .field("ShouldContinue", &self.ShouldContinue)
            .field("PostError", &self.PostError)
            .field("PostCimError", &self.PostCimError)
            .field("WriteError", &self.WriteError)
            .finish()
    }
}
impl ::windows::core::TypeKind for MI_ContextFT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MI_ContextFT {
    fn eq(&self, other: &Self) -> bool {
        self.PostResult == other.PostResult
            && self.PostInstance == other.PostInstance
            && self.PostIndication == other.PostIndication
            && self.ConstructInstance == other.ConstructInstance
            && self.ConstructParameters == other.ConstructParameters
            && self.NewInstance == other.NewInstance
            && self.NewDynamicInstance == other.NewDynamicInstance
            && self.NewParameters == other.NewParameters
            && self.Canceled == other.Canceled
            && self.GetLocale == other.GetLocale
            && self.RegisterCancel == other.RegisterCancel
            && self.RequestUnload == other.RequestUnload
            && self.RefuseUnload == other.RefuseUnload
            && self.GetLocalSession == other.GetLocalSession
            && self.SetStringOption == other.SetStringOption
            && self.GetStringOption == other.GetStringOption
            && self.GetNumberOption == other.GetNumberOption
            && self.GetCustomOption == other.GetCustomOption
            && self.GetCustomOptionCount == other.GetCustomOptionCount
            && self.GetCustomOptionAt == other.GetCustomOptionAt
            && self.WriteMessage == other.WriteMessage
            && self.WriteProgress == other.WriteProgress
            && self.WriteStreamParameter == other.WriteStreamParameter
            && self.WriteCimError == other.WriteCimError
            && self.PromptUser == other.PromptUser
            && self.ShouldProcess == other.ShouldProcess
            && self.ShouldContinue == other.ShouldContinue
            && self.PostError == other.PostError
            && self.PostCimError == other.PostCimError
            && self.WriteError == other.WriteError
    }
}
impl ::core::cmp::Eq for MI_ContextFT {}
impl ::core::default::Default for MI_ContextFT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
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
impl ::windows::core::TypeKind for MI_Datetime {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for MI_Datetime {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
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
impl ::windows::core::TypeKind for MI_Datetime_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for MI_Datetime_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
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
impl ::core::fmt::Debug for MI_DatetimeA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_DatetimeA").field("data", &self.data).field("size", &self.size).finish()
    }
}
impl ::windows::core::TypeKind for MI_DatetimeA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MI_DatetimeA {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data && self.size == other.size
    }
}
impl ::core::cmp::Eq for MI_DatetimeA {}
impl ::core::default::Default for MI_DatetimeA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
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
impl ::core::fmt::Debug for MI_DatetimeAField {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_DatetimeAField").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
impl ::windows::core::TypeKind for MI_DatetimeAField {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MI_DatetimeAField {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.exists == other.exists && self.flags == other.flags
    }
}
impl ::core::cmp::Eq for MI_DatetimeAField {}
impl ::core::default::Default for MI_DatetimeAField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
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
impl ::windows::core::TypeKind for MI_DatetimeField {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for MI_DatetimeField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
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
impl ::core::fmt::Debug for MI_Deserializer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_Deserializer").field("reserved1", &self.reserved1).field("reserved2", &self.reserved2).finish()
    }
}
impl ::windows::core::TypeKind for MI_Deserializer {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MI_Deserializer {
    fn eq(&self, other: &Self) -> bool {
        self.reserved1 == other.reserved1 && self.reserved2 == other.reserved2
    }
}
impl ::core::cmp::Eq for MI_Deserializer {}
impl ::core::default::Default for MI_Deserializer {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
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
impl ::core::fmt::Debug for MI_DeserializerFT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_DeserializerFT").field("Close", &self.Close).field("DeserializeClass", &self.DeserializeClass).field("Class_GetClassName", &self.Class_GetClassName).field("Class_GetParentClassName", &self.Class_GetParentClassName).field("DeserializeInstance", &self.DeserializeInstance).field("Instance_GetClassName", &self.Instance_GetClassName).finish()
    }
}
impl ::windows::core::TypeKind for MI_DeserializerFT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MI_DeserializerFT {
    fn eq(&self, other: &Self) -> bool {
        self.Close == other.Close && self.DeserializeClass == other.DeserializeClass && self.Class_GetClassName == other.Class_GetClassName && self.Class_GetParentClassName == other.Class_GetParentClassName && self.DeserializeInstance == other.DeserializeInstance && self.Instance_GetClassName == other.Instance_GetClassName
    }
}
impl ::core::cmp::Eq for MI_DeserializerFT {}
impl ::core::default::Default for MI_DeserializerFT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_DestinationOptions {
    pub reserved1: u64,
    pub reserved2: isize,
    pub ft: *const MI_DestinationOptionsFT,
}
impl ::core::marker::Copy for MI_DestinationOptions {}
impl ::core::clone::Clone for MI_DestinationOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_DestinationOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_DestinationOptions").field("reserved1", &self.reserved1).field("reserved2", &self.reserved2).field("ft", &self.ft).finish()
    }
}
impl ::windows::core::TypeKind for MI_DestinationOptions {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MI_DestinationOptions {
    fn eq(&self, other: &Self) -> bool {
        self.reserved1 == other.reserved1 && self.reserved2 == other.reserved2 && self.ft == other.ft
    }
}
impl ::core::cmp::Eq for MI_DestinationOptions {}
impl ::core::default::Default for MI_DestinationOptions {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
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
impl ::core::fmt::Debug for MI_DestinationOptionsFT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_DestinationOptionsFT")
            .field("Delete", &self.Delete)
            .field("SetString", &self.SetString)
            .field("SetNumber", &self.SetNumber)
            .field("AddCredentials", &self.AddCredentials)
            .field("GetString", &self.GetString)
            .field("GetNumber", &self.GetNumber)
            .field("GetOptionCount", &self.GetOptionCount)
            .field("GetOptionAt", &self.GetOptionAt)
            .field("GetOption", &self.GetOption)
            .field("GetCredentialsCount", &self.GetCredentialsCount)
            .field("GetCredentialsAt", &self.GetCredentialsAt)
            .field("GetCredentialsPasswordAt", &self.GetCredentialsPasswordAt)
            .field("Clone", &self.Clone)
            .field("SetInterval", &self.SetInterval)
            .field("GetInterval", &self.GetInterval)
            .finish()
    }
}
impl ::windows::core::TypeKind for MI_DestinationOptionsFT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MI_DestinationOptionsFT {
    fn eq(&self, other: &Self) -> bool {
        self.Delete == other.Delete && self.SetString == other.SetString && self.SetNumber == other.SetNumber && self.AddCredentials == other.AddCredentials && self.GetString == other.GetString && self.GetNumber == other.GetNumber && self.GetOptionCount == other.GetOptionCount && self.GetOptionAt == other.GetOptionAt && self.GetOption == other.GetOption && self.GetCredentialsCount == other.GetCredentialsCount && self.GetCredentialsAt == other.GetCredentialsAt && self.GetCredentialsPasswordAt == other.GetCredentialsPasswordAt && self.Clone == other.Clone && self.SetInterval == other.SetInterval && self.GetInterval == other.GetInterval
    }
}
impl ::core::cmp::Eq for MI_DestinationOptionsFT {}
impl ::core::default::Default for MI_DestinationOptionsFT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_FeatureDecl {
    pub flags: u32,
    pub code: u32,
    pub name: *const u16,
    pub qualifiers: *const *const MI_Qualifier,
    pub numQualifiers: u32,
}
impl ::core::marker::Copy for MI_FeatureDecl {}
impl ::core::clone::Clone for MI_FeatureDecl {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_FeatureDecl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_FeatureDecl").field("flags", &self.flags).field("code", &self.code).field("name", &self.name).field("qualifiers", &self.qualifiers).field("numQualifiers", &self.numQualifiers).finish()
    }
}
impl ::windows::core::TypeKind for MI_FeatureDecl {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MI_FeatureDecl {
    fn eq(&self, other: &Self) -> bool {
        self.flags == other.flags && self.code == other.code && self.name == other.name && self.qualifiers == other.qualifiers && self.numQualifiers == other.numQualifiers
    }
}
impl ::core::cmp::Eq for MI_FeatureDecl {}
impl ::core::default::Default for MI_FeatureDecl {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_Filter {
    pub ft: *const MI_FilterFT,
    pub reserved: [isize; 3],
}
impl ::core::marker::Copy for MI_Filter {}
impl ::core::clone::Clone for MI_Filter {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_Filter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_Filter").field("ft", &self.ft).field("reserved", &self.reserved).finish()
    }
}
impl ::windows::core::TypeKind for MI_Filter {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MI_Filter {
    fn eq(&self, other: &Self) -> bool {
        self.ft == other.ft && self.reserved == other.reserved
    }
}
impl ::core::cmp::Eq for MI_Filter {}
impl ::core::default::Default for MI_Filter {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
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
impl ::core::fmt::Debug for MI_FilterFT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_FilterFT").field("Evaluate", &self.Evaluate).field("GetExpression", &self.GetExpression).finish()
    }
}
impl ::windows::core::TypeKind for MI_FilterFT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MI_FilterFT {
    fn eq(&self, other: &Self) -> bool {
        self.Evaluate == other.Evaluate && self.GetExpression == other.GetExpression
    }
}
impl ::core::cmp::Eq for MI_FilterFT {}
impl ::core::default::Default for MI_FilterFT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_HostedProvider {
    pub reserved1: u64,
    pub reserved2: isize,
    pub ft: *const MI_HostedProviderFT,
}
impl ::core::marker::Copy for MI_HostedProvider {}
impl ::core::clone::Clone for MI_HostedProvider {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_HostedProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_HostedProvider").field("reserved1", &self.reserved1).field("reserved2", &self.reserved2).field("ft", &self.ft).finish()
    }
}
impl ::windows::core::TypeKind for MI_HostedProvider {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MI_HostedProvider {
    fn eq(&self, other: &Self) -> bool {
        self.reserved1 == other.reserved1 && self.reserved2 == other.reserved2 && self.ft == other.ft
    }
}
impl ::core::cmp::Eq for MI_HostedProvider {}
impl ::core::default::Default for MI_HostedProvider {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
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
impl ::core::fmt::Debug for MI_HostedProviderFT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_HostedProviderFT").field("Close", &self.Close).field("GetApplication", &self.GetApplication).finish()
    }
}
impl ::windows::core::TypeKind for MI_HostedProviderFT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MI_HostedProviderFT {
    fn eq(&self, other: &Self) -> bool {
        self.Close == other.Close && self.GetApplication == other.GetApplication
    }
}
impl ::core::cmp::Eq for MI_HostedProviderFT {}
impl ::core::default::Default for MI_HostedProviderFT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_Instance {
    pub ft: *const MI_InstanceFT,
    pub classDecl: *const MI_ClassDecl,
    pub serverName: *const u16,
    pub nameSpace: *const u16,
    pub reserved: [isize; 4],
}
impl ::core::marker::Copy for MI_Instance {}
impl ::core::clone::Clone for MI_Instance {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_Instance {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_Instance").field("ft", &self.ft).field("classDecl", &self.classDecl).field("serverName", &self.serverName).field("nameSpace", &self.nameSpace).field("reserved", &self.reserved).finish()
    }
}
impl ::windows::core::TypeKind for MI_Instance {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MI_Instance {
    fn eq(&self, other: &Self) -> bool {
        self.ft == other.ft && self.classDecl == other.classDecl && self.serverName == other.serverName && self.nameSpace == other.nameSpace && self.reserved == other.reserved
    }
}
impl ::core::cmp::Eq for MI_Instance {}
impl ::core::default::Default for MI_Instance {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
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
impl ::core::fmt::Debug for MI_InstanceA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_InstanceA").field("data", &self.data).field("size", &self.size).finish()
    }
}
impl ::windows::core::TypeKind for MI_InstanceA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MI_InstanceA {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data && self.size == other.size
    }
}
impl ::core::cmp::Eq for MI_InstanceA {}
impl ::core::default::Default for MI_InstanceA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
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
impl ::core::fmt::Debug for MI_InstanceAField {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_InstanceAField").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
impl ::windows::core::TypeKind for MI_InstanceAField {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MI_InstanceAField {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.exists == other.exists && self.flags == other.flags
    }
}
impl ::core::cmp::Eq for MI_InstanceAField {}
impl ::core::default::Default for MI_InstanceAField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
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
impl ::core::fmt::Debug for MI_InstanceExFT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_InstanceExFT").field("parent", &self.parent).field("Normalize", &self.Normalize).finish()
    }
}
impl ::windows::core::TypeKind for MI_InstanceExFT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MI_InstanceExFT {
    fn eq(&self, other: &Self) -> bool {
        self.parent == other.parent && self.Normalize == other.Normalize
    }
}
impl ::core::cmp::Eq for MI_InstanceExFT {}
impl ::core::default::Default for MI_InstanceExFT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
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
impl ::core::fmt::Debug for MI_InstanceFT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_InstanceFT")
            .field("Clone", &self.Clone)
            .field("Destruct", &self.Destruct)
            .field("Delete", &self.Delete)
            .field("IsA", &self.IsA)
            .field("GetClassNameA", &self.GetClassNameA)
            .field("SetNameSpace", &self.SetNameSpace)
            .field("GetNameSpace", &self.GetNameSpace)
            .field("GetElementCount", &self.GetElementCount)
            .field("AddElement", &self.AddElement)
            .field("SetElement", &self.SetElement)
            .field("SetElementAt", &self.SetElementAt)
            .field("GetElement", &self.GetElement)
            .field("GetElementAt", &self.GetElementAt)
            .field("ClearElement", &self.ClearElement)
            .field("ClearElementAt", &self.ClearElementAt)
            .field("GetServerName", &self.GetServerName)
            .field("SetServerName", &self.SetServerName)
            .field("GetClass", &self.GetClass)
            .finish()
    }
}
impl ::windows::core::TypeKind for MI_InstanceFT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MI_InstanceFT {
    fn eq(&self, other: &Self) -> bool {
        self.Clone == other.Clone && self.Destruct == other.Destruct && self.Delete == other.Delete && self.IsA == other.IsA && self.GetClassNameA == other.GetClassNameA && self.SetNameSpace == other.SetNameSpace && self.GetNameSpace == other.GetNameSpace && self.GetElementCount == other.GetElementCount && self.AddElement == other.AddElement && self.SetElement == other.SetElement && self.SetElementAt == other.SetElementAt && self.GetElement == other.GetElement && self.GetElementAt == other.GetElementAt && self.ClearElement == other.ClearElement && self.ClearElementAt == other.ClearElementAt && self.GetServerName == other.GetServerName && self.SetServerName == other.SetServerName && self.GetClass == other.GetClass
    }
}
impl ::core::cmp::Eq for MI_InstanceFT {}
impl ::core::default::Default for MI_InstanceFT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
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
impl ::core::fmt::Debug for MI_InstanceField {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_InstanceField").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
impl ::windows::core::TypeKind for MI_InstanceField {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MI_InstanceField {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.exists == other.exists && self.flags == other.flags
    }
}
impl ::core::cmp::Eq for MI_InstanceField {}
impl ::core::default::Default for MI_InstanceField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
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
impl ::core::fmt::Debug for MI_Interval {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_Interval").field("days", &self.days).field("hours", &self.hours).field("minutes", &self.minutes).field("seconds", &self.seconds).field("microseconds", &self.microseconds).field("__padding1", &self.__padding1).field("__padding2", &self.__padding2).field("__padding3", &self.__padding3).finish()
    }
}
impl ::windows::core::TypeKind for MI_Interval {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MI_Interval {
    fn eq(&self, other: &Self) -> bool {
        self.days == other.days && self.hours == other.hours && self.minutes == other.minutes && self.seconds == other.seconds && self.microseconds == other.microseconds && self.__padding1 == other.__padding1 && self.__padding2 == other.__padding2 && self.__padding3 == other.__padding3
    }
}
impl ::core::cmp::Eq for MI_Interval {}
impl ::core::default::Default for MI_Interval {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_MethodDecl {
    pub flags: u32,
    pub code: u32,
    pub name: *const u16,
    pub qualifiers: *const *const MI_Qualifier,
    pub numQualifiers: u32,
    pub parameters: *const *const MI_ParameterDecl,
    pub numParameters: u32,
    pub size: u32,
    pub returnType: u32,
    pub origin: *const u16,
    pub propagator: *const u16,
    pub schema: *const MI_SchemaDecl,
    pub function: MI_MethodDecl_Invoke,
}
impl ::core::marker::Copy for MI_MethodDecl {}
impl ::core::clone::Clone for MI_MethodDecl {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_MethodDecl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_MethodDecl").field("flags", &self.flags).field("code", &self.code).field("name", &self.name).field("qualifiers", &self.qualifiers).field("numQualifiers", &self.numQualifiers).field("parameters", &self.parameters).field("numParameters", &self.numParameters).field("size", &self.size).field("returnType", &self.returnType).field("origin", &self.origin).field("propagator", &self.propagator).field("schema", &self.schema).finish()
    }
}
impl ::windows::core::TypeKind for MI_MethodDecl {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for MI_MethodDecl {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_Module {
    pub version: u32,
    pub generatorVersion: u32,
    pub flags: u32,
    pub charSize: u32,
    pub schemaDecl: *mut MI_SchemaDecl,
    pub Load: MI_Module_Load,
    pub Unload: MI_Module_Unload,
    pub dynamicProviderFT: *const MI_ProviderFT,
}
impl ::core::marker::Copy for MI_Module {}
impl ::core::clone::Clone for MI_Module {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_Module {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_Module").field("version", &self.version).field("generatorVersion", &self.generatorVersion).field("flags", &self.flags).field("charSize", &self.charSize).field("schemaDecl", &self.schemaDecl).field("dynamicProviderFT", &self.dynamicProviderFT).finish()
    }
}
impl ::windows::core::TypeKind for MI_Module {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for MI_Module {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MI_Module_Self(pub u8);
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_ObjectDecl {
    pub flags: u32,
    pub code: u32,
    pub name: *const u16,
    pub qualifiers: *const *const MI_Qualifier,
    pub numQualifiers: u32,
    pub properties: *const *const MI_PropertyDecl,
    pub numProperties: u32,
    pub size: u32,
}
impl ::core::marker::Copy for MI_ObjectDecl {}
impl ::core::clone::Clone for MI_ObjectDecl {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_ObjectDecl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ObjectDecl").field("flags", &self.flags).field("code", &self.code).field("name", &self.name).field("qualifiers", &self.qualifiers).field("numQualifiers", &self.numQualifiers).field("properties", &self.properties).field("numProperties", &self.numProperties).field("size", &self.size).finish()
    }
}
impl ::windows::core::TypeKind for MI_ObjectDecl {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MI_ObjectDecl {
    fn eq(&self, other: &Self) -> bool {
        self.flags == other.flags && self.code == other.code && self.name == other.name && self.qualifiers == other.qualifiers && self.numQualifiers == other.numQualifiers && self.properties == other.properties && self.numProperties == other.numProperties && self.size == other.size
    }
}
impl ::core::cmp::Eq for MI_ObjectDecl {}
impl ::core::default::Default for MI_ObjectDecl {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_Operation {
    pub reserved1: u64,
    pub reserved2: isize,
    pub ft: *const MI_OperationFT,
}
impl ::core::marker::Copy for MI_Operation {}
impl ::core::clone::Clone for MI_Operation {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_Operation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_Operation").field("reserved1", &self.reserved1).field("reserved2", &self.reserved2).field("ft", &self.ft).finish()
    }
}
impl ::windows::core::TypeKind for MI_Operation {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MI_Operation {
    fn eq(&self, other: &Self) -> bool {
        self.reserved1 == other.reserved1 && self.reserved2 == other.reserved2 && self.ft == other.ft
    }
}
impl ::core::cmp::Eq for MI_Operation {}
impl ::core::default::Default for MI_Operation {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
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
impl ::core::fmt::Debug for MI_OperationCallbacks {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_OperationCallbacks").field("callbackContext", &self.callbackContext).finish()
    }
}
impl ::windows::core::TypeKind for MI_OperationCallbacks {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for MI_OperationCallbacks {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
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
impl ::core::fmt::Debug for MI_OperationFT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_OperationFT").field("Close", &self.Close).field("Cancel", &self.Cancel).field("GetSession", &self.GetSession).field("GetInstance", &self.GetInstance).field("GetIndication", &self.GetIndication).field("GetClass", &self.GetClass).finish()
    }
}
impl ::windows::core::TypeKind for MI_OperationFT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MI_OperationFT {
    fn eq(&self, other: &Self) -> bool {
        self.Close == other.Close && self.Cancel == other.Cancel && self.GetSession == other.GetSession && self.GetInstance == other.GetInstance && self.GetIndication == other.GetIndication && self.GetClass == other.GetClass
    }
}
impl ::core::cmp::Eq for MI_OperationFT {}
impl ::core::default::Default for MI_OperationFT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_OperationOptions {
    pub reserved1: u64,
    pub reserved2: isize,
    pub ft: *const MI_OperationOptionsFT,
}
impl ::core::marker::Copy for MI_OperationOptions {}
impl ::core::clone::Clone for MI_OperationOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_OperationOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_OperationOptions").field("reserved1", &self.reserved1).field("reserved2", &self.reserved2).field("ft", &self.ft).finish()
    }
}
impl ::windows::core::TypeKind for MI_OperationOptions {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MI_OperationOptions {
    fn eq(&self, other: &Self) -> bool {
        self.reserved1 == other.reserved1 && self.reserved2 == other.reserved2 && self.ft == other.ft
    }
}
impl ::core::cmp::Eq for MI_OperationOptions {}
impl ::core::default::Default for MI_OperationOptions {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
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
impl ::core::fmt::Debug for MI_OperationOptionsFT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_OperationOptionsFT")
            .field("Delete", &self.Delete)
            .field("SetString", &self.SetString)
            .field("SetNumber", &self.SetNumber)
            .field("SetCustomOption", &self.SetCustomOption)
            .field("GetString", &self.GetString)
            .field("GetNumber", &self.GetNumber)
            .field("GetOptionCount", &self.GetOptionCount)
            .field("GetOptionAt", &self.GetOptionAt)
            .field("GetOption", &self.GetOption)
            .field("GetEnabledChannels", &self.GetEnabledChannels)
            .field("Clone", &self.Clone)
            .field("SetInterval", &self.SetInterval)
            .field("GetInterval", &self.GetInterval)
            .finish()
    }
}
impl ::windows::core::TypeKind for MI_OperationOptionsFT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MI_OperationOptionsFT {
    fn eq(&self, other: &Self) -> bool {
        self.Delete == other.Delete && self.SetString == other.SetString && self.SetNumber == other.SetNumber && self.SetCustomOption == other.SetCustomOption && self.GetString == other.GetString && self.GetNumber == other.GetNumber && self.GetOptionCount == other.GetOptionCount && self.GetOptionAt == other.GetOptionAt && self.GetOption == other.GetOption && self.GetEnabledChannels == other.GetEnabledChannels && self.Clone == other.Clone && self.SetInterval == other.SetInterval && self.GetInterval == other.GetInterval
    }
}
impl ::core::cmp::Eq for MI_OperationOptionsFT {}
impl ::core::default::Default for MI_OperationOptionsFT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_ParameterDecl {
    pub flags: u32,
    pub code: u32,
    pub name: *const u16,
    pub qualifiers: *const *const MI_Qualifier,
    pub numQualifiers: u32,
    pub r#type: u32,
    pub className: *const u16,
    pub subscript: u32,
    pub offset: u32,
}
impl ::core::marker::Copy for MI_ParameterDecl {}
impl ::core::clone::Clone for MI_ParameterDecl {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_ParameterDecl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ParameterDecl").field("flags", &self.flags).field("code", &self.code).field("name", &self.name).field("qualifiers", &self.qualifiers).field("numQualifiers", &self.numQualifiers).field("type", &self.r#type).field("className", &self.className).field("subscript", &self.subscript).field("offset", &self.offset).finish()
    }
}
impl ::windows::core::TypeKind for MI_ParameterDecl {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MI_ParameterDecl {
    fn eq(&self, other: &Self) -> bool {
        self.flags == other.flags && self.code == other.code && self.name == other.name && self.qualifiers == other.qualifiers && self.numQualifiers == other.numQualifiers && self.r#type == other.r#type && self.className == other.className && self.subscript == other.subscript && self.offset == other.offset
    }
}
impl ::core::cmp::Eq for MI_ParameterDecl {}
impl ::core::default::Default for MI_ParameterDecl {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_ParameterSet {
    pub reserved1: u64,
    pub reserved2: isize,
    pub ft: *const MI_ParameterSetFT,
}
impl ::core::marker::Copy for MI_ParameterSet {}
impl ::core::clone::Clone for MI_ParameterSet {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_ParameterSet {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ParameterSet").field("reserved1", &self.reserved1).field("reserved2", &self.reserved2).field("ft", &self.ft).finish()
    }
}
impl ::windows::core::TypeKind for MI_ParameterSet {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MI_ParameterSet {
    fn eq(&self, other: &Self) -> bool {
        self.reserved1 == other.reserved1 && self.reserved2 == other.reserved2 && self.ft == other.ft
    }
}
impl ::core::cmp::Eq for MI_ParameterSet {}
impl ::core::default::Default for MI_ParameterSet {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
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
impl ::core::fmt::Debug for MI_ParameterSetFT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ParameterSetFT").field("GetMethodReturnType", &self.GetMethodReturnType).field("GetParameterCount", &self.GetParameterCount).field("GetParameterAt", &self.GetParameterAt).field("GetParameter", &self.GetParameter).finish()
    }
}
impl ::windows::core::TypeKind for MI_ParameterSetFT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MI_ParameterSetFT {
    fn eq(&self, other: &Self) -> bool {
        self.GetMethodReturnType == other.GetMethodReturnType && self.GetParameterCount == other.GetParameterCount && self.GetParameterAt == other.GetParameterAt && self.GetParameter == other.GetParameter
    }
}
impl ::core::cmp::Eq for MI_ParameterSetFT {}
impl ::core::default::Default for MI_ParameterSetFT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_PropertyDecl {
    pub flags: u32,
    pub code: u32,
    pub name: *const u16,
    pub qualifiers: *const *const MI_Qualifier,
    pub numQualifiers: u32,
    pub r#type: u32,
    pub className: *const u16,
    pub subscript: u32,
    pub offset: u32,
    pub origin: *const u16,
    pub propagator: *const u16,
    pub value: *const ::core::ffi::c_void,
}
impl ::core::marker::Copy for MI_PropertyDecl {}
impl ::core::clone::Clone for MI_PropertyDecl {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_PropertyDecl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_PropertyDecl").field("flags", &self.flags).field("code", &self.code).field("name", &self.name).field("qualifiers", &self.qualifiers).field("numQualifiers", &self.numQualifiers).field("type", &self.r#type).field("className", &self.className).field("subscript", &self.subscript).field("offset", &self.offset).field("origin", &self.origin).field("propagator", &self.propagator).field("value", &self.value).finish()
    }
}
impl ::windows::core::TypeKind for MI_PropertyDecl {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MI_PropertyDecl {
    fn eq(&self, other: &Self) -> bool {
        self.flags == other.flags && self.code == other.code && self.name == other.name && self.qualifiers == other.qualifiers && self.numQualifiers == other.numQualifiers && self.r#type == other.r#type && self.className == other.className && self.subscript == other.subscript && self.offset == other.offset && self.origin == other.origin && self.propagator == other.propagator && self.value == other.value
    }
}
impl ::core::cmp::Eq for MI_PropertyDecl {}
impl ::core::default::Default for MI_PropertyDecl {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_PropertySet {
    pub ft: *const MI_PropertySetFT,
    pub reserved: [isize; 3],
}
impl ::core::marker::Copy for MI_PropertySet {}
impl ::core::clone::Clone for MI_PropertySet {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_PropertySet {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_PropertySet").field("ft", &self.ft).field("reserved", &self.reserved).finish()
    }
}
impl ::windows::core::TypeKind for MI_PropertySet {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MI_PropertySet {
    fn eq(&self, other: &Self) -> bool {
        self.ft == other.ft && self.reserved == other.reserved
    }
}
impl ::core::cmp::Eq for MI_PropertySet {}
impl ::core::default::Default for MI_PropertySet {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
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
impl ::core::fmt::Debug for MI_PropertySetFT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_PropertySetFT").field("GetElementCount", &self.GetElementCount).field("ContainsElement", &self.ContainsElement).field("AddElement", &self.AddElement).field("GetElementAt", &self.GetElementAt).field("Clear", &self.Clear).field("Destruct", &self.Destruct).field("Delete", &self.Delete).field("Clone", &self.Clone).finish()
    }
}
impl ::windows::core::TypeKind for MI_PropertySetFT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MI_PropertySetFT {
    fn eq(&self, other: &Self) -> bool {
        self.GetElementCount == other.GetElementCount && self.ContainsElement == other.ContainsElement && self.AddElement == other.AddElement && self.GetElementAt == other.GetElementAt && self.Clear == other.Clear && self.Destruct == other.Destruct && self.Delete == other.Delete && self.Clone == other.Clone
    }
}
impl ::core::cmp::Eq for MI_PropertySetFT {}
impl ::core::default::Default for MI_PropertySetFT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
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
impl ::core::fmt::Debug for MI_ProviderFT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ProviderFT").finish()
    }
}
impl ::windows::core::TypeKind for MI_ProviderFT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for MI_ProviderFT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_Qualifier {
    pub name: *const u16,
    pub r#type: u32,
    pub flavor: u32,
    pub value: *const ::core::ffi::c_void,
}
impl ::core::marker::Copy for MI_Qualifier {}
impl ::core::clone::Clone for MI_Qualifier {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_Qualifier {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_Qualifier").field("name", &self.name).field("type", &self.r#type).field("flavor", &self.flavor).field("value", &self.value).finish()
    }
}
impl ::windows::core::TypeKind for MI_Qualifier {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MI_Qualifier {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.r#type == other.r#type && self.flavor == other.flavor && self.value == other.value
    }
}
impl ::core::cmp::Eq for MI_Qualifier {}
impl ::core::default::Default for MI_Qualifier {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_QualifierDecl {
    pub name: *const u16,
    pub r#type: u32,
    pub scope: u32,
    pub flavor: u32,
    pub subscript: u32,
    pub value: *const ::core::ffi::c_void,
}
impl ::core::marker::Copy for MI_QualifierDecl {}
impl ::core::clone::Clone for MI_QualifierDecl {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_QualifierDecl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_QualifierDecl").field("name", &self.name).field("type", &self.r#type).field("scope", &self.scope).field("flavor", &self.flavor).field("subscript", &self.subscript).field("value", &self.value).finish()
    }
}
impl ::windows::core::TypeKind for MI_QualifierDecl {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MI_QualifierDecl {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.r#type == other.r#type && self.scope == other.scope && self.flavor == other.flavor && self.subscript == other.subscript && self.value == other.value
    }
}
impl ::core::cmp::Eq for MI_QualifierDecl {}
impl ::core::default::Default for MI_QualifierDecl {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_QualifierSet {
    pub reserved1: u64,
    pub reserved2: isize,
    pub ft: *const MI_QualifierSetFT,
}
impl ::core::marker::Copy for MI_QualifierSet {}
impl ::core::clone::Clone for MI_QualifierSet {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_QualifierSet {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_QualifierSet").field("reserved1", &self.reserved1).field("reserved2", &self.reserved2).field("ft", &self.ft).finish()
    }
}
impl ::windows::core::TypeKind for MI_QualifierSet {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MI_QualifierSet {
    fn eq(&self, other: &Self) -> bool {
        self.reserved1 == other.reserved1 && self.reserved2 == other.reserved2 && self.ft == other.ft
    }
}
impl ::core::cmp::Eq for MI_QualifierSet {}
impl ::core::default::Default for MI_QualifierSet {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
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
impl ::core::fmt::Debug for MI_QualifierSetFT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_QualifierSetFT").field("GetQualifierCount", &self.GetQualifierCount).field("GetQualifierAt", &self.GetQualifierAt).field("GetQualifier", &self.GetQualifier).finish()
    }
}
impl ::windows::core::TypeKind for MI_QualifierSetFT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MI_QualifierSetFT {
    fn eq(&self, other: &Self) -> bool {
        self.GetQualifierCount == other.GetQualifierCount && self.GetQualifierAt == other.GetQualifierAt && self.GetQualifier == other.GetQualifier
    }
}
impl ::core::cmp::Eq for MI_QualifierSetFT {}
impl ::core::default::Default for MI_QualifierSetFT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
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
impl ::core::fmt::Debug for MI_Real32A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_Real32A").field("data", &self.data).field("size", &self.size).finish()
    }
}
impl ::windows::core::TypeKind for MI_Real32A {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MI_Real32A {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data && self.size == other.size
    }
}
impl ::core::cmp::Eq for MI_Real32A {}
impl ::core::default::Default for MI_Real32A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
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
impl ::core::fmt::Debug for MI_Real32AField {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_Real32AField").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
impl ::windows::core::TypeKind for MI_Real32AField {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MI_Real32AField {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.exists == other.exists && self.flags == other.flags
    }
}
impl ::core::cmp::Eq for MI_Real32AField {}
impl ::core::default::Default for MI_Real32AField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
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
impl ::core::fmt::Debug for MI_Real32Field {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_Real32Field").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
impl ::windows::core::TypeKind for MI_Real32Field {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MI_Real32Field {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.exists == other.exists && self.flags == other.flags
    }
}
impl ::core::cmp::Eq for MI_Real32Field {}
impl ::core::default::Default for MI_Real32Field {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
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
impl ::core::fmt::Debug for MI_Real64A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_Real64A").field("data", &self.data).field("size", &self.size).finish()
    }
}
impl ::windows::core::TypeKind for MI_Real64A {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MI_Real64A {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data && self.size == other.size
    }
}
impl ::core::cmp::Eq for MI_Real64A {}
impl ::core::default::Default for MI_Real64A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
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
impl ::core::fmt::Debug for MI_Real64AField {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_Real64AField").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
impl ::windows::core::TypeKind for MI_Real64AField {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MI_Real64AField {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.exists == other.exists && self.flags == other.flags
    }
}
impl ::core::cmp::Eq for MI_Real64AField {}
impl ::core::default::Default for MI_Real64AField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
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
impl ::core::fmt::Debug for MI_Real64Field {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_Real64Field").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
impl ::windows::core::TypeKind for MI_Real64Field {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MI_Real64Field {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.exists == other.exists && self.flags == other.flags
    }
}
impl ::core::cmp::Eq for MI_Real64Field {}
impl ::core::default::Default for MI_Real64Field {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
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
impl ::core::fmt::Debug for MI_ReferenceA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ReferenceA").field("data", &self.data).field("size", &self.size).finish()
    }
}
impl ::windows::core::TypeKind for MI_ReferenceA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MI_ReferenceA {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data && self.size == other.size
    }
}
impl ::core::cmp::Eq for MI_ReferenceA {}
impl ::core::default::Default for MI_ReferenceA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
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
impl ::core::fmt::Debug for MI_ReferenceAField {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ReferenceAField").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
impl ::windows::core::TypeKind for MI_ReferenceAField {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MI_ReferenceAField {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.exists == other.exists && self.flags == other.flags
    }
}
impl ::core::cmp::Eq for MI_ReferenceAField {}
impl ::core::default::Default for MI_ReferenceAField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
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
impl ::core::fmt::Debug for MI_ReferenceField {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ReferenceField").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
impl ::windows::core::TypeKind for MI_ReferenceField {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MI_ReferenceField {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.exists == other.exists && self.flags == other.flags
    }
}
impl ::core::cmp::Eq for MI_ReferenceField {}
impl ::core::default::Default for MI_ReferenceField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_SchemaDecl {
    pub qualifierDecls: *const *const MI_QualifierDecl,
    pub numQualifierDecls: u32,
    pub classDecls: *const *const MI_ClassDecl,
    pub numClassDecls: u32,
}
impl ::core::marker::Copy for MI_SchemaDecl {}
impl ::core::clone::Clone for MI_SchemaDecl {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_SchemaDecl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_SchemaDecl").field("qualifierDecls", &self.qualifierDecls).field("numQualifierDecls", &self.numQualifierDecls).field("classDecls", &self.classDecls).field("numClassDecls", &self.numClassDecls).finish()
    }
}
impl ::windows::core::TypeKind for MI_SchemaDecl {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MI_SchemaDecl {
    fn eq(&self, other: &Self) -> bool {
        self.qualifierDecls == other.qualifierDecls && self.numQualifierDecls == other.numQualifierDecls && self.classDecls == other.classDecls && self.numClassDecls == other.numClassDecls
    }
}
impl ::core::cmp::Eq for MI_SchemaDecl {}
impl ::core::default::Default for MI_SchemaDecl {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
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
impl ::core::fmt::Debug for MI_Serializer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_Serializer").field("reserved1", &self.reserved1).field("reserved2", &self.reserved2).finish()
    }
}
impl ::windows::core::TypeKind for MI_Serializer {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MI_Serializer {
    fn eq(&self, other: &Self) -> bool {
        self.reserved1 == other.reserved1 && self.reserved2 == other.reserved2
    }
}
impl ::core::cmp::Eq for MI_Serializer {}
impl ::core::default::Default for MI_Serializer {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
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
impl ::core::fmt::Debug for MI_SerializerFT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_SerializerFT").field("Close", &self.Close).field("SerializeClass", &self.SerializeClass).field("SerializeInstance", &self.SerializeInstance).finish()
    }
}
impl ::windows::core::TypeKind for MI_SerializerFT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MI_SerializerFT {
    fn eq(&self, other: &Self) -> bool {
        self.Close == other.Close && self.SerializeClass == other.SerializeClass && self.SerializeInstance == other.SerializeInstance
    }
}
impl ::core::cmp::Eq for MI_SerializerFT {}
impl ::core::default::Default for MI_SerializerFT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_Server {
    pub serverFT: *const MI_ServerFT,
    pub contextFT: *const MI_ContextFT,
    pub instanceFT: *const MI_InstanceFT,
    pub propertySetFT: *const MI_PropertySetFT,
    pub filterFT: *const MI_FilterFT,
}
impl ::core::marker::Copy for MI_Server {}
impl ::core::clone::Clone for MI_Server {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_Server {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_Server").field("serverFT", &self.serverFT).field("contextFT", &self.contextFT).field("instanceFT", &self.instanceFT).field("propertySetFT", &self.propertySetFT).field("filterFT", &self.filterFT).finish()
    }
}
impl ::windows::core::TypeKind for MI_Server {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MI_Server {
    fn eq(&self, other: &Self) -> bool {
        self.serverFT == other.serverFT && self.contextFT == other.contextFT && self.instanceFT == other.instanceFT && self.propertySetFT == other.propertySetFT && self.filterFT == other.filterFT
    }
}
impl ::core::cmp::Eq for MI_Server {}
impl ::core::default::Default for MI_Server {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
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
impl ::core::fmt::Debug for MI_ServerFT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ServerFT").field("GetVersion", &self.GetVersion).field("GetSystemName", &self.GetSystemName).finish()
    }
}
impl ::windows::core::TypeKind for MI_ServerFT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MI_ServerFT {
    fn eq(&self, other: &Self) -> bool {
        self.GetVersion == other.GetVersion && self.GetSystemName == other.GetSystemName
    }
}
impl ::core::cmp::Eq for MI_ServerFT {}
impl ::core::default::Default for MI_ServerFT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_Session {
    pub reserved1: u64,
    pub reserved2: isize,
    pub ft: *const MI_SessionFT,
}
impl ::core::marker::Copy for MI_Session {}
impl ::core::clone::Clone for MI_Session {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_Session {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_Session").field("reserved1", &self.reserved1).field("reserved2", &self.reserved2).field("ft", &self.ft).finish()
    }
}
impl ::windows::core::TypeKind for MI_Session {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MI_Session {
    fn eq(&self, other: &Self) -> bool {
        self.reserved1 == other.reserved1 && self.reserved2 == other.reserved2 && self.ft == other.ft
    }
}
impl ::core::cmp::Eq for MI_Session {}
impl ::core::default::Default for MI_Session {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
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
impl ::core::fmt::Debug for MI_SessionCallbacks {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_SessionCallbacks").field("callbackContext", &self.callbackContext).field("writeMessage", &self.writeMessage).field("writeError", &self.writeError).finish()
    }
}
impl ::windows::core::TypeKind for MI_SessionCallbacks {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MI_SessionCallbacks {
    fn eq(&self, other: &Self) -> bool {
        self.callbackContext == other.callbackContext && self.writeMessage == other.writeMessage && self.writeError == other.writeError
    }
}
impl ::core::cmp::Eq for MI_SessionCallbacks {}
impl ::core::default::Default for MI_SessionCallbacks {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
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
impl ::core::fmt::Debug for MI_SessionFT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_SessionFT")
            .field("Close", &self.Close)
            .field("GetApplication", &self.GetApplication)
            .field("GetInstance", &self.GetInstance)
            .field("ModifyInstance", &self.ModifyInstance)
            .field("CreateInstance", &self.CreateInstance)
            .field("DeleteInstance", &self.DeleteInstance)
            .field("Invoke", &self.Invoke)
            .field("EnumerateInstances", &self.EnumerateInstances)
            .field("QueryInstances", &self.QueryInstances)
            .field("AssociatorInstances", &self.AssociatorInstances)
            .field("ReferenceInstances", &self.ReferenceInstances)
            .field("Subscribe", &self.Subscribe)
            .field("GetClass", &self.GetClass)
            .field("EnumerateClasses", &self.EnumerateClasses)
            .field("TestConnection", &self.TestConnection)
            .finish()
    }
}
impl ::windows::core::TypeKind for MI_SessionFT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MI_SessionFT {
    fn eq(&self, other: &Self) -> bool {
        self.Close == other.Close && self.GetApplication == other.GetApplication && self.GetInstance == other.GetInstance && self.ModifyInstance == other.ModifyInstance && self.CreateInstance == other.CreateInstance && self.DeleteInstance == other.DeleteInstance && self.Invoke == other.Invoke && self.EnumerateInstances == other.EnumerateInstances && self.QueryInstances == other.QueryInstances && self.AssociatorInstances == other.AssociatorInstances && self.ReferenceInstances == other.ReferenceInstances && self.Subscribe == other.Subscribe && self.GetClass == other.GetClass && self.EnumerateClasses == other.EnumerateClasses && self.TestConnection == other.TestConnection
    }
}
impl ::core::cmp::Eq for MI_SessionFT {}
impl ::core::default::Default for MI_SessionFT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
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
impl ::core::fmt::Debug for MI_Sint16A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_Sint16A").field("data", &self.data).field("size", &self.size).finish()
    }
}
impl ::windows::core::TypeKind for MI_Sint16A {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MI_Sint16A {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data && self.size == other.size
    }
}
impl ::core::cmp::Eq for MI_Sint16A {}
impl ::core::default::Default for MI_Sint16A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
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
impl ::core::fmt::Debug for MI_Sint16AField {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_Sint16AField").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
impl ::windows::core::TypeKind for MI_Sint16AField {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MI_Sint16AField {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.exists == other.exists && self.flags == other.flags
    }
}
impl ::core::cmp::Eq for MI_Sint16AField {}
impl ::core::default::Default for MI_Sint16AField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
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
impl ::core::fmt::Debug for MI_Sint16Field {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_Sint16Field").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
impl ::windows::core::TypeKind for MI_Sint16Field {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MI_Sint16Field {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.exists == other.exists && self.flags == other.flags
    }
}
impl ::core::cmp::Eq for MI_Sint16Field {}
impl ::core::default::Default for MI_Sint16Field {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
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
impl ::core::fmt::Debug for MI_Sint32A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_Sint32A").field("data", &self.data).field("size", &self.size).finish()
    }
}
impl ::windows::core::TypeKind for MI_Sint32A {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MI_Sint32A {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data && self.size == other.size
    }
}
impl ::core::cmp::Eq for MI_Sint32A {}
impl ::core::default::Default for MI_Sint32A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
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
impl ::core::fmt::Debug for MI_Sint32AField {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_Sint32AField").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
impl ::windows::core::TypeKind for MI_Sint32AField {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MI_Sint32AField {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.exists == other.exists && self.flags == other.flags
    }
}
impl ::core::cmp::Eq for MI_Sint32AField {}
impl ::core::default::Default for MI_Sint32AField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
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
impl ::core::fmt::Debug for MI_Sint32Field {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_Sint32Field").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
impl ::windows::core::TypeKind for MI_Sint32Field {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MI_Sint32Field {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.exists == other.exists && self.flags == other.flags
    }
}
impl ::core::cmp::Eq for MI_Sint32Field {}
impl ::core::default::Default for MI_Sint32Field {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
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
impl ::core::fmt::Debug for MI_Sint64A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_Sint64A").field("data", &self.data).field("size", &self.size).finish()
    }
}
impl ::windows::core::TypeKind for MI_Sint64A {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MI_Sint64A {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data && self.size == other.size
    }
}
impl ::core::cmp::Eq for MI_Sint64A {}
impl ::core::default::Default for MI_Sint64A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
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
impl ::core::fmt::Debug for MI_Sint64AField {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_Sint64AField").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
impl ::windows::core::TypeKind for MI_Sint64AField {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MI_Sint64AField {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.exists == other.exists && self.flags == other.flags
    }
}
impl ::core::cmp::Eq for MI_Sint64AField {}
impl ::core::default::Default for MI_Sint64AField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
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
impl ::core::fmt::Debug for MI_Sint64Field {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_Sint64Field").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
impl ::windows::core::TypeKind for MI_Sint64Field {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MI_Sint64Field {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.exists == other.exists && self.flags == other.flags
    }
}
impl ::core::cmp::Eq for MI_Sint64Field {}
impl ::core::default::Default for MI_Sint64Field {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
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
impl ::core::fmt::Debug for MI_Sint8A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_Sint8A").field("data", &self.data).field("size", &self.size).finish()
    }
}
impl ::windows::core::TypeKind for MI_Sint8A {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MI_Sint8A {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data && self.size == other.size
    }
}
impl ::core::cmp::Eq for MI_Sint8A {}
impl ::core::default::Default for MI_Sint8A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
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
impl ::core::fmt::Debug for MI_Sint8AField {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_Sint8AField").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
impl ::windows::core::TypeKind for MI_Sint8AField {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MI_Sint8AField {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.exists == other.exists && self.flags == other.flags
    }
}
impl ::core::cmp::Eq for MI_Sint8AField {}
impl ::core::default::Default for MI_Sint8AField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
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
impl ::core::fmt::Debug for MI_Sint8Field {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_Sint8Field").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
impl ::windows::core::TypeKind for MI_Sint8Field {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MI_Sint8Field {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.exists == other.exists && self.flags == other.flags
    }
}
impl ::core::cmp::Eq for MI_Sint8Field {}
impl ::core::default::Default for MI_Sint8Field {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
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
impl ::core::fmt::Debug for MI_StringA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_StringA").field("data", &self.data).field("size", &self.size).finish()
    }
}
impl ::windows::core::TypeKind for MI_StringA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MI_StringA {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data && self.size == other.size
    }
}
impl ::core::cmp::Eq for MI_StringA {}
impl ::core::default::Default for MI_StringA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
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
impl ::core::fmt::Debug for MI_StringAField {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_StringAField").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
impl ::windows::core::TypeKind for MI_StringAField {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MI_StringAField {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.exists == other.exists && self.flags == other.flags
    }
}
impl ::core::cmp::Eq for MI_StringAField {}
impl ::core::default::Default for MI_StringAField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
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
impl ::core::fmt::Debug for MI_StringField {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_StringField").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
impl ::windows::core::TypeKind for MI_StringField {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MI_StringField {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.exists == other.exists && self.flags == other.flags
    }
}
impl ::core::cmp::Eq for MI_StringField {}
impl ::core::default::Default for MI_StringField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_SubscriptionDeliveryOptions {
    pub reserved1: u64,
    pub reserved2: isize,
    pub ft: *const MI_SubscriptionDeliveryOptionsFT,
}
impl ::core::marker::Copy for MI_SubscriptionDeliveryOptions {}
impl ::core::clone::Clone for MI_SubscriptionDeliveryOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_SubscriptionDeliveryOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_SubscriptionDeliveryOptions").field("reserved1", &self.reserved1).field("reserved2", &self.reserved2).field("ft", &self.ft).finish()
    }
}
impl ::windows::core::TypeKind for MI_SubscriptionDeliveryOptions {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MI_SubscriptionDeliveryOptions {
    fn eq(&self, other: &Self) -> bool {
        self.reserved1 == other.reserved1 && self.reserved2 == other.reserved2 && self.ft == other.ft
    }
}
impl ::core::cmp::Eq for MI_SubscriptionDeliveryOptions {}
impl ::core::default::Default for MI_SubscriptionDeliveryOptions {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
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
impl ::core::fmt::Debug for MI_SubscriptionDeliveryOptionsFT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_SubscriptionDeliveryOptionsFT")
            .field("SetString", &self.SetString)
            .field("SetNumber", &self.SetNumber)
            .field("SetDateTime", &self.SetDateTime)
            .field("SetInterval", &self.SetInterval)
            .field("AddCredentials", &self.AddCredentials)
            .field("Delete", &self.Delete)
            .field("GetString", &self.GetString)
            .field("GetNumber", &self.GetNumber)
            .field("GetDateTime", &self.GetDateTime)
            .field("GetInterval", &self.GetInterval)
            .field("GetOptionCount", &self.GetOptionCount)
            .field("GetOptionAt", &self.GetOptionAt)
            .field("GetOption", &self.GetOption)
            .field("GetCredentialsCount", &self.GetCredentialsCount)
            .field("GetCredentialsAt", &self.GetCredentialsAt)
            .field("GetCredentialsPasswordAt", &self.GetCredentialsPasswordAt)
            .field("Clone", &self.Clone)
            .finish()
    }
}
impl ::windows::core::TypeKind for MI_SubscriptionDeliveryOptionsFT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MI_SubscriptionDeliveryOptionsFT {
    fn eq(&self, other: &Self) -> bool {
        self.SetString == other.SetString && self.SetNumber == other.SetNumber && self.SetDateTime == other.SetDateTime && self.SetInterval == other.SetInterval && self.AddCredentials == other.AddCredentials && self.Delete == other.Delete && self.GetString == other.GetString && self.GetNumber == other.GetNumber && self.GetDateTime == other.GetDateTime && self.GetInterval == other.GetInterval && self.GetOptionCount == other.GetOptionCount && self.GetOptionAt == other.GetOptionAt && self.GetOption == other.GetOption && self.GetCredentialsCount == other.GetCredentialsCount && self.GetCredentialsAt == other.GetCredentialsAt && self.GetCredentialsPasswordAt == other.GetCredentialsPasswordAt && self.Clone == other.Clone
    }
}
impl ::core::cmp::Eq for MI_SubscriptionDeliveryOptionsFT {}
impl ::core::default::Default for MI_SubscriptionDeliveryOptionsFT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
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
impl ::core::fmt::Debug for MI_Timestamp {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_Timestamp").field("year", &self.year).field("month", &self.month).field("day", &self.day).field("hour", &self.hour).field("minute", &self.minute).field("second", &self.second).field("microseconds", &self.microseconds).field("utc", &self.utc).finish()
    }
}
impl ::windows::core::TypeKind for MI_Timestamp {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MI_Timestamp {
    fn eq(&self, other: &Self) -> bool {
        self.year == other.year && self.month == other.month && self.day == other.day && self.hour == other.hour && self.minute == other.minute && self.second == other.second && self.microseconds == other.microseconds && self.utc == other.utc
    }
}
impl ::core::cmp::Eq for MI_Timestamp {}
impl ::core::default::Default for MI_Timestamp {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
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
impl ::core::fmt::Debug for MI_Uint16A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_Uint16A").field("data", &self.data).field("size", &self.size).finish()
    }
}
impl ::windows::core::TypeKind for MI_Uint16A {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MI_Uint16A {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data && self.size == other.size
    }
}
impl ::core::cmp::Eq for MI_Uint16A {}
impl ::core::default::Default for MI_Uint16A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
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
impl ::core::fmt::Debug for MI_Uint16AField {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_Uint16AField").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
impl ::windows::core::TypeKind for MI_Uint16AField {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MI_Uint16AField {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.exists == other.exists && self.flags == other.flags
    }
}
impl ::core::cmp::Eq for MI_Uint16AField {}
impl ::core::default::Default for MI_Uint16AField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
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
impl ::core::fmt::Debug for MI_Uint16Field {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_Uint16Field").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
impl ::windows::core::TypeKind for MI_Uint16Field {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MI_Uint16Field {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.exists == other.exists && self.flags == other.flags
    }
}
impl ::core::cmp::Eq for MI_Uint16Field {}
impl ::core::default::Default for MI_Uint16Field {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
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
impl ::core::fmt::Debug for MI_Uint32A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_Uint32A").field("data", &self.data).field("size", &self.size).finish()
    }
}
impl ::windows::core::TypeKind for MI_Uint32A {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MI_Uint32A {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data && self.size == other.size
    }
}
impl ::core::cmp::Eq for MI_Uint32A {}
impl ::core::default::Default for MI_Uint32A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
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
impl ::core::fmt::Debug for MI_Uint32AField {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_Uint32AField").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
impl ::windows::core::TypeKind for MI_Uint32AField {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MI_Uint32AField {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.exists == other.exists && self.flags == other.flags
    }
}
impl ::core::cmp::Eq for MI_Uint32AField {}
impl ::core::default::Default for MI_Uint32AField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
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
impl ::core::fmt::Debug for MI_Uint32Field {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_Uint32Field").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
impl ::windows::core::TypeKind for MI_Uint32Field {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MI_Uint32Field {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.exists == other.exists && self.flags == other.flags
    }
}
impl ::core::cmp::Eq for MI_Uint32Field {}
impl ::core::default::Default for MI_Uint32Field {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
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
impl ::core::fmt::Debug for MI_Uint64A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_Uint64A").field("data", &self.data).field("size", &self.size).finish()
    }
}
impl ::windows::core::TypeKind for MI_Uint64A {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MI_Uint64A {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data && self.size == other.size
    }
}
impl ::core::cmp::Eq for MI_Uint64A {}
impl ::core::default::Default for MI_Uint64A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
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
impl ::core::fmt::Debug for MI_Uint64AField {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_Uint64AField").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
impl ::windows::core::TypeKind for MI_Uint64AField {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MI_Uint64AField {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.exists == other.exists && self.flags == other.flags
    }
}
impl ::core::cmp::Eq for MI_Uint64AField {}
impl ::core::default::Default for MI_Uint64AField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
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
impl ::core::fmt::Debug for MI_Uint64Field {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_Uint64Field").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
impl ::windows::core::TypeKind for MI_Uint64Field {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MI_Uint64Field {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.exists == other.exists && self.flags == other.flags
    }
}
impl ::core::cmp::Eq for MI_Uint64Field {}
impl ::core::default::Default for MI_Uint64Field {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
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
impl ::core::fmt::Debug for MI_Uint8A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_Uint8A").field("data", &self.data).field("size", &self.size).finish()
    }
}
impl ::windows::core::TypeKind for MI_Uint8A {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MI_Uint8A {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data && self.size == other.size
    }
}
impl ::core::cmp::Eq for MI_Uint8A {}
impl ::core::default::Default for MI_Uint8A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
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
impl ::core::fmt::Debug for MI_Uint8AField {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_Uint8AField").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
impl ::windows::core::TypeKind for MI_Uint8AField {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MI_Uint8AField {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.exists == other.exists && self.flags == other.flags
    }
}
impl ::core::cmp::Eq for MI_Uint8AField {}
impl ::core::default::Default for MI_Uint8AField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
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
impl ::core::fmt::Debug for MI_Uint8Field {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_Uint8Field").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
impl ::windows::core::TypeKind for MI_Uint8Field {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MI_Uint8Field {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.exists == other.exists && self.flags == other.flags
    }
}
impl ::core::cmp::Eq for MI_Uint8Field {}
impl ::core::default::Default for MI_Uint8Field {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_UserCredentials {
    pub authenticationType: *const u16,
    pub credentials: MI_UserCredentials_0,
}
impl ::core::marker::Copy for MI_UserCredentials {}
impl ::core::clone::Clone for MI_UserCredentials {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for MI_UserCredentials {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for MI_UserCredentials {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub union MI_UserCredentials_0 {
    pub usernamePassword: MI_UsernamePasswordCreds,
    pub certificateThumbprint: *const u16,
}
impl ::core::marker::Copy for MI_UserCredentials_0 {}
impl ::core::clone::Clone for MI_UserCredentials_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for MI_UserCredentials_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for MI_UserCredentials_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct MI_UsernamePasswordCreds {
    pub domain: *const u16,
    pub username: *const u16,
    pub password: *const u16,
}
impl ::core::marker::Copy for MI_UsernamePasswordCreds {}
impl ::core::clone::Clone for MI_UsernamePasswordCreds {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MI_UsernamePasswordCreds {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_UsernamePasswordCreds").field("domain", &self.domain).field("username", &self.username).field("password", &self.password).finish()
    }
}
impl ::windows::core::TypeKind for MI_UsernamePasswordCreds {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MI_UsernamePasswordCreds {
    fn eq(&self, other: &Self) -> bool {
        self.domain == other.domain && self.username == other.username && self.password == other.password
    }
}
impl ::core::cmp::Eq for MI_UsernamePasswordCreds {}
impl ::core::default::Default for MI_UsernamePasswordCreds {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
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
impl ::core::fmt::Debug for MI_UtilitiesFT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_UtilitiesFT").field("MapErrorToMiErrorCategory", &self.MapErrorToMiErrorCategory).field("CimErrorFromErrorCode", &self.CimErrorFromErrorCode).finish()
    }
}
impl ::windows::core::TypeKind for MI_UtilitiesFT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MI_UtilitiesFT {
    fn eq(&self, other: &Self) -> bool {
        self.MapErrorToMiErrorCategory == other.MapErrorToMiErrorCategory && self.CimErrorFromErrorCode == other.CimErrorFromErrorCode
    }
}
impl ::core::cmp::Eq for MI_UtilitiesFT {}
impl ::core::default::Default for MI_UtilitiesFT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
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
impl ::windows::core::TypeKind for MI_Value {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for MI_Value {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SWbemAnalysisMatrix {
    pub m_uVersion: u32,
    pub m_uMatrixType: u32,
    pub m_pszProperty: ::windows::core::PCWSTR,
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
impl ::core::fmt::Debug for SWbemAnalysisMatrix {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SWbemAnalysisMatrix").field("m_uVersion", &self.m_uVersion).field("m_uMatrixType", &self.m_uMatrixType).field("m_pszProperty", &self.m_pszProperty).field("m_uPropertyType", &self.m_uPropertyType).field("m_uEntries", &self.m_uEntries).field("m_pValues", &self.m_pValues).field("m_pbTruthTable", &self.m_pbTruthTable).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for SWbemAnalysisMatrix {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SWbemAnalysisMatrix {
    fn eq(&self, other: &Self) -> bool {
        self.m_uVersion == other.m_uVersion && self.m_uMatrixType == other.m_uMatrixType && self.m_pszProperty == other.m_pszProperty && self.m_uPropertyType == other.m_uPropertyType && self.m_uEntries == other.m_uEntries && self.m_pValues == other.m_pValues && self.m_pbTruthTable == other.m_pbTruthTable
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
#[doc = "*Required features: `\"Win32_System_Wmi\"`, `\"Win32_Foundation\"`*"]
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
impl ::core::fmt::Debug for SWbemAnalysisMatrixList {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SWbemAnalysisMatrixList").field("m_uVersion", &self.m_uVersion).field("m_uMatrixType", &self.m_uMatrixType).field("m_uNumMatrices", &self.m_uNumMatrices).field("m_pMatrices", &self.m_pMatrices).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for SWbemAnalysisMatrixList {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SWbemAnalysisMatrixList {
    fn eq(&self, other: &Self) -> bool {
        self.m_uVersion == other.m_uVersion && self.m_uMatrixType == other.m_uMatrixType && self.m_uNumMatrices == other.m_uNumMatrices && self.m_pMatrices == other.m_pMatrices
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
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub struct SWbemAssocQueryInf {
    pub m_uVersion: u32,
    pub m_uAnalysisType: u32,
    pub m_uFeatureMask: u32,
    pub m_pPath: ::std::mem::ManuallyDrop<::core::option::Option<IWbemPath>>,
    pub m_pszPath: ::windows::core::PWSTR,
    pub m_pszQueryText: ::windows::core::PWSTR,
    pub m_pszResultClass: ::windows::core::PWSTR,
    pub m_pszAssocClass: ::windows::core::PWSTR,
    pub m_pszRole: ::windows::core::PWSTR,
    pub m_pszResultRole: ::windows::core::PWSTR,
    pub m_pszRequiredQualifier: ::windows::core::PWSTR,
    pub m_pszRequiredAssocQualifier: ::windows::core::PWSTR,
}
impl ::core::clone::Clone for SWbemAssocQueryInf {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
impl ::core::fmt::Debug for SWbemAssocQueryInf {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SWbemAssocQueryInf")
            .field("m_uVersion", &self.m_uVersion)
            .field("m_uAnalysisType", &self.m_uAnalysisType)
            .field("m_uFeatureMask", &self.m_uFeatureMask)
            .field("m_pPath", &self.m_pPath)
            .field("m_pszPath", &self.m_pszPath)
            .field("m_pszQueryText", &self.m_pszQueryText)
            .field("m_pszResultClass", &self.m_pszResultClass)
            .field("m_pszAssocClass", &self.m_pszAssocClass)
            .field("m_pszRole", &self.m_pszRole)
            .field("m_pszResultRole", &self.m_pszResultRole)
            .field("m_pszRequiredQualifier", &self.m_pszRequiredQualifier)
            .field("m_pszRequiredAssocQualifier", &self.m_pszRequiredAssocQualifier)
            .finish()
    }
}
impl ::windows::core::TypeKind for SWbemAssocQueryInf {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SWbemAssocQueryInf {
    fn eq(&self, other: &Self) -> bool {
        self.m_uVersion == other.m_uVersion && self.m_uAnalysisType == other.m_uAnalysisType && self.m_uFeatureMask == other.m_uFeatureMask && self.m_pPath == other.m_pPath && self.m_pszPath == other.m_pszPath && self.m_pszQueryText == other.m_pszQueryText && self.m_pszResultClass == other.m_pszResultClass && self.m_pszAssocClass == other.m_pszAssocClass && self.m_pszRole == other.m_pszRole && self.m_pszResultRole == other.m_pszResultRole && self.m_pszRequiredQualifier == other.m_pszRequiredQualifier && self.m_pszRequiredAssocQualifier == other.m_pszRequiredAssocQualifier
    }
}
impl ::core::cmp::Eq for SWbemAssocQueryInf {}
impl ::core::default::Default for SWbemAssocQueryInf {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SWbemQueryQualifiedName {
    pub m_uVersion: u32,
    pub m_uTokenType: u32,
    pub m_uNameListSize: u32,
    pub m_ppszNameList: *const ::windows::core::PCWSTR,
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
impl ::core::fmt::Debug for SWbemQueryQualifiedName {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SWbemQueryQualifiedName").field("m_uVersion", &self.m_uVersion).field("m_uTokenType", &self.m_uTokenType).field("m_uNameListSize", &self.m_uNameListSize).field("m_ppszNameList", &self.m_ppszNameList).field("m_bArraysUsed", &self.m_bArraysUsed).field("m_pbArrayElUsed", &self.m_pbArrayElUsed).field("m_puArrayIndex", &self.m_puArrayIndex).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for SWbemQueryQualifiedName {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SWbemQueryQualifiedName {
    fn eq(&self, other: &Self) -> bool {
        self.m_uVersion == other.m_uVersion && self.m_uTokenType == other.m_uTokenType && self.m_uNameListSize == other.m_uNameListSize && self.m_ppszNameList == other.m_ppszNameList && self.m_bArraysUsed == other.m_bArraysUsed && self.m_pbArrayElUsed == other.m_pbArrayElUsed && self.m_puArrayIndex == other.m_puArrayIndex
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
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union SWbemRpnConst {
    pub m_pszStrVal: ::windows::core::PCWSTR,
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
impl ::windows::core::TypeKind for SWbemRpnConst {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SWbemRpnConst {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`, `\"Win32_Foundation\"`*"]
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
    pub m_pszOptionalFromPath: ::windows::core::PCWSTR,
    pub m_uFromListSize: u32,
    pub m_ppszFromList: *const ::windows::core::PCWSTR,
    pub m_uWhereClauseSize: u32,
    pub m_ppRpnWhereClause: *mut *mut SWbemRpnQueryToken,
    pub m_dblWithinPolling: f64,
    pub m_dblWithinWindow: f64,
    pub m_uOrderByListSize: u32,
    pub m_ppszOrderByList: *const ::windows::core::PCWSTR,
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
impl ::core::fmt::Debug for SWbemRpnEncodedQuery {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SWbemRpnEncodedQuery")
            .field("m_uVersion", &self.m_uVersion)
            .field("m_uTokenType", &self.m_uTokenType)
            .field("m_uParsedFeatureMask", &self.m_uParsedFeatureMask)
            .field("m_uDetectedArraySize", &self.m_uDetectedArraySize)
            .field("m_puDetectedFeatures", &self.m_puDetectedFeatures)
            .field("m_uSelectListSize", &self.m_uSelectListSize)
            .field("m_ppSelectList", &self.m_ppSelectList)
            .field("m_uFromTargetType", &self.m_uFromTargetType)
            .field("m_pszOptionalFromPath", &self.m_pszOptionalFromPath)
            .field("m_uFromListSize", &self.m_uFromListSize)
            .field("m_ppszFromList", &self.m_ppszFromList)
            .field("m_uWhereClauseSize", &self.m_uWhereClauseSize)
            .field("m_ppRpnWhereClause", &self.m_ppRpnWhereClause)
            .field("m_dblWithinPolling", &self.m_dblWithinPolling)
            .field("m_dblWithinWindow", &self.m_dblWithinWindow)
            .field("m_uOrderByListSize", &self.m_uOrderByListSize)
            .field("m_ppszOrderByList", &self.m_ppszOrderByList)
            .field("m_uOrderDirectionEl", &self.m_uOrderDirectionEl)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for SWbemRpnEncodedQuery {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SWbemRpnEncodedQuery {
    fn eq(&self, other: &Self) -> bool {
        self.m_uVersion == other.m_uVersion
            && self.m_uTokenType == other.m_uTokenType
            && self.m_uParsedFeatureMask == other.m_uParsedFeatureMask
            && self.m_uDetectedArraySize == other.m_uDetectedArraySize
            && self.m_puDetectedFeatures == other.m_puDetectedFeatures
            && self.m_uSelectListSize == other.m_uSelectListSize
            && self.m_ppSelectList == other.m_ppSelectList
            && self.m_uFromTargetType == other.m_uFromTargetType
            && self.m_pszOptionalFromPath == other.m_pszOptionalFromPath
            && self.m_uFromListSize == other.m_uFromListSize
            && self.m_ppszFromList == other.m_ppszFromList
            && self.m_uWhereClauseSize == other.m_uWhereClauseSize
            && self.m_ppRpnWhereClause == other.m_ppRpnWhereClause
            && self.m_dblWithinPolling == other.m_dblWithinPolling
            && self.m_dblWithinWindow == other.m_dblWithinWindow
            && self.m_uOrderByListSize == other.m_uOrderByListSize
            && self.m_ppszOrderByList == other.m_ppszOrderByList
            && self.m_uOrderDirectionEl == other.m_uOrderDirectionEl
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
#[doc = "*Required features: `\"Win32_System_Wmi\"`, `\"Win32_Foundation\"`*"]
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
    pub m_pszRightFunc: ::windows::core::PCWSTR,
    pub m_pszLeftFunc: ::windows::core::PCWSTR,
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
impl ::windows::core::TypeKind for SWbemRpnQueryToken {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SWbemRpnQueryToken {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
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
impl ::core::fmt::Debug for SWbemRpnTokenList {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SWbemRpnTokenList").field("m_uVersion", &self.m_uVersion).field("m_uTokenType", &self.m_uTokenType).field("m_uNumTokens", &self.m_uNumTokens).finish()
    }
}
impl ::windows::core::TypeKind for SWbemRpnTokenList {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SWbemRpnTokenList {
    fn eq(&self, other: &Self) -> bool {
        self.m_uVersion == other.m_uVersion && self.m_uTokenType == other.m_uTokenType && self.m_uNumTokens == other.m_uNumTokens
    }
}
impl ::core::cmp::Eq for SWbemRpnTokenList {}
impl ::core::default::Default for SWbemRpnTokenList {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
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
impl ::core::fmt::Debug for WBEM_COMPILE_STATUS_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WBEM_COMPILE_STATUS_INFO").field("lPhaseError", &self.lPhaseError).field("hRes", &self.hRes).field("ObjectNum", &self.ObjectNum).field("FirstLine", &self.FirstLine).field("LastLine", &self.LastLine).field("dwOutFlags", &self.dwOutFlags).finish()
    }
}
impl ::windows::core::TypeKind for WBEM_COMPILE_STATUS_INFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WBEM_COMPILE_STATUS_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.lPhaseError == other.lPhaseError && self.hRes == other.hRes && self.ObjectNum == other.ObjectNum && self.FirstLine == other.FirstLine && self.LastLine == other.LastLine && self.dwOutFlags == other.dwOutFlags
    }
}
impl ::core::cmp::Eq for WBEM_COMPILE_STATUS_INFO {}
impl ::core::default::Default for WBEM_COMPILE_STATUS_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub type MI_CancelCallback = ::core::option::Option<unsafe extern "system" fn(reason: MI_CancellationReason, callbackdata: *const ::core::ffi::c_void) -> ()>;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub type MI_Deserializer_ClassObjectNeeded = ::core::option::Option<unsafe extern "system" fn(context: *const ::core::ffi::c_void, servername: *const u16, namespacename: *const u16, classname: *const u16, requestedclassobject: *mut *mut MI_Class) -> MI_Result>;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub type MI_MainFunction = ::core::option::Option<unsafe extern "system" fn(server: *mut MI_Server) -> *mut MI_Module>;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub type MI_MethodDecl_Invoke = ::core::option::Option<unsafe extern "system" fn(self_: *const ::core::ffi::c_void, context: *const MI_Context, namespace: *const u16, classname: *const u16, methodname: *const u16, instancename: *const MI_Instance, parameters: *const MI_Instance) -> ()>;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub type MI_Module_Load = ::core::option::Option<unsafe extern "system" fn(self_: *mut *mut MI_Module_Self, context: *const MI_Context) -> ()>;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub type MI_Module_Unload = ::core::option::Option<unsafe extern "system" fn(self_: *const MI_Module_Self, context: *const MI_Context) -> ()>;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub type MI_OperationCallback_Class = ::core::option::Option<unsafe extern "system" fn(operation: *const MI_Operation, callbackcontext: *const ::core::ffi::c_void, classresult: *const MI_Class, moreresults: u8, resultcode: MI_Result, errorstring: *const u16, errordetails: *const MI_Instance, resultacknowledgement: isize) -> ()>;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub type MI_OperationCallback_Indication = ::core::option::Option<unsafe extern "system" fn(operation: *const MI_Operation, callbackcontext: *const ::core::ffi::c_void, instance: *const MI_Instance, bookmark: *const u16, machineid: *const u16, moreresults: u8, resultcode: MI_Result, errorstring: *const u16, errordetails: *const MI_Instance, resultacknowledgement: isize) -> ()>;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub type MI_OperationCallback_Instance = ::core::option::Option<unsafe extern "system" fn(operation: *const MI_Operation, callbackcontext: *const ::core::ffi::c_void, instance: *const MI_Instance, moreresults: u8, resultcode: MI_Result, errorstring: *const u16, errordetails: *const MI_Instance, resultacknowledgement: isize) -> ()>;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub type MI_OperationCallback_PromptUser = ::core::option::Option<unsafe extern "system" fn(operation: *const MI_Operation, callbackcontext: *const ::core::ffi::c_void, message: *const u16, prompttype: MI_PromptType, promptuserresult: isize) -> ()>;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub type MI_OperationCallback_StreamedParameter = ::core::option::Option<unsafe extern "system" fn(operation: *const MI_Operation, callbackcontext: *const ::core::ffi::c_void, parametername: *const u16, resulttype: MI_Type, result: *const MI_Value, resultacknowledgement: isize) -> ()>;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub type MI_OperationCallback_WriteError = ::core::option::Option<unsafe extern "system" fn(operation: *const MI_Operation, callbackcontext: *const ::core::ffi::c_void, instance: *const MI_Instance, writeerrorresult: isize) -> ()>;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub type MI_OperationCallback_WriteMessage = ::core::option::Option<unsafe extern "system" fn(operation: *const MI_Operation, callbackcontext: *const ::core::ffi::c_void, channel: u32, message: *const u16) -> ()>;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub type MI_OperationCallback_WriteProgress = ::core::option::Option<unsafe extern "system" fn(operation: *const MI_Operation, callbackcontext: *const ::core::ffi::c_void, activity: *const u16, currentoperation: *const u16, statusdescription: *const u16, percentagecomplete: u32, secondsremaining: u32) -> ()>;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub type MI_ProviderFT_AssociatorInstances = ::core::option::Option<unsafe extern "system" fn(self_: *const ::core::ffi::c_void, context: *const MI_Context, namespace: *const u16, classname: *const u16, instancename: *const MI_Instance, resultclass: *const u16, role: *const u16, resultrole: *const u16, propertyset: *const MI_PropertySet, keysonly: u8, filter: *const MI_Filter) -> ()>;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub type MI_ProviderFT_CreateInstance = ::core::option::Option<unsafe extern "system" fn(self_: *const ::core::ffi::c_void, context: *const MI_Context, namespace: *const u16, classname: *const u16, newinstance: *const MI_Instance) -> ()>;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub type MI_ProviderFT_DeleteInstance = ::core::option::Option<unsafe extern "system" fn(self_: *const ::core::ffi::c_void, context: *const MI_Context, namespace: *const u16, classname: *const u16, instancename: *const MI_Instance) -> ()>;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub type MI_ProviderFT_DisableIndications = ::core::option::Option<unsafe extern "system" fn(self_: *const ::core::ffi::c_void, indicationscontext: *const MI_Context, namespace: *const u16, classname: *const u16) -> ()>;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub type MI_ProviderFT_EnableIndications = ::core::option::Option<unsafe extern "system" fn(self_: *const ::core::ffi::c_void, indicationscontext: *const MI_Context, namespace: *const u16, classname: *const u16) -> ()>;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub type MI_ProviderFT_EnumerateInstances = ::core::option::Option<unsafe extern "system" fn(self_: *const ::core::ffi::c_void, context: *const MI_Context, namespace: *const u16, classname: *const u16, propertyset: *const MI_PropertySet, keysonly: u8, filter: *const MI_Filter) -> ()>;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub type MI_ProviderFT_GetInstance = ::core::option::Option<unsafe extern "system" fn(self_: *const ::core::ffi::c_void, context: *const MI_Context, namespace: *const u16, classname: *const u16, instancename: *const MI_Instance, propertyset: *const MI_PropertySet) -> ()>;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub type MI_ProviderFT_Invoke = ::core::option::Option<unsafe extern "system" fn(self_: *const ::core::ffi::c_void, context: *const MI_Context, namespace: *const u16, classname: *const u16, methodname: *const u16, instancename: *const MI_Instance, inputparameters: *const MI_Instance) -> ()>;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub type MI_ProviderFT_Load = ::core::option::Option<unsafe extern "system" fn(self_: *mut *mut ::core::ffi::c_void, selfmodule: *const MI_Module_Self, context: *const MI_Context) -> ()>;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub type MI_ProviderFT_ModifyInstance = ::core::option::Option<unsafe extern "system" fn(self_: *mut ::core::ffi::c_void, context: *mut MI_Context, namespace: *const u16, classname: *const u16, modifiedinstance: *const MI_Instance, propertyset: *const MI_PropertySet) -> ()>;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub type MI_ProviderFT_ReferenceInstances = ::core::option::Option<unsafe extern "system" fn(self_: *const ::core::ffi::c_void, context: *const MI_Context, namespace: *const u16, classname: *const u16, instancename: *const MI_Instance, role: *const u16, propertyset: *const MI_PropertySet, keysonly: u8, filter: *const MI_Filter) -> ()>;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub type MI_ProviderFT_Subscribe = ::core::option::Option<unsafe extern "system" fn(self_: *const ::core::ffi::c_void, context: *const MI_Context, namespace: *const u16, classname: *const u16, filter: *const MI_Filter, bookmark: *const u16, subscriptionid: u64, subscriptionself: *mut *mut ::core::ffi::c_void) -> ()>;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub type MI_ProviderFT_Unload = ::core::option::Option<unsafe extern "system" fn(self_: *const ::core::ffi::c_void, context: *const MI_Context) -> ()>;
#[doc = "*Required features: `\"Win32_System_Wmi\"`*"]
pub type MI_ProviderFT_Unsubscribe = ::core::option::Option<unsafe extern "system" fn(self_: *const ::core::ffi::c_void, context: *const MI_Context, namespace: *const u16, classname: *const u16, subscriptionid: u64, subscriptionself: *const ::core::ffi::c_void) -> ()>;
#[cfg(feature = "implement")]
::core::include!("impl.rs");
