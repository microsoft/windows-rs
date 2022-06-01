pub trait IEnumWbemClassObject_Impl: Sized {
    fn Reset(&self) -> ::windows::core::Result<()>;
    fn Next(&self, ltimeout: i32, ucount: u32, apobjects: *mut ::core::option::Option<IWbemClassObject>, pureturned: *mut u32) -> ::windows::core::HRESULT;
    fn NextAsync(&self, ucount: u32, psink: &::core::option::Option<IWbemObjectSink>) -> ::windows::core::HRESULT;
    fn Clone(&self) -> ::windows::core::Result<IEnumWbemClassObject>;
    fn Skip(&self, ltimeout: i32, ncount: u32) -> ::windows::core::HRESULT;
}
impl ::windows::core::RuntimeName for IEnumWbemClassObject {}
impl IEnumWbemClassObject_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumWbemClassObject_Impl, const OFFSET: isize>() -> IEnumWbemClassObject_Vtbl {
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumWbemClassObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset().into()
        }
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumWbemClassObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ltimeout: i32, ucount: u32, apobjects: *mut *mut ::core::ffi::c_void, pureturned: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Next(::core::mem::transmute_copy(&ltimeout), ::core::mem::transmute_copy(&ucount), ::core::mem::transmute_copy(&apobjects), ::core::mem::transmute_copy(&pureturned))
        }
        unsafe extern "system" fn NextAsync<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumWbemClassObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ucount: u32, psink: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.NextAsync(::core::mem::transmute_copy(&ucount), ::core::mem::transmute(&psink))
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumWbemClassObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Skip<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumWbemClassObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ltimeout: i32, ncount: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Skip(::core::mem::transmute_copy(&ltimeout), ::core::mem::transmute_copy(&ncount))
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Reset: Reset::<Identity, Impl, OFFSET>,
            Next: Next::<Identity, Impl, OFFSET>,
            NextAsync: NextAsync::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumWbemClassObject as ::windows::core::Interface>::IID
    }
}
pub trait IMofCompiler_Impl: Sized {
    fn CompileFile(&self, filename: &::windows::core::PCWSTR, serverandnamespace: &::windows::core::PCWSTR, user: &::windows::core::PCWSTR, authority: &::windows::core::PCWSTR, password: &::windows::core::PCWSTR, loptionflags: i32, lclassflags: i32, linstanceflags: i32, pinfo: *mut WBEM_COMPILE_STATUS_INFO) -> ::windows::core::Result<()>;
    fn CompileBuffer(&self, buffsize: i32, pbuffer: *const u8, serverandnamespace: &::windows::core::PCWSTR, user: &::windows::core::PCWSTR, authority: &::windows::core::PCWSTR, password: &::windows::core::PCWSTR, loptionflags: i32, lclassflags: i32, linstanceflags: i32, pinfo: *mut WBEM_COMPILE_STATUS_INFO) -> ::windows::core::Result<()>;
    fn CreateBMOF(&self, textfilename: &::windows::core::PCWSTR, bmoffilename: &::windows::core::PCWSTR, serverandnamespace: &::windows::core::PCWSTR, loptionflags: i32, lclassflags: i32, linstanceflags: i32, pinfo: *mut WBEM_COMPILE_STATUS_INFO) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IMofCompiler {}
impl IMofCompiler_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMofCompiler_Impl, const OFFSET: isize>() -> IMofCompiler_Vtbl {
        unsafe extern "system" fn CompileFile<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMofCompiler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: ::windows::core::PCWSTR, serverandnamespace: ::windows::core::PCWSTR, user: ::windows::core::PCWSTR, authority: ::windows::core::PCWSTR, password: ::windows::core::PCWSTR, loptionflags: i32, lclassflags: i32, linstanceflags: i32, pinfo: *mut WBEM_COMPILE_STATUS_INFO) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CompileFile(::core::mem::transmute(&filename), ::core::mem::transmute(&serverandnamespace), ::core::mem::transmute(&user), ::core::mem::transmute(&authority), ::core::mem::transmute(&password), ::core::mem::transmute_copy(&loptionflags), ::core::mem::transmute_copy(&lclassflags), ::core::mem::transmute_copy(&linstanceflags), ::core::mem::transmute_copy(&pinfo)).into()
        }
        unsafe extern "system" fn CompileBuffer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMofCompiler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buffsize: i32, pbuffer: *const u8, serverandnamespace: ::windows::core::PCWSTR, user: ::windows::core::PCWSTR, authority: ::windows::core::PCWSTR, password: ::windows::core::PCWSTR, loptionflags: i32, lclassflags: i32, linstanceflags: i32, pinfo: *mut WBEM_COMPILE_STATUS_INFO) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CompileBuffer(::core::mem::transmute_copy(&buffsize), ::core::mem::transmute_copy(&pbuffer), ::core::mem::transmute(&serverandnamespace), ::core::mem::transmute(&user), ::core::mem::transmute(&authority), ::core::mem::transmute(&password), ::core::mem::transmute_copy(&loptionflags), ::core::mem::transmute_copy(&lclassflags), ::core::mem::transmute_copy(&linstanceflags), ::core::mem::transmute_copy(&pinfo)).into()
        }
        unsafe extern "system" fn CreateBMOF<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMofCompiler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, textfilename: ::windows::core::PCWSTR, bmoffilename: ::windows::core::PCWSTR, serverandnamespace: ::windows::core::PCWSTR, loptionflags: i32, lclassflags: i32, linstanceflags: i32, pinfo: *mut WBEM_COMPILE_STATUS_INFO) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateBMOF(::core::mem::transmute(&textfilename), ::core::mem::transmute(&bmoffilename), ::core::mem::transmute(&serverandnamespace), ::core::mem::transmute_copy(&loptionflags), ::core::mem::transmute_copy(&lclassflags), ::core::mem::transmute_copy(&linstanceflags), ::core::mem::transmute_copy(&pinfo)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            CompileFile: CompileFile::<Identity, Impl, OFFSET>,
            CompileBuffer: CompileBuffer::<Identity, Impl, OFFSET>,
            CreateBMOF: CreateBMOF::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMofCompiler as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISWbemDateTime_Impl: Sized + super::Com::IDispatch_Impl {
    fn Value(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetValue(&self, strvalue: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Year(&self) -> ::windows::core::Result<i32>;
    fn SetYear(&self, iyear: i32) -> ::windows::core::Result<()>;
    fn YearSpecified(&self) -> ::windows::core::Result<i16>;
    fn SetYearSpecified(&self, byearspecified: i16) -> ::windows::core::Result<()>;
    fn Month(&self) -> ::windows::core::Result<i32>;
    fn SetMonth(&self, imonth: i32) -> ::windows::core::Result<()>;
    fn MonthSpecified(&self) -> ::windows::core::Result<i16>;
    fn SetMonthSpecified(&self, bmonthspecified: i16) -> ::windows::core::Result<()>;
    fn Day(&self) -> ::windows::core::Result<i32>;
    fn SetDay(&self, iday: i32) -> ::windows::core::Result<()>;
    fn DaySpecified(&self) -> ::windows::core::Result<i16>;
    fn SetDaySpecified(&self, bdayspecified: i16) -> ::windows::core::Result<()>;
    fn Hours(&self) -> ::windows::core::Result<i32>;
    fn SetHours(&self, ihours: i32) -> ::windows::core::Result<()>;
    fn HoursSpecified(&self) -> ::windows::core::Result<i16>;
    fn SetHoursSpecified(&self, bhoursspecified: i16) -> ::windows::core::Result<()>;
    fn Minutes(&self) -> ::windows::core::Result<i32>;
    fn SetMinutes(&self, iminutes: i32) -> ::windows::core::Result<()>;
    fn MinutesSpecified(&self) -> ::windows::core::Result<i16>;
    fn SetMinutesSpecified(&self, bminutesspecified: i16) -> ::windows::core::Result<()>;
    fn Seconds(&self) -> ::windows::core::Result<i32>;
    fn SetSeconds(&self, iseconds: i32) -> ::windows::core::Result<()>;
    fn SecondsSpecified(&self) -> ::windows::core::Result<i16>;
    fn SetSecondsSpecified(&self, bsecondsspecified: i16) -> ::windows::core::Result<()>;
    fn Microseconds(&self) -> ::windows::core::Result<i32>;
    fn SetMicroseconds(&self, imicroseconds: i32) -> ::windows::core::Result<()>;
    fn MicrosecondsSpecified(&self) -> ::windows::core::Result<i16>;
    fn SetMicrosecondsSpecified(&self, bmicrosecondsspecified: i16) -> ::windows::core::Result<()>;
    fn UTC(&self) -> ::windows::core::Result<i32>;
    fn SetUTC(&self, iutc: i32) -> ::windows::core::Result<()>;
    fn UTCSpecified(&self) -> ::windows::core::Result<i16>;
    fn SetUTCSpecified(&self, butcspecified: i16) -> ::windows::core::Result<()>;
    fn IsInterval(&self) -> ::windows::core::Result<i16>;
    fn SetIsInterval(&self, bisinterval: i16) -> ::windows::core::Result<()>;
    fn GetVarDate(&self, bislocal: i16) -> ::windows::core::Result<f64>;
    fn SetVarDate(&self, dvardate: f64, bislocal: i16) -> ::windows::core::Result<()>;
    fn GetFileTime(&self, bislocal: i16) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetFileTime(&self, strfiletime: &super::super::Foundation::BSTR, bislocal: i16) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ISWbemDateTime {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISWbemDateTime_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemDateTime_Impl, const OFFSET: isize>() -> ISWbemDateTime_Vtbl {
        unsafe extern "system" fn Value<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strvalue: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Value() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(strvalue, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValue<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strvalue: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetValue(::core::mem::transmute(&strvalue)).into()
        }
        unsafe extern "system" fn Year<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iyear: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Year() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(iyear, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetYear<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iyear: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetYear(::core::mem::transmute_copy(&iyear)).into()
        }
        unsafe extern "system" fn YearSpecified<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, byearspecified: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.YearSpecified() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(byearspecified, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetYearSpecified<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, byearspecified: i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetYearSpecified(::core::mem::transmute_copy(&byearspecified)).into()
        }
        unsafe extern "system" fn Month<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, imonth: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Month() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(imonth, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMonth<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, imonth: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMonth(::core::mem::transmute_copy(&imonth)).into()
        }
        unsafe extern "system" fn MonthSpecified<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bmonthspecified: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MonthSpecified() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bmonthspecified, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMonthSpecified<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bmonthspecified: i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMonthSpecified(::core::mem::transmute_copy(&bmonthspecified)).into()
        }
        unsafe extern "system" fn Day<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iday: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Day() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(iday, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDay<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iday: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDay(::core::mem::transmute_copy(&iday)).into()
        }
        unsafe extern "system" fn DaySpecified<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bdayspecified: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DaySpecified() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bdayspecified, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDaySpecified<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bdayspecified: i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDaySpecified(::core::mem::transmute_copy(&bdayspecified)).into()
        }
        unsafe extern "system" fn Hours<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ihours: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Hours() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ihours, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHours<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ihours: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetHours(::core::mem::transmute_copy(&ihours)).into()
        }
        unsafe extern "system" fn HoursSpecified<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bhoursspecified: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.HoursSpecified() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bhoursspecified, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHoursSpecified<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bhoursspecified: i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetHoursSpecified(::core::mem::transmute_copy(&bhoursspecified)).into()
        }
        unsafe extern "system" fn Minutes<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iminutes: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Minutes() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(iminutes, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMinutes<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iminutes: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMinutes(::core::mem::transmute_copy(&iminutes)).into()
        }
        unsafe extern "system" fn MinutesSpecified<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bminutesspecified: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MinutesSpecified() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bminutesspecified, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMinutesSpecified<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bminutesspecified: i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMinutesSpecified(::core::mem::transmute_copy(&bminutesspecified)).into()
        }
        unsafe extern "system" fn Seconds<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iseconds: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Seconds() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(iseconds, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSeconds<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iseconds: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSeconds(::core::mem::transmute_copy(&iseconds)).into()
        }
        unsafe extern "system" fn SecondsSpecified<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bsecondsspecified: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SecondsSpecified() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bsecondsspecified, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSecondsSpecified<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bsecondsspecified: i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSecondsSpecified(::core::mem::transmute_copy(&bsecondsspecified)).into()
        }
        unsafe extern "system" fn Microseconds<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, imicroseconds: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Microseconds() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(imicroseconds, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMicroseconds<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, imicroseconds: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMicroseconds(::core::mem::transmute_copy(&imicroseconds)).into()
        }
        unsafe extern "system" fn MicrosecondsSpecified<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bmicrosecondsspecified: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MicrosecondsSpecified() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bmicrosecondsspecified, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMicrosecondsSpecified<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bmicrosecondsspecified: i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMicrosecondsSpecified(::core::mem::transmute_copy(&bmicrosecondsspecified)).into()
        }
        unsafe extern "system" fn UTC<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iutc: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.UTC() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(iutc, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUTC<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iutc: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetUTC(::core::mem::transmute_copy(&iutc)).into()
        }
        unsafe extern "system" fn UTCSpecified<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, butcspecified: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.UTCSpecified() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(butcspecified, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUTCSpecified<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, butcspecified: i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetUTCSpecified(::core::mem::transmute_copy(&butcspecified)).into()
        }
        unsafe extern "system" fn IsInterval<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bisinterval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsInterval() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bisinterval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsInterval<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bisinterval: i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetIsInterval(::core::mem::transmute_copy(&bisinterval)).into()
        }
        unsafe extern "system" fn GetVarDate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bislocal: i16, dvardate: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetVarDate(::core::mem::transmute_copy(&bislocal)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(dvardate, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVarDate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dvardate: f64, bislocal: i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetVarDate(::core::mem::transmute_copy(&dvardate), ::core::mem::transmute_copy(&bislocal)).into()
        }
        unsafe extern "system" fn GetFileTime<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bislocal: i16, strfiletime: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFileTime(::core::mem::transmute_copy(&bislocal)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(strfiletime, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFileTime<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemDateTime_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strfiletime: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bislocal: i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetFileTime(::core::mem::transmute(&strfiletime), ::core::mem::transmute_copy(&bislocal)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Value: Value::<Identity, Impl, OFFSET>,
            SetValue: SetValue::<Identity, Impl, OFFSET>,
            Year: Year::<Identity, Impl, OFFSET>,
            SetYear: SetYear::<Identity, Impl, OFFSET>,
            YearSpecified: YearSpecified::<Identity, Impl, OFFSET>,
            SetYearSpecified: SetYearSpecified::<Identity, Impl, OFFSET>,
            Month: Month::<Identity, Impl, OFFSET>,
            SetMonth: SetMonth::<Identity, Impl, OFFSET>,
            MonthSpecified: MonthSpecified::<Identity, Impl, OFFSET>,
            SetMonthSpecified: SetMonthSpecified::<Identity, Impl, OFFSET>,
            Day: Day::<Identity, Impl, OFFSET>,
            SetDay: SetDay::<Identity, Impl, OFFSET>,
            DaySpecified: DaySpecified::<Identity, Impl, OFFSET>,
            SetDaySpecified: SetDaySpecified::<Identity, Impl, OFFSET>,
            Hours: Hours::<Identity, Impl, OFFSET>,
            SetHours: SetHours::<Identity, Impl, OFFSET>,
            HoursSpecified: HoursSpecified::<Identity, Impl, OFFSET>,
            SetHoursSpecified: SetHoursSpecified::<Identity, Impl, OFFSET>,
            Minutes: Minutes::<Identity, Impl, OFFSET>,
            SetMinutes: SetMinutes::<Identity, Impl, OFFSET>,
            MinutesSpecified: MinutesSpecified::<Identity, Impl, OFFSET>,
            SetMinutesSpecified: SetMinutesSpecified::<Identity, Impl, OFFSET>,
            Seconds: Seconds::<Identity, Impl, OFFSET>,
            SetSeconds: SetSeconds::<Identity, Impl, OFFSET>,
            SecondsSpecified: SecondsSpecified::<Identity, Impl, OFFSET>,
            SetSecondsSpecified: SetSecondsSpecified::<Identity, Impl, OFFSET>,
            Microseconds: Microseconds::<Identity, Impl, OFFSET>,
            SetMicroseconds: SetMicroseconds::<Identity, Impl, OFFSET>,
            MicrosecondsSpecified: MicrosecondsSpecified::<Identity, Impl, OFFSET>,
            SetMicrosecondsSpecified: SetMicrosecondsSpecified::<Identity, Impl, OFFSET>,
            UTC: UTC::<Identity, Impl, OFFSET>,
            SetUTC: SetUTC::<Identity, Impl, OFFSET>,
            UTCSpecified: UTCSpecified::<Identity, Impl, OFFSET>,
            SetUTCSpecified: SetUTCSpecified::<Identity, Impl, OFFSET>,
            IsInterval: IsInterval::<Identity, Impl, OFFSET>,
            SetIsInterval: SetIsInterval::<Identity, Impl, OFFSET>,
            GetVarDate: GetVarDate::<Identity, Impl, OFFSET>,
            SetVarDate: SetVarDate::<Identity, Impl, OFFSET>,
            GetFileTime: GetFileTime::<Identity, Impl, OFFSET>,
            SetFileTime: SetFileTime::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISWbemDateTime as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISWbemEventSource_Impl: Sized + super::Com::IDispatch_Impl {
    fn NextEvent(&self, itimeoutms: i32) -> ::windows::core::Result<ISWbemObject>;
    fn Security_(&self) -> ::windows::core::Result<ISWbemSecurity>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ISWbemEventSource {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISWbemEventSource_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemEventSource_Impl, const OFFSET: isize>() -> ISWbemEventSource_Vtbl {
        unsafe extern "system" fn NextEvent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemEventSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, itimeoutms: i32, objwbemobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.NextEvent(::core::mem::transmute_copy(&itimeoutms)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(objwbemobject, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Security_<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemEventSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objwbemsecurity: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Security_() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(objwbemsecurity, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            NextEvent: NextEvent::<Identity, Impl, OFFSET>,
            Security_: Security_::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISWbemEventSource as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISWbemLastError_Impl: Sized + super::Com::IDispatch_Impl + ISWbemObject_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ISWbemLastError {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISWbemLastError_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemLastError_Impl, const OFFSET: isize>() -> ISWbemLastError_Vtbl {
        Self { base__: ISWbemObject_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISWbemLastError as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<ISWbemObject as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISWbemLocator_Impl: Sized + super::Com::IDispatch_Impl {
    fn ConnectServer(&self, strserver: &super::super::Foundation::BSTR, strnamespace: &super::super::Foundation::BSTR, struser: &super::super::Foundation::BSTR, strpassword: &super::super::Foundation::BSTR, strlocale: &super::super::Foundation::BSTR, strauthority: &super::super::Foundation::BSTR, isecurityflags: i32, objwbemnamedvalueset: &::core::option::Option<super::Com::IDispatch>) -> ::windows::core::Result<ISWbemServices>;
    fn Security_(&self) -> ::windows::core::Result<ISWbemSecurity>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ISWbemLocator {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISWbemLocator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemLocator_Impl, const OFFSET: isize>() -> ISWbemLocator_Vtbl {
        unsafe extern "system" fn ConnectServer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemLocator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strserver: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strnamespace: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, struser: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strlocale: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strauthority: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, isecurityflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void, objwbemservices: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ConnectServer(::core::mem::transmute(&strserver), ::core::mem::transmute(&strnamespace), ::core::mem::transmute(&struser), ::core::mem::transmute(&strpassword), ::core::mem::transmute(&strlocale), ::core::mem::transmute(&strauthority), ::core::mem::transmute_copy(&isecurityflags), ::core::mem::transmute(&objwbemnamedvalueset)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(objwbemservices, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Security_<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemLocator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objwbemsecurity: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Security_() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(objwbemsecurity, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            ConnectServer: ConnectServer::<Identity, Impl, OFFSET>,
            Security_: Security_::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISWbemLocator as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISWbemMethod_Impl: Sized + super::Com::IDispatch_Impl {
    fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Origin(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn InParameters(&self) -> ::windows::core::Result<ISWbemObject>;
    fn OutParameters(&self) -> ::windows::core::Result<ISWbemObject>;
    fn Qualifiers_(&self) -> ::windows::core::Result<ISWbemQualifierSet>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ISWbemMethod {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISWbemMethod_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemMethod_Impl, const OFFSET: isize>() -> ISWbemMethod_Vtbl {
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemMethod_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Name() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(strname, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Origin<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemMethod_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strorigin: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Origin() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(strorigin, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InParameters<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemMethod_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objwbeminparameters: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.InParameters() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(objwbeminparameters, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OutParameters<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemMethod_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objwbemoutparameters: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.OutParameters() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(objwbemoutparameters, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Qualifiers_<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemMethod_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objwbemqualifierset: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Qualifiers_() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(objwbemqualifierset, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Name: Name::<Identity, Impl, OFFSET>,
            Origin: Origin::<Identity, Impl, OFFSET>,
            InParameters: InParameters::<Identity, Impl, OFFSET>,
            OutParameters: OutParameters::<Identity, Impl, OFFSET>,
            Qualifiers_: Qualifiers_::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISWbemMethod as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISWbemMethodSet_Impl: Sized + super::Com::IDispatch_Impl {
    fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Item(&self, strname: &super::super::Foundation::BSTR, iflags: i32) -> ::windows::core::Result<ISWbemMethod>;
    fn Count(&self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ISWbemMethodSet {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISWbemMethodSet_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemMethodSet_Impl, const OFFSET: isize>() -> ISWbemMethodSet_Vtbl {
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemMethodSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(punk, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemMethodSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemmethod: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Item(::core::mem::transmute(&strname), ::core::mem::transmute_copy(&iflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(objwbemmethod, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemMethodSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, icount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(icount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISWbemMethodSet as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISWbemNamedValue_Impl: Sized + super::Com::IDispatch_Impl {
    fn Value(&self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SetValue(&self, varvalue: *const super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ISWbemNamedValue {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISWbemNamedValue_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemNamedValue_Impl, const OFFSET: isize>() -> ISWbemNamedValue_Vtbl {
        unsafe extern "system" fn Value<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemNamedValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varvalue: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Value() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(varvalue, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValue<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemNamedValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varvalue: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetValue(::core::mem::transmute_copy(&varvalue)).into()
        }
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemNamedValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Name() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(strname, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Value: Value::<Identity, Impl, OFFSET>,
            SetValue: SetValue::<Identity, Impl, OFFSET>,
            Name: Name::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISWbemNamedValue as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISWbemNamedValueSet_Impl: Sized + super::Com::IDispatch_Impl {
    fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Item(&self, strname: &super::super::Foundation::BSTR, iflags: i32) -> ::windows::core::Result<ISWbemNamedValue>;
    fn Count(&self) -> ::windows::core::Result<i32>;
    fn Add(&self, strname: &super::super::Foundation::BSTR, varvalue: *const super::Com::VARIANT, iflags: i32) -> ::windows::core::Result<ISWbemNamedValue>;
    fn Remove(&self, strname: &super::super::Foundation::BSTR, iflags: i32) -> ::windows::core::Result<()>;
    fn Clone(&self) -> ::windows::core::Result<ISWbemNamedValueSet>;
    fn DeleteAll(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ISWbemNamedValueSet {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISWbemNamedValueSet_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemNamedValueSet_Impl, const OFFSET: isize>() -> ISWbemNamedValueSet_Vtbl {
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemNamedValueSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(punk, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemNamedValueSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemnamedvalue: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Item(::core::mem::transmute(&strname), ::core::mem::transmute_copy(&iflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(objwbemnamedvalue, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemNamedValueSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, icount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(icount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemNamedValueSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varvalue: *const super::Com::VARIANT, iflags: i32, objwbemnamedvalue: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Add(::core::mem::transmute(&strname), ::core::mem::transmute_copy(&varvalue), ::core::mem::transmute_copy(&iflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(objwbemnamedvalue, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Remove<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemNamedValueSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Remove(::core::mem::transmute(&strname), ::core::mem::transmute_copy(&iflags)).into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemNamedValueSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objwbemnamedvalueset: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(objwbemnamedvalueset, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteAll<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemNamedValueSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteAll().into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            Add: Add::<Identity, Impl, OFFSET>,
            Remove: Remove::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
            DeleteAll: DeleteAll::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISWbemNamedValueSet as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISWbemObject_Impl: Sized + super::Com::IDispatch_Impl {
    fn Put_(&self, iflags: i32, objwbemnamedvalueset: &::core::option::Option<super::Com::IDispatch>) -> ::windows::core::Result<ISWbemObjectPath>;
    fn PutAsync_(&self, objwbemsink: &::core::option::Option<super::Com::IDispatch>, iflags: i32, objwbemnamedvalueset: &::core::option::Option<super::Com::IDispatch>, objwbemasynccontext: &::core::option::Option<super::Com::IDispatch>) -> ::windows::core::Result<()>;
    fn Delete_(&self, iflags: i32, objwbemnamedvalueset: &::core::option::Option<super::Com::IDispatch>) -> ::windows::core::Result<()>;
    fn DeleteAsync_(&self, objwbemsink: &::core::option::Option<super::Com::IDispatch>, iflags: i32, objwbemnamedvalueset: &::core::option::Option<super::Com::IDispatch>, objwbemasynccontext: &::core::option::Option<super::Com::IDispatch>) -> ::windows::core::Result<()>;
    fn Instances_(&self, iflags: i32, objwbemnamedvalueset: &::core::option::Option<super::Com::IDispatch>) -> ::windows::core::Result<ISWbemObjectSet>;
    fn InstancesAsync_(&self, objwbemsink: &::core::option::Option<super::Com::IDispatch>, iflags: i32, objwbemnamedvalueset: &::core::option::Option<super::Com::IDispatch>, objwbemasynccontext: &::core::option::Option<super::Com::IDispatch>) -> ::windows::core::Result<()>;
    fn Subclasses_(&self, iflags: i32, objwbemnamedvalueset: &::core::option::Option<super::Com::IDispatch>) -> ::windows::core::Result<ISWbemObjectSet>;
    fn SubclassesAsync_(&self, objwbemsink: &::core::option::Option<super::Com::IDispatch>, iflags: i32, objwbemnamedvalueset: &::core::option::Option<super::Com::IDispatch>, objwbemasynccontext: &::core::option::Option<super::Com::IDispatch>) -> ::windows::core::Result<()>;
    fn Associators_(&self, strassocclass: &super::super::Foundation::BSTR, strresultclass: &super::super::Foundation::BSTR, strresultrole: &super::super::Foundation::BSTR, strrole: &super::super::Foundation::BSTR, bclassesonly: i16, bschemaonly: i16, strrequiredassocqualifier: &super::super::Foundation::BSTR, strrequiredqualifier: &super::super::Foundation::BSTR, iflags: i32, objwbemnamedvalueset: &::core::option::Option<super::Com::IDispatch>) -> ::windows::core::Result<ISWbemObjectSet>;
    fn AssociatorsAsync_(&self, objwbemsink: &::core::option::Option<super::Com::IDispatch>, strassocclass: &super::super::Foundation::BSTR, strresultclass: &super::super::Foundation::BSTR, strresultrole: &super::super::Foundation::BSTR, strrole: &super::super::Foundation::BSTR, bclassesonly: i16, bschemaonly: i16, strrequiredassocqualifier: &super::super::Foundation::BSTR, strrequiredqualifier: &super::super::Foundation::BSTR, iflags: i32, objwbemnamedvalueset: &::core::option::Option<super::Com::IDispatch>, objwbemasynccontext: &::core::option::Option<super::Com::IDispatch>) -> ::windows::core::Result<()>;
    fn References_(&self, strresultclass: &super::super::Foundation::BSTR, strrole: &super::super::Foundation::BSTR, bclassesonly: i16, bschemaonly: i16, strrequiredqualifier: &super::super::Foundation::BSTR, iflags: i32, objwbemnamedvalueset: &::core::option::Option<super::Com::IDispatch>) -> ::windows::core::Result<ISWbemObjectSet>;
    fn ReferencesAsync_(&self, objwbemsink: &::core::option::Option<super::Com::IDispatch>, strresultclass: &super::super::Foundation::BSTR, strrole: &super::super::Foundation::BSTR, bclassesonly: i16, bschemaonly: i16, strrequiredqualifier: &super::super::Foundation::BSTR, iflags: i32, objwbemnamedvalueset: &::core::option::Option<super::Com::IDispatch>, objwbemasynccontext: &::core::option::Option<super::Com::IDispatch>) -> ::windows::core::Result<()>;
    fn ExecMethod_(&self, strmethodname: &super::super::Foundation::BSTR, objwbeminparameters: &::core::option::Option<super::Com::IDispatch>, iflags: i32, objwbemnamedvalueset: &::core::option::Option<super::Com::IDispatch>) -> ::windows::core::Result<ISWbemObject>;
    fn ExecMethodAsync_(&self, objwbemsink: &::core::option::Option<super::Com::IDispatch>, strmethodname: &super::super::Foundation::BSTR, objwbeminparameters: &::core::option::Option<super::Com::IDispatch>, iflags: i32, objwbemnamedvalueset: &::core::option::Option<super::Com::IDispatch>, objwbemasynccontext: &::core::option::Option<super::Com::IDispatch>) -> ::windows::core::Result<()>;
    fn Clone_(&self) -> ::windows::core::Result<ISWbemObject>;
    fn GetObjectText_(&self, iflags: i32) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SpawnDerivedClass_(&self, iflags: i32) -> ::windows::core::Result<ISWbemObject>;
    fn SpawnInstance_(&self, iflags: i32) -> ::windows::core::Result<ISWbemObject>;
    fn CompareTo_(&self, objwbemobject: &::core::option::Option<super::Com::IDispatch>, iflags: i32) -> ::windows::core::Result<i16>;
    fn Qualifiers_(&self) -> ::windows::core::Result<ISWbemQualifierSet>;
    fn Properties_(&self) -> ::windows::core::Result<ISWbemPropertySet>;
    fn Methods_(&self) -> ::windows::core::Result<ISWbemMethodSet>;
    fn Derivation_(&self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn Path_(&self) -> ::windows::core::Result<ISWbemObjectPath>;
    fn Security_(&self) -> ::windows::core::Result<ISWbemSecurity>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ISWbemObject {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISWbemObject_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemObject_Impl, const OFFSET: isize>() -> ISWbemObject_Vtbl {
        unsafe extern "system" fn Put_<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void, objwbemobjectpath: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Put_(::core::mem::transmute_copy(&iflags), ::core::mem::transmute(&objwbemnamedvalueset)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(objwbemobjectpath, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PutAsync_<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objwbemsink: *mut ::core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void, objwbemasynccontext: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PutAsync_(::core::mem::transmute(&objwbemsink), ::core::mem::transmute_copy(&iflags), ::core::mem::transmute(&objwbemnamedvalueset), ::core::mem::transmute(&objwbemasynccontext)).into()
        }
        unsafe extern "system" fn Delete_<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Delete_(::core::mem::transmute_copy(&iflags), ::core::mem::transmute(&objwbemnamedvalueset)).into()
        }
        unsafe extern "system" fn DeleteAsync_<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objwbemsink: *mut ::core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void, objwbemasynccontext: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteAsync_(::core::mem::transmute(&objwbemsink), ::core::mem::transmute_copy(&iflags), ::core::mem::transmute(&objwbemnamedvalueset), ::core::mem::transmute(&objwbemasynccontext)).into()
        }
        unsafe extern "system" fn Instances_<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void, objwbemobjectset: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Instances_(::core::mem::transmute_copy(&iflags), ::core::mem::transmute(&objwbemnamedvalueset)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(objwbemobjectset, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InstancesAsync_<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objwbemsink: *mut ::core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void, objwbemasynccontext: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InstancesAsync_(::core::mem::transmute(&objwbemsink), ::core::mem::transmute_copy(&iflags), ::core::mem::transmute(&objwbemnamedvalueset), ::core::mem::transmute(&objwbemasynccontext)).into()
        }
        unsafe extern "system" fn Subclasses_<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void, objwbemobjectset: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Subclasses_(::core::mem::transmute_copy(&iflags), ::core::mem::transmute(&objwbemnamedvalueset)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(objwbemobjectset, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SubclassesAsync_<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objwbemsink: *mut ::core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void, objwbemasynccontext: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SubclassesAsync_(::core::mem::transmute(&objwbemsink), ::core::mem::transmute_copy(&iflags), ::core::mem::transmute(&objwbemnamedvalueset), ::core::mem::transmute(&objwbemasynccontext)).into()
        }
        unsafe extern "system" fn Associators_<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemObject_Impl, const OFFSET: isize>(
            this: *mut ::core::ffi::c_void,
            strassocclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
            strresultclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
            strresultrole: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
            strrole: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
            bclassesonly: i16,
            bschemaonly: i16,
            strrequiredassocqualifier: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
            strrequiredqualifier: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
            iflags: i32,
            objwbemnamedvalueset: *mut ::core::ffi::c_void,
            objwbemobjectset: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Associators_(::core::mem::transmute(&strassocclass), ::core::mem::transmute(&strresultclass), ::core::mem::transmute(&strresultrole), ::core::mem::transmute(&strrole), ::core::mem::transmute_copy(&bclassesonly), ::core::mem::transmute_copy(&bschemaonly), ::core::mem::transmute(&strrequiredassocqualifier), ::core::mem::transmute(&strrequiredqualifier), ::core::mem::transmute_copy(&iflags), ::core::mem::transmute(&objwbemnamedvalueset)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(objwbemobjectset, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AssociatorsAsync_<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemObject_Impl, const OFFSET: isize>(
            this: *mut ::core::ffi::c_void,
            objwbemsink: *mut ::core::ffi::c_void,
            strassocclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
            strresultclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
            strresultrole: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
            strrole: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
            bclassesonly: i16,
            bschemaonly: i16,
            strrequiredassocqualifier: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
            strrequiredqualifier: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
            iflags: i32,
            objwbemnamedvalueset: *mut ::core::ffi::c_void,
            objwbemasynccontext: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AssociatorsAsync_(
                ::core::mem::transmute(&objwbemsink),
                ::core::mem::transmute(&strassocclass),
                ::core::mem::transmute(&strresultclass),
                ::core::mem::transmute(&strresultrole),
                ::core::mem::transmute(&strrole),
                ::core::mem::transmute_copy(&bclassesonly),
                ::core::mem::transmute_copy(&bschemaonly),
                ::core::mem::transmute(&strrequiredassocqualifier),
                ::core::mem::transmute(&strrequiredqualifier),
                ::core::mem::transmute_copy(&iflags),
                ::core::mem::transmute(&objwbemnamedvalueset),
                ::core::mem::transmute(&objwbemasynccontext),
            )
            .into()
        }
        unsafe extern "system" fn References_<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strresultclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strrole: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bclassesonly: i16, bschemaonly: i16, strrequiredqualifier: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void, objwbemobjectset: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.References_(::core::mem::transmute(&strresultclass), ::core::mem::transmute(&strrole), ::core::mem::transmute_copy(&bclassesonly), ::core::mem::transmute_copy(&bschemaonly), ::core::mem::transmute(&strrequiredqualifier), ::core::mem::transmute_copy(&iflags), ::core::mem::transmute(&objwbemnamedvalueset)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(objwbemobjectset, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReferencesAsync_<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objwbemsink: *mut ::core::ffi::c_void, strresultclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strrole: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bclassesonly: i16, bschemaonly: i16, strrequiredqualifier: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void, objwbemasynccontext: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReferencesAsync_(::core::mem::transmute(&objwbemsink), ::core::mem::transmute(&strresultclass), ::core::mem::transmute(&strrole), ::core::mem::transmute_copy(&bclassesonly), ::core::mem::transmute_copy(&bschemaonly), ::core::mem::transmute(&strrequiredqualifier), ::core::mem::transmute_copy(&iflags), ::core::mem::transmute(&objwbemnamedvalueset), ::core::mem::transmute(&objwbemasynccontext)).into()
        }
        unsafe extern "system" fn ExecMethod_<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strmethodname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, objwbeminparameters: *mut ::core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void, objwbemoutparameters: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ExecMethod_(::core::mem::transmute(&strmethodname), ::core::mem::transmute(&objwbeminparameters), ::core::mem::transmute_copy(&iflags), ::core::mem::transmute(&objwbemnamedvalueset)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(objwbemoutparameters, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExecMethodAsync_<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objwbemsink: *mut ::core::ffi::c_void, strmethodname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, objwbeminparameters: *mut ::core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void, objwbemasynccontext: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ExecMethodAsync_(::core::mem::transmute(&objwbemsink), ::core::mem::transmute(&strmethodname), ::core::mem::transmute(&objwbeminparameters), ::core::mem::transmute_copy(&iflags), ::core::mem::transmute(&objwbemnamedvalueset), ::core::mem::transmute(&objwbemasynccontext)).into()
        }
        unsafe extern "system" fn Clone_<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objwbemobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone_() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(objwbemobject, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetObjectText_<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iflags: i32, strobjecttext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetObjectText_(::core::mem::transmute_copy(&iflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(strobjecttext, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SpawnDerivedClass_<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iflags: i32, objwbemobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SpawnDerivedClass_(::core::mem::transmute_copy(&iflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(objwbemobject, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SpawnInstance_<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iflags: i32, objwbemobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SpawnInstance_(::core::mem::transmute_copy(&iflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(objwbemobject, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CompareTo_<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objwbemobject: *mut ::core::ffi::c_void, iflags: i32, bresult: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CompareTo_(::core::mem::transmute(&objwbemobject), ::core::mem::transmute_copy(&iflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bresult, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Qualifiers_<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objwbemqualifierset: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Qualifiers_() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(objwbemqualifierset, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties_<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objwbempropertyset: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Properties_() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(objwbempropertyset, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Methods_<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objwbemmethodset: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Methods_() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(objwbemmethodset, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Derivation_<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strclassnamearray: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Derivation_() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(strclassnamearray, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Path_<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objwbemobjectpath: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Path_() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(objwbemobjectpath, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Security_<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objwbemsecurity: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Security_() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(objwbemsecurity, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Put_: Put_::<Identity, Impl, OFFSET>,
            PutAsync_: PutAsync_::<Identity, Impl, OFFSET>,
            Delete_: Delete_::<Identity, Impl, OFFSET>,
            DeleteAsync_: DeleteAsync_::<Identity, Impl, OFFSET>,
            Instances_: Instances_::<Identity, Impl, OFFSET>,
            InstancesAsync_: InstancesAsync_::<Identity, Impl, OFFSET>,
            Subclasses_: Subclasses_::<Identity, Impl, OFFSET>,
            SubclassesAsync_: SubclassesAsync_::<Identity, Impl, OFFSET>,
            Associators_: Associators_::<Identity, Impl, OFFSET>,
            AssociatorsAsync_: AssociatorsAsync_::<Identity, Impl, OFFSET>,
            References_: References_::<Identity, Impl, OFFSET>,
            ReferencesAsync_: ReferencesAsync_::<Identity, Impl, OFFSET>,
            ExecMethod_: ExecMethod_::<Identity, Impl, OFFSET>,
            ExecMethodAsync_: ExecMethodAsync_::<Identity, Impl, OFFSET>,
            Clone_: Clone_::<Identity, Impl, OFFSET>,
            GetObjectText_: GetObjectText_::<Identity, Impl, OFFSET>,
            SpawnDerivedClass_: SpawnDerivedClass_::<Identity, Impl, OFFSET>,
            SpawnInstance_: SpawnInstance_::<Identity, Impl, OFFSET>,
            CompareTo_: CompareTo_::<Identity, Impl, OFFSET>,
            Qualifiers_: Qualifiers_::<Identity, Impl, OFFSET>,
            Properties_: Properties_::<Identity, Impl, OFFSET>,
            Methods_: Methods_::<Identity, Impl, OFFSET>,
            Derivation_: Derivation_::<Identity, Impl, OFFSET>,
            Path_: Path_::<Identity, Impl, OFFSET>,
            Security_: Security_::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISWbemObject as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISWbemObjectEx_Impl: Sized + super::Com::IDispatch_Impl + ISWbemObject_Impl {
    fn Refresh_(&self, iflags: i32, objwbemnamedvalueset: &::core::option::Option<super::Com::IDispatch>) -> ::windows::core::Result<()>;
    fn SystemProperties_(&self) -> ::windows::core::Result<ISWbemPropertySet>;
    fn GetText_(&self, iobjecttextformat: WbemObjectTextFormatEnum, iflags: i32, objwbemnamedvalueset: &::core::option::Option<super::Com::IDispatch>) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetFromText_(&self, bstext: &super::super::Foundation::BSTR, iobjecttextformat: WbemObjectTextFormatEnum, iflags: i32, objwbemnamedvalueset: &::core::option::Option<super::Com::IDispatch>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ISWbemObjectEx {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISWbemObjectEx_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemObjectEx_Impl, const OFFSET: isize>() -> ISWbemObjectEx_Vtbl {
        unsafe extern "system" fn Refresh_<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemObjectEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Refresh_(::core::mem::transmute_copy(&iflags), ::core::mem::transmute(&objwbemnamedvalueset)).into()
        }
        unsafe extern "system" fn SystemProperties_<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemObjectEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objwbempropertyset: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SystemProperties_() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(objwbempropertyset, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetText_<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemObjectEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iobjecttextformat: WbemObjectTextFormatEnum, iflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void, bstext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetText_(::core::mem::transmute_copy(&iobjecttextformat), ::core::mem::transmute_copy(&iflags), ::core::mem::transmute(&objwbemnamedvalueset)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bstext, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFromText_<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemObjectEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstext: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iobjecttextformat: WbemObjectTextFormatEnum, iflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetFromText_(::core::mem::transmute(&bstext), ::core::mem::transmute_copy(&iobjecttextformat), ::core::mem::transmute_copy(&iflags), ::core::mem::transmute(&objwbemnamedvalueset)).into()
        }
        Self {
            base__: ISWbemObject_Vtbl::new::<Identity, Impl, OFFSET>(),
            Refresh_: Refresh_::<Identity, Impl, OFFSET>,
            SystemProperties_: SystemProperties_::<Identity, Impl, OFFSET>,
            GetText_: GetText_::<Identity, Impl, OFFSET>,
            SetFromText_: SetFromText_::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISWbemObjectEx as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<ISWbemObject as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISWbemObjectPath_Impl: Sized + super::Com::IDispatch_Impl {
    fn Path(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetPath(&self, strpath: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn RelPath(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetRelPath(&self, strrelpath: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Server(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetServer(&self, strserver: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Namespace(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetNamespace(&self, strnamespace: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ParentNamespace(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn DisplayName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetDisplayName(&self, strdisplayname: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Class(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetClass(&self, strclass: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn IsClass(&self) -> ::windows::core::Result<i16>;
    fn SetAsClass(&self) -> ::windows::core::Result<()>;
    fn IsSingleton(&self) -> ::windows::core::Result<i16>;
    fn SetAsSingleton(&self) -> ::windows::core::Result<()>;
    fn Keys(&self) -> ::windows::core::Result<ISWbemNamedValueSet>;
    fn Security_(&self) -> ::windows::core::Result<ISWbemSecurity>;
    fn Locale(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetLocale(&self, strlocale: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Authority(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetAuthority(&self, strauthority: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ISWbemObjectPath {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISWbemObjectPath_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemObjectPath_Impl, const OFFSET: isize>() -> ISWbemObjectPath_Vtbl {
        unsafe extern "system" fn Path<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemObjectPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strpath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Path() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(strpath, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPath<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemObjectPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPath(::core::mem::transmute(&strpath)).into()
        }
        unsafe extern "system" fn RelPath<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemObjectPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strrelpath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RelPath() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(strrelpath, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRelPath<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemObjectPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strrelpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetRelPath(::core::mem::transmute(&strrelpath)).into()
        }
        unsafe extern "system" fn Server<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemObjectPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strserver: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Server() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(strserver, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetServer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemObjectPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strserver: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetServer(::core::mem::transmute(&strserver)).into()
        }
        unsafe extern "system" fn Namespace<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemObjectPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strnamespace: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Namespace() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(strnamespace, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNamespace<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemObjectPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strnamespace: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetNamespace(::core::mem::transmute(&strnamespace)).into()
        }
        unsafe extern "system" fn ParentNamespace<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemObjectPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strparentnamespace: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ParentNamespace() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(strparentnamespace, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplayName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemObjectPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strdisplayname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(strdisplayname, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisplayName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemObjectPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strdisplayname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDisplayName(::core::mem::transmute(&strdisplayname)).into()
        }
        unsafe extern "system" fn Class<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemObjectPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strclass: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Class() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(strclass, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClass<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemObjectPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetClass(::core::mem::transmute(&strclass)).into()
        }
        unsafe extern "system" fn IsClass<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemObjectPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bisclass: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsClass() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bisclass, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAsClass<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemObjectPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAsClass().into()
        }
        unsafe extern "system" fn IsSingleton<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemObjectPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bissingleton: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsSingleton() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bissingleton, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAsSingleton<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemObjectPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAsSingleton().into()
        }
        unsafe extern "system" fn Keys<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemObjectPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objwbemnamedvalueset: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Keys() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(objwbemnamedvalueset, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Security_<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemObjectPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objwbemsecurity: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Security_() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(objwbemsecurity, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Locale<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemObjectPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strlocale: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Locale() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(strlocale, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLocale<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemObjectPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strlocale: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetLocale(::core::mem::transmute(&strlocale)).into()
        }
        unsafe extern "system" fn Authority<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemObjectPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strauthority: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Authority() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(strauthority, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAuthority<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemObjectPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strauthority: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAuthority(::core::mem::transmute(&strauthority)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Path: Path::<Identity, Impl, OFFSET>,
            SetPath: SetPath::<Identity, Impl, OFFSET>,
            RelPath: RelPath::<Identity, Impl, OFFSET>,
            SetRelPath: SetRelPath::<Identity, Impl, OFFSET>,
            Server: Server::<Identity, Impl, OFFSET>,
            SetServer: SetServer::<Identity, Impl, OFFSET>,
            Namespace: Namespace::<Identity, Impl, OFFSET>,
            SetNamespace: SetNamespace::<Identity, Impl, OFFSET>,
            ParentNamespace: ParentNamespace::<Identity, Impl, OFFSET>,
            DisplayName: DisplayName::<Identity, Impl, OFFSET>,
            SetDisplayName: SetDisplayName::<Identity, Impl, OFFSET>,
            Class: Class::<Identity, Impl, OFFSET>,
            SetClass: SetClass::<Identity, Impl, OFFSET>,
            IsClass: IsClass::<Identity, Impl, OFFSET>,
            SetAsClass: SetAsClass::<Identity, Impl, OFFSET>,
            IsSingleton: IsSingleton::<Identity, Impl, OFFSET>,
            SetAsSingleton: SetAsSingleton::<Identity, Impl, OFFSET>,
            Keys: Keys::<Identity, Impl, OFFSET>,
            Security_: Security_::<Identity, Impl, OFFSET>,
            Locale: Locale::<Identity, Impl, OFFSET>,
            SetLocale: SetLocale::<Identity, Impl, OFFSET>,
            Authority: Authority::<Identity, Impl, OFFSET>,
            SetAuthority: SetAuthority::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISWbemObjectPath as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISWbemObjectSet_Impl: Sized + super::Com::IDispatch_Impl {
    fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Item(&self, strobjectpath: &super::super::Foundation::BSTR, iflags: i32) -> ::windows::core::Result<ISWbemObject>;
    fn Count(&self) -> ::windows::core::Result<i32>;
    fn Security_(&self) -> ::windows::core::Result<ISWbemSecurity>;
    fn ItemIndex(&self, lindex: i32) -> ::windows::core::Result<ISWbemObject>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ISWbemObjectSet {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISWbemObjectSet_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemObjectSet_Impl, const OFFSET: isize>() -> ISWbemObjectSet_Vtbl {
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemObjectSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(punk, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemObjectSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strobjectpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Item(::core::mem::transmute(&strobjectpath), ::core::mem::transmute_copy(&iflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(objwbemobject, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemObjectSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, icount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(icount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Security_<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemObjectSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objwbemsecurity: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Security_() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(objwbemsecurity, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ItemIndex<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemObjectSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32, objwbemobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ItemIndex(::core::mem::transmute_copy(&lindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(objwbemobject, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            Security_: Security_::<Identity, Impl, OFFSET>,
            ItemIndex: ItemIndex::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISWbemObjectSet as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISWbemPrivilege_Impl: Sized + super::Com::IDispatch_Impl {
    fn IsEnabled(&self) -> ::windows::core::Result<i16>;
    fn SetIsEnabled(&self, bisenabled: i16) -> ::windows::core::Result<()>;
    fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn DisplayName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Identifier(&self) -> ::windows::core::Result<WbemPrivilegeEnum>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ISWbemPrivilege {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISWbemPrivilege_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemPrivilege_Impl, const OFFSET: isize>() -> ISWbemPrivilege_Vtbl {
        unsafe extern "system" fn IsEnabled<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemPrivilege_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bisenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bisenabled, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsEnabled<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemPrivilege_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bisenabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetIsEnabled(::core::mem::transmute_copy(&bisenabled)).into()
        }
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemPrivilege_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strdisplayname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Name() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(strdisplayname, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplayName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemPrivilege_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strdisplayname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(strdisplayname, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Identifier<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemPrivilege_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iprivilege: *mut WbemPrivilegeEnum) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Identifier() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(iprivilege, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            IsEnabled: IsEnabled::<Identity, Impl, OFFSET>,
            SetIsEnabled: SetIsEnabled::<Identity, Impl, OFFSET>,
            Name: Name::<Identity, Impl, OFFSET>,
            DisplayName: DisplayName::<Identity, Impl, OFFSET>,
            Identifier: Identifier::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISWbemPrivilege as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISWbemPrivilegeSet_Impl: Sized + super::Com::IDispatch_Impl {
    fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Item(&self, iprivilege: WbemPrivilegeEnum) -> ::windows::core::Result<ISWbemPrivilege>;
    fn Count(&self) -> ::windows::core::Result<i32>;
    fn Add(&self, iprivilege: WbemPrivilegeEnum, bisenabled: i16) -> ::windows::core::Result<ISWbemPrivilege>;
    fn Remove(&self, iprivilege: WbemPrivilegeEnum) -> ::windows::core::Result<()>;
    fn DeleteAll(&self) -> ::windows::core::Result<()>;
    fn AddAsString(&self, strprivilege: &super::super::Foundation::BSTR, bisenabled: i16) -> ::windows::core::Result<ISWbemPrivilege>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ISWbemPrivilegeSet {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISWbemPrivilegeSet_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemPrivilegeSet_Impl, const OFFSET: isize>() -> ISWbemPrivilegeSet_Vtbl {
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemPrivilegeSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(punk, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemPrivilegeSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iprivilege: WbemPrivilegeEnum, objwbemprivilege: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Item(::core::mem::transmute_copy(&iprivilege)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(objwbemprivilege, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemPrivilegeSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, icount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(icount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemPrivilegeSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iprivilege: WbemPrivilegeEnum, bisenabled: i16, objwbemprivilege: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Add(::core::mem::transmute_copy(&iprivilege), ::core::mem::transmute_copy(&bisenabled)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(objwbemprivilege, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Remove<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemPrivilegeSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iprivilege: WbemPrivilegeEnum) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Remove(::core::mem::transmute_copy(&iprivilege)).into()
        }
        unsafe extern "system" fn DeleteAll<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemPrivilegeSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteAll().into()
        }
        unsafe extern "system" fn AddAsString<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemPrivilegeSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strprivilege: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bisenabled: i16, objwbemprivilege: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AddAsString(::core::mem::transmute(&strprivilege), ::core::mem::transmute_copy(&bisenabled)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(objwbemprivilege, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            Add: Add::<Identity, Impl, OFFSET>,
            Remove: Remove::<Identity, Impl, OFFSET>,
            DeleteAll: DeleteAll::<Identity, Impl, OFFSET>,
            AddAsString: AddAsString::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISWbemPrivilegeSet as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISWbemProperty_Impl: Sized + super::Com::IDispatch_Impl {
    fn Value(&self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SetValue(&self, varvalue: *const super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn IsLocal(&self) -> ::windows::core::Result<i16>;
    fn Origin(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn CIMType(&self) -> ::windows::core::Result<WbemCimtypeEnum>;
    fn Qualifiers_(&self) -> ::windows::core::Result<ISWbemQualifierSet>;
    fn IsArray(&self) -> ::windows::core::Result<i16>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ISWbemProperty {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISWbemProperty_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemProperty_Impl, const OFFSET: isize>() -> ISWbemProperty_Vtbl {
        unsafe extern "system" fn Value<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varvalue: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Value() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(varvalue, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValue<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varvalue: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetValue(::core::mem::transmute_copy(&varvalue)).into()
        }
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Name() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(strname, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsLocal<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bislocal: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsLocal() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bislocal, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Origin<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strorigin: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Origin() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(strorigin, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CIMType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, icimtype: *mut WbemCimtypeEnum) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CIMType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(icimtype, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Qualifiers_<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objwbemqualifierset: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Qualifiers_() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(objwbemqualifierset, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsArray<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bisarray: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsArray() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bisarray, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Value: Value::<Identity, Impl, OFFSET>,
            SetValue: SetValue::<Identity, Impl, OFFSET>,
            Name: Name::<Identity, Impl, OFFSET>,
            IsLocal: IsLocal::<Identity, Impl, OFFSET>,
            Origin: Origin::<Identity, Impl, OFFSET>,
            CIMType: CIMType::<Identity, Impl, OFFSET>,
            Qualifiers_: Qualifiers_::<Identity, Impl, OFFSET>,
            IsArray: IsArray::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISWbemProperty as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISWbemPropertySet_Impl: Sized + super::Com::IDispatch_Impl {
    fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Item(&self, strname: &super::super::Foundation::BSTR, iflags: i32) -> ::windows::core::Result<ISWbemProperty>;
    fn Count(&self) -> ::windows::core::Result<i32>;
    fn Add(&self, strname: &super::super::Foundation::BSTR, icimtype: WbemCimtypeEnum, bisarray: i16, iflags: i32) -> ::windows::core::Result<ISWbemProperty>;
    fn Remove(&self, strname: &super::super::Foundation::BSTR, iflags: i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ISWbemPropertySet {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISWbemPropertySet_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemPropertySet_Impl, const OFFSET: isize>() -> ISWbemPropertySet_Vtbl {
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemPropertySet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(punk, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemPropertySet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemproperty: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Item(::core::mem::transmute(&strname), ::core::mem::transmute_copy(&iflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(objwbemproperty, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemPropertySet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, icount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(icount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemPropertySet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, icimtype: WbemCimtypeEnum, bisarray: i16, iflags: i32, objwbemproperty: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Add(::core::mem::transmute(&strname), ::core::mem::transmute_copy(&icimtype), ::core::mem::transmute_copy(&bisarray), ::core::mem::transmute_copy(&iflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(objwbemproperty, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Remove<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemPropertySet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Remove(::core::mem::transmute(&strname), ::core::mem::transmute_copy(&iflags)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            Add: Add::<Identity, Impl, OFFSET>,
            Remove: Remove::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISWbemPropertySet as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISWbemQualifier_Impl: Sized + super::Com::IDispatch_Impl {
    fn Value(&self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SetValue(&self, varvalue: *const super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn IsLocal(&self) -> ::windows::core::Result<i16>;
    fn PropagatesToSubclass(&self) -> ::windows::core::Result<i16>;
    fn SetPropagatesToSubclass(&self, bpropagatestosubclass: i16) -> ::windows::core::Result<()>;
    fn PropagatesToInstance(&self) -> ::windows::core::Result<i16>;
    fn SetPropagatesToInstance(&self, bpropagatestoinstance: i16) -> ::windows::core::Result<()>;
    fn IsOverridable(&self) -> ::windows::core::Result<i16>;
    fn SetIsOverridable(&self, bisoverridable: i16) -> ::windows::core::Result<()>;
    fn IsAmended(&self) -> ::windows::core::Result<i16>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ISWbemQualifier {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISWbemQualifier_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemQualifier_Impl, const OFFSET: isize>() -> ISWbemQualifier_Vtbl {
        unsafe extern "system" fn Value<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemQualifier_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varvalue: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Value() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(varvalue, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValue<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemQualifier_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varvalue: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetValue(::core::mem::transmute_copy(&varvalue)).into()
        }
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemQualifier_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Name() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(strname, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsLocal<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemQualifier_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bislocal: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsLocal() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bislocal, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PropagatesToSubclass<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemQualifier_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bpropagatestosubclass: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PropagatesToSubclass() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bpropagatestosubclass, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPropagatesToSubclass<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemQualifier_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bpropagatestosubclass: i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPropagatesToSubclass(::core::mem::transmute_copy(&bpropagatestosubclass)).into()
        }
        unsafe extern "system" fn PropagatesToInstance<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemQualifier_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bpropagatestoinstance: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PropagatesToInstance() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bpropagatestoinstance, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPropagatesToInstance<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemQualifier_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bpropagatestoinstance: i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPropagatesToInstance(::core::mem::transmute_copy(&bpropagatestoinstance)).into()
        }
        unsafe extern "system" fn IsOverridable<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemQualifier_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bisoverridable: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsOverridable() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bisoverridable, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsOverridable<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemQualifier_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bisoverridable: i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetIsOverridable(::core::mem::transmute_copy(&bisoverridable)).into()
        }
        unsafe extern "system" fn IsAmended<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemQualifier_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bisamended: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsAmended() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bisamended, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Value: Value::<Identity, Impl, OFFSET>,
            SetValue: SetValue::<Identity, Impl, OFFSET>,
            Name: Name::<Identity, Impl, OFFSET>,
            IsLocal: IsLocal::<Identity, Impl, OFFSET>,
            PropagatesToSubclass: PropagatesToSubclass::<Identity, Impl, OFFSET>,
            SetPropagatesToSubclass: SetPropagatesToSubclass::<Identity, Impl, OFFSET>,
            PropagatesToInstance: PropagatesToInstance::<Identity, Impl, OFFSET>,
            SetPropagatesToInstance: SetPropagatesToInstance::<Identity, Impl, OFFSET>,
            IsOverridable: IsOverridable::<Identity, Impl, OFFSET>,
            SetIsOverridable: SetIsOverridable::<Identity, Impl, OFFSET>,
            IsAmended: IsAmended::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISWbemQualifier as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISWbemQualifierSet_Impl: Sized + super::Com::IDispatch_Impl {
    fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Item(&self, name: &super::super::Foundation::BSTR, iflags: i32) -> ::windows::core::Result<ISWbemQualifier>;
    fn Count(&self) -> ::windows::core::Result<i32>;
    fn Add(&self, strname: &super::super::Foundation::BSTR, varval: *const super::Com::VARIANT, bpropagatestosubclass: i16, bpropagatestoinstance: i16, bisoverridable: i16, iflags: i32) -> ::windows::core::Result<ISWbemQualifier>;
    fn Remove(&self, strname: &super::super::Foundation::BSTR, iflags: i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ISWbemQualifierSet {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISWbemQualifierSet_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemQualifierSet_Impl, const OFFSET: isize>() -> ISWbemQualifierSet_Vtbl {
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemQualifierSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(punk, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemQualifierSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemqualifier: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Item(::core::mem::transmute(&name), ::core::mem::transmute_copy(&iflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(objwbemqualifier, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemQualifierSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, icount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(icount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemQualifierSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varval: *const super::Com::VARIANT, bpropagatestosubclass: i16, bpropagatestoinstance: i16, bisoverridable: i16, iflags: i32, objwbemqualifier: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Add(::core::mem::transmute(&strname), ::core::mem::transmute_copy(&varval), ::core::mem::transmute_copy(&bpropagatestosubclass), ::core::mem::transmute_copy(&bpropagatestoinstance), ::core::mem::transmute_copy(&bisoverridable), ::core::mem::transmute_copy(&iflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(objwbemqualifier, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Remove<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemQualifierSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Remove(::core::mem::transmute(&strname), ::core::mem::transmute_copy(&iflags)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            Add: Add::<Identity, Impl, OFFSET>,
            Remove: Remove::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISWbemQualifierSet as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISWbemRefreshableItem_Impl: Sized + super::Com::IDispatch_Impl {
    fn Index(&self) -> ::windows::core::Result<i32>;
    fn Refresher(&self) -> ::windows::core::Result<ISWbemRefresher>;
    fn IsSet(&self) -> ::windows::core::Result<i16>;
    fn Object(&self) -> ::windows::core::Result<ISWbemObjectEx>;
    fn ObjectSet(&self) -> ::windows::core::Result<ISWbemObjectSet>;
    fn Remove(&self, iflags: i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ISWbemRefreshableItem {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISWbemRefreshableItem_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemRefreshableItem_Impl, const OFFSET: isize>() -> ISWbemRefreshableItem_Vtbl {
        unsafe extern "system" fn Index<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemRefreshableItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iindex: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Index() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(iindex, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresher<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemRefreshableItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objwbemrefresher: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Refresher() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(objwbemrefresher, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsSet<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemRefreshableItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bisset: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsSet() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bisset, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Object<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemRefreshableItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objwbemobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Object() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(objwbemobject, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ObjectSet<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemRefreshableItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objwbemobjectset: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ObjectSet() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(objwbemobjectset, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Remove<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemRefreshableItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Remove(::core::mem::transmute_copy(&iflags)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Index: Index::<Identity, Impl, OFFSET>,
            Refresher: Refresher::<Identity, Impl, OFFSET>,
            IsSet: IsSet::<Identity, Impl, OFFSET>,
            Object: Object::<Identity, Impl, OFFSET>,
            ObjectSet: ObjectSet::<Identity, Impl, OFFSET>,
            Remove: Remove::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISWbemRefreshableItem as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISWbemRefresher_Impl: Sized + super::Com::IDispatch_Impl {
    fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Item(&self, iindex: i32) -> ::windows::core::Result<ISWbemRefreshableItem>;
    fn Count(&self) -> ::windows::core::Result<i32>;
    fn Add(&self, objwbemservices: &::core::option::Option<ISWbemServicesEx>, bsinstancepath: &super::super::Foundation::BSTR, iflags: i32, objwbemnamedvalueset: &::core::option::Option<super::Com::IDispatch>) -> ::windows::core::Result<ISWbemRefreshableItem>;
    fn AddEnum(&self, objwbemservices: &::core::option::Option<ISWbemServicesEx>, bsclassname: &super::super::Foundation::BSTR, iflags: i32, objwbemnamedvalueset: &::core::option::Option<super::Com::IDispatch>) -> ::windows::core::Result<ISWbemRefreshableItem>;
    fn Remove(&self, iindex: i32, iflags: i32) -> ::windows::core::Result<()>;
    fn Refresh(&self, iflags: i32) -> ::windows::core::Result<()>;
    fn AutoReconnect(&self) -> ::windows::core::Result<i16>;
    fn SetAutoReconnect(&self, bcount: i16) -> ::windows::core::Result<()>;
    fn DeleteAll(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ISWbemRefresher {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISWbemRefresher_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemRefresher_Impl, const OFFSET: isize>() -> ISWbemRefresher_Vtbl {
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemRefresher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(punk, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemRefresher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iindex: i32, objwbemrefreshableitem: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Item(::core::mem::transmute_copy(&iindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(objwbemrefreshableitem, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemRefresher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, icount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(icount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemRefresher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objwbemservices: *mut ::core::ffi::c_void, bsinstancepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void, objwbemrefreshableitem: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Add(::core::mem::transmute(&objwbemservices), ::core::mem::transmute(&bsinstancepath), ::core::mem::transmute_copy(&iflags), ::core::mem::transmute(&objwbemnamedvalueset)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(objwbemrefreshableitem, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddEnum<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemRefresher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objwbemservices: *mut ::core::ffi::c_void, bsclassname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void, objwbemrefreshableitem: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AddEnum(::core::mem::transmute(&objwbemservices), ::core::mem::transmute(&bsclassname), ::core::mem::transmute_copy(&iflags), ::core::mem::transmute(&objwbemnamedvalueset)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(objwbemrefreshableitem, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Remove<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemRefresher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iindex: i32, iflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Remove(::core::mem::transmute_copy(&iindex), ::core::mem::transmute_copy(&iflags)).into()
        }
        unsafe extern "system" fn Refresh<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemRefresher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Refresh(::core::mem::transmute_copy(&iflags)).into()
        }
        unsafe extern "system" fn AutoReconnect<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemRefresher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bcount: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AutoReconnect() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bcount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoReconnect<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemRefresher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bcount: i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAutoReconnect(::core::mem::transmute_copy(&bcount)).into()
        }
        unsafe extern "system" fn DeleteAll<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemRefresher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteAll().into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            Add: Add::<Identity, Impl, OFFSET>,
            AddEnum: AddEnum::<Identity, Impl, OFFSET>,
            Remove: Remove::<Identity, Impl, OFFSET>,
            Refresh: Refresh::<Identity, Impl, OFFSET>,
            AutoReconnect: AutoReconnect::<Identity, Impl, OFFSET>,
            SetAutoReconnect: SetAutoReconnect::<Identity, Impl, OFFSET>,
            DeleteAll: DeleteAll::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISWbemRefresher as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISWbemSecurity_Impl: Sized + super::Com::IDispatch_Impl {
    fn ImpersonationLevel(&self) -> ::windows::core::Result<WbemImpersonationLevelEnum>;
    fn SetImpersonationLevel(&self, iimpersonationlevel: WbemImpersonationLevelEnum) -> ::windows::core::Result<()>;
    fn AuthenticationLevel(&self) -> ::windows::core::Result<WbemAuthenticationLevelEnum>;
    fn SetAuthenticationLevel(&self, iauthenticationlevel: WbemAuthenticationLevelEnum) -> ::windows::core::Result<()>;
    fn Privileges(&self) -> ::windows::core::Result<ISWbemPrivilegeSet>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ISWbemSecurity {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISWbemSecurity_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemSecurity_Impl, const OFFSET: isize>() -> ISWbemSecurity_Vtbl {
        unsafe extern "system" fn ImpersonationLevel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemSecurity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iimpersonationlevel: *mut WbemImpersonationLevelEnum) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ImpersonationLevel() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(iimpersonationlevel, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetImpersonationLevel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemSecurity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iimpersonationlevel: WbemImpersonationLevelEnum) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetImpersonationLevel(::core::mem::transmute_copy(&iimpersonationlevel)).into()
        }
        unsafe extern "system" fn AuthenticationLevel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemSecurity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iauthenticationlevel: *mut WbemAuthenticationLevelEnum) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AuthenticationLevel() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(iauthenticationlevel, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAuthenticationLevel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemSecurity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iauthenticationlevel: WbemAuthenticationLevelEnum) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAuthenticationLevel(::core::mem::transmute_copy(&iauthenticationlevel)).into()
        }
        unsafe extern "system" fn Privileges<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemSecurity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objwbemprivilegeset: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Privileges() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(objwbemprivilegeset, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            ImpersonationLevel: ImpersonationLevel::<Identity, Impl, OFFSET>,
            SetImpersonationLevel: SetImpersonationLevel::<Identity, Impl, OFFSET>,
            AuthenticationLevel: AuthenticationLevel::<Identity, Impl, OFFSET>,
            SetAuthenticationLevel: SetAuthenticationLevel::<Identity, Impl, OFFSET>,
            Privileges: Privileges::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISWbemSecurity as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISWbemServices_Impl: Sized + super::Com::IDispatch_Impl {
    fn Get(&self, strobjectpath: &super::super::Foundation::BSTR, iflags: i32, objwbemnamedvalueset: &::core::option::Option<super::Com::IDispatch>) -> ::windows::core::Result<ISWbemObject>;
    fn GetAsync(&self, objwbemsink: &::core::option::Option<super::Com::IDispatch>, strobjectpath: &super::super::Foundation::BSTR, iflags: i32, objwbemnamedvalueset: &::core::option::Option<super::Com::IDispatch>, objwbemasynccontext: &::core::option::Option<super::Com::IDispatch>) -> ::windows::core::Result<()>;
    fn Delete(&self, strobjectpath: &super::super::Foundation::BSTR, iflags: i32, objwbemnamedvalueset: &::core::option::Option<super::Com::IDispatch>) -> ::windows::core::Result<()>;
    fn DeleteAsync(&self, objwbemsink: &::core::option::Option<super::Com::IDispatch>, strobjectpath: &super::super::Foundation::BSTR, iflags: i32, objwbemnamedvalueset: &::core::option::Option<super::Com::IDispatch>, objwbemasynccontext: &::core::option::Option<super::Com::IDispatch>) -> ::windows::core::Result<()>;
    fn InstancesOf(&self, strclass: &super::super::Foundation::BSTR, iflags: i32, objwbemnamedvalueset: &::core::option::Option<super::Com::IDispatch>) -> ::windows::core::Result<ISWbemObjectSet>;
    fn InstancesOfAsync(&self, objwbemsink: &::core::option::Option<super::Com::IDispatch>, strclass: &super::super::Foundation::BSTR, iflags: i32, objwbemnamedvalueset: &::core::option::Option<super::Com::IDispatch>, objwbemasynccontext: &::core::option::Option<super::Com::IDispatch>) -> ::windows::core::Result<()>;
    fn SubclassesOf(&self, strsuperclass: &super::super::Foundation::BSTR, iflags: i32, objwbemnamedvalueset: &::core::option::Option<super::Com::IDispatch>) -> ::windows::core::Result<ISWbemObjectSet>;
    fn SubclassesOfAsync(&self, objwbemsink: &::core::option::Option<super::Com::IDispatch>, strsuperclass: &super::super::Foundation::BSTR, iflags: i32, objwbemnamedvalueset: &::core::option::Option<super::Com::IDispatch>, objwbemasynccontext: &::core::option::Option<super::Com::IDispatch>) -> ::windows::core::Result<()>;
    fn ExecQuery(&self, strquery: &super::super::Foundation::BSTR, strquerylanguage: &super::super::Foundation::BSTR, iflags: i32, objwbemnamedvalueset: &::core::option::Option<super::Com::IDispatch>) -> ::windows::core::Result<ISWbemObjectSet>;
    fn ExecQueryAsync(&self, objwbemsink: &::core::option::Option<super::Com::IDispatch>, strquery: &super::super::Foundation::BSTR, strquerylanguage: &super::super::Foundation::BSTR, lflags: i32, objwbemnamedvalueset: &::core::option::Option<super::Com::IDispatch>, objwbemasynccontext: &::core::option::Option<super::Com::IDispatch>) -> ::windows::core::Result<()>;
    fn AssociatorsOf(&self, strobjectpath: &super::super::Foundation::BSTR, strassocclass: &super::super::Foundation::BSTR, strresultclass: &super::super::Foundation::BSTR, strresultrole: &super::super::Foundation::BSTR, strrole: &super::super::Foundation::BSTR, bclassesonly: i16, bschemaonly: i16, strrequiredassocqualifier: &super::super::Foundation::BSTR, strrequiredqualifier: &super::super::Foundation::BSTR, iflags: i32, objwbemnamedvalueset: &::core::option::Option<super::Com::IDispatch>) -> ::windows::core::Result<ISWbemObjectSet>;
    fn AssociatorsOfAsync(&self, objwbemsink: &::core::option::Option<super::Com::IDispatch>, strobjectpath: &super::super::Foundation::BSTR, strassocclass: &super::super::Foundation::BSTR, strresultclass: &super::super::Foundation::BSTR, strresultrole: &super::super::Foundation::BSTR, strrole: &super::super::Foundation::BSTR, bclassesonly: i16, bschemaonly: i16, strrequiredassocqualifier: &super::super::Foundation::BSTR, strrequiredqualifier: &super::super::Foundation::BSTR, iflags: i32, objwbemnamedvalueset: &::core::option::Option<super::Com::IDispatch>, objwbemasynccontext: &::core::option::Option<super::Com::IDispatch>) -> ::windows::core::Result<()>;
    fn ReferencesTo(&self, strobjectpath: &super::super::Foundation::BSTR, strresultclass: &super::super::Foundation::BSTR, strrole: &super::super::Foundation::BSTR, bclassesonly: i16, bschemaonly: i16, strrequiredqualifier: &super::super::Foundation::BSTR, iflags: i32, objwbemnamedvalueset: &::core::option::Option<super::Com::IDispatch>) -> ::windows::core::Result<ISWbemObjectSet>;
    fn ReferencesToAsync(&self, objwbemsink: &::core::option::Option<super::Com::IDispatch>, strobjectpath: &super::super::Foundation::BSTR, strresultclass: &super::super::Foundation::BSTR, strrole: &super::super::Foundation::BSTR, bclassesonly: i16, bschemaonly: i16, strrequiredqualifier: &super::super::Foundation::BSTR, iflags: i32, objwbemnamedvalueset: &::core::option::Option<super::Com::IDispatch>, objwbemasynccontext: &::core::option::Option<super::Com::IDispatch>) -> ::windows::core::Result<()>;
    fn ExecNotificationQuery(&self, strquery: &super::super::Foundation::BSTR, strquerylanguage: &super::super::Foundation::BSTR, iflags: i32, objwbemnamedvalueset: &::core::option::Option<super::Com::IDispatch>) -> ::windows::core::Result<ISWbemEventSource>;
    fn ExecNotificationQueryAsync(&self, objwbemsink: &::core::option::Option<super::Com::IDispatch>, strquery: &super::super::Foundation::BSTR, strquerylanguage: &super::super::Foundation::BSTR, iflags: i32, objwbemnamedvalueset: &::core::option::Option<super::Com::IDispatch>, objwbemasynccontext: &::core::option::Option<super::Com::IDispatch>) -> ::windows::core::Result<()>;
    fn ExecMethod(&self, strobjectpath: &super::super::Foundation::BSTR, strmethodname: &super::super::Foundation::BSTR, objwbeminparameters: &::core::option::Option<super::Com::IDispatch>, iflags: i32, objwbemnamedvalueset: &::core::option::Option<super::Com::IDispatch>) -> ::windows::core::Result<ISWbemObject>;
    fn ExecMethodAsync(&self, objwbemsink: &::core::option::Option<super::Com::IDispatch>, strobjectpath: &super::super::Foundation::BSTR, strmethodname: &super::super::Foundation::BSTR, objwbeminparameters: &::core::option::Option<super::Com::IDispatch>, iflags: i32, objwbemnamedvalueset: &::core::option::Option<super::Com::IDispatch>, objwbemasynccontext: &::core::option::Option<super::Com::IDispatch>) -> ::windows::core::Result<()>;
    fn Security_(&self) -> ::windows::core::Result<ISWbemSecurity>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ISWbemServices {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISWbemServices_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemServices_Impl, const OFFSET: isize>() -> ISWbemServices_Vtbl {
        unsafe extern "system" fn Get<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strobjectpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void, objwbemobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Get(::core::mem::transmute(&strobjectpath), ::core::mem::transmute_copy(&iflags), ::core::mem::transmute(&objwbemnamedvalueset)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(objwbemobject, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAsync<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objwbemsink: *mut ::core::ffi::c_void, strobjectpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void, objwbemasynccontext: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetAsync(::core::mem::transmute(&objwbemsink), ::core::mem::transmute(&strobjectpath), ::core::mem::transmute_copy(&iflags), ::core::mem::transmute(&objwbemnamedvalueset), ::core::mem::transmute(&objwbemasynccontext)).into()
        }
        unsafe extern "system" fn Delete<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strobjectpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Delete(::core::mem::transmute(&strobjectpath), ::core::mem::transmute_copy(&iflags), ::core::mem::transmute(&objwbemnamedvalueset)).into()
        }
        unsafe extern "system" fn DeleteAsync<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objwbemsink: *mut ::core::ffi::c_void, strobjectpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void, objwbemasynccontext: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteAsync(::core::mem::transmute(&objwbemsink), ::core::mem::transmute(&strobjectpath), ::core::mem::transmute_copy(&iflags), ::core::mem::transmute(&objwbemnamedvalueset), ::core::mem::transmute(&objwbemasynccontext)).into()
        }
        unsafe extern "system" fn InstancesOf<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void, objwbemobjectset: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.InstancesOf(::core::mem::transmute(&strclass), ::core::mem::transmute_copy(&iflags), ::core::mem::transmute(&objwbemnamedvalueset)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(objwbemobjectset, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InstancesOfAsync<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objwbemsink: *mut ::core::ffi::c_void, strclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void, objwbemasynccontext: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InstancesOfAsync(::core::mem::transmute(&objwbemsink), ::core::mem::transmute(&strclass), ::core::mem::transmute_copy(&iflags), ::core::mem::transmute(&objwbemnamedvalueset), ::core::mem::transmute(&objwbemasynccontext)).into()
        }
        unsafe extern "system" fn SubclassesOf<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strsuperclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void, objwbemobjectset: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SubclassesOf(::core::mem::transmute(&strsuperclass), ::core::mem::transmute_copy(&iflags), ::core::mem::transmute(&objwbemnamedvalueset)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(objwbemobjectset, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SubclassesOfAsync<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objwbemsink: *mut ::core::ffi::c_void, strsuperclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void, objwbemasynccontext: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SubclassesOfAsync(::core::mem::transmute(&objwbemsink), ::core::mem::transmute(&strsuperclass), ::core::mem::transmute_copy(&iflags), ::core::mem::transmute(&objwbemnamedvalueset), ::core::mem::transmute(&objwbemasynccontext)).into()
        }
        unsafe extern "system" fn ExecQuery<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strquery: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strquerylanguage: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void, objwbemobjectset: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ExecQuery(::core::mem::transmute(&strquery), ::core::mem::transmute(&strquerylanguage), ::core::mem::transmute_copy(&iflags), ::core::mem::transmute(&objwbemnamedvalueset)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(objwbemobjectset, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExecQueryAsync<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objwbemsink: *mut ::core::ffi::c_void, strquery: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strquerylanguage: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void, objwbemasynccontext: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ExecQueryAsync(::core::mem::transmute(&objwbemsink), ::core::mem::transmute(&strquery), ::core::mem::transmute(&strquerylanguage), ::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&objwbemnamedvalueset), ::core::mem::transmute(&objwbemasynccontext)).into()
        }
        unsafe extern "system" fn AssociatorsOf<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemServices_Impl, const OFFSET: isize>(
            this: *mut ::core::ffi::c_void,
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
            objwbemnamedvalueset: *mut ::core::ffi::c_void,
            objwbemobjectset: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AssociatorsOf(::core::mem::transmute(&strobjectpath), ::core::mem::transmute(&strassocclass), ::core::mem::transmute(&strresultclass), ::core::mem::transmute(&strresultrole), ::core::mem::transmute(&strrole), ::core::mem::transmute_copy(&bclassesonly), ::core::mem::transmute_copy(&bschemaonly), ::core::mem::transmute(&strrequiredassocqualifier), ::core::mem::transmute(&strrequiredqualifier), ::core::mem::transmute_copy(&iflags), ::core::mem::transmute(&objwbemnamedvalueset)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(objwbemobjectset, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AssociatorsOfAsync<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemServices_Impl, const OFFSET: isize>(
            this: *mut ::core::ffi::c_void,
            objwbemsink: *mut ::core::ffi::c_void,
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
            objwbemnamedvalueset: *mut ::core::ffi::c_void,
            objwbemasynccontext: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AssociatorsOfAsync(
                ::core::mem::transmute(&objwbemsink),
                ::core::mem::transmute(&strobjectpath),
                ::core::mem::transmute(&strassocclass),
                ::core::mem::transmute(&strresultclass),
                ::core::mem::transmute(&strresultrole),
                ::core::mem::transmute(&strrole),
                ::core::mem::transmute_copy(&bclassesonly),
                ::core::mem::transmute_copy(&bschemaonly),
                ::core::mem::transmute(&strrequiredassocqualifier),
                ::core::mem::transmute(&strrequiredqualifier),
                ::core::mem::transmute_copy(&iflags),
                ::core::mem::transmute(&objwbemnamedvalueset),
                ::core::mem::transmute(&objwbemasynccontext),
            )
            .into()
        }
        unsafe extern "system" fn ReferencesTo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strobjectpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strresultclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strrole: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bclassesonly: i16, bschemaonly: i16, strrequiredqualifier: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void, objwbemobjectset: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ReferencesTo(::core::mem::transmute(&strobjectpath), ::core::mem::transmute(&strresultclass), ::core::mem::transmute(&strrole), ::core::mem::transmute_copy(&bclassesonly), ::core::mem::transmute_copy(&bschemaonly), ::core::mem::transmute(&strrequiredqualifier), ::core::mem::transmute_copy(&iflags), ::core::mem::transmute(&objwbemnamedvalueset)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(objwbemobjectset, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReferencesToAsync<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objwbemsink: *mut ::core::ffi::c_void, strobjectpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strresultclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strrole: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bclassesonly: i16, bschemaonly: i16, strrequiredqualifier: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void, objwbemasynccontext: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReferencesToAsync(::core::mem::transmute(&objwbemsink), ::core::mem::transmute(&strobjectpath), ::core::mem::transmute(&strresultclass), ::core::mem::transmute(&strrole), ::core::mem::transmute_copy(&bclassesonly), ::core::mem::transmute_copy(&bschemaonly), ::core::mem::transmute(&strrequiredqualifier), ::core::mem::transmute_copy(&iflags), ::core::mem::transmute(&objwbemnamedvalueset), ::core::mem::transmute(&objwbemasynccontext)).into()
        }
        unsafe extern "system" fn ExecNotificationQuery<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strquery: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strquerylanguage: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void, objwbemeventsource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ExecNotificationQuery(::core::mem::transmute(&strquery), ::core::mem::transmute(&strquerylanguage), ::core::mem::transmute_copy(&iflags), ::core::mem::transmute(&objwbemnamedvalueset)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(objwbemeventsource, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExecNotificationQueryAsync<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objwbemsink: *mut ::core::ffi::c_void, strquery: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strquerylanguage: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void, objwbemasynccontext: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ExecNotificationQueryAsync(::core::mem::transmute(&objwbemsink), ::core::mem::transmute(&strquery), ::core::mem::transmute(&strquerylanguage), ::core::mem::transmute_copy(&iflags), ::core::mem::transmute(&objwbemnamedvalueset), ::core::mem::transmute(&objwbemasynccontext)).into()
        }
        unsafe extern "system" fn ExecMethod<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strobjectpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strmethodname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, objwbeminparameters: *mut ::core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void, objwbemoutparameters: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ExecMethod(::core::mem::transmute(&strobjectpath), ::core::mem::transmute(&strmethodname), ::core::mem::transmute(&objwbeminparameters), ::core::mem::transmute_copy(&iflags), ::core::mem::transmute(&objwbemnamedvalueset)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(objwbemoutparameters, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExecMethodAsync<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objwbemsink: *mut ::core::ffi::c_void, strobjectpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strmethodname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, objwbeminparameters: *mut ::core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void, objwbemasynccontext: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ExecMethodAsync(::core::mem::transmute(&objwbemsink), ::core::mem::transmute(&strobjectpath), ::core::mem::transmute(&strmethodname), ::core::mem::transmute(&objwbeminparameters), ::core::mem::transmute_copy(&iflags), ::core::mem::transmute(&objwbemnamedvalueset), ::core::mem::transmute(&objwbemasynccontext)).into()
        }
        unsafe extern "system" fn Security_<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objwbemsecurity: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Security_() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(objwbemsecurity, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Get: Get::<Identity, Impl, OFFSET>,
            GetAsync: GetAsync::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
            DeleteAsync: DeleteAsync::<Identity, Impl, OFFSET>,
            InstancesOf: InstancesOf::<Identity, Impl, OFFSET>,
            InstancesOfAsync: InstancesOfAsync::<Identity, Impl, OFFSET>,
            SubclassesOf: SubclassesOf::<Identity, Impl, OFFSET>,
            SubclassesOfAsync: SubclassesOfAsync::<Identity, Impl, OFFSET>,
            ExecQuery: ExecQuery::<Identity, Impl, OFFSET>,
            ExecQueryAsync: ExecQueryAsync::<Identity, Impl, OFFSET>,
            AssociatorsOf: AssociatorsOf::<Identity, Impl, OFFSET>,
            AssociatorsOfAsync: AssociatorsOfAsync::<Identity, Impl, OFFSET>,
            ReferencesTo: ReferencesTo::<Identity, Impl, OFFSET>,
            ReferencesToAsync: ReferencesToAsync::<Identity, Impl, OFFSET>,
            ExecNotificationQuery: ExecNotificationQuery::<Identity, Impl, OFFSET>,
            ExecNotificationQueryAsync: ExecNotificationQueryAsync::<Identity, Impl, OFFSET>,
            ExecMethod: ExecMethod::<Identity, Impl, OFFSET>,
            ExecMethodAsync: ExecMethodAsync::<Identity, Impl, OFFSET>,
            Security_: Security_::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISWbemServices as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISWbemServicesEx_Impl: Sized + super::Com::IDispatch_Impl + ISWbemServices_Impl {
    fn Put(&self, objwbemobject: &::core::option::Option<ISWbemObjectEx>, iflags: i32, objwbemnamedvalueset: &::core::option::Option<super::Com::IDispatch>) -> ::windows::core::Result<ISWbemObjectPath>;
    fn PutAsync(&self, objwbemsink: &::core::option::Option<ISWbemSink>, objwbemobject: &::core::option::Option<ISWbemObjectEx>, iflags: i32, objwbemnamedvalueset: &::core::option::Option<super::Com::IDispatch>, objwbemasynccontext: &::core::option::Option<super::Com::IDispatch>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ISWbemServicesEx {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISWbemServicesEx_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemServicesEx_Impl, const OFFSET: isize>() -> ISWbemServicesEx_Vtbl {
        unsafe extern "system" fn Put<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemServicesEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objwbemobject: *mut ::core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void, objwbemobjectpath: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Put(::core::mem::transmute(&objwbemobject), ::core::mem::transmute_copy(&iflags), ::core::mem::transmute(&objwbemnamedvalueset)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(objwbemobjectpath, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PutAsync<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemServicesEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objwbemsink: *mut ::core::ffi::c_void, objwbemobject: *mut ::core::ffi::c_void, iflags: i32, objwbemnamedvalueset: *mut ::core::ffi::c_void, objwbemasynccontext: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PutAsync(::core::mem::transmute(&objwbemsink), ::core::mem::transmute(&objwbemobject), ::core::mem::transmute_copy(&iflags), ::core::mem::transmute(&objwbemnamedvalueset), ::core::mem::transmute(&objwbemasynccontext)).into()
        }
        Self { base__: ISWbemServices_Vtbl::new::<Identity, Impl, OFFSET>(), Put: Put::<Identity, Impl, OFFSET>, PutAsync: PutAsync::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISWbemServicesEx as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<ISWbemServices as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISWbemSink_Impl: Sized + super::Com::IDispatch_Impl {
    fn Cancel(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ISWbemSink {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISWbemSink_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemSink_Impl, const OFFSET: isize>() -> ISWbemSink_Vtbl {
        unsafe extern "system" fn Cancel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Cancel().into()
        }
        Self { base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(), Cancel: Cancel::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISWbemSink as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISWbemSinkEvents_Impl: Sized + super::Com::IDispatch_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ISWbemSinkEvents {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISWbemSinkEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISWbemSinkEvents_Impl, const OFFSET: isize>() -> ISWbemSinkEvents_Vtbl {
        Self { base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISWbemSinkEvents as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
pub trait IUnsecuredApartment_Impl: Sized {
    fn CreateObjectStub(&self, pobject: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<::windows::core::IUnknown>;
}
impl ::windows::core::RuntimeName for IUnsecuredApartment {}
impl IUnsecuredApartment_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUnsecuredApartment_Impl, const OFFSET: isize>() -> IUnsecuredApartment_Vtbl {
        unsafe extern "system" fn CreateObjectStub<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUnsecuredApartment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pobject: *mut ::core::ffi::c_void, ppstub: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateObjectStub(::core::mem::transmute(&pobject)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppstub, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), CreateObjectStub: CreateObjectStub::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUnsecuredApartment as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWMIExtension_Impl: Sized + super::Com::IDispatch_Impl {
    fn WMIObjectPath(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetWMIObject(&self) -> ::windows::core::Result<ISWbemObject>;
    fn GetWMIServices(&self) -> ::windows::core::Result<ISWbemServices>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IWMIExtension {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMIExtension_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMIExtension_Impl, const OFFSET: isize>() -> IWMIExtension_Vtbl {
        unsafe extern "system" fn WMIObjectPath<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMIExtension_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strwmiobjectpath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.WMIObjectPath() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(strwmiobjectpath, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWMIObject<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMIExtension_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objwmiobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetWMIObject() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(objwmiobject, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWMIServices<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMIExtension_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objwmiservices: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetWMIServices() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(objwmiservices, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            WMIObjectPath: WMIObjectPath::<Identity, Impl, OFFSET>,
            GetWMIObject: GetWMIObject::<Identity, Impl, OFFSET>,
            GetWMIServices: GetWMIServices::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMIExtension as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
pub trait IWbemAddressResolution_Impl: Sized {
    fn Resolve(&self, wsznamespacepath: &::windows::core::PCWSTR, wszaddresstype: ::windows::core::PWSTR, pdwaddresslength: *mut u32, pabbinaryaddress: *mut *mut u8) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IWbemAddressResolution {}
impl IWbemAddressResolution_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemAddressResolution_Impl, const OFFSET: isize>() -> IWbemAddressResolution_Vtbl {
        unsafe extern "system" fn Resolve<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemAddressResolution_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wsznamespacepath: ::windows::core::PCWSTR, wszaddresstype: ::windows::core::PWSTR, pdwaddresslength: *mut u32, pabbinaryaddress: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Resolve(::core::mem::transmute(&wsznamespacepath), ::core::mem::transmute_copy(&wszaddresstype), ::core::mem::transmute_copy(&pdwaddresslength), ::core::mem::transmute_copy(&pabbinaryaddress)).into()
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), Resolve: Resolve::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWbemAddressResolution as ::windows::core::Interface>::IID
    }
}
pub trait IWbemBackupRestore_Impl: Sized {
    fn Backup(&self, strbackuptofile: &::windows::core::PCWSTR, lflags: i32) -> ::windows::core::Result<()>;
    fn Restore(&self, strrestorefromfile: &::windows::core::PCWSTR, lflags: i32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IWbemBackupRestore {}
impl IWbemBackupRestore_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemBackupRestore_Impl, const OFFSET: isize>() -> IWbemBackupRestore_Vtbl {
        unsafe extern "system" fn Backup<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemBackupRestore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strbackuptofile: ::windows::core::PCWSTR, lflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Backup(::core::mem::transmute(&strbackuptofile), ::core::mem::transmute_copy(&lflags)).into()
        }
        unsafe extern "system" fn Restore<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemBackupRestore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strrestorefromfile: ::windows::core::PCWSTR, lflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Restore(::core::mem::transmute(&strrestorefromfile), ::core::mem::transmute_copy(&lflags)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Backup: Backup::<Identity, Impl, OFFSET>,
            Restore: Restore::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWbemBackupRestore as ::windows::core::Interface>::IID
    }
}
pub trait IWbemBackupRestoreEx_Impl: Sized + IWbemBackupRestore_Impl {
    fn Pause(&self) -> ::windows::core::Result<()>;
    fn Resume(&self) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IWbemBackupRestoreEx {}
impl IWbemBackupRestoreEx_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemBackupRestoreEx_Impl, const OFFSET: isize>() -> IWbemBackupRestoreEx_Vtbl {
        unsafe extern "system" fn Pause<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemBackupRestoreEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Pause().into()
        }
        unsafe extern "system" fn Resume<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemBackupRestoreEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Resume().into()
        }
        Self {
            base__: IWbemBackupRestore_Vtbl::new::<Identity, Impl, OFFSET>(),
            Pause: Pause::<Identity, Impl, OFFSET>,
            Resume: Resume::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWbemBackupRestoreEx as ::windows::core::Interface>::IID || iid == &<IWbemBackupRestore as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWbemCallResult_Impl: Sized {
    fn GetResultObject(&self, ltimeout: i32) -> ::windows::core::Result<IWbemClassObject>;
    fn GetResultString(&self, ltimeout: i32) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetResultServices(&self, ltimeout: i32) -> ::windows::core::Result<IWbemServices>;
    fn GetCallStatus(&self, ltimeout: i32) -> ::windows::core::Result<i32>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IWbemCallResult {}
#[cfg(feature = "Win32_Foundation")]
impl IWbemCallResult_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemCallResult_Impl, const OFFSET: isize>() -> IWbemCallResult_Vtbl {
        unsafe extern "system" fn GetResultObject<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemCallResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ltimeout: i32, ppresultobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetResultObject(::core::mem::transmute_copy(&ltimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresultobject, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetResultString<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemCallResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ltimeout: i32, pstrresultstring: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetResultString(::core::mem::transmute_copy(&ltimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pstrresultstring, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetResultServices<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemCallResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ltimeout: i32, ppservices: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetResultServices(::core::mem::transmute_copy(&ltimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppservices, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCallStatus<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemCallResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ltimeout: i32, plstatus: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCallStatus(::core::mem::transmute_copy(&ltimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plstatus, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetResultObject: GetResultObject::<Identity, Impl, OFFSET>,
            GetResultString: GetResultString::<Identity, Impl, OFFSET>,
            GetResultServices: GetResultServices::<Identity, Impl, OFFSET>,
            GetCallStatus: GetCallStatus::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWbemCallResult as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWbemClassObject_Impl: Sized {
    fn GetQualifierSet(&self) -> ::windows::core::Result<IWbemQualifierSet>;
    fn Get(&self, wszname: &::windows::core::PCWSTR, lflags: i32, pval: *mut super::Com::VARIANT, ptype: *mut i32, plflavor: *mut i32) -> ::windows::core::Result<()>;
    fn Put(&self, wszname: &::windows::core::PCWSTR, lflags: i32, pval: *const super::Com::VARIANT, r#type: i32) -> ::windows::core::Result<()>;
    fn Delete(&self, wszname: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn GetNames(&self, wszqualifiername: &::windows::core::PCWSTR, lflags: i32, pqualifierval: *const super::Com::VARIANT) -> ::windows::core::Result<*mut super::Com::SAFEARRAY>;
    fn BeginEnumeration(&self, lenumflags: i32) -> ::windows::core::Result<()>;
    fn Next(&self, lflags: i32, strname: *mut super::super::Foundation::BSTR, pval: *mut super::Com::VARIANT, ptype: *mut i32, plflavor: *mut i32) -> ::windows::core::Result<()>;
    fn EndEnumeration(&self) -> ::windows::core::Result<()>;
    fn GetPropertyQualifierSet(&self, wszproperty: &::windows::core::PCWSTR) -> ::windows::core::Result<IWbemQualifierSet>;
    fn Clone(&self) -> ::windows::core::Result<IWbemClassObject>;
    fn GetObjectText(&self, lflags: i32) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SpawnDerivedClass(&self, lflags: i32) -> ::windows::core::Result<IWbemClassObject>;
    fn SpawnInstance(&self, lflags: i32) -> ::windows::core::Result<IWbemClassObject>;
    fn CompareTo(&self, lflags: i32, pcompareto: &::core::option::Option<IWbemClassObject>) -> ::windows::core::Result<()>;
    fn GetPropertyOrigin(&self, wszname: &::windows::core::PCWSTR) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn InheritsFrom(&self, strancestor: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn GetMethod(&self, wszname: &::windows::core::PCWSTR, lflags: i32, ppinsignature: *mut ::core::option::Option<IWbemClassObject>, ppoutsignature: *mut ::core::option::Option<IWbemClassObject>) -> ::windows::core::Result<()>;
    fn PutMethod(&self, wszname: &::windows::core::PCWSTR, lflags: i32, pinsignature: &::core::option::Option<IWbemClassObject>, poutsignature: &::core::option::Option<IWbemClassObject>) -> ::windows::core::Result<()>;
    fn DeleteMethod(&self, wszname: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn BeginMethodEnumeration(&self, lenumflags: i32) -> ::windows::core::Result<()>;
    fn NextMethod(&self, lflags: i32, pstrname: *mut super::super::Foundation::BSTR, ppinsignature: *mut ::core::option::Option<IWbemClassObject>, ppoutsignature: *mut ::core::option::Option<IWbemClassObject>) -> ::windows::core::Result<()>;
    fn EndMethodEnumeration(&self) -> ::windows::core::Result<()>;
    fn GetMethodQualifierSet(&self, wszmethod: &::windows::core::PCWSTR) -> ::windows::core::Result<IWbemQualifierSet>;
    fn GetMethodOrigin(&self, wszmethodname: &::windows::core::PCWSTR) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IWbemClassObject {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWbemClassObject_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemClassObject_Impl, const OFFSET: isize>() -> IWbemClassObject_Vtbl {
        unsafe extern "system" fn GetQualifierSet<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemClassObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqualset: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetQualifierSet() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppqualset, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Get<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemClassObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszname: ::windows::core::PCWSTR, lflags: i32, pval: *mut super::Com::VARIANT, ptype: *mut i32, plflavor: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Get(::core::mem::transmute(&wszname), ::core::mem::transmute_copy(&lflags), ::core::mem::transmute_copy(&pval), ::core::mem::transmute_copy(&ptype), ::core::mem::transmute_copy(&plflavor)).into()
        }
        unsafe extern "system" fn Put<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemClassObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszname: ::windows::core::PCWSTR, lflags: i32, pval: *const super::Com::VARIANT, r#type: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Put(::core::mem::transmute(&wszname), ::core::mem::transmute_copy(&lflags), ::core::mem::transmute_copy(&pval), ::core::mem::transmute_copy(&r#type)).into()
        }
        unsafe extern "system" fn Delete<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemClassObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Delete(::core::mem::transmute(&wszname)).into()
        }
        unsafe extern "system" fn GetNames<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemClassObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszqualifiername: ::windows::core::PCWSTR, lflags: i32, pqualifierval: *const super::Com::VARIANT, pnames: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetNames(::core::mem::transmute(&wszqualifiername), ::core::mem::transmute_copy(&lflags), ::core::mem::transmute_copy(&pqualifierval)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pnames, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginEnumeration<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemClassObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lenumflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.BeginEnumeration(::core::mem::transmute_copy(&lenumflags)).into()
        }
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemClassObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, strname: *mut super::super::Foundation::BSTR, pval: *mut super::Com::VARIANT, ptype: *mut i32, plflavor: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Next(::core::mem::transmute_copy(&lflags), ::core::mem::transmute_copy(&strname), ::core::mem::transmute_copy(&pval), ::core::mem::transmute_copy(&ptype), ::core::mem::transmute_copy(&plflavor)).into()
        }
        unsafe extern "system" fn EndEnumeration<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemClassObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndEnumeration().into()
        }
        unsafe extern "system" fn GetPropertyQualifierSet<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemClassObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszproperty: ::windows::core::PCWSTR, ppqualset: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPropertyQualifierSet(::core::mem::transmute(&wszproperty)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppqualset, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemClassObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcopy: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcopy, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetObjectText<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemClassObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, pstrobjecttext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetObjectText(::core::mem::transmute_copy(&lflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pstrobjecttext, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SpawnDerivedClass<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemClassObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, ppnewclass: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SpawnDerivedClass(::core::mem::transmute_copy(&lflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppnewclass, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SpawnInstance<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemClassObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, ppnewinstance: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SpawnInstance(::core::mem::transmute_copy(&lflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppnewinstance, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CompareTo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemClassObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, pcompareto: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CompareTo(::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&pcompareto)).into()
        }
        unsafe extern "system" fn GetPropertyOrigin<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemClassObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszname: ::windows::core::PCWSTR, pstrclassname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPropertyOrigin(::core::mem::transmute(&wszname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pstrclassname, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InheritsFrom<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemClassObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strancestor: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InheritsFrom(::core::mem::transmute(&strancestor)).into()
        }
        unsafe extern "system" fn GetMethod<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemClassObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszname: ::windows::core::PCWSTR, lflags: i32, ppinsignature: *mut *mut ::core::ffi::c_void, ppoutsignature: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetMethod(::core::mem::transmute(&wszname), ::core::mem::transmute_copy(&lflags), ::core::mem::transmute_copy(&ppinsignature), ::core::mem::transmute_copy(&ppoutsignature)).into()
        }
        unsafe extern "system" fn PutMethod<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemClassObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszname: ::windows::core::PCWSTR, lflags: i32, pinsignature: *mut ::core::ffi::c_void, poutsignature: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PutMethod(::core::mem::transmute(&wszname), ::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&pinsignature), ::core::mem::transmute(&poutsignature)).into()
        }
        unsafe extern "system" fn DeleteMethod<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemClassObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteMethod(::core::mem::transmute(&wszname)).into()
        }
        unsafe extern "system" fn BeginMethodEnumeration<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemClassObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lenumflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.BeginMethodEnumeration(::core::mem::transmute_copy(&lenumflags)).into()
        }
        unsafe extern "system" fn NextMethod<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemClassObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, pstrname: *mut super::super::Foundation::BSTR, ppinsignature: *mut *mut ::core::ffi::c_void, ppoutsignature: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.NextMethod(::core::mem::transmute_copy(&lflags), ::core::mem::transmute_copy(&pstrname), ::core::mem::transmute_copy(&ppinsignature), ::core::mem::transmute_copy(&ppoutsignature)).into()
        }
        unsafe extern "system" fn EndMethodEnumeration<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemClassObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndMethodEnumeration().into()
        }
        unsafe extern "system" fn GetMethodQualifierSet<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemClassObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszmethod: ::windows::core::PCWSTR, ppqualset: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetMethodQualifierSet(::core::mem::transmute(&wszmethod)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppqualset, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMethodOrigin<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemClassObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszmethodname: ::windows::core::PCWSTR, pstrclassname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetMethodOrigin(::core::mem::transmute(&wszmethodname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pstrclassname, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetQualifierSet: GetQualifierSet::<Identity, Impl, OFFSET>,
            Get: Get::<Identity, Impl, OFFSET>,
            Put: Put::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
            GetNames: GetNames::<Identity, Impl, OFFSET>,
            BeginEnumeration: BeginEnumeration::<Identity, Impl, OFFSET>,
            Next: Next::<Identity, Impl, OFFSET>,
            EndEnumeration: EndEnumeration::<Identity, Impl, OFFSET>,
            GetPropertyQualifierSet: GetPropertyQualifierSet::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
            GetObjectText: GetObjectText::<Identity, Impl, OFFSET>,
            SpawnDerivedClass: SpawnDerivedClass::<Identity, Impl, OFFSET>,
            SpawnInstance: SpawnInstance::<Identity, Impl, OFFSET>,
            CompareTo: CompareTo::<Identity, Impl, OFFSET>,
            GetPropertyOrigin: GetPropertyOrigin::<Identity, Impl, OFFSET>,
            InheritsFrom: InheritsFrom::<Identity, Impl, OFFSET>,
            GetMethod: GetMethod::<Identity, Impl, OFFSET>,
            PutMethod: PutMethod::<Identity, Impl, OFFSET>,
            DeleteMethod: DeleteMethod::<Identity, Impl, OFFSET>,
            BeginMethodEnumeration: BeginMethodEnumeration::<Identity, Impl, OFFSET>,
            NextMethod: NextMethod::<Identity, Impl, OFFSET>,
            EndMethodEnumeration: EndMethodEnumeration::<Identity, Impl, OFFSET>,
            GetMethodQualifierSet: GetMethodQualifierSet::<Identity, Impl, OFFSET>,
            GetMethodOrigin: GetMethodOrigin::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWbemClassObject as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWbemClientConnectionTransport_Impl: Sized {
    fn Open(&self, straddresstype: &super::super::Foundation::BSTR, dwbinaryaddresslength: u32, abbinaryaddress: *const u8, strobject: &super::super::Foundation::BSTR, struser: &super::super::Foundation::BSTR, strpassword: &super::super::Foundation::BSTR, strlocale: &super::super::Foundation::BSTR, lflags: i32, pctx: &::core::option::Option<IWbemContext>, riid: *const ::windows::core::GUID, pinterface: *mut *mut ::core::ffi::c_void, pcallres: *mut ::core::option::Option<IWbemCallResult>) -> ::windows::core::Result<()>;
    fn OpenAsync(&self, straddresstype: &super::super::Foundation::BSTR, dwbinaryaddresslength: u32, abbinaryaddress: *const u8, strobject: &super::super::Foundation::BSTR, struser: &super::super::Foundation::BSTR, strpassword: &super::super::Foundation::BSTR, strlocale: &super::super::Foundation::BSTR, lflags: i32, pctx: &::core::option::Option<IWbemContext>, riid: *const ::windows::core::GUID, presponsehandler: &::core::option::Option<IWbemObjectSink>) -> ::windows::core::Result<()>;
    fn Cancel(&self, lflags: i32, phandler: &::core::option::Option<IWbemObjectSink>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IWbemClientConnectionTransport {}
#[cfg(feature = "Win32_Foundation")]
impl IWbemClientConnectionTransport_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemClientConnectionTransport_Impl, const OFFSET: isize>() -> IWbemClientConnectionTransport_Vtbl {
        unsafe extern "system" fn Open<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemClientConnectionTransport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, straddresstype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwbinaryaddresslength: u32, abbinaryaddress: *const u8, strobject: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, struser: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strlocale: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lflags: i32, pctx: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, pinterface: *mut *mut ::core::ffi::c_void, pcallres: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Open(::core::mem::transmute(&straddresstype), ::core::mem::transmute_copy(&dwbinaryaddresslength), ::core::mem::transmute_copy(&abbinaryaddress), ::core::mem::transmute(&strobject), ::core::mem::transmute(&struser), ::core::mem::transmute(&strpassword), ::core::mem::transmute(&strlocale), ::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&pctx), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&pinterface), ::core::mem::transmute_copy(&pcallres))
                .into()
        }
        unsafe extern "system" fn OpenAsync<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemClientConnectionTransport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, straddresstype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwbinaryaddresslength: u32, abbinaryaddress: *const u8, strobject: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, struser: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strlocale: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lflags: i32, pctx: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, presponsehandler: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OpenAsync(::core::mem::transmute(&straddresstype), ::core::mem::transmute_copy(&dwbinaryaddresslength), ::core::mem::transmute_copy(&abbinaryaddress), ::core::mem::transmute(&strobject), ::core::mem::transmute(&struser), ::core::mem::transmute(&strpassword), ::core::mem::transmute(&strlocale), ::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&pctx), ::core::mem::transmute_copy(&riid), ::core::mem::transmute(&presponsehandler)).into()
        }
        unsafe extern "system" fn Cancel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemClientConnectionTransport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, phandler: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Cancel(::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&phandler)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Open: Open::<Identity, Impl, OFFSET>,
            OpenAsync: OpenAsync::<Identity, Impl, OFFSET>,
            Cancel: Cancel::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWbemClientConnectionTransport as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWbemClientTransport_Impl: Sized {
    fn ConnectServer(&self, straddresstype: &super::super::Foundation::BSTR, dwbinaryaddresslength: u32, abbinaryaddress: *const u8, strnetworkresource: &super::super::Foundation::BSTR, struser: &super::super::Foundation::BSTR, strpassword: &super::super::Foundation::BSTR, strlocale: &super::super::Foundation::BSTR, lsecurityflags: i32, strauthority: &super::super::Foundation::BSTR, pctx: &::core::option::Option<IWbemContext>) -> ::windows::core::Result<IWbemServices>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IWbemClientTransport {}
#[cfg(feature = "Win32_Foundation")]
impl IWbemClientTransport_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemClientTransport_Impl, const OFFSET: isize>() -> IWbemClientTransport_Vtbl {
        unsafe extern "system" fn ConnectServer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemClientTransport_Impl, const OFFSET: isize>(
            this: *mut ::core::ffi::c_void,
            straddresstype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
            dwbinaryaddresslength: u32,
            abbinaryaddress: *const u8,
            strnetworkresource: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
            struser: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
            strpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
            strlocale: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
            lsecurityflags: i32,
            strauthority: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
            pctx: *mut ::core::ffi::c_void,
            ppnamespace: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ConnectServer(::core::mem::transmute(&straddresstype), ::core::mem::transmute_copy(&dwbinaryaddresslength), ::core::mem::transmute_copy(&abbinaryaddress), ::core::mem::transmute(&strnetworkresource), ::core::mem::transmute(&struser), ::core::mem::transmute(&strpassword), ::core::mem::transmute(&strlocale), ::core::mem::transmute_copy(&lsecurityflags), ::core::mem::transmute(&strauthority), ::core::mem::transmute(&pctx)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppnamespace, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), ConnectServer: ConnectServer::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWbemClientTransport as ::windows::core::Interface>::IID
    }
}
pub trait IWbemConfigureRefresher_Impl: Sized {
    fn AddObjectByPath(&self, pnamespace: &::core::option::Option<IWbemServices>, wszpath: &::windows::core::PCWSTR, lflags: i32, pcontext: &::core::option::Option<IWbemContext>, pprefreshable: *mut ::core::option::Option<IWbemClassObject>, plid: *mut i32) -> ::windows::core::Result<()>;
    fn AddObjectByTemplate(&self, pnamespace: &::core::option::Option<IWbemServices>, ptemplate: &::core::option::Option<IWbemClassObject>, lflags: i32, pcontext: &::core::option::Option<IWbemContext>, pprefreshable: *mut ::core::option::Option<IWbemClassObject>, plid: *mut i32) -> ::windows::core::Result<()>;
    fn AddRefresher(&self, prefresher: &::core::option::Option<IWbemRefresher>, lflags: i32, plid: *mut i32) -> ::windows::core::Result<()>;
    fn Remove(&self, lid: i32, lflags: i32) -> ::windows::core::Result<()>;
    fn AddEnum(&self, pnamespace: &::core::option::Option<IWbemServices>, wszclassname: &::windows::core::PCWSTR, lflags: i32, pcontext: &::core::option::Option<IWbemContext>, ppenum: *mut ::core::option::Option<IWbemHiPerfEnum>, plid: *mut i32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IWbemConfigureRefresher {}
impl IWbemConfigureRefresher_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemConfigureRefresher_Impl, const OFFSET: isize>() -> IWbemConfigureRefresher_Vtbl {
        unsafe extern "system" fn AddObjectByPath<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemConfigureRefresher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnamespace: *mut ::core::ffi::c_void, wszpath: ::windows::core::PCWSTR, lflags: i32, pcontext: *mut ::core::ffi::c_void, pprefreshable: *mut *mut ::core::ffi::c_void, plid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddObjectByPath(::core::mem::transmute(&pnamespace), ::core::mem::transmute(&wszpath), ::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&pcontext), ::core::mem::transmute_copy(&pprefreshable), ::core::mem::transmute_copy(&plid)).into()
        }
        unsafe extern "system" fn AddObjectByTemplate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemConfigureRefresher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnamespace: *mut ::core::ffi::c_void, ptemplate: *mut ::core::ffi::c_void, lflags: i32, pcontext: *mut ::core::ffi::c_void, pprefreshable: *mut *mut ::core::ffi::c_void, plid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddObjectByTemplate(::core::mem::transmute(&pnamespace), ::core::mem::transmute(&ptemplate), ::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&pcontext), ::core::mem::transmute_copy(&pprefreshable), ::core::mem::transmute_copy(&plid)).into()
        }
        unsafe extern "system" fn AddRefresher<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemConfigureRefresher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prefresher: *mut ::core::ffi::c_void, lflags: i32, plid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddRefresher(::core::mem::transmute(&prefresher), ::core::mem::transmute_copy(&lflags), ::core::mem::transmute_copy(&plid)).into()
        }
        unsafe extern "system" fn Remove<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemConfigureRefresher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lid: i32, lflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Remove(::core::mem::transmute_copy(&lid), ::core::mem::transmute_copy(&lflags)).into()
        }
        unsafe extern "system" fn AddEnum<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemConfigureRefresher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnamespace: *mut ::core::ffi::c_void, wszclassname: ::windows::core::PCWSTR, lflags: i32, pcontext: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void, plid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddEnum(::core::mem::transmute(&pnamespace), ::core::mem::transmute(&wszclassname), ::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&pcontext), ::core::mem::transmute_copy(&ppenum), ::core::mem::transmute_copy(&plid)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            AddObjectByPath: AddObjectByPath::<Identity, Impl, OFFSET>,
            AddObjectByTemplate: AddObjectByTemplate::<Identity, Impl, OFFSET>,
            AddRefresher: AddRefresher::<Identity, Impl, OFFSET>,
            Remove: Remove::<Identity, Impl, OFFSET>,
            AddEnum: AddEnum::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWbemConfigureRefresher as ::windows::core::Interface>::IID
    }
}
pub trait IWbemConnectorLogin_Impl: Sized {
    fn ConnectorLogin(&self, wsznetworkresource: &::windows::core::PCWSTR, wszpreferredlocale: &::windows::core::PCWSTR, lflags: i32, pctx: &::core::option::Option<IWbemContext>, riid: *const ::windows::core::GUID, pinterface: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IWbemConnectorLogin {}
impl IWbemConnectorLogin_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemConnectorLogin_Impl, const OFFSET: isize>() -> IWbemConnectorLogin_Vtbl {
        unsafe extern "system" fn ConnectorLogin<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemConnectorLogin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wsznetworkresource: ::windows::core::PCWSTR, wszpreferredlocale: ::windows::core::PCWSTR, lflags: i32, pctx: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, pinterface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ConnectorLogin(::core::mem::transmute(&wsznetworkresource), ::core::mem::transmute(&wszpreferredlocale), ::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&pctx), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&pinterface)).into()
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), ConnectorLogin: ConnectorLogin::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWbemConnectorLogin as ::windows::core::Interface>::IID
    }
}
pub trait IWbemConstructClassObject_Impl: Sized {
    fn SetInheritanceChain(&self, lnumantecedents: i32, awszantecedents: *const ::windows::core::PWSTR) -> ::windows::core::Result<()>;
    fn SetPropertyOrigin(&self, wszpropertyname: &::windows::core::PCWSTR, loriginindex: i32) -> ::windows::core::Result<()>;
    fn SetMethodOrigin(&self, wszmethodname: &::windows::core::PCWSTR, loriginindex: i32) -> ::windows::core::Result<()>;
    fn SetServerNamespace(&self, wszserver: &::windows::core::PCWSTR, wsznamespace: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IWbemConstructClassObject {}
impl IWbemConstructClassObject_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemConstructClassObject_Impl, const OFFSET: isize>() -> IWbemConstructClassObject_Vtbl {
        unsafe extern "system" fn SetInheritanceChain<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemConstructClassObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnumantecedents: i32, awszantecedents: *const ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetInheritanceChain(::core::mem::transmute_copy(&lnumantecedents), ::core::mem::transmute_copy(&awszantecedents)).into()
        }
        unsafe extern "system" fn SetPropertyOrigin<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemConstructClassObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszpropertyname: ::windows::core::PCWSTR, loriginindex: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPropertyOrigin(::core::mem::transmute(&wszpropertyname), ::core::mem::transmute_copy(&loriginindex)).into()
        }
        unsafe extern "system" fn SetMethodOrigin<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemConstructClassObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszmethodname: ::windows::core::PCWSTR, loriginindex: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMethodOrigin(::core::mem::transmute(&wszmethodname), ::core::mem::transmute_copy(&loriginindex)).into()
        }
        unsafe extern "system" fn SetServerNamespace<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemConstructClassObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszserver: ::windows::core::PCWSTR, wsznamespace: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetServerNamespace(::core::mem::transmute(&wszserver), ::core::mem::transmute(&wsznamespace)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SetInheritanceChain: SetInheritanceChain::<Identity, Impl, OFFSET>,
            SetPropertyOrigin: SetPropertyOrigin::<Identity, Impl, OFFSET>,
            SetMethodOrigin: SetMethodOrigin::<Identity, Impl, OFFSET>,
            SetServerNamespace: SetServerNamespace::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWbemConstructClassObject as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWbemContext_Impl: Sized {
    fn Clone(&self) -> ::windows::core::Result<IWbemContext>;
    fn GetNames(&self, lflags: i32) -> ::windows::core::Result<*mut super::Com::SAFEARRAY>;
    fn BeginEnumeration(&self, lflags: i32) -> ::windows::core::Result<()>;
    fn Next(&self, lflags: i32, pstrname: *mut super::super::Foundation::BSTR, pvalue: *mut super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn EndEnumeration(&self) -> ::windows::core::Result<()>;
    fn SetValue(&self, wszname: &::windows::core::PCWSTR, lflags: i32, pvalue: *const super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn GetValue(&self, wszname: &::windows::core::PCWSTR, lflags: i32) -> ::windows::core::Result<super::Com::VARIANT>;
    fn DeleteValue(&self, wszname: &::windows::core::PCWSTR, lflags: i32) -> ::windows::core::Result<()>;
    fn DeleteAll(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IWbemContext {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWbemContext_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemContext_Impl, const OFFSET: isize>() -> IWbemContext_Vtbl {
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppnewcopy: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppnewcopy, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNames<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, pnames: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetNames(::core::mem::transmute_copy(&lflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pnames, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginEnumeration<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.BeginEnumeration(::core::mem::transmute_copy(&lflags)).into()
        }
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, pstrname: *mut super::super::Foundation::BSTR, pvalue: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Next(::core::mem::transmute_copy(&lflags), ::core::mem::transmute_copy(&pstrname), ::core::mem::transmute_copy(&pvalue)).into()
        }
        unsafe extern "system" fn EndEnumeration<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndEnumeration().into()
        }
        unsafe extern "system" fn SetValue<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszname: ::windows::core::PCWSTR, lflags: i32, pvalue: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetValue(::core::mem::transmute(&wszname), ::core::mem::transmute_copy(&lflags), ::core::mem::transmute_copy(&pvalue)).into()
        }
        unsafe extern "system" fn GetValue<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszname: ::windows::core::PCWSTR, lflags: i32, pvalue: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetValue(::core::mem::transmute(&wszname), ::core::mem::transmute_copy(&lflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvalue, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteValue<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszname: ::windows::core::PCWSTR, lflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteValue(::core::mem::transmute(&wszname), ::core::mem::transmute_copy(&lflags)).into()
        }
        unsafe extern "system" fn DeleteAll<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteAll().into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Clone: Clone::<Identity, Impl, OFFSET>,
            GetNames: GetNames::<Identity, Impl, OFFSET>,
            BeginEnumeration: BeginEnumeration::<Identity, Impl, OFFSET>,
            Next: Next::<Identity, Impl, OFFSET>,
            EndEnumeration: EndEnumeration::<Identity, Impl, OFFSET>,
            SetValue: SetValue::<Identity, Impl, OFFSET>,
            GetValue: GetValue::<Identity, Impl, OFFSET>,
            DeleteValue: DeleteValue::<Identity, Impl, OFFSET>,
            DeleteAll: DeleteAll::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWbemContext as ::windows::core::Interface>::IID
    }
}
pub trait IWbemDecoupledBasicEventProvider_Impl: Sized + IWbemDecoupledRegistrar_Impl {
    fn GetSink(&self, a_flags: i32, a_context: &::core::option::Option<IWbemContext>) -> ::windows::core::Result<IWbemObjectSink>;
    fn GetService(&self, a_flags: i32, a_context: &::core::option::Option<IWbemContext>) -> ::windows::core::Result<IWbemServices>;
}
impl ::windows::core::RuntimeName for IWbemDecoupledBasicEventProvider {}
impl IWbemDecoupledBasicEventProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemDecoupledBasicEventProvider_Impl, const OFFSET: isize>() -> IWbemDecoupledBasicEventProvider_Vtbl {
        unsafe extern "system" fn GetSink<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemDecoupledBasicEventProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, a_flags: i32, a_context: *mut ::core::ffi::c_void, a_sink: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSink(::core::mem::transmute_copy(&a_flags), ::core::mem::transmute(&a_context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(a_sink, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetService<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemDecoupledBasicEventProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, a_flags: i32, a_context: *mut ::core::ffi::c_void, a_service: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetService(::core::mem::transmute_copy(&a_flags), ::core::mem::transmute(&a_context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(a_service, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IWbemDecoupledRegistrar_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetSink: GetSink::<Identity, Impl, OFFSET>,
            GetService: GetService::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWbemDecoupledBasicEventProvider as ::windows::core::Interface>::IID || iid == &<IWbemDecoupledRegistrar as ::windows::core::Interface>::IID
    }
}
pub trait IWbemDecoupledRegistrar_Impl: Sized {
    fn Register(&self, a_flags: i32, a_context: &::core::option::Option<IWbemContext>, a_user: &::windows::core::PCWSTR, a_locale: &::windows::core::PCWSTR, a_scope: &::windows::core::PCWSTR, a_registration: &::windows::core::PCWSTR, piunknown: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn UnRegister(&self) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IWbemDecoupledRegistrar {}
impl IWbemDecoupledRegistrar_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemDecoupledRegistrar_Impl, const OFFSET: isize>() -> IWbemDecoupledRegistrar_Vtbl {
        unsafe extern "system" fn Register<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemDecoupledRegistrar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, a_flags: i32, a_context: *mut ::core::ffi::c_void, a_user: ::windows::core::PCWSTR, a_locale: ::windows::core::PCWSTR, a_scope: ::windows::core::PCWSTR, a_registration: ::windows::core::PCWSTR, piunknown: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Register(::core::mem::transmute_copy(&a_flags), ::core::mem::transmute(&a_context), ::core::mem::transmute(&a_user), ::core::mem::transmute(&a_locale), ::core::mem::transmute(&a_scope), ::core::mem::transmute(&a_registration), ::core::mem::transmute(&piunknown)).into()
        }
        unsafe extern "system" fn UnRegister<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemDecoupledRegistrar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UnRegister().into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Register: Register::<Identity, Impl, OFFSET>,
            UnRegister: UnRegister::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWbemDecoupledRegistrar as ::windows::core::Interface>::IID
    }
}
pub trait IWbemEventConsumerProvider_Impl: Sized {
    fn FindConsumer(&self, plogicalconsumer: &::core::option::Option<IWbemClassObject>) -> ::windows::core::Result<IWbemUnboundObjectSink>;
}
impl ::windows::core::RuntimeName for IWbemEventConsumerProvider {}
impl IWbemEventConsumerProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemEventConsumerProvider_Impl, const OFFSET: isize>() -> IWbemEventConsumerProvider_Vtbl {
        unsafe extern "system" fn FindConsumer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemEventConsumerProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plogicalconsumer: *mut ::core::ffi::c_void, ppconsumer: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FindConsumer(::core::mem::transmute(&plogicalconsumer)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppconsumer, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), FindConsumer: FindConsumer::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWbemEventConsumerProvider as ::windows::core::Interface>::IID
    }
}
pub trait IWbemEventProvider_Impl: Sized {
    fn ProvideEvents(&self, psink: &::core::option::Option<IWbemObjectSink>, lflags: i32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IWbemEventProvider {}
impl IWbemEventProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemEventProvider_Impl, const OFFSET: isize>() -> IWbemEventProvider_Vtbl {
        unsafe extern "system" fn ProvideEvents<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemEventProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psink: *mut ::core::ffi::c_void, lflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ProvideEvents(::core::mem::transmute(&psink), ::core::mem::transmute_copy(&lflags)).into()
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), ProvideEvents: ProvideEvents::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWbemEventProvider as ::windows::core::Interface>::IID
    }
}
pub trait IWbemEventProviderQuerySink_Impl: Sized {
    fn NewQuery(&self, dwid: u32, wszquerylanguage: *const u16, wszquery: *const u16) -> ::windows::core::Result<()>;
    fn CancelQuery(&self, dwid: u32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IWbemEventProviderQuerySink {}
impl IWbemEventProviderQuerySink_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemEventProviderQuerySink_Impl, const OFFSET: isize>() -> IWbemEventProviderQuerySink_Vtbl {
        unsafe extern "system" fn NewQuery<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemEventProviderQuerySink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwid: u32, wszquerylanguage: *const u16, wszquery: *const u16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.NewQuery(::core::mem::transmute_copy(&dwid), ::core::mem::transmute_copy(&wszquerylanguage), ::core::mem::transmute_copy(&wszquery)).into()
        }
        unsafe extern "system" fn CancelQuery<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemEventProviderQuerySink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwid: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CancelQuery(::core::mem::transmute_copy(&dwid)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            NewQuery: NewQuery::<Identity, Impl, OFFSET>,
            CancelQuery: CancelQuery::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWbemEventProviderQuerySink as ::windows::core::Interface>::IID
    }
}
pub trait IWbemEventProviderSecurity_Impl: Sized {
    fn AccessCheck(&self, wszquerylanguage: *const u16, wszquery: *const u16, lsidlength: i32, psid: *const u8) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IWbemEventProviderSecurity {}
impl IWbemEventProviderSecurity_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemEventProviderSecurity_Impl, const OFFSET: isize>() -> IWbemEventProviderSecurity_Vtbl {
        unsafe extern "system" fn AccessCheck<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemEventProviderSecurity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszquerylanguage: *const u16, wszquery: *const u16, lsidlength: i32, psid: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AccessCheck(::core::mem::transmute_copy(&wszquerylanguage), ::core::mem::transmute_copy(&wszquery), ::core::mem::transmute_copy(&lsidlength), ::core::mem::transmute_copy(&psid)).into()
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), AccessCheck: AccessCheck::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWbemEventProviderSecurity as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWbemEventSink_Impl: Sized + IWbemObjectSink_Impl {
    fn SetSinkSecurity(&self, lsdlength: i32, psd: *const u8) -> ::windows::core::Result<()>;
    fn IsActive(&self) -> ::windows::core::Result<()>;
    fn GetRestrictedSink(&self, lnumqueries: i32, awszqueries: *const ::windows::core::PWSTR, pcallback: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<IWbemEventSink>;
    fn SetBatchingParameters(&self, lflags: i32, dwmaxbuffersize: u32, dwmaxsendlatency: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IWbemEventSink {}
#[cfg(feature = "Win32_Foundation")]
impl IWbemEventSink_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemEventSink_Impl, const OFFSET: isize>() -> IWbemEventSink_Vtbl {
        unsafe extern "system" fn SetSinkSecurity<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemEventSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lsdlength: i32, psd: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSinkSecurity(::core::mem::transmute_copy(&lsdlength), ::core::mem::transmute_copy(&psd)).into()
        }
        unsafe extern "system" fn IsActive<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemEventSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsActive().into()
        }
        unsafe extern "system" fn GetRestrictedSink<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemEventSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnumqueries: i32, awszqueries: *const ::windows::core::PWSTR, pcallback: *mut ::core::ffi::c_void, ppsink: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetRestrictedSink(::core::mem::transmute_copy(&lnumqueries), ::core::mem::transmute_copy(&awszqueries), ::core::mem::transmute(&pcallback)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsink, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBatchingParameters<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemEventSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, dwmaxbuffersize: u32, dwmaxsendlatency: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetBatchingParameters(::core::mem::transmute_copy(&lflags), ::core::mem::transmute_copy(&dwmaxbuffersize), ::core::mem::transmute_copy(&dwmaxsendlatency)).into()
        }
        Self {
            base__: IWbemObjectSink_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetSinkSecurity: SetSinkSecurity::<Identity, Impl, OFFSET>,
            IsActive: IsActive::<Identity, Impl, OFFSET>,
            GetRestrictedSink: GetRestrictedSink::<Identity, Impl, OFFSET>,
            SetBatchingParameters: SetBatchingParameters::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWbemEventSink as ::windows::core::Interface>::IID || iid == &<IWbemObjectSink as ::windows::core::Interface>::IID
    }
}
pub trait IWbemHiPerfEnum_Impl: Sized {
    fn AddObjects(&self, lflags: i32, unumobjects: u32, apids: *const i32, apobj: *const ::core::option::Option<IWbemObjectAccess>) -> ::windows::core::Result<()>;
    fn RemoveObjects(&self, lflags: i32, unumobjects: u32, apids: *const i32) -> ::windows::core::Result<()>;
    fn GetObjects(&self, lflags: i32, unumobjects: u32, apobj: *mut ::core::option::Option<IWbemObjectAccess>, pureturned: *mut u32) -> ::windows::core::Result<()>;
    fn RemoveAll(&self, lflags: i32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IWbemHiPerfEnum {}
impl IWbemHiPerfEnum_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemHiPerfEnum_Impl, const OFFSET: isize>() -> IWbemHiPerfEnum_Vtbl {
        unsafe extern "system" fn AddObjects<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemHiPerfEnum_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, unumobjects: u32, apids: *const i32, apobj: *const *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddObjects(::core::mem::transmute_copy(&lflags), ::core::mem::transmute_copy(&unumobjects), ::core::mem::transmute_copy(&apids), ::core::mem::transmute_copy(&apobj)).into()
        }
        unsafe extern "system" fn RemoveObjects<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemHiPerfEnum_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, unumobjects: u32, apids: *const i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveObjects(::core::mem::transmute_copy(&lflags), ::core::mem::transmute_copy(&unumobjects), ::core::mem::transmute_copy(&apids)).into()
        }
        unsafe extern "system" fn GetObjects<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemHiPerfEnum_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, unumobjects: u32, apobj: *mut *mut ::core::ffi::c_void, pureturned: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetObjects(::core::mem::transmute_copy(&lflags), ::core::mem::transmute_copy(&unumobjects), ::core::mem::transmute_copy(&apobj), ::core::mem::transmute_copy(&pureturned)).into()
        }
        unsafe extern "system" fn RemoveAll<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemHiPerfEnum_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveAll(::core::mem::transmute_copy(&lflags)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            AddObjects: AddObjects::<Identity, Impl, OFFSET>,
            RemoveObjects: RemoveObjects::<Identity, Impl, OFFSET>,
            GetObjects: GetObjects::<Identity, Impl, OFFSET>,
            RemoveAll: RemoveAll::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWbemHiPerfEnum as ::windows::core::Interface>::IID
    }
}
pub trait IWbemHiPerfProvider_Impl: Sized {
    fn QueryInstances(&self, pnamespace: &::core::option::Option<IWbemServices>, wszclass: &::windows::core::PCWSTR, lflags: i32, pctx: &::core::option::Option<IWbemContext>, psink: &::core::option::Option<IWbemObjectSink>) -> ::windows::core::Result<()>;
    fn CreateRefresher(&self, pnamespace: &::core::option::Option<IWbemServices>, lflags: i32) -> ::windows::core::Result<IWbemRefresher>;
    fn CreateRefreshableObject(&self, pnamespace: &::core::option::Option<IWbemServices>, ptemplate: &::core::option::Option<IWbemObjectAccess>, prefresher: &::core::option::Option<IWbemRefresher>, lflags: i32, pcontext: &::core::option::Option<IWbemContext>, pprefreshable: *mut ::core::option::Option<IWbemObjectAccess>, plid: *mut i32) -> ::windows::core::Result<()>;
    fn StopRefreshing(&self, prefresher: &::core::option::Option<IWbemRefresher>, lid: i32, lflags: i32) -> ::windows::core::Result<()>;
    fn CreateRefreshableEnum(&self, pnamespace: &::core::option::Option<IWbemServices>, wszclass: &::windows::core::PCWSTR, prefresher: &::core::option::Option<IWbemRefresher>, lflags: i32, pcontext: &::core::option::Option<IWbemContext>, phiperfenum: &::core::option::Option<IWbemHiPerfEnum>) -> ::windows::core::Result<i32>;
    fn GetObjects(&self, pnamespace: &::core::option::Option<IWbemServices>, lnumobjects: i32, apobj: *mut ::core::option::Option<IWbemObjectAccess>, lflags: i32, pcontext: &::core::option::Option<IWbemContext>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IWbemHiPerfProvider {}
impl IWbemHiPerfProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemHiPerfProvider_Impl, const OFFSET: isize>() -> IWbemHiPerfProvider_Vtbl {
        unsafe extern "system" fn QueryInstances<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemHiPerfProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnamespace: *mut ::core::ffi::c_void, wszclass: ::windows::core::PCWSTR, lflags: i32, pctx: *mut ::core::ffi::c_void, psink: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.QueryInstances(::core::mem::transmute(&pnamespace), ::core::mem::transmute(&wszclass), ::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&pctx), ::core::mem::transmute(&psink)).into()
        }
        unsafe extern "system" fn CreateRefresher<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemHiPerfProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnamespace: *mut ::core::ffi::c_void, lflags: i32, pprefresher: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateRefresher(::core::mem::transmute(&pnamespace), ::core::mem::transmute_copy(&lflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pprefresher, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRefreshableObject<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemHiPerfProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnamespace: *mut ::core::ffi::c_void, ptemplate: *mut ::core::ffi::c_void, prefresher: *mut ::core::ffi::c_void, lflags: i32, pcontext: *mut ::core::ffi::c_void, pprefreshable: *mut *mut ::core::ffi::c_void, plid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateRefreshableObject(::core::mem::transmute(&pnamespace), ::core::mem::transmute(&ptemplate), ::core::mem::transmute(&prefresher), ::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&pcontext), ::core::mem::transmute_copy(&pprefreshable), ::core::mem::transmute_copy(&plid)).into()
        }
        unsafe extern "system" fn StopRefreshing<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemHiPerfProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prefresher: *mut ::core::ffi::c_void, lid: i32, lflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.StopRefreshing(::core::mem::transmute(&prefresher), ::core::mem::transmute_copy(&lid), ::core::mem::transmute_copy(&lflags)).into()
        }
        unsafe extern "system" fn CreateRefreshableEnum<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemHiPerfProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnamespace: *mut ::core::ffi::c_void, wszclass: ::windows::core::PCWSTR, prefresher: *mut ::core::ffi::c_void, lflags: i32, pcontext: *mut ::core::ffi::c_void, phiperfenum: *mut ::core::ffi::c_void, plid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateRefreshableEnum(::core::mem::transmute(&pnamespace), ::core::mem::transmute(&wszclass), ::core::mem::transmute(&prefresher), ::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&pcontext), ::core::mem::transmute(&phiperfenum)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetObjects<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemHiPerfProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnamespace: *mut ::core::ffi::c_void, lnumobjects: i32, apobj: *mut *mut ::core::ffi::c_void, lflags: i32, pcontext: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetObjects(::core::mem::transmute(&pnamespace), ::core::mem::transmute_copy(&lnumobjects), ::core::mem::transmute_copy(&apobj), ::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&pcontext)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            QueryInstances: QueryInstances::<Identity, Impl, OFFSET>,
            CreateRefresher: CreateRefresher::<Identity, Impl, OFFSET>,
            CreateRefreshableObject: CreateRefreshableObject::<Identity, Impl, OFFSET>,
            StopRefreshing: StopRefreshing::<Identity, Impl, OFFSET>,
            CreateRefreshableEnum: CreateRefreshableEnum::<Identity, Impl, OFFSET>,
            GetObjects: GetObjects::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWbemHiPerfProvider as ::windows::core::Interface>::IID
    }
}
pub trait IWbemLevel1Login_Impl: Sized {
    fn EstablishPosition(&self, wszlocalelist: &::windows::core::PCWSTR, dwnumlocales: u32) -> ::windows::core::Result<u32>;
    fn RequestChallenge(&self, wsznetworkresource: &::windows::core::PCWSTR, wszuser: &::windows::core::PCWSTR) -> ::windows::core::Result<u8>;
    fn WBEMLogin(&self, wszpreferredlocale: &::windows::core::PCWSTR, accesstoken: *const u8, lflags: i32, pctx: &::core::option::Option<IWbemContext>) -> ::windows::core::Result<IWbemServices>;
    fn NTLMLogin(&self, wsznetworkresource: &::windows::core::PCWSTR, wszpreferredlocale: &::windows::core::PCWSTR, lflags: i32, pctx: &::core::option::Option<IWbemContext>) -> ::windows::core::Result<IWbemServices>;
}
impl ::windows::core::RuntimeName for IWbemLevel1Login {}
impl IWbemLevel1Login_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemLevel1Login_Impl, const OFFSET: isize>() -> IWbemLevel1Login_Vtbl {
        unsafe extern "system" fn EstablishPosition<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemLevel1Login_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszlocalelist: ::windows::core::PCWSTR, dwnumlocales: u32, reserved: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EstablishPosition(::core::mem::transmute(&wszlocalelist), ::core::mem::transmute_copy(&dwnumlocales)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(reserved, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestChallenge<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemLevel1Login_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wsznetworkresource: ::windows::core::PCWSTR, wszuser: ::windows::core::PCWSTR, nonce: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RequestChallenge(::core::mem::transmute(&wsznetworkresource), ::core::mem::transmute(&wszuser)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(nonce, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WBEMLogin<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemLevel1Login_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszpreferredlocale: ::windows::core::PCWSTR, accesstoken: *const u8, lflags: i32, pctx: *mut ::core::ffi::c_void, ppnamespace: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.WBEMLogin(::core::mem::transmute(&wszpreferredlocale), ::core::mem::transmute_copy(&accesstoken), ::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&pctx)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppnamespace, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NTLMLogin<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemLevel1Login_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wsznetworkresource: ::windows::core::PCWSTR, wszpreferredlocale: ::windows::core::PCWSTR, lflags: i32, pctx: *mut ::core::ffi::c_void, ppnamespace: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.NTLMLogin(::core::mem::transmute(&wsznetworkresource), ::core::mem::transmute(&wszpreferredlocale), ::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&pctx)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppnamespace, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            EstablishPosition: EstablishPosition::<Identity, Impl, OFFSET>,
            RequestChallenge: RequestChallenge::<Identity, Impl, OFFSET>,
            WBEMLogin: WBEMLogin::<Identity, Impl, OFFSET>,
            NTLMLogin: NTLMLogin::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWbemLevel1Login as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWbemLocator_Impl: Sized {
    fn ConnectServer(&self, strnetworkresource: &super::super::Foundation::BSTR, struser: &super::super::Foundation::BSTR, strpassword: &super::super::Foundation::BSTR, strlocale: &super::super::Foundation::BSTR, lsecurityflags: i32, strauthority: &super::super::Foundation::BSTR, pctx: &::core::option::Option<IWbemContext>) -> ::windows::core::Result<IWbemServices>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IWbemLocator {}
#[cfg(feature = "Win32_Foundation")]
impl IWbemLocator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemLocator_Impl, const OFFSET: isize>() -> IWbemLocator_Vtbl {
        unsafe extern "system" fn ConnectServer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemLocator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strnetworkresource: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, struser: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strlocale: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lsecurityflags: i32, strauthority: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pctx: *mut ::core::ffi::c_void, ppnamespace: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ConnectServer(::core::mem::transmute(&strnetworkresource), ::core::mem::transmute(&struser), ::core::mem::transmute(&strpassword), ::core::mem::transmute(&strlocale), ::core::mem::transmute_copy(&lsecurityflags), ::core::mem::transmute(&strauthority), ::core::mem::transmute(&pctx)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppnamespace, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), ConnectServer: ConnectServer::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWbemLocator as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWbemObjectAccess_Impl: Sized + IWbemClassObject_Impl {
    fn GetPropertyHandle(&self, wszpropertyname: &::windows::core::PCWSTR, ptype: *mut i32, plhandle: *mut i32) -> ::windows::core::Result<()>;
    fn WritePropertyValue(&self, lhandle: i32, lnumbytes: i32, adata: *const u8) -> ::windows::core::Result<()>;
    fn ReadPropertyValue(&self, lhandle: i32, lbuffersize: i32, plnumbytes: *mut i32, adata: *mut u8) -> ::windows::core::Result<()>;
    fn ReadDWORD(&self, lhandle: i32) -> ::windows::core::Result<u32>;
    fn WriteDWORD(&self, lhandle: i32, dw: u32) -> ::windows::core::Result<()>;
    fn ReadQWORD(&self, lhandle: i32) -> ::windows::core::Result<u64>;
    fn WriteQWORD(&self, lhandle: i32, pw: u64) -> ::windows::core::Result<()>;
    fn GetPropertyInfoByHandle(&self, lhandle: i32, pstrname: *mut super::super::Foundation::BSTR, ptype: *mut i32) -> ::windows::core::Result<()>;
    fn Lock(&self, lflags: i32) -> ::windows::core::Result<()>;
    fn Unlock(&self, lflags: i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IWbemObjectAccess {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWbemObjectAccess_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemObjectAccess_Impl, const OFFSET: isize>() -> IWbemObjectAccess_Vtbl {
        unsafe extern "system" fn GetPropertyHandle<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemObjectAccess_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszpropertyname: ::windows::core::PCWSTR, ptype: *mut i32, plhandle: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetPropertyHandle(::core::mem::transmute(&wszpropertyname), ::core::mem::transmute_copy(&ptype), ::core::mem::transmute_copy(&plhandle)).into()
        }
        unsafe extern "system" fn WritePropertyValue<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemObjectAccess_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lhandle: i32, lnumbytes: i32, adata: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.WritePropertyValue(::core::mem::transmute_copy(&lhandle), ::core::mem::transmute_copy(&lnumbytes), ::core::mem::transmute_copy(&adata)).into()
        }
        unsafe extern "system" fn ReadPropertyValue<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemObjectAccess_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lhandle: i32, lbuffersize: i32, plnumbytes: *mut i32, adata: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReadPropertyValue(::core::mem::transmute_copy(&lhandle), ::core::mem::transmute_copy(&lbuffersize), ::core::mem::transmute_copy(&plnumbytes), ::core::mem::transmute_copy(&adata)).into()
        }
        unsafe extern "system" fn ReadDWORD<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemObjectAccess_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lhandle: i32, pdw: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ReadDWORD(::core::mem::transmute_copy(&lhandle)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdw, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WriteDWORD<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemObjectAccess_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lhandle: i32, dw: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.WriteDWORD(::core::mem::transmute_copy(&lhandle), ::core::mem::transmute_copy(&dw)).into()
        }
        unsafe extern "system" fn ReadQWORD<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemObjectAccess_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lhandle: i32, pqw: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ReadQWORD(::core::mem::transmute_copy(&lhandle)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pqw, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WriteQWORD<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemObjectAccess_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lhandle: i32, pw: u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.WriteQWORD(::core::mem::transmute_copy(&lhandle), ::core::mem::transmute_copy(&pw)).into()
        }
        unsafe extern "system" fn GetPropertyInfoByHandle<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemObjectAccess_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lhandle: i32, pstrname: *mut super::super::Foundation::BSTR, ptype: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetPropertyInfoByHandle(::core::mem::transmute_copy(&lhandle), ::core::mem::transmute_copy(&pstrname), ::core::mem::transmute_copy(&ptype)).into()
        }
        unsafe extern "system" fn Lock<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemObjectAccess_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Lock(::core::mem::transmute_copy(&lflags)).into()
        }
        unsafe extern "system" fn Unlock<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemObjectAccess_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Unlock(::core::mem::transmute_copy(&lflags)).into()
        }
        Self {
            base__: IWbemClassObject_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetPropertyHandle: GetPropertyHandle::<Identity, Impl, OFFSET>,
            WritePropertyValue: WritePropertyValue::<Identity, Impl, OFFSET>,
            ReadPropertyValue: ReadPropertyValue::<Identity, Impl, OFFSET>,
            ReadDWORD: ReadDWORD::<Identity, Impl, OFFSET>,
            WriteDWORD: WriteDWORD::<Identity, Impl, OFFSET>,
            ReadQWORD: ReadQWORD::<Identity, Impl, OFFSET>,
            WriteQWORD: WriteQWORD::<Identity, Impl, OFFSET>,
            GetPropertyInfoByHandle: GetPropertyInfoByHandle::<Identity, Impl, OFFSET>,
            Lock: Lock::<Identity, Impl, OFFSET>,
            Unlock: Unlock::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWbemObjectAccess as ::windows::core::Interface>::IID || iid == &<IWbemClassObject as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWbemObjectSink_Impl: Sized {
    fn Indicate(&self, lobjectcount: i32, apobjarray: *const ::core::option::Option<IWbemClassObject>) -> ::windows::core::Result<()>;
    fn SetStatus(&self, lflags: i32, hresult: ::windows::core::HRESULT, strparam: &super::super::Foundation::BSTR, pobjparam: &::core::option::Option<IWbemClassObject>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IWbemObjectSink {}
#[cfg(feature = "Win32_Foundation")]
impl IWbemObjectSink_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemObjectSink_Impl, const OFFSET: isize>() -> IWbemObjectSink_Vtbl {
        unsafe extern "system" fn Indicate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemObjectSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lobjectcount: i32, apobjarray: *const *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Indicate(::core::mem::transmute_copy(&lobjectcount), ::core::mem::transmute_copy(&apobjarray)).into()
        }
        unsafe extern "system" fn SetStatus<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemObjectSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, hresult: ::windows::core::HRESULT, strparam: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pobjparam: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetStatus(::core::mem::transmute_copy(&lflags), ::core::mem::transmute_copy(&hresult), ::core::mem::transmute(&strparam), ::core::mem::transmute(&pobjparam)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Indicate: Indicate::<Identity, Impl, OFFSET>,
            SetStatus: SetStatus::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWbemObjectSink as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWbemObjectSinkEx_Impl: Sized + IWbemObjectSink_Impl {
    fn WriteMessage(&self, uchannel: u32, strmessage: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn WriteError(&self, pobjerror: &::core::option::Option<IWbemClassObject>) -> ::windows::core::Result<u8>;
    fn PromptUser(&self, strmessage: &super::super::Foundation::BSTR, uprompttype: u8) -> ::windows::core::Result<u8>;
    fn WriteProgress(&self, stractivity: &super::super::Foundation::BSTR, strcurrentoperation: &super::super::Foundation::BSTR, strstatusdescription: &super::super::Foundation::BSTR, upercentcomplete: u32, usecondsremaining: u32) -> ::windows::core::Result<()>;
    fn WriteStreamParameter(&self, strname: &super::super::Foundation::BSTR, vtvalue: *const super::Com::VARIANT, ultype: u32, ulflags: u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IWbemObjectSinkEx {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWbemObjectSinkEx_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemObjectSinkEx_Impl, const OFFSET: isize>() -> IWbemObjectSinkEx_Vtbl {
        unsafe extern "system" fn WriteMessage<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemObjectSinkEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uchannel: u32, strmessage: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.WriteMessage(::core::mem::transmute_copy(&uchannel), ::core::mem::transmute(&strmessage)).into()
        }
        unsafe extern "system" fn WriteError<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemObjectSinkEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pobjerror: *mut ::core::ffi::c_void, pureturned: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.WriteError(::core::mem::transmute(&pobjerror)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pureturned, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PromptUser<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemObjectSinkEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strmessage: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, uprompttype: u8, pureturned: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PromptUser(::core::mem::transmute(&strmessage), ::core::mem::transmute_copy(&uprompttype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pureturned, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WriteProgress<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemObjectSinkEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stractivity: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strcurrentoperation: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strstatusdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, upercentcomplete: u32, usecondsremaining: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.WriteProgress(::core::mem::transmute(&stractivity), ::core::mem::transmute(&strcurrentoperation), ::core::mem::transmute(&strstatusdescription), ::core::mem::transmute_copy(&upercentcomplete), ::core::mem::transmute_copy(&usecondsremaining)).into()
        }
        unsafe extern "system" fn WriteStreamParameter<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemObjectSinkEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, vtvalue: *const super::Com::VARIANT, ultype: u32, ulflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.WriteStreamParameter(::core::mem::transmute(&strname), ::core::mem::transmute_copy(&vtvalue), ::core::mem::transmute_copy(&ultype), ::core::mem::transmute_copy(&ulflags)).into()
        }
        Self {
            base__: IWbemObjectSink_Vtbl::new::<Identity, Impl, OFFSET>(),
            WriteMessage: WriteMessage::<Identity, Impl, OFFSET>,
            WriteError: WriteError::<Identity, Impl, OFFSET>,
            PromptUser: PromptUser::<Identity, Impl, OFFSET>,
            WriteProgress: WriteProgress::<Identity, Impl, OFFSET>,
            WriteStreamParameter: WriteStreamParameter::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWbemObjectSinkEx as ::windows::core::Interface>::IID || iid == &<IWbemObjectSink as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWbemObjectTextSrc_Impl: Sized {
    fn GetText(&self, lflags: i32, pobj: &::core::option::Option<IWbemClassObject>, uobjtextformat: u32, pctx: &::core::option::Option<IWbemContext>) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn CreateFromText(&self, lflags: i32, strtext: &super::super::Foundation::BSTR, uobjtextformat: u32, pctx: &::core::option::Option<IWbemContext>) -> ::windows::core::Result<IWbemClassObject>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IWbemObjectTextSrc {}
#[cfg(feature = "Win32_Foundation")]
impl IWbemObjectTextSrc_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemObjectTextSrc_Impl, const OFFSET: isize>() -> IWbemObjectTextSrc_Vtbl {
        unsafe extern "system" fn GetText<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemObjectTextSrc_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, pobj: *mut ::core::ffi::c_void, uobjtextformat: u32, pctx: *mut ::core::ffi::c_void, strtext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetText(::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&pobj), ::core::mem::transmute_copy(&uobjtextformat), ::core::mem::transmute(&pctx)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(strtext, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromText<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemObjectTextSrc_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, strtext: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, uobjtextformat: u32, pctx: *mut ::core::ffi::c_void, pnewobj: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateFromText(::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&strtext), ::core::mem::transmute_copy(&uobjtextformat), ::core::mem::transmute(&pctx)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pnewobj, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetText: GetText::<Identity, Impl, OFFSET>,
            CreateFromText: CreateFromText::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWbemObjectTextSrc as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWbemPath_Impl: Sized {
    fn SetText(&self, umode: u32, pszpath: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn GetText(&self, lflags: i32, pubufflength: *mut u32, psztext: ::windows::core::PWSTR) -> ::windows::core::Result<()>;
    fn GetInfo(&self, urequestedinfo: u32) -> ::windows::core::Result<u64>;
    fn SetServer(&self, name: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn GetServer(&self, punamebuflength: *mut u32, pname: ::windows::core::PWSTR) -> ::windows::core::Result<()>;
    fn GetNamespaceCount(&self) -> ::windows::core::Result<u32>;
    fn SetNamespaceAt(&self, uindex: u32, pszname: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn GetNamespaceAt(&self, uindex: u32, punamebuflength: *mut u32, pname: ::windows::core::PWSTR) -> ::windows::core::Result<()>;
    fn RemoveNamespaceAt(&self, uindex: u32) -> ::windows::core::Result<()>;
    fn RemoveAllNamespaces(&self) -> ::windows::core::Result<()>;
    fn GetScopeCount(&self) -> ::windows::core::Result<u32>;
    fn SetScope(&self, uindex: u32, pszclass: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn SetScopeFromText(&self, uindex: u32, psztext: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn GetScope(&self, uindex: u32, puclassnamebufsize: *mut u32, pszclass: ::windows::core::PWSTR, pkeylist: *mut ::core::option::Option<IWbemPathKeyList>) -> ::windows::core::Result<()>;
    fn GetScopeAsText(&self, uindex: u32, putextbufsize: *mut u32, psztext: ::windows::core::PWSTR) -> ::windows::core::Result<()>;
    fn RemoveScope(&self, uindex: u32) -> ::windows::core::Result<()>;
    fn RemoveAllScopes(&self) -> ::windows::core::Result<()>;
    fn SetClassName(&self, name: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn GetClassName(&self, pubufflength: *mut u32, pszname: ::windows::core::PWSTR) -> ::windows::core::Result<()>;
    fn GetKeyList(&self) -> ::windows::core::Result<IWbemPathKeyList>;
    fn CreateClassPart(&self, lflags: i32, name: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn DeleteClassPart(&self, lflags: i32) -> ::windows::core::Result<()>;
    fn IsRelative(&self, wszmachine: &::windows::core::PCWSTR, wsznamespace: &::windows::core::PCWSTR) -> super::super::Foundation::BOOL;
    fn IsRelativeOrChild(&self, wszmachine: &::windows::core::PCWSTR, wsznamespace: &::windows::core::PCWSTR, lflags: i32) -> super::super::Foundation::BOOL;
    fn IsLocal(&self, wszmachine: &::windows::core::PCWSTR) -> super::super::Foundation::BOOL;
    fn IsSameClassName(&self, wszclass: &::windows::core::PCWSTR) -> super::super::Foundation::BOOL;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IWbemPath {}
#[cfg(feature = "Win32_Foundation")]
impl IWbemPath_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemPath_Impl, const OFFSET: isize>() -> IWbemPath_Vtbl {
        unsafe extern "system" fn SetText<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, umode: u32, pszpath: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetText(::core::mem::transmute_copy(&umode), ::core::mem::transmute(&pszpath)).into()
        }
        unsafe extern "system" fn GetText<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, pubufflength: *mut u32, psztext: ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetText(::core::mem::transmute_copy(&lflags), ::core::mem::transmute_copy(&pubufflength), ::core::mem::transmute_copy(&psztext)).into()
        }
        unsafe extern "system" fn GetInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, urequestedinfo: u32, puresponse: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetInfo(::core::mem::transmute_copy(&urequestedinfo)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(puresponse, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetServer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetServer(::core::mem::transmute(&name)).into()
        }
        unsafe extern "system" fn GetServer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punamebuflength: *mut u32, pname: ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetServer(::core::mem::transmute_copy(&punamebuflength), ::core::mem::transmute_copy(&pname)).into()
        }
        unsafe extern "system" fn GetNamespaceCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pucount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetNamespaceCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pucount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNamespaceAt<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uindex: u32, pszname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetNamespaceAt(::core::mem::transmute_copy(&uindex), ::core::mem::transmute(&pszname)).into()
        }
        unsafe extern "system" fn GetNamespaceAt<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uindex: u32, punamebuflength: *mut u32, pname: ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetNamespaceAt(::core::mem::transmute_copy(&uindex), ::core::mem::transmute_copy(&punamebuflength), ::core::mem::transmute_copy(&pname)).into()
        }
        unsafe extern "system" fn RemoveNamespaceAt<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveNamespaceAt(::core::mem::transmute_copy(&uindex)).into()
        }
        unsafe extern "system" fn RemoveAllNamespaces<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveAllNamespaces().into()
        }
        unsafe extern "system" fn GetScopeCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pucount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetScopeCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pucount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetScope<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uindex: u32, pszclass: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetScope(::core::mem::transmute_copy(&uindex), ::core::mem::transmute(&pszclass)).into()
        }
        unsafe extern "system" fn SetScopeFromText<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uindex: u32, psztext: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetScopeFromText(::core::mem::transmute_copy(&uindex), ::core::mem::transmute(&psztext)).into()
        }
        unsafe extern "system" fn GetScope<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uindex: u32, puclassnamebufsize: *mut u32, pszclass: ::windows::core::PWSTR, pkeylist: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetScope(::core::mem::transmute_copy(&uindex), ::core::mem::transmute_copy(&puclassnamebufsize), ::core::mem::transmute_copy(&pszclass), ::core::mem::transmute_copy(&pkeylist)).into()
        }
        unsafe extern "system" fn GetScopeAsText<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uindex: u32, putextbufsize: *mut u32, psztext: ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetScopeAsText(::core::mem::transmute_copy(&uindex), ::core::mem::transmute_copy(&putextbufsize), ::core::mem::transmute_copy(&psztext)).into()
        }
        unsafe extern "system" fn RemoveScope<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveScope(::core::mem::transmute_copy(&uindex)).into()
        }
        unsafe extern "system" fn RemoveAllScopes<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveAllScopes().into()
        }
        unsafe extern "system" fn SetClassName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetClassName(::core::mem::transmute(&name)).into()
        }
        unsafe extern "system" fn GetClassName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pubufflength: *mut u32, pszname: ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetClassName(::core::mem::transmute_copy(&pubufflength), ::core::mem::transmute_copy(&pszname)).into()
        }
        unsafe extern "system" fn GetKeyList<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pout: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetKeyList() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pout, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateClassPart<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, name: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateClassPart(::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&name)).into()
        }
        unsafe extern "system" fn DeleteClassPart<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteClassPart(::core::mem::transmute_copy(&lflags)).into()
        }
        unsafe extern "system" fn IsRelative<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszmachine: ::windows::core::PCWSTR, wsznamespace: ::windows::core::PCWSTR) -> super::super::Foundation::BOOL {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsRelative(::core::mem::transmute(&wszmachine), ::core::mem::transmute(&wsznamespace))
        }
        unsafe extern "system" fn IsRelativeOrChild<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszmachine: ::windows::core::PCWSTR, wsznamespace: ::windows::core::PCWSTR, lflags: i32) -> super::super::Foundation::BOOL {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsRelativeOrChild(::core::mem::transmute(&wszmachine), ::core::mem::transmute(&wsznamespace), ::core::mem::transmute_copy(&lflags))
        }
        unsafe extern "system" fn IsLocal<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszmachine: ::windows::core::PCWSTR) -> super::super::Foundation::BOOL {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsLocal(::core::mem::transmute(&wszmachine))
        }
        unsafe extern "system" fn IsSameClassName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszclass: ::windows::core::PCWSTR) -> super::super::Foundation::BOOL {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsSameClassName(::core::mem::transmute(&wszclass))
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SetText: SetText::<Identity, Impl, OFFSET>,
            GetText: GetText::<Identity, Impl, OFFSET>,
            GetInfo: GetInfo::<Identity, Impl, OFFSET>,
            SetServer: SetServer::<Identity, Impl, OFFSET>,
            GetServer: GetServer::<Identity, Impl, OFFSET>,
            GetNamespaceCount: GetNamespaceCount::<Identity, Impl, OFFSET>,
            SetNamespaceAt: SetNamespaceAt::<Identity, Impl, OFFSET>,
            GetNamespaceAt: GetNamespaceAt::<Identity, Impl, OFFSET>,
            RemoveNamespaceAt: RemoveNamespaceAt::<Identity, Impl, OFFSET>,
            RemoveAllNamespaces: RemoveAllNamespaces::<Identity, Impl, OFFSET>,
            GetScopeCount: GetScopeCount::<Identity, Impl, OFFSET>,
            SetScope: SetScope::<Identity, Impl, OFFSET>,
            SetScopeFromText: SetScopeFromText::<Identity, Impl, OFFSET>,
            GetScope: GetScope::<Identity, Impl, OFFSET>,
            GetScopeAsText: GetScopeAsText::<Identity, Impl, OFFSET>,
            RemoveScope: RemoveScope::<Identity, Impl, OFFSET>,
            RemoveAllScopes: RemoveAllScopes::<Identity, Impl, OFFSET>,
            SetClassName: SetClassName::<Identity, Impl, OFFSET>,
            GetClassName: GetClassName::<Identity, Impl, OFFSET>,
            GetKeyList: GetKeyList::<Identity, Impl, OFFSET>,
            CreateClassPart: CreateClassPart::<Identity, Impl, OFFSET>,
            DeleteClassPart: DeleteClassPart::<Identity, Impl, OFFSET>,
            IsRelative: IsRelative::<Identity, Impl, OFFSET>,
            IsRelativeOrChild: IsRelativeOrChild::<Identity, Impl, OFFSET>,
            IsLocal: IsLocal::<Identity, Impl, OFFSET>,
            IsSameClassName: IsSameClassName::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWbemPath as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWbemPathKeyList_Impl: Sized {
    fn GetCount(&self) -> ::windows::core::Result<u32>;
    fn SetKey(&self, wszname: &::windows::core::PCWSTR, uflags: u32, ucimtype: u32, pkeyval: *const ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn SetKey2(&self, wszname: &::windows::core::PCWSTR, uflags: u32, ucimtype: u32, pkeyval: *const super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn GetKey(&self, ukeyix: u32, uflags: u32, punamebufsize: *mut u32, pszkeyname: ::windows::core::PWSTR, pukeyvalbufsize: *mut u32, pkeyval: *mut ::core::ffi::c_void, puapparentcimtype: *mut u32) -> ::windows::core::Result<()>;
    fn GetKey2(&self, ukeyix: u32, uflags: u32, punamebufsize: *mut u32, pszkeyname: ::windows::core::PWSTR, pkeyvalue: *mut super::Com::VARIANT, puapparentcimtype: *mut u32) -> ::windows::core::Result<()>;
    fn RemoveKey(&self, wszname: &::windows::core::PCWSTR, uflags: u32) -> ::windows::core::Result<()>;
    fn RemoveAllKeys(&self, uflags: u32) -> ::windows::core::Result<()>;
    fn MakeSingleton(&self, bset: u8) -> ::windows::core::Result<()>;
    fn GetInfo(&self, urequestedinfo: u32) -> ::windows::core::Result<u64>;
    fn GetText(&self, lflags: i32, pubufflength: *mut u32, psztext: ::windows::core::PWSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IWbemPathKeyList {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWbemPathKeyList_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemPathKeyList_Impl, const OFFSET: isize>() -> IWbemPathKeyList_Vtbl {
        unsafe extern "system" fn GetCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemPathKeyList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pukeycount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pukeycount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetKey<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemPathKeyList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszname: ::windows::core::PCWSTR, uflags: u32, ucimtype: u32, pkeyval: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetKey(::core::mem::transmute(&wszname), ::core::mem::transmute_copy(&uflags), ::core::mem::transmute_copy(&ucimtype), ::core::mem::transmute_copy(&pkeyval)).into()
        }
        unsafe extern "system" fn SetKey2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemPathKeyList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszname: ::windows::core::PCWSTR, uflags: u32, ucimtype: u32, pkeyval: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetKey2(::core::mem::transmute(&wszname), ::core::mem::transmute_copy(&uflags), ::core::mem::transmute_copy(&ucimtype), ::core::mem::transmute_copy(&pkeyval)).into()
        }
        unsafe extern "system" fn GetKey<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemPathKeyList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ukeyix: u32, uflags: u32, punamebufsize: *mut u32, pszkeyname: ::windows::core::PWSTR, pukeyvalbufsize: *mut u32, pkeyval: *mut ::core::ffi::c_void, puapparentcimtype: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetKey(::core::mem::transmute_copy(&ukeyix), ::core::mem::transmute_copy(&uflags), ::core::mem::transmute_copy(&punamebufsize), ::core::mem::transmute_copy(&pszkeyname), ::core::mem::transmute_copy(&pukeyvalbufsize), ::core::mem::transmute_copy(&pkeyval), ::core::mem::transmute_copy(&puapparentcimtype)).into()
        }
        unsafe extern "system" fn GetKey2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemPathKeyList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ukeyix: u32, uflags: u32, punamebufsize: *mut u32, pszkeyname: ::windows::core::PWSTR, pkeyvalue: *mut super::Com::VARIANT, puapparentcimtype: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetKey2(::core::mem::transmute_copy(&ukeyix), ::core::mem::transmute_copy(&uflags), ::core::mem::transmute_copy(&punamebufsize), ::core::mem::transmute_copy(&pszkeyname), ::core::mem::transmute_copy(&pkeyvalue), ::core::mem::transmute_copy(&puapparentcimtype)).into()
        }
        unsafe extern "system" fn RemoveKey<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemPathKeyList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszname: ::windows::core::PCWSTR, uflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveKey(::core::mem::transmute(&wszname), ::core::mem::transmute_copy(&uflags)).into()
        }
        unsafe extern "system" fn RemoveAllKeys<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemPathKeyList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveAllKeys(::core::mem::transmute_copy(&uflags)).into()
        }
        unsafe extern "system" fn MakeSingleton<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemPathKeyList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bset: u8) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.MakeSingleton(::core::mem::transmute_copy(&bset)).into()
        }
        unsafe extern "system" fn GetInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemPathKeyList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, urequestedinfo: u32, puresponse: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetInfo(::core::mem::transmute_copy(&urequestedinfo)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(puresponse, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetText<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemPathKeyList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, pubufflength: *mut u32, psztext: ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetText(::core::mem::transmute_copy(&lflags), ::core::mem::transmute_copy(&pubufflength), ::core::mem::transmute_copy(&psztext)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            SetKey: SetKey::<Identity, Impl, OFFSET>,
            SetKey2: SetKey2::<Identity, Impl, OFFSET>,
            GetKey: GetKey::<Identity, Impl, OFFSET>,
            GetKey2: GetKey2::<Identity, Impl, OFFSET>,
            RemoveKey: RemoveKey::<Identity, Impl, OFFSET>,
            RemoveAllKeys: RemoveAllKeys::<Identity, Impl, OFFSET>,
            MakeSingleton: MakeSingleton::<Identity, Impl, OFFSET>,
            GetInfo: GetInfo::<Identity, Impl, OFFSET>,
            GetText: GetText::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWbemPathKeyList as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWbemPropertyProvider_Impl: Sized {
    fn GetProperty(&self, lflags: i32, strlocale: &super::super::Foundation::BSTR, strclassmapping: &super::super::Foundation::BSTR, strinstmapping: &super::super::Foundation::BSTR, strpropmapping: &super::super::Foundation::BSTR) -> ::windows::core::Result<super::Com::VARIANT>;
    fn PutProperty(&self, lflags: i32, strlocale: &super::super::Foundation::BSTR, strclassmapping: &super::super::Foundation::BSTR, strinstmapping: &super::super::Foundation::BSTR, strpropmapping: &super::super::Foundation::BSTR, pvvalue: *const super::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IWbemPropertyProvider {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWbemPropertyProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemPropertyProvider_Impl, const OFFSET: isize>() -> IWbemPropertyProvider_Vtbl {
        unsafe extern "system" fn GetProperty<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemPropertyProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, strlocale: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strclassmapping: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strinstmapping: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strpropmapping: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvvalue: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetProperty(::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&strlocale), ::core::mem::transmute(&strclassmapping), ::core::mem::transmute(&strinstmapping), ::core::mem::transmute(&strpropmapping)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvvalue, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PutProperty<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemPropertyProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, strlocale: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strclassmapping: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strinstmapping: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strpropmapping: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvvalue: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PutProperty(::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&strlocale), ::core::mem::transmute(&strclassmapping), ::core::mem::transmute(&strinstmapping), ::core::mem::transmute(&strpropmapping), ::core::mem::transmute_copy(&pvvalue)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetProperty: GetProperty::<Identity, Impl, OFFSET>,
            PutProperty: PutProperty::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWbemPropertyProvider as ::windows::core::Interface>::IID
    }
}
pub trait IWbemProviderIdentity_Impl: Sized {
    fn SetRegistrationObject(&self, lflags: i32, pprovreg: &::core::option::Option<IWbemClassObject>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IWbemProviderIdentity {}
impl IWbemProviderIdentity_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemProviderIdentity_Impl, const OFFSET: isize>() -> IWbemProviderIdentity_Vtbl {
        unsafe extern "system" fn SetRegistrationObject<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemProviderIdentity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, pprovreg: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetRegistrationObject(::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&pprovreg)).into()
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), SetRegistrationObject: SetRegistrationObject::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWbemProviderIdentity as ::windows::core::Interface>::IID
    }
}
pub trait IWbemProviderInit_Impl: Sized {
    fn Initialize(&self, wszuser: &::windows::core::PCWSTR, lflags: i32, wsznamespace: &::windows::core::PCWSTR, wszlocale: &::windows::core::PCWSTR, pnamespace: &::core::option::Option<IWbemServices>, pctx: &::core::option::Option<IWbemContext>, pinitsink: &::core::option::Option<IWbemProviderInitSink>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IWbemProviderInit {}
impl IWbemProviderInit_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemProviderInit_Impl, const OFFSET: isize>() -> IWbemProviderInit_Vtbl {
        unsafe extern "system" fn Initialize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemProviderInit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszuser: ::windows::core::PCWSTR, lflags: i32, wsznamespace: ::windows::core::PCWSTR, wszlocale: ::windows::core::PCWSTR, pnamespace: *mut ::core::ffi::c_void, pctx: *mut ::core::ffi::c_void, pinitsink: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Initialize(::core::mem::transmute(&wszuser), ::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&wsznamespace), ::core::mem::transmute(&wszlocale), ::core::mem::transmute(&pnamespace), ::core::mem::transmute(&pctx), ::core::mem::transmute(&pinitsink)).into()
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), Initialize: Initialize::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWbemProviderInit as ::windows::core::Interface>::IID
    }
}
pub trait IWbemProviderInitSink_Impl: Sized {
    fn SetStatus(&self, lstatus: i32, lflags: i32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IWbemProviderInitSink {}
impl IWbemProviderInitSink_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemProviderInitSink_Impl, const OFFSET: isize>() -> IWbemProviderInitSink_Vtbl {
        unsafe extern "system" fn SetStatus<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemProviderInitSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lstatus: i32, lflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetStatus(::core::mem::transmute_copy(&lstatus), ::core::mem::transmute_copy(&lflags)).into()
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), SetStatus: SetStatus::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWbemProviderInitSink as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWbemQualifierSet_Impl: Sized {
    fn Get(&self, wszname: &::windows::core::PCWSTR, lflags: i32, pval: *mut super::Com::VARIANT, plflavor: *mut i32) -> ::windows::core::Result<()>;
    fn Put(&self, wszname: &::windows::core::PCWSTR, pval: *const super::Com::VARIANT, lflavor: i32) -> ::windows::core::Result<()>;
    fn Delete(&self, wszname: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn GetNames(&self, lflags: i32) -> ::windows::core::Result<*mut super::Com::SAFEARRAY>;
    fn BeginEnumeration(&self, lflags: i32) -> ::windows::core::Result<()>;
    fn Next(&self, lflags: i32, pstrname: *mut super::super::Foundation::BSTR, pval: *mut super::Com::VARIANT, plflavor: *mut i32) -> ::windows::core::Result<()>;
    fn EndEnumeration(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IWbemQualifierSet {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWbemQualifierSet_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemQualifierSet_Impl, const OFFSET: isize>() -> IWbemQualifierSet_Vtbl {
        unsafe extern "system" fn Get<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemQualifierSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszname: ::windows::core::PCWSTR, lflags: i32, pval: *mut super::Com::VARIANT, plflavor: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Get(::core::mem::transmute(&wszname), ::core::mem::transmute_copy(&lflags), ::core::mem::transmute_copy(&pval), ::core::mem::transmute_copy(&plflavor)).into()
        }
        unsafe extern "system" fn Put<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemQualifierSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszname: ::windows::core::PCWSTR, pval: *const super::Com::VARIANT, lflavor: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Put(::core::mem::transmute(&wszname), ::core::mem::transmute_copy(&pval), ::core::mem::transmute_copy(&lflavor)).into()
        }
        unsafe extern "system" fn Delete<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemQualifierSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Delete(::core::mem::transmute(&wszname)).into()
        }
        unsafe extern "system" fn GetNames<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemQualifierSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, pnames: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetNames(::core::mem::transmute_copy(&lflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pnames, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginEnumeration<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemQualifierSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.BeginEnumeration(::core::mem::transmute_copy(&lflags)).into()
        }
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemQualifierSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, pstrname: *mut super::super::Foundation::BSTR, pval: *mut super::Com::VARIANT, plflavor: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Next(::core::mem::transmute_copy(&lflags), ::core::mem::transmute_copy(&pstrname), ::core::mem::transmute_copy(&pval), ::core::mem::transmute_copy(&plflavor)).into()
        }
        unsafe extern "system" fn EndEnumeration<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemQualifierSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndEnumeration().into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Get: Get::<Identity, Impl, OFFSET>,
            Put: Put::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
            GetNames: GetNames::<Identity, Impl, OFFSET>,
            BeginEnumeration: BeginEnumeration::<Identity, Impl, OFFSET>,
            Next: Next::<Identity, Impl, OFFSET>,
            EndEnumeration: EndEnumeration::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWbemQualifierSet as ::windows::core::Interface>::IID
    }
}
pub trait IWbemQuery_Impl: Sized {
    fn Empty(&self) -> ::windows::core::Result<()>;
    fn SetLanguageFeatures(&self, uflags: u32, uarraysize: u32, pufeatures: *const u32) -> ::windows::core::Result<()>;
    fn TestLanguageFeatures(&self, uflags: u32, uarraysize: *mut u32, pufeatures: *mut u32) -> ::windows::core::Result<()>;
    fn Parse(&self, pszlang: &::windows::core::PCWSTR, pszquery: &::windows::core::PCWSTR, uflags: u32) -> ::windows::core::Result<()>;
    fn GetAnalysis(&self, uanalysistype: u32, uflags: u32, panalysis: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn FreeMemory(&self, pmem: *const ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn GetQueryInfo(&self, uanalysistype: u32, uinfoid: u32, ubufsize: u32, pdestbuf: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IWbemQuery {}
impl IWbemQuery_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemQuery_Impl, const OFFSET: isize>() -> IWbemQuery_Vtbl {
        unsafe extern "system" fn Empty<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemQuery_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Empty().into()
        }
        unsafe extern "system" fn SetLanguageFeatures<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemQuery_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uflags: u32, uarraysize: u32, pufeatures: *const u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetLanguageFeatures(::core::mem::transmute_copy(&uflags), ::core::mem::transmute_copy(&uarraysize), ::core::mem::transmute_copy(&pufeatures)).into()
        }
        unsafe extern "system" fn TestLanguageFeatures<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemQuery_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uflags: u32, uarraysize: *mut u32, pufeatures: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.TestLanguageFeatures(::core::mem::transmute_copy(&uflags), ::core::mem::transmute_copy(&uarraysize), ::core::mem::transmute_copy(&pufeatures)).into()
        }
        unsafe extern "system" fn Parse<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemQuery_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszlang: ::windows::core::PCWSTR, pszquery: ::windows::core::PCWSTR, uflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Parse(::core::mem::transmute(&pszlang), ::core::mem::transmute(&pszquery), ::core::mem::transmute_copy(&uflags)).into()
        }
        unsafe extern "system" fn GetAnalysis<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemQuery_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uanalysistype: u32, uflags: u32, panalysis: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetAnalysis(::core::mem::transmute_copy(&uanalysistype), ::core::mem::transmute_copy(&uflags), ::core::mem::transmute_copy(&panalysis)).into()
        }
        unsafe extern "system" fn FreeMemory<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemQuery_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmem: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FreeMemory(::core::mem::transmute_copy(&pmem)).into()
        }
        unsafe extern "system" fn GetQueryInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemQuery_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uanalysistype: u32, uinfoid: u32, ubufsize: u32, pdestbuf: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetQueryInfo(::core::mem::transmute_copy(&uanalysistype), ::core::mem::transmute_copy(&uinfoid), ::core::mem::transmute_copy(&ubufsize), ::core::mem::transmute_copy(&pdestbuf)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Empty: Empty::<Identity, Impl, OFFSET>,
            SetLanguageFeatures: SetLanguageFeatures::<Identity, Impl, OFFSET>,
            TestLanguageFeatures: TestLanguageFeatures::<Identity, Impl, OFFSET>,
            Parse: Parse::<Identity, Impl, OFFSET>,
            GetAnalysis: GetAnalysis::<Identity, Impl, OFFSET>,
            FreeMemory: FreeMemory::<Identity, Impl, OFFSET>,
            GetQueryInfo: GetQueryInfo::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWbemQuery as ::windows::core::Interface>::IID
    }
}
pub trait IWbemRefresher_Impl: Sized {
    fn Refresh(&self, lflags: i32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IWbemRefresher {}
impl IWbemRefresher_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemRefresher_Impl, const OFFSET: isize>() -> IWbemRefresher_Vtbl {
        unsafe extern "system" fn Refresh<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemRefresher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Refresh(::core::mem::transmute_copy(&lflags)).into()
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), Refresh: Refresh::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWbemRefresher as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWbemServices_Impl: Sized {
    fn OpenNamespace(&self, strnamespace: &super::super::Foundation::BSTR, lflags: i32, pctx: &::core::option::Option<IWbemContext>, ppworkingnamespace: *mut ::core::option::Option<IWbemServices>, ppresult: *mut ::core::option::Option<IWbemCallResult>) -> ::windows::core::Result<()>;
    fn CancelAsyncCall(&self, psink: &::core::option::Option<IWbemObjectSink>) -> ::windows::core::Result<()>;
    fn QueryObjectSink(&self, lflags: i32) -> ::windows::core::Result<IWbemObjectSink>;
    fn GetObject(&self, strobjectpath: &super::super::Foundation::BSTR, lflags: i32, pctx: &::core::option::Option<IWbemContext>, ppobject: *mut ::core::option::Option<IWbemClassObject>, ppcallresult: *mut ::core::option::Option<IWbemCallResult>) -> ::windows::core::Result<()>;
    fn GetObjectAsync(&self, strobjectpath: &super::super::Foundation::BSTR, lflags: i32, pctx: &::core::option::Option<IWbemContext>, presponsehandler: &::core::option::Option<IWbemObjectSink>) -> ::windows::core::Result<()>;
    fn PutClass(&self, pobject: &::core::option::Option<IWbemClassObject>, lflags: i32, pctx: &::core::option::Option<IWbemContext>, ppcallresult: *mut ::core::option::Option<IWbemCallResult>) -> ::windows::core::Result<()>;
    fn PutClassAsync(&self, pobject: &::core::option::Option<IWbemClassObject>, lflags: i32, pctx: &::core::option::Option<IWbemContext>, presponsehandler: &::core::option::Option<IWbemObjectSink>) -> ::windows::core::Result<()>;
    fn DeleteClass(&self, strclass: &super::super::Foundation::BSTR, lflags: i32, pctx: &::core::option::Option<IWbemContext>, ppcallresult: *mut ::core::option::Option<IWbemCallResult>) -> ::windows::core::Result<()>;
    fn DeleteClassAsync(&self, strclass: &super::super::Foundation::BSTR, lflags: i32, pctx: &::core::option::Option<IWbemContext>, presponsehandler: &::core::option::Option<IWbemObjectSink>) -> ::windows::core::Result<()>;
    fn CreateClassEnum(&self, strsuperclass: &super::super::Foundation::BSTR, lflags: i32, pctx: &::core::option::Option<IWbemContext>) -> ::windows::core::Result<IEnumWbemClassObject>;
    fn CreateClassEnumAsync(&self, strsuperclass: &super::super::Foundation::BSTR, lflags: i32, pctx: &::core::option::Option<IWbemContext>, presponsehandler: &::core::option::Option<IWbemObjectSink>) -> ::windows::core::Result<()>;
    fn PutInstance(&self, pinst: &::core::option::Option<IWbemClassObject>, lflags: i32, pctx: &::core::option::Option<IWbemContext>, ppcallresult: *mut ::core::option::Option<IWbemCallResult>) -> ::windows::core::Result<()>;
    fn PutInstanceAsync(&self, pinst: &::core::option::Option<IWbemClassObject>, lflags: i32, pctx: &::core::option::Option<IWbemContext>, presponsehandler: &::core::option::Option<IWbemObjectSink>) -> ::windows::core::Result<()>;
    fn DeleteInstance(&self, strobjectpath: &super::super::Foundation::BSTR, lflags: i32, pctx: &::core::option::Option<IWbemContext>, ppcallresult: *mut ::core::option::Option<IWbemCallResult>) -> ::windows::core::Result<()>;
    fn DeleteInstanceAsync(&self, strobjectpath: &super::super::Foundation::BSTR, lflags: i32, pctx: &::core::option::Option<IWbemContext>, presponsehandler: &::core::option::Option<IWbemObjectSink>) -> ::windows::core::Result<()>;
    fn CreateInstanceEnum(&self, strfilter: &super::super::Foundation::BSTR, lflags: i32, pctx: &::core::option::Option<IWbemContext>) -> ::windows::core::Result<IEnumWbemClassObject>;
    fn CreateInstanceEnumAsync(&self, strfilter: &super::super::Foundation::BSTR, lflags: i32, pctx: &::core::option::Option<IWbemContext>, presponsehandler: &::core::option::Option<IWbemObjectSink>) -> ::windows::core::Result<()>;
    fn ExecQuery(&self, strquerylanguage: &super::super::Foundation::BSTR, strquery: &super::super::Foundation::BSTR, lflags: i32, pctx: &::core::option::Option<IWbemContext>) -> ::windows::core::Result<IEnumWbemClassObject>;
    fn ExecQueryAsync(&self, strquerylanguage: &super::super::Foundation::BSTR, strquery: &super::super::Foundation::BSTR, lflags: i32, pctx: &::core::option::Option<IWbemContext>, presponsehandler: &::core::option::Option<IWbemObjectSink>) -> ::windows::core::Result<()>;
    fn ExecNotificationQuery(&self, strquerylanguage: &super::super::Foundation::BSTR, strquery: &super::super::Foundation::BSTR, lflags: i32, pctx: &::core::option::Option<IWbemContext>) -> ::windows::core::Result<IEnumWbemClassObject>;
    fn ExecNotificationQueryAsync(&self, strquerylanguage: &super::super::Foundation::BSTR, strquery: &super::super::Foundation::BSTR, lflags: i32, pctx: &::core::option::Option<IWbemContext>, presponsehandler: &::core::option::Option<IWbemObjectSink>) -> ::windows::core::Result<()>;
    fn ExecMethod(&self, strobjectpath: &super::super::Foundation::BSTR, strmethodname: &super::super::Foundation::BSTR, lflags: i32, pctx: &::core::option::Option<IWbemContext>, pinparams: &::core::option::Option<IWbemClassObject>, ppoutparams: *mut ::core::option::Option<IWbemClassObject>, ppcallresult: *mut ::core::option::Option<IWbemCallResult>) -> ::windows::core::Result<()>;
    fn ExecMethodAsync(&self, strobjectpath: &super::super::Foundation::BSTR, strmethodname: &super::super::Foundation::BSTR, lflags: i32, pctx: &::core::option::Option<IWbemContext>, pinparams: &::core::option::Option<IWbemClassObject>, presponsehandler: &::core::option::Option<IWbemObjectSink>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IWbemServices {}
#[cfg(feature = "Win32_Foundation")]
impl IWbemServices_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemServices_Impl, const OFFSET: isize>() -> IWbemServices_Vtbl {
        unsafe extern "system" fn OpenNamespace<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strnamespace: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lflags: i32, pctx: *mut ::core::ffi::c_void, ppworkingnamespace: *mut *mut ::core::ffi::c_void, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OpenNamespace(::core::mem::transmute(&strnamespace), ::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&pctx), ::core::mem::transmute_copy(&ppworkingnamespace), ::core::mem::transmute_copy(&ppresult)).into()
        }
        unsafe extern "system" fn CancelAsyncCall<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psink: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CancelAsyncCall(::core::mem::transmute(&psink)).into()
        }
        unsafe extern "system" fn QueryObjectSink<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, ppresponsehandler: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.QueryObjectSink(::core::mem::transmute_copy(&lflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresponsehandler, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetObject<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strobjectpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lflags: i32, pctx: *mut ::core::ffi::c_void, ppobject: *mut *mut ::core::ffi::c_void, ppcallresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetObject(::core::mem::transmute(&strobjectpath), ::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&pctx), ::core::mem::transmute_copy(&ppobject), ::core::mem::transmute_copy(&ppcallresult)).into()
        }
        unsafe extern "system" fn GetObjectAsync<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strobjectpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lflags: i32, pctx: *mut ::core::ffi::c_void, presponsehandler: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetObjectAsync(::core::mem::transmute(&strobjectpath), ::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&pctx), ::core::mem::transmute(&presponsehandler)).into()
        }
        unsafe extern "system" fn PutClass<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pobject: *mut ::core::ffi::c_void, lflags: i32, pctx: *mut ::core::ffi::c_void, ppcallresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PutClass(::core::mem::transmute(&pobject), ::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&pctx), ::core::mem::transmute_copy(&ppcallresult)).into()
        }
        unsafe extern "system" fn PutClassAsync<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pobject: *mut ::core::ffi::c_void, lflags: i32, pctx: *mut ::core::ffi::c_void, presponsehandler: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PutClassAsync(::core::mem::transmute(&pobject), ::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&pctx), ::core::mem::transmute(&presponsehandler)).into()
        }
        unsafe extern "system" fn DeleteClass<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lflags: i32, pctx: *mut ::core::ffi::c_void, ppcallresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteClass(::core::mem::transmute(&strclass), ::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&pctx), ::core::mem::transmute_copy(&ppcallresult)).into()
        }
        unsafe extern "system" fn DeleteClassAsync<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lflags: i32, pctx: *mut ::core::ffi::c_void, presponsehandler: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteClassAsync(::core::mem::transmute(&strclass), ::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&pctx), ::core::mem::transmute(&presponsehandler)).into()
        }
        unsafe extern "system" fn CreateClassEnum<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strsuperclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lflags: i32, pctx: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateClassEnum(::core::mem::transmute(&strsuperclass), ::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&pctx)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateClassEnumAsync<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strsuperclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lflags: i32, pctx: *mut ::core::ffi::c_void, presponsehandler: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateClassEnumAsync(::core::mem::transmute(&strsuperclass), ::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&pctx), ::core::mem::transmute(&presponsehandler)).into()
        }
        unsafe extern "system" fn PutInstance<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinst: *mut ::core::ffi::c_void, lflags: i32, pctx: *mut ::core::ffi::c_void, ppcallresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PutInstance(::core::mem::transmute(&pinst), ::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&pctx), ::core::mem::transmute_copy(&ppcallresult)).into()
        }
        unsafe extern "system" fn PutInstanceAsync<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinst: *mut ::core::ffi::c_void, lflags: i32, pctx: *mut ::core::ffi::c_void, presponsehandler: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PutInstanceAsync(::core::mem::transmute(&pinst), ::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&pctx), ::core::mem::transmute(&presponsehandler)).into()
        }
        unsafe extern "system" fn DeleteInstance<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strobjectpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lflags: i32, pctx: *mut ::core::ffi::c_void, ppcallresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteInstance(::core::mem::transmute(&strobjectpath), ::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&pctx), ::core::mem::transmute_copy(&ppcallresult)).into()
        }
        unsafe extern "system" fn DeleteInstanceAsync<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strobjectpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lflags: i32, pctx: *mut ::core::ffi::c_void, presponsehandler: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteInstanceAsync(::core::mem::transmute(&strobjectpath), ::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&pctx), ::core::mem::transmute(&presponsehandler)).into()
        }
        unsafe extern "system" fn CreateInstanceEnum<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strfilter: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lflags: i32, pctx: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateInstanceEnum(::core::mem::transmute(&strfilter), ::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&pctx)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateInstanceEnumAsync<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strfilter: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lflags: i32, pctx: *mut ::core::ffi::c_void, presponsehandler: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateInstanceEnumAsync(::core::mem::transmute(&strfilter), ::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&pctx), ::core::mem::transmute(&presponsehandler)).into()
        }
        unsafe extern "system" fn ExecQuery<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strquerylanguage: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strquery: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lflags: i32, pctx: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ExecQuery(::core::mem::transmute(&strquerylanguage), ::core::mem::transmute(&strquery), ::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&pctx)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExecQueryAsync<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strquerylanguage: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strquery: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lflags: i32, pctx: *mut ::core::ffi::c_void, presponsehandler: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ExecQueryAsync(::core::mem::transmute(&strquerylanguage), ::core::mem::transmute(&strquery), ::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&pctx), ::core::mem::transmute(&presponsehandler)).into()
        }
        unsafe extern "system" fn ExecNotificationQuery<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strquerylanguage: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strquery: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lflags: i32, pctx: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ExecNotificationQuery(::core::mem::transmute(&strquerylanguage), ::core::mem::transmute(&strquery), ::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&pctx)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExecNotificationQueryAsync<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strquerylanguage: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strquery: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lflags: i32, pctx: *mut ::core::ffi::c_void, presponsehandler: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ExecNotificationQueryAsync(::core::mem::transmute(&strquerylanguage), ::core::mem::transmute(&strquery), ::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&pctx), ::core::mem::transmute(&presponsehandler)).into()
        }
        unsafe extern "system" fn ExecMethod<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strobjectpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strmethodname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lflags: i32, pctx: *mut ::core::ffi::c_void, pinparams: *mut ::core::ffi::c_void, ppoutparams: *mut *mut ::core::ffi::c_void, ppcallresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ExecMethod(::core::mem::transmute(&strobjectpath), ::core::mem::transmute(&strmethodname), ::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&pctx), ::core::mem::transmute(&pinparams), ::core::mem::transmute_copy(&ppoutparams), ::core::mem::transmute_copy(&ppcallresult)).into()
        }
        unsafe extern "system" fn ExecMethodAsync<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strobjectpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strmethodname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lflags: i32, pctx: *mut ::core::ffi::c_void, pinparams: *mut ::core::ffi::c_void, presponsehandler: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ExecMethodAsync(::core::mem::transmute(&strobjectpath), ::core::mem::transmute(&strmethodname), ::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&pctx), ::core::mem::transmute(&pinparams), ::core::mem::transmute(&presponsehandler)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            OpenNamespace: OpenNamespace::<Identity, Impl, OFFSET>,
            CancelAsyncCall: CancelAsyncCall::<Identity, Impl, OFFSET>,
            QueryObjectSink: QueryObjectSink::<Identity, Impl, OFFSET>,
            GetObject: GetObject::<Identity, Impl, OFFSET>,
            GetObjectAsync: GetObjectAsync::<Identity, Impl, OFFSET>,
            PutClass: PutClass::<Identity, Impl, OFFSET>,
            PutClassAsync: PutClassAsync::<Identity, Impl, OFFSET>,
            DeleteClass: DeleteClass::<Identity, Impl, OFFSET>,
            DeleteClassAsync: DeleteClassAsync::<Identity, Impl, OFFSET>,
            CreateClassEnum: CreateClassEnum::<Identity, Impl, OFFSET>,
            CreateClassEnumAsync: CreateClassEnumAsync::<Identity, Impl, OFFSET>,
            PutInstance: PutInstance::<Identity, Impl, OFFSET>,
            PutInstanceAsync: PutInstanceAsync::<Identity, Impl, OFFSET>,
            DeleteInstance: DeleteInstance::<Identity, Impl, OFFSET>,
            DeleteInstanceAsync: DeleteInstanceAsync::<Identity, Impl, OFFSET>,
            CreateInstanceEnum: CreateInstanceEnum::<Identity, Impl, OFFSET>,
            CreateInstanceEnumAsync: CreateInstanceEnumAsync::<Identity, Impl, OFFSET>,
            ExecQuery: ExecQuery::<Identity, Impl, OFFSET>,
            ExecQueryAsync: ExecQueryAsync::<Identity, Impl, OFFSET>,
            ExecNotificationQuery: ExecNotificationQuery::<Identity, Impl, OFFSET>,
            ExecNotificationQueryAsync: ExecNotificationQueryAsync::<Identity, Impl, OFFSET>,
            ExecMethod: ExecMethod::<Identity, Impl, OFFSET>,
            ExecMethodAsync: ExecMethodAsync::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWbemServices as ::windows::core::Interface>::IID
    }
}
pub trait IWbemShutdown_Impl: Sized {
    fn Shutdown(&self, ureason: i32, umaxmilliseconds: u32, pctx: &::core::option::Option<IWbemContext>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IWbemShutdown {}
impl IWbemShutdown_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemShutdown_Impl, const OFFSET: isize>() -> IWbemShutdown_Vtbl {
        unsafe extern "system" fn Shutdown<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemShutdown_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ureason: i32, umaxmilliseconds: u32, pctx: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Shutdown(::core::mem::transmute_copy(&ureason), ::core::mem::transmute_copy(&umaxmilliseconds), ::core::mem::transmute(&pctx)).into()
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), Shutdown: Shutdown::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWbemShutdown as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWbemStatusCodeText_Impl: Sized {
    fn GetErrorCodeText(&self, hres: ::windows::core::HRESULT, localeid: u32, lflags: i32) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetFacilityCodeText(&self, hres: ::windows::core::HRESULT, localeid: u32, lflags: i32) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IWbemStatusCodeText {}
#[cfg(feature = "Win32_Foundation")]
impl IWbemStatusCodeText_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemStatusCodeText_Impl, const OFFSET: isize>() -> IWbemStatusCodeText_Vtbl {
        unsafe extern "system" fn GetErrorCodeText<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemStatusCodeText_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hres: ::windows::core::HRESULT, localeid: u32, lflags: i32, messagetext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetErrorCodeText(::core::mem::transmute_copy(&hres), ::core::mem::transmute_copy(&localeid), ::core::mem::transmute_copy(&lflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(messagetext, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFacilityCodeText<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemStatusCodeText_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hres: ::windows::core::HRESULT, localeid: u32, lflags: i32, messagetext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFacilityCodeText(::core::mem::transmute_copy(&hres), ::core::mem::transmute_copy(&localeid), ::core::mem::transmute_copy(&lflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(messagetext, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetErrorCodeText: GetErrorCodeText::<Identity, Impl, OFFSET>,
            GetFacilityCodeText: GetFacilityCodeText::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWbemStatusCodeText as ::windows::core::Interface>::IID
    }
}
pub trait IWbemTransport_Impl: Sized {
    fn Initialize(&self) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IWbemTransport {}
impl IWbemTransport_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemTransport_Impl, const OFFSET: isize>() -> IWbemTransport_Vtbl {
        unsafe extern "system" fn Initialize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemTransport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Initialize().into()
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), Initialize: Initialize::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWbemTransport as ::windows::core::Interface>::IID
    }
}
pub trait IWbemUnboundObjectSink_Impl: Sized {
    fn IndicateToConsumer(&self, plogicalconsumer: &::core::option::Option<IWbemClassObject>, lnumobjects: i32, apobjects: *const ::core::option::Option<IWbemClassObject>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IWbemUnboundObjectSink {}
impl IWbemUnboundObjectSink_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemUnboundObjectSink_Impl, const OFFSET: isize>() -> IWbemUnboundObjectSink_Vtbl {
        unsafe extern "system" fn IndicateToConsumer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemUnboundObjectSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plogicalconsumer: *mut ::core::ffi::c_void, lnumobjects: i32, apobjects: *const *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IndicateToConsumer(::core::mem::transmute(&plogicalconsumer), ::core::mem::transmute_copy(&lnumobjects), ::core::mem::transmute_copy(&apobjects)).into()
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), IndicateToConsumer: IndicateToConsumer::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWbemUnboundObjectSink as ::windows::core::Interface>::IID
    }
}
pub trait IWbemUnsecuredApartment_Impl: Sized + IUnsecuredApartment_Impl {
    fn CreateSinkStub(&self, psink: &::core::option::Option<IWbemObjectSink>, dwflags: u32, wszreserved: &::windows::core::PCWSTR) -> ::windows::core::Result<IWbemObjectSink>;
}
impl ::windows::core::RuntimeName for IWbemUnsecuredApartment {}
impl IWbemUnsecuredApartment_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemUnsecuredApartment_Impl, const OFFSET: isize>() -> IWbemUnsecuredApartment_Vtbl {
        unsafe extern "system" fn CreateSinkStub<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWbemUnsecuredApartment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psink: *mut ::core::ffi::c_void, dwflags: u32, wszreserved: ::windows::core::PCWSTR, ppstub: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateSinkStub(::core::mem::transmute(&psink), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute(&wszreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppstub, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: IUnsecuredApartment_Vtbl::new::<Identity, Impl, OFFSET>(), CreateSinkStub: CreateSinkStub::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWbemUnsecuredApartment as ::windows::core::Interface>::IID || iid == &<IUnsecuredApartment as ::windows::core::Interface>::IID
    }
}
