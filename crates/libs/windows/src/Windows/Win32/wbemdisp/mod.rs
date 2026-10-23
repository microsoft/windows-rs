#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(ISWbemDateTime, ISWbemDateTime_Vtbl, 0x5e97458a_cf77_11d3_b38f_00105a1f473a);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for ISWbemDateTime {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(ISWbemDateTime, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl ISWbemDateTime {
    pub unsafe fn get_Value(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Value)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_Value(&self, strvalue: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Value)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strvalue)) }
    }
    pub unsafe fn get_Year(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Year)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_Year(&self, iyear: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Year)(windows_core::Interface::as_raw(self), iyear) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_YearSpecified(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_YearSpecified)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_YearSpecified(&self, byearspecified: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_YearSpecified)(windows_core::Interface::as_raw(self), byearspecified) }
    }
    pub unsafe fn get_Month(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Month)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_Month(&self, imonth: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Month)(windows_core::Interface::as_raw(self), imonth) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_MonthSpecified(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_MonthSpecified)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_MonthSpecified(&self, bmonthspecified: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_MonthSpecified)(windows_core::Interface::as_raw(self), bmonthspecified) }
    }
    pub unsafe fn get_Day(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Day)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_Day(&self, iday: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Day)(windows_core::Interface::as_raw(self), iday) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_DaySpecified(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_DaySpecified)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_DaySpecified(&self, bdayspecified: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_DaySpecified)(windows_core::Interface::as_raw(self), bdayspecified) }
    }
    pub unsafe fn get_Hours(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Hours)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_Hours(&self, ihours: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Hours)(windows_core::Interface::as_raw(self), ihours) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_HoursSpecified(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_HoursSpecified)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_HoursSpecified(&self, bhoursspecified: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_HoursSpecified)(windows_core::Interface::as_raw(self), bhoursspecified) }
    }
    pub unsafe fn get_Minutes(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Minutes)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_Minutes(&self, iminutes: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Minutes)(windows_core::Interface::as_raw(self), iminutes) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_MinutesSpecified(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_MinutesSpecified)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_MinutesSpecified(&self, bminutesspecified: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_MinutesSpecified)(windows_core::Interface::as_raw(self), bminutesspecified) }
    }
    pub unsafe fn get_Seconds(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Seconds)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_Seconds(&self, iseconds: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Seconds)(windows_core::Interface::as_raw(self), iseconds) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_SecondsSpecified(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_SecondsSpecified)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_SecondsSpecified(&self, bsecondsspecified: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_SecondsSpecified)(windows_core::Interface::as_raw(self), bsecondsspecified) }
    }
    pub unsafe fn get_Microseconds(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Microseconds)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_Microseconds(&self, imicroseconds: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Microseconds)(windows_core::Interface::as_raw(self), imicroseconds) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_MicrosecondsSpecified(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_MicrosecondsSpecified)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_MicrosecondsSpecified(&self, bmicrosecondsspecified: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_MicrosecondsSpecified)(windows_core::Interface::as_raw(self), bmicrosecondsspecified) }
    }
    pub unsafe fn get_UTC(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_UTC)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_UTC(&self, iutc: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_UTC)(windows_core::Interface::as_raw(self), iutc) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_UTCSpecified(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_UTCSpecified)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_UTCSpecified(&self, butcspecified: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_UTCSpecified)(windows_core::Interface::as_raw(self), butcspecified) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_IsInterval(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_IsInterval)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_IsInterval(&self, bisinterval: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_IsInterval)(windows_core::Interface::as_raw(self), bisinterval) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn GetVarDate(&self, bislocal: super::VARIANT_BOOL) -> windows_core::Result<super::DATE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetVarDate)(windows_core::Interface::as_raw(self), bislocal, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn SetVarDate(&self, dvardate: super::DATE, bislocal: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetVarDate)(windows_core::Interface::as_raw(self), dvardate, bislocal) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn GetFileTime(&self, bislocal: super::VARIANT_BOOL) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFileTime)(windows_core::Interface::as_raw(self), bislocal, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn SetFileTime(&self, strfiletime: &windows_core::BSTR, bislocal: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetFileTime)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strfiletime), bislocal) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct ISWbemDateTime_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub get_Value: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_Value: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Year: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub put_Year: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub get_YearSpecified: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_YearSpecified: usize,
    #[cfg(feature = "wtypes")]
    pub put_YearSpecified: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_YearSpecified: usize,
    pub get_Month: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub put_Month: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub get_MonthSpecified: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_MonthSpecified: usize,
    #[cfg(feature = "wtypes")]
    pub put_MonthSpecified: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_MonthSpecified: usize,
    pub get_Day: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub put_Day: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub get_DaySpecified: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_DaySpecified: usize,
    #[cfg(feature = "wtypes")]
    pub put_DaySpecified: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_DaySpecified: usize,
    pub get_Hours: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub put_Hours: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub get_HoursSpecified: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_HoursSpecified: usize,
    #[cfg(feature = "wtypes")]
    pub put_HoursSpecified: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_HoursSpecified: usize,
    pub get_Minutes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub put_Minutes: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub get_MinutesSpecified: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_MinutesSpecified: usize,
    #[cfg(feature = "wtypes")]
    pub put_MinutesSpecified: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_MinutesSpecified: usize,
    pub get_Seconds: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub put_Seconds: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub get_SecondsSpecified: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_SecondsSpecified: usize,
    #[cfg(feature = "wtypes")]
    pub put_SecondsSpecified: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_SecondsSpecified: usize,
    pub get_Microseconds: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub put_Microseconds: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub get_MicrosecondsSpecified: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_MicrosecondsSpecified: usize,
    #[cfg(feature = "wtypes")]
    pub put_MicrosecondsSpecified: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_MicrosecondsSpecified: usize,
    pub get_UTC: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub put_UTC: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub get_UTCSpecified: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_UTCSpecified: usize,
    #[cfg(feature = "wtypes")]
    pub put_UTCSpecified: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_UTCSpecified: usize,
    #[cfg(feature = "wtypes")]
    pub get_IsInterval: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_IsInterval: usize,
    #[cfg(feature = "wtypes")]
    pub put_IsInterval: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_IsInterval: usize,
    #[cfg(feature = "wtypes")]
    pub GetVarDate: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL, *mut super::DATE) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    GetVarDate: usize,
    #[cfg(feature = "wtypes")]
    pub SetVarDate: unsafe extern "system" fn(*mut core::ffi::c_void, super::DATE, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    SetVarDate: usize,
    #[cfg(feature = "wtypes")]
    pub GetFileTime: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    GetFileTime: usize,
    #[cfg(feature = "wtypes")]
    pub SetFileTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    SetFileTime: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait ISWbemDateTime_Impl: super::IDispatch_Impl {
    fn get_Value(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_Value(&self, strvalue: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_Year(&self) -> windows_core::Result<i32>;
    fn put_Year(&self, iyear: i32) -> windows_core::Result<()>;
    fn get_YearSpecified(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_YearSpecified(&self, byearspecified: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn get_Month(&self) -> windows_core::Result<i32>;
    fn put_Month(&self, imonth: i32) -> windows_core::Result<()>;
    fn get_MonthSpecified(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_MonthSpecified(&self, bmonthspecified: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn get_Day(&self) -> windows_core::Result<i32>;
    fn put_Day(&self, iday: i32) -> windows_core::Result<()>;
    fn get_DaySpecified(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_DaySpecified(&self, bdayspecified: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn get_Hours(&self) -> windows_core::Result<i32>;
    fn put_Hours(&self, ihours: i32) -> windows_core::Result<()>;
    fn get_HoursSpecified(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_HoursSpecified(&self, bhoursspecified: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn get_Minutes(&self) -> windows_core::Result<i32>;
    fn put_Minutes(&self, iminutes: i32) -> windows_core::Result<()>;
    fn get_MinutesSpecified(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_MinutesSpecified(&self, bminutesspecified: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn get_Seconds(&self) -> windows_core::Result<i32>;
    fn put_Seconds(&self, iseconds: i32) -> windows_core::Result<()>;
    fn get_SecondsSpecified(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_SecondsSpecified(&self, bsecondsspecified: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn get_Microseconds(&self) -> windows_core::Result<i32>;
    fn put_Microseconds(&self, imicroseconds: i32) -> windows_core::Result<()>;
    fn get_MicrosecondsSpecified(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_MicrosecondsSpecified(&self, bmicrosecondsspecified: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn get_UTC(&self) -> windows_core::Result<i32>;
    fn put_UTC(&self, iutc: i32) -> windows_core::Result<()>;
    fn get_UTCSpecified(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_UTCSpecified(&self, butcspecified: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn get_IsInterval(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_IsInterval(&self, bisinterval: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn GetVarDate(&self, bislocal: super::VARIANT_BOOL) -> windows_core::Result<super::DATE>;
    fn SetVarDate(&self, dvardate: super::DATE, bislocal: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn GetFileTime(&self, bislocal: super::VARIANT_BOOL) -> windows_core::Result<windows_core::BSTR>;
    fn SetFileTime(&self, strfiletime: &windows_core::BSTR, bislocal: super::VARIANT_BOOL) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl ISWbemDateTime_Vtbl {
    pub const fn new<Identity: ISWbemDateTime_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_Value<Identity: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemDateTime_Impl::get_Value(this) {
                    Ok(ok__) => {
                        strvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Value<Identity: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strvalue: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISWbemDateTime_Impl::put_Value(this, core::mem::transmute(&strvalue)).into()
            }
        }
        unsafe extern "system" fn get_Year<Identity: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iyear: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemDateTime_Impl::get_Year(this) {
                    Ok(ok__) => {
                        iyear.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Year<Identity: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iyear: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISWbemDateTime_Impl::put_Year(this, core::mem::transmute_copy(&iyear)).into()
            }
        }
        unsafe extern "system" fn get_YearSpecified<Identity: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, byearspecified: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemDateTime_Impl::get_YearSpecified(this) {
                    Ok(ok__) => {
                        byearspecified.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_YearSpecified<Identity: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, byearspecified: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISWbemDateTime_Impl::put_YearSpecified(this, core::mem::transmute_copy(&byearspecified)).into()
            }
        }
        unsafe extern "system" fn get_Month<Identity: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, imonth: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemDateTime_Impl::get_Month(this) {
                    Ok(ok__) => {
                        imonth.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Month<Identity: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, imonth: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISWbemDateTime_Impl::put_Month(this, core::mem::transmute_copy(&imonth)).into()
            }
        }
        unsafe extern "system" fn get_MonthSpecified<Identity: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bmonthspecified: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemDateTime_Impl::get_MonthSpecified(this) {
                    Ok(ok__) => {
                        bmonthspecified.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_MonthSpecified<Identity: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bmonthspecified: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISWbemDateTime_Impl::put_MonthSpecified(this, core::mem::transmute_copy(&bmonthspecified)).into()
            }
        }
        unsafe extern "system" fn get_Day<Identity: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iday: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemDateTime_Impl::get_Day(this) {
                    Ok(ok__) => {
                        iday.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Day<Identity: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iday: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISWbemDateTime_Impl::put_Day(this, core::mem::transmute_copy(&iday)).into()
            }
        }
        unsafe extern "system" fn get_DaySpecified<Identity: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bdayspecified: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemDateTime_Impl::get_DaySpecified(this) {
                    Ok(ok__) => {
                        bdayspecified.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_DaySpecified<Identity: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bdayspecified: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISWbemDateTime_Impl::put_DaySpecified(this, core::mem::transmute_copy(&bdayspecified)).into()
            }
        }
        unsafe extern "system" fn get_Hours<Identity: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ihours: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemDateTime_Impl::get_Hours(this) {
                    Ok(ok__) => {
                        ihours.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Hours<Identity: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ihours: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISWbemDateTime_Impl::put_Hours(this, core::mem::transmute_copy(&ihours)).into()
            }
        }
        unsafe extern "system" fn get_HoursSpecified<Identity: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bhoursspecified: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemDateTime_Impl::get_HoursSpecified(this) {
                    Ok(ok__) => {
                        bhoursspecified.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_HoursSpecified<Identity: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bhoursspecified: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISWbemDateTime_Impl::put_HoursSpecified(this, core::mem::transmute_copy(&bhoursspecified)).into()
            }
        }
        unsafe extern "system" fn get_Minutes<Identity: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iminutes: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemDateTime_Impl::get_Minutes(this) {
                    Ok(ok__) => {
                        iminutes.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Minutes<Identity: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iminutes: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISWbemDateTime_Impl::put_Minutes(this, core::mem::transmute_copy(&iminutes)).into()
            }
        }
        unsafe extern "system" fn get_MinutesSpecified<Identity: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bminutesspecified: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemDateTime_Impl::get_MinutesSpecified(this) {
                    Ok(ok__) => {
                        bminutesspecified.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_MinutesSpecified<Identity: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bminutesspecified: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISWbemDateTime_Impl::put_MinutesSpecified(this, core::mem::transmute_copy(&bminutesspecified)).into()
            }
        }
        unsafe extern "system" fn get_Seconds<Identity: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iseconds: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemDateTime_Impl::get_Seconds(this) {
                    Ok(ok__) => {
                        iseconds.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Seconds<Identity: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iseconds: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISWbemDateTime_Impl::put_Seconds(this, core::mem::transmute_copy(&iseconds)).into()
            }
        }
        unsafe extern "system" fn get_SecondsSpecified<Identity: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bsecondsspecified: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemDateTime_Impl::get_SecondsSpecified(this) {
                    Ok(ok__) => {
                        bsecondsspecified.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_SecondsSpecified<Identity: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bsecondsspecified: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISWbemDateTime_Impl::put_SecondsSpecified(this, core::mem::transmute_copy(&bsecondsspecified)).into()
            }
        }
        unsafe extern "system" fn get_Microseconds<Identity: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, imicroseconds: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemDateTime_Impl::get_Microseconds(this) {
                    Ok(ok__) => {
                        imicroseconds.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Microseconds<Identity: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, imicroseconds: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISWbemDateTime_Impl::put_Microseconds(this, core::mem::transmute_copy(&imicroseconds)).into()
            }
        }
        unsafe extern "system" fn get_MicrosecondsSpecified<Identity: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bmicrosecondsspecified: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemDateTime_Impl::get_MicrosecondsSpecified(this) {
                    Ok(ok__) => {
                        bmicrosecondsspecified.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_MicrosecondsSpecified<Identity: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bmicrosecondsspecified: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISWbemDateTime_Impl::put_MicrosecondsSpecified(this, core::mem::transmute_copy(&bmicrosecondsspecified)).into()
            }
        }
        unsafe extern "system" fn get_UTC<Identity: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iutc: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemDateTime_Impl::get_UTC(this) {
                    Ok(ok__) => {
                        iutc.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_UTC<Identity: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iutc: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISWbemDateTime_Impl::put_UTC(this, core::mem::transmute_copy(&iutc)).into()
            }
        }
        unsafe extern "system" fn get_UTCSpecified<Identity: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, butcspecified: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemDateTime_Impl::get_UTCSpecified(this) {
                    Ok(ok__) => {
                        butcspecified.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_UTCSpecified<Identity: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, butcspecified: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISWbemDateTime_Impl::put_UTCSpecified(this, core::mem::transmute_copy(&butcspecified)).into()
            }
        }
        unsafe extern "system" fn get_IsInterval<Identity: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bisinterval: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemDateTime_Impl::get_IsInterval(this) {
                    Ok(ok__) => {
                        bisinterval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_IsInterval<Identity: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bisinterval: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISWbemDateTime_Impl::put_IsInterval(this, core::mem::transmute_copy(&bisinterval)).into()
            }
        }
        unsafe extern "system" fn GetVarDate<Identity: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bislocal: super::VARIANT_BOOL, dvardate: *mut super::DATE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemDateTime_Impl::GetVarDate(this, core::mem::transmute_copy(&bislocal)) {
                    Ok(ok__) => {
                        dvardate.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetVarDate<Identity: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dvardate: super::DATE, bislocal: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISWbemDateTime_Impl::SetVarDate(this, core::mem::transmute_copy(&dvardate), core::mem::transmute_copy(&bislocal)).into()
            }
        }
        unsafe extern "system" fn GetFileTime<Identity: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bislocal: super::VARIANT_BOOL, strfiletime: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemDateTime_Impl::GetFileTime(this, core::mem::transmute_copy(&bislocal)) {
                    Ok(ok__) => {
                        strfiletime.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetFileTime<Identity: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strfiletime: *mut core::ffi::c_void, bislocal: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISWbemDateTime_Impl::SetFileTime(this, core::mem::transmute(&strfiletime), core::mem::transmute_copy(&bislocal)).into()
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_Value: get_Value::<Identity, OFFSET>,
            put_Value: put_Value::<Identity, OFFSET>,
            get_Year: get_Year::<Identity, OFFSET>,
            put_Year: put_Year::<Identity, OFFSET>,
            get_YearSpecified: get_YearSpecified::<Identity, OFFSET>,
            put_YearSpecified: put_YearSpecified::<Identity, OFFSET>,
            get_Month: get_Month::<Identity, OFFSET>,
            put_Month: put_Month::<Identity, OFFSET>,
            get_MonthSpecified: get_MonthSpecified::<Identity, OFFSET>,
            put_MonthSpecified: put_MonthSpecified::<Identity, OFFSET>,
            get_Day: get_Day::<Identity, OFFSET>,
            put_Day: put_Day::<Identity, OFFSET>,
            get_DaySpecified: get_DaySpecified::<Identity, OFFSET>,
            put_DaySpecified: put_DaySpecified::<Identity, OFFSET>,
            get_Hours: get_Hours::<Identity, OFFSET>,
            put_Hours: put_Hours::<Identity, OFFSET>,
            get_HoursSpecified: get_HoursSpecified::<Identity, OFFSET>,
            put_HoursSpecified: put_HoursSpecified::<Identity, OFFSET>,
            get_Minutes: get_Minutes::<Identity, OFFSET>,
            put_Minutes: put_Minutes::<Identity, OFFSET>,
            get_MinutesSpecified: get_MinutesSpecified::<Identity, OFFSET>,
            put_MinutesSpecified: put_MinutesSpecified::<Identity, OFFSET>,
            get_Seconds: get_Seconds::<Identity, OFFSET>,
            put_Seconds: put_Seconds::<Identity, OFFSET>,
            get_SecondsSpecified: get_SecondsSpecified::<Identity, OFFSET>,
            put_SecondsSpecified: put_SecondsSpecified::<Identity, OFFSET>,
            get_Microseconds: get_Microseconds::<Identity, OFFSET>,
            put_Microseconds: put_Microseconds::<Identity, OFFSET>,
            get_MicrosecondsSpecified: get_MicrosecondsSpecified::<Identity, OFFSET>,
            put_MicrosecondsSpecified: put_MicrosecondsSpecified::<Identity, OFFSET>,
            get_UTC: get_UTC::<Identity, OFFSET>,
            put_UTC: put_UTC::<Identity, OFFSET>,
            get_UTCSpecified: get_UTCSpecified::<Identity, OFFSET>,
            put_UTCSpecified: put_UTCSpecified::<Identity, OFFSET>,
            get_IsInterval: get_IsInterval::<Identity, OFFSET>,
            put_IsInterval: put_IsInterval::<Identity, OFFSET>,
            GetVarDate: GetVarDate::<Identity, OFFSET>,
            SetVarDate: SetVarDate::<Identity, OFFSET>,
            GetFileTime: GetFileTime::<Identity, OFFSET>,
            SetFileTime: SetFileTime::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISWbemDateTime as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for ISWbemDateTime {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(ISWbemEventSource, ISWbemEventSource_Vtbl, 0x27d54d92_0ebe_11d2_8b22_00600806d9b6);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for ISWbemEventSource {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(ISWbemEventSource, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl ISWbemEventSource {
    pub unsafe fn NextEvent(&self, itimeoutms: i32) -> windows_core::Result<ISWbemObject> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).NextEvent)(windows_core::Interface::as_raw(self), itimeoutms, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_Security_(&self) -> windows_core::Result<ISWbemSecurity> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Security_)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct ISWbemEventSource_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub NextEvent: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Security_: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait ISWbemEventSource_Impl: super::IDispatch_Impl {
    fn NextEvent(&self, itimeoutms: i32) -> windows_core::Result<ISWbemObject>;
    fn get_Security_(&self) -> windows_core::Result<ISWbemSecurity>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl ISWbemEventSource_Vtbl {
    pub const fn new<Identity: ISWbemEventSource_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn NextEvent<Identity: ISWbemEventSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, itimeoutms: i32, objwbemobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemEventSource_Impl::NextEvent(this, core::mem::transmute_copy(&itimeoutms)) {
                    Ok(ok__) => {
                        objwbemobject.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Security_<Identity: ISWbemEventSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemsecurity: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemEventSource_Impl::get_Security_(this) {
                    Ok(ok__) => {
                        objwbemsecurity.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            NextEvent: NextEvent::<Identity, OFFSET>,
            get_Security_: get_Security_::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISWbemEventSource as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for ISWbemEventSource {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(ISWbemLastError, ISWbemLastError_Vtbl, 0xd962db84_d4bb_11d1_8b09_00600806d9b6);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for ISWbemLastError {
    type Target = ISWbemObject;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(ISWbemLastError, windows_core::IUnknown, super::IDispatch, ISWbemObject);
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct ISWbemLastError_Vtbl {
    pub base__: ISWbemObject_Vtbl,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait ISWbemLastError_Impl: ISWbemObject_Impl {}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl ISWbemLastError_Vtbl {
    pub const fn new<Identity: ISWbemLastError_Impl, const OFFSET: isize>() -> Self {
        Self { base__: ISWbemObject_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISWbemLastError as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<ISWbemObject as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for ISWbemLastError {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(ISWbemLocator, ISWbemLocator_Vtbl, 0x76a6415b_cb41_11d1_8b02_00600806d9b6);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for ISWbemLocator {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(ISWbemLocator, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl ISWbemLocator {
    pub unsafe fn ConnectServer<P7>(&self, strserver: &windows_core::BSTR, strnamespace: &windows_core::BSTR, struser: &windows_core::BSTR, strpassword: &windows_core::BSTR, strlocale: &windows_core::BSTR, strauthority: &windows_core::BSTR, isecurityflags: i32, objwbemnamedvalueset: P7) -> windows_core::Result<ISWbemServices>
    where
        P7: windows_core::Param<super::IDispatch>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ConnectServer)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strserver), core::mem::transmute_copy(strnamespace), core::mem::transmute_copy(struser), core::mem::transmute_copy(strpassword), core::mem::transmute_copy(strlocale), core::mem::transmute_copy(strauthority), isecurityflags, objwbemnamedvalueset.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_Security_(&self) -> windows_core::Result<ISWbemSecurity> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Security_)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct ISWbemLocator_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub ConnectServer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Security_: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait ISWbemLocator_Impl: super::IDispatch_Impl {
    fn ConnectServer(&self, strserver: &windows_core::BSTR, strnamespace: &windows_core::BSTR, struser: &windows_core::BSTR, strpassword: &windows_core::BSTR, strlocale: &windows_core::BSTR, strauthority: &windows_core::BSTR, isecurityflags: i32, objwbemnamedvalueset: windows_core::Ref<super::IDispatch>) -> windows_core::Result<ISWbemServices>;
    fn get_Security_(&self) -> windows_core::Result<ISWbemSecurity>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl ISWbemLocator_Vtbl {
    pub const fn new<Identity: ISWbemLocator_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ConnectServer<Identity: ISWbemLocator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strserver: *mut core::ffi::c_void, strnamespace: *mut core::ffi::c_void, struser: *mut core::ffi::c_void, strpassword: *mut core::ffi::c_void, strlocale: *mut core::ffi::c_void, strauthority: *mut core::ffi::c_void, isecurityflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void, objwbemservices: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemLocator_Impl::ConnectServer(this, core::mem::transmute(&strserver), core::mem::transmute(&strnamespace), core::mem::transmute(&struser), core::mem::transmute(&strpassword), core::mem::transmute(&strlocale), core::mem::transmute(&strauthority), core::mem::transmute_copy(&isecurityflags), core::mem::transmute_copy(&objwbemnamedvalueset)) {
                    Ok(ok__) => {
                        objwbemservices.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Security_<Identity: ISWbemLocator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemsecurity: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemLocator_Impl::get_Security_(this) {
                    Ok(ok__) => {
                        objwbemsecurity.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            ConnectServer: ConnectServer::<Identity, OFFSET>,
            get_Security_: get_Security_::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISWbemLocator as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for ISWbemLocator {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(ISWbemMethod, ISWbemMethod_Vtbl, 0x422e8e90_d955_11d1_8b09_00600806d9b6);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for ISWbemMethod {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(ISWbemMethod, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl ISWbemMethod {
    pub unsafe fn get_Name(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_Origin(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Origin)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_InParameters(&self) -> windows_core::Result<ISWbemObject> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_InParameters)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_OutParameters(&self) -> windows_core::Result<ISWbemObject> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_OutParameters)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_Qualifiers_(&self) -> windows_core::Result<ISWbemQualifierSet> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Qualifiers_)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct ISWbemMethod_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub get_Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Origin: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_InParameters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_OutParameters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Qualifiers_: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait ISWbemMethod_Impl: super::IDispatch_Impl {
    fn get_Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_Origin(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_InParameters(&self) -> windows_core::Result<ISWbemObject>;
    fn get_OutParameters(&self) -> windows_core::Result<ISWbemObject>;
    fn get_Qualifiers_(&self) -> windows_core::Result<ISWbemQualifierSet>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl ISWbemMethod_Vtbl {
    pub const fn new<Identity: ISWbemMethod_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_Name<Identity: ISWbemMethod_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemMethod_Impl::get_Name(this) {
                    Ok(ok__) => {
                        strname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Origin<Identity: ISWbemMethod_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strorigin: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemMethod_Impl::get_Origin(this) {
                    Ok(ok__) => {
                        strorigin.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_InParameters<Identity: ISWbemMethod_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbeminparameters: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemMethod_Impl::get_InParameters(this) {
                    Ok(ok__) => {
                        objwbeminparameters.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_OutParameters<Identity: ISWbemMethod_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemoutparameters: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemMethod_Impl::get_OutParameters(this) {
                    Ok(ok__) => {
                        objwbemoutparameters.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Qualifiers_<Identity: ISWbemMethod_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemqualifierset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemMethod_Impl::get_Qualifiers_(this) {
                    Ok(ok__) => {
                        objwbemqualifierset.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_Name: get_Name::<Identity, OFFSET>,
            get_Origin: get_Origin::<Identity, OFFSET>,
            get_InParameters: get_InParameters::<Identity, OFFSET>,
            get_OutParameters: get_OutParameters::<Identity, OFFSET>,
            get_Qualifiers_: get_Qualifiers_::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISWbemMethod as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for ISWbemMethod {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(ISWbemMethodSet, ISWbemMethodSet_Vtbl, 0xc93ba292_d955_11d1_8b09_00600806d9b6);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for ISWbemMethodSet {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(ISWbemMethodSet, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl ISWbemMethodSet {
    pub unsafe fn get__NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get__NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn Item(&self, strname: &windows_core::BSTR, iflags: i32) -> windows_core::Result<ISWbemMethod> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Item)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strname), iflags, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct ISWbemMethodSet_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub get__NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Item: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait ISWbemMethodSet_Impl: super::IDispatch_Impl {
    fn get__NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Item(&self, strname: &windows_core::BSTR, iflags: i32) -> windows_core::Result<ISWbemMethod>;
    fn get_Count(&self) -> windows_core::Result<i32>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl ISWbemMethodSet_Vtbl {
    pub const fn new<Identity: ISWbemMethodSet_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get__NewEnum<Identity: ISWbemMethodSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemMethodSet_Impl::get__NewEnum(this) {
                    Ok(ok__) => {
                        punk.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Item<Identity: ISWbemMethodSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strname: *mut core::ffi::c_void, iflags: i32, objwbemmethod: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemMethodSet_Impl::Item(this, core::mem::transmute(&strname), core::mem::transmute_copy(&iflags)) {
                    Ok(ok__) => {
                        objwbemmethod.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Count<Identity: ISWbemMethodSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, icount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemMethodSet_Impl::get_Count(this) {
                    Ok(ok__) => {
                        icount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get__NewEnum: get__NewEnum::<Identity, OFFSET>,
            Item: Item::<Identity, OFFSET>,
            get_Count: get_Count::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISWbemMethodSet as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for ISWbemMethodSet {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(ISWbemNamedValue, ISWbemNamedValue_Vtbl, 0x76a64164_cb41_11d1_8b02_00600806d9b6);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for ISWbemNamedValue {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(ISWbemNamedValue, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl ISWbemNamedValue {
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn get_Value(&self) -> windows_core::Result<super::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Value)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn put_Value(&self, varvalue: *const super::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Value)(windows_core::Interface::as_raw(self), varvalue) }
    }
    pub unsafe fn get_Name(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct ISWbemNamedValue_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub get_Value: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    get_Value: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub put_Value: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    put_Value: usize,
    pub get_Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait ISWbemNamedValue_Impl: super::IDispatch_Impl {
    fn get_Value(&self) -> windows_core::Result<super::VARIANT>;
    fn put_Value(&self, varvalue: *const super::VARIANT) -> windows_core::Result<()>;
    fn get_Name(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl ISWbemNamedValue_Vtbl {
    pub const fn new<Identity: ISWbemNamedValue_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_Value<Identity: ISWbemNamedValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varvalue: *mut super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemNamedValue_Impl::get_Value(this) {
                    Ok(ok__) => {
                        varvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Value<Identity: ISWbemNamedValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varvalue: *const super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISWbemNamedValue_Impl::put_Value(this, core::mem::transmute_copy(&varvalue)).into()
            }
        }
        unsafe extern "system" fn get_Name<Identity: ISWbemNamedValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemNamedValue_Impl::get_Name(this) {
                    Ok(ok__) => {
                        strname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_Value: get_Value::<Identity, OFFSET>,
            put_Value: put_Value::<Identity, OFFSET>,
            get_Name: get_Name::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISWbemNamedValue as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for ISWbemNamedValue {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(ISWbemNamedValueSet, ISWbemNamedValueSet_Vtbl, 0xcf2376ea_ce8c_11d1_8b05_00600806d9b6);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for ISWbemNamedValueSet {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(ISWbemNamedValueSet, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl ISWbemNamedValueSet {
    pub unsafe fn get__NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get__NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn Item(&self, strname: &windows_core::BSTR, iflags: i32) -> windows_core::Result<ISWbemNamedValue> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Item)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strname), iflags, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn Add(&self, strname: &windows_core::BSTR, varvalue: *const super::VARIANT, iflags: i32) -> windows_core::Result<ISWbemNamedValue> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strname), varvalue, iflags, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn Remove(&self, strname: &windows_core::BSTR, iflags: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strname), iflags) }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn DeleteAll(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DeleteAll)(windows_core::Interface::as_raw(self)) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct ISWbemNamedValueSet_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub get__NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Item: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const super::VARIANT, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    Add: usize,
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DeleteAll: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait ISWbemNamedValueSet_Impl: super::IDispatch_Impl {
    fn get__NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Item(&self, strname: &windows_core::BSTR, iflags: i32) -> windows_core::Result<ISWbemNamedValue>;
    fn get_Count(&self) -> windows_core::Result<i32>;
    fn Add(&self, strname: &windows_core::BSTR, varvalue: *const super::VARIANT, iflags: i32) -> windows_core::Result<ISWbemNamedValue>;
    fn Remove(&self, strname: &windows_core::BSTR, iflags: i32) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<ISWbemNamedValueSet>;
    fn DeleteAll(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl ISWbemNamedValueSet_Vtbl {
    pub const fn new<Identity: ISWbemNamedValueSet_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get__NewEnum<Identity: ISWbemNamedValueSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemNamedValueSet_Impl::get__NewEnum(this) {
                    Ok(ok__) => {
                        punk.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Item<Identity: ISWbemNamedValueSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strname: *mut core::ffi::c_void, iflags: i32, objwbemnamedvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemNamedValueSet_Impl::Item(this, core::mem::transmute(&strname), core::mem::transmute_copy(&iflags)) {
                    Ok(ok__) => {
                        objwbemnamedvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Count<Identity: ISWbemNamedValueSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, icount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemNamedValueSet_Impl::get_Count(this) {
                    Ok(ok__) => {
                        icount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Add<Identity: ISWbemNamedValueSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strname: *mut core::ffi::c_void, varvalue: *const super::VARIANT, iflags: i32, objwbemnamedvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemNamedValueSet_Impl::Add(this, core::mem::transmute(&strname), core::mem::transmute_copy(&varvalue), core::mem::transmute_copy(&iflags)) {
                    Ok(ok__) => {
                        objwbemnamedvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Remove<Identity: ISWbemNamedValueSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strname: *mut core::ffi::c_void, iflags: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISWbemNamedValueSet_Impl::Remove(this, core::mem::transmute(&strname), core::mem::transmute_copy(&iflags)).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: ISWbemNamedValueSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemnamedvalueset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemNamedValueSet_Impl::Clone(this) {
                    Ok(ok__) => {
                        objwbemnamedvalueset.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DeleteAll<Identity: ISWbemNamedValueSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISWbemNamedValueSet_Impl::DeleteAll(this).into()
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get__NewEnum: get__NewEnum::<Identity, OFFSET>,
            Item: Item::<Identity, OFFSET>,
            get_Count: get_Count::<Identity, OFFSET>,
            Add: Add::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
            DeleteAll: DeleteAll::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISWbemNamedValueSet as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for ISWbemNamedValueSet {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(ISWbemObject, ISWbemObject_Vtbl, 0x76a6415a_cb41_11d1_8b02_00600806d9b6);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for ISWbemObject {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(ISWbemObject, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl ISWbemObject {
    pub unsafe fn Put_<P1>(&self, iflags: i32, objwbemnamedvalueset: P1) -> windows_core::Result<ISWbemObjectPath>
    where
        P1: windows_core::Param<super::IDispatch>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Put_)(windows_core::Interface::as_raw(self), iflags, objwbemnamedvalueset.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn PutAsync_<P0, P2, P3>(&self, objwbemsink: P0, iflags: i32, objwbemnamedvalueset: P2, objwbemasynccontext: P3) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::IDispatch>,
        P2: windows_core::Param<super::IDispatch>,
        P3: windows_core::Param<super::IDispatch>,
    {
        unsafe { (windows_core::Interface::vtable(self).PutAsync_)(windows_core::Interface::as_raw(self), objwbemsink.param().abi(), iflags, objwbemnamedvalueset.param().abi(), objwbemasynccontext.param().abi()) }
    }
    pub unsafe fn Delete_<P1>(&self, iflags: i32, objwbemnamedvalueset: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<super::IDispatch>,
    {
        unsafe { (windows_core::Interface::vtable(self).Delete_)(windows_core::Interface::as_raw(self), iflags, objwbemnamedvalueset.param().abi()) }
    }
    pub unsafe fn DeleteAsync_<P0, P2, P3>(&self, objwbemsink: P0, iflags: i32, objwbemnamedvalueset: P2, objwbemasynccontext: P3) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::IDispatch>,
        P2: windows_core::Param<super::IDispatch>,
        P3: windows_core::Param<super::IDispatch>,
    {
        unsafe { (windows_core::Interface::vtable(self).DeleteAsync_)(windows_core::Interface::as_raw(self), objwbemsink.param().abi(), iflags, objwbemnamedvalueset.param().abi(), objwbemasynccontext.param().abi()) }
    }
    pub unsafe fn Instances_<P1>(&self, iflags: i32, objwbemnamedvalueset: P1) -> windows_core::Result<ISWbemObjectSet>
    where
        P1: windows_core::Param<super::IDispatch>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Instances_)(windows_core::Interface::as_raw(self), iflags, objwbemnamedvalueset.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn InstancesAsync_<P0, P2, P3>(&self, objwbemsink: P0, iflags: i32, objwbemnamedvalueset: P2, objwbemasynccontext: P3) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::IDispatch>,
        P2: windows_core::Param<super::IDispatch>,
        P3: windows_core::Param<super::IDispatch>,
    {
        unsafe { (windows_core::Interface::vtable(self).InstancesAsync_)(windows_core::Interface::as_raw(self), objwbemsink.param().abi(), iflags, objwbemnamedvalueset.param().abi(), objwbemasynccontext.param().abi()) }
    }
    pub unsafe fn Subclasses_<P1>(&self, iflags: i32, objwbemnamedvalueset: P1) -> windows_core::Result<ISWbemObjectSet>
    where
        P1: windows_core::Param<super::IDispatch>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Subclasses_)(windows_core::Interface::as_raw(self), iflags, objwbemnamedvalueset.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn SubclassesAsync_<P0, P2, P3>(&self, objwbemsink: P0, iflags: i32, objwbemnamedvalueset: P2, objwbemasynccontext: P3) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::IDispatch>,
        P2: windows_core::Param<super::IDispatch>,
        P3: windows_core::Param<super::IDispatch>,
    {
        unsafe { (windows_core::Interface::vtable(self).SubclassesAsync_)(windows_core::Interface::as_raw(self), objwbemsink.param().abi(), iflags, objwbemnamedvalueset.param().abi(), objwbemasynccontext.param().abi()) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn Associators_<P9>(&self, strassocclass: &windows_core::BSTR, strresultclass: &windows_core::BSTR, strresultrole: &windows_core::BSTR, strrole: &windows_core::BSTR, bclassesonly: super::VARIANT_BOOL, bschemaonly: super::VARIANT_BOOL, strrequiredassocqualifier: &windows_core::BSTR, strrequiredqualifier: &windows_core::BSTR, iflags: i32, objwbemnamedvalueset: P9) -> windows_core::Result<ISWbemObjectSet>
    where
        P9: windows_core::Param<super::IDispatch>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Associators_)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strassocclass), core::mem::transmute_copy(strresultclass), core::mem::transmute_copy(strresultrole), core::mem::transmute_copy(strrole), bclassesonly, bschemaonly, core::mem::transmute_copy(strrequiredassocqualifier), core::mem::transmute_copy(strrequiredqualifier), iflags, objwbemnamedvalueset.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn AssociatorsAsync_<P0, P10, P11>(&self, objwbemsink: P0, strassocclass: &windows_core::BSTR, strresultclass: &windows_core::BSTR, strresultrole: &windows_core::BSTR, strrole: &windows_core::BSTR, bclassesonly: super::VARIANT_BOOL, bschemaonly: super::VARIANT_BOOL, strrequiredassocqualifier: &windows_core::BSTR, strrequiredqualifier: &windows_core::BSTR, iflags: i32, objwbemnamedvalueset: P10, objwbemasynccontext: P11) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::IDispatch>,
        P10: windows_core::Param<super::IDispatch>,
        P11: windows_core::Param<super::IDispatch>,
    {
        unsafe { (windows_core::Interface::vtable(self).AssociatorsAsync_)(windows_core::Interface::as_raw(self), objwbemsink.param().abi(), core::mem::transmute_copy(strassocclass), core::mem::transmute_copy(strresultclass), core::mem::transmute_copy(strresultrole), core::mem::transmute_copy(strrole), bclassesonly, bschemaonly, core::mem::transmute_copy(strrequiredassocqualifier), core::mem::transmute_copy(strrequiredqualifier), iflags, objwbemnamedvalueset.param().abi(), objwbemasynccontext.param().abi()) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn References_<P6>(&self, strresultclass: &windows_core::BSTR, strrole: &windows_core::BSTR, bclassesonly: super::VARIANT_BOOL, bschemaonly: super::VARIANT_BOOL, strrequiredqualifier: &windows_core::BSTR, iflags: i32, objwbemnamedvalueset: P6) -> windows_core::Result<ISWbemObjectSet>
    where
        P6: windows_core::Param<super::IDispatch>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).References_)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strresultclass), core::mem::transmute_copy(strrole), bclassesonly, bschemaonly, core::mem::transmute_copy(strrequiredqualifier), iflags, objwbemnamedvalueset.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn ReferencesAsync_<P0, P7, P8>(&self, objwbemsink: P0, strresultclass: &windows_core::BSTR, strrole: &windows_core::BSTR, bclassesonly: super::VARIANT_BOOL, bschemaonly: super::VARIANT_BOOL, strrequiredqualifier: &windows_core::BSTR, iflags: i32, objwbemnamedvalueset: P7, objwbemasynccontext: P8) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::IDispatch>,
        P7: windows_core::Param<super::IDispatch>,
        P8: windows_core::Param<super::IDispatch>,
    {
        unsafe { (windows_core::Interface::vtable(self).ReferencesAsync_)(windows_core::Interface::as_raw(self), objwbemsink.param().abi(), core::mem::transmute_copy(strresultclass), core::mem::transmute_copy(strrole), bclassesonly, bschemaonly, core::mem::transmute_copy(strrequiredqualifier), iflags, objwbemnamedvalueset.param().abi(), objwbemasynccontext.param().abi()) }
    }
    pub unsafe fn ExecMethod_<P1, P3>(&self, strmethodname: &windows_core::BSTR, objwbeminparameters: P1, iflags: i32, objwbemnamedvalueset: P3) -> windows_core::Result<Self>
    where
        P1: windows_core::Param<super::IDispatch>,
        P3: windows_core::Param<super::IDispatch>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ExecMethod_)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strmethodname), objwbeminparameters.param().abi(), iflags, objwbemnamedvalueset.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn ExecMethodAsync_<P0, P2, P4, P5>(&self, objwbemsink: P0, strmethodname: &windows_core::BSTR, objwbeminparameters: P2, iflags: i32, objwbemnamedvalueset: P4, objwbemasynccontext: P5) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::IDispatch>,
        P2: windows_core::Param<super::IDispatch>,
        P4: windows_core::Param<super::IDispatch>,
        P5: windows_core::Param<super::IDispatch>,
    {
        unsafe { (windows_core::Interface::vtable(self).ExecMethodAsync_)(windows_core::Interface::as_raw(self), objwbemsink.param().abi(), core::mem::transmute_copy(strmethodname), objwbeminparameters.param().abi(), iflags, objwbemnamedvalueset.param().abi(), objwbemasynccontext.param().abi()) }
    }
    pub unsafe fn Clone_(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone_)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetObjectText_(&self, iflags: i32) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetObjectText_)(windows_core::Interface::as_raw(self), iflags, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SpawnDerivedClass_(&self, iflags: i32) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SpawnDerivedClass_)(windows_core::Interface::as_raw(self), iflags, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn SpawnInstance_(&self, iflags: i32) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SpawnInstance_)(windows_core::Interface::as_raw(self), iflags, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn CompareTo_<P0>(&self, objwbemobject: P0, iflags: i32) -> windows_core::Result<super::VARIANT_BOOL>
    where
        P0: windows_core::Param<super::IDispatch>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CompareTo_)(windows_core::Interface::as_raw(self), objwbemobject.param().abi(), iflags, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_Qualifiers_(&self) -> windows_core::Result<ISWbemQualifierSet> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Qualifiers_)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_Properties_(&self) -> windows_core::Result<ISWbemPropertySet> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Properties_)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_Methods_(&self) -> windows_core::Result<ISWbemMethodSet> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Methods_)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn get_Derivation_(&self) -> windows_core::Result<super::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Derivation_)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_Path_(&self) -> windows_core::Result<ISWbemObjectPath> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Path_)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_Security_(&self) -> windows_core::Result<ISWbemSecurity> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Security_)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct ISWbemObject_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub Put_: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PutAsync_: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Delete_: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DeleteAsync_: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Instances_: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub InstancesAsync_: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Subclasses_: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SubclassesAsync_: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub Associators_: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, super::VARIANT_BOOL, super::VARIANT_BOOL, *mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    Associators_: usize,
    #[cfg(feature = "wtypes")]
    pub AssociatorsAsync_: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, super::VARIANT_BOOL, super::VARIANT_BOOL, *mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    AssociatorsAsync_: usize,
    #[cfg(feature = "wtypes")]
    pub References_: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, super::VARIANT_BOOL, super::VARIANT_BOOL, *mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    References_: usize,
    #[cfg(feature = "wtypes")]
    pub ReferencesAsync_: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, super::VARIANT_BOOL, super::VARIANT_BOOL, *mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    ReferencesAsync_: usize,
    pub ExecMethod_: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ExecMethodAsync_: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone_: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetObjectText_: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SpawnDerivedClass_: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SpawnInstance_: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub CompareTo_: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    CompareTo_: usize,
    pub get_Qualifiers_: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Properties_: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Methods_: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub get_Derivation_: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    get_Derivation_: usize,
    pub get_Path_: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Security_: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait ISWbemObject_Impl: super::IDispatch_Impl {
    fn Put_(&self, iflags: i32, objwbemnamedvalueset: windows_core::Ref<super::IDispatch>) -> windows_core::Result<ISWbemObjectPath>;
    fn PutAsync_(&self, objwbemsink: windows_core::Ref<super::IDispatch>, iflags: i32, objwbemnamedvalueset: windows_core::Ref<super::IDispatch>, objwbemasynccontext: windows_core::Ref<super::IDispatch>) -> windows_core::Result<()>;
    fn Delete_(&self, iflags: i32, objwbemnamedvalueset: windows_core::Ref<super::IDispatch>) -> windows_core::Result<()>;
    fn DeleteAsync_(&self, objwbemsink: windows_core::Ref<super::IDispatch>, iflags: i32, objwbemnamedvalueset: windows_core::Ref<super::IDispatch>, objwbemasynccontext: windows_core::Ref<super::IDispatch>) -> windows_core::Result<()>;
    fn Instances_(&self, iflags: i32, objwbemnamedvalueset: windows_core::Ref<super::IDispatch>) -> windows_core::Result<ISWbemObjectSet>;
    fn InstancesAsync_(&self, objwbemsink: windows_core::Ref<super::IDispatch>, iflags: i32, objwbemnamedvalueset: windows_core::Ref<super::IDispatch>, objwbemasynccontext: windows_core::Ref<super::IDispatch>) -> windows_core::Result<()>;
    fn Subclasses_(&self, iflags: i32, objwbemnamedvalueset: windows_core::Ref<super::IDispatch>) -> windows_core::Result<ISWbemObjectSet>;
    fn SubclassesAsync_(&self, objwbemsink: windows_core::Ref<super::IDispatch>, iflags: i32, objwbemnamedvalueset: windows_core::Ref<super::IDispatch>, objwbemasynccontext: windows_core::Ref<super::IDispatch>) -> windows_core::Result<()>;
    fn Associators_(&self, strassocclass: &windows_core::BSTR, strresultclass: &windows_core::BSTR, strresultrole: &windows_core::BSTR, strrole: &windows_core::BSTR, bclassesonly: super::VARIANT_BOOL, bschemaonly: super::VARIANT_BOOL, strrequiredassocqualifier: &windows_core::BSTR, strrequiredqualifier: &windows_core::BSTR, iflags: i32, objwbemnamedvalueset: windows_core::Ref<super::IDispatch>) -> windows_core::Result<ISWbemObjectSet>;
    fn AssociatorsAsync_(&self, objwbemsink: windows_core::Ref<super::IDispatch>, strassocclass: &windows_core::BSTR, strresultclass: &windows_core::BSTR, strresultrole: &windows_core::BSTR, strrole: &windows_core::BSTR, bclassesonly: super::VARIANT_BOOL, bschemaonly: super::VARIANT_BOOL, strrequiredassocqualifier: &windows_core::BSTR, strrequiredqualifier: &windows_core::BSTR, iflags: i32, objwbemnamedvalueset: windows_core::Ref<super::IDispatch>, objwbemasynccontext: windows_core::Ref<super::IDispatch>) -> windows_core::Result<()>;
    fn References_(&self, strresultclass: &windows_core::BSTR, strrole: &windows_core::BSTR, bclassesonly: super::VARIANT_BOOL, bschemaonly: super::VARIANT_BOOL, strrequiredqualifier: &windows_core::BSTR, iflags: i32, objwbemnamedvalueset: windows_core::Ref<super::IDispatch>) -> windows_core::Result<ISWbemObjectSet>;
    fn ReferencesAsync_(&self, objwbemsink: windows_core::Ref<super::IDispatch>, strresultclass: &windows_core::BSTR, strrole: &windows_core::BSTR, bclassesonly: super::VARIANT_BOOL, bschemaonly: super::VARIANT_BOOL, strrequiredqualifier: &windows_core::BSTR, iflags: i32, objwbemnamedvalueset: windows_core::Ref<super::IDispatch>, objwbemasynccontext: windows_core::Ref<super::IDispatch>) -> windows_core::Result<()>;
    fn ExecMethod_(&self, strmethodname: &windows_core::BSTR, objwbeminparameters: windows_core::Ref<super::IDispatch>, iflags: i32, objwbemnamedvalueset: windows_core::Ref<super::IDispatch>) -> windows_core::Result<ISWbemObject>;
    fn ExecMethodAsync_(&self, objwbemsink: windows_core::Ref<super::IDispatch>, strmethodname: &windows_core::BSTR, objwbeminparameters: windows_core::Ref<super::IDispatch>, iflags: i32, objwbemnamedvalueset: windows_core::Ref<super::IDispatch>, objwbemasynccontext: windows_core::Ref<super::IDispatch>) -> windows_core::Result<()>;
    fn Clone_(&self) -> windows_core::Result<ISWbemObject>;
    fn GetObjectText_(&self, iflags: i32) -> windows_core::Result<windows_core::BSTR>;
    fn SpawnDerivedClass_(&self, iflags: i32) -> windows_core::Result<ISWbemObject>;
    fn SpawnInstance_(&self, iflags: i32) -> windows_core::Result<ISWbemObject>;
    fn CompareTo_(&self, objwbemobject: windows_core::Ref<super::IDispatch>, iflags: i32) -> windows_core::Result<super::VARIANT_BOOL>;
    fn get_Qualifiers_(&self) -> windows_core::Result<ISWbemQualifierSet>;
    fn get_Properties_(&self) -> windows_core::Result<ISWbemPropertySet>;
    fn get_Methods_(&self) -> windows_core::Result<ISWbemMethodSet>;
    fn get_Derivation_(&self) -> windows_core::Result<super::VARIANT>;
    fn get_Path_(&self) -> windows_core::Result<ISWbemObjectPath>;
    fn get_Security_(&self) -> windows_core::Result<ISWbemSecurity>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl ISWbemObject_Vtbl {
    pub const fn new<Identity: ISWbemObject_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Put_<Identity: ISWbemObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void, objwbemobjectpath: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemObject_Impl::Put_(this, core::mem::transmute_copy(&iflags), core::mem::transmute_copy(&objwbemnamedvalueset)) {
                    Ok(ok__) => {
                        objwbemobjectpath.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PutAsync_<Identity: ISWbemObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemsink: *mut core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void, objwbemasynccontext: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISWbemObject_Impl::PutAsync_(this, core::mem::transmute_copy(&objwbemsink), core::mem::transmute_copy(&iflags), core::mem::transmute_copy(&objwbemnamedvalueset), core::mem::transmute_copy(&objwbemasynccontext)).into()
            }
        }
        unsafe extern "system" fn Delete_<Identity: ISWbemObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISWbemObject_Impl::Delete_(this, core::mem::transmute_copy(&iflags), core::mem::transmute_copy(&objwbemnamedvalueset)).into()
            }
        }
        unsafe extern "system" fn DeleteAsync_<Identity: ISWbemObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemsink: *mut core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void, objwbemasynccontext: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISWbemObject_Impl::DeleteAsync_(this, core::mem::transmute_copy(&objwbemsink), core::mem::transmute_copy(&iflags), core::mem::transmute_copy(&objwbemnamedvalueset), core::mem::transmute_copy(&objwbemasynccontext)).into()
            }
        }
        unsafe extern "system" fn Instances_<Identity: ISWbemObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void, objwbemobjectset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemObject_Impl::Instances_(this, core::mem::transmute_copy(&iflags), core::mem::transmute_copy(&objwbemnamedvalueset)) {
                    Ok(ok__) => {
                        objwbemobjectset.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn InstancesAsync_<Identity: ISWbemObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemsink: *mut core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void, objwbemasynccontext: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISWbemObject_Impl::InstancesAsync_(this, core::mem::transmute_copy(&objwbemsink), core::mem::transmute_copy(&iflags), core::mem::transmute_copy(&objwbemnamedvalueset), core::mem::transmute_copy(&objwbemasynccontext)).into()
            }
        }
        unsafe extern "system" fn Subclasses_<Identity: ISWbemObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void, objwbemobjectset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemObject_Impl::Subclasses_(this, core::mem::transmute_copy(&iflags), core::mem::transmute_copy(&objwbemnamedvalueset)) {
                    Ok(ok__) => {
                        objwbemobjectset.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SubclassesAsync_<Identity: ISWbemObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemsink: *mut core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void, objwbemasynccontext: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISWbemObject_Impl::SubclassesAsync_(this, core::mem::transmute_copy(&objwbemsink), core::mem::transmute_copy(&iflags), core::mem::transmute_copy(&objwbemnamedvalueset), core::mem::transmute_copy(&objwbemasynccontext)).into()
            }
        }
        unsafe extern "system" fn Associators_<Identity: ISWbemObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strassocclass: *mut core::ffi::c_void, strresultclass: *mut core::ffi::c_void, strresultrole: *mut core::ffi::c_void, strrole: *mut core::ffi::c_void, bclassesonly: super::VARIANT_BOOL, bschemaonly: super::VARIANT_BOOL, strrequiredassocqualifier: *mut core::ffi::c_void, strrequiredqualifier: *mut core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void, objwbemobjectset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemObject_Impl::Associators_(this, core::mem::transmute(&strassocclass), core::mem::transmute(&strresultclass), core::mem::transmute(&strresultrole), core::mem::transmute(&strrole), core::mem::transmute_copy(&bclassesonly), core::mem::transmute_copy(&bschemaonly), core::mem::transmute(&strrequiredassocqualifier), core::mem::transmute(&strrequiredqualifier), core::mem::transmute_copy(&iflags), core::mem::transmute_copy(&objwbemnamedvalueset)) {
                    Ok(ok__) => {
                        objwbemobjectset.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AssociatorsAsync_<Identity: ISWbemObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemsink: *mut core::ffi::c_void, strassocclass: *mut core::ffi::c_void, strresultclass: *mut core::ffi::c_void, strresultrole: *mut core::ffi::c_void, strrole: *mut core::ffi::c_void, bclassesonly: super::VARIANT_BOOL, bschemaonly: super::VARIANT_BOOL, strrequiredassocqualifier: *mut core::ffi::c_void, strrequiredqualifier: *mut core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void, objwbemasynccontext: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISWbemObject_Impl::AssociatorsAsync_(
                    this,
                    core::mem::transmute_copy(&objwbemsink),
                    core::mem::transmute(&strassocclass),
                    core::mem::transmute(&strresultclass),
                    core::mem::transmute(&strresultrole),
                    core::mem::transmute(&strrole),
                    core::mem::transmute_copy(&bclassesonly),
                    core::mem::transmute_copy(&bschemaonly),
                    core::mem::transmute(&strrequiredassocqualifier),
                    core::mem::transmute(&strrequiredqualifier),
                    core::mem::transmute_copy(&iflags),
                    core::mem::transmute_copy(&objwbemnamedvalueset),
                    core::mem::transmute_copy(&objwbemasynccontext),
                )
                .into()
            }
        }
        unsafe extern "system" fn References_<Identity: ISWbemObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strresultclass: *mut core::ffi::c_void, strrole: *mut core::ffi::c_void, bclassesonly: super::VARIANT_BOOL, bschemaonly: super::VARIANT_BOOL, strrequiredqualifier: *mut core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void, objwbemobjectset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemObject_Impl::References_(this, core::mem::transmute(&strresultclass), core::mem::transmute(&strrole), core::mem::transmute_copy(&bclassesonly), core::mem::transmute_copy(&bschemaonly), core::mem::transmute(&strrequiredqualifier), core::mem::transmute_copy(&iflags), core::mem::transmute_copy(&objwbemnamedvalueset)) {
                    Ok(ok__) => {
                        objwbemobjectset.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ReferencesAsync_<Identity: ISWbemObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemsink: *mut core::ffi::c_void, strresultclass: *mut core::ffi::c_void, strrole: *mut core::ffi::c_void, bclassesonly: super::VARIANT_BOOL, bschemaonly: super::VARIANT_BOOL, strrequiredqualifier: *mut core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void, objwbemasynccontext: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISWbemObject_Impl::ReferencesAsync_(this, core::mem::transmute_copy(&objwbemsink), core::mem::transmute(&strresultclass), core::mem::transmute(&strrole), core::mem::transmute_copy(&bclassesonly), core::mem::transmute_copy(&bschemaonly), core::mem::transmute(&strrequiredqualifier), core::mem::transmute_copy(&iflags), core::mem::transmute_copy(&objwbemnamedvalueset), core::mem::transmute_copy(&objwbemasynccontext)).into()
            }
        }
        unsafe extern "system" fn ExecMethod_<Identity: ISWbemObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strmethodname: *mut core::ffi::c_void, objwbeminparameters: *mut core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void, objwbemoutparameters: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemObject_Impl::ExecMethod_(this, core::mem::transmute(&strmethodname), core::mem::transmute_copy(&objwbeminparameters), core::mem::transmute_copy(&iflags), core::mem::transmute_copy(&objwbemnamedvalueset)) {
                    Ok(ok__) => {
                        objwbemoutparameters.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ExecMethodAsync_<Identity: ISWbemObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemsink: *mut core::ffi::c_void, strmethodname: *mut core::ffi::c_void, objwbeminparameters: *mut core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void, objwbemasynccontext: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISWbemObject_Impl::ExecMethodAsync_(this, core::mem::transmute_copy(&objwbemsink), core::mem::transmute(&strmethodname), core::mem::transmute_copy(&objwbeminparameters), core::mem::transmute_copy(&iflags), core::mem::transmute_copy(&objwbemnamedvalueset), core::mem::transmute_copy(&objwbemasynccontext)).into()
            }
        }
        unsafe extern "system" fn Clone_<Identity: ISWbemObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemObject_Impl::Clone_(this) {
                    Ok(ok__) => {
                        objwbemobject.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetObjectText_<Identity: ISWbemObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iflags: i32, strobjecttext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemObject_Impl::GetObjectText_(this, core::mem::transmute_copy(&iflags)) {
                    Ok(ok__) => {
                        strobjecttext.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SpawnDerivedClass_<Identity: ISWbemObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iflags: i32, objwbemobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemObject_Impl::SpawnDerivedClass_(this, core::mem::transmute_copy(&iflags)) {
                    Ok(ok__) => {
                        objwbemobject.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SpawnInstance_<Identity: ISWbemObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iflags: i32, objwbemobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemObject_Impl::SpawnInstance_(this, core::mem::transmute_copy(&iflags)) {
                    Ok(ok__) => {
                        objwbemobject.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CompareTo_<Identity: ISWbemObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemobject: *mut core::ffi::c_void, iflags: i32, bresult: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemObject_Impl::CompareTo_(this, core::mem::transmute_copy(&objwbemobject), core::mem::transmute_copy(&iflags)) {
                    Ok(ok__) => {
                        bresult.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Qualifiers_<Identity: ISWbemObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemqualifierset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemObject_Impl::get_Qualifiers_(this) {
                    Ok(ok__) => {
                        objwbemqualifierset.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Properties_<Identity: ISWbemObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbempropertyset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemObject_Impl::get_Properties_(this) {
                    Ok(ok__) => {
                        objwbempropertyset.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Methods_<Identity: ISWbemObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemmethodset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemObject_Impl::get_Methods_(this) {
                    Ok(ok__) => {
                        objwbemmethodset.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Derivation_<Identity: ISWbemObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strclassnamearray: *mut super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemObject_Impl::get_Derivation_(this) {
                    Ok(ok__) => {
                        strclassnamearray.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Path_<Identity: ISWbemObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemobjectpath: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemObject_Impl::get_Path_(this) {
                    Ok(ok__) => {
                        objwbemobjectpath.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Security_<Identity: ISWbemObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemsecurity: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemObject_Impl::get_Security_(this) {
                    Ok(ok__) => {
                        objwbemsecurity.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Put_: Put_::<Identity, OFFSET>,
            PutAsync_: PutAsync_::<Identity, OFFSET>,
            Delete_: Delete_::<Identity, OFFSET>,
            DeleteAsync_: DeleteAsync_::<Identity, OFFSET>,
            Instances_: Instances_::<Identity, OFFSET>,
            InstancesAsync_: InstancesAsync_::<Identity, OFFSET>,
            Subclasses_: Subclasses_::<Identity, OFFSET>,
            SubclassesAsync_: SubclassesAsync_::<Identity, OFFSET>,
            Associators_: Associators_::<Identity, OFFSET>,
            AssociatorsAsync_: AssociatorsAsync_::<Identity, OFFSET>,
            References_: References_::<Identity, OFFSET>,
            ReferencesAsync_: ReferencesAsync_::<Identity, OFFSET>,
            ExecMethod_: ExecMethod_::<Identity, OFFSET>,
            ExecMethodAsync_: ExecMethodAsync_::<Identity, OFFSET>,
            Clone_: Clone_::<Identity, OFFSET>,
            GetObjectText_: GetObjectText_::<Identity, OFFSET>,
            SpawnDerivedClass_: SpawnDerivedClass_::<Identity, OFFSET>,
            SpawnInstance_: SpawnInstance_::<Identity, OFFSET>,
            CompareTo_: CompareTo_::<Identity, OFFSET>,
            get_Qualifiers_: get_Qualifiers_::<Identity, OFFSET>,
            get_Properties_: get_Properties_::<Identity, OFFSET>,
            get_Methods_: get_Methods_::<Identity, OFFSET>,
            get_Derivation_: get_Derivation_::<Identity, OFFSET>,
            get_Path_: get_Path_::<Identity, OFFSET>,
            get_Security_: get_Security_::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISWbemObject as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for ISWbemObject {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(ISWbemObjectEx, ISWbemObjectEx_Vtbl, 0x269ad56a_8a67_4129_bc8c_0506dcfe9880);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for ISWbemObjectEx {
    type Target = ISWbemObject;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(ISWbemObjectEx, windows_core::IUnknown, super::IDispatch, ISWbemObject);
#[cfg(feature = "oaidl")]
impl ISWbemObjectEx {
    pub unsafe fn Refresh_<P1>(&self, iflags: i32, objwbemnamedvalueset: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<super::IDispatch>,
    {
        unsafe { (windows_core::Interface::vtable(self).Refresh_)(windows_core::Interface::as_raw(self), iflags, objwbemnamedvalueset.param().abi()) }
    }
    pub unsafe fn get_SystemProperties_(&self) -> windows_core::Result<ISWbemPropertySet> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_SystemProperties_)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetText_<P2>(&self, iobjecttextformat: WbemObjectTextFormatEnum, iflags: i32, objwbemnamedvalueset: P2) -> windows_core::Result<windows_core::BSTR>
    where
        P2: windows_core::Param<super::IDispatch>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetText_)(windows_core::Interface::as_raw(self), iobjecttextformat, iflags, objwbemnamedvalueset.param().abi(), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetFromText_<P3>(&self, bstext: &windows_core::BSTR, iobjecttextformat: WbemObjectTextFormatEnum, iflags: i32, objwbemnamedvalueset: P3) -> windows_core::HRESULT
    where
        P3: windows_core::Param<super::IDispatch>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetFromText_)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstext), iobjecttextformat, iflags, objwbemnamedvalueset.param().abi()) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct ISWbemObjectEx_Vtbl {
    pub base__: ISWbemObject_Vtbl,
    pub Refresh_: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_SystemProperties_: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetText_: unsafe extern "system" fn(*mut core::ffi::c_void, WbemObjectTextFormatEnum, i32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetFromText_: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, WbemObjectTextFormatEnum, i32, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait ISWbemObjectEx_Impl: ISWbemObject_Impl {
    fn Refresh_(&self, iflags: i32, objwbemnamedvalueset: windows_core::Ref<super::IDispatch>) -> windows_core::Result<()>;
    fn get_SystemProperties_(&self) -> windows_core::Result<ISWbemPropertySet>;
    fn GetText_(&self, iobjecttextformat: WbemObjectTextFormatEnum, iflags: i32, objwbemnamedvalueset: windows_core::Ref<super::IDispatch>) -> windows_core::Result<windows_core::BSTR>;
    fn SetFromText_(&self, bstext: &windows_core::BSTR, iobjecttextformat: WbemObjectTextFormatEnum, iflags: i32, objwbemnamedvalueset: windows_core::Ref<super::IDispatch>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl ISWbemObjectEx_Vtbl {
    pub const fn new<Identity: ISWbemObjectEx_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Refresh_<Identity: ISWbemObjectEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISWbemObjectEx_Impl::Refresh_(this, core::mem::transmute_copy(&iflags), core::mem::transmute_copy(&objwbemnamedvalueset)).into()
            }
        }
        unsafe extern "system" fn get_SystemProperties_<Identity: ISWbemObjectEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbempropertyset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemObjectEx_Impl::get_SystemProperties_(this) {
                    Ok(ok__) => {
                        objwbempropertyset.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetText_<Identity: ISWbemObjectEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iobjecttextformat: WbemObjectTextFormatEnum, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void, bstext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemObjectEx_Impl::GetText_(this, core::mem::transmute_copy(&iobjecttextformat), core::mem::transmute_copy(&iflags), core::mem::transmute_copy(&objwbemnamedvalueset)) {
                    Ok(ok__) => {
                        bstext.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetFromText_<Identity: ISWbemObjectEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstext: *mut core::ffi::c_void, iobjecttextformat: WbemObjectTextFormatEnum, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISWbemObjectEx_Impl::SetFromText_(this, core::mem::transmute(&bstext), core::mem::transmute_copy(&iobjecttextformat), core::mem::transmute_copy(&iflags), core::mem::transmute_copy(&objwbemnamedvalueset)).into()
            }
        }
        Self {
            base__: ISWbemObject_Vtbl::new::<Identity, OFFSET>(),
            Refresh_: Refresh_::<Identity, OFFSET>,
            get_SystemProperties_: get_SystemProperties_::<Identity, OFFSET>,
            GetText_: GetText_::<Identity, OFFSET>,
            SetFromText_: SetFromText_::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISWbemObjectEx as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<ISWbemObject as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for ISWbemObjectEx {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(ISWbemObjectPath, ISWbemObjectPath_Vtbl, 0x5791bc27_ce9c_11d1_97bf_0000f81e849c);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for ISWbemObjectPath {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(ISWbemObjectPath, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl ISWbemObjectPath {
    pub unsafe fn get_Path(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Path)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_Path(&self, strpath: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Path)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strpath)) }
    }
    pub unsafe fn get_RelPath(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_RelPath)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_RelPath(&self, strrelpath: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_RelPath)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strrelpath)) }
    }
    pub unsafe fn get_Server(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Server)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_Server(&self, strserver: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Server)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strserver)) }
    }
    pub unsafe fn get_Namespace(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Namespace)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_Namespace(&self, strnamespace: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Namespace)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strnamespace)) }
    }
    pub unsafe fn get_ParentNamespace(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ParentNamespace)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_DisplayName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_DisplayName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_DisplayName(&self, strdisplayname: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_DisplayName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strdisplayname)) }
    }
    pub unsafe fn get_Class(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Class)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_Class(&self, strclass: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Class)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strclass)) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_IsClass(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_IsClass)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetAsClass(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetAsClass)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_IsSingleton(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_IsSingleton)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetAsSingleton(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetAsSingleton)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn get_Keys(&self) -> windows_core::Result<ISWbemNamedValueSet> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Keys)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_Security_(&self) -> windows_core::Result<ISWbemSecurity> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Security_)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_Locale(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Locale)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_Locale(&self, strlocale: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Locale)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strlocale)) }
    }
    pub unsafe fn get_Authority(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Authority)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_Authority(&self, strauthority: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Authority)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strauthority)) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct ISWbemObjectPath_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub get_Path: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_Path: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_RelPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_RelPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Server: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_Server: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Namespace: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_Namespace: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_ParentNamespace: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_DisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_DisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Class: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_Class: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub get_IsClass: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_IsClass: usize,
    pub SetAsClass: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub get_IsSingleton: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_IsSingleton: usize,
    pub SetAsSingleton: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Keys: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Security_: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Locale: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_Locale: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Authority: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_Authority: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait ISWbemObjectPath_Impl: super::IDispatch_Impl {
    fn get_Path(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_Path(&self, strpath: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_RelPath(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_RelPath(&self, strrelpath: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_Server(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_Server(&self, strserver: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_Namespace(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_Namespace(&self, strnamespace: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_ParentNamespace(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_DisplayName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_DisplayName(&self, strdisplayname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_Class(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_Class(&self, strclass: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_IsClass(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn SetAsClass(&self) -> windows_core::Result<()>;
    fn get_IsSingleton(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn SetAsSingleton(&self) -> windows_core::Result<()>;
    fn get_Keys(&self) -> windows_core::Result<ISWbemNamedValueSet>;
    fn get_Security_(&self) -> windows_core::Result<ISWbemSecurity>;
    fn get_Locale(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_Locale(&self, strlocale: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_Authority(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_Authority(&self, strauthority: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl ISWbemObjectPath_Vtbl {
    pub const fn new<Identity: ISWbemObjectPath_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_Path<Identity: ISWbemObjectPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strpath: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemObjectPath_Impl::get_Path(this) {
                    Ok(ok__) => {
                        strpath.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Path<Identity: ISWbemObjectPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strpath: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISWbemObjectPath_Impl::put_Path(this, core::mem::transmute(&strpath)).into()
            }
        }
        unsafe extern "system" fn get_RelPath<Identity: ISWbemObjectPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strrelpath: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemObjectPath_Impl::get_RelPath(this) {
                    Ok(ok__) => {
                        strrelpath.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_RelPath<Identity: ISWbemObjectPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strrelpath: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISWbemObjectPath_Impl::put_RelPath(this, core::mem::transmute(&strrelpath)).into()
            }
        }
        unsafe extern "system" fn get_Server<Identity: ISWbemObjectPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strserver: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemObjectPath_Impl::get_Server(this) {
                    Ok(ok__) => {
                        strserver.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Server<Identity: ISWbemObjectPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strserver: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISWbemObjectPath_Impl::put_Server(this, core::mem::transmute(&strserver)).into()
            }
        }
        unsafe extern "system" fn get_Namespace<Identity: ISWbemObjectPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strnamespace: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemObjectPath_Impl::get_Namespace(this) {
                    Ok(ok__) => {
                        strnamespace.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Namespace<Identity: ISWbemObjectPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strnamespace: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISWbemObjectPath_Impl::put_Namespace(this, core::mem::transmute(&strnamespace)).into()
            }
        }
        unsafe extern "system" fn get_ParentNamespace<Identity: ISWbemObjectPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strparentnamespace: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemObjectPath_Impl::get_ParentNamespace(this) {
                    Ok(ok__) => {
                        strparentnamespace.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_DisplayName<Identity: ISWbemObjectPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strdisplayname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemObjectPath_Impl::get_DisplayName(this) {
                    Ok(ok__) => {
                        strdisplayname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_DisplayName<Identity: ISWbemObjectPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strdisplayname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISWbemObjectPath_Impl::put_DisplayName(this, core::mem::transmute(&strdisplayname)).into()
            }
        }
        unsafe extern "system" fn get_Class<Identity: ISWbemObjectPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strclass: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemObjectPath_Impl::get_Class(this) {
                    Ok(ok__) => {
                        strclass.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Class<Identity: ISWbemObjectPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strclass: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISWbemObjectPath_Impl::put_Class(this, core::mem::transmute(&strclass)).into()
            }
        }
        unsafe extern "system" fn get_IsClass<Identity: ISWbemObjectPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bisclass: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemObjectPath_Impl::get_IsClass(this) {
                    Ok(ok__) => {
                        bisclass.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAsClass<Identity: ISWbemObjectPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISWbemObjectPath_Impl::SetAsClass(this).into()
            }
        }
        unsafe extern "system" fn get_IsSingleton<Identity: ISWbemObjectPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bissingleton: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemObjectPath_Impl::get_IsSingleton(this) {
                    Ok(ok__) => {
                        bissingleton.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAsSingleton<Identity: ISWbemObjectPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISWbemObjectPath_Impl::SetAsSingleton(this).into()
            }
        }
        unsafe extern "system" fn get_Keys<Identity: ISWbemObjectPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemnamedvalueset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemObjectPath_Impl::get_Keys(this) {
                    Ok(ok__) => {
                        objwbemnamedvalueset.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Security_<Identity: ISWbemObjectPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemsecurity: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemObjectPath_Impl::get_Security_(this) {
                    Ok(ok__) => {
                        objwbemsecurity.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Locale<Identity: ISWbemObjectPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strlocale: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemObjectPath_Impl::get_Locale(this) {
                    Ok(ok__) => {
                        strlocale.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Locale<Identity: ISWbemObjectPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strlocale: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISWbemObjectPath_Impl::put_Locale(this, core::mem::transmute(&strlocale)).into()
            }
        }
        unsafe extern "system" fn get_Authority<Identity: ISWbemObjectPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strauthority: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemObjectPath_Impl::get_Authority(this) {
                    Ok(ok__) => {
                        strauthority.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Authority<Identity: ISWbemObjectPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strauthority: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISWbemObjectPath_Impl::put_Authority(this, core::mem::transmute(&strauthority)).into()
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_Path: get_Path::<Identity, OFFSET>,
            put_Path: put_Path::<Identity, OFFSET>,
            get_RelPath: get_RelPath::<Identity, OFFSET>,
            put_RelPath: put_RelPath::<Identity, OFFSET>,
            get_Server: get_Server::<Identity, OFFSET>,
            put_Server: put_Server::<Identity, OFFSET>,
            get_Namespace: get_Namespace::<Identity, OFFSET>,
            put_Namespace: put_Namespace::<Identity, OFFSET>,
            get_ParentNamespace: get_ParentNamespace::<Identity, OFFSET>,
            get_DisplayName: get_DisplayName::<Identity, OFFSET>,
            put_DisplayName: put_DisplayName::<Identity, OFFSET>,
            get_Class: get_Class::<Identity, OFFSET>,
            put_Class: put_Class::<Identity, OFFSET>,
            get_IsClass: get_IsClass::<Identity, OFFSET>,
            SetAsClass: SetAsClass::<Identity, OFFSET>,
            get_IsSingleton: get_IsSingleton::<Identity, OFFSET>,
            SetAsSingleton: SetAsSingleton::<Identity, OFFSET>,
            get_Keys: get_Keys::<Identity, OFFSET>,
            get_Security_: get_Security_::<Identity, OFFSET>,
            get_Locale: get_Locale::<Identity, OFFSET>,
            put_Locale: put_Locale::<Identity, OFFSET>,
            get_Authority: get_Authority::<Identity, OFFSET>,
            put_Authority: put_Authority::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISWbemObjectPath as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for ISWbemObjectPath {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(ISWbemObjectSet, ISWbemObjectSet_Vtbl, 0x76a6415f_cb41_11d1_8b02_00600806d9b6);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for ISWbemObjectSet {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(ISWbemObjectSet, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl ISWbemObjectSet {
    pub unsafe fn get__NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get__NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn Item(&self, strobjectpath: &windows_core::BSTR, iflags: i32) -> windows_core::Result<ISWbemObject> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Item)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strobjectpath), iflags, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_Security_(&self) -> windows_core::Result<ISWbemSecurity> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Security_)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn ItemIndex(&self, lindex: i32) -> windows_core::Result<ISWbemObject> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ItemIndex)(windows_core::Interface::as_raw(self), lindex, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct ISWbemObjectSet_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub get__NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Item: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub get_Security_: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ItemIndex: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait ISWbemObjectSet_Impl: super::IDispatch_Impl {
    fn get__NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Item(&self, strobjectpath: &windows_core::BSTR, iflags: i32) -> windows_core::Result<ISWbemObject>;
    fn get_Count(&self) -> windows_core::Result<i32>;
    fn get_Security_(&self) -> windows_core::Result<ISWbemSecurity>;
    fn ItemIndex(&self, lindex: i32) -> windows_core::Result<ISWbemObject>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl ISWbemObjectSet_Vtbl {
    pub const fn new<Identity: ISWbemObjectSet_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get__NewEnum<Identity: ISWbemObjectSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemObjectSet_Impl::get__NewEnum(this) {
                    Ok(ok__) => {
                        punk.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Item<Identity: ISWbemObjectSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strobjectpath: *mut core::ffi::c_void, iflags: i32, objwbemobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemObjectSet_Impl::Item(this, core::mem::transmute(&strobjectpath), core::mem::transmute_copy(&iflags)) {
                    Ok(ok__) => {
                        objwbemobject.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Count<Identity: ISWbemObjectSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, icount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemObjectSet_Impl::get_Count(this) {
                    Ok(ok__) => {
                        icount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Security_<Identity: ISWbemObjectSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemsecurity: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemObjectSet_Impl::get_Security_(this) {
                    Ok(ok__) => {
                        objwbemsecurity.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ItemIndex<Identity: ISWbemObjectSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lindex: i32, objwbemobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemObjectSet_Impl::ItemIndex(this, core::mem::transmute_copy(&lindex)) {
                    Ok(ok__) => {
                        objwbemobject.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get__NewEnum: get__NewEnum::<Identity, OFFSET>,
            Item: Item::<Identity, OFFSET>,
            get_Count: get_Count::<Identity, OFFSET>,
            get_Security_: get_Security_::<Identity, OFFSET>,
            ItemIndex: ItemIndex::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISWbemObjectSet as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for ISWbemObjectSet {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(ISWbemPrivilege, ISWbemPrivilege_Vtbl, 0x26ee67bd_5804_11d2_8b4a_00600806d9b6);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for ISWbemPrivilege {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(ISWbemPrivilege, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl ISWbemPrivilege {
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_IsEnabled(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_IsEnabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_IsEnabled(&self, bisenabled: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_IsEnabled)(windows_core::Interface::as_raw(self), bisenabled) }
    }
    pub unsafe fn get_Name(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_DisplayName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_DisplayName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_Identifier(&self) -> windows_core::Result<WbemPrivilegeEnum> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Identifier)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct ISWbemPrivilege_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    #[cfg(feature = "wtypes")]
    pub get_IsEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_IsEnabled: usize,
    #[cfg(feature = "wtypes")]
    pub put_IsEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_IsEnabled: usize,
    pub get_Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_DisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Identifier: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WbemPrivilegeEnum) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait ISWbemPrivilege_Impl: super::IDispatch_Impl {
    fn get_IsEnabled(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_IsEnabled(&self, bisenabled: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn get_Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_DisplayName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_Identifier(&self) -> windows_core::Result<WbemPrivilegeEnum>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl ISWbemPrivilege_Vtbl {
    pub const fn new<Identity: ISWbemPrivilege_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_IsEnabled<Identity: ISWbemPrivilege_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bisenabled: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemPrivilege_Impl::get_IsEnabled(this) {
                    Ok(ok__) => {
                        bisenabled.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_IsEnabled<Identity: ISWbemPrivilege_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bisenabled: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISWbemPrivilege_Impl::put_IsEnabled(this, core::mem::transmute_copy(&bisenabled)).into()
            }
        }
        unsafe extern "system" fn get_Name<Identity: ISWbemPrivilege_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strdisplayname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemPrivilege_Impl::get_Name(this) {
                    Ok(ok__) => {
                        strdisplayname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_DisplayName<Identity: ISWbemPrivilege_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strdisplayname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemPrivilege_Impl::get_DisplayName(this) {
                    Ok(ok__) => {
                        strdisplayname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Identifier<Identity: ISWbemPrivilege_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iprivilege: *mut WbemPrivilegeEnum) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemPrivilege_Impl::get_Identifier(this) {
                    Ok(ok__) => {
                        iprivilege.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_IsEnabled: get_IsEnabled::<Identity, OFFSET>,
            put_IsEnabled: put_IsEnabled::<Identity, OFFSET>,
            get_Name: get_Name::<Identity, OFFSET>,
            get_DisplayName: get_DisplayName::<Identity, OFFSET>,
            get_Identifier: get_Identifier::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISWbemPrivilege as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for ISWbemPrivilege {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(ISWbemPrivilegeSet, ISWbemPrivilegeSet_Vtbl, 0x26ee67bf_5804_11d2_8b4a_00600806d9b6);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for ISWbemPrivilegeSet {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(ISWbemPrivilegeSet, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl ISWbemPrivilegeSet {
    pub unsafe fn get__NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get__NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn Item(&self, iprivilege: WbemPrivilegeEnum) -> windows_core::Result<ISWbemPrivilege> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Item)(windows_core::Interface::as_raw(self), iprivilege, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn Add(&self, iprivilege: WbemPrivilegeEnum, bisenabled: super::VARIANT_BOOL) -> windows_core::Result<ISWbemPrivilege> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), iprivilege, bisenabled, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn Remove(&self, iprivilege: WbemPrivilegeEnum) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), iprivilege) }
    }
    pub unsafe fn DeleteAll(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DeleteAll)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn AddAsString(&self, strprivilege: &windows_core::BSTR, bisenabled: super::VARIANT_BOOL) -> windows_core::Result<ISWbemPrivilege> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AddAsString)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strprivilege), bisenabled, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct ISWbemPrivilegeSet_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub get__NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Item: unsafe extern "system" fn(*mut core::ffi::c_void, WbemPrivilegeEnum, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, WbemPrivilegeEnum, super::VARIANT_BOOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    Add: usize,
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, WbemPrivilegeEnum) -> windows_core::HRESULT,
    pub DeleteAll: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub AddAsString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::VARIANT_BOOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    AddAsString: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait ISWbemPrivilegeSet_Impl: super::IDispatch_Impl {
    fn get__NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Item(&self, iprivilege: WbemPrivilegeEnum) -> windows_core::Result<ISWbemPrivilege>;
    fn get_Count(&self) -> windows_core::Result<i32>;
    fn Add(&self, iprivilege: WbemPrivilegeEnum, bisenabled: super::VARIANT_BOOL) -> windows_core::Result<ISWbemPrivilege>;
    fn Remove(&self, iprivilege: WbemPrivilegeEnum) -> windows_core::Result<()>;
    fn DeleteAll(&self) -> windows_core::Result<()>;
    fn AddAsString(&self, strprivilege: &windows_core::BSTR, bisenabled: super::VARIANT_BOOL) -> windows_core::Result<ISWbemPrivilege>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl ISWbemPrivilegeSet_Vtbl {
    pub const fn new<Identity: ISWbemPrivilegeSet_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get__NewEnum<Identity: ISWbemPrivilegeSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemPrivilegeSet_Impl::get__NewEnum(this) {
                    Ok(ok__) => {
                        punk.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Item<Identity: ISWbemPrivilegeSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iprivilege: WbemPrivilegeEnum, objwbemprivilege: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemPrivilegeSet_Impl::Item(this, core::mem::transmute_copy(&iprivilege)) {
                    Ok(ok__) => {
                        objwbemprivilege.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Count<Identity: ISWbemPrivilegeSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, icount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemPrivilegeSet_Impl::get_Count(this) {
                    Ok(ok__) => {
                        icount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Add<Identity: ISWbemPrivilegeSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iprivilege: WbemPrivilegeEnum, bisenabled: super::VARIANT_BOOL, objwbemprivilege: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemPrivilegeSet_Impl::Add(this, core::mem::transmute_copy(&iprivilege), core::mem::transmute_copy(&bisenabled)) {
                    Ok(ok__) => {
                        objwbemprivilege.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Remove<Identity: ISWbemPrivilegeSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iprivilege: WbemPrivilegeEnum) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISWbemPrivilegeSet_Impl::Remove(this, core::mem::transmute_copy(&iprivilege)).into()
            }
        }
        unsafe extern "system" fn DeleteAll<Identity: ISWbemPrivilegeSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISWbemPrivilegeSet_Impl::DeleteAll(this).into()
            }
        }
        unsafe extern "system" fn AddAsString<Identity: ISWbemPrivilegeSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strprivilege: *mut core::ffi::c_void, bisenabled: super::VARIANT_BOOL, objwbemprivilege: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemPrivilegeSet_Impl::AddAsString(this, core::mem::transmute(&strprivilege), core::mem::transmute_copy(&bisenabled)) {
                    Ok(ok__) => {
                        objwbemprivilege.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get__NewEnum: get__NewEnum::<Identity, OFFSET>,
            Item: Item::<Identity, OFFSET>,
            get_Count: get_Count::<Identity, OFFSET>,
            Add: Add::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
            DeleteAll: DeleteAll::<Identity, OFFSET>,
            AddAsString: AddAsString::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISWbemPrivilegeSet as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for ISWbemPrivilegeSet {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(ISWbemProperty, ISWbemProperty_Vtbl, 0x1a388f98_d4ba_11d1_8b09_00600806d9b6);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for ISWbemProperty {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(ISWbemProperty, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl ISWbemProperty {
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn get_Value(&self) -> windows_core::Result<super::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Value)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn put_Value(&self, varvalue: *const super::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Value)(windows_core::Interface::as_raw(self), varvalue) }
    }
    pub unsafe fn get_Name(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_IsLocal(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_IsLocal)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_Origin(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Origin)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_CIMType(&self) -> windows_core::Result<WbemCimtypeEnum> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CIMType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_Qualifiers_(&self) -> windows_core::Result<ISWbemQualifierSet> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Qualifiers_)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_IsArray(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_IsArray)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct ISWbemProperty_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub get_Value: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    get_Value: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub put_Value: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    put_Value: usize,
    pub get_Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub get_IsLocal: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_IsLocal: usize,
    pub get_Origin: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_CIMType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WbemCimtypeEnum) -> windows_core::HRESULT,
    pub get_Qualifiers_: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub get_IsArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_IsArray: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait ISWbemProperty_Impl: super::IDispatch_Impl {
    fn get_Value(&self) -> windows_core::Result<super::VARIANT>;
    fn put_Value(&self, varvalue: *const super::VARIANT) -> windows_core::Result<()>;
    fn get_Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_IsLocal(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn get_Origin(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_CIMType(&self) -> windows_core::Result<WbemCimtypeEnum>;
    fn get_Qualifiers_(&self) -> windows_core::Result<ISWbemQualifierSet>;
    fn get_IsArray(&self) -> windows_core::Result<super::VARIANT_BOOL>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl ISWbemProperty_Vtbl {
    pub const fn new<Identity: ISWbemProperty_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_Value<Identity: ISWbemProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varvalue: *mut super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemProperty_Impl::get_Value(this) {
                    Ok(ok__) => {
                        varvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Value<Identity: ISWbemProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varvalue: *const super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISWbemProperty_Impl::put_Value(this, core::mem::transmute_copy(&varvalue)).into()
            }
        }
        unsafe extern "system" fn get_Name<Identity: ISWbemProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemProperty_Impl::get_Name(this) {
                    Ok(ok__) => {
                        strname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_IsLocal<Identity: ISWbemProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bislocal: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemProperty_Impl::get_IsLocal(this) {
                    Ok(ok__) => {
                        bislocal.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Origin<Identity: ISWbemProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strorigin: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemProperty_Impl::get_Origin(this) {
                    Ok(ok__) => {
                        strorigin.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CIMType<Identity: ISWbemProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, icimtype: *mut WbemCimtypeEnum) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemProperty_Impl::get_CIMType(this) {
                    Ok(ok__) => {
                        icimtype.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Qualifiers_<Identity: ISWbemProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemqualifierset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemProperty_Impl::get_Qualifiers_(this) {
                    Ok(ok__) => {
                        objwbemqualifierset.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_IsArray<Identity: ISWbemProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bisarray: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemProperty_Impl::get_IsArray(this) {
                    Ok(ok__) => {
                        bisarray.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_Value: get_Value::<Identity, OFFSET>,
            put_Value: put_Value::<Identity, OFFSET>,
            get_Name: get_Name::<Identity, OFFSET>,
            get_IsLocal: get_IsLocal::<Identity, OFFSET>,
            get_Origin: get_Origin::<Identity, OFFSET>,
            get_CIMType: get_CIMType::<Identity, OFFSET>,
            get_Qualifiers_: get_Qualifiers_::<Identity, OFFSET>,
            get_IsArray: get_IsArray::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISWbemProperty as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for ISWbemProperty {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(ISWbemPropertySet, ISWbemPropertySet_Vtbl, 0xdea0a7b2_d4ba_11d1_8b09_00600806d9b6);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for ISWbemPropertySet {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(ISWbemPropertySet, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl ISWbemPropertySet {
    pub unsafe fn get__NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get__NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn Item(&self, strname: &windows_core::BSTR, iflags: i32) -> windows_core::Result<ISWbemProperty> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Item)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strname), iflags, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn Add(&self, strname: &windows_core::BSTR, icimtype: WbemCimtypeEnum, bisarray: super::VARIANT_BOOL, iflags: i32) -> windows_core::Result<ISWbemProperty> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strname), icimtype, bisarray, iflags, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn Remove(&self, strname: &windows_core::BSTR, iflags: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strname), iflags) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct ISWbemPropertySet_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub get__NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Item: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, WbemCimtypeEnum, super::VARIANT_BOOL, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    Add: usize,
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait ISWbemPropertySet_Impl: super::IDispatch_Impl {
    fn get__NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Item(&self, strname: &windows_core::BSTR, iflags: i32) -> windows_core::Result<ISWbemProperty>;
    fn get_Count(&self) -> windows_core::Result<i32>;
    fn Add(&self, strname: &windows_core::BSTR, icimtype: WbemCimtypeEnum, bisarray: super::VARIANT_BOOL, iflags: i32) -> windows_core::Result<ISWbemProperty>;
    fn Remove(&self, strname: &windows_core::BSTR, iflags: i32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl ISWbemPropertySet_Vtbl {
    pub const fn new<Identity: ISWbemPropertySet_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get__NewEnum<Identity: ISWbemPropertySet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemPropertySet_Impl::get__NewEnum(this) {
                    Ok(ok__) => {
                        punk.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Item<Identity: ISWbemPropertySet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strname: *mut core::ffi::c_void, iflags: i32, objwbemproperty: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemPropertySet_Impl::Item(this, core::mem::transmute(&strname), core::mem::transmute_copy(&iflags)) {
                    Ok(ok__) => {
                        objwbemproperty.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Count<Identity: ISWbemPropertySet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, icount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemPropertySet_Impl::get_Count(this) {
                    Ok(ok__) => {
                        icount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Add<Identity: ISWbemPropertySet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strname: *mut core::ffi::c_void, icimtype: WbemCimtypeEnum, bisarray: super::VARIANT_BOOL, iflags: i32, objwbemproperty: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemPropertySet_Impl::Add(this, core::mem::transmute(&strname), core::mem::transmute_copy(&icimtype), core::mem::transmute_copy(&bisarray), core::mem::transmute_copy(&iflags)) {
                    Ok(ok__) => {
                        objwbemproperty.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Remove<Identity: ISWbemPropertySet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strname: *mut core::ffi::c_void, iflags: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISWbemPropertySet_Impl::Remove(this, core::mem::transmute(&strname), core::mem::transmute_copy(&iflags)).into()
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get__NewEnum: get__NewEnum::<Identity, OFFSET>,
            Item: Item::<Identity, OFFSET>,
            get_Count: get_Count::<Identity, OFFSET>,
            Add: Add::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISWbemPropertySet as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for ISWbemPropertySet {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(ISWbemQualifier, ISWbemQualifier_Vtbl, 0x79b05932_d3b7_11d1_8b06_00600806d9b6);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for ISWbemQualifier {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(ISWbemQualifier, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl ISWbemQualifier {
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn get_Value(&self) -> windows_core::Result<super::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Value)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn put_Value(&self, varvalue: *const super::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Value)(windows_core::Interface::as_raw(self), varvalue) }
    }
    pub unsafe fn get_Name(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_IsLocal(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_IsLocal)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_PropagatesToSubclass(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_PropagatesToSubclass)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_PropagatesToSubclass(&self, bpropagatestosubclass: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_PropagatesToSubclass)(windows_core::Interface::as_raw(self), bpropagatestosubclass) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_PropagatesToInstance(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_PropagatesToInstance)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_PropagatesToInstance(&self, bpropagatestoinstance: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_PropagatesToInstance)(windows_core::Interface::as_raw(self), bpropagatestoinstance) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_IsOverridable(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_IsOverridable)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_IsOverridable(&self, bisoverridable: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_IsOverridable)(windows_core::Interface::as_raw(self), bisoverridable) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_IsAmended(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_IsAmended)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct ISWbemQualifier_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub get_Value: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    get_Value: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub put_Value: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    put_Value: usize,
    pub get_Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub get_IsLocal: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_IsLocal: usize,
    #[cfg(feature = "wtypes")]
    pub get_PropagatesToSubclass: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_PropagatesToSubclass: usize,
    #[cfg(feature = "wtypes")]
    pub put_PropagatesToSubclass: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_PropagatesToSubclass: usize,
    #[cfg(feature = "wtypes")]
    pub get_PropagatesToInstance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_PropagatesToInstance: usize,
    #[cfg(feature = "wtypes")]
    pub put_PropagatesToInstance: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_PropagatesToInstance: usize,
    #[cfg(feature = "wtypes")]
    pub get_IsOverridable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_IsOverridable: usize,
    #[cfg(feature = "wtypes")]
    pub put_IsOverridable: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_IsOverridable: usize,
    #[cfg(feature = "wtypes")]
    pub get_IsAmended: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_IsAmended: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait ISWbemQualifier_Impl: super::IDispatch_Impl {
    fn get_Value(&self) -> windows_core::Result<super::VARIANT>;
    fn put_Value(&self, varvalue: *const super::VARIANT) -> windows_core::Result<()>;
    fn get_Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_IsLocal(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn get_PropagatesToSubclass(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_PropagatesToSubclass(&self, bpropagatestosubclass: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn get_PropagatesToInstance(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_PropagatesToInstance(&self, bpropagatestoinstance: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn get_IsOverridable(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_IsOverridable(&self, bisoverridable: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn get_IsAmended(&self) -> windows_core::Result<super::VARIANT_BOOL>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl ISWbemQualifier_Vtbl {
    pub const fn new<Identity: ISWbemQualifier_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_Value<Identity: ISWbemQualifier_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varvalue: *mut super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemQualifier_Impl::get_Value(this) {
                    Ok(ok__) => {
                        varvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Value<Identity: ISWbemQualifier_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varvalue: *const super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISWbemQualifier_Impl::put_Value(this, core::mem::transmute_copy(&varvalue)).into()
            }
        }
        unsafe extern "system" fn get_Name<Identity: ISWbemQualifier_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemQualifier_Impl::get_Name(this) {
                    Ok(ok__) => {
                        strname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_IsLocal<Identity: ISWbemQualifier_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bislocal: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemQualifier_Impl::get_IsLocal(this) {
                    Ok(ok__) => {
                        bislocal.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_PropagatesToSubclass<Identity: ISWbemQualifier_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bpropagatestosubclass: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemQualifier_Impl::get_PropagatesToSubclass(this) {
                    Ok(ok__) => {
                        bpropagatestosubclass.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_PropagatesToSubclass<Identity: ISWbemQualifier_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bpropagatestosubclass: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISWbemQualifier_Impl::put_PropagatesToSubclass(this, core::mem::transmute_copy(&bpropagatestosubclass)).into()
            }
        }
        unsafe extern "system" fn get_PropagatesToInstance<Identity: ISWbemQualifier_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bpropagatestoinstance: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemQualifier_Impl::get_PropagatesToInstance(this) {
                    Ok(ok__) => {
                        bpropagatestoinstance.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_PropagatesToInstance<Identity: ISWbemQualifier_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bpropagatestoinstance: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISWbemQualifier_Impl::put_PropagatesToInstance(this, core::mem::transmute_copy(&bpropagatestoinstance)).into()
            }
        }
        unsafe extern "system" fn get_IsOverridable<Identity: ISWbemQualifier_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bisoverridable: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemQualifier_Impl::get_IsOverridable(this) {
                    Ok(ok__) => {
                        bisoverridable.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_IsOverridable<Identity: ISWbemQualifier_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bisoverridable: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISWbemQualifier_Impl::put_IsOverridable(this, core::mem::transmute_copy(&bisoverridable)).into()
            }
        }
        unsafe extern "system" fn get_IsAmended<Identity: ISWbemQualifier_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bisamended: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemQualifier_Impl::get_IsAmended(this) {
                    Ok(ok__) => {
                        bisamended.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_Value: get_Value::<Identity, OFFSET>,
            put_Value: put_Value::<Identity, OFFSET>,
            get_Name: get_Name::<Identity, OFFSET>,
            get_IsLocal: get_IsLocal::<Identity, OFFSET>,
            get_PropagatesToSubclass: get_PropagatesToSubclass::<Identity, OFFSET>,
            put_PropagatesToSubclass: put_PropagatesToSubclass::<Identity, OFFSET>,
            get_PropagatesToInstance: get_PropagatesToInstance::<Identity, OFFSET>,
            put_PropagatesToInstance: put_PropagatesToInstance::<Identity, OFFSET>,
            get_IsOverridable: get_IsOverridable::<Identity, OFFSET>,
            put_IsOverridable: put_IsOverridable::<Identity, OFFSET>,
            get_IsAmended: get_IsAmended::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISWbemQualifier as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for ISWbemQualifier {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(ISWbemQualifierSet, ISWbemQualifierSet_Vtbl, 0x9b16ed16_d3df_11d1_8b08_00600806d9b6);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for ISWbemQualifierSet {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(ISWbemQualifierSet, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl ISWbemQualifierSet {
    pub unsafe fn get__NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get__NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn Item(&self, name: &windows_core::BSTR, iflags: i32) -> windows_core::Result<ISWbemQualifier> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Item)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(name), iflags, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn Add(&self, strname: &windows_core::BSTR, varval: *const super::VARIANT, bpropagatestosubclass: super::VARIANT_BOOL, bpropagatestoinstance: super::VARIANT_BOOL, bisoverridable: super::VARIANT_BOOL, iflags: i32) -> windows_core::Result<ISWbemQualifier> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strname), varval, bpropagatestosubclass, bpropagatestoinstance, bisoverridable, iflags, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn Remove(&self, strname: &windows_core::BSTR, iflags: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strname), iflags) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct ISWbemQualifierSet_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub get__NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Item: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const super::VARIANT, super::VARIANT_BOOL, super::VARIANT_BOOL, super::VARIANT_BOOL, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    Add: usize,
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait ISWbemQualifierSet_Impl: super::IDispatch_Impl {
    fn get__NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Item(&self, name: &windows_core::BSTR, iflags: i32) -> windows_core::Result<ISWbemQualifier>;
    fn get_Count(&self) -> windows_core::Result<i32>;
    fn Add(&self, strname: &windows_core::BSTR, varval: *const super::VARIANT, bpropagatestosubclass: super::VARIANT_BOOL, bpropagatestoinstance: super::VARIANT_BOOL, bisoverridable: super::VARIANT_BOOL, iflags: i32) -> windows_core::Result<ISWbemQualifier>;
    fn Remove(&self, strname: &windows_core::BSTR, iflags: i32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl ISWbemQualifierSet_Vtbl {
    pub const fn new<Identity: ISWbemQualifierSet_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get__NewEnum<Identity: ISWbemQualifierSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemQualifierSet_Impl::get__NewEnum(this) {
                    Ok(ok__) => {
                        punk.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Item<Identity: ISWbemQualifierSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::ffi::c_void, iflags: i32, objwbemqualifier: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemQualifierSet_Impl::Item(this, core::mem::transmute(&name), core::mem::transmute_copy(&iflags)) {
                    Ok(ok__) => {
                        objwbemqualifier.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Count<Identity: ISWbemQualifierSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, icount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemQualifierSet_Impl::get_Count(this) {
                    Ok(ok__) => {
                        icount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Add<Identity: ISWbemQualifierSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strname: *mut core::ffi::c_void, varval: *const super::VARIANT, bpropagatestosubclass: super::VARIANT_BOOL, bpropagatestoinstance: super::VARIANT_BOOL, bisoverridable: super::VARIANT_BOOL, iflags: i32, objwbemqualifier: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemQualifierSet_Impl::Add(this, core::mem::transmute(&strname), core::mem::transmute_copy(&varval), core::mem::transmute_copy(&bpropagatestosubclass), core::mem::transmute_copy(&bpropagatestoinstance), core::mem::transmute_copy(&bisoverridable), core::mem::transmute_copy(&iflags)) {
                    Ok(ok__) => {
                        objwbemqualifier.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Remove<Identity: ISWbemQualifierSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strname: *mut core::ffi::c_void, iflags: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISWbemQualifierSet_Impl::Remove(this, core::mem::transmute(&strname), core::mem::transmute_copy(&iflags)).into()
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get__NewEnum: get__NewEnum::<Identity, OFFSET>,
            Item: Item::<Identity, OFFSET>,
            get_Count: get_Count::<Identity, OFFSET>,
            Add: Add::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISWbemQualifierSet as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for ISWbemQualifierSet {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(ISWbemRefreshableItem, ISWbemRefreshableItem_Vtbl, 0x5ad4bf92_daab_11d3_b38f_00105a1f473a);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for ISWbemRefreshableItem {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(ISWbemRefreshableItem, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl ISWbemRefreshableItem {
    pub unsafe fn get_Index(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Index)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_Refresher(&self) -> windows_core::Result<ISWbemRefresher> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Refresher)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_IsSet(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_IsSet)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_Object(&self) -> windows_core::Result<ISWbemObjectEx> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Object)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_ObjectSet(&self) -> windows_core::Result<ISWbemObjectSet> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ObjectSet)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn Remove(&self, iflags: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), iflags) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct ISWbemRefreshableItem_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub get_Index: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub get_Refresher: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub get_IsSet: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_IsSet: usize,
    pub get_Object: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_ObjectSet: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait ISWbemRefreshableItem_Impl: super::IDispatch_Impl {
    fn get_Index(&self) -> windows_core::Result<i32>;
    fn get_Refresher(&self) -> windows_core::Result<ISWbemRefresher>;
    fn get_IsSet(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn get_Object(&self) -> windows_core::Result<ISWbemObjectEx>;
    fn get_ObjectSet(&self) -> windows_core::Result<ISWbemObjectSet>;
    fn Remove(&self, iflags: i32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl ISWbemRefreshableItem_Vtbl {
    pub const fn new<Identity: ISWbemRefreshableItem_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_Index<Identity: ISWbemRefreshableItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iindex: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemRefreshableItem_Impl::get_Index(this) {
                    Ok(ok__) => {
                        iindex.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Refresher<Identity: ISWbemRefreshableItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemrefresher: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemRefreshableItem_Impl::get_Refresher(this) {
                    Ok(ok__) => {
                        objwbemrefresher.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_IsSet<Identity: ISWbemRefreshableItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bisset: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemRefreshableItem_Impl::get_IsSet(this) {
                    Ok(ok__) => {
                        bisset.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Object<Identity: ISWbemRefreshableItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemRefreshableItem_Impl::get_Object(this) {
                    Ok(ok__) => {
                        objwbemobject.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_ObjectSet<Identity: ISWbemRefreshableItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemobjectset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemRefreshableItem_Impl::get_ObjectSet(this) {
                    Ok(ok__) => {
                        objwbemobjectset.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Remove<Identity: ISWbemRefreshableItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iflags: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISWbemRefreshableItem_Impl::Remove(this, core::mem::transmute_copy(&iflags)).into()
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_Index: get_Index::<Identity, OFFSET>,
            get_Refresher: get_Refresher::<Identity, OFFSET>,
            get_IsSet: get_IsSet::<Identity, OFFSET>,
            get_Object: get_Object::<Identity, OFFSET>,
            get_ObjectSet: get_ObjectSet::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISWbemRefreshableItem as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for ISWbemRefreshableItem {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(ISWbemRefresher, ISWbemRefresher_Vtbl, 0x14d8250e_d9c2_11d3_b38f_00105a1f473a);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for ISWbemRefresher {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(ISWbemRefresher, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl ISWbemRefresher {
    pub unsafe fn get__NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get__NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn Item(&self, iindex: i32) -> windows_core::Result<ISWbemRefreshableItem> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Item)(windows_core::Interface::as_raw(self), iindex, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Add<P0, P3>(&self, objwbemservices: P0, bsinstancepath: &windows_core::BSTR, iflags: i32, objwbemnamedvalueset: P3) -> windows_core::Result<ISWbemRefreshableItem>
    where
        P0: windows_core::Param<ISWbemServicesEx>,
        P3: windows_core::Param<super::IDispatch>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), objwbemservices.param().abi(), core::mem::transmute_copy(bsinstancepath), iflags, objwbemnamedvalueset.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn AddEnum<P0, P3>(&self, objwbemservices: P0, bsclassname: &windows_core::BSTR, iflags: i32, objwbemnamedvalueset: P3) -> windows_core::Result<ISWbemRefreshableItem>
    where
        P0: windows_core::Param<ISWbemServicesEx>,
        P3: windows_core::Param<super::IDispatch>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AddEnum)(windows_core::Interface::as_raw(self), objwbemservices.param().abi(), core::mem::transmute_copy(bsclassname), iflags, objwbemnamedvalueset.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn Remove(&self, iindex: i32, iflags: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), iindex, iflags) }
    }
    pub unsafe fn Refresh(&self, iflags: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Refresh)(windows_core::Interface::as_raw(self), iflags) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_AutoReconnect(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_AutoReconnect)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_AutoReconnect(&self, bcount: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_AutoReconnect)(windows_core::Interface::as_raw(self), bcount) }
    }
    pub unsafe fn DeleteAll(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DeleteAll)(windows_core::Interface::as_raw(self)) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct ISWbemRefresher_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub get__NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Item: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32) -> windows_core::HRESULT,
    pub Refresh: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub get_AutoReconnect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_AutoReconnect: usize,
    #[cfg(feature = "wtypes")]
    pub put_AutoReconnect: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_AutoReconnect: usize,
    pub DeleteAll: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait ISWbemRefresher_Impl: super::IDispatch_Impl {
    fn get__NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Item(&self, iindex: i32) -> windows_core::Result<ISWbemRefreshableItem>;
    fn get_Count(&self) -> windows_core::Result<i32>;
    fn Add(&self, objwbemservices: windows_core::Ref<ISWbemServicesEx>, bsinstancepath: &windows_core::BSTR, iflags: i32, objwbemnamedvalueset: windows_core::Ref<super::IDispatch>) -> windows_core::Result<ISWbemRefreshableItem>;
    fn AddEnum(&self, objwbemservices: windows_core::Ref<ISWbemServicesEx>, bsclassname: &windows_core::BSTR, iflags: i32, objwbemnamedvalueset: windows_core::Ref<super::IDispatch>) -> windows_core::Result<ISWbemRefreshableItem>;
    fn Remove(&self, iindex: i32, iflags: i32) -> windows_core::Result<()>;
    fn Refresh(&self, iflags: i32) -> windows_core::Result<()>;
    fn get_AutoReconnect(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_AutoReconnect(&self, bcount: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn DeleteAll(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl ISWbemRefresher_Vtbl {
    pub const fn new<Identity: ISWbemRefresher_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get__NewEnum<Identity: ISWbemRefresher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemRefresher_Impl::get__NewEnum(this) {
                    Ok(ok__) => {
                        punk.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Item<Identity: ISWbemRefresher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iindex: i32, objwbemrefreshableitem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemRefresher_Impl::Item(this, core::mem::transmute_copy(&iindex)) {
                    Ok(ok__) => {
                        objwbemrefreshableitem.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Count<Identity: ISWbemRefresher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, icount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemRefresher_Impl::get_Count(this) {
                    Ok(ok__) => {
                        icount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Add<Identity: ISWbemRefresher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemservices: *mut core::ffi::c_void, bsinstancepath: *mut core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void, objwbemrefreshableitem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemRefresher_Impl::Add(this, core::mem::transmute_copy(&objwbemservices), core::mem::transmute(&bsinstancepath), core::mem::transmute_copy(&iflags), core::mem::transmute_copy(&objwbemnamedvalueset)) {
                    Ok(ok__) => {
                        objwbemrefreshableitem.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AddEnum<Identity: ISWbemRefresher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemservices: *mut core::ffi::c_void, bsclassname: *mut core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void, objwbemrefreshableitem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemRefresher_Impl::AddEnum(this, core::mem::transmute_copy(&objwbemservices), core::mem::transmute(&bsclassname), core::mem::transmute_copy(&iflags), core::mem::transmute_copy(&objwbemnamedvalueset)) {
                    Ok(ok__) => {
                        objwbemrefreshableitem.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Remove<Identity: ISWbemRefresher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iindex: i32, iflags: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISWbemRefresher_Impl::Remove(this, core::mem::transmute_copy(&iindex), core::mem::transmute_copy(&iflags)).into()
            }
        }
        unsafe extern "system" fn Refresh<Identity: ISWbemRefresher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iflags: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISWbemRefresher_Impl::Refresh(this, core::mem::transmute_copy(&iflags)).into()
            }
        }
        unsafe extern "system" fn get_AutoReconnect<Identity: ISWbemRefresher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bcount: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemRefresher_Impl::get_AutoReconnect(this) {
                    Ok(ok__) => {
                        bcount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_AutoReconnect<Identity: ISWbemRefresher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bcount: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISWbemRefresher_Impl::put_AutoReconnect(this, core::mem::transmute_copy(&bcount)).into()
            }
        }
        unsafe extern "system" fn DeleteAll<Identity: ISWbemRefresher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISWbemRefresher_Impl::DeleteAll(this).into()
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get__NewEnum: get__NewEnum::<Identity, OFFSET>,
            Item: Item::<Identity, OFFSET>,
            get_Count: get_Count::<Identity, OFFSET>,
            Add: Add::<Identity, OFFSET>,
            AddEnum: AddEnum::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
            Refresh: Refresh::<Identity, OFFSET>,
            get_AutoReconnect: get_AutoReconnect::<Identity, OFFSET>,
            put_AutoReconnect: put_AutoReconnect::<Identity, OFFSET>,
            DeleteAll: DeleteAll::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISWbemRefresher as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for ISWbemRefresher {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(ISWbemSecurity, ISWbemSecurity_Vtbl, 0xb54d66e6_2287_11d2_8b33_00600806d9b6);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for ISWbemSecurity {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(ISWbemSecurity, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl ISWbemSecurity {
    pub unsafe fn get_ImpersonationLevel(&self) -> windows_core::Result<WbemImpersonationLevelEnum> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ImpersonationLevel)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_ImpersonationLevel(&self, iimpersonationlevel: WbemImpersonationLevelEnum) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_ImpersonationLevel)(windows_core::Interface::as_raw(self), iimpersonationlevel) }
    }
    pub unsafe fn get_AuthenticationLevel(&self) -> windows_core::Result<WbemAuthenticationLevelEnum> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_AuthenticationLevel)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_AuthenticationLevel(&self, iauthenticationlevel: WbemAuthenticationLevelEnum) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_AuthenticationLevel)(windows_core::Interface::as_raw(self), iauthenticationlevel) }
    }
    pub unsafe fn get_Privileges(&self) -> windows_core::Result<ISWbemPrivilegeSet> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Privileges)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct ISWbemSecurity_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub get_ImpersonationLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WbemImpersonationLevelEnum) -> windows_core::HRESULT,
    pub put_ImpersonationLevel: unsafe extern "system" fn(*mut core::ffi::c_void, WbemImpersonationLevelEnum) -> windows_core::HRESULT,
    pub get_AuthenticationLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WbemAuthenticationLevelEnum) -> windows_core::HRESULT,
    pub put_AuthenticationLevel: unsafe extern "system" fn(*mut core::ffi::c_void, WbemAuthenticationLevelEnum) -> windows_core::HRESULT,
    pub get_Privileges: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait ISWbemSecurity_Impl: super::IDispatch_Impl {
    fn get_ImpersonationLevel(&self) -> windows_core::Result<WbemImpersonationLevelEnum>;
    fn put_ImpersonationLevel(&self, iimpersonationlevel: WbemImpersonationLevelEnum) -> windows_core::Result<()>;
    fn get_AuthenticationLevel(&self) -> windows_core::Result<WbemAuthenticationLevelEnum>;
    fn put_AuthenticationLevel(&self, iauthenticationlevel: WbemAuthenticationLevelEnum) -> windows_core::Result<()>;
    fn get_Privileges(&self) -> windows_core::Result<ISWbemPrivilegeSet>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl ISWbemSecurity_Vtbl {
    pub const fn new<Identity: ISWbemSecurity_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_ImpersonationLevel<Identity: ISWbemSecurity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iimpersonationlevel: *mut WbemImpersonationLevelEnum) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemSecurity_Impl::get_ImpersonationLevel(this) {
                    Ok(ok__) => {
                        iimpersonationlevel.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_ImpersonationLevel<Identity: ISWbemSecurity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iimpersonationlevel: WbemImpersonationLevelEnum) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISWbemSecurity_Impl::put_ImpersonationLevel(this, core::mem::transmute_copy(&iimpersonationlevel)).into()
            }
        }
        unsafe extern "system" fn get_AuthenticationLevel<Identity: ISWbemSecurity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iauthenticationlevel: *mut WbemAuthenticationLevelEnum) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemSecurity_Impl::get_AuthenticationLevel(this) {
                    Ok(ok__) => {
                        iauthenticationlevel.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_AuthenticationLevel<Identity: ISWbemSecurity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iauthenticationlevel: WbemAuthenticationLevelEnum) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISWbemSecurity_Impl::put_AuthenticationLevel(this, core::mem::transmute_copy(&iauthenticationlevel)).into()
            }
        }
        unsafe extern "system" fn get_Privileges<Identity: ISWbemSecurity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemprivilegeset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemSecurity_Impl::get_Privileges(this) {
                    Ok(ok__) => {
                        objwbemprivilegeset.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_ImpersonationLevel: get_ImpersonationLevel::<Identity, OFFSET>,
            put_ImpersonationLevel: put_ImpersonationLevel::<Identity, OFFSET>,
            get_AuthenticationLevel: get_AuthenticationLevel::<Identity, OFFSET>,
            put_AuthenticationLevel: put_AuthenticationLevel::<Identity, OFFSET>,
            get_Privileges: get_Privileges::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISWbemSecurity as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for ISWbemSecurity {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(ISWbemServices, ISWbemServices_Vtbl, 0x76a6415c_cb41_11d1_8b02_00600806d9b6);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for ISWbemServices {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(ISWbemServices, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl ISWbemServices {
    pub unsafe fn Get<P2>(&self, strobjectpath: &windows_core::BSTR, iflags: i32, objwbemnamedvalueset: P2) -> windows_core::Result<ISWbemObject>
    where
        P2: windows_core::Param<super::IDispatch>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Get)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strobjectpath), iflags, objwbemnamedvalueset.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetAsync<P0, P3, P4>(&self, objwbemsink: P0, strobjectpath: &windows_core::BSTR, iflags: i32, objwbemnamedvalueset: P3, objwbemasynccontext: P4) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::IDispatch>,
        P3: windows_core::Param<super::IDispatch>,
        P4: windows_core::Param<super::IDispatch>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetAsync)(windows_core::Interface::as_raw(self), objwbemsink.param().abi(), core::mem::transmute_copy(strobjectpath), iflags, objwbemnamedvalueset.param().abi(), objwbemasynccontext.param().abi()) }
    }
    pub unsafe fn Delete<P2>(&self, strobjectpath: &windows_core::BSTR, iflags: i32, objwbemnamedvalueset: P2) -> windows_core::HRESULT
    where
        P2: windows_core::Param<super::IDispatch>,
    {
        unsafe { (windows_core::Interface::vtable(self).Delete)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strobjectpath), iflags, objwbemnamedvalueset.param().abi()) }
    }
    pub unsafe fn DeleteAsync<P0, P3, P4>(&self, objwbemsink: P0, strobjectpath: &windows_core::BSTR, iflags: i32, objwbemnamedvalueset: P3, objwbemasynccontext: P4) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::IDispatch>,
        P3: windows_core::Param<super::IDispatch>,
        P4: windows_core::Param<super::IDispatch>,
    {
        unsafe { (windows_core::Interface::vtable(self).DeleteAsync)(windows_core::Interface::as_raw(self), objwbemsink.param().abi(), core::mem::transmute_copy(strobjectpath), iflags, objwbemnamedvalueset.param().abi(), objwbemasynccontext.param().abi()) }
    }
    pub unsafe fn InstancesOf<P2>(&self, strclass: &windows_core::BSTR, iflags: i32, objwbemnamedvalueset: P2) -> windows_core::Result<ISWbemObjectSet>
    where
        P2: windows_core::Param<super::IDispatch>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).InstancesOf)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strclass), iflags, objwbemnamedvalueset.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn InstancesOfAsync<P0, P3, P4>(&self, objwbemsink: P0, strclass: &windows_core::BSTR, iflags: i32, objwbemnamedvalueset: P3, objwbemasynccontext: P4) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::IDispatch>,
        P3: windows_core::Param<super::IDispatch>,
        P4: windows_core::Param<super::IDispatch>,
    {
        unsafe { (windows_core::Interface::vtable(self).InstancesOfAsync)(windows_core::Interface::as_raw(self), objwbemsink.param().abi(), core::mem::transmute_copy(strclass), iflags, objwbemnamedvalueset.param().abi(), objwbemasynccontext.param().abi()) }
    }
    pub unsafe fn SubclassesOf<P2>(&self, strsuperclass: &windows_core::BSTR, iflags: i32, objwbemnamedvalueset: P2) -> windows_core::Result<ISWbemObjectSet>
    where
        P2: windows_core::Param<super::IDispatch>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SubclassesOf)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strsuperclass), iflags, objwbemnamedvalueset.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn SubclassesOfAsync<P0, P3, P4>(&self, objwbemsink: P0, strsuperclass: &windows_core::BSTR, iflags: i32, objwbemnamedvalueset: P3, objwbemasynccontext: P4) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::IDispatch>,
        P3: windows_core::Param<super::IDispatch>,
        P4: windows_core::Param<super::IDispatch>,
    {
        unsafe { (windows_core::Interface::vtable(self).SubclassesOfAsync)(windows_core::Interface::as_raw(self), objwbemsink.param().abi(), core::mem::transmute_copy(strsuperclass), iflags, objwbemnamedvalueset.param().abi(), objwbemasynccontext.param().abi()) }
    }
    pub unsafe fn ExecQuery<P3>(&self, strquery: &windows_core::BSTR, strquerylanguage: &windows_core::BSTR, iflags: i32, objwbemnamedvalueset: P3) -> windows_core::Result<ISWbemObjectSet>
    where
        P3: windows_core::Param<super::IDispatch>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ExecQuery)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strquery), core::mem::transmute_copy(strquerylanguage), iflags, objwbemnamedvalueset.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn ExecQueryAsync<P0, P4, P5>(&self, objwbemsink: P0, strquery: &windows_core::BSTR, strquerylanguage: &windows_core::BSTR, lflags: i32, objwbemnamedvalueset: P4, objwbemasynccontext: P5) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::IDispatch>,
        P4: windows_core::Param<super::IDispatch>,
        P5: windows_core::Param<super::IDispatch>,
    {
        unsafe { (windows_core::Interface::vtable(self).ExecQueryAsync)(windows_core::Interface::as_raw(self), objwbemsink.param().abi(), core::mem::transmute_copy(strquery), core::mem::transmute_copy(strquerylanguage), lflags, objwbemnamedvalueset.param().abi(), objwbemasynccontext.param().abi()) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn AssociatorsOf<P10>(&self, strobjectpath: &windows_core::BSTR, strassocclass: &windows_core::BSTR, strresultclass: &windows_core::BSTR, strresultrole: &windows_core::BSTR, strrole: &windows_core::BSTR, bclassesonly: super::VARIANT_BOOL, bschemaonly: super::VARIANT_BOOL, strrequiredassocqualifier: &windows_core::BSTR, strrequiredqualifier: &windows_core::BSTR, iflags: i32, objwbemnamedvalueset: P10) -> windows_core::Result<ISWbemObjectSet>
    where
        P10: windows_core::Param<super::IDispatch>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AssociatorsOf)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strobjectpath), core::mem::transmute_copy(strassocclass), core::mem::transmute_copy(strresultclass), core::mem::transmute_copy(strresultrole), core::mem::transmute_copy(strrole), bclassesonly, bschemaonly, core::mem::transmute_copy(strrequiredassocqualifier), core::mem::transmute_copy(strrequiredqualifier), iflags, objwbemnamedvalueset.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn AssociatorsOfAsync<P0, P11, P12>(&self, objwbemsink: P0, strobjectpath: &windows_core::BSTR, strassocclass: &windows_core::BSTR, strresultclass: &windows_core::BSTR, strresultrole: &windows_core::BSTR, strrole: &windows_core::BSTR, bclassesonly: super::VARIANT_BOOL, bschemaonly: super::VARIANT_BOOL, strrequiredassocqualifier: &windows_core::BSTR, strrequiredqualifier: &windows_core::BSTR, iflags: i32, objwbemnamedvalueset: P11, objwbemasynccontext: P12) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::IDispatch>,
        P11: windows_core::Param<super::IDispatch>,
        P12: windows_core::Param<super::IDispatch>,
    {
        unsafe { (windows_core::Interface::vtable(self).AssociatorsOfAsync)(windows_core::Interface::as_raw(self), objwbemsink.param().abi(), core::mem::transmute_copy(strobjectpath), core::mem::transmute_copy(strassocclass), core::mem::transmute_copy(strresultclass), core::mem::transmute_copy(strresultrole), core::mem::transmute_copy(strrole), bclassesonly, bschemaonly, core::mem::transmute_copy(strrequiredassocqualifier), core::mem::transmute_copy(strrequiredqualifier), iflags, objwbemnamedvalueset.param().abi(), objwbemasynccontext.param().abi()) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn ReferencesTo<P7>(&self, strobjectpath: &windows_core::BSTR, strresultclass: &windows_core::BSTR, strrole: &windows_core::BSTR, bclassesonly: super::VARIANT_BOOL, bschemaonly: super::VARIANT_BOOL, strrequiredqualifier: &windows_core::BSTR, iflags: i32, objwbemnamedvalueset: P7) -> windows_core::Result<ISWbemObjectSet>
    where
        P7: windows_core::Param<super::IDispatch>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReferencesTo)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strobjectpath), core::mem::transmute_copy(strresultclass), core::mem::transmute_copy(strrole), bclassesonly, bschemaonly, core::mem::transmute_copy(strrequiredqualifier), iflags, objwbemnamedvalueset.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn ReferencesToAsync<P0, P8, P9>(&self, objwbemsink: P0, strobjectpath: &windows_core::BSTR, strresultclass: &windows_core::BSTR, strrole: &windows_core::BSTR, bclassesonly: super::VARIANT_BOOL, bschemaonly: super::VARIANT_BOOL, strrequiredqualifier: &windows_core::BSTR, iflags: i32, objwbemnamedvalueset: P8, objwbemasynccontext: P9) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::IDispatch>,
        P8: windows_core::Param<super::IDispatch>,
        P9: windows_core::Param<super::IDispatch>,
    {
        unsafe { (windows_core::Interface::vtable(self).ReferencesToAsync)(windows_core::Interface::as_raw(self), objwbemsink.param().abi(), core::mem::transmute_copy(strobjectpath), core::mem::transmute_copy(strresultclass), core::mem::transmute_copy(strrole), bclassesonly, bschemaonly, core::mem::transmute_copy(strrequiredqualifier), iflags, objwbemnamedvalueset.param().abi(), objwbemasynccontext.param().abi()) }
    }
    pub unsafe fn ExecNotificationQuery<P3>(&self, strquery: &windows_core::BSTR, strquerylanguage: &windows_core::BSTR, iflags: i32, objwbemnamedvalueset: P3) -> windows_core::Result<ISWbemEventSource>
    where
        P3: windows_core::Param<super::IDispatch>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ExecNotificationQuery)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strquery), core::mem::transmute_copy(strquerylanguage), iflags, objwbemnamedvalueset.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn ExecNotificationQueryAsync<P0, P4, P5>(&self, objwbemsink: P0, strquery: &windows_core::BSTR, strquerylanguage: &windows_core::BSTR, iflags: i32, objwbemnamedvalueset: P4, objwbemasynccontext: P5) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::IDispatch>,
        P4: windows_core::Param<super::IDispatch>,
        P5: windows_core::Param<super::IDispatch>,
    {
        unsafe { (windows_core::Interface::vtable(self).ExecNotificationQueryAsync)(windows_core::Interface::as_raw(self), objwbemsink.param().abi(), core::mem::transmute_copy(strquery), core::mem::transmute_copy(strquerylanguage), iflags, objwbemnamedvalueset.param().abi(), objwbemasynccontext.param().abi()) }
    }
    pub unsafe fn ExecMethod<P2, P4>(&self, strobjectpath: &windows_core::BSTR, strmethodname: &windows_core::BSTR, objwbeminparameters: P2, iflags: i32, objwbemnamedvalueset: P4) -> windows_core::Result<ISWbemObject>
    where
        P2: windows_core::Param<super::IDispatch>,
        P4: windows_core::Param<super::IDispatch>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ExecMethod)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strobjectpath), core::mem::transmute_copy(strmethodname), objwbeminparameters.param().abi(), iflags, objwbemnamedvalueset.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn ExecMethodAsync<P0, P3, P5, P6>(&self, objwbemsink: P0, strobjectpath: &windows_core::BSTR, strmethodname: &windows_core::BSTR, objwbeminparameters: P3, iflags: i32, objwbemnamedvalueset: P5, objwbemasynccontext: P6) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::IDispatch>,
        P3: windows_core::Param<super::IDispatch>,
        P5: windows_core::Param<super::IDispatch>,
        P6: windows_core::Param<super::IDispatch>,
    {
        unsafe { (windows_core::Interface::vtable(self).ExecMethodAsync)(windows_core::Interface::as_raw(self), objwbemsink.param().abi(), core::mem::transmute_copy(strobjectpath), core::mem::transmute_copy(strmethodname), objwbeminparameters.param().abi(), iflags, objwbemnamedvalueset.param().abi(), objwbemasynccontext.param().abi()) }
    }
    pub unsafe fn get_Security_(&self) -> windows_core::Result<ISWbemSecurity> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Security_)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct ISWbemServices_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub Get: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Delete: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DeleteAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub InstancesOf: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub InstancesOfAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SubclassesOf: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SubclassesOfAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ExecQuery: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ExecQueryAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub AssociatorsOf: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, super::VARIANT_BOOL, super::VARIANT_BOOL, *mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    AssociatorsOf: usize,
    #[cfg(feature = "wtypes")]
    pub AssociatorsOfAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, super::VARIANT_BOOL, super::VARIANT_BOOL, *mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    AssociatorsOfAsync: usize,
    #[cfg(feature = "wtypes")]
    pub ReferencesTo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, super::VARIANT_BOOL, super::VARIANT_BOOL, *mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    ReferencesTo: usize,
    #[cfg(feature = "wtypes")]
    pub ReferencesToAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, super::VARIANT_BOOL, super::VARIANT_BOOL, *mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    ReferencesToAsync: usize,
    pub ExecNotificationQuery: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ExecNotificationQueryAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ExecMethod: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ExecMethodAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Security_: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait ISWbemServices_Impl: super::IDispatch_Impl {
    fn Get(&self, strobjectpath: &windows_core::BSTR, iflags: i32, objwbemnamedvalueset: windows_core::Ref<super::IDispatch>) -> windows_core::Result<ISWbemObject>;
    fn GetAsync(&self, objwbemsink: windows_core::Ref<super::IDispatch>, strobjectpath: &windows_core::BSTR, iflags: i32, objwbemnamedvalueset: windows_core::Ref<super::IDispatch>, objwbemasynccontext: windows_core::Ref<super::IDispatch>) -> windows_core::Result<()>;
    fn Delete(&self, strobjectpath: &windows_core::BSTR, iflags: i32, objwbemnamedvalueset: windows_core::Ref<super::IDispatch>) -> windows_core::Result<()>;
    fn DeleteAsync(&self, objwbemsink: windows_core::Ref<super::IDispatch>, strobjectpath: &windows_core::BSTR, iflags: i32, objwbemnamedvalueset: windows_core::Ref<super::IDispatch>, objwbemasynccontext: windows_core::Ref<super::IDispatch>) -> windows_core::Result<()>;
    fn InstancesOf(&self, strclass: &windows_core::BSTR, iflags: i32, objwbemnamedvalueset: windows_core::Ref<super::IDispatch>) -> windows_core::Result<ISWbemObjectSet>;
    fn InstancesOfAsync(&self, objwbemsink: windows_core::Ref<super::IDispatch>, strclass: &windows_core::BSTR, iflags: i32, objwbemnamedvalueset: windows_core::Ref<super::IDispatch>, objwbemasynccontext: windows_core::Ref<super::IDispatch>) -> windows_core::Result<()>;
    fn SubclassesOf(&self, strsuperclass: &windows_core::BSTR, iflags: i32, objwbemnamedvalueset: windows_core::Ref<super::IDispatch>) -> windows_core::Result<ISWbemObjectSet>;
    fn SubclassesOfAsync(&self, objwbemsink: windows_core::Ref<super::IDispatch>, strsuperclass: &windows_core::BSTR, iflags: i32, objwbemnamedvalueset: windows_core::Ref<super::IDispatch>, objwbemasynccontext: windows_core::Ref<super::IDispatch>) -> windows_core::Result<()>;
    fn ExecQuery(&self, strquery: &windows_core::BSTR, strquerylanguage: &windows_core::BSTR, iflags: i32, objwbemnamedvalueset: windows_core::Ref<super::IDispatch>) -> windows_core::Result<ISWbemObjectSet>;
    fn ExecQueryAsync(&self, objwbemsink: windows_core::Ref<super::IDispatch>, strquery: &windows_core::BSTR, strquerylanguage: &windows_core::BSTR, lflags: i32, objwbemnamedvalueset: windows_core::Ref<super::IDispatch>, objwbemasynccontext: windows_core::Ref<super::IDispatch>) -> windows_core::Result<()>;
    fn AssociatorsOf(&self, strobjectpath: &windows_core::BSTR, strassocclass: &windows_core::BSTR, strresultclass: &windows_core::BSTR, strresultrole: &windows_core::BSTR, strrole: &windows_core::BSTR, bclassesonly: super::VARIANT_BOOL, bschemaonly: super::VARIANT_BOOL, strrequiredassocqualifier: &windows_core::BSTR, strrequiredqualifier: &windows_core::BSTR, iflags: i32, objwbemnamedvalueset: windows_core::Ref<super::IDispatch>) -> windows_core::Result<ISWbemObjectSet>;
    fn AssociatorsOfAsync(&self, objwbemsink: windows_core::Ref<super::IDispatch>, strobjectpath: &windows_core::BSTR, strassocclass: &windows_core::BSTR, strresultclass: &windows_core::BSTR, strresultrole: &windows_core::BSTR, strrole: &windows_core::BSTR, bclassesonly: super::VARIANT_BOOL, bschemaonly: super::VARIANT_BOOL, strrequiredassocqualifier: &windows_core::BSTR, strrequiredqualifier: &windows_core::BSTR, iflags: i32, objwbemnamedvalueset: windows_core::Ref<super::IDispatch>, objwbemasynccontext: windows_core::Ref<super::IDispatch>) -> windows_core::Result<()>;
    fn ReferencesTo(&self, strobjectpath: &windows_core::BSTR, strresultclass: &windows_core::BSTR, strrole: &windows_core::BSTR, bclassesonly: super::VARIANT_BOOL, bschemaonly: super::VARIANT_BOOL, strrequiredqualifier: &windows_core::BSTR, iflags: i32, objwbemnamedvalueset: windows_core::Ref<super::IDispatch>) -> windows_core::Result<ISWbemObjectSet>;
    fn ReferencesToAsync(&self, objwbemsink: windows_core::Ref<super::IDispatch>, strobjectpath: &windows_core::BSTR, strresultclass: &windows_core::BSTR, strrole: &windows_core::BSTR, bclassesonly: super::VARIANT_BOOL, bschemaonly: super::VARIANT_BOOL, strrequiredqualifier: &windows_core::BSTR, iflags: i32, objwbemnamedvalueset: windows_core::Ref<super::IDispatch>, objwbemasynccontext: windows_core::Ref<super::IDispatch>) -> windows_core::Result<()>;
    fn ExecNotificationQuery(&self, strquery: &windows_core::BSTR, strquerylanguage: &windows_core::BSTR, iflags: i32, objwbemnamedvalueset: windows_core::Ref<super::IDispatch>) -> windows_core::Result<ISWbemEventSource>;
    fn ExecNotificationQueryAsync(&self, objwbemsink: windows_core::Ref<super::IDispatch>, strquery: &windows_core::BSTR, strquerylanguage: &windows_core::BSTR, iflags: i32, objwbemnamedvalueset: windows_core::Ref<super::IDispatch>, objwbemasynccontext: windows_core::Ref<super::IDispatch>) -> windows_core::Result<()>;
    fn ExecMethod(&self, strobjectpath: &windows_core::BSTR, strmethodname: &windows_core::BSTR, objwbeminparameters: windows_core::Ref<super::IDispatch>, iflags: i32, objwbemnamedvalueset: windows_core::Ref<super::IDispatch>) -> windows_core::Result<ISWbemObject>;
    fn ExecMethodAsync(&self, objwbemsink: windows_core::Ref<super::IDispatch>, strobjectpath: &windows_core::BSTR, strmethodname: &windows_core::BSTR, objwbeminparameters: windows_core::Ref<super::IDispatch>, iflags: i32, objwbemnamedvalueset: windows_core::Ref<super::IDispatch>, objwbemasynccontext: windows_core::Ref<super::IDispatch>) -> windows_core::Result<()>;
    fn get_Security_(&self) -> windows_core::Result<ISWbemSecurity>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl ISWbemServices_Vtbl {
    pub const fn new<Identity: ISWbemServices_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Get<Identity: ISWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strobjectpath: *mut core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void, objwbemobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemServices_Impl::Get(this, core::mem::transmute(&strobjectpath), core::mem::transmute_copy(&iflags), core::mem::transmute_copy(&objwbemnamedvalueset)) {
                    Ok(ok__) => {
                        objwbemobject.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetAsync<Identity: ISWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemsink: *mut core::ffi::c_void, strobjectpath: *mut core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void, objwbemasynccontext: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISWbemServices_Impl::GetAsync(this, core::mem::transmute_copy(&objwbemsink), core::mem::transmute(&strobjectpath), core::mem::transmute_copy(&iflags), core::mem::transmute_copy(&objwbemnamedvalueset), core::mem::transmute_copy(&objwbemasynccontext)).into()
            }
        }
        unsafe extern "system" fn Delete<Identity: ISWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strobjectpath: *mut core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISWbemServices_Impl::Delete(this, core::mem::transmute(&strobjectpath), core::mem::transmute_copy(&iflags), core::mem::transmute_copy(&objwbemnamedvalueset)).into()
            }
        }
        unsafe extern "system" fn DeleteAsync<Identity: ISWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemsink: *mut core::ffi::c_void, strobjectpath: *mut core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void, objwbemasynccontext: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISWbemServices_Impl::DeleteAsync(this, core::mem::transmute_copy(&objwbemsink), core::mem::transmute(&strobjectpath), core::mem::transmute_copy(&iflags), core::mem::transmute_copy(&objwbemnamedvalueset), core::mem::transmute_copy(&objwbemasynccontext)).into()
            }
        }
        unsafe extern "system" fn InstancesOf<Identity: ISWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strclass: *mut core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void, objwbemobjectset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemServices_Impl::InstancesOf(this, core::mem::transmute(&strclass), core::mem::transmute_copy(&iflags), core::mem::transmute_copy(&objwbemnamedvalueset)) {
                    Ok(ok__) => {
                        objwbemobjectset.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn InstancesOfAsync<Identity: ISWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemsink: *mut core::ffi::c_void, strclass: *mut core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void, objwbemasynccontext: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISWbemServices_Impl::InstancesOfAsync(this, core::mem::transmute_copy(&objwbemsink), core::mem::transmute(&strclass), core::mem::transmute_copy(&iflags), core::mem::transmute_copy(&objwbemnamedvalueset), core::mem::transmute_copy(&objwbemasynccontext)).into()
            }
        }
        unsafe extern "system" fn SubclassesOf<Identity: ISWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strsuperclass: *mut core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void, objwbemobjectset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemServices_Impl::SubclassesOf(this, core::mem::transmute(&strsuperclass), core::mem::transmute_copy(&iflags), core::mem::transmute_copy(&objwbemnamedvalueset)) {
                    Ok(ok__) => {
                        objwbemobjectset.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SubclassesOfAsync<Identity: ISWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemsink: *mut core::ffi::c_void, strsuperclass: *mut core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void, objwbemasynccontext: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISWbemServices_Impl::SubclassesOfAsync(this, core::mem::transmute_copy(&objwbemsink), core::mem::transmute(&strsuperclass), core::mem::transmute_copy(&iflags), core::mem::transmute_copy(&objwbemnamedvalueset), core::mem::transmute_copy(&objwbemasynccontext)).into()
            }
        }
        unsafe extern "system" fn ExecQuery<Identity: ISWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strquery: *mut core::ffi::c_void, strquerylanguage: *mut core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void, objwbemobjectset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemServices_Impl::ExecQuery(this, core::mem::transmute(&strquery), core::mem::transmute(&strquerylanguage), core::mem::transmute_copy(&iflags), core::mem::transmute_copy(&objwbemnamedvalueset)) {
                    Ok(ok__) => {
                        objwbemobjectset.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ExecQueryAsync<Identity: ISWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemsink: *mut core::ffi::c_void, strquery: *mut core::ffi::c_void, strquerylanguage: *mut core::ffi::c_void, lflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void, objwbemasynccontext: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISWbemServices_Impl::ExecQueryAsync(this, core::mem::transmute_copy(&objwbemsink), core::mem::transmute(&strquery), core::mem::transmute(&strquerylanguage), core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&objwbemnamedvalueset), core::mem::transmute_copy(&objwbemasynccontext)).into()
            }
        }
        unsafe extern "system" fn AssociatorsOf<Identity: ISWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strobjectpath: *mut core::ffi::c_void, strassocclass: *mut core::ffi::c_void, strresultclass: *mut core::ffi::c_void, strresultrole: *mut core::ffi::c_void, strrole: *mut core::ffi::c_void, bclassesonly: super::VARIANT_BOOL, bschemaonly: super::VARIANT_BOOL, strrequiredassocqualifier: *mut core::ffi::c_void, strrequiredqualifier: *mut core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void, objwbemobjectset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemServices_Impl::AssociatorsOf(this, core::mem::transmute(&strobjectpath), core::mem::transmute(&strassocclass), core::mem::transmute(&strresultclass), core::mem::transmute(&strresultrole), core::mem::transmute(&strrole), core::mem::transmute_copy(&bclassesonly), core::mem::transmute_copy(&bschemaonly), core::mem::transmute(&strrequiredassocqualifier), core::mem::transmute(&strrequiredqualifier), core::mem::transmute_copy(&iflags), core::mem::transmute_copy(&objwbemnamedvalueset)) {
                    Ok(ok__) => {
                        objwbemobjectset.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AssociatorsOfAsync<Identity: ISWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemsink: *mut core::ffi::c_void, strobjectpath: *mut core::ffi::c_void, strassocclass: *mut core::ffi::c_void, strresultclass: *mut core::ffi::c_void, strresultrole: *mut core::ffi::c_void, strrole: *mut core::ffi::c_void, bclassesonly: super::VARIANT_BOOL, bschemaonly: super::VARIANT_BOOL, strrequiredassocqualifier: *mut core::ffi::c_void, strrequiredqualifier: *mut core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void, objwbemasynccontext: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISWbemServices_Impl::AssociatorsOfAsync(
                    this,
                    core::mem::transmute_copy(&objwbemsink),
                    core::mem::transmute(&strobjectpath),
                    core::mem::transmute(&strassocclass),
                    core::mem::transmute(&strresultclass),
                    core::mem::transmute(&strresultrole),
                    core::mem::transmute(&strrole),
                    core::mem::transmute_copy(&bclassesonly),
                    core::mem::transmute_copy(&bschemaonly),
                    core::mem::transmute(&strrequiredassocqualifier),
                    core::mem::transmute(&strrequiredqualifier),
                    core::mem::transmute_copy(&iflags),
                    core::mem::transmute_copy(&objwbemnamedvalueset),
                    core::mem::transmute_copy(&objwbemasynccontext),
                )
                .into()
            }
        }
        unsafe extern "system" fn ReferencesTo<Identity: ISWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strobjectpath: *mut core::ffi::c_void, strresultclass: *mut core::ffi::c_void, strrole: *mut core::ffi::c_void, bclassesonly: super::VARIANT_BOOL, bschemaonly: super::VARIANT_BOOL, strrequiredqualifier: *mut core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void, objwbemobjectset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemServices_Impl::ReferencesTo(this, core::mem::transmute(&strobjectpath), core::mem::transmute(&strresultclass), core::mem::transmute(&strrole), core::mem::transmute_copy(&bclassesonly), core::mem::transmute_copy(&bschemaonly), core::mem::transmute(&strrequiredqualifier), core::mem::transmute_copy(&iflags), core::mem::transmute_copy(&objwbemnamedvalueset)) {
                    Ok(ok__) => {
                        objwbemobjectset.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ReferencesToAsync<Identity: ISWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemsink: *mut core::ffi::c_void, strobjectpath: *mut core::ffi::c_void, strresultclass: *mut core::ffi::c_void, strrole: *mut core::ffi::c_void, bclassesonly: super::VARIANT_BOOL, bschemaonly: super::VARIANT_BOOL, strrequiredqualifier: *mut core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void, objwbemasynccontext: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISWbemServices_Impl::ReferencesToAsync(this, core::mem::transmute_copy(&objwbemsink), core::mem::transmute(&strobjectpath), core::mem::transmute(&strresultclass), core::mem::transmute(&strrole), core::mem::transmute_copy(&bclassesonly), core::mem::transmute_copy(&bschemaonly), core::mem::transmute(&strrequiredqualifier), core::mem::transmute_copy(&iflags), core::mem::transmute_copy(&objwbemnamedvalueset), core::mem::transmute_copy(&objwbemasynccontext)).into()
            }
        }
        unsafe extern "system" fn ExecNotificationQuery<Identity: ISWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strquery: *mut core::ffi::c_void, strquerylanguage: *mut core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void, objwbemeventsource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemServices_Impl::ExecNotificationQuery(this, core::mem::transmute(&strquery), core::mem::transmute(&strquerylanguage), core::mem::transmute_copy(&iflags), core::mem::transmute_copy(&objwbemnamedvalueset)) {
                    Ok(ok__) => {
                        objwbemeventsource.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ExecNotificationQueryAsync<Identity: ISWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemsink: *mut core::ffi::c_void, strquery: *mut core::ffi::c_void, strquerylanguage: *mut core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void, objwbemasynccontext: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISWbemServices_Impl::ExecNotificationQueryAsync(this, core::mem::transmute_copy(&objwbemsink), core::mem::transmute(&strquery), core::mem::transmute(&strquerylanguage), core::mem::transmute_copy(&iflags), core::mem::transmute_copy(&objwbemnamedvalueset), core::mem::transmute_copy(&objwbemasynccontext)).into()
            }
        }
        unsafe extern "system" fn ExecMethod<Identity: ISWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strobjectpath: *mut core::ffi::c_void, strmethodname: *mut core::ffi::c_void, objwbeminparameters: *mut core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void, objwbemoutparameters: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemServices_Impl::ExecMethod(this, core::mem::transmute(&strobjectpath), core::mem::transmute(&strmethodname), core::mem::transmute_copy(&objwbeminparameters), core::mem::transmute_copy(&iflags), core::mem::transmute_copy(&objwbemnamedvalueset)) {
                    Ok(ok__) => {
                        objwbemoutparameters.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ExecMethodAsync<Identity: ISWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemsink: *mut core::ffi::c_void, strobjectpath: *mut core::ffi::c_void, strmethodname: *mut core::ffi::c_void, objwbeminparameters: *mut core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void, objwbemasynccontext: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISWbemServices_Impl::ExecMethodAsync(this, core::mem::transmute_copy(&objwbemsink), core::mem::transmute(&strobjectpath), core::mem::transmute(&strmethodname), core::mem::transmute_copy(&objwbeminparameters), core::mem::transmute_copy(&iflags), core::mem::transmute_copy(&objwbemnamedvalueset), core::mem::transmute_copy(&objwbemasynccontext)).into()
            }
        }
        unsafe extern "system" fn get_Security_<Identity: ISWbemServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemsecurity: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemServices_Impl::get_Security_(this) {
                    Ok(ok__) => {
                        objwbemsecurity.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Get: Get::<Identity, OFFSET>,
            GetAsync: GetAsync::<Identity, OFFSET>,
            Delete: Delete::<Identity, OFFSET>,
            DeleteAsync: DeleteAsync::<Identity, OFFSET>,
            InstancesOf: InstancesOf::<Identity, OFFSET>,
            InstancesOfAsync: InstancesOfAsync::<Identity, OFFSET>,
            SubclassesOf: SubclassesOf::<Identity, OFFSET>,
            SubclassesOfAsync: SubclassesOfAsync::<Identity, OFFSET>,
            ExecQuery: ExecQuery::<Identity, OFFSET>,
            ExecQueryAsync: ExecQueryAsync::<Identity, OFFSET>,
            AssociatorsOf: AssociatorsOf::<Identity, OFFSET>,
            AssociatorsOfAsync: AssociatorsOfAsync::<Identity, OFFSET>,
            ReferencesTo: ReferencesTo::<Identity, OFFSET>,
            ReferencesToAsync: ReferencesToAsync::<Identity, OFFSET>,
            ExecNotificationQuery: ExecNotificationQuery::<Identity, OFFSET>,
            ExecNotificationQueryAsync: ExecNotificationQueryAsync::<Identity, OFFSET>,
            ExecMethod: ExecMethod::<Identity, OFFSET>,
            ExecMethodAsync: ExecMethodAsync::<Identity, OFFSET>,
            get_Security_: get_Security_::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISWbemServices as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for ISWbemServices {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(ISWbemServicesEx, ISWbemServicesEx_Vtbl, 0xd2f68443_85dc_427e_91d8_366554cc754c);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for ISWbemServicesEx {
    type Target = ISWbemServices;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(ISWbemServicesEx, windows_core::IUnknown, super::IDispatch, ISWbemServices);
#[cfg(feature = "oaidl")]
impl ISWbemServicesEx {
    pub unsafe fn Put<P0, P2>(&self, objwbemobject: P0, iflags: i32, objwbemnamedvalueset: P2) -> windows_core::Result<ISWbemObjectPath>
    where
        P0: windows_core::Param<ISWbemObjectEx>,
        P2: windows_core::Param<super::IDispatch>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Put)(windows_core::Interface::as_raw(self), objwbemobject.param().abi(), iflags, objwbemnamedvalueset.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn PutAsync<P0, P1, P3, P4>(&self, objwbemsink: P0, objwbemobject: P1, iflags: i32, objwbemnamedvalueset: P3, objwbemasynccontext: P4) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ISWbemSink>,
        P1: windows_core::Param<ISWbemObjectEx>,
        P3: windows_core::Param<super::IDispatch>,
        P4: windows_core::Param<super::IDispatch>,
    {
        unsafe { (windows_core::Interface::vtable(self).PutAsync)(windows_core::Interface::as_raw(self), objwbemsink.param().abi(), objwbemobject.param().abi(), iflags, objwbemnamedvalueset.param().abi(), objwbemasynccontext.param().abi()) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct ISWbemServicesEx_Vtbl {
    pub base__: ISWbemServices_Vtbl,
    pub Put: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PutAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait ISWbemServicesEx_Impl: ISWbemServices_Impl {
    fn Put(&self, objwbemobject: windows_core::Ref<ISWbemObjectEx>, iflags: i32, objwbemnamedvalueset: windows_core::Ref<super::IDispatch>) -> windows_core::Result<ISWbemObjectPath>;
    fn PutAsync(&self, objwbemsink: windows_core::Ref<ISWbemSink>, objwbemobject: windows_core::Ref<ISWbemObjectEx>, iflags: i32, objwbemnamedvalueset: windows_core::Ref<super::IDispatch>, objwbemasynccontext: windows_core::Ref<super::IDispatch>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl ISWbemServicesEx_Vtbl {
    pub const fn new<Identity: ISWbemServicesEx_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Put<Identity: ISWbemServicesEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemobject: *mut core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void, objwbemobjectpath: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISWbemServicesEx_Impl::Put(this, core::mem::transmute_copy(&objwbemobject), core::mem::transmute_copy(&iflags), core::mem::transmute_copy(&objwbemnamedvalueset)) {
                    Ok(ok__) => {
                        objwbemobjectpath.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PutAsync<Identity: ISWbemServicesEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objwbemsink: *mut core::ffi::c_void, objwbemobject: *mut core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut core::ffi::c_void, objwbemasynccontext: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISWbemServicesEx_Impl::PutAsync(this, core::mem::transmute_copy(&objwbemsink), core::mem::transmute_copy(&objwbemobject), core::mem::transmute_copy(&iflags), core::mem::transmute_copy(&objwbemnamedvalueset), core::mem::transmute_copy(&objwbemasynccontext)).into()
            }
        }
        Self { base__: ISWbemServices_Vtbl::new::<Identity, OFFSET>(), Put: Put::<Identity, OFFSET>, PutAsync: PutAsync::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISWbemServicesEx as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<ISWbemServices as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for ISWbemServicesEx {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(ISWbemSink, ISWbemSink_Vtbl, 0x75718c9f_f029_11d1_a1ac_00c04fb6c223);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for ISWbemSink {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(ISWbemSink, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl ISWbemSink {
    pub unsafe fn Cancel(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Cancel)(windows_core::Interface::as_raw(self)) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct ISWbemSink_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub Cancel: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait ISWbemSink_Impl: super::IDispatch_Impl {
    fn Cancel(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl ISWbemSink_Vtbl {
    pub const fn new<Identity: ISWbemSink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Cancel<Identity: ISWbemSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISWbemSink_Impl::Cancel(this).into()
            }
        }
        Self { base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(), Cancel: Cancel::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISWbemSink as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for ISWbemSink {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(ISWbemSinkEvents, ISWbemSinkEvents_Vtbl, 0x75718ca0_f029_11d1_a1ac_00c04fb6c223);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for ISWbemSinkEvents {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(ISWbemSinkEvents, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct ISWbemSinkEvents_Vtbl {
    pub base__: super::IDispatch_Vtbl,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait ISWbemSinkEvents_Impl: super::IDispatch_Impl {}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl ISWbemSinkEvents_Vtbl {
    pub const fn new<Identity: ISWbemSinkEvents_Impl, const OFFSET: isize>() -> Self {
        Self { base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISWbemSinkEvents as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for ISWbemSinkEvents {}
pub const SWbemDateTime: windows_core::GUID = windows_core::GUID::from_u128(0x47dfbe54_cf76_11d3_b38f_00105a1f473a);
pub const SWbemEventSource: windows_core::GUID = windows_core::GUID::from_u128(0x04b83d58_21ae_11d2_8b33_00600806d9b6);
pub const SWbemLastError: windows_core::GUID = windows_core::GUID::from_u128(0xc2feeeac_cfcd_11d1_8b05_00600806d9b6);
pub const SWbemLocator: windows_core::GUID = windows_core::GUID::from_u128(0x76a64158_cb41_11d1_8b02_00600806d9b6);
pub const SWbemMethod: windows_core::GUID = windows_core::GUID::from_u128(0x04b83d5b_21ae_11d2_8b33_00600806d9b6);
pub const SWbemMethodSet: windows_core::GUID = windows_core::GUID::from_u128(0x04b83d5a_21ae_11d2_8b33_00600806d9b6);
pub const SWbemNamedValue: windows_core::GUID = windows_core::GUID::from_u128(0x04b83d60_21ae_11d2_8b33_00600806d9b6);
pub const SWbemNamedValueSet: windows_core::GUID = windows_core::GUID::from_u128(0x9aed384e_ce8b_11d1_8b05_00600806d9b6);
pub const SWbemObject: windows_core::GUID = windows_core::GUID::from_u128(0x04b83d62_21ae_11d2_8b33_00600806d9b6);
pub const SWbemObjectEx: windows_core::GUID = windows_core::GUID::from_u128(0xd6bdafb2_9435_491f_bb87_6aa0f0bc31a2);
pub const SWbemObjectPath: windows_core::GUID = windows_core::GUID::from_u128(0x5791bc26_ce9c_11d1_97bf_0000f81e849c);
pub const SWbemObjectSet: windows_core::GUID = windows_core::GUID::from_u128(0x04b83d61_21ae_11d2_8b33_00600806d9b6);
pub const SWbemPrivilege: windows_core::GUID = windows_core::GUID::from_u128(0x26ee67bc_5804_11d2_8b4a_00600806d9b6);
pub const SWbemPrivilegeSet: windows_core::GUID = windows_core::GUID::from_u128(0x26ee67be_5804_11d2_8b4a_00600806d9b6);
pub const SWbemProperty: windows_core::GUID = windows_core::GUID::from_u128(0x04b83d5d_21ae_11d2_8b33_00600806d9b6);
pub const SWbemPropertySet: windows_core::GUID = windows_core::GUID::from_u128(0x04b83d5c_21ae_11d2_8b33_00600806d9b6);
pub const SWbemQualifier: windows_core::GUID = windows_core::GUID::from_u128(0x04b83d5f_21ae_11d2_8b33_00600806d9b6);
pub const SWbemQualifierSet: windows_core::GUID = windows_core::GUID::from_u128(0x04b83d5e_21ae_11d2_8b33_00600806d9b6);
pub const SWbemRefreshableItem: windows_core::GUID = windows_core::GUID::from_u128(0x8c6854bc_de4b_11d3_b390_00105a1f473a);
pub const SWbemRefresher: windows_core::GUID = windows_core::GUID::from_u128(0xd269bf5c_d9c1_11d3_b38f_00105a1f473a);
pub const SWbemSecurity: windows_core::GUID = windows_core::GUID::from_u128(0xb54d66e9_2287_11d2_8b33_00600806d9b6);
pub const SWbemServices: windows_core::GUID = windows_core::GUID::from_u128(0x04b83d63_21ae_11d2_8b33_00600806d9b6);
pub const SWbemServicesEx: windows_core::GUID = windows_core::GUID::from_u128(0x62e522dc_8cf3_40a8_8b2e_37d595651e40);
pub const SWbemSink: windows_core::GUID = windows_core::GUID::from_u128(0x75718c9a_f029_11d1_a1ac_00c04fb6c223);
pub const WBEMS_DISPID_COMPLETED: i32 = 2;
pub const WBEMS_DISPID_CONNECTION_READY: i32 = 5;
pub const WBEMS_DISPID_DERIVATION: i32 = 23;
pub const WBEMS_DISPID_OBJECT_PUT: i32 = 4;
pub const WBEMS_DISPID_OBJECT_READY: i32 = 1;
pub const WBEMS_DISPID_PROGRESS: i32 = 3;
pub type WbemAuthenticationLevelEnum = i32;
pub type WbemChangeFlagEnum = i32;
pub type WbemCimtypeEnum = i32;
pub type WbemComparisonFlagEnum = i32;
pub type WbemConnectOptionsEnum = i32;
pub type WbemErrorEnum = i32;
pub type WbemFlagEnum = i32;
pub type WbemImpersonationLevelEnum = i32;
pub type WbemObjectTextFormatEnum = i32;
pub type WbemPrivilegeEnum = i32;
pub type WbemQueryFlagEnum = i32;
pub type WbemTextFlagEnum = i32;
pub type WbemTimeout = i32;
pub const wbemAuthenticationLevelCall: WbemAuthenticationLevelEnum = 3;
pub const wbemAuthenticationLevelConnect: WbemAuthenticationLevelEnum = 2;
pub const wbemAuthenticationLevelDefault: WbemAuthenticationLevelEnum = 0;
pub const wbemAuthenticationLevelNone: WbemAuthenticationLevelEnum = 1;
pub const wbemAuthenticationLevelPkt: WbemAuthenticationLevelEnum = 4;
pub const wbemAuthenticationLevelPktIntegrity: WbemAuthenticationLevelEnum = 5;
pub const wbemAuthenticationLevelPktPrivacy: WbemAuthenticationLevelEnum = 6;
pub const wbemChangeFlagAdvisory: WbemChangeFlagEnum = 65536;
pub const wbemChangeFlagCreateOnly: WbemChangeFlagEnum = 2;
pub const wbemChangeFlagCreateOrUpdate: WbemChangeFlagEnum = 0;
pub const wbemChangeFlagStrongValidation: WbemChangeFlagEnum = 128;
pub const wbemChangeFlagUpdateCompatible: WbemChangeFlagEnum = 0;
pub const wbemChangeFlagUpdateForceMode: WbemChangeFlagEnum = 64;
pub const wbemChangeFlagUpdateOnly: WbemChangeFlagEnum = 1;
pub const wbemChangeFlagUpdateSafeMode: WbemChangeFlagEnum = 32;
pub const wbemCimtypeBoolean: WbemCimtypeEnum = 11;
pub const wbemCimtypeChar16: WbemCimtypeEnum = 103;
pub const wbemCimtypeDatetime: WbemCimtypeEnum = 101;
pub const wbemCimtypeObject: WbemCimtypeEnum = 13;
pub const wbemCimtypeReal32: WbemCimtypeEnum = 4;
pub const wbemCimtypeReal64: WbemCimtypeEnum = 5;
pub const wbemCimtypeReference: WbemCimtypeEnum = 102;
pub const wbemCimtypeSint16: WbemCimtypeEnum = 2;
pub const wbemCimtypeSint32: WbemCimtypeEnum = 3;
pub const wbemCimtypeSint64: WbemCimtypeEnum = 20;
pub const wbemCimtypeSint8: WbemCimtypeEnum = 16;
pub const wbemCimtypeString: WbemCimtypeEnum = 8;
pub const wbemCimtypeUint16: WbemCimtypeEnum = 18;
pub const wbemCimtypeUint32: WbemCimtypeEnum = 19;
pub const wbemCimtypeUint64: WbemCimtypeEnum = 21;
pub const wbemCimtypeUint8: WbemCimtypeEnum = 17;
pub const wbemComparisonFlagIgnoreCase: WbemComparisonFlagEnum = 16;
pub const wbemComparisonFlagIgnoreClass: WbemComparisonFlagEnum = 8;
pub const wbemComparisonFlagIgnoreDefaultValues: WbemComparisonFlagEnum = 4;
pub const wbemComparisonFlagIgnoreFlavor: WbemComparisonFlagEnum = 32;
pub const wbemComparisonFlagIgnoreObjectSource: WbemComparisonFlagEnum = 2;
pub const wbemComparisonFlagIgnoreQualifiers: WbemComparisonFlagEnum = 1;
pub const wbemComparisonFlagIncludeAll: WbemComparisonFlagEnum = 0;
pub const wbemConnectFlagUseMaxWait: WbemConnectOptionsEnum = 128;
pub const wbemErrAccessDenied: WbemErrorEnum = -2147217405;
pub const wbemErrAggregatingByObject: WbemErrorEnum = -2147217315;
pub const wbemErrAlreadyExists: WbemErrorEnum = -2147217383;
pub const wbemErrAmbiguousOperation: WbemErrorEnum = -2147217301;
pub const wbemErrAmendedObject: WbemErrorEnum = -2147217306;
pub const wbemErrBackupRestoreWinmgmtRunning: WbemErrorEnum = -2147217312;
pub const wbemErrBufferTooSmall: WbemErrorEnum = -2147217348;
pub const wbemErrCallCancelled: WbemErrorEnum = -2147217358;
pub const wbemErrCannotBeAbstract: WbemErrorEnum = -2147217307;
pub const wbemErrCannotBeKey: WbemErrorEnum = -2147217377;
pub const wbemErrCannotBeSingleton: WbemErrorEnum = -2147217364;
pub const wbemErrCannotChangeIndexInheritance: WbemErrorEnum = -2147217328;
pub const wbemErrCannotChangeKeyInheritance: WbemErrorEnum = -2147217335;
pub const wbemErrCircularReference: WbemErrorEnum = -2147217337;
pub const wbemErrClassHasChildren: WbemErrorEnum = -2147217371;
pub const wbemErrClassHasInstances: WbemErrorEnum = -2147217370;
pub const wbemErrClassNameTooWide: WbemErrorEnum = -2147217292;
pub const wbemErrClientTooSlow: WbemErrorEnum = -2147217305;
pub const wbemErrConnectionFailed: WbemErrorEnum = -2147217295;
pub const wbemErrCriticalError: WbemErrorEnum = -2147217398;
pub const wbemErrDatabaseVerMismatch: WbemErrorEnum = -2147217288;
pub const wbemErrEncryptedConnectionRequired: WbemErrorEnum = -2147217273;
pub const wbemErrFailed: WbemErrorEnum = -2147217407;
pub const wbemErrFatalTransportError: WbemErrorEnum = -2147217274;
pub const wbemErrForcedRollback: WbemErrorEnum = -2147217298;
pub const wbemErrHandleOutOfDate: WbemErrorEnum = -2147217296;
pub const wbemErrIllegalNull: WbemErrorEnum = -2147217368;
pub const wbemErrIllegalOperation: WbemErrorEnum = -2147217378;
pub const wbemErrIncompleteClass: WbemErrorEnum = -2147217376;
pub const wbemErrInitializationFailure: WbemErrorEnum = -2147217388;
pub const wbemErrInvalidAssociation: WbemErrorEnum = -2147217302;
pub const wbemErrInvalidCimType: WbemErrorEnum = -2147217363;
pub const wbemErrInvalidClass: WbemErrorEnum = -2147217392;
pub const wbemErrInvalidContext: WbemErrorEnum = -2147217401;
pub const wbemErrInvalidDuplicateParameter: WbemErrorEnum = -2147217341;
pub const wbemErrInvalidFlavor: WbemErrorEnum = -2147217338;
pub const wbemErrInvalidHandleRequest: WbemErrorEnum = -2147217294;
pub const wbemErrInvalidLocale: WbemErrorEnum = -2147217280;
pub const wbemErrInvalidMethod: WbemErrorEnum = -2147217362;
pub const wbemErrInvalidMethodParameters: WbemErrorEnum = -2147217361;
pub const wbemErrInvalidNamespace: WbemErrorEnum = -2147217394;
pub const wbemErrInvalidObject: WbemErrorEnum = -2147217393;
pub const wbemErrInvalidObjectPath: WbemErrorEnum = -2147217350;
pub const wbemErrInvalidOperation: WbemErrorEnum = -2147217386;
pub const wbemErrInvalidOperator: WbemErrorEnum = -2147217309;
pub const wbemErrInvalidParameter: WbemErrorEnum = -2147217400;
pub const wbemErrInvalidParameterId: WbemErrorEnum = -2147217353;
pub const wbemErrInvalidProperty: WbemErrorEnum = -2147217359;
pub const wbemErrInvalidPropertyType: WbemErrorEnum = -2147217366;
pub const wbemErrInvalidProviderRegistration: WbemErrorEnum = -2147217390;
pub const wbemErrInvalidQualifier: WbemErrorEnum = -2147217342;
pub const wbemErrInvalidQualifierType: WbemErrorEnum = -2147217367;
pub const wbemErrInvalidQuery: WbemErrorEnum = -2147217385;
pub const wbemErrInvalidQueryType: WbemErrorEnum = -2147217384;
pub const wbemErrInvalidStream: WbemErrorEnum = -2147217397;
pub const wbemErrInvalidSuperclass: WbemErrorEnum = -2147217395;
pub const wbemErrInvalidSyntax: WbemErrorEnum = -2147217375;
pub const wbemErrLocalCredentials: WbemErrorEnum = -2147217308;
pub const wbemErrMarshalInvalidSignature: WbemErrorEnum = -2147217343;
pub const wbemErrMarshalVersionMismatch: WbemErrorEnum = -2147217344;
pub const wbemErrMethodDisabled: WbemErrorEnum = -2147217322;
pub const wbemErrMethodNameTooWide: WbemErrorEnum = -2147217291;
pub const wbemErrMethodNotImplemented: WbemErrorEnum = -2147217323;
pub const wbemErrMissingAggregationList: WbemErrorEnum = -2147217317;
pub const wbemErrMissingGroupWithin: WbemErrorEnum = -2147217318;
pub const wbemErrMissingParameter: WbemErrorEnum = -2147217354;
pub const wbemErrNoSchema: WbemErrorEnum = -2147217277;
pub const wbemErrNonConsecutiveParameterIds: WbemErrorEnum = -2147217352;
pub const wbemErrNondecoratedObject: WbemErrorEnum = -2147217374;
pub const wbemErrNotAvailable: WbemErrorEnum = -2147217399;
pub const wbemErrNotEventClass: WbemErrorEnum = -2147217319;
pub const wbemErrNotFound: WbemErrorEnum = -2147217406;
pub const wbemErrNotSupported: WbemErrorEnum = -2147217396;
pub const wbemErrNullSecurityDescriptor: WbemErrorEnum = -2147217304;
pub const wbemErrOutOfDiskSpace: WbemErrorEnum = -2147217349;
pub const wbemErrOutOfMemory: WbemErrorEnum = -2147217402;
pub const wbemErrOverrideNotAllowed: WbemErrorEnum = -2147217382;
pub const wbemErrParameterIdOnRetval: WbemErrorEnum = -2147217351;
pub const wbemErrPrivilegeNotHeld: WbemErrorEnum = -2147217310;
pub const wbemErrPropagatedMethod: WbemErrorEnum = -2147217356;
pub const wbemErrPropagatedProperty: WbemErrorEnum = -2147217380;
pub const wbemErrPropagatedQualifier: WbemErrorEnum = -2147217381;
pub const wbemErrPropertyNameTooWide: WbemErrorEnum = -2147217293;
pub const wbemErrPropertyNotAnObject: WbemErrorEnum = -2147217316;
pub const wbemErrProviderAlreadyRegistered: WbemErrorEnum = -2147217276;
pub const wbemErrProviderFailure: WbemErrorEnum = -2147217404;
pub const wbemErrProviderLoadFailure: WbemErrorEnum = -2147217389;
pub const wbemErrProviderNotCapable: WbemErrorEnum = -2147217372;
pub const wbemErrProviderNotFound: WbemErrorEnum = -2147217391;
pub const wbemErrProviderNotRegistered: WbemErrorEnum = -2147217275;
pub const wbemErrProviderSuspended: WbemErrorEnum = -2147217279;
pub const wbemErrQualifierNameTooWide: WbemErrorEnum = -2147217290;
pub const wbemErrQueryNotImplemented: WbemErrorEnum = -2147217369;
pub const wbemErrQueueOverflow: WbemErrorEnum = -2147217311;
pub const wbemErrQuotaViolation: WbemErrorEnum = -2147217300;
pub const wbemErrReadOnly: WbemErrorEnum = -2147217373;
pub const wbemErrRefresherBusy: WbemErrorEnum = -2147217321;
pub const wbemErrRegistrationTooBroad: WbemErrorEnum = -2147213311;
pub const wbemErrRegistrationTooPrecise: WbemErrorEnum = -2147213310;
pub const wbemErrRerunCommand: WbemErrorEnum = -2147217289;
pub const wbemErrResetToDefault: WbemErrorEnum = -2147209214;
pub const wbemErrServerTooBusy: WbemErrorEnum = -2147217339;
pub const wbemErrShuttingDown: WbemErrorEnum = -2147217357;
pub const wbemErrSynchronizationRequired: WbemErrorEnum = -2147217278;
pub const wbemErrSystemProperty: WbemErrorEnum = -2147217360;
pub const wbemErrTimedout: WbemErrorEnum = -2147209215;
pub const wbemErrTimeout: WbemErrorEnum = -2147217303;
pub const wbemErrTooManyProperties: WbemErrorEnum = -2147217327;
pub const wbemErrTooMuchData: WbemErrorEnum = -2147217340;
pub const wbemErrTransactionConflict: WbemErrorEnum = -2147217299;
pub const wbemErrTransportFailure: WbemErrorEnum = -2147217387;
pub const wbemErrTypeMismatch: WbemErrorEnum = -2147217403;
pub const wbemErrUnexpected: WbemErrorEnum = -2147217379;
pub const wbemErrUninterpretableProviderQuery: WbemErrorEnum = -2147217313;
pub const wbemErrUnknownObjectType: WbemErrorEnum = -2147217346;
pub const wbemErrUnknownPacketType: WbemErrorEnum = -2147217345;
pub const wbemErrUnparsableQuery: WbemErrorEnum = -2147217320;
pub const wbemErrUnsupportedClassUpdate: WbemErrorEnum = -2147217336;
pub const wbemErrUnsupportedLocale: WbemErrorEnum = -2147217297;
pub const wbemErrUnsupportedParameter: WbemErrorEnum = -2147217355;
pub const wbemErrUnsupportedPutExtension: WbemErrorEnum = -2147217347;
pub const wbemErrUpdateOverrideNotAllowed: WbemErrorEnum = -2147217325;
pub const wbemErrUpdatePropagatedMethod: WbemErrorEnum = -2147217324;
pub const wbemErrUpdateTypeMismatch: WbemErrorEnum = -2147217326;
pub const wbemErrValueOutOfRange: WbemErrorEnum = -2147217365;
pub const wbemErrVetoDelete: WbemErrorEnum = -2147217286;
pub const wbemErrVetoPut: WbemErrorEnum = -2147217287;
pub const wbemFlagBidirectional: WbemFlagEnum = 0;
pub const wbemFlagDirectRead: WbemFlagEnum = 512;
pub const wbemFlagDontSendStatus: WbemFlagEnum = 0;
pub const wbemFlagEnsureLocatable: WbemFlagEnum = 256;
pub const wbemFlagForwardOnly: WbemFlagEnum = 32;
pub const wbemFlagGetDefault: WbemFlagEnum = 0;
pub const wbemFlagNoErrorObject: WbemFlagEnum = 64;
pub const wbemFlagReturnErrorObject: WbemFlagEnum = 0;
pub const wbemFlagReturnImmediately: WbemFlagEnum = 16;
pub const wbemFlagReturnWhenComplete: WbemFlagEnum = 0;
pub const wbemFlagSendOnlySelected: WbemFlagEnum = 0;
pub const wbemFlagSendStatus: WbemFlagEnum = 128;
pub const wbemFlagSpawnInstance: WbemFlagEnum = 1;
pub const wbemFlagUseAmendedQualifiers: WbemFlagEnum = 131072;
pub const wbemFlagUseCurrentTime: WbemFlagEnum = 1;
pub const wbemImpersonationLevelAnonymous: WbemImpersonationLevelEnum = 1;
pub const wbemImpersonationLevelDelegate: WbemImpersonationLevelEnum = 4;
pub const wbemImpersonationLevelIdentify: WbemImpersonationLevelEnum = 2;
pub const wbemImpersonationLevelImpersonate: WbemImpersonationLevelEnum = 3;
pub const wbemNoErr: WbemErrorEnum = 0;
pub const wbemObjectTextFormatCIMDTD20: WbemObjectTextFormatEnum = 1;
pub const wbemObjectTextFormatWMIDTD20: WbemObjectTextFormatEnum = 2;
pub const wbemPrivilegeAudit: WbemPrivilegeEnum = 20;
pub const wbemPrivilegeBackup: WbemPrivilegeEnum = 16;
pub const wbemPrivilegeChangeNotify: WbemPrivilegeEnum = 22;
pub const wbemPrivilegeCreatePagefile: WbemPrivilegeEnum = 14;
pub const wbemPrivilegeCreatePermanent: WbemPrivilegeEnum = 15;
pub const wbemPrivilegeCreateToken: WbemPrivilegeEnum = 1;
pub const wbemPrivilegeDebug: WbemPrivilegeEnum = 19;
pub const wbemPrivilegeEnableDelegation: WbemPrivilegeEnum = 26;
pub const wbemPrivilegeIncreaseBasePriority: WbemPrivilegeEnum = 13;
pub const wbemPrivilegeIncreaseQuota: WbemPrivilegeEnum = 4;
pub const wbemPrivilegeLoadDriver: WbemPrivilegeEnum = 9;
pub const wbemPrivilegeLockMemory: WbemPrivilegeEnum = 3;
pub const wbemPrivilegeMachineAccount: WbemPrivilegeEnum = 5;
pub const wbemPrivilegeManageVolume: WbemPrivilegeEnum = 27;
pub const wbemPrivilegePrimaryToken: WbemPrivilegeEnum = 2;
pub const wbemPrivilegeProfileSingleProcess: WbemPrivilegeEnum = 12;
pub const wbemPrivilegeRemoteShutdown: WbemPrivilegeEnum = 23;
pub const wbemPrivilegeRestore: WbemPrivilegeEnum = 17;
pub const wbemPrivilegeSecurity: WbemPrivilegeEnum = 7;
pub const wbemPrivilegeShutdown: WbemPrivilegeEnum = 18;
pub const wbemPrivilegeSyncAgent: WbemPrivilegeEnum = 25;
pub const wbemPrivilegeSystemEnvironment: WbemPrivilegeEnum = 21;
pub const wbemPrivilegeSystemProfile: WbemPrivilegeEnum = 10;
pub const wbemPrivilegeSystemtime: WbemPrivilegeEnum = 11;
pub const wbemPrivilegeTakeOwnership: WbemPrivilegeEnum = 8;
pub const wbemPrivilegeTcb: WbemPrivilegeEnum = 6;
pub const wbemPrivilegeUndock: WbemPrivilegeEnum = 24;
pub const wbemQueryFlagDeep: WbemQueryFlagEnum = 0;
pub const wbemQueryFlagPrototype: WbemQueryFlagEnum = 2;
pub const wbemQueryFlagShallow: WbemQueryFlagEnum = 1;
pub const wbemTextFlagNoFlavors: WbemTextFlagEnum = 1;
pub const wbemTimeoutInfinite: WbemTimeout = -1;
