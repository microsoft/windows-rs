pub trait IEnumWbemClassObjectImpl: Sized {
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Next(&mut self, ltimeout: i32, ucount: u32, apobjects: *mut ::core::option::Option<IWbemClassObject>, pureturned: *mut u32) -> ::windows::core::HRESULT;
    fn NextAsync(&mut self, ucount: u32, psink: ::core::option::Option<IWbemObjectSink>) -> ::windows::core::HRESULT;
    fn Clone(&mut self) -> ::windows::core::Result<IEnumWbemClassObject>;
    fn Skip(&mut self, ltimeout: i32, ncount: u32) -> ::windows::core::HRESULT;
}
impl IEnumWbemClassObjectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumWbemClassObjectImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumWbemClassObjectVtbl {
        unsafe extern "system" fn Reset<Impl: IEnumWbemClassObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Next<Impl: IEnumWbemClassObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ltimeout: i32, ucount: u32, apobjects: *mut ::windows::core::RawPtr, pureturned: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&ltimeout), ::core::mem::transmute_copy(&ucount), ::core::mem::transmute_copy(&apobjects), ::core::mem::transmute_copy(&pureturned))
        }
        unsafe extern "system" fn NextAsync<Impl: IEnumWbemClassObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ucount: u32, psink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).NextAsync(::core::mem::transmute_copy(&ucount), ::core::mem::transmute(&psink))
        }
        unsafe extern "system" fn Clone<Impl: IEnumWbemClassObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Skip<Impl: IEnumWbemClassObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ltimeout: i32, ncount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&ltimeout), ::core::mem::transmute_copy(&ncount))
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Reset: Reset::<Impl, IMPL_OFFSET>,
            Next: Next::<Impl, IMPL_OFFSET>,
            NextAsync: NextAsync::<Impl, IMPL_OFFSET>,
            Clone: Clone::<Impl, IMPL_OFFSET>,
            Skip: Skip::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumWbemClassObject as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMofCompilerImpl: Sized {
    fn CompileFile(&mut self, filename: super::super::Foundation::PWSTR, serverandnamespace: super::super::Foundation::PWSTR, user: super::super::Foundation::PWSTR, authority: super::super::Foundation::PWSTR, password: super::super::Foundation::PWSTR, loptionflags: i32, lclassflags: i32, linstanceflags: i32, pinfo: *mut WBEM_COMPILE_STATUS_INFO) -> ::windows::core::Result<()>;
    fn CompileBuffer(&mut self, buffsize: i32, pbuffer: *const u8, serverandnamespace: super::super::Foundation::PWSTR, user: super::super::Foundation::PWSTR, authority: super::super::Foundation::PWSTR, password: super::super::Foundation::PWSTR, loptionflags: i32, lclassflags: i32, linstanceflags: i32, pinfo: *mut WBEM_COMPILE_STATUS_INFO) -> ::windows::core::Result<()>;
    fn CreateBMOF(&mut self, textfilename: super::super::Foundation::PWSTR, bmoffilename: super::super::Foundation::PWSTR, serverandnamespace: super::super::Foundation::PWSTR, loptionflags: i32, lclassflags: i32, linstanceflags: i32, pinfo: *mut WBEM_COMPILE_STATUS_INFO) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IMofCompilerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMofCompilerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMofCompilerVtbl {
        unsafe extern "system" fn CompileFile<Impl: IMofCompilerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: super::super::Foundation::PWSTR, serverandnamespace: super::super::Foundation::PWSTR, user: super::super::Foundation::PWSTR, authority: super::super::Foundation::PWSTR, password: super::super::Foundation::PWSTR, loptionflags: i32, lclassflags: i32, linstanceflags: i32, pinfo: *mut WBEM_COMPILE_STATUS_INFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CompileFile(::core::mem::transmute_copy(&filename), ::core::mem::transmute_copy(&serverandnamespace), ::core::mem::transmute_copy(&user), ::core::mem::transmute_copy(&authority), ::core::mem::transmute_copy(&password), ::core::mem::transmute_copy(&loptionflags), ::core::mem::transmute_copy(&lclassflags), ::core::mem::transmute_copy(&linstanceflags), ::core::mem::transmute_copy(&pinfo)).into()
        }
        unsafe extern "system" fn CompileBuffer<Impl: IMofCompilerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buffsize: i32, pbuffer: *const u8, serverandnamespace: super::super::Foundation::PWSTR, user: super::super::Foundation::PWSTR, authority: super::super::Foundation::PWSTR, password: super::super::Foundation::PWSTR, loptionflags: i32, lclassflags: i32, linstanceflags: i32, pinfo: *mut WBEM_COMPILE_STATUS_INFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CompileBuffer(::core::mem::transmute_copy(&buffsize), ::core::mem::transmute_copy(&pbuffer), ::core::mem::transmute_copy(&serverandnamespace), ::core::mem::transmute_copy(&user), ::core::mem::transmute_copy(&authority), ::core::mem::transmute_copy(&password), ::core::mem::transmute_copy(&loptionflags), ::core::mem::transmute_copy(&lclassflags), ::core::mem::transmute_copy(&linstanceflags), ::core::mem::transmute_copy(&pinfo)).into()
        }
        unsafe extern "system" fn CreateBMOF<Impl: IMofCompilerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, textfilename: super::super::Foundation::PWSTR, bmoffilename: super::super::Foundation::PWSTR, serverandnamespace: super::super::Foundation::PWSTR, loptionflags: i32, lclassflags: i32, linstanceflags: i32, pinfo: *mut WBEM_COMPILE_STATUS_INFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateBMOF(::core::mem::transmute_copy(&textfilename), ::core::mem::transmute_copy(&bmoffilename), ::core::mem::transmute_copy(&serverandnamespace), ::core::mem::transmute_copy(&loptionflags), ::core::mem::transmute_copy(&lclassflags), ::core::mem::transmute_copy(&linstanceflags), ::core::mem::transmute_copy(&pinfo)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            CompileFile: CompileFile::<Impl, IMPL_OFFSET>,
            CompileBuffer: CompileBuffer::<Impl, IMPL_OFFSET>,
            CreateBMOF: CreateBMOF::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMofCompiler as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISWbemDateTimeImpl: Sized + IDispatchImpl {
    fn Value(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetValue(&mut self, strvalue: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Year(&mut self) -> ::windows::core::Result<i32>;
    fn SetYear(&mut self, iyear: i32) -> ::windows::core::Result<()>;
    fn YearSpecified(&mut self) -> ::windows::core::Result<i16>;
    fn SetYearSpecified(&mut self, byearspecified: i16) -> ::windows::core::Result<()>;
    fn Month(&mut self) -> ::windows::core::Result<i32>;
    fn SetMonth(&mut self, imonth: i32) -> ::windows::core::Result<()>;
    fn MonthSpecified(&mut self) -> ::windows::core::Result<i16>;
    fn SetMonthSpecified(&mut self, bmonthspecified: i16) -> ::windows::core::Result<()>;
    fn Day(&mut self) -> ::windows::core::Result<i32>;
    fn SetDay(&mut self, iday: i32) -> ::windows::core::Result<()>;
    fn DaySpecified(&mut self) -> ::windows::core::Result<i16>;
    fn SetDaySpecified(&mut self, bdayspecified: i16) -> ::windows::core::Result<()>;
    fn Hours(&mut self) -> ::windows::core::Result<i32>;
    fn SetHours(&mut self, ihours: i32) -> ::windows::core::Result<()>;
    fn HoursSpecified(&mut self) -> ::windows::core::Result<i16>;
    fn SetHoursSpecified(&mut self, bhoursspecified: i16) -> ::windows::core::Result<()>;
    fn Minutes(&mut self) -> ::windows::core::Result<i32>;
    fn SetMinutes(&mut self, iminutes: i32) -> ::windows::core::Result<()>;
    fn MinutesSpecified(&mut self) -> ::windows::core::Result<i16>;
    fn SetMinutesSpecified(&mut self, bminutesspecified: i16) -> ::windows::core::Result<()>;
    fn Seconds(&mut self) -> ::windows::core::Result<i32>;
    fn SetSeconds(&mut self, iseconds: i32) -> ::windows::core::Result<()>;
    fn SecondsSpecified(&mut self) -> ::windows::core::Result<i16>;
    fn SetSecondsSpecified(&mut self, bsecondsspecified: i16) -> ::windows::core::Result<()>;
    fn Microseconds(&mut self) -> ::windows::core::Result<i32>;
    fn SetMicroseconds(&mut self, imicroseconds: i32) -> ::windows::core::Result<()>;
    fn MicrosecondsSpecified(&mut self) -> ::windows::core::Result<i16>;
    fn SetMicrosecondsSpecified(&mut self, bmicrosecondsspecified: i16) -> ::windows::core::Result<()>;
    fn UTC(&mut self) -> ::windows::core::Result<i32>;
    fn SetUTC(&mut self, iutc: i32) -> ::windows::core::Result<()>;
    fn UTCSpecified(&mut self) -> ::windows::core::Result<i16>;
    fn SetUTCSpecified(&mut self, butcspecified: i16) -> ::windows::core::Result<()>;
    fn IsInterval(&mut self) -> ::windows::core::Result<i16>;
    fn SetIsInterval(&mut self, bisinterval: i16) -> ::windows::core::Result<()>;
    fn GetVarDate(&mut self, bislocal: i16) -> ::windows::core::Result<f64>;
    fn SetVarDate(&mut self, dvardate: f64, bislocal: i16) -> ::windows::core::Result<()>;
    fn GetFileTime(&mut self, bislocal: i16) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetFileTime(&mut self, strfiletime: super::super::Foundation::BSTR, bislocal: i16) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISWbemDateTimeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISWbemDateTimeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISWbemDateTimeVtbl {
        unsafe extern "system" fn Value<Impl: ISWbemDateTimeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strvalue: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Value() {
                ::core::result::Result::Ok(ok__) => {
                    *strvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValue<Impl: ISWbemDateTimeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strvalue: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetValue(::core::mem::transmute_copy(&strvalue)).into()
        }
        unsafe extern "system" fn Year<Impl: ISWbemDateTimeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iyear: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Year() {
                ::core::result::Result::Ok(ok__) => {
                    *iyear = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetYear<Impl: ISWbemDateTimeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iyear: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetYear(::core::mem::transmute_copy(&iyear)).into()
        }
        unsafe extern "system" fn YearSpecified<Impl: ISWbemDateTimeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, byearspecified: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).YearSpecified() {
                ::core::result::Result::Ok(ok__) => {
                    *byearspecified = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetYearSpecified<Impl: ISWbemDateTimeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, byearspecified: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetYearSpecified(::core::mem::transmute_copy(&byearspecified)).into()
        }
        unsafe extern "system" fn Month<Impl: ISWbemDateTimeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, imonth: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Month() {
                ::core::result::Result::Ok(ok__) => {
                    *imonth = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMonth<Impl: ISWbemDateTimeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, imonth: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMonth(::core::mem::transmute_copy(&imonth)).into()
        }
        unsafe extern "system" fn MonthSpecified<Impl: ISWbemDateTimeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bmonthspecified: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MonthSpecified() {
                ::core::result::Result::Ok(ok__) => {
                    *bmonthspecified = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMonthSpecified<Impl: ISWbemDateTimeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bmonthspecified: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMonthSpecified(::core::mem::transmute_copy(&bmonthspecified)).into()
        }
        unsafe extern "system" fn Day<Impl: ISWbemDateTimeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iday: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Day() {
                ::core::result::Result::Ok(ok__) => {
                    *iday = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDay<Impl: ISWbemDateTimeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iday: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDay(::core::mem::transmute_copy(&iday)).into()
        }
        unsafe extern "system" fn DaySpecified<Impl: ISWbemDateTimeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bdayspecified: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DaySpecified() {
                ::core::result::Result::Ok(ok__) => {
                    *bdayspecified = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDaySpecified<Impl: ISWbemDateTimeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bdayspecified: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDaySpecified(::core::mem::transmute_copy(&bdayspecified)).into()
        }
        unsafe extern "system" fn Hours<Impl: ISWbemDateTimeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ihours: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Hours() {
                ::core::result::Result::Ok(ok__) => {
                    *ihours = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHours<Impl: ISWbemDateTimeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ihours: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHours(::core::mem::transmute_copy(&ihours)).into()
        }
        unsafe extern "system" fn HoursSpecified<Impl: ISWbemDateTimeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bhoursspecified: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HoursSpecified() {
                ::core::result::Result::Ok(ok__) => {
                    *bhoursspecified = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHoursSpecified<Impl: ISWbemDateTimeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bhoursspecified: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHoursSpecified(::core::mem::transmute_copy(&bhoursspecified)).into()
        }
        unsafe extern "system" fn Minutes<Impl: ISWbemDateTimeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iminutes: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Minutes() {
                ::core::result::Result::Ok(ok__) => {
                    *iminutes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMinutes<Impl: ISWbemDateTimeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iminutes: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMinutes(::core::mem::transmute_copy(&iminutes)).into()
        }
        unsafe extern "system" fn MinutesSpecified<Impl: ISWbemDateTimeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bminutesspecified: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MinutesSpecified() {
                ::core::result::Result::Ok(ok__) => {
                    *bminutesspecified = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMinutesSpecified<Impl: ISWbemDateTimeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bminutesspecified: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMinutesSpecified(::core::mem::transmute_copy(&bminutesspecified)).into()
        }
        unsafe extern "system" fn Seconds<Impl: ISWbemDateTimeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iseconds: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Seconds() {
                ::core::result::Result::Ok(ok__) => {
                    *iseconds = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSeconds<Impl: ISWbemDateTimeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iseconds: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSeconds(::core::mem::transmute_copy(&iseconds)).into()
        }
        unsafe extern "system" fn SecondsSpecified<Impl: ISWbemDateTimeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bsecondsspecified: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SecondsSpecified() {
                ::core::result::Result::Ok(ok__) => {
                    *bsecondsspecified = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSecondsSpecified<Impl: ISWbemDateTimeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bsecondsspecified: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSecondsSpecified(::core::mem::transmute_copy(&bsecondsspecified)).into()
        }
        unsafe extern "system" fn Microseconds<Impl: ISWbemDateTimeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, imicroseconds: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Microseconds() {
                ::core::result::Result::Ok(ok__) => {
                    *imicroseconds = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMicroseconds<Impl: ISWbemDateTimeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, imicroseconds: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMicroseconds(::core::mem::transmute_copy(&imicroseconds)).into()
        }
        unsafe extern "system" fn MicrosecondsSpecified<Impl: ISWbemDateTimeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bmicrosecondsspecified: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MicrosecondsSpecified() {
                ::core::result::Result::Ok(ok__) => {
                    *bmicrosecondsspecified = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMicrosecondsSpecified<Impl: ISWbemDateTimeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bmicrosecondsspecified: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMicrosecondsSpecified(::core::mem::transmute_copy(&bmicrosecondsspecified)).into()
        }
        unsafe extern "system" fn UTC<Impl: ISWbemDateTimeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iutc: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UTC() {
                ::core::result::Result::Ok(ok__) => {
                    *iutc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUTC<Impl: ISWbemDateTimeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iutc: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetUTC(::core::mem::transmute_copy(&iutc)).into()
        }
        unsafe extern "system" fn UTCSpecified<Impl: ISWbemDateTimeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, butcspecified: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UTCSpecified() {
                ::core::result::Result::Ok(ok__) => {
                    *butcspecified = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUTCSpecified<Impl: ISWbemDateTimeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, butcspecified: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetUTCSpecified(::core::mem::transmute_copy(&butcspecified)).into()
        }
        unsafe extern "system" fn IsInterval<Impl: ISWbemDateTimeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bisinterval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsInterval() {
                ::core::result::Result::Ok(ok__) => {
                    *bisinterval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsInterval<Impl: ISWbemDateTimeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bisinterval: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsInterval(::core::mem::transmute_copy(&bisinterval)).into()
        }
        unsafe extern "system" fn GetVarDate<Impl: ISWbemDateTimeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bislocal: i16, dvardate: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetVarDate(::core::mem::transmute_copy(&bislocal)) {
                ::core::result::Result::Ok(ok__) => {
                    *dvardate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVarDate<Impl: ISWbemDateTimeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dvardate: f64, bislocal: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetVarDate(::core::mem::transmute_copy(&dvardate), ::core::mem::transmute_copy(&bislocal)).into()
        }
        unsafe extern "system" fn GetFileTime<Impl: ISWbemDateTimeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bislocal: i16, strfiletime: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFileTime(::core::mem::transmute_copy(&bislocal)) {
                ::core::result::Result::Ok(ok__) => {
                    *strfiletime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFileTime<Impl: ISWbemDateTimeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strfiletime: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bislocal: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFileTime(::core::mem::transmute_copy(&strfiletime), ::core::mem::transmute_copy(&bislocal)).into()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Value: Value::<Impl, IMPL_OFFSET>,
            SetValue: SetValue::<Impl, IMPL_OFFSET>,
            Year: Year::<Impl, IMPL_OFFSET>,
            SetYear: SetYear::<Impl, IMPL_OFFSET>,
            YearSpecified: YearSpecified::<Impl, IMPL_OFFSET>,
            SetYearSpecified: SetYearSpecified::<Impl, IMPL_OFFSET>,
            Month: Month::<Impl, IMPL_OFFSET>,
            SetMonth: SetMonth::<Impl, IMPL_OFFSET>,
            MonthSpecified: MonthSpecified::<Impl, IMPL_OFFSET>,
            SetMonthSpecified: SetMonthSpecified::<Impl, IMPL_OFFSET>,
            Day: Day::<Impl, IMPL_OFFSET>,
            SetDay: SetDay::<Impl, IMPL_OFFSET>,
            DaySpecified: DaySpecified::<Impl, IMPL_OFFSET>,
            SetDaySpecified: SetDaySpecified::<Impl, IMPL_OFFSET>,
            Hours: Hours::<Impl, IMPL_OFFSET>,
            SetHours: SetHours::<Impl, IMPL_OFFSET>,
            HoursSpecified: HoursSpecified::<Impl, IMPL_OFFSET>,
            SetHoursSpecified: SetHoursSpecified::<Impl, IMPL_OFFSET>,
            Minutes: Minutes::<Impl, IMPL_OFFSET>,
            SetMinutes: SetMinutes::<Impl, IMPL_OFFSET>,
            MinutesSpecified: MinutesSpecified::<Impl, IMPL_OFFSET>,
            SetMinutesSpecified: SetMinutesSpecified::<Impl, IMPL_OFFSET>,
            Seconds: Seconds::<Impl, IMPL_OFFSET>,
            SetSeconds: SetSeconds::<Impl, IMPL_OFFSET>,
            SecondsSpecified: SecondsSpecified::<Impl, IMPL_OFFSET>,
            SetSecondsSpecified: SetSecondsSpecified::<Impl, IMPL_OFFSET>,
            Microseconds: Microseconds::<Impl, IMPL_OFFSET>,
            SetMicroseconds: SetMicroseconds::<Impl, IMPL_OFFSET>,
            MicrosecondsSpecified: MicrosecondsSpecified::<Impl, IMPL_OFFSET>,
            SetMicrosecondsSpecified: SetMicrosecondsSpecified::<Impl, IMPL_OFFSET>,
            UTC: UTC::<Impl, IMPL_OFFSET>,
            SetUTC: SetUTC::<Impl, IMPL_OFFSET>,
            UTCSpecified: UTCSpecified::<Impl, IMPL_OFFSET>,
            SetUTCSpecified: SetUTCSpecified::<Impl, IMPL_OFFSET>,
            IsInterval: IsInterval::<Impl, IMPL_OFFSET>,
            SetIsInterval: SetIsInterval::<Impl, IMPL_OFFSET>,
            GetVarDate: GetVarDate::<Impl, IMPL_OFFSET>,
            SetVarDate: SetVarDate::<Impl, IMPL_OFFSET>,
            GetFileTime: GetFileTime::<Impl, IMPL_OFFSET>,
            SetFileTime: SetFileTime::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISWbemDateTime as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISWbemEventSourceImpl: Sized + IDispatchImpl {
    fn NextEvent(&mut self, itimeoutms: i32) -> ::windows::core::Result<ISWbemObject>;
    fn Security_(&mut self) -> ::windows::core::Result<ISWbemSecurity>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISWbemEventSourceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISWbemEventSourceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISWbemEventSourceVtbl {
        unsafe extern "system" fn NextEvent<Impl: ISWbemEventSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, itimeoutms: i32, objwbemobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NextEvent(::core::mem::transmute_copy(&itimeoutms)) {
                ::core::result::Result::Ok(ok__) => {
                    *objwbemobject = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Security_<Impl: ISWbemEventSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objwbemsecurity: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Security_() {
                ::core::result::Result::Ok(ok__) => {
                    *objwbemsecurity = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            NextEvent: NextEvent::<Impl, IMPL_OFFSET>,
            Security_: Security_::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISWbemEventSource as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISWbemLastErrorImpl: Sized + IDispatchImpl + ISWbemObjectImpl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISWbemLastErrorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISWbemLastErrorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISWbemLastErrorVtbl {
        Self { base: ISWbemObjectVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISWbemLastError as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISWbemLocatorImpl: Sized + IDispatchImpl {
    fn ConnectServer(&mut self, strserver: super::super::Foundation::BSTR, strnamespace: super::super::Foundation::BSTR, struser: super::super::Foundation::BSTR, strpassword: super::super::Foundation::BSTR, strlocale: super::super::Foundation::BSTR, strauthority: super::super::Foundation::BSTR, isecurityflags: i32, objwbemnamedvalueset: ::core::option::Option<super::Com::IDispatch>) -> ::windows::core::Result<ISWbemServices>;
    fn Security_(&mut self) -> ::windows::core::Result<ISWbemSecurity>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISWbemLocatorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISWbemLocatorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISWbemLocatorVtbl {
        unsafe extern "system" fn ConnectServer<Impl: ISWbemLocatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strserver: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strnamespace: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, struser: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strlocale: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strauthority: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, isecurityflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemservices: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConnectServer(::core::mem::transmute_copy(&strserver), ::core::mem::transmute_copy(&strnamespace), ::core::mem::transmute_copy(&struser), ::core::mem::transmute_copy(&strpassword), ::core::mem::transmute_copy(&strlocale), ::core::mem::transmute_copy(&strauthority), ::core::mem::transmute_copy(&isecurityflags), ::core::mem::transmute(&objwbemnamedvalueset)) {
                ::core::result::Result::Ok(ok__) => {
                    *objwbemservices = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Security_<Impl: ISWbemLocatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objwbemsecurity: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Security_() {
                ::core::result::Result::Ok(ok__) => {
                    *objwbemsecurity = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            ConnectServer: ConnectServer::<Impl, IMPL_OFFSET>,
            Security_: Security_::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISWbemLocator as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISWbemMethodImpl: Sized + IDispatchImpl {
    fn Name(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Origin(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn InParameters(&mut self) -> ::windows::core::Result<ISWbemObject>;
    fn OutParameters(&mut self) -> ::windows::core::Result<ISWbemObject>;
    fn Qualifiers_(&mut self) -> ::windows::core::Result<ISWbemQualifierSet>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISWbemMethodVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISWbemMethodImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISWbemMethodVtbl {
        unsafe extern "system" fn Name<Impl: ISWbemMethodImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *strname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Origin<Impl: ISWbemMethodImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strorigin: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Origin() {
                ::core::result::Result::Ok(ok__) => {
                    *strorigin = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InParameters<Impl: ISWbemMethodImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objwbeminparameters: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InParameters() {
                ::core::result::Result::Ok(ok__) => {
                    *objwbeminparameters = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OutParameters<Impl: ISWbemMethodImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objwbemoutparameters: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OutParameters() {
                ::core::result::Result::Ok(ok__) => {
                    *objwbemoutparameters = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Qualifiers_<Impl: ISWbemMethodImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objwbemqualifierset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Qualifiers_() {
                ::core::result::Result::Ok(ok__) => {
                    *objwbemqualifierset = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Name: Name::<Impl, IMPL_OFFSET>,
            Origin: Origin::<Impl, IMPL_OFFSET>,
            InParameters: InParameters::<Impl, IMPL_OFFSET>,
            OutParameters: OutParameters::<Impl, IMPL_OFFSET>,
            Qualifiers_: Qualifiers_::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISWbemMethod as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISWbemMethodSetImpl: Sized + IDispatchImpl {
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Item(&mut self, strname: super::super::Foundation::BSTR, iflags: i32) -> ::windows::core::Result<ISWbemMethod>;
    fn Count(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISWbemMethodSetVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISWbemMethodSetImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISWbemMethodSetVtbl {
        unsafe extern "system" fn _NewEnum<Impl: ISWbemMethodSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *punk = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: ISWbemMethodSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemmethod: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&strname), ::core::mem::transmute_copy(&iflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *objwbemmethod = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: ISWbemMethodSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, icount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *icount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
            Item: Item::<Impl, IMPL_OFFSET>,
            Count: Count::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISWbemMethodSet as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISWbemNamedValueImpl: Sized + IDispatchImpl {
    fn Value(&mut self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SetValue(&mut self, varvalue: *const super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Name(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISWbemNamedValueVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISWbemNamedValueImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISWbemNamedValueVtbl {
        unsafe extern "system" fn Value<Impl: ISWbemNamedValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varvalue: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Value() {
                ::core::result::Result::Ok(ok__) => {
                    *varvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValue<Impl: ISWbemNamedValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varvalue: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetValue(::core::mem::transmute_copy(&varvalue)).into()
        }
        unsafe extern "system" fn Name<Impl: ISWbemNamedValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *strname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Value: Value::<Impl, IMPL_OFFSET>,
            SetValue: SetValue::<Impl, IMPL_OFFSET>,
            Name: Name::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISWbemNamedValue as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISWbemNamedValueSetImpl: Sized + IDispatchImpl {
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Item(&mut self, strname: super::super::Foundation::BSTR, iflags: i32) -> ::windows::core::Result<ISWbemNamedValue>;
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn Add(&mut self, strname: super::super::Foundation::BSTR, varvalue: *const super::Com::VARIANT, iflags: i32) -> ::windows::core::Result<ISWbemNamedValue>;
    fn Remove(&mut self, strname: super::super::Foundation::BSTR, iflags: i32) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<ISWbemNamedValueSet>;
    fn DeleteAll(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISWbemNamedValueSetVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISWbemNamedValueSetImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISWbemNamedValueSetVtbl {
        unsafe extern "system" fn _NewEnum<Impl: ISWbemNamedValueSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *punk = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: ISWbemNamedValueSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemnamedvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&strname), ::core::mem::transmute_copy(&iflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *objwbemnamedvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: ISWbemNamedValueSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, icount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *icount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Impl: ISWbemNamedValueSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varvalue: *const super::Com::VARIANT, iflags: i32, objwbemnamedvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Add(::core::mem::transmute_copy(&strname), ::core::mem::transmute_copy(&varvalue), ::core::mem::transmute_copy(&iflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *objwbemnamedvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Remove<Impl: ISWbemNamedValueSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Remove(::core::mem::transmute_copy(&strname), ::core::mem::transmute_copy(&iflags)).into()
        }
        unsafe extern "system" fn Clone<Impl: ISWbemNamedValueSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objwbemnamedvalueset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *objwbemnamedvalueset = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteAll<Impl: ISWbemNamedValueSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteAll().into()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
            Item: Item::<Impl, IMPL_OFFSET>,
            Count: Count::<Impl, IMPL_OFFSET>,
            Add: Add::<Impl, IMPL_OFFSET>,
            Remove: Remove::<Impl, IMPL_OFFSET>,
            Clone: Clone::<Impl, IMPL_OFFSET>,
            DeleteAll: DeleteAll::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISWbemNamedValueSet as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISWbemObjectImpl: Sized + IDispatchImpl {
    fn Put_(&mut self, iflags: i32, objwbemnamedvalueset: ::core::option::Option<super::Com::IDispatch>) -> ::windows::core::Result<ISWbemObjectPath>;
    fn PutAsync_(&mut self, objwbemsink: ::core::option::Option<super::Com::IDispatch>, iflags: i32, objwbemnamedvalueset: ::core::option::Option<super::Com::IDispatch>, objwbemasynccontext: ::core::option::Option<super::Com::IDispatch>) -> ::windows::core::Result<()>;
    fn Delete_(&mut self, iflags: i32, objwbemnamedvalueset: ::core::option::Option<super::Com::IDispatch>) -> ::windows::core::Result<()>;
    fn DeleteAsync_(&mut self, objwbemsink: ::core::option::Option<super::Com::IDispatch>, iflags: i32, objwbemnamedvalueset: ::core::option::Option<super::Com::IDispatch>, objwbemasynccontext: ::core::option::Option<super::Com::IDispatch>) -> ::windows::core::Result<()>;
    fn Instances_(&mut self, iflags: i32, objwbemnamedvalueset: ::core::option::Option<super::Com::IDispatch>) -> ::windows::core::Result<ISWbemObjectSet>;
    fn InstancesAsync_(&mut self, objwbemsink: ::core::option::Option<super::Com::IDispatch>, iflags: i32, objwbemnamedvalueset: ::core::option::Option<super::Com::IDispatch>, objwbemasynccontext: ::core::option::Option<super::Com::IDispatch>) -> ::windows::core::Result<()>;
    fn Subclasses_(&mut self, iflags: i32, objwbemnamedvalueset: ::core::option::Option<super::Com::IDispatch>) -> ::windows::core::Result<ISWbemObjectSet>;
    fn SubclassesAsync_(&mut self, objwbemsink: ::core::option::Option<super::Com::IDispatch>, iflags: i32, objwbemnamedvalueset: ::core::option::Option<super::Com::IDispatch>, objwbemasynccontext: ::core::option::Option<super::Com::IDispatch>) -> ::windows::core::Result<()>;
    fn Associators_(&mut self, strassocclass: super::super::Foundation::BSTR, strresultclass: super::super::Foundation::BSTR, strresultrole: super::super::Foundation::BSTR, strrole: super::super::Foundation::BSTR, bclassesonly: i16, bschemaonly: i16, strrequiredassocqualifier: super::super::Foundation::BSTR, strrequiredqualifier: super::super::Foundation::BSTR, iflags: i32, objwbemnamedvalueset: ::core::option::Option<super::Com::IDispatch>) -> ::windows::core::Result<ISWbemObjectSet>;
    fn AssociatorsAsync_(&mut self, objwbemsink: ::core::option::Option<super::Com::IDispatch>, strassocclass: super::super::Foundation::BSTR, strresultclass: super::super::Foundation::BSTR, strresultrole: super::super::Foundation::BSTR, strrole: super::super::Foundation::BSTR, bclassesonly: i16, bschemaonly: i16, strrequiredassocqualifier: super::super::Foundation::BSTR, strrequiredqualifier: super::super::Foundation::BSTR, iflags: i32, objwbemnamedvalueset: ::core::option::Option<super::Com::IDispatch>, objwbemasynccontext: ::core::option::Option<super::Com::IDispatch>) -> ::windows::core::Result<()>;
    fn References_(&mut self, strresultclass: super::super::Foundation::BSTR, strrole: super::super::Foundation::BSTR, bclassesonly: i16, bschemaonly: i16, strrequiredqualifier: super::super::Foundation::BSTR, iflags: i32, objwbemnamedvalueset: ::core::option::Option<super::Com::IDispatch>) -> ::windows::core::Result<ISWbemObjectSet>;
    fn ReferencesAsync_(&mut self, objwbemsink: ::core::option::Option<super::Com::IDispatch>, strresultclass: super::super::Foundation::BSTR, strrole: super::super::Foundation::BSTR, bclassesonly: i16, bschemaonly: i16, strrequiredqualifier: super::super::Foundation::BSTR, iflags: i32, objwbemnamedvalueset: ::core::option::Option<super::Com::IDispatch>, objwbemasynccontext: ::core::option::Option<super::Com::IDispatch>) -> ::windows::core::Result<()>;
    fn ExecMethod_(&mut self, strmethodname: super::super::Foundation::BSTR, objwbeminparameters: ::core::option::Option<super::Com::IDispatch>, iflags: i32, objwbemnamedvalueset: ::core::option::Option<super::Com::IDispatch>) -> ::windows::core::Result<ISWbemObject>;
    fn ExecMethodAsync_(&mut self, objwbemsink: ::core::option::Option<super::Com::IDispatch>, strmethodname: super::super::Foundation::BSTR, objwbeminparameters: ::core::option::Option<super::Com::IDispatch>, iflags: i32, objwbemnamedvalueset: ::core::option::Option<super::Com::IDispatch>, objwbemasynccontext: ::core::option::Option<super::Com::IDispatch>) -> ::windows::core::Result<()>;
    fn Clone_(&mut self) -> ::windows::core::Result<ISWbemObject>;
    fn GetObjectText_(&mut self, iflags: i32) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SpawnDerivedClass_(&mut self, iflags: i32) -> ::windows::core::Result<ISWbemObject>;
    fn SpawnInstance_(&mut self, iflags: i32) -> ::windows::core::Result<ISWbemObject>;
    fn CompareTo_(&mut self, objwbemobject: ::core::option::Option<super::Com::IDispatch>, iflags: i32) -> ::windows::core::Result<i16>;
    fn Qualifiers_(&mut self) -> ::windows::core::Result<ISWbemQualifierSet>;
    fn Properties_(&mut self) -> ::windows::core::Result<ISWbemPropertySet>;
    fn Methods_(&mut self) -> ::windows::core::Result<ISWbemMethodSet>;
    fn Derivation_(&mut self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn Path_(&mut self) -> ::windows::core::Result<ISWbemObjectPath>;
    fn Security_(&mut self) -> ::windows::core::Result<ISWbemSecurity>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISWbemObjectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISWbemObjectImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISWbemObjectVtbl {
        unsafe extern "system" fn Put_<Impl: ISWbemObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemobjectpath: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Put_(::core::mem::transmute_copy(&iflags), ::core::mem::transmute(&objwbemnamedvalueset)) {
                ::core::result::Result::Ok(ok__) => {
                    *objwbemobjectpath = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PutAsync_<Impl: ISWbemObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objwbemsink: ::windows::core::RawPtr, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemasynccontext: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PutAsync_(::core::mem::transmute(&objwbemsink), ::core::mem::transmute_copy(&iflags), ::core::mem::transmute(&objwbemnamedvalueset), ::core::mem::transmute(&objwbemasynccontext)).into()
        }
        unsafe extern "system" fn Delete_<Impl: ISWbemObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Delete_(::core::mem::transmute_copy(&iflags), ::core::mem::transmute(&objwbemnamedvalueset)).into()
        }
        unsafe extern "system" fn DeleteAsync_<Impl: ISWbemObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objwbemsink: ::windows::core::RawPtr, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemasynccontext: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteAsync_(::core::mem::transmute(&objwbemsink), ::core::mem::transmute_copy(&iflags), ::core::mem::transmute(&objwbemnamedvalueset), ::core::mem::transmute(&objwbemasynccontext)).into()
        }
        unsafe extern "system" fn Instances_<Impl: ISWbemObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemobjectset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Instances_(::core::mem::transmute_copy(&iflags), ::core::mem::transmute(&objwbemnamedvalueset)) {
                ::core::result::Result::Ok(ok__) => {
                    *objwbemobjectset = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InstancesAsync_<Impl: ISWbemObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objwbemsink: ::windows::core::RawPtr, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemasynccontext: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InstancesAsync_(::core::mem::transmute(&objwbemsink), ::core::mem::transmute_copy(&iflags), ::core::mem::transmute(&objwbemnamedvalueset), ::core::mem::transmute(&objwbemasynccontext)).into()
        }
        unsafe extern "system" fn Subclasses_<Impl: ISWbemObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemobjectset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Subclasses_(::core::mem::transmute_copy(&iflags), ::core::mem::transmute(&objwbemnamedvalueset)) {
                ::core::result::Result::Ok(ok__) => {
                    *objwbemobjectset = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SubclassesAsync_<Impl: ISWbemObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objwbemsink: ::windows::core::RawPtr, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemasynccontext: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SubclassesAsync_(::core::mem::transmute(&objwbemsink), ::core::mem::transmute_copy(&iflags), ::core::mem::transmute(&objwbemnamedvalueset), ::core::mem::transmute(&objwbemasynccontext)).into()
        }
        unsafe extern "system" fn Associators_<Impl: ISWbemObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strassocclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strresultclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strresultrole: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strrole: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bclassesonly: i16, bschemaonly: i16, strrequiredassocqualifier: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strrequiredqualifier: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemobjectset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Associators_(::core::mem::transmute_copy(&strassocclass), ::core::mem::transmute_copy(&strresultclass), ::core::mem::transmute_copy(&strresultrole), ::core::mem::transmute_copy(&strrole), ::core::mem::transmute_copy(&bclassesonly), ::core::mem::transmute_copy(&bschemaonly), ::core::mem::transmute_copy(&strrequiredassocqualifier), ::core::mem::transmute_copy(&strrequiredqualifier), ::core::mem::transmute_copy(&iflags), ::core::mem::transmute(&objwbemnamedvalueset)) {
                ::core::result::Result::Ok(ok__) => {
                    *objwbemobjectset = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AssociatorsAsync_<Impl: ISWbemObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objwbemsink: ::windows::core::RawPtr, strassocclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strresultclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strresultrole: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strrole: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bclassesonly: i16, bschemaonly: i16, strrequiredassocqualifier: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strrequiredqualifier: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemasynccontext: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .AssociatorsAsync_(
                    ::core::mem::transmute(&objwbemsink),
                    ::core::mem::transmute_copy(&strassocclass),
                    ::core::mem::transmute_copy(&strresultclass),
                    ::core::mem::transmute_copy(&strresultrole),
                    ::core::mem::transmute_copy(&strrole),
                    ::core::mem::transmute_copy(&bclassesonly),
                    ::core::mem::transmute_copy(&bschemaonly),
                    ::core::mem::transmute_copy(&strrequiredassocqualifier),
                    ::core::mem::transmute_copy(&strrequiredqualifier),
                    ::core::mem::transmute_copy(&iflags),
                    ::core::mem::transmute(&objwbemnamedvalueset),
                    ::core::mem::transmute(&objwbemasynccontext),
                )
                .into()
        }
        unsafe extern "system" fn References_<Impl: ISWbemObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strresultclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strrole: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bclassesonly: i16, bschemaonly: i16, strrequiredqualifier: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemobjectset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).References_(::core::mem::transmute_copy(&strresultclass), ::core::mem::transmute_copy(&strrole), ::core::mem::transmute_copy(&bclassesonly), ::core::mem::transmute_copy(&bschemaonly), ::core::mem::transmute_copy(&strrequiredqualifier), ::core::mem::transmute_copy(&iflags), ::core::mem::transmute(&objwbemnamedvalueset)) {
                ::core::result::Result::Ok(ok__) => {
                    *objwbemobjectset = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReferencesAsync_<Impl: ISWbemObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objwbemsink: ::windows::core::RawPtr, strresultclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strrole: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bclassesonly: i16, bschemaonly: i16, strrequiredqualifier: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemasynccontext: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReferencesAsync_(::core::mem::transmute(&objwbemsink), ::core::mem::transmute_copy(&strresultclass), ::core::mem::transmute_copy(&strrole), ::core::mem::transmute_copy(&bclassesonly), ::core::mem::transmute_copy(&bschemaonly), ::core::mem::transmute_copy(&strrequiredqualifier), ::core::mem::transmute_copy(&iflags), ::core::mem::transmute(&objwbemnamedvalueset), ::core::mem::transmute(&objwbemasynccontext)).into()
        }
        unsafe extern "system" fn ExecMethod_<Impl: ISWbemObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strmethodname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, objwbeminparameters: ::windows::core::RawPtr, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemoutparameters: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExecMethod_(::core::mem::transmute_copy(&strmethodname), ::core::mem::transmute(&objwbeminparameters), ::core::mem::transmute_copy(&iflags), ::core::mem::transmute(&objwbemnamedvalueset)) {
                ::core::result::Result::Ok(ok__) => {
                    *objwbemoutparameters = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExecMethodAsync_<Impl: ISWbemObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objwbemsink: ::windows::core::RawPtr, strmethodname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, objwbeminparameters: ::windows::core::RawPtr, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemasynccontext: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ExecMethodAsync_(::core::mem::transmute(&objwbemsink), ::core::mem::transmute_copy(&strmethodname), ::core::mem::transmute(&objwbeminparameters), ::core::mem::transmute_copy(&iflags), ::core::mem::transmute(&objwbemnamedvalueset), ::core::mem::transmute(&objwbemasynccontext)).into()
        }
        unsafe extern "system" fn Clone_<Impl: ISWbemObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objwbemobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone_() {
                ::core::result::Result::Ok(ok__) => {
                    *objwbemobject = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetObjectText_<Impl: ISWbemObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iflags: i32, strobjecttext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetObjectText_(::core::mem::transmute_copy(&iflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *strobjecttext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SpawnDerivedClass_<Impl: ISWbemObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iflags: i32, objwbemobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SpawnDerivedClass_(::core::mem::transmute_copy(&iflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *objwbemobject = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SpawnInstance_<Impl: ISWbemObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iflags: i32, objwbemobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SpawnInstance_(::core::mem::transmute_copy(&iflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *objwbemobject = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CompareTo_<Impl: ISWbemObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objwbemobject: ::windows::core::RawPtr, iflags: i32, bresult: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CompareTo_(::core::mem::transmute(&objwbemobject), ::core::mem::transmute_copy(&iflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *bresult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Qualifiers_<Impl: ISWbemObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objwbemqualifierset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Qualifiers_() {
                ::core::result::Result::Ok(ok__) => {
                    *objwbemqualifierset = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties_<Impl: ISWbemObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objwbempropertyset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Properties_() {
                ::core::result::Result::Ok(ok__) => {
                    *objwbempropertyset = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Methods_<Impl: ISWbemObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objwbemmethodset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Methods_() {
                ::core::result::Result::Ok(ok__) => {
                    *objwbemmethodset = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Derivation_<Impl: ISWbemObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strclassnamearray: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Derivation_() {
                ::core::result::Result::Ok(ok__) => {
                    *strclassnamearray = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Path_<Impl: ISWbemObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objwbemobjectpath: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Path_() {
                ::core::result::Result::Ok(ok__) => {
                    *objwbemobjectpath = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Security_<Impl: ISWbemObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objwbemsecurity: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Security_() {
                ::core::result::Result::Ok(ok__) => {
                    *objwbemsecurity = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Put_: Put_::<Impl, IMPL_OFFSET>,
            PutAsync_: PutAsync_::<Impl, IMPL_OFFSET>,
            Delete_: Delete_::<Impl, IMPL_OFFSET>,
            DeleteAsync_: DeleteAsync_::<Impl, IMPL_OFFSET>,
            Instances_: Instances_::<Impl, IMPL_OFFSET>,
            InstancesAsync_: InstancesAsync_::<Impl, IMPL_OFFSET>,
            Subclasses_: Subclasses_::<Impl, IMPL_OFFSET>,
            SubclassesAsync_: SubclassesAsync_::<Impl, IMPL_OFFSET>,
            Associators_: Associators_::<Impl, IMPL_OFFSET>,
            AssociatorsAsync_: AssociatorsAsync_::<Impl, IMPL_OFFSET>,
            References_: References_::<Impl, IMPL_OFFSET>,
            ReferencesAsync_: ReferencesAsync_::<Impl, IMPL_OFFSET>,
            ExecMethod_: ExecMethod_::<Impl, IMPL_OFFSET>,
            ExecMethodAsync_: ExecMethodAsync_::<Impl, IMPL_OFFSET>,
            Clone_: Clone_::<Impl, IMPL_OFFSET>,
            GetObjectText_: GetObjectText_::<Impl, IMPL_OFFSET>,
            SpawnDerivedClass_: SpawnDerivedClass_::<Impl, IMPL_OFFSET>,
            SpawnInstance_: SpawnInstance_::<Impl, IMPL_OFFSET>,
            CompareTo_: CompareTo_::<Impl, IMPL_OFFSET>,
            Qualifiers_: Qualifiers_::<Impl, IMPL_OFFSET>,
            Properties_: Properties_::<Impl, IMPL_OFFSET>,
            Methods_: Methods_::<Impl, IMPL_OFFSET>,
            Derivation_: Derivation_::<Impl, IMPL_OFFSET>,
            Path_: Path_::<Impl, IMPL_OFFSET>,
            Security_: Security_::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISWbemObject as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISWbemObjectExImpl: Sized + IDispatchImpl + ISWbemObjectImpl {
    fn Refresh_(&mut self, iflags: i32, objwbemnamedvalueset: ::core::option::Option<super::Com::IDispatch>) -> ::windows::core::Result<()>;
    fn SystemProperties_(&mut self) -> ::windows::core::Result<ISWbemPropertySet>;
    fn GetText_(&mut self, iobjecttextformat: WbemObjectTextFormatEnum, iflags: i32, objwbemnamedvalueset: ::core::option::Option<super::Com::IDispatch>) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetFromText_(&mut self, bstext: super::super::Foundation::BSTR, iobjecttextformat: WbemObjectTextFormatEnum, iflags: i32, objwbemnamedvalueset: ::core::option::Option<super::Com::IDispatch>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISWbemObjectExVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISWbemObjectExImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISWbemObjectExVtbl {
        unsafe extern "system" fn Refresh_<Impl: ISWbemObjectExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Refresh_(::core::mem::transmute_copy(&iflags), ::core::mem::transmute(&objwbemnamedvalueset)).into()
        }
        unsafe extern "system" fn SystemProperties_<Impl: ISWbemObjectExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objwbempropertyset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SystemProperties_() {
                ::core::result::Result::Ok(ok__) => {
                    *objwbempropertyset = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetText_<Impl: ISWbemObjectExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iobjecttextformat: WbemObjectTextFormatEnum, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, bstext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetText_(::core::mem::transmute_copy(&iobjecttextformat), ::core::mem::transmute_copy(&iflags), ::core::mem::transmute(&objwbemnamedvalueset)) {
                ::core::result::Result::Ok(ok__) => {
                    *bstext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFromText_<Impl: ISWbemObjectExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstext: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iobjecttextformat: WbemObjectTextFormatEnum, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFromText_(::core::mem::transmute_copy(&bstext), ::core::mem::transmute_copy(&iobjecttextformat), ::core::mem::transmute_copy(&iflags), ::core::mem::transmute(&objwbemnamedvalueset)).into()
        }
        Self {
            base: ISWbemObjectVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Refresh_: Refresh_::<Impl, IMPL_OFFSET>,
            SystemProperties_: SystemProperties_::<Impl, IMPL_OFFSET>,
            GetText_: GetText_::<Impl, IMPL_OFFSET>,
            SetFromText_: SetFromText_::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISWbemObjectEx as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISWbemObjectPathImpl: Sized + IDispatchImpl {
    fn Path(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetPath(&mut self, strpath: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn RelPath(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetRelPath(&mut self, strrelpath: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Server(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetServer(&mut self, strserver: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Namespace(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetNamespace(&mut self, strnamespace: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ParentNamespace(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn DisplayName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetDisplayName(&mut self, strdisplayname: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Class(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetClass(&mut self, strclass: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn IsClass(&mut self) -> ::windows::core::Result<i16>;
    fn SetAsClass(&mut self) -> ::windows::core::Result<()>;
    fn IsSingleton(&mut self) -> ::windows::core::Result<i16>;
    fn SetAsSingleton(&mut self) -> ::windows::core::Result<()>;
    fn Keys(&mut self) -> ::windows::core::Result<ISWbemNamedValueSet>;
    fn Security_(&mut self) -> ::windows::core::Result<ISWbemSecurity>;
    fn Locale(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetLocale(&mut self, strlocale: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Authority(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetAuthority(&mut self, strauthority: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISWbemObjectPathVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISWbemObjectPathImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISWbemObjectPathVtbl {
        unsafe extern "system" fn Path<Impl: ISWbemObjectPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strpath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Path() {
                ::core::result::Result::Ok(ok__) => {
                    *strpath = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPath<Impl: ISWbemObjectPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPath(::core::mem::transmute_copy(&strpath)).into()
        }
        unsafe extern "system" fn RelPath<Impl: ISWbemObjectPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strrelpath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RelPath() {
                ::core::result::Result::Ok(ok__) => {
                    *strrelpath = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRelPath<Impl: ISWbemObjectPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strrelpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRelPath(::core::mem::transmute_copy(&strrelpath)).into()
        }
        unsafe extern "system" fn Server<Impl: ISWbemObjectPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strserver: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Server() {
                ::core::result::Result::Ok(ok__) => {
                    *strserver = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetServer<Impl: ISWbemObjectPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strserver: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetServer(::core::mem::transmute_copy(&strserver)).into()
        }
        unsafe extern "system" fn Namespace<Impl: ISWbemObjectPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strnamespace: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Namespace() {
                ::core::result::Result::Ok(ok__) => {
                    *strnamespace = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNamespace<Impl: ISWbemObjectPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strnamespace: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNamespace(::core::mem::transmute_copy(&strnamespace)).into()
        }
        unsafe extern "system" fn ParentNamespace<Impl: ISWbemObjectPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strparentnamespace: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ParentNamespace() {
                ::core::result::Result::Ok(ok__) => {
                    *strparentnamespace = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplayName<Impl: ISWbemObjectPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strdisplayname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    *strdisplayname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisplayName<Impl: ISWbemObjectPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strdisplayname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDisplayName(::core::mem::transmute_copy(&strdisplayname)).into()
        }
        unsafe extern "system" fn Class<Impl: ISWbemObjectPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strclass: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Class() {
                ::core::result::Result::Ok(ok__) => {
                    *strclass = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClass<Impl: ISWbemObjectPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetClass(::core::mem::transmute_copy(&strclass)).into()
        }
        unsafe extern "system" fn IsClass<Impl: ISWbemObjectPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bisclass: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsClass() {
                ::core::result::Result::Ok(ok__) => {
                    *bisclass = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAsClass<Impl: ISWbemObjectPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAsClass().into()
        }
        unsafe extern "system" fn IsSingleton<Impl: ISWbemObjectPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bissingleton: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsSingleton() {
                ::core::result::Result::Ok(ok__) => {
                    *bissingleton = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAsSingleton<Impl: ISWbemObjectPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAsSingleton().into()
        }
        unsafe extern "system" fn Keys<Impl: ISWbemObjectPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objwbemnamedvalueset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Keys() {
                ::core::result::Result::Ok(ok__) => {
                    *objwbemnamedvalueset = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Security_<Impl: ISWbemObjectPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objwbemsecurity: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Security_() {
                ::core::result::Result::Ok(ok__) => {
                    *objwbemsecurity = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Locale<Impl: ISWbemObjectPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strlocale: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Locale() {
                ::core::result::Result::Ok(ok__) => {
                    *strlocale = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLocale<Impl: ISWbemObjectPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strlocale: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLocale(::core::mem::transmute_copy(&strlocale)).into()
        }
        unsafe extern "system" fn Authority<Impl: ISWbemObjectPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strauthority: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Authority() {
                ::core::result::Result::Ok(ok__) => {
                    *strauthority = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAuthority<Impl: ISWbemObjectPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strauthority: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAuthority(::core::mem::transmute_copy(&strauthority)).into()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Path: Path::<Impl, IMPL_OFFSET>,
            SetPath: SetPath::<Impl, IMPL_OFFSET>,
            RelPath: RelPath::<Impl, IMPL_OFFSET>,
            SetRelPath: SetRelPath::<Impl, IMPL_OFFSET>,
            Server: Server::<Impl, IMPL_OFFSET>,
            SetServer: SetServer::<Impl, IMPL_OFFSET>,
            Namespace: Namespace::<Impl, IMPL_OFFSET>,
            SetNamespace: SetNamespace::<Impl, IMPL_OFFSET>,
            ParentNamespace: ParentNamespace::<Impl, IMPL_OFFSET>,
            DisplayName: DisplayName::<Impl, IMPL_OFFSET>,
            SetDisplayName: SetDisplayName::<Impl, IMPL_OFFSET>,
            Class: Class::<Impl, IMPL_OFFSET>,
            SetClass: SetClass::<Impl, IMPL_OFFSET>,
            IsClass: IsClass::<Impl, IMPL_OFFSET>,
            SetAsClass: SetAsClass::<Impl, IMPL_OFFSET>,
            IsSingleton: IsSingleton::<Impl, IMPL_OFFSET>,
            SetAsSingleton: SetAsSingleton::<Impl, IMPL_OFFSET>,
            Keys: Keys::<Impl, IMPL_OFFSET>,
            Security_: Security_::<Impl, IMPL_OFFSET>,
            Locale: Locale::<Impl, IMPL_OFFSET>,
            SetLocale: SetLocale::<Impl, IMPL_OFFSET>,
            Authority: Authority::<Impl, IMPL_OFFSET>,
            SetAuthority: SetAuthority::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISWbemObjectPath as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISWbemObjectSetImpl: Sized + IDispatchImpl {
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Item(&mut self, strobjectpath: super::super::Foundation::BSTR, iflags: i32) -> ::windows::core::Result<ISWbemObject>;
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn Security_(&mut self) -> ::windows::core::Result<ISWbemSecurity>;
    fn ItemIndex(&mut self, lindex: i32) -> ::windows::core::Result<ISWbemObject>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISWbemObjectSetVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISWbemObjectSetImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISWbemObjectSetVtbl {
        unsafe extern "system" fn _NewEnum<Impl: ISWbemObjectSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *punk = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: ISWbemObjectSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strobjectpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&strobjectpath), ::core::mem::transmute_copy(&iflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *objwbemobject = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: ISWbemObjectSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, icount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *icount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Security_<Impl: ISWbemObjectSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objwbemsecurity: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Security_() {
                ::core::result::Result::Ok(ok__) => {
                    *objwbemsecurity = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ItemIndex<Impl: ISWbemObjectSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32, objwbemobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ItemIndex(::core::mem::transmute_copy(&lindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *objwbemobject = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
            Item: Item::<Impl, IMPL_OFFSET>,
            Count: Count::<Impl, IMPL_OFFSET>,
            Security_: Security_::<Impl, IMPL_OFFSET>,
            ItemIndex: ItemIndex::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISWbemObjectSet as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISWbemPrivilegeImpl: Sized + IDispatchImpl {
    fn IsEnabled(&mut self) -> ::windows::core::Result<i16>;
    fn SetIsEnabled(&mut self, bisenabled: i16) -> ::windows::core::Result<()>;
    fn Name(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn DisplayName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Identifier(&mut self) -> ::windows::core::Result<WbemPrivilegeEnum>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISWbemPrivilegeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISWbemPrivilegeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISWbemPrivilegeVtbl {
        unsafe extern "system" fn IsEnabled<Impl: ISWbemPrivilegeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bisenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *bisenabled = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsEnabled<Impl: ISWbemPrivilegeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bisenabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsEnabled(::core::mem::transmute_copy(&bisenabled)).into()
        }
        unsafe extern "system" fn Name<Impl: ISWbemPrivilegeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strdisplayname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *strdisplayname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplayName<Impl: ISWbemPrivilegeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strdisplayname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    *strdisplayname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Identifier<Impl: ISWbemPrivilegeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iprivilege: *mut WbemPrivilegeEnum) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Identifier() {
                ::core::result::Result::Ok(ok__) => {
                    *iprivilege = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            IsEnabled: IsEnabled::<Impl, IMPL_OFFSET>,
            SetIsEnabled: SetIsEnabled::<Impl, IMPL_OFFSET>,
            Name: Name::<Impl, IMPL_OFFSET>,
            DisplayName: DisplayName::<Impl, IMPL_OFFSET>,
            Identifier: Identifier::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISWbemPrivilege as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISWbemPrivilegeSetImpl: Sized + IDispatchImpl {
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Item(&mut self, iprivilege: WbemPrivilegeEnum) -> ::windows::core::Result<ISWbemPrivilege>;
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn Add(&mut self, iprivilege: WbemPrivilegeEnum, bisenabled: i16) -> ::windows::core::Result<ISWbemPrivilege>;
    fn Remove(&mut self, iprivilege: WbemPrivilegeEnum) -> ::windows::core::Result<()>;
    fn DeleteAll(&mut self) -> ::windows::core::Result<()>;
    fn AddAsString(&mut self, strprivilege: super::super::Foundation::BSTR, bisenabled: i16) -> ::windows::core::Result<ISWbemPrivilege>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISWbemPrivilegeSetVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISWbemPrivilegeSetImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISWbemPrivilegeSetVtbl {
        unsafe extern "system" fn _NewEnum<Impl: ISWbemPrivilegeSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *punk = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: ISWbemPrivilegeSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iprivilege: WbemPrivilegeEnum, objwbemprivilege: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&iprivilege)) {
                ::core::result::Result::Ok(ok__) => {
                    *objwbemprivilege = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: ISWbemPrivilegeSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, icount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *icount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Impl: ISWbemPrivilegeSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iprivilege: WbemPrivilegeEnum, bisenabled: i16, objwbemprivilege: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Add(::core::mem::transmute_copy(&iprivilege), ::core::mem::transmute_copy(&bisenabled)) {
                ::core::result::Result::Ok(ok__) => {
                    *objwbemprivilege = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Remove<Impl: ISWbemPrivilegeSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iprivilege: WbemPrivilegeEnum) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Remove(::core::mem::transmute_copy(&iprivilege)).into()
        }
        unsafe extern "system" fn DeleteAll<Impl: ISWbemPrivilegeSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteAll().into()
        }
        unsafe extern "system" fn AddAsString<Impl: ISWbemPrivilegeSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strprivilege: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bisenabled: i16, objwbemprivilege: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddAsString(::core::mem::transmute_copy(&strprivilege), ::core::mem::transmute_copy(&bisenabled)) {
                ::core::result::Result::Ok(ok__) => {
                    *objwbemprivilege = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
            Item: Item::<Impl, IMPL_OFFSET>,
            Count: Count::<Impl, IMPL_OFFSET>,
            Add: Add::<Impl, IMPL_OFFSET>,
            Remove: Remove::<Impl, IMPL_OFFSET>,
            DeleteAll: DeleteAll::<Impl, IMPL_OFFSET>,
            AddAsString: AddAsString::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISWbemPrivilegeSet as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISWbemPropertyImpl: Sized + IDispatchImpl {
    fn Value(&mut self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SetValue(&mut self, varvalue: *const super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Name(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn IsLocal(&mut self) -> ::windows::core::Result<i16>;
    fn Origin(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn CIMType(&mut self) -> ::windows::core::Result<WbemCimtypeEnum>;
    fn Qualifiers_(&mut self) -> ::windows::core::Result<ISWbemQualifierSet>;
    fn IsArray(&mut self) -> ::windows::core::Result<i16>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISWbemPropertyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISWbemPropertyImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISWbemPropertyVtbl {
        unsafe extern "system" fn Value<Impl: ISWbemPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varvalue: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Value() {
                ::core::result::Result::Ok(ok__) => {
                    *varvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValue<Impl: ISWbemPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varvalue: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetValue(::core::mem::transmute_copy(&varvalue)).into()
        }
        unsafe extern "system" fn Name<Impl: ISWbemPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *strname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsLocal<Impl: ISWbemPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bislocal: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsLocal() {
                ::core::result::Result::Ok(ok__) => {
                    *bislocal = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Origin<Impl: ISWbemPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strorigin: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Origin() {
                ::core::result::Result::Ok(ok__) => {
                    *strorigin = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CIMType<Impl: ISWbemPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, icimtype: *mut WbemCimtypeEnum) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CIMType() {
                ::core::result::Result::Ok(ok__) => {
                    *icimtype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Qualifiers_<Impl: ISWbemPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objwbemqualifierset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Qualifiers_() {
                ::core::result::Result::Ok(ok__) => {
                    *objwbemqualifierset = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsArray<Impl: ISWbemPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bisarray: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsArray() {
                ::core::result::Result::Ok(ok__) => {
                    *bisarray = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Value: Value::<Impl, IMPL_OFFSET>,
            SetValue: SetValue::<Impl, IMPL_OFFSET>,
            Name: Name::<Impl, IMPL_OFFSET>,
            IsLocal: IsLocal::<Impl, IMPL_OFFSET>,
            Origin: Origin::<Impl, IMPL_OFFSET>,
            CIMType: CIMType::<Impl, IMPL_OFFSET>,
            Qualifiers_: Qualifiers_::<Impl, IMPL_OFFSET>,
            IsArray: IsArray::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISWbemProperty as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISWbemPropertySetImpl: Sized + IDispatchImpl {
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Item(&mut self, strname: super::super::Foundation::BSTR, iflags: i32) -> ::windows::core::Result<ISWbemProperty>;
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn Add(&mut self, strname: super::super::Foundation::BSTR, icimtype: WbemCimtypeEnum, bisarray: i16, iflags: i32) -> ::windows::core::Result<ISWbemProperty>;
    fn Remove(&mut self, strname: super::super::Foundation::BSTR, iflags: i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISWbemPropertySetVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISWbemPropertySetImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISWbemPropertySetVtbl {
        unsafe extern "system" fn _NewEnum<Impl: ISWbemPropertySetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *punk = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: ISWbemPropertySetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemproperty: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&strname), ::core::mem::transmute_copy(&iflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *objwbemproperty = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: ISWbemPropertySetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, icount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *icount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Impl: ISWbemPropertySetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, icimtype: WbemCimtypeEnum, bisarray: i16, iflags: i32, objwbemproperty: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Add(::core::mem::transmute_copy(&strname), ::core::mem::transmute_copy(&icimtype), ::core::mem::transmute_copy(&bisarray), ::core::mem::transmute_copy(&iflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *objwbemproperty = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Remove<Impl: ISWbemPropertySetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Remove(::core::mem::transmute_copy(&strname), ::core::mem::transmute_copy(&iflags)).into()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
            Item: Item::<Impl, IMPL_OFFSET>,
            Count: Count::<Impl, IMPL_OFFSET>,
            Add: Add::<Impl, IMPL_OFFSET>,
            Remove: Remove::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISWbemPropertySet as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISWbemQualifierImpl: Sized + IDispatchImpl {
    fn Value(&mut self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SetValue(&mut self, varvalue: *const super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Name(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn IsLocal(&mut self) -> ::windows::core::Result<i16>;
    fn PropagatesToSubclass(&mut self) -> ::windows::core::Result<i16>;
    fn SetPropagatesToSubclass(&mut self, bpropagatestosubclass: i16) -> ::windows::core::Result<()>;
    fn PropagatesToInstance(&mut self) -> ::windows::core::Result<i16>;
    fn SetPropagatesToInstance(&mut self, bpropagatestoinstance: i16) -> ::windows::core::Result<()>;
    fn IsOverridable(&mut self) -> ::windows::core::Result<i16>;
    fn SetIsOverridable(&mut self, bisoverridable: i16) -> ::windows::core::Result<()>;
    fn IsAmended(&mut self) -> ::windows::core::Result<i16>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISWbemQualifierVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISWbemQualifierImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISWbemQualifierVtbl {
        unsafe extern "system" fn Value<Impl: ISWbemQualifierImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varvalue: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Value() {
                ::core::result::Result::Ok(ok__) => {
                    *varvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValue<Impl: ISWbemQualifierImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varvalue: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetValue(::core::mem::transmute_copy(&varvalue)).into()
        }
        unsafe extern "system" fn Name<Impl: ISWbemQualifierImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *strname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsLocal<Impl: ISWbemQualifierImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bislocal: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsLocal() {
                ::core::result::Result::Ok(ok__) => {
                    *bislocal = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PropagatesToSubclass<Impl: ISWbemQualifierImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bpropagatestosubclass: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PropagatesToSubclass() {
                ::core::result::Result::Ok(ok__) => {
                    *bpropagatestosubclass = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPropagatesToSubclass<Impl: ISWbemQualifierImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bpropagatestosubclass: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPropagatesToSubclass(::core::mem::transmute_copy(&bpropagatestosubclass)).into()
        }
        unsafe extern "system" fn PropagatesToInstance<Impl: ISWbemQualifierImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bpropagatestoinstance: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PropagatesToInstance() {
                ::core::result::Result::Ok(ok__) => {
                    *bpropagatestoinstance = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPropagatesToInstance<Impl: ISWbemQualifierImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bpropagatestoinstance: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPropagatesToInstance(::core::mem::transmute_copy(&bpropagatestoinstance)).into()
        }
        unsafe extern "system" fn IsOverridable<Impl: ISWbemQualifierImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bisoverridable: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsOverridable() {
                ::core::result::Result::Ok(ok__) => {
                    *bisoverridable = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsOverridable<Impl: ISWbemQualifierImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bisoverridable: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsOverridable(::core::mem::transmute_copy(&bisoverridable)).into()
        }
        unsafe extern "system" fn IsAmended<Impl: ISWbemQualifierImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bisamended: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsAmended() {
                ::core::result::Result::Ok(ok__) => {
                    *bisamended = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Value: Value::<Impl, IMPL_OFFSET>,
            SetValue: SetValue::<Impl, IMPL_OFFSET>,
            Name: Name::<Impl, IMPL_OFFSET>,
            IsLocal: IsLocal::<Impl, IMPL_OFFSET>,
            PropagatesToSubclass: PropagatesToSubclass::<Impl, IMPL_OFFSET>,
            SetPropagatesToSubclass: SetPropagatesToSubclass::<Impl, IMPL_OFFSET>,
            PropagatesToInstance: PropagatesToInstance::<Impl, IMPL_OFFSET>,
            SetPropagatesToInstance: SetPropagatesToInstance::<Impl, IMPL_OFFSET>,
            IsOverridable: IsOverridable::<Impl, IMPL_OFFSET>,
            SetIsOverridable: SetIsOverridable::<Impl, IMPL_OFFSET>,
            IsAmended: IsAmended::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISWbemQualifier as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISWbemQualifierSetImpl: Sized + IDispatchImpl {
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Item(&mut self, name: super::super::Foundation::BSTR, iflags: i32) -> ::windows::core::Result<ISWbemQualifier>;
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn Add(&mut self, strname: super::super::Foundation::BSTR, varval: *const super::Com::VARIANT, bpropagatestosubclass: i16, bpropagatestoinstance: i16, bisoverridable: i16, iflags: i32) -> ::windows::core::Result<ISWbemQualifier>;
    fn Remove(&mut self, strname: super::super::Foundation::BSTR, iflags: i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISWbemQualifierSetVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISWbemQualifierSetImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISWbemQualifierSetVtbl {
        unsafe extern "system" fn _NewEnum<Impl: ISWbemQualifierSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *punk = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: ISWbemQualifierSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemqualifier: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&name), ::core::mem::transmute_copy(&iflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *objwbemqualifier = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: ISWbemQualifierSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, icount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *icount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Impl: ISWbemQualifierSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varval: *const super::Com::VARIANT, bpropagatestosubclass: i16, bpropagatestoinstance: i16, bisoverridable: i16, iflags: i32, objwbemqualifier: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Add(::core::mem::transmute_copy(&strname), ::core::mem::transmute_copy(&varval), ::core::mem::transmute_copy(&bpropagatestosubclass), ::core::mem::transmute_copy(&bpropagatestoinstance), ::core::mem::transmute_copy(&bisoverridable), ::core::mem::transmute_copy(&iflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *objwbemqualifier = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Remove<Impl: ISWbemQualifierSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Remove(::core::mem::transmute_copy(&strname), ::core::mem::transmute_copy(&iflags)).into()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
            Item: Item::<Impl, IMPL_OFFSET>,
            Count: Count::<Impl, IMPL_OFFSET>,
            Add: Add::<Impl, IMPL_OFFSET>,
            Remove: Remove::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISWbemQualifierSet as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISWbemRefreshableItemImpl: Sized + IDispatchImpl {
    fn Index(&mut self) -> ::windows::core::Result<i32>;
    fn Refresher(&mut self) -> ::windows::core::Result<ISWbemRefresher>;
    fn IsSet(&mut self) -> ::windows::core::Result<i16>;
    fn Object(&mut self) -> ::windows::core::Result<ISWbemObjectEx>;
    fn ObjectSet(&mut self) -> ::windows::core::Result<ISWbemObjectSet>;
    fn Remove(&mut self, iflags: i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISWbemRefreshableItemVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISWbemRefreshableItemImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISWbemRefreshableItemVtbl {
        unsafe extern "system" fn Index<Impl: ISWbemRefreshableItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iindex: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Index() {
                ::core::result::Result::Ok(ok__) => {
                    *iindex = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresher<Impl: ISWbemRefreshableItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objwbemrefresher: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Refresher() {
                ::core::result::Result::Ok(ok__) => {
                    *objwbemrefresher = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsSet<Impl: ISWbemRefreshableItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bisset: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsSet() {
                ::core::result::Result::Ok(ok__) => {
                    *bisset = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Object<Impl: ISWbemRefreshableItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objwbemobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Object() {
                ::core::result::Result::Ok(ok__) => {
                    *objwbemobject = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ObjectSet<Impl: ISWbemRefreshableItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objwbemobjectset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ObjectSet() {
                ::core::result::Result::Ok(ok__) => {
                    *objwbemobjectset = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Remove<Impl: ISWbemRefreshableItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Remove(::core::mem::transmute_copy(&iflags)).into()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Index: Index::<Impl, IMPL_OFFSET>,
            Refresher: Refresher::<Impl, IMPL_OFFSET>,
            IsSet: IsSet::<Impl, IMPL_OFFSET>,
            Object: Object::<Impl, IMPL_OFFSET>,
            ObjectSet: ObjectSet::<Impl, IMPL_OFFSET>,
            Remove: Remove::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISWbemRefreshableItem as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISWbemRefresherImpl: Sized + IDispatchImpl {
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Item(&mut self, iindex: i32) -> ::windows::core::Result<ISWbemRefreshableItem>;
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn Add(&mut self, objwbemservices: ::core::option::Option<ISWbemServicesEx>, bsinstancepath: super::super::Foundation::BSTR, iflags: i32, objwbemnamedvalueset: ::core::option::Option<super::Com::IDispatch>) -> ::windows::core::Result<ISWbemRefreshableItem>;
    fn AddEnum(&mut self, objwbemservices: ::core::option::Option<ISWbemServicesEx>, bsclassname: super::super::Foundation::BSTR, iflags: i32, objwbemnamedvalueset: ::core::option::Option<super::Com::IDispatch>) -> ::windows::core::Result<ISWbemRefreshableItem>;
    fn Remove(&mut self, iindex: i32, iflags: i32) -> ::windows::core::Result<()>;
    fn Refresh(&mut self, iflags: i32) -> ::windows::core::Result<()>;
    fn AutoReconnect(&mut self) -> ::windows::core::Result<i16>;
    fn SetAutoReconnect(&mut self, bcount: i16) -> ::windows::core::Result<()>;
    fn DeleteAll(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISWbemRefresherVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISWbemRefresherImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISWbemRefresherVtbl {
        unsafe extern "system" fn _NewEnum<Impl: ISWbemRefresherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *punk = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: ISWbemRefresherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iindex: i32, objwbemrefreshableitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&iindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *objwbemrefreshableitem = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: ISWbemRefresherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, icount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *icount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Impl: ISWbemRefresherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objwbemservices: ::windows::core::RawPtr, bsinstancepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemrefreshableitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Add(::core::mem::transmute(&objwbemservices), ::core::mem::transmute_copy(&bsinstancepath), ::core::mem::transmute_copy(&iflags), ::core::mem::transmute(&objwbemnamedvalueset)) {
                ::core::result::Result::Ok(ok__) => {
                    *objwbemrefreshableitem = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddEnum<Impl: ISWbemRefresherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objwbemservices: ::windows::core::RawPtr, bsclassname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemrefreshableitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddEnum(::core::mem::transmute(&objwbemservices), ::core::mem::transmute_copy(&bsclassname), ::core::mem::transmute_copy(&iflags), ::core::mem::transmute(&objwbemnamedvalueset)) {
                ::core::result::Result::Ok(ok__) => {
                    *objwbemrefreshableitem = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Remove<Impl: ISWbemRefresherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iindex: i32, iflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Remove(::core::mem::transmute_copy(&iindex), ::core::mem::transmute_copy(&iflags)).into()
        }
        unsafe extern "system" fn Refresh<Impl: ISWbemRefresherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Refresh(::core::mem::transmute_copy(&iflags)).into()
        }
        unsafe extern "system" fn AutoReconnect<Impl: ISWbemRefresherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bcount: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AutoReconnect() {
                ::core::result::Result::Ok(ok__) => {
                    *bcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoReconnect<Impl: ISWbemRefresherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bcount: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAutoReconnect(::core::mem::transmute_copy(&bcount)).into()
        }
        unsafe extern "system" fn DeleteAll<Impl: ISWbemRefresherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteAll().into()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
            Item: Item::<Impl, IMPL_OFFSET>,
            Count: Count::<Impl, IMPL_OFFSET>,
            Add: Add::<Impl, IMPL_OFFSET>,
            AddEnum: AddEnum::<Impl, IMPL_OFFSET>,
            Remove: Remove::<Impl, IMPL_OFFSET>,
            Refresh: Refresh::<Impl, IMPL_OFFSET>,
            AutoReconnect: AutoReconnect::<Impl, IMPL_OFFSET>,
            SetAutoReconnect: SetAutoReconnect::<Impl, IMPL_OFFSET>,
            DeleteAll: DeleteAll::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISWbemRefresher as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISWbemSecurityImpl: Sized + IDispatchImpl {
    fn ImpersonationLevel(&mut self) -> ::windows::core::Result<WbemImpersonationLevelEnum>;
    fn SetImpersonationLevel(&mut self, iimpersonationlevel: WbemImpersonationLevelEnum) -> ::windows::core::Result<()>;
    fn AuthenticationLevel(&mut self) -> ::windows::core::Result<WbemAuthenticationLevelEnum>;
    fn SetAuthenticationLevel(&mut self, iauthenticationlevel: WbemAuthenticationLevelEnum) -> ::windows::core::Result<()>;
    fn Privileges(&mut self) -> ::windows::core::Result<ISWbemPrivilegeSet>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISWbemSecurityVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISWbemSecurityImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISWbemSecurityVtbl {
        unsafe extern "system" fn ImpersonationLevel<Impl: ISWbemSecurityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iimpersonationlevel: *mut WbemImpersonationLevelEnum) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ImpersonationLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *iimpersonationlevel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetImpersonationLevel<Impl: ISWbemSecurityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iimpersonationlevel: WbemImpersonationLevelEnum) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetImpersonationLevel(::core::mem::transmute_copy(&iimpersonationlevel)).into()
        }
        unsafe extern "system" fn AuthenticationLevel<Impl: ISWbemSecurityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iauthenticationlevel: *mut WbemAuthenticationLevelEnum) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AuthenticationLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *iauthenticationlevel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAuthenticationLevel<Impl: ISWbemSecurityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iauthenticationlevel: WbemAuthenticationLevelEnum) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAuthenticationLevel(::core::mem::transmute_copy(&iauthenticationlevel)).into()
        }
        unsafe extern "system" fn Privileges<Impl: ISWbemSecurityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objwbemprivilegeset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Privileges() {
                ::core::result::Result::Ok(ok__) => {
                    *objwbemprivilegeset = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            ImpersonationLevel: ImpersonationLevel::<Impl, IMPL_OFFSET>,
            SetImpersonationLevel: SetImpersonationLevel::<Impl, IMPL_OFFSET>,
            AuthenticationLevel: AuthenticationLevel::<Impl, IMPL_OFFSET>,
            SetAuthenticationLevel: SetAuthenticationLevel::<Impl, IMPL_OFFSET>,
            Privileges: Privileges::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISWbemSecurity as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISWbemServicesImpl: Sized + IDispatchImpl {
    fn Get(&mut self, strobjectpath: super::super::Foundation::BSTR, iflags: i32, objwbemnamedvalueset: ::core::option::Option<super::Com::IDispatch>) -> ::windows::core::Result<ISWbemObject>;
    fn GetAsync(&mut self, objwbemsink: ::core::option::Option<super::Com::IDispatch>, strobjectpath: super::super::Foundation::BSTR, iflags: i32, objwbemnamedvalueset: ::core::option::Option<super::Com::IDispatch>, objwbemasynccontext: ::core::option::Option<super::Com::IDispatch>) -> ::windows::core::Result<()>;
    fn Delete(&mut self, strobjectpath: super::super::Foundation::BSTR, iflags: i32, objwbemnamedvalueset: ::core::option::Option<super::Com::IDispatch>) -> ::windows::core::Result<()>;
    fn DeleteAsync(&mut self, objwbemsink: ::core::option::Option<super::Com::IDispatch>, strobjectpath: super::super::Foundation::BSTR, iflags: i32, objwbemnamedvalueset: ::core::option::Option<super::Com::IDispatch>, objwbemasynccontext: ::core::option::Option<super::Com::IDispatch>) -> ::windows::core::Result<()>;
    fn InstancesOf(&mut self, strclass: super::super::Foundation::BSTR, iflags: i32, objwbemnamedvalueset: ::core::option::Option<super::Com::IDispatch>) -> ::windows::core::Result<ISWbemObjectSet>;
    fn InstancesOfAsync(&mut self, objwbemsink: ::core::option::Option<super::Com::IDispatch>, strclass: super::super::Foundation::BSTR, iflags: i32, objwbemnamedvalueset: ::core::option::Option<super::Com::IDispatch>, objwbemasynccontext: ::core::option::Option<super::Com::IDispatch>) -> ::windows::core::Result<()>;
    fn SubclassesOf(&mut self, strsuperclass: super::super::Foundation::BSTR, iflags: i32, objwbemnamedvalueset: ::core::option::Option<super::Com::IDispatch>) -> ::windows::core::Result<ISWbemObjectSet>;
    fn SubclassesOfAsync(&mut self, objwbemsink: ::core::option::Option<super::Com::IDispatch>, strsuperclass: super::super::Foundation::BSTR, iflags: i32, objwbemnamedvalueset: ::core::option::Option<super::Com::IDispatch>, objwbemasynccontext: ::core::option::Option<super::Com::IDispatch>) -> ::windows::core::Result<()>;
    fn ExecQuery(&mut self, strquery: super::super::Foundation::BSTR, strquerylanguage: super::super::Foundation::BSTR, iflags: i32, objwbemnamedvalueset: ::core::option::Option<super::Com::IDispatch>) -> ::windows::core::Result<ISWbemObjectSet>;
    fn ExecQueryAsync(&mut self, objwbemsink: ::core::option::Option<super::Com::IDispatch>, strquery: super::super::Foundation::BSTR, strquerylanguage: super::super::Foundation::BSTR, lflags: i32, objwbemnamedvalueset: ::core::option::Option<super::Com::IDispatch>, objwbemasynccontext: ::core::option::Option<super::Com::IDispatch>) -> ::windows::core::Result<()>;
    fn AssociatorsOf(&mut self, strobjectpath: super::super::Foundation::BSTR, strassocclass: super::super::Foundation::BSTR, strresultclass: super::super::Foundation::BSTR, strresultrole: super::super::Foundation::BSTR, strrole: super::super::Foundation::BSTR, bclassesonly: i16, bschemaonly: i16, strrequiredassocqualifier: super::super::Foundation::BSTR, strrequiredqualifier: super::super::Foundation::BSTR, iflags: i32, objwbemnamedvalueset: ::core::option::Option<super::Com::IDispatch>) -> ::windows::core::Result<ISWbemObjectSet>;
    fn AssociatorsOfAsync(&mut self, objwbemsink: ::core::option::Option<super::Com::IDispatch>, strobjectpath: super::super::Foundation::BSTR, strassocclass: super::super::Foundation::BSTR, strresultclass: super::super::Foundation::BSTR, strresultrole: super::super::Foundation::BSTR, strrole: super::super::Foundation::BSTR, bclassesonly: i16, bschemaonly: i16, strrequiredassocqualifier: super::super::Foundation::BSTR, strrequiredqualifier: super::super::Foundation::BSTR, iflags: i32, objwbemnamedvalueset: ::core::option::Option<super::Com::IDispatch>, objwbemasynccontext: ::core::option::Option<super::Com::IDispatch>) -> ::windows::core::Result<()>;
    fn ReferencesTo(&mut self, strobjectpath: super::super::Foundation::BSTR, strresultclass: super::super::Foundation::BSTR, strrole: super::super::Foundation::BSTR, bclassesonly: i16, bschemaonly: i16, strrequiredqualifier: super::super::Foundation::BSTR, iflags: i32, objwbemnamedvalueset: ::core::option::Option<super::Com::IDispatch>) -> ::windows::core::Result<ISWbemObjectSet>;
    fn ReferencesToAsync(&mut self, objwbemsink: ::core::option::Option<super::Com::IDispatch>, strobjectpath: super::super::Foundation::BSTR, strresultclass: super::super::Foundation::BSTR, strrole: super::super::Foundation::BSTR, bclassesonly: i16, bschemaonly: i16, strrequiredqualifier: super::super::Foundation::BSTR, iflags: i32, objwbemnamedvalueset: ::core::option::Option<super::Com::IDispatch>, objwbemasynccontext: ::core::option::Option<super::Com::IDispatch>) -> ::windows::core::Result<()>;
    fn ExecNotificationQuery(&mut self, strquery: super::super::Foundation::BSTR, strquerylanguage: super::super::Foundation::BSTR, iflags: i32, objwbemnamedvalueset: ::core::option::Option<super::Com::IDispatch>) -> ::windows::core::Result<ISWbemEventSource>;
    fn ExecNotificationQueryAsync(&mut self, objwbemsink: ::core::option::Option<super::Com::IDispatch>, strquery: super::super::Foundation::BSTR, strquerylanguage: super::super::Foundation::BSTR, iflags: i32, objwbemnamedvalueset: ::core::option::Option<super::Com::IDispatch>, objwbemasynccontext: ::core::option::Option<super::Com::IDispatch>) -> ::windows::core::Result<()>;
    fn ExecMethod(&mut self, strobjectpath: super::super::Foundation::BSTR, strmethodname: super::super::Foundation::BSTR, objwbeminparameters: ::core::option::Option<super::Com::IDispatch>, iflags: i32, objwbemnamedvalueset: ::core::option::Option<super::Com::IDispatch>) -> ::windows::core::Result<ISWbemObject>;
    fn ExecMethodAsync(&mut self, objwbemsink: ::core::option::Option<super::Com::IDispatch>, strobjectpath: super::super::Foundation::BSTR, strmethodname: super::super::Foundation::BSTR, objwbeminparameters: ::core::option::Option<super::Com::IDispatch>, iflags: i32, objwbemnamedvalueset: ::core::option::Option<super::Com::IDispatch>, objwbemasynccontext: ::core::option::Option<super::Com::IDispatch>) -> ::windows::core::Result<()>;
    fn Security_(&mut self) -> ::windows::core::Result<ISWbemSecurity>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISWbemServicesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISWbemServicesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISWbemServicesVtbl {
        unsafe extern "system" fn Get<Impl: ISWbemServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strobjectpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Get(::core::mem::transmute_copy(&strobjectpath), ::core::mem::transmute_copy(&iflags), ::core::mem::transmute(&objwbemnamedvalueset)) {
                ::core::result::Result::Ok(ok__) => {
                    *objwbemobject = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAsync<Impl: ISWbemServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objwbemsink: ::windows::core::RawPtr, strobjectpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemasynccontext: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetAsync(::core::mem::transmute(&objwbemsink), ::core::mem::transmute_copy(&strobjectpath), ::core::mem::transmute_copy(&iflags), ::core::mem::transmute(&objwbemnamedvalueset), ::core::mem::transmute(&objwbemasynccontext)).into()
        }
        unsafe extern "system" fn Delete<Impl: ISWbemServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strobjectpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Delete(::core::mem::transmute_copy(&strobjectpath), ::core::mem::transmute_copy(&iflags), ::core::mem::transmute(&objwbemnamedvalueset)).into()
        }
        unsafe extern "system" fn DeleteAsync<Impl: ISWbemServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objwbemsink: ::windows::core::RawPtr, strobjectpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemasynccontext: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteAsync(::core::mem::transmute(&objwbemsink), ::core::mem::transmute_copy(&strobjectpath), ::core::mem::transmute_copy(&iflags), ::core::mem::transmute(&objwbemnamedvalueset), ::core::mem::transmute(&objwbemasynccontext)).into()
        }
        unsafe extern "system" fn InstancesOf<Impl: ISWbemServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemobjectset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InstancesOf(::core::mem::transmute_copy(&strclass), ::core::mem::transmute_copy(&iflags), ::core::mem::transmute(&objwbemnamedvalueset)) {
                ::core::result::Result::Ok(ok__) => {
                    *objwbemobjectset = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InstancesOfAsync<Impl: ISWbemServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objwbemsink: ::windows::core::RawPtr, strclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemasynccontext: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InstancesOfAsync(::core::mem::transmute(&objwbemsink), ::core::mem::transmute_copy(&strclass), ::core::mem::transmute_copy(&iflags), ::core::mem::transmute(&objwbemnamedvalueset), ::core::mem::transmute(&objwbemasynccontext)).into()
        }
        unsafe extern "system" fn SubclassesOf<Impl: ISWbemServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strsuperclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemobjectset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SubclassesOf(::core::mem::transmute_copy(&strsuperclass), ::core::mem::transmute_copy(&iflags), ::core::mem::transmute(&objwbemnamedvalueset)) {
                ::core::result::Result::Ok(ok__) => {
                    *objwbemobjectset = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SubclassesOfAsync<Impl: ISWbemServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objwbemsink: ::windows::core::RawPtr, strsuperclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemasynccontext: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SubclassesOfAsync(::core::mem::transmute(&objwbemsink), ::core::mem::transmute_copy(&strsuperclass), ::core::mem::transmute_copy(&iflags), ::core::mem::transmute(&objwbemnamedvalueset), ::core::mem::transmute(&objwbemasynccontext)).into()
        }
        unsafe extern "system" fn ExecQuery<Impl: ISWbemServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strquery: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strquerylanguage: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemobjectset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExecQuery(::core::mem::transmute_copy(&strquery), ::core::mem::transmute_copy(&strquerylanguage), ::core::mem::transmute_copy(&iflags), ::core::mem::transmute(&objwbemnamedvalueset)) {
                ::core::result::Result::Ok(ok__) => {
                    *objwbemobjectset = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExecQueryAsync<Impl: ISWbemServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objwbemsink: ::windows::core::RawPtr, strquery: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strquerylanguage: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemasynccontext: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ExecQueryAsync(::core::mem::transmute(&objwbemsink), ::core::mem::transmute_copy(&strquery), ::core::mem::transmute_copy(&strquerylanguage), ::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&objwbemnamedvalueset), ::core::mem::transmute(&objwbemasynccontext)).into()
        }
        unsafe extern "system" fn AssociatorsOf<Impl: ISWbemServicesImpl, const OFFSET: isize>(
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
            objwbemnamedvalueset: ::windows::core::RawPtr,
            objwbemobjectset: *mut ::windows::core::RawPtr,
        ) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AssociatorsOf(
                ::core::mem::transmute_copy(&strobjectpath),
                ::core::mem::transmute_copy(&strassocclass),
                ::core::mem::transmute_copy(&strresultclass),
                ::core::mem::transmute_copy(&strresultrole),
                ::core::mem::transmute_copy(&strrole),
                ::core::mem::transmute_copy(&bclassesonly),
                ::core::mem::transmute_copy(&bschemaonly),
                ::core::mem::transmute_copy(&strrequiredassocqualifier),
                ::core::mem::transmute_copy(&strrequiredqualifier),
                ::core::mem::transmute_copy(&iflags),
                ::core::mem::transmute(&objwbemnamedvalueset),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *objwbemobjectset = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AssociatorsOfAsync<Impl: ISWbemServicesImpl, const OFFSET: isize>(
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
        ) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .AssociatorsOfAsync(
                    ::core::mem::transmute(&objwbemsink),
                    ::core::mem::transmute_copy(&strobjectpath),
                    ::core::mem::transmute_copy(&strassocclass),
                    ::core::mem::transmute_copy(&strresultclass),
                    ::core::mem::transmute_copy(&strresultrole),
                    ::core::mem::transmute_copy(&strrole),
                    ::core::mem::transmute_copy(&bclassesonly),
                    ::core::mem::transmute_copy(&bschemaonly),
                    ::core::mem::transmute_copy(&strrequiredassocqualifier),
                    ::core::mem::transmute_copy(&strrequiredqualifier),
                    ::core::mem::transmute_copy(&iflags),
                    ::core::mem::transmute(&objwbemnamedvalueset),
                    ::core::mem::transmute(&objwbemasynccontext),
                )
                .into()
        }
        unsafe extern "system" fn ReferencesTo<Impl: ISWbemServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strobjectpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strresultclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strrole: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bclassesonly: i16, bschemaonly: i16, strrequiredqualifier: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemobjectset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReferencesTo(::core::mem::transmute_copy(&strobjectpath), ::core::mem::transmute_copy(&strresultclass), ::core::mem::transmute_copy(&strrole), ::core::mem::transmute_copy(&bclassesonly), ::core::mem::transmute_copy(&bschemaonly), ::core::mem::transmute_copy(&strrequiredqualifier), ::core::mem::transmute_copy(&iflags), ::core::mem::transmute(&objwbemnamedvalueset)) {
                ::core::result::Result::Ok(ok__) => {
                    *objwbemobjectset = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReferencesToAsync<Impl: ISWbemServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objwbemsink: ::windows::core::RawPtr, strobjectpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strresultclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strrole: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bclassesonly: i16, bschemaonly: i16, strrequiredqualifier: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemasynccontext: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReferencesToAsync(::core::mem::transmute(&objwbemsink), ::core::mem::transmute_copy(&strobjectpath), ::core::mem::transmute_copy(&strresultclass), ::core::mem::transmute_copy(&strrole), ::core::mem::transmute_copy(&bclassesonly), ::core::mem::transmute_copy(&bschemaonly), ::core::mem::transmute_copy(&strrequiredqualifier), ::core::mem::transmute_copy(&iflags), ::core::mem::transmute(&objwbemnamedvalueset), ::core::mem::transmute(&objwbemasynccontext)).into()
        }
        unsafe extern "system" fn ExecNotificationQuery<Impl: ISWbemServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strquery: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strquerylanguage: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemeventsource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExecNotificationQuery(::core::mem::transmute_copy(&strquery), ::core::mem::transmute_copy(&strquerylanguage), ::core::mem::transmute_copy(&iflags), ::core::mem::transmute(&objwbemnamedvalueset)) {
                ::core::result::Result::Ok(ok__) => {
                    *objwbemeventsource = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExecNotificationQueryAsync<Impl: ISWbemServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objwbemsink: ::windows::core::RawPtr, strquery: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strquerylanguage: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemasynccontext: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ExecNotificationQueryAsync(::core::mem::transmute(&objwbemsink), ::core::mem::transmute_copy(&strquery), ::core::mem::transmute_copy(&strquerylanguage), ::core::mem::transmute_copy(&iflags), ::core::mem::transmute(&objwbemnamedvalueset), ::core::mem::transmute(&objwbemasynccontext)).into()
        }
        unsafe extern "system" fn ExecMethod<Impl: ISWbemServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strobjectpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strmethodname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, objwbeminparameters: ::windows::core::RawPtr, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemoutparameters: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExecMethod(::core::mem::transmute_copy(&strobjectpath), ::core::mem::transmute_copy(&strmethodname), ::core::mem::transmute(&objwbeminparameters), ::core::mem::transmute_copy(&iflags), ::core::mem::transmute(&objwbemnamedvalueset)) {
                ::core::result::Result::Ok(ok__) => {
                    *objwbemoutparameters = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExecMethodAsync<Impl: ISWbemServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objwbemsink: ::windows::core::RawPtr, strobjectpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strmethodname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, objwbeminparameters: ::windows::core::RawPtr, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemasynccontext: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ExecMethodAsync(::core::mem::transmute(&objwbemsink), ::core::mem::transmute_copy(&strobjectpath), ::core::mem::transmute_copy(&strmethodname), ::core::mem::transmute(&objwbeminparameters), ::core::mem::transmute_copy(&iflags), ::core::mem::transmute(&objwbemnamedvalueset), ::core::mem::transmute(&objwbemasynccontext)).into()
        }
        unsafe extern "system" fn Security_<Impl: ISWbemServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objwbemsecurity: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Security_() {
                ::core::result::Result::Ok(ok__) => {
                    *objwbemsecurity = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Get: Get::<Impl, IMPL_OFFSET>,
            GetAsync: GetAsync::<Impl, IMPL_OFFSET>,
            Delete: Delete::<Impl, IMPL_OFFSET>,
            DeleteAsync: DeleteAsync::<Impl, IMPL_OFFSET>,
            InstancesOf: InstancesOf::<Impl, IMPL_OFFSET>,
            InstancesOfAsync: InstancesOfAsync::<Impl, IMPL_OFFSET>,
            SubclassesOf: SubclassesOf::<Impl, IMPL_OFFSET>,
            SubclassesOfAsync: SubclassesOfAsync::<Impl, IMPL_OFFSET>,
            ExecQuery: ExecQuery::<Impl, IMPL_OFFSET>,
            ExecQueryAsync: ExecQueryAsync::<Impl, IMPL_OFFSET>,
            AssociatorsOf: AssociatorsOf::<Impl, IMPL_OFFSET>,
            AssociatorsOfAsync: AssociatorsOfAsync::<Impl, IMPL_OFFSET>,
            ReferencesTo: ReferencesTo::<Impl, IMPL_OFFSET>,
            ReferencesToAsync: ReferencesToAsync::<Impl, IMPL_OFFSET>,
            ExecNotificationQuery: ExecNotificationQuery::<Impl, IMPL_OFFSET>,
            ExecNotificationQueryAsync: ExecNotificationQueryAsync::<Impl, IMPL_OFFSET>,
            ExecMethod: ExecMethod::<Impl, IMPL_OFFSET>,
            ExecMethodAsync: ExecMethodAsync::<Impl, IMPL_OFFSET>,
            Security_: Security_::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISWbemServices as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISWbemServicesExImpl: Sized + IDispatchImpl + ISWbemServicesImpl {
    fn Put(&mut self, objwbemobject: ::core::option::Option<ISWbemObjectEx>, iflags: i32, objwbemnamedvalueset: ::core::option::Option<super::Com::IDispatch>) -> ::windows::core::Result<ISWbemObjectPath>;
    fn PutAsync(&mut self, objwbemsink: ::core::option::Option<ISWbemSink>, objwbemobject: ::core::option::Option<ISWbemObjectEx>, iflags: i32, objwbemnamedvalueset: ::core::option::Option<super::Com::IDispatch>, objwbemasynccontext: ::core::option::Option<super::Com::IDispatch>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISWbemServicesExVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISWbemServicesExImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISWbemServicesExVtbl {
        unsafe extern "system" fn Put<Impl: ISWbemServicesExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objwbemobject: ::windows::core::RawPtr, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemobjectpath: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Put(::core::mem::transmute(&objwbemobject), ::core::mem::transmute_copy(&iflags), ::core::mem::transmute(&objwbemnamedvalueset)) {
                ::core::result::Result::Ok(ok__) => {
                    *objwbemobjectpath = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PutAsync<Impl: ISWbemServicesExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objwbemsink: ::windows::core::RawPtr, objwbemobject: ::windows::core::RawPtr, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemasynccontext: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PutAsync(::core::mem::transmute(&objwbemsink), ::core::mem::transmute(&objwbemobject), ::core::mem::transmute_copy(&iflags), ::core::mem::transmute(&objwbemnamedvalueset), ::core::mem::transmute(&objwbemasynccontext)).into()
        }
        Self {
            base: ISWbemServicesVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Put: Put::<Impl, IMPL_OFFSET>,
            PutAsync: PutAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISWbemServicesEx as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISWbemSinkImpl: Sized + IDispatchImpl {
    fn Cancel(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISWbemSinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISWbemSinkImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISWbemSinkVtbl {
        unsafe extern "system" fn Cancel<Impl: ISWbemSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Cancel().into()
        }
        Self { base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), Cancel: Cancel::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISWbemSink as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISWbemSinkEventsImpl: Sized + IDispatchImpl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISWbemSinkEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISWbemSinkEventsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISWbemSinkEventsVtbl {
        Self { base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISWbemSinkEvents as ::windows::core::Interface>::IID
    }
}
pub trait IUnsecuredApartmentImpl: Sized {
    fn CreateObjectStub(&mut self, pobject: ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<::windows::core::IUnknown>;
}
impl IUnsecuredApartmentVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUnsecuredApartmentImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUnsecuredApartmentVtbl {
        unsafe extern "system" fn CreateObjectStub<Impl: IUnsecuredApartmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pobject: *mut ::core::ffi::c_void, ppstub: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateObjectStub(::core::mem::transmute(&pobject)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppstub = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), CreateObjectStub: CreateObjectStub::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUnsecuredApartment as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWMIExtensionImpl: Sized + IDispatchImpl {
    fn WMIObjectPath(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetWMIObject(&mut self) -> ::windows::core::Result<ISWbemObject>;
    fn GetWMIServices(&mut self) -> ::windows::core::Result<ISWbemServices>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMIExtensionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMIExtensionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMIExtensionVtbl {
        unsafe extern "system" fn WMIObjectPath<Impl: IWMIExtensionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strwmiobjectpath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WMIObjectPath() {
                ::core::result::Result::Ok(ok__) => {
                    *strwmiobjectpath = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWMIObject<Impl: IWMIExtensionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objwmiobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetWMIObject() {
                ::core::result::Result::Ok(ok__) => {
                    *objwmiobject = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWMIServices<Impl: IWMIExtensionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objwmiservices: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetWMIServices() {
                ::core::result::Result::Ok(ok__) => {
                    *objwmiservices = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            WMIObjectPath: WMIObjectPath::<Impl, IMPL_OFFSET>,
            GetWMIObject: GetWMIObject::<Impl, IMPL_OFFSET>,
            GetWMIServices: GetWMIServices::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMIExtension as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWbemAddressResolutionImpl: Sized {
    fn Resolve(&mut self, wsznamespacepath: super::super::Foundation::PWSTR, wszaddresstype: super::super::Foundation::PWSTR, pdwaddresslength: *mut u32, pabbinaryaddress: *mut *mut u8) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWbemAddressResolutionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWbemAddressResolutionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWbemAddressResolutionVtbl {
        unsafe extern "system" fn Resolve<Impl: IWbemAddressResolutionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wsznamespacepath: super::super::Foundation::PWSTR, wszaddresstype: super::super::Foundation::PWSTR, pdwaddresslength: *mut u32, pabbinaryaddress: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Resolve(::core::mem::transmute_copy(&wsznamespacepath), ::core::mem::transmute_copy(&wszaddresstype), ::core::mem::transmute_copy(&pdwaddresslength), ::core::mem::transmute_copy(&pabbinaryaddress)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), Resolve: Resolve::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWbemAddressResolution as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWbemBackupRestoreImpl: Sized {
    fn Backup(&mut self, strbackuptofile: super::super::Foundation::PWSTR, lflags: i32) -> ::windows::core::Result<()>;
    fn Restore(&mut self, strrestorefromfile: super::super::Foundation::PWSTR, lflags: i32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWbemBackupRestoreVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWbemBackupRestoreImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWbemBackupRestoreVtbl {
        unsafe extern "system" fn Backup<Impl: IWbemBackupRestoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strbackuptofile: super::super::Foundation::PWSTR, lflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Backup(::core::mem::transmute_copy(&strbackuptofile), ::core::mem::transmute_copy(&lflags)).into()
        }
        unsafe extern "system" fn Restore<Impl: IWbemBackupRestoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strrestorefromfile: super::super::Foundation::PWSTR, lflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Restore(::core::mem::transmute_copy(&strrestorefromfile), ::core::mem::transmute_copy(&lflags)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), Backup: Backup::<Impl, IMPL_OFFSET>, Restore: Restore::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWbemBackupRestore as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWbemBackupRestoreExImpl: Sized + IWbemBackupRestoreImpl {
    fn Pause(&mut self) -> ::windows::core::Result<()>;
    fn Resume(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWbemBackupRestoreExVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWbemBackupRestoreExImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWbemBackupRestoreExVtbl {
        unsafe extern "system" fn Pause<Impl: IWbemBackupRestoreExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Pause().into()
        }
        unsafe extern "system" fn Resume<Impl: IWbemBackupRestoreExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Resume().into()
        }
        Self {
            base: IWbemBackupRestoreVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Pause: Pause::<Impl, IMPL_OFFSET>,
            Resume: Resume::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWbemBackupRestoreEx as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWbemCallResultImpl: Sized {
    fn GetResultObject(&mut self, ltimeout: i32) -> ::windows::core::Result<IWbemClassObject>;
    fn GetResultString(&mut self, ltimeout: i32) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetResultServices(&mut self, ltimeout: i32) -> ::windows::core::Result<IWbemServices>;
    fn GetCallStatus(&mut self, ltimeout: i32) -> ::windows::core::Result<i32>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWbemCallResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWbemCallResultImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWbemCallResultVtbl {
        unsafe extern "system" fn GetResultObject<Impl: IWbemCallResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ltimeout: i32, ppresultobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetResultObject(::core::mem::transmute_copy(&ltimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppresultobject = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetResultString<Impl: IWbemCallResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ltimeout: i32, pstrresultstring: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetResultString(::core::mem::transmute_copy(&ltimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    *pstrresultstring = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetResultServices<Impl: IWbemCallResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ltimeout: i32, ppservices: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetResultServices(::core::mem::transmute_copy(&ltimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppservices = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCallStatus<Impl: IWbemCallResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ltimeout: i32, plstatus: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCallStatus(::core::mem::transmute_copy(&ltimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    *plstatus = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetResultObject: GetResultObject::<Impl, IMPL_OFFSET>,
            GetResultString: GetResultString::<Impl, IMPL_OFFSET>,
            GetResultServices: GetResultServices::<Impl, IMPL_OFFSET>,
            GetCallStatus: GetCallStatus::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWbemCallResult as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWbemClassObjectImpl: Sized {
    fn GetQualifierSet(&mut self) -> ::windows::core::Result<IWbemQualifierSet>;
    fn Get(&mut self, wszname: super::super::Foundation::PWSTR, lflags: i32, pval: *mut super::Com::VARIANT, ptype: *mut i32, plflavor: *mut i32) -> ::windows::core::Result<()>;
    fn Put(&mut self, wszname: super::super::Foundation::PWSTR, lflags: i32, pval: *const super::Com::VARIANT, r#type: i32) -> ::windows::core::Result<()>;
    fn Delete(&mut self, wszname: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn GetNames(&mut self, wszqualifiername: super::super::Foundation::PWSTR, lflags: i32, pqualifierval: *const super::Com::VARIANT) -> ::windows::core::Result<*mut super::Com::SAFEARRAY>;
    fn BeginEnumeration(&mut self, lenumflags: i32) -> ::windows::core::Result<()>;
    fn Next(&mut self, lflags: i32, strname: *mut super::super::Foundation::BSTR, pval: *mut super::Com::VARIANT, ptype: *mut i32, plflavor: *mut i32) -> ::windows::core::Result<()>;
    fn EndEnumeration(&mut self) -> ::windows::core::Result<()>;
    fn GetPropertyQualifierSet(&mut self, wszproperty: super::super::Foundation::PWSTR) -> ::windows::core::Result<IWbemQualifierSet>;
    fn Clone(&mut self) -> ::windows::core::Result<IWbemClassObject>;
    fn GetObjectText(&mut self, lflags: i32) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SpawnDerivedClass(&mut self, lflags: i32) -> ::windows::core::Result<IWbemClassObject>;
    fn SpawnInstance(&mut self, lflags: i32) -> ::windows::core::Result<IWbemClassObject>;
    fn CompareTo(&mut self, lflags: i32, pcompareto: ::core::option::Option<IWbemClassObject>) -> ::windows::core::Result<()>;
    fn GetPropertyOrigin(&mut self, wszname: super::super::Foundation::PWSTR) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn InheritsFrom(&mut self, strancestor: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn GetMethod(&mut self, wszname: super::super::Foundation::PWSTR, lflags: i32, ppinsignature: *mut ::core::option::Option<IWbemClassObject>, ppoutsignature: *mut ::core::option::Option<IWbemClassObject>) -> ::windows::core::Result<()>;
    fn PutMethod(&mut self, wszname: super::super::Foundation::PWSTR, lflags: i32, pinsignature: ::core::option::Option<IWbemClassObject>, poutsignature: ::core::option::Option<IWbemClassObject>) -> ::windows::core::Result<()>;
    fn DeleteMethod(&mut self, wszname: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn BeginMethodEnumeration(&mut self, lenumflags: i32) -> ::windows::core::Result<()>;
    fn NextMethod(&mut self, lflags: i32, pstrname: *mut super::super::Foundation::BSTR, ppinsignature: *mut ::core::option::Option<IWbemClassObject>, ppoutsignature: *mut ::core::option::Option<IWbemClassObject>) -> ::windows::core::Result<()>;
    fn EndMethodEnumeration(&mut self) -> ::windows::core::Result<()>;
    fn GetMethodQualifierSet(&mut self, wszmethod: super::super::Foundation::PWSTR) -> ::windows::core::Result<IWbemQualifierSet>;
    fn GetMethodOrigin(&mut self, wszmethodname: super::super::Foundation::PWSTR) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWbemClassObjectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWbemClassObjectImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWbemClassObjectVtbl {
        unsafe extern "system" fn GetQualifierSet<Impl: IWbemClassObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqualset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetQualifierSet() {
                ::core::result::Result::Ok(ok__) => {
                    *ppqualset = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Get<Impl: IWbemClassObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszname: super::super::Foundation::PWSTR, lflags: i32, pval: *mut super::Com::VARIANT, ptype: *mut i32, plflavor: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Get(::core::mem::transmute_copy(&wszname), ::core::mem::transmute_copy(&lflags), ::core::mem::transmute_copy(&pval), ::core::mem::transmute_copy(&ptype), ::core::mem::transmute_copy(&plflavor)).into()
        }
        unsafe extern "system" fn Put<Impl: IWbemClassObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszname: super::super::Foundation::PWSTR, lflags: i32, pval: *const super::Com::VARIANT, r#type: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Put(::core::mem::transmute_copy(&wszname), ::core::mem::transmute_copy(&lflags), ::core::mem::transmute_copy(&pval), ::core::mem::transmute_copy(&r#type)).into()
        }
        unsafe extern "system" fn Delete<Impl: IWbemClassObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Delete(::core::mem::transmute_copy(&wszname)).into()
        }
        unsafe extern "system" fn GetNames<Impl: IWbemClassObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszqualifiername: super::super::Foundation::PWSTR, lflags: i32, pqualifierval: *const super::Com::VARIANT, pnames: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNames(::core::mem::transmute_copy(&wszqualifiername), ::core::mem::transmute_copy(&lflags), ::core::mem::transmute_copy(&pqualifierval)) {
                ::core::result::Result::Ok(ok__) => {
                    *pnames = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginEnumeration<Impl: IWbemClassObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lenumflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BeginEnumeration(::core::mem::transmute_copy(&lenumflags)).into()
        }
        unsafe extern "system" fn Next<Impl: IWbemClassObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, strname: *mut super::super::Foundation::BSTR, pval: *mut super::Com::VARIANT, ptype: *mut i32, plflavor: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&lflags), ::core::mem::transmute_copy(&strname), ::core::mem::transmute_copy(&pval), ::core::mem::transmute_copy(&ptype), ::core::mem::transmute_copy(&plflavor)).into()
        }
        unsafe extern "system" fn EndEnumeration<Impl: IWbemClassObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EndEnumeration().into()
        }
        unsafe extern "system" fn GetPropertyQualifierSet<Impl: IWbemClassObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszproperty: super::super::Foundation::PWSTR, ppqualset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPropertyQualifierSet(::core::mem::transmute_copy(&wszproperty)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppqualset = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IWbemClassObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcopy: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcopy = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetObjectText<Impl: IWbemClassObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, pstrobjecttext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetObjectText(::core::mem::transmute_copy(&lflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *pstrobjecttext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SpawnDerivedClass<Impl: IWbemClassObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, ppnewclass: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SpawnDerivedClass(::core::mem::transmute_copy(&lflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppnewclass = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SpawnInstance<Impl: IWbemClassObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, ppnewinstance: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SpawnInstance(::core::mem::transmute_copy(&lflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppnewinstance = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CompareTo<Impl: IWbemClassObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, pcompareto: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CompareTo(::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&pcompareto)).into()
        }
        unsafe extern "system" fn GetPropertyOrigin<Impl: IWbemClassObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszname: super::super::Foundation::PWSTR, pstrclassname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPropertyOrigin(::core::mem::transmute_copy(&wszname)) {
                ::core::result::Result::Ok(ok__) => {
                    *pstrclassname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InheritsFrom<Impl: IWbemClassObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strancestor: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InheritsFrom(::core::mem::transmute_copy(&strancestor)).into()
        }
        unsafe extern "system" fn GetMethod<Impl: IWbemClassObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszname: super::super::Foundation::PWSTR, lflags: i32, ppinsignature: *mut ::windows::core::RawPtr, ppoutsignature: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetMethod(::core::mem::transmute_copy(&wszname), ::core::mem::transmute_copy(&lflags), ::core::mem::transmute_copy(&ppinsignature), ::core::mem::transmute_copy(&ppoutsignature)).into()
        }
        unsafe extern "system" fn PutMethod<Impl: IWbemClassObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszname: super::super::Foundation::PWSTR, lflags: i32, pinsignature: ::windows::core::RawPtr, poutsignature: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PutMethod(::core::mem::transmute_copy(&wszname), ::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&pinsignature), ::core::mem::transmute(&poutsignature)).into()
        }
        unsafe extern "system" fn DeleteMethod<Impl: IWbemClassObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteMethod(::core::mem::transmute_copy(&wszname)).into()
        }
        unsafe extern "system" fn BeginMethodEnumeration<Impl: IWbemClassObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lenumflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BeginMethodEnumeration(::core::mem::transmute_copy(&lenumflags)).into()
        }
        unsafe extern "system" fn NextMethod<Impl: IWbemClassObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, pstrname: *mut super::super::Foundation::BSTR, ppinsignature: *mut ::windows::core::RawPtr, ppoutsignature: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).NextMethod(::core::mem::transmute_copy(&lflags), ::core::mem::transmute_copy(&pstrname), ::core::mem::transmute_copy(&ppinsignature), ::core::mem::transmute_copy(&ppoutsignature)).into()
        }
        unsafe extern "system" fn EndMethodEnumeration<Impl: IWbemClassObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EndMethodEnumeration().into()
        }
        unsafe extern "system" fn GetMethodQualifierSet<Impl: IWbemClassObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszmethod: super::super::Foundation::PWSTR, ppqualset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMethodQualifierSet(::core::mem::transmute_copy(&wszmethod)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppqualset = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMethodOrigin<Impl: IWbemClassObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszmethodname: super::super::Foundation::PWSTR, pstrclassname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMethodOrigin(::core::mem::transmute_copy(&wszmethodname)) {
                ::core::result::Result::Ok(ok__) => {
                    *pstrclassname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetQualifierSet: GetQualifierSet::<Impl, IMPL_OFFSET>,
            Get: Get::<Impl, IMPL_OFFSET>,
            Put: Put::<Impl, IMPL_OFFSET>,
            Delete: Delete::<Impl, IMPL_OFFSET>,
            GetNames: GetNames::<Impl, IMPL_OFFSET>,
            BeginEnumeration: BeginEnumeration::<Impl, IMPL_OFFSET>,
            Next: Next::<Impl, IMPL_OFFSET>,
            EndEnumeration: EndEnumeration::<Impl, IMPL_OFFSET>,
            GetPropertyQualifierSet: GetPropertyQualifierSet::<Impl, IMPL_OFFSET>,
            Clone: Clone::<Impl, IMPL_OFFSET>,
            GetObjectText: GetObjectText::<Impl, IMPL_OFFSET>,
            SpawnDerivedClass: SpawnDerivedClass::<Impl, IMPL_OFFSET>,
            SpawnInstance: SpawnInstance::<Impl, IMPL_OFFSET>,
            CompareTo: CompareTo::<Impl, IMPL_OFFSET>,
            GetPropertyOrigin: GetPropertyOrigin::<Impl, IMPL_OFFSET>,
            InheritsFrom: InheritsFrom::<Impl, IMPL_OFFSET>,
            GetMethod: GetMethod::<Impl, IMPL_OFFSET>,
            PutMethod: PutMethod::<Impl, IMPL_OFFSET>,
            DeleteMethod: DeleteMethod::<Impl, IMPL_OFFSET>,
            BeginMethodEnumeration: BeginMethodEnumeration::<Impl, IMPL_OFFSET>,
            NextMethod: NextMethod::<Impl, IMPL_OFFSET>,
            EndMethodEnumeration: EndMethodEnumeration::<Impl, IMPL_OFFSET>,
            GetMethodQualifierSet: GetMethodQualifierSet::<Impl, IMPL_OFFSET>,
            GetMethodOrigin: GetMethodOrigin::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWbemClassObject as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWbemClientConnectionTransportImpl: Sized {
    fn Open(&mut self, straddresstype: super::super::Foundation::BSTR, dwbinaryaddresslength: u32, abbinaryaddress: *const u8, strobject: super::super::Foundation::BSTR, struser: super::super::Foundation::BSTR, strpassword: super::super::Foundation::BSTR, strlocale: super::super::Foundation::BSTR, lflags: i32, pctx: ::core::option::Option<IWbemContext>, riid: *const ::windows::core::GUID, pinterface: *mut *mut ::core::ffi::c_void, pcallres: *mut ::core::option::Option<IWbemCallResult>) -> ::windows::core::Result<()>;
    fn OpenAsync(&mut self, straddresstype: super::super::Foundation::BSTR, dwbinaryaddresslength: u32, abbinaryaddress: *const u8, strobject: super::super::Foundation::BSTR, struser: super::super::Foundation::BSTR, strpassword: super::super::Foundation::BSTR, strlocale: super::super::Foundation::BSTR, lflags: i32, pctx: ::core::option::Option<IWbemContext>, riid: *const ::windows::core::GUID, presponsehandler: ::core::option::Option<IWbemObjectSink>) -> ::windows::core::Result<()>;
    fn Cancel(&mut self, lflags: i32, phandler: ::core::option::Option<IWbemObjectSink>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWbemClientConnectionTransportVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWbemClientConnectionTransportImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWbemClientConnectionTransportVtbl {
        unsafe extern "system" fn Open<Impl: IWbemClientConnectionTransportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, straddresstype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwbinaryaddresslength: u32, abbinaryaddress: *const u8, strobject: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, struser: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strlocale: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lflags: i32, pctx: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, pinterface: *mut *mut ::core::ffi::c_void, pcallres: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .Open(
                    ::core::mem::transmute_copy(&straddresstype),
                    ::core::mem::transmute_copy(&dwbinaryaddresslength),
                    ::core::mem::transmute_copy(&abbinaryaddress),
                    ::core::mem::transmute_copy(&strobject),
                    ::core::mem::transmute_copy(&struser),
                    ::core::mem::transmute_copy(&strpassword),
                    ::core::mem::transmute_copy(&strlocale),
                    ::core::mem::transmute_copy(&lflags),
                    ::core::mem::transmute(&pctx),
                    ::core::mem::transmute_copy(&riid),
                    ::core::mem::transmute_copy(&pinterface),
                    ::core::mem::transmute_copy(&pcallres),
                )
                .into()
        }
        unsafe extern "system" fn OpenAsync<Impl: IWbemClientConnectionTransportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, straddresstype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwbinaryaddresslength: u32, abbinaryaddress: *const u8, strobject: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, struser: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strlocale: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lflags: i32, pctx: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, presponsehandler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .OpenAsync(::core::mem::transmute_copy(&straddresstype), ::core::mem::transmute_copy(&dwbinaryaddresslength), ::core::mem::transmute_copy(&abbinaryaddress), ::core::mem::transmute_copy(&strobject), ::core::mem::transmute_copy(&struser), ::core::mem::transmute_copy(&strpassword), ::core::mem::transmute_copy(&strlocale), ::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&pctx), ::core::mem::transmute_copy(&riid), ::core::mem::transmute(&presponsehandler))
                .into()
        }
        unsafe extern "system" fn Cancel<Impl: IWbemClientConnectionTransportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, phandler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Cancel(::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&phandler)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Open: Open::<Impl, IMPL_OFFSET>,
            OpenAsync: OpenAsync::<Impl, IMPL_OFFSET>,
            Cancel: Cancel::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWbemClientConnectionTransport as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWbemClientTransportImpl: Sized {
    fn ConnectServer(&mut self, straddresstype: super::super::Foundation::BSTR, dwbinaryaddresslength: u32, abbinaryaddress: *const u8, strnetworkresource: super::super::Foundation::BSTR, struser: super::super::Foundation::BSTR, strpassword: super::super::Foundation::BSTR, strlocale: super::super::Foundation::BSTR, lsecurityflags: i32, strauthority: super::super::Foundation::BSTR, pctx: ::core::option::Option<IWbemContext>) -> ::windows::core::Result<IWbemServices>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWbemClientTransportVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWbemClientTransportImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWbemClientTransportVtbl {
        unsafe extern "system" fn ConnectServer<Impl: IWbemClientTransportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, straddresstype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwbinaryaddresslength: u32, abbinaryaddress: *const u8, strnetworkresource: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, struser: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strlocale: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lsecurityflags: i32, strauthority: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pctx: ::windows::core::RawPtr, ppnamespace: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConnectServer(::core::mem::transmute_copy(&straddresstype), ::core::mem::transmute_copy(&dwbinaryaddresslength), ::core::mem::transmute_copy(&abbinaryaddress), ::core::mem::transmute_copy(&strnetworkresource), ::core::mem::transmute_copy(&struser), ::core::mem::transmute_copy(&strpassword), ::core::mem::transmute_copy(&strlocale), ::core::mem::transmute_copy(&lsecurityflags), ::core::mem::transmute_copy(&strauthority), ::core::mem::transmute(&pctx)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppnamespace = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), ConnectServer: ConnectServer::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWbemClientTransport as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWbemConfigureRefresherImpl: Sized {
    fn AddObjectByPath(&mut self, pnamespace: ::core::option::Option<IWbemServices>, wszpath: super::super::Foundation::PWSTR, lflags: i32, pcontext: ::core::option::Option<IWbemContext>, pprefreshable: *mut ::core::option::Option<IWbemClassObject>, plid: *mut i32) -> ::windows::core::Result<()>;
    fn AddObjectByTemplate(&mut self, pnamespace: ::core::option::Option<IWbemServices>, ptemplate: ::core::option::Option<IWbemClassObject>, lflags: i32, pcontext: ::core::option::Option<IWbemContext>, pprefreshable: *mut ::core::option::Option<IWbemClassObject>, plid: *mut i32) -> ::windows::core::Result<()>;
    fn AddRefresher(&mut self, prefresher: ::core::option::Option<IWbemRefresher>, lflags: i32, plid: *mut i32) -> ::windows::core::Result<()>;
    fn Remove(&mut self, lid: i32, lflags: i32) -> ::windows::core::Result<()>;
    fn AddEnum(&mut self, pnamespace: ::core::option::Option<IWbemServices>, wszclassname: super::super::Foundation::PWSTR, lflags: i32, pcontext: ::core::option::Option<IWbemContext>, ppenum: *mut ::core::option::Option<IWbemHiPerfEnum>, plid: *mut i32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWbemConfigureRefresherVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWbemConfigureRefresherImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWbemConfigureRefresherVtbl {
        unsafe extern "system" fn AddObjectByPath<Impl: IWbemConfigureRefresherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnamespace: ::windows::core::RawPtr, wszpath: super::super::Foundation::PWSTR, lflags: i32, pcontext: ::windows::core::RawPtr, pprefreshable: *mut ::windows::core::RawPtr, plid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddObjectByPath(::core::mem::transmute(&pnamespace), ::core::mem::transmute_copy(&wszpath), ::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&pcontext), ::core::mem::transmute_copy(&pprefreshable), ::core::mem::transmute_copy(&plid)).into()
        }
        unsafe extern "system" fn AddObjectByTemplate<Impl: IWbemConfigureRefresherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnamespace: ::windows::core::RawPtr, ptemplate: ::windows::core::RawPtr, lflags: i32, pcontext: ::windows::core::RawPtr, pprefreshable: *mut ::windows::core::RawPtr, plid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddObjectByTemplate(::core::mem::transmute(&pnamespace), ::core::mem::transmute(&ptemplate), ::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&pcontext), ::core::mem::transmute_copy(&pprefreshable), ::core::mem::transmute_copy(&plid)).into()
        }
        unsafe extern "system" fn AddRefresher<Impl: IWbemConfigureRefresherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prefresher: ::windows::core::RawPtr, lflags: i32, plid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddRefresher(::core::mem::transmute(&prefresher), ::core::mem::transmute_copy(&lflags), ::core::mem::transmute_copy(&plid)).into()
        }
        unsafe extern "system" fn Remove<Impl: IWbemConfigureRefresherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lid: i32, lflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Remove(::core::mem::transmute_copy(&lid), ::core::mem::transmute_copy(&lflags)).into()
        }
        unsafe extern "system" fn AddEnum<Impl: IWbemConfigureRefresherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnamespace: ::windows::core::RawPtr, wszclassname: super::super::Foundation::PWSTR, lflags: i32, pcontext: ::windows::core::RawPtr, ppenum: *mut ::windows::core::RawPtr, plid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddEnum(::core::mem::transmute(&pnamespace), ::core::mem::transmute_copy(&wszclassname), ::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&pcontext), ::core::mem::transmute_copy(&ppenum), ::core::mem::transmute_copy(&plid)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            AddObjectByPath: AddObjectByPath::<Impl, IMPL_OFFSET>,
            AddObjectByTemplate: AddObjectByTemplate::<Impl, IMPL_OFFSET>,
            AddRefresher: AddRefresher::<Impl, IMPL_OFFSET>,
            Remove: Remove::<Impl, IMPL_OFFSET>,
            AddEnum: AddEnum::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWbemConfigureRefresher as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWbemConnectorLoginImpl: Sized {
    fn ConnectorLogin(&mut self, wsznetworkresource: super::super::Foundation::PWSTR, wszpreferredlocale: super::super::Foundation::PWSTR, lflags: i32, pctx: ::core::option::Option<IWbemContext>, riid: *const ::windows::core::GUID, pinterface: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWbemConnectorLoginVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWbemConnectorLoginImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWbemConnectorLoginVtbl {
        unsafe extern "system" fn ConnectorLogin<Impl: IWbemConnectorLoginImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wsznetworkresource: super::super::Foundation::PWSTR, wszpreferredlocale: super::super::Foundation::PWSTR, lflags: i32, pctx: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, pinterface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ConnectorLogin(::core::mem::transmute_copy(&wsznetworkresource), ::core::mem::transmute_copy(&wszpreferredlocale), ::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&pctx), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&pinterface)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), ConnectorLogin: ConnectorLogin::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWbemConnectorLogin as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWbemConstructClassObjectImpl: Sized {
    fn SetInheritanceChain(&mut self, lnumantecedents: i32, awszantecedents: *const super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn SetPropertyOrigin(&mut self, wszpropertyname: super::super::Foundation::PWSTR, loriginindex: i32) -> ::windows::core::Result<()>;
    fn SetMethodOrigin(&mut self, wszmethodname: super::super::Foundation::PWSTR, loriginindex: i32) -> ::windows::core::Result<()>;
    fn SetServerNamespace(&mut self, wszserver: super::super::Foundation::PWSTR, wsznamespace: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWbemConstructClassObjectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWbemConstructClassObjectImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWbemConstructClassObjectVtbl {
        unsafe extern "system" fn SetInheritanceChain<Impl: IWbemConstructClassObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnumantecedents: i32, awszantecedents: *const super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInheritanceChain(::core::mem::transmute_copy(&lnumantecedents), ::core::mem::transmute_copy(&awszantecedents)).into()
        }
        unsafe extern "system" fn SetPropertyOrigin<Impl: IWbemConstructClassObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszpropertyname: super::super::Foundation::PWSTR, loriginindex: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPropertyOrigin(::core::mem::transmute_copy(&wszpropertyname), ::core::mem::transmute_copy(&loriginindex)).into()
        }
        unsafe extern "system" fn SetMethodOrigin<Impl: IWbemConstructClassObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszmethodname: super::super::Foundation::PWSTR, loriginindex: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMethodOrigin(::core::mem::transmute_copy(&wszmethodname), ::core::mem::transmute_copy(&loriginindex)).into()
        }
        unsafe extern "system" fn SetServerNamespace<Impl: IWbemConstructClassObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszserver: super::super::Foundation::PWSTR, wsznamespace: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetServerNamespace(::core::mem::transmute_copy(&wszserver), ::core::mem::transmute_copy(&wsznamespace)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetInheritanceChain: SetInheritanceChain::<Impl, IMPL_OFFSET>,
            SetPropertyOrigin: SetPropertyOrigin::<Impl, IMPL_OFFSET>,
            SetMethodOrigin: SetMethodOrigin::<Impl, IMPL_OFFSET>,
            SetServerNamespace: SetServerNamespace::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWbemConstructClassObject as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWbemContextImpl: Sized {
    fn Clone(&mut self) -> ::windows::core::Result<IWbemContext>;
    fn GetNames(&mut self, lflags: i32) -> ::windows::core::Result<*mut super::Com::SAFEARRAY>;
    fn BeginEnumeration(&mut self, lflags: i32) -> ::windows::core::Result<()>;
    fn Next(&mut self, lflags: i32, pstrname: *mut super::super::Foundation::BSTR, pvalue: *mut super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn EndEnumeration(&mut self) -> ::windows::core::Result<()>;
    fn SetValue(&mut self, wszname: super::super::Foundation::PWSTR, lflags: i32, pvalue: *const super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn GetValue(&mut self, wszname: super::super::Foundation::PWSTR, lflags: i32) -> ::windows::core::Result<super::Com::VARIANT>;
    fn DeleteValue(&mut self, wszname: super::super::Foundation::PWSTR, lflags: i32) -> ::windows::core::Result<()>;
    fn DeleteAll(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWbemContextVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWbemContextImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWbemContextVtbl {
        unsafe extern "system" fn Clone<Impl: IWbemContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppnewcopy: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppnewcopy = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNames<Impl: IWbemContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, pnames: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNames(::core::mem::transmute_copy(&lflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *pnames = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginEnumeration<Impl: IWbemContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BeginEnumeration(::core::mem::transmute_copy(&lflags)).into()
        }
        unsafe extern "system" fn Next<Impl: IWbemContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, pstrname: *mut super::super::Foundation::BSTR, pvalue: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&lflags), ::core::mem::transmute_copy(&pstrname), ::core::mem::transmute_copy(&pvalue)).into()
        }
        unsafe extern "system" fn EndEnumeration<Impl: IWbemContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EndEnumeration().into()
        }
        unsafe extern "system" fn SetValue<Impl: IWbemContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszname: super::super::Foundation::PWSTR, lflags: i32, pvalue: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetValue(::core::mem::transmute_copy(&wszname), ::core::mem::transmute_copy(&lflags), ::core::mem::transmute_copy(&pvalue)).into()
        }
        unsafe extern "system" fn GetValue<Impl: IWbemContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszname: super::super::Foundation::PWSTR, lflags: i32, pvalue: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetValue(::core::mem::transmute_copy(&wszname), ::core::mem::transmute_copy(&lflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteValue<Impl: IWbemContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszname: super::super::Foundation::PWSTR, lflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteValue(::core::mem::transmute_copy(&wszname), ::core::mem::transmute_copy(&lflags)).into()
        }
        unsafe extern "system" fn DeleteAll<Impl: IWbemContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteAll().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Clone: Clone::<Impl, IMPL_OFFSET>,
            GetNames: GetNames::<Impl, IMPL_OFFSET>,
            BeginEnumeration: BeginEnumeration::<Impl, IMPL_OFFSET>,
            Next: Next::<Impl, IMPL_OFFSET>,
            EndEnumeration: EndEnumeration::<Impl, IMPL_OFFSET>,
            SetValue: SetValue::<Impl, IMPL_OFFSET>,
            GetValue: GetValue::<Impl, IMPL_OFFSET>,
            DeleteValue: DeleteValue::<Impl, IMPL_OFFSET>,
            DeleteAll: DeleteAll::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWbemContext as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWbemDecoupledBasicEventProviderImpl: Sized + IWbemDecoupledRegistrarImpl {
    fn GetSink(&mut self, a_flags: i32, a_context: ::core::option::Option<IWbemContext>) -> ::windows::core::Result<IWbemObjectSink>;
    fn GetService(&mut self, a_flags: i32, a_context: ::core::option::Option<IWbemContext>) -> ::windows::core::Result<IWbemServices>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWbemDecoupledBasicEventProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWbemDecoupledBasicEventProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWbemDecoupledBasicEventProviderVtbl {
        unsafe extern "system" fn GetSink<Impl: IWbemDecoupledBasicEventProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, a_flags: i32, a_context: ::windows::core::RawPtr, a_sink: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSink(::core::mem::transmute_copy(&a_flags), ::core::mem::transmute(&a_context)) {
                ::core::result::Result::Ok(ok__) => {
                    *a_sink = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetService<Impl: IWbemDecoupledBasicEventProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, a_flags: i32, a_context: ::windows::core::RawPtr, a_service: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetService(::core::mem::transmute_copy(&a_flags), ::core::mem::transmute(&a_context)) {
                ::core::result::Result::Ok(ok__) => {
                    *a_service = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IWbemDecoupledRegistrarVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetSink: GetSink::<Impl, IMPL_OFFSET>,
            GetService: GetService::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWbemDecoupledBasicEventProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWbemDecoupledRegistrarImpl: Sized {
    fn Register(&mut self, a_flags: i32, a_context: ::core::option::Option<IWbemContext>, a_user: super::super::Foundation::PWSTR, a_locale: super::super::Foundation::PWSTR, a_scope: super::super::Foundation::PWSTR, a_registration: super::super::Foundation::PWSTR, piunknown: ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn UnRegister(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWbemDecoupledRegistrarVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWbemDecoupledRegistrarImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWbemDecoupledRegistrarVtbl {
        unsafe extern "system" fn Register<Impl: IWbemDecoupledRegistrarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, a_flags: i32, a_context: ::windows::core::RawPtr, a_user: super::super::Foundation::PWSTR, a_locale: super::super::Foundation::PWSTR, a_scope: super::super::Foundation::PWSTR, a_registration: super::super::Foundation::PWSTR, piunknown: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Register(::core::mem::transmute_copy(&a_flags), ::core::mem::transmute(&a_context), ::core::mem::transmute_copy(&a_user), ::core::mem::transmute_copy(&a_locale), ::core::mem::transmute_copy(&a_scope), ::core::mem::transmute_copy(&a_registration), ::core::mem::transmute(&piunknown)).into()
        }
        unsafe extern "system" fn UnRegister<Impl: IWbemDecoupledRegistrarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UnRegister().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Register: Register::<Impl, IMPL_OFFSET>,
            UnRegister: UnRegister::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWbemDecoupledRegistrar as ::windows::core::Interface>::IID
    }
}
pub trait IWbemEventConsumerProviderImpl: Sized {
    fn FindConsumer(&mut self, plogicalconsumer: ::core::option::Option<IWbemClassObject>) -> ::windows::core::Result<IWbemUnboundObjectSink>;
}
impl IWbemEventConsumerProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWbemEventConsumerProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWbemEventConsumerProviderVtbl {
        unsafe extern "system" fn FindConsumer<Impl: IWbemEventConsumerProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plogicalconsumer: ::windows::core::RawPtr, ppconsumer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindConsumer(::core::mem::transmute(&plogicalconsumer)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppconsumer = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), FindConsumer: FindConsumer::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWbemEventConsumerProvider as ::windows::core::Interface>::IID
    }
}
pub trait IWbemEventProviderImpl: Sized {
    fn ProvideEvents(&mut self, psink: ::core::option::Option<IWbemObjectSink>, lflags: i32) -> ::windows::core::Result<()>;
}
impl IWbemEventProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWbemEventProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWbemEventProviderVtbl {
        unsafe extern "system" fn ProvideEvents<Impl: IWbemEventProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psink: ::windows::core::RawPtr, lflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ProvideEvents(::core::mem::transmute(&psink), ::core::mem::transmute_copy(&lflags)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), ProvideEvents: ProvideEvents::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWbemEventProvider as ::windows::core::Interface>::IID
    }
}
pub trait IWbemEventProviderQuerySinkImpl: Sized {
    fn NewQuery(&mut self, dwid: u32, wszquerylanguage: *const u16, wszquery: *const u16) -> ::windows::core::Result<()>;
    fn CancelQuery(&mut self, dwid: u32) -> ::windows::core::Result<()>;
}
impl IWbemEventProviderQuerySinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWbemEventProviderQuerySinkImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWbemEventProviderQuerySinkVtbl {
        unsafe extern "system" fn NewQuery<Impl: IWbemEventProviderQuerySinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwid: u32, wszquerylanguage: *const u16, wszquery: *const u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).NewQuery(::core::mem::transmute_copy(&dwid), ::core::mem::transmute_copy(&wszquerylanguage), ::core::mem::transmute_copy(&wszquery)).into()
        }
        unsafe extern "system" fn CancelQuery<Impl: IWbemEventProviderQuerySinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CancelQuery(::core::mem::transmute_copy(&dwid)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            NewQuery: NewQuery::<Impl, IMPL_OFFSET>,
            CancelQuery: CancelQuery::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWbemEventProviderQuerySink as ::windows::core::Interface>::IID
    }
}
pub trait IWbemEventProviderSecurityImpl: Sized {
    fn AccessCheck(&mut self, wszquerylanguage: *const u16, wszquery: *const u16, lsidlength: i32, psid: *const u8) -> ::windows::core::Result<()>;
}
impl IWbemEventProviderSecurityVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWbemEventProviderSecurityImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWbemEventProviderSecurityVtbl {
        unsafe extern "system" fn AccessCheck<Impl: IWbemEventProviderSecurityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszquerylanguage: *const u16, wszquery: *const u16, lsidlength: i32, psid: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AccessCheck(::core::mem::transmute_copy(&wszquerylanguage), ::core::mem::transmute_copy(&wszquery), ::core::mem::transmute_copy(&lsidlength), ::core::mem::transmute_copy(&psid)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), AccessCheck: AccessCheck::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWbemEventProviderSecurity as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWbemEventSinkImpl: Sized + IWbemObjectSinkImpl {
    fn SetSinkSecurity(&mut self, lsdlength: i32, psd: *const u8) -> ::windows::core::Result<()>;
    fn IsActive(&mut self) -> ::windows::core::Result<()>;
    fn GetRestrictedSink(&mut self, lnumqueries: i32, awszqueries: *const super::super::Foundation::PWSTR, pcallback: ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<IWbemEventSink>;
    fn SetBatchingParameters(&mut self, lflags: i32, dwmaxbuffersize: u32, dwmaxsendlatency: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWbemEventSinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWbemEventSinkImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWbemEventSinkVtbl {
        unsafe extern "system" fn SetSinkSecurity<Impl: IWbemEventSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lsdlength: i32, psd: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSinkSecurity(::core::mem::transmute_copy(&lsdlength), ::core::mem::transmute_copy(&psd)).into()
        }
        unsafe extern "system" fn IsActive<Impl: IWbemEventSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IsActive().into()
        }
        unsafe extern "system" fn GetRestrictedSink<Impl: IWbemEventSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnumqueries: i32, awszqueries: *const super::super::Foundation::PWSTR, pcallback: *mut ::core::ffi::c_void, ppsink: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRestrictedSink(::core::mem::transmute_copy(&lnumqueries), ::core::mem::transmute_copy(&awszqueries), ::core::mem::transmute(&pcallback)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppsink = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBatchingParameters<Impl: IWbemEventSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, dwmaxbuffersize: u32, dwmaxsendlatency: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBatchingParameters(::core::mem::transmute_copy(&lflags), ::core::mem::transmute_copy(&dwmaxbuffersize), ::core::mem::transmute_copy(&dwmaxsendlatency)).into()
        }
        Self {
            base: IWbemObjectSinkVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetSinkSecurity: SetSinkSecurity::<Impl, IMPL_OFFSET>,
            IsActive: IsActive::<Impl, IMPL_OFFSET>,
            GetRestrictedSink: GetRestrictedSink::<Impl, IMPL_OFFSET>,
            SetBatchingParameters: SetBatchingParameters::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWbemEventSink as ::windows::core::Interface>::IID
    }
}
pub trait IWbemHiPerfEnumImpl: Sized {
    fn AddObjects(&mut self, lflags: i32, unumobjects: u32, apids: *const i32, apobj: *const ::core::option::Option<IWbemObjectAccess>) -> ::windows::core::Result<()>;
    fn RemoveObjects(&mut self, lflags: i32, unumobjects: u32, apids: *const i32) -> ::windows::core::Result<()>;
    fn GetObjects(&mut self, lflags: i32, unumobjects: u32, apobj: *mut ::core::option::Option<IWbemObjectAccess>, pureturned: *mut u32) -> ::windows::core::Result<()>;
    fn RemoveAll(&mut self, lflags: i32) -> ::windows::core::Result<()>;
}
impl IWbemHiPerfEnumVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWbemHiPerfEnumImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWbemHiPerfEnumVtbl {
        unsafe extern "system" fn AddObjects<Impl: IWbemHiPerfEnumImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, unumobjects: u32, apids: *const i32, apobj: *const ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddObjects(::core::mem::transmute_copy(&lflags), ::core::mem::transmute_copy(&unumobjects), ::core::mem::transmute_copy(&apids), ::core::mem::transmute_copy(&apobj)).into()
        }
        unsafe extern "system" fn RemoveObjects<Impl: IWbemHiPerfEnumImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, unumobjects: u32, apids: *const i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveObjects(::core::mem::transmute_copy(&lflags), ::core::mem::transmute_copy(&unumobjects), ::core::mem::transmute_copy(&apids)).into()
        }
        unsafe extern "system" fn GetObjects<Impl: IWbemHiPerfEnumImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, unumobjects: u32, apobj: *mut ::windows::core::RawPtr, pureturned: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetObjects(::core::mem::transmute_copy(&lflags), ::core::mem::transmute_copy(&unumobjects), ::core::mem::transmute_copy(&apobj), ::core::mem::transmute_copy(&pureturned)).into()
        }
        unsafe extern "system" fn RemoveAll<Impl: IWbemHiPerfEnumImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAll(::core::mem::transmute_copy(&lflags)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            AddObjects: AddObjects::<Impl, IMPL_OFFSET>,
            RemoveObjects: RemoveObjects::<Impl, IMPL_OFFSET>,
            GetObjects: GetObjects::<Impl, IMPL_OFFSET>,
            RemoveAll: RemoveAll::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWbemHiPerfEnum as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWbemHiPerfProviderImpl: Sized {
    fn QueryInstances(&mut self, pnamespace: ::core::option::Option<IWbemServices>, wszclass: super::super::Foundation::PWSTR, lflags: i32, pctx: ::core::option::Option<IWbemContext>, psink: ::core::option::Option<IWbemObjectSink>) -> ::windows::core::Result<()>;
    fn CreateRefresher(&mut self, pnamespace: ::core::option::Option<IWbemServices>, lflags: i32) -> ::windows::core::Result<IWbemRefresher>;
    fn CreateRefreshableObject(&mut self, pnamespace: ::core::option::Option<IWbemServices>, ptemplate: ::core::option::Option<IWbemObjectAccess>, prefresher: ::core::option::Option<IWbemRefresher>, lflags: i32, pcontext: ::core::option::Option<IWbemContext>, pprefreshable: *mut ::core::option::Option<IWbemObjectAccess>, plid: *mut i32) -> ::windows::core::Result<()>;
    fn StopRefreshing(&mut self, prefresher: ::core::option::Option<IWbemRefresher>, lid: i32, lflags: i32) -> ::windows::core::Result<()>;
    fn CreateRefreshableEnum(&mut self, pnamespace: ::core::option::Option<IWbemServices>, wszclass: super::super::Foundation::PWSTR, prefresher: ::core::option::Option<IWbemRefresher>, lflags: i32, pcontext: ::core::option::Option<IWbemContext>, phiperfenum: ::core::option::Option<IWbemHiPerfEnum>) -> ::windows::core::Result<i32>;
    fn GetObjects(&mut self, pnamespace: ::core::option::Option<IWbemServices>, lnumobjects: i32, apobj: *mut ::core::option::Option<IWbemObjectAccess>, lflags: i32, pcontext: ::core::option::Option<IWbemContext>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWbemHiPerfProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWbemHiPerfProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWbemHiPerfProviderVtbl {
        unsafe extern "system" fn QueryInstances<Impl: IWbemHiPerfProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnamespace: ::windows::core::RawPtr, wszclass: super::super::Foundation::PWSTR, lflags: i32, pctx: ::windows::core::RawPtr, psink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).QueryInstances(::core::mem::transmute(&pnamespace), ::core::mem::transmute_copy(&wszclass), ::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&pctx), ::core::mem::transmute(&psink)).into()
        }
        unsafe extern "system" fn CreateRefresher<Impl: IWbemHiPerfProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnamespace: ::windows::core::RawPtr, lflags: i32, pprefresher: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateRefresher(::core::mem::transmute(&pnamespace), ::core::mem::transmute_copy(&lflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *pprefresher = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRefreshableObject<Impl: IWbemHiPerfProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnamespace: ::windows::core::RawPtr, ptemplate: ::windows::core::RawPtr, prefresher: ::windows::core::RawPtr, lflags: i32, pcontext: ::windows::core::RawPtr, pprefreshable: *mut ::windows::core::RawPtr, plid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateRefreshableObject(::core::mem::transmute(&pnamespace), ::core::mem::transmute(&ptemplate), ::core::mem::transmute(&prefresher), ::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&pcontext), ::core::mem::transmute_copy(&pprefreshable), ::core::mem::transmute_copy(&plid)).into()
        }
        unsafe extern "system" fn StopRefreshing<Impl: IWbemHiPerfProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prefresher: ::windows::core::RawPtr, lid: i32, lflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StopRefreshing(::core::mem::transmute(&prefresher), ::core::mem::transmute_copy(&lid), ::core::mem::transmute_copy(&lflags)).into()
        }
        unsafe extern "system" fn CreateRefreshableEnum<Impl: IWbemHiPerfProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnamespace: ::windows::core::RawPtr, wszclass: super::super::Foundation::PWSTR, prefresher: ::windows::core::RawPtr, lflags: i32, pcontext: ::windows::core::RawPtr, phiperfenum: ::windows::core::RawPtr, plid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateRefreshableEnum(::core::mem::transmute(&pnamespace), ::core::mem::transmute_copy(&wszclass), ::core::mem::transmute(&prefresher), ::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&pcontext), ::core::mem::transmute(&phiperfenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *plid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetObjects<Impl: IWbemHiPerfProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnamespace: ::windows::core::RawPtr, lnumobjects: i32, apobj: *mut ::windows::core::RawPtr, lflags: i32, pcontext: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetObjects(::core::mem::transmute(&pnamespace), ::core::mem::transmute_copy(&lnumobjects), ::core::mem::transmute_copy(&apobj), ::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&pcontext)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            QueryInstances: QueryInstances::<Impl, IMPL_OFFSET>,
            CreateRefresher: CreateRefresher::<Impl, IMPL_OFFSET>,
            CreateRefreshableObject: CreateRefreshableObject::<Impl, IMPL_OFFSET>,
            StopRefreshing: StopRefreshing::<Impl, IMPL_OFFSET>,
            CreateRefreshableEnum: CreateRefreshableEnum::<Impl, IMPL_OFFSET>,
            GetObjects: GetObjects::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWbemHiPerfProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWbemLevel1LoginImpl: Sized {
    fn EstablishPosition(&mut self, wszlocalelist: super::super::Foundation::PWSTR, dwnumlocales: u32) -> ::windows::core::Result<u32>;
    fn RequestChallenge(&mut self, wsznetworkresource: super::super::Foundation::PWSTR, wszuser: super::super::Foundation::PWSTR) -> ::windows::core::Result<u8>;
    fn WBEMLogin(&mut self, wszpreferredlocale: super::super::Foundation::PWSTR, accesstoken: *const u8, lflags: i32, pctx: ::core::option::Option<IWbemContext>) -> ::windows::core::Result<IWbemServices>;
    fn NTLMLogin(&mut self, wsznetworkresource: super::super::Foundation::PWSTR, wszpreferredlocale: super::super::Foundation::PWSTR, lflags: i32, pctx: ::core::option::Option<IWbemContext>) -> ::windows::core::Result<IWbemServices>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWbemLevel1LoginVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWbemLevel1LoginImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWbemLevel1LoginVtbl {
        unsafe extern "system" fn EstablishPosition<Impl: IWbemLevel1LoginImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszlocalelist: super::super::Foundation::PWSTR, dwnumlocales: u32, reserved: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EstablishPosition(::core::mem::transmute_copy(&wszlocalelist), ::core::mem::transmute_copy(&dwnumlocales)) {
                ::core::result::Result::Ok(ok__) => {
                    *reserved = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestChallenge<Impl: IWbemLevel1LoginImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wsznetworkresource: super::super::Foundation::PWSTR, wszuser: super::super::Foundation::PWSTR, nonce: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestChallenge(::core::mem::transmute_copy(&wsznetworkresource), ::core::mem::transmute_copy(&wszuser)) {
                ::core::result::Result::Ok(ok__) => {
                    *nonce = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WBEMLogin<Impl: IWbemLevel1LoginImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszpreferredlocale: super::super::Foundation::PWSTR, accesstoken: *const u8, lflags: i32, pctx: ::windows::core::RawPtr, ppnamespace: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WBEMLogin(::core::mem::transmute_copy(&wszpreferredlocale), ::core::mem::transmute_copy(&accesstoken), ::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&pctx)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppnamespace = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NTLMLogin<Impl: IWbemLevel1LoginImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wsznetworkresource: super::super::Foundation::PWSTR, wszpreferredlocale: super::super::Foundation::PWSTR, lflags: i32, pctx: ::windows::core::RawPtr, ppnamespace: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NTLMLogin(::core::mem::transmute_copy(&wsznetworkresource), ::core::mem::transmute_copy(&wszpreferredlocale), ::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&pctx)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppnamespace = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            EstablishPosition: EstablishPosition::<Impl, IMPL_OFFSET>,
            RequestChallenge: RequestChallenge::<Impl, IMPL_OFFSET>,
            WBEMLogin: WBEMLogin::<Impl, IMPL_OFFSET>,
            NTLMLogin: NTLMLogin::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWbemLevel1Login as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWbemLocatorImpl: Sized {
    fn ConnectServer(&mut self, strnetworkresource: super::super::Foundation::BSTR, struser: super::super::Foundation::BSTR, strpassword: super::super::Foundation::BSTR, strlocale: super::super::Foundation::BSTR, lsecurityflags: i32, strauthority: super::super::Foundation::BSTR, pctx: ::core::option::Option<IWbemContext>) -> ::windows::core::Result<IWbemServices>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWbemLocatorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWbemLocatorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWbemLocatorVtbl {
        unsafe extern "system" fn ConnectServer<Impl: IWbemLocatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strnetworkresource: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, struser: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strlocale: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lsecurityflags: i32, strauthority: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pctx: ::windows::core::RawPtr, ppnamespace: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConnectServer(::core::mem::transmute_copy(&strnetworkresource), ::core::mem::transmute_copy(&struser), ::core::mem::transmute_copy(&strpassword), ::core::mem::transmute_copy(&strlocale), ::core::mem::transmute_copy(&lsecurityflags), ::core::mem::transmute_copy(&strauthority), ::core::mem::transmute(&pctx)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppnamespace = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), ConnectServer: ConnectServer::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWbemLocator as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWbemObjectAccessImpl: Sized + IWbemClassObjectImpl {
    fn GetPropertyHandle(&mut self, wszpropertyname: super::super::Foundation::PWSTR, ptype: *mut i32, plhandle: *mut i32) -> ::windows::core::Result<()>;
    fn WritePropertyValue(&mut self, lhandle: i32, lnumbytes: i32, adata: *const u8) -> ::windows::core::Result<()>;
    fn ReadPropertyValue(&mut self, lhandle: i32, lbuffersize: i32, plnumbytes: *mut i32, adata: *mut u8) -> ::windows::core::Result<()>;
    fn ReadDWORD(&mut self, lhandle: i32) -> ::windows::core::Result<u32>;
    fn WriteDWORD(&mut self, lhandle: i32, dw: u32) -> ::windows::core::Result<()>;
    fn ReadQWORD(&mut self, lhandle: i32) -> ::windows::core::Result<u64>;
    fn WriteQWORD(&mut self, lhandle: i32, pw: u64) -> ::windows::core::Result<()>;
    fn GetPropertyInfoByHandle(&mut self, lhandle: i32, pstrname: *mut super::super::Foundation::BSTR, ptype: *mut i32) -> ::windows::core::Result<()>;
    fn Lock(&mut self, lflags: i32) -> ::windows::core::Result<()>;
    fn Unlock(&mut self, lflags: i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWbemObjectAccessVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWbemObjectAccessImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWbemObjectAccessVtbl {
        unsafe extern "system" fn GetPropertyHandle<Impl: IWbemObjectAccessImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszpropertyname: super::super::Foundation::PWSTR, ptype: *mut i32, plhandle: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetPropertyHandle(::core::mem::transmute_copy(&wszpropertyname), ::core::mem::transmute_copy(&ptype), ::core::mem::transmute_copy(&plhandle)).into()
        }
        unsafe extern "system" fn WritePropertyValue<Impl: IWbemObjectAccessImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lhandle: i32, lnumbytes: i32, adata: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WritePropertyValue(::core::mem::transmute_copy(&lhandle), ::core::mem::transmute_copy(&lnumbytes), ::core::mem::transmute_copy(&adata)).into()
        }
        unsafe extern "system" fn ReadPropertyValue<Impl: IWbemObjectAccessImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lhandle: i32, lbuffersize: i32, plnumbytes: *mut i32, adata: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReadPropertyValue(::core::mem::transmute_copy(&lhandle), ::core::mem::transmute_copy(&lbuffersize), ::core::mem::transmute_copy(&plnumbytes), ::core::mem::transmute_copy(&adata)).into()
        }
        unsafe extern "system" fn ReadDWORD<Impl: IWbemObjectAccessImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lhandle: i32, pdw: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReadDWORD(::core::mem::transmute_copy(&lhandle)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdw = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WriteDWORD<Impl: IWbemObjectAccessImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lhandle: i32, dw: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WriteDWORD(::core::mem::transmute_copy(&lhandle), ::core::mem::transmute_copy(&dw)).into()
        }
        unsafe extern "system" fn ReadQWORD<Impl: IWbemObjectAccessImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lhandle: i32, pqw: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReadQWORD(::core::mem::transmute_copy(&lhandle)) {
                ::core::result::Result::Ok(ok__) => {
                    *pqw = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WriteQWORD<Impl: IWbemObjectAccessImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lhandle: i32, pw: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WriteQWORD(::core::mem::transmute_copy(&lhandle), ::core::mem::transmute_copy(&pw)).into()
        }
        unsafe extern "system" fn GetPropertyInfoByHandle<Impl: IWbemObjectAccessImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lhandle: i32, pstrname: *mut super::super::Foundation::BSTR, ptype: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetPropertyInfoByHandle(::core::mem::transmute_copy(&lhandle), ::core::mem::transmute_copy(&pstrname), ::core::mem::transmute_copy(&ptype)).into()
        }
        unsafe extern "system" fn Lock<Impl: IWbemObjectAccessImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Lock(::core::mem::transmute_copy(&lflags)).into()
        }
        unsafe extern "system" fn Unlock<Impl: IWbemObjectAccessImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Unlock(::core::mem::transmute_copy(&lflags)).into()
        }
        Self {
            base: IWbemClassObjectVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetPropertyHandle: GetPropertyHandle::<Impl, IMPL_OFFSET>,
            WritePropertyValue: WritePropertyValue::<Impl, IMPL_OFFSET>,
            ReadPropertyValue: ReadPropertyValue::<Impl, IMPL_OFFSET>,
            ReadDWORD: ReadDWORD::<Impl, IMPL_OFFSET>,
            WriteDWORD: WriteDWORD::<Impl, IMPL_OFFSET>,
            ReadQWORD: ReadQWORD::<Impl, IMPL_OFFSET>,
            WriteQWORD: WriteQWORD::<Impl, IMPL_OFFSET>,
            GetPropertyInfoByHandle: GetPropertyInfoByHandle::<Impl, IMPL_OFFSET>,
            Lock: Lock::<Impl, IMPL_OFFSET>,
            Unlock: Unlock::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWbemObjectAccess as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWbemObjectSinkImpl: Sized {
    fn Indicate(&mut self, lobjectcount: i32, apobjarray: *const ::core::option::Option<IWbemClassObject>) -> ::windows::core::Result<()>;
    fn SetStatus(&mut self, lflags: i32, hresult: ::windows::core::HRESULT, strparam: super::super::Foundation::BSTR, pobjparam: ::core::option::Option<IWbemClassObject>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWbemObjectSinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWbemObjectSinkImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWbemObjectSinkVtbl {
        unsafe extern "system" fn Indicate<Impl: IWbemObjectSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lobjectcount: i32, apobjarray: *const ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Indicate(::core::mem::transmute_copy(&lobjectcount), ::core::mem::transmute_copy(&apobjarray)).into()
        }
        unsafe extern "system" fn SetStatus<Impl: IWbemObjectSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, hresult: ::windows::core::HRESULT, strparam: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pobjparam: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStatus(::core::mem::transmute_copy(&lflags), ::core::mem::transmute_copy(&hresult), ::core::mem::transmute_copy(&strparam), ::core::mem::transmute(&pobjparam)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Indicate: Indicate::<Impl, IMPL_OFFSET>,
            SetStatus: SetStatus::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWbemObjectSink as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWbemObjectSinkExImpl: Sized + IWbemObjectSinkImpl {
    fn WriteMessage(&mut self, uchannel: u32, strmessage: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn WriteError(&mut self, pobjerror: ::core::option::Option<IWbemClassObject>) -> ::windows::core::Result<u8>;
    fn PromptUser(&mut self, strmessage: super::super::Foundation::BSTR, uprompttype: u8) -> ::windows::core::Result<u8>;
    fn WriteProgress(&mut self, stractivity: super::super::Foundation::BSTR, strcurrentoperation: super::super::Foundation::BSTR, strstatusdescription: super::super::Foundation::BSTR, upercentcomplete: u32, usecondsremaining: u32) -> ::windows::core::Result<()>;
    fn WriteStreamParameter(&mut self, strname: super::super::Foundation::BSTR, vtvalue: *const super::Com::VARIANT, ultype: u32, ulflags: u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWbemObjectSinkExVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWbemObjectSinkExImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWbemObjectSinkExVtbl {
        unsafe extern "system" fn WriteMessage<Impl: IWbemObjectSinkExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uchannel: u32, strmessage: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WriteMessage(::core::mem::transmute_copy(&uchannel), ::core::mem::transmute_copy(&strmessage)).into()
        }
        unsafe extern "system" fn WriteError<Impl: IWbemObjectSinkExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pobjerror: ::windows::core::RawPtr, pureturned: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WriteError(::core::mem::transmute(&pobjerror)) {
                ::core::result::Result::Ok(ok__) => {
                    *pureturned = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PromptUser<Impl: IWbemObjectSinkExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strmessage: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, uprompttype: u8, pureturned: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PromptUser(::core::mem::transmute_copy(&strmessage), ::core::mem::transmute_copy(&uprompttype)) {
                ::core::result::Result::Ok(ok__) => {
                    *pureturned = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WriteProgress<Impl: IWbemObjectSinkExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stractivity: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strcurrentoperation: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strstatusdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, upercentcomplete: u32, usecondsremaining: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WriteProgress(::core::mem::transmute_copy(&stractivity), ::core::mem::transmute_copy(&strcurrentoperation), ::core::mem::transmute_copy(&strstatusdescription), ::core::mem::transmute_copy(&upercentcomplete), ::core::mem::transmute_copy(&usecondsremaining)).into()
        }
        unsafe extern "system" fn WriteStreamParameter<Impl: IWbemObjectSinkExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, vtvalue: *const super::Com::VARIANT, ultype: u32, ulflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WriteStreamParameter(::core::mem::transmute_copy(&strname), ::core::mem::transmute_copy(&vtvalue), ::core::mem::transmute_copy(&ultype), ::core::mem::transmute_copy(&ulflags)).into()
        }
        Self {
            base: IWbemObjectSinkVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            WriteMessage: WriteMessage::<Impl, IMPL_OFFSET>,
            WriteError: WriteError::<Impl, IMPL_OFFSET>,
            PromptUser: PromptUser::<Impl, IMPL_OFFSET>,
            WriteProgress: WriteProgress::<Impl, IMPL_OFFSET>,
            WriteStreamParameter: WriteStreamParameter::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWbemObjectSinkEx as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWbemObjectTextSrcImpl: Sized {
    fn GetText(&mut self, lflags: i32, pobj: ::core::option::Option<IWbemClassObject>, uobjtextformat: u32, pctx: ::core::option::Option<IWbemContext>) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn CreateFromText(&mut self, lflags: i32, strtext: super::super::Foundation::BSTR, uobjtextformat: u32, pctx: ::core::option::Option<IWbemContext>) -> ::windows::core::Result<IWbemClassObject>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWbemObjectTextSrcVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWbemObjectTextSrcImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWbemObjectTextSrcVtbl {
        unsafe extern "system" fn GetText<Impl: IWbemObjectTextSrcImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, pobj: ::windows::core::RawPtr, uobjtextformat: u32, pctx: ::windows::core::RawPtr, strtext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetText(::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&pobj), ::core::mem::transmute_copy(&uobjtextformat), ::core::mem::transmute(&pctx)) {
                ::core::result::Result::Ok(ok__) => {
                    *strtext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromText<Impl: IWbemObjectTextSrcImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, strtext: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, uobjtextformat: u32, pctx: ::windows::core::RawPtr, pnewobj: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromText(::core::mem::transmute_copy(&lflags), ::core::mem::transmute_copy(&strtext), ::core::mem::transmute_copy(&uobjtextformat), ::core::mem::transmute(&pctx)) {
                ::core::result::Result::Ok(ok__) => {
                    *pnewobj = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetText: GetText::<Impl, IMPL_OFFSET>,
            CreateFromText: CreateFromText::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWbemObjectTextSrc as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWbemPathImpl: Sized {
    fn SetText(&mut self, umode: u32, pszpath: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn GetText(&mut self, lflags: i32, pubufflength: *mut u32, psztext: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn GetInfo(&mut self, urequestedinfo: u32) -> ::windows::core::Result<u64>;
    fn SetServer(&mut self, name: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn GetServer(&mut self, punamebuflength: *mut u32, pname: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn GetNamespaceCount(&mut self) -> ::windows::core::Result<u32>;
    fn SetNamespaceAt(&mut self, uindex: u32, pszname: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn GetNamespaceAt(&mut self, uindex: u32, punamebuflength: *mut u32, pname: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn RemoveNamespaceAt(&mut self, uindex: u32) -> ::windows::core::Result<()>;
    fn RemoveAllNamespaces(&mut self) -> ::windows::core::Result<()>;
    fn GetScopeCount(&mut self) -> ::windows::core::Result<u32>;
    fn SetScope(&mut self, uindex: u32, pszclass: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn SetScopeFromText(&mut self, uindex: u32, psztext: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn GetScope(&mut self, uindex: u32, puclassnamebufsize: *mut u32, pszclass: super::super::Foundation::PWSTR, pkeylist: *mut ::core::option::Option<IWbemPathKeyList>) -> ::windows::core::Result<()>;
    fn GetScopeAsText(&mut self, uindex: u32, putextbufsize: *mut u32, psztext: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn RemoveScope(&mut self, uindex: u32) -> ::windows::core::Result<()>;
    fn RemoveAllScopes(&mut self) -> ::windows::core::Result<()>;
    fn SetClassName(&mut self, name: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn GetClassName(&mut self, pubufflength: *mut u32, pszname: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn GetKeyList(&mut self) -> ::windows::core::Result<IWbemPathKeyList>;
    fn CreateClassPart(&mut self, lflags: i32, name: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn DeleteClassPart(&mut self, lflags: i32) -> ::windows::core::Result<()>;
    fn IsRelative(&mut self, wszmachine: super::super::Foundation::PWSTR, wsznamespace: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    fn IsRelativeOrChild(&mut self, wszmachine: super::super::Foundation::PWSTR, wsznamespace: super::super::Foundation::PWSTR, lflags: i32) -> super::super::Foundation::BOOL;
    fn IsLocal(&mut self, wszmachine: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    fn IsSameClassName(&mut self, wszclass: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
}
#[cfg(feature = "Win32_Foundation")]
impl IWbemPathVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWbemPathImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWbemPathVtbl {
        unsafe extern "system" fn SetText<Impl: IWbemPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, umode: u32, pszpath: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetText(::core::mem::transmute_copy(&umode), ::core::mem::transmute_copy(&pszpath)).into()
        }
        unsafe extern "system" fn GetText<Impl: IWbemPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, pubufflength: *mut u32, psztext: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetText(::core::mem::transmute_copy(&lflags), ::core::mem::transmute_copy(&pubufflength), ::core::mem::transmute_copy(&psztext)).into()
        }
        unsafe extern "system" fn GetInfo<Impl: IWbemPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, urequestedinfo: u32, puresponse: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetInfo(::core::mem::transmute_copy(&urequestedinfo)) {
                ::core::result::Result::Ok(ok__) => {
                    *puresponse = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetServer<Impl: IWbemPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetServer(::core::mem::transmute_copy(&name)).into()
        }
        unsafe extern "system" fn GetServer<Impl: IWbemPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punamebuflength: *mut u32, pname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetServer(::core::mem::transmute_copy(&punamebuflength), ::core::mem::transmute_copy(&pname)).into()
        }
        unsafe extern "system" fn GetNamespaceCount<Impl: IWbemPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pucount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNamespaceCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pucount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNamespaceAt<Impl: IWbemPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uindex: u32, pszname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNamespaceAt(::core::mem::transmute_copy(&uindex), ::core::mem::transmute_copy(&pszname)).into()
        }
        unsafe extern "system" fn GetNamespaceAt<Impl: IWbemPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uindex: u32, punamebuflength: *mut u32, pname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetNamespaceAt(::core::mem::transmute_copy(&uindex), ::core::mem::transmute_copy(&punamebuflength), ::core::mem::transmute_copy(&pname)).into()
        }
        unsafe extern "system" fn RemoveNamespaceAt<Impl: IWbemPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveNamespaceAt(::core::mem::transmute_copy(&uindex)).into()
        }
        unsafe extern "system" fn RemoveAllNamespaces<Impl: IWbemPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAllNamespaces().into()
        }
        unsafe extern "system" fn GetScopeCount<Impl: IWbemPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pucount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetScopeCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pucount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetScope<Impl: IWbemPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uindex: u32, pszclass: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetScope(::core::mem::transmute_copy(&uindex), ::core::mem::transmute_copy(&pszclass)).into()
        }
        unsafe extern "system" fn SetScopeFromText<Impl: IWbemPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uindex: u32, psztext: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetScopeFromText(::core::mem::transmute_copy(&uindex), ::core::mem::transmute_copy(&psztext)).into()
        }
        unsafe extern "system" fn GetScope<Impl: IWbemPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uindex: u32, puclassnamebufsize: *mut u32, pszclass: super::super::Foundation::PWSTR, pkeylist: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetScope(::core::mem::transmute_copy(&uindex), ::core::mem::transmute_copy(&puclassnamebufsize), ::core::mem::transmute_copy(&pszclass), ::core::mem::transmute_copy(&pkeylist)).into()
        }
        unsafe extern "system" fn GetScopeAsText<Impl: IWbemPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uindex: u32, putextbufsize: *mut u32, psztext: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetScopeAsText(::core::mem::transmute_copy(&uindex), ::core::mem::transmute_copy(&putextbufsize), ::core::mem::transmute_copy(&psztext)).into()
        }
        unsafe extern "system" fn RemoveScope<Impl: IWbemPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveScope(::core::mem::transmute_copy(&uindex)).into()
        }
        unsafe extern "system" fn RemoveAllScopes<Impl: IWbemPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAllScopes().into()
        }
        unsafe extern "system" fn SetClassName<Impl: IWbemPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetClassName(::core::mem::transmute_copy(&name)).into()
        }
        unsafe extern "system" fn GetClassName<Impl: IWbemPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pubufflength: *mut u32, pszname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetClassName(::core::mem::transmute_copy(&pubufflength), ::core::mem::transmute_copy(&pszname)).into()
        }
        unsafe extern "system" fn GetKeyList<Impl: IWbemPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pout: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetKeyList() {
                ::core::result::Result::Ok(ok__) => {
                    *pout = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateClassPart<Impl: IWbemPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, name: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateClassPart(::core::mem::transmute_copy(&lflags), ::core::mem::transmute_copy(&name)).into()
        }
        unsafe extern "system" fn DeleteClassPart<Impl: IWbemPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteClassPart(::core::mem::transmute_copy(&lflags)).into()
        }
        unsafe extern "system" fn IsRelative<Impl: IWbemPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszmachine: super::super::Foundation::PWSTR, wsznamespace: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IsRelative(::core::mem::transmute_copy(&wszmachine), ::core::mem::transmute_copy(&wsznamespace))
        }
        unsafe extern "system" fn IsRelativeOrChild<Impl: IWbemPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszmachine: super::super::Foundation::PWSTR, wsznamespace: super::super::Foundation::PWSTR, lflags: i32) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IsRelativeOrChild(::core::mem::transmute_copy(&wszmachine), ::core::mem::transmute_copy(&wsznamespace), ::core::mem::transmute_copy(&lflags))
        }
        unsafe extern "system" fn IsLocal<Impl: IWbemPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszmachine: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IsLocal(::core::mem::transmute_copy(&wszmachine))
        }
        unsafe extern "system" fn IsSameClassName<Impl: IWbemPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszclass: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IsSameClassName(::core::mem::transmute_copy(&wszclass))
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetText: SetText::<Impl, IMPL_OFFSET>,
            GetText: GetText::<Impl, IMPL_OFFSET>,
            GetInfo: GetInfo::<Impl, IMPL_OFFSET>,
            SetServer: SetServer::<Impl, IMPL_OFFSET>,
            GetServer: GetServer::<Impl, IMPL_OFFSET>,
            GetNamespaceCount: GetNamespaceCount::<Impl, IMPL_OFFSET>,
            SetNamespaceAt: SetNamespaceAt::<Impl, IMPL_OFFSET>,
            GetNamespaceAt: GetNamespaceAt::<Impl, IMPL_OFFSET>,
            RemoveNamespaceAt: RemoveNamespaceAt::<Impl, IMPL_OFFSET>,
            RemoveAllNamespaces: RemoveAllNamespaces::<Impl, IMPL_OFFSET>,
            GetScopeCount: GetScopeCount::<Impl, IMPL_OFFSET>,
            SetScope: SetScope::<Impl, IMPL_OFFSET>,
            SetScopeFromText: SetScopeFromText::<Impl, IMPL_OFFSET>,
            GetScope: GetScope::<Impl, IMPL_OFFSET>,
            GetScopeAsText: GetScopeAsText::<Impl, IMPL_OFFSET>,
            RemoveScope: RemoveScope::<Impl, IMPL_OFFSET>,
            RemoveAllScopes: RemoveAllScopes::<Impl, IMPL_OFFSET>,
            SetClassName: SetClassName::<Impl, IMPL_OFFSET>,
            GetClassName: GetClassName::<Impl, IMPL_OFFSET>,
            GetKeyList: GetKeyList::<Impl, IMPL_OFFSET>,
            CreateClassPart: CreateClassPart::<Impl, IMPL_OFFSET>,
            DeleteClassPart: DeleteClassPart::<Impl, IMPL_OFFSET>,
            IsRelative: IsRelative::<Impl, IMPL_OFFSET>,
            IsRelativeOrChild: IsRelativeOrChild::<Impl, IMPL_OFFSET>,
            IsLocal: IsLocal::<Impl, IMPL_OFFSET>,
            IsSameClassName: IsSameClassName::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWbemPath as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWbemPathKeyListImpl: Sized {
    fn GetCount(&mut self) -> ::windows::core::Result<u32>;
    fn SetKey(&mut self, wszname: super::super::Foundation::PWSTR, uflags: u32, ucimtype: u32, pkeyval: *const ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn SetKey2(&mut self, wszname: super::super::Foundation::PWSTR, uflags: u32, ucimtype: u32, pkeyval: *const super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn GetKey(&mut self, ukeyix: u32, uflags: u32, punamebufsize: *mut u32, pszkeyname: super::super::Foundation::PWSTR, pukeyvalbufsize: *mut u32, pkeyval: *mut ::core::ffi::c_void, puapparentcimtype: *mut u32) -> ::windows::core::Result<()>;
    fn GetKey2(&mut self, ukeyix: u32, uflags: u32, punamebufsize: *mut u32, pszkeyname: super::super::Foundation::PWSTR, pkeyvalue: *mut super::Com::VARIANT, puapparentcimtype: *mut u32) -> ::windows::core::Result<()>;
    fn RemoveKey(&mut self, wszname: super::super::Foundation::PWSTR, uflags: u32) -> ::windows::core::Result<()>;
    fn RemoveAllKeys(&mut self, uflags: u32) -> ::windows::core::Result<()>;
    fn MakeSingleton(&mut self, bset: u8) -> ::windows::core::Result<()>;
    fn GetInfo(&mut self, urequestedinfo: u32) -> ::windows::core::Result<u64>;
    fn GetText(&mut self, lflags: i32, pubufflength: *mut u32, psztext: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWbemPathKeyListVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWbemPathKeyListImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWbemPathKeyListVtbl {
        unsafe extern "system" fn GetCount<Impl: IWbemPathKeyListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pukeycount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pukeycount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetKey<Impl: IWbemPathKeyListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszname: super::super::Foundation::PWSTR, uflags: u32, ucimtype: u32, pkeyval: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetKey(::core::mem::transmute_copy(&wszname), ::core::mem::transmute_copy(&uflags), ::core::mem::transmute_copy(&ucimtype), ::core::mem::transmute_copy(&pkeyval)).into()
        }
        unsafe extern "system" fn SetKey2<Impl: IWbemPathKeyListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszname: super::super::Foundation::PWSTR, uflags: u32, ucimtype: u32, pkeyval: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetKey2(::core::mem::transmute_copy(&wszname), ::core::mem::transmute_copy(&uflags), ::core::mem::transmute_copy(&ucimtype), ::core::mem::transmute_copy(&pkeyval)).into()
        }
        unsafe extern "system" fn GetKey<Impl: IWbemPathKeyListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ukeyix: u32, uflags: u32, punamebufsize: *mut u32, pszkeyname: super::super::Foundation::PWSTR, pukeyvalbufsize: *mut u32, pkeyval: *mut ::core::ffi::c_void, puapparentcimtype: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetKey(::core::mem::transmute_copy(&ukeyix), ::core::mem::transmute_copy(&uflags), ::core::mem::transmute_copy(&punamebufsize), ::core::mem::transmute_copy(&pszkeyname), ::core::mem::transmute_copy(&pukeyvalbufsize), ::core::mem::transmute_copy(&pkeyval), ::core::mem::transmute_copy(&puapparentcimtype)).into()
        }
        unsafe extern "system" fn GetKey2<Impl: IWbemPathKeyListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ukeyix: u32, uflags: u32, punamebufsize: *mut u32, pszkeyname: super::super::Foundation::PWSTR, pkeyvalue: *mut super::Com::VARIANT, puapparentcimtype: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetKey2(::core::mem::transmute_copy(&ukeyix), ::core::mem::transmute_copy(&uflags), ::core::mem::transmute_copy(&punamebufsize), ::core::mem::transmute_copy(&pszkeyname), ::core::mem::transmute_copy(&pkeyvalue), ::core::mem::transmute_copy(&puapparentcimtype)).into()
        }
        unsafe extern "system" fn RemoveKey<Impl: IWbemPathKeyListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszname: super::super::Foundation::PWSTR, uflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveKey(::core::mem::transmute_copy(&wszname), ::core::mem::transmute_copy(&uflags)).into()
        }
        unsafe extern "system" fn RemoveAllKeys<Impl: IWbemPathKeyListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAllKeys(::core::mem::transmute_copy(&uflags)).into()
        }
        unsafe extern "system" fn MakeSingleton<Impl: IWbemPathKeyListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bset: u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).MakeSingleton(::core::mem::transmute_copy(&bset)).into()
        }
        unsafe extern "system" fn GetInfo<Impl: IWbemPathKeyListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, urequestedinfo: u32, puresponse: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetInfo(::core::mem::transmute_copy(&urequestedinfo)) {
                ::core::result::Result::Ok(ok__) => {
                    *puresponse = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetText<Impl: IWbemPathKeyListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, pubufflength: *mut u32, psztext: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetText(::core::mem::transmute_copy(&lflags), ::core::mem::transmute_copy(&pubufflength), ::core::mem::transmute_copy(&psztext)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetCount: GetCount::<Impl, IMPL_OFFSET>,
            SetKey: SetKey::<Impl, IMPL_OFFSET>,
            SetKey2: SetKey2::<Impl, IMPL_OFFSET>,
            GetKey: GetKey::<Impl, IMPL_OFFSET>,
            GetKey2: GetKey2::<Impl, IMPL_OFFSET>,
            RemoveKey: RemoveKey::<Impl, IMPL_OFFSET>,
            RemoveAllKeys: RemoveAllKeys::<Impl, IMPL_OFFSET>,
            MakeSingleton: MakeSingleton::<Impl, IMPL_OFFSET>,
            GetInfo: GetInfo::<Impl, IMPL_OFFSET>,
            GetText: GetText::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWbemPathKeyList as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWbemPropertyProviderImpl: Sized {
    fn GetProperty(&mut self, lflags: i32, strlocale: super::super::Foundation::BSTR, strclassmapping: super::super::Foundation::BSTR, strinstmapping: super::super::Foundation::BSTR, strpropmapping: super::super::Foundation::BSTR) -> ::windows::core::Result<super::Com::VARIANT>;
    fn PutProperty(&mut self, lflags: i32, strlocale: super::super::Foundation::BSTR, strclassmapping: super::super::Foundation::BSTR, strinstmapping: super::super::Foundation::BSTR, strpropmapping: super::super::Foundation::BSTR, pvvalue: *const super::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWbemPropertyProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWbemPropertyProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWbemPropertyProviderVtbl {
        unsafe extern "system" fn GetProperty<Impl: IWbemPropertyProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, strlocale: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strclassmapping: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strinstmapping: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strpropmapping: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvvalue: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProperty(::core::mem::transmute_copy(&lflags), ::core::mem::transmute_copy(&strlocale), ::core::mem::transmute_copy(&strclassmapping), ::core::mem::transmute_copy(&strinstmapping), ::core::mem::transmute_copy(&strpropmapping)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PutProperty<Impl: IWbemPropertyProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, strlocale: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strclassmapping: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strinstmapping: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strpropmapping: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvvalue: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PutProperty(::core::mem::transmute_copy(&lflags), ::core::mem::transmute_copy(&strlocale), ::core::mem::transmute_copy(&strclassmapping), ::core::mem::transmute_copy(&strinstmapping), ::core::mem::transmute_copy(&strpropmapping), ::core::mem::transmute_copy(&pvvalue)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetProperty: GetProperty::<Impl, IMPL_OFFSET>,
            PutProperty: PutProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWbemPropertyProvider as ::windows::core::Interface>::IID
    }
}
pub trait IWbemProviderIdentityImpl: Sized {
    fn SetRegistrationObject(&mut self, lflags: i32, pprovreg: ::core::option::Option<IWbemClassObject>) -> ::windows::core::Result<()>;
}
impl IWbemProviderIdentityVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWbemProviderIdentityImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWbemProviderIdentityVtbl {
        unsafe extern "system" fn SetRegistrationObject<Impl: IWbemProviderIdentityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, pprovreg: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRegistrationObject(::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&pprovreg)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), SetRegistrationObject: SetRegistrationObject::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWbemProviderIdentity as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWbemProviderInitImpl: Sized {
    fn Initialize(&mut self, wszuser: super::super::Foundation::PWSTR, lflags: i32, wsznamespace: super::super::Foundation::PWSTR, wszlocale: super::super::Foundation::PWSTR, pnamespace: ::core::option::Option<IWbemServices>, pctx: ::core::option::Option<IWbemContext>, pinitsink: ::core::option::Option<IWbemProviderInitSink>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWbemProviderInitVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWbemProviderInitImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWbemProviderInitVtbl {
        unsafe extern "system" fn Initialize<Impl: IWbemProviderInitImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszuser: super::super::Foundation::PWSTR, lflags: i32, wsznamespace: super::super::Foundation::PWSTR, wszlocale: super::super::Foundation::PWSTR, pnamespace: ::windows::core::RawPtr, pctx: ::windows::core::RawPtr, pinitsink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Initialize(::core::mem::transmute_copy(&wszuser), ::core::mem::transmute_copy(&lflags), ::core::mem::transmute_copy(&wsznamespace), ::core::mem::transmute_copy(&wszlocale), ::core::mem::transmute(&pnamespace), ::core::mem::transmute(&pctx), ::core::mem::transmute(&pinitsink)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), Initialize: Initialize::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWbemProviderInit as ::windows::core::Interface>::IID
    }
}
pub trait IWbemProviderInitSinkImpl: Sized {
    fn SetStatus(&mut self, lstatus: i32, lflags: i32) -> ::windows::core::Result<()>;
}
impl IWbemProviderInitSinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWbemProviderInitSinkImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWbemProviderInitSinkVtbl {
        unsafe extern "system" fn SetStatus<Impl: IWbemProviderInitSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lstatus: i32, lflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStatus(::core::mem::transmute_copy(&lstatus), ::core::mem::transmute_copy(&lflags)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), SetStatus: SetStatus::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWbemProviderInitSink as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWbemQualifierSetImpl: Sized {
    fn Get(&mut self, wszname: super::super::Foundation::PWSTR, lflags: i32, pval: *mut super::Com::VARIANT, plflavor: *mut i32) -> ::windows::core::Result<()>;
    fn Put(&mut self, wszname: super::super::Foundation::PWSTR, pval: *const super::Com::VARIANT, lflavor: i32) -> ::windows::core::Result<()>;
    fn Delete(&mut self, wszname: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn GetNames(&mut self, lflags: i32) -> ::windows::core::Result<*mut super::Com::SAFEARRAY>;
    fn BeginEnumeration(&mut self, lflags: i32) -> ::windows::core::Result<()>;
    fn Next(&mut self, lflags: i32, pstrname: *mut super::super::Foundation::BSTR, pval: *mut super::Com::VARIANT, plflavor: *mut i32) -> ::windows::core::Result<()>;
    fn EndEnumeration(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWbemQualifierSetVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWbemQualifierSetImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWbemQualifierSetVtbl {
        unsafe extern "system" fn Get<Impl: IWbemQualifierSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszname: super::super::Foundation::PWSTR, lflags: i32, pval: *mut super::Com::VARIANT, plflavor: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Get(::core::mem::transmute_copy(&wszname), ::core::mem::transmute_copy(&lflags), ::core::mem::transmute_copy(&pval), ::core::mem::transmute_copy(&plflavor)).into()
        }
        unsafe extern "system" fn Put<Impl: IWbemQualifierSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszname: super::super::Foundation::PWSTR, pval: *const super::Com::VARIANT, lflavor: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Put(::core::mem::transmute_copy(&wszname), ::core::mem::transmute_copy(&pval), ::core::mem::transmute_copy(&lflavor)).into()
        }
        unsafe extern "system" fn Delete<Impl: IWbemQualifierSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Delete(::core::mem::transmute_copy(&wszname)).into()
        }
        unsafe extern "system" fn GetNames<Impl: IWbemQualifierSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, pnames: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNames(::core::mem::transmute_copy(&lflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *pnames = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginEnumeration<Impl: IWbemQualifierSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BeginEnumeration(::core::mem::transmute_copy(&lflags)).into()
        }
        unsafe extern "system" fn Next<Impl: IWbemQualifierSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, pstrname: *mut super::super::Foundation::BSTR, pval: *mut super::Com::VARIANT, plflavor: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&lflags), ::core::mem::transmute_copy(&pstrname), ::core::mem::transmute_copy(&pval), ::core::mem::transmute_copy(&plflavor)).into()
        }
        unsafe extern "system" fn EndEnumeration<Impl: IWbemQualifierSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EndEnumeration().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Get: Get::<Impl, IMPL_OFFSET>,
            Put: Put::<Impl, IMPL_OFFSET>,
            Delete: Delete::<Impl, IMPL_OFFSET>,
            GetNames: GetNames::<Impl, IMPL_OFFSET>,
            BeginEnumeration: BeginEnumeration::<Impl, IMPL_OFFSET>,
            Next: Next::<Impl, IMPL_OFFSET>,
            EndEnumeration: EndEnumeration::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWbemQualifierSet as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWbemQueryImpl: Sized {
    fn Empty(&mut self) -> ::windows::core::Result<()>;
    fn SetLanguageFeatures(&mut self, uflags: u32, uarraysize: u32, pufeatures: *const u32) -> ::windows::core::Result<()>;
    fn TestLanguageFeatures(&mut self, uflags: u32, uarraysize: *mut u32, pufeatures: *mut u32) -> ::windows::core::Result<()>;
    fn Parse(&mut self, pszlang: super::super::Foundation::PWSTR, pszquery: super::super::Foundation::PWSTR, uflags: u32) -> ::windows::core::Result<()>;
    fn GetAnalysis(&mut self, uanalysistype: u32, uflags: u32, panalysis: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn FreeMemory(&mut self, pmem: *const ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn GetQueryInfo(&mut self, uanalysistype: u32, uinfoid: u32, ubufsize: u32, pdestbuf: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWbemQueryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWbemQueryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWbemQueryVtbl {
        unsafe extern "system" fn Empty<Impl: IWbemQueryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Empty().into()
        }
        unsafe extern "system" fn SetLanguageFeatures<Impl: IWbemQueryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uflags: u32, uarraysize: u32, pufeatures: *const u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLanguageFeatures(::core::mem::transmute_copy(&uflags), ::core::mem::transmute_copy(&uarraysize), ::core::mem::transmute_copy(&pufeatures)).into()
        }
        unsafe extern "system" fn TestLanguageFeatures<Impl: IWbemQueryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uflags: u32, uarraysize: *mut u32, pufeatures: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TestLanguageFeatures(::core::mem::transmute_copy(&uflags), ::core::mem::transmute_copy(&uarraysize), ::core::mem::transmute_copy(&pufeatures)).into()
        }
        unsafe extern "system" fn Parse<Impl: IWbemQueryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszlang: super::super::Foundation::PWSTR, pszquery: super::super::Foundation::PWSTR, uflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Parse(::core::mem::transmute_copy(&pszlang), ::core::mem::transmute_copy(&pszquery), ::core::mem::transmute_copy(&uflags)).into()
        }
        unsafe extern "system" fn GetAnalysis<Impl: IWbemQueryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uanalysistype: u32, uflags: u32, panalysis: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetAnalysis(::core::mem::transmute_copy(&uanalysistype), ::core::mem::transmute_copy(&uflags), ::core::mem::transmute_copy(&panalysis)).into()
        }
        unsafe extern "system" fn FreeMemory<Impl: IWbemQueryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmem: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FreeMemory(::core::mem::transmute_copy(&pmem)).into()
        }
        unsafe extern "system" fn GetQueryInfo<Impl: IWbemQueryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uanalysistype: u32, uinfoid: u32, ubufsize: u32, pdestbuf: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetQueryInfo(::core::mem::transmute_copy(&uanalysistype), ::core::mem::transmute_copy(&uinfoid), ::core::mem::transmute_copy(&ubufsize), ::core::mem::transmute_copy(&pdestbuf)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Empty: Empty::<Impl, IMPL_OFFSET>,
            SetLanguageFeatures: SetLanguageFeatures::<Impl, IMPL_OFFSET>,
            TestLanguageFeatures: TestLanguageFeatures::<Impl, IMPL_OFFSET>,
            Parse: Parse::<Impl, IMPL_OFFSET>,
            GetAnalysis: GetAnalysis::<Impl, IMPL_OFFSET>,
            FreeMemory: FreeMemory::<Impl, IMPL_OFFSET>,
            GetQueryInfo: GetQueryInfo::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWbemQuery as ::windows::core::Interface>::IID
    }
}
pub trait IWbemRefresherImpl: Sized {
    fn Refresh(&mut self, lflags: i32) -> ::windows::core::Result<()>;
}
impl IWbemRefresherVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWbemRefresherImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWbemRefresherVtbl {
        unsafe extern "system" fn Refresh<Impl: IWbemRefresherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Refresh(::core::mem::transmute_copy(&lflags)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), Refresh: Refresh::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWbemRefresher as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWbemServicesImpl: Sized {
    fn OpenNamespace(&mut self, strnamespace: super::super::Foundation::BSTR, lflags: i32, pctx: ::core::option::Option<IWbemContext>, ppworkingnamespace: *mut ::core::option::Option<IWbemServices>, ppresult: *mut ::core::option::Option<IWbemCallResult>) -> ::windows::core::Result<()>;
    fn CancelAsyncCall(&mut self, psink: ::core::option::Option<IWbemObjectSink>) -> ::windows::core::Result<()>;
    fn QueryObjectSink(&mut self, lflags: i32) -> ::windows::core::Result<IWbemObjectSink>;
    fn GetObject(&mut self, strobjectpath: super::super::Foundation::BSTR, lflags: i32, pctx: ::core::option::Option<IWbemContext>, ppobject: *mut ::core::option::Option<IWbemClassObject>, ppcallresult: *mut ::core::option::Option<IWbemCallResult>) -> ::windows::core::Result<()>;
    fn GetObjectAsync(&mut self, strobjectpath: super::super::Foundation::BSTR, lflags: i32, pctx: ::core::option::Option<IWbemContext>, presponsehandler: ::core::option::Option<IWbemObjectSink>) -> ::windows::core::Result<()>;
    fn PutClass(&mut self, pobject: ::core::option::Option<IWbemClassObject>, lflags: i32, pctx: ::core::option::Option<IWbemContext>, ppcallresult: *mut ::core::option::Option<IWbemCallResult>) -> ::windows::core::Result<()>;
    fn PutClassAsync(&mut self, pobject: ::core::option::Option<IWbemClassObject>, lflags: i32, pctx: ::core::option::Option<IWbemContext>, presponsehandler: ::core::option::Option<IWbemObjectSink>) -> ::windows::core::Result<()>;
    fn DeleteClass(&mut self, strclass: super::super::Foundation::BSTR, lflags: i32, pctx: ::core::option::Option<IWbemContext>, ppcallresult: *mut ::core::option::Option<IWbemCallResult>) -> ::windows::core::Result<()>;
    fn DeleteClassAsync(&mut self, strclass: super::super::Foundation::BSTR, lflags: i32, pctx: ::core::option::Option<IWbemContext>, presponsehandler: ::core::option::Option<IWbemObjectSink>) -> ::windows::core::Result<()>;
    fn CreateClassEnum(&mut self, strsuperclass: super::super::Foundation::BSTR, lflags: i32, pctx: ::core::option::Option<IWbemContext>) -> ::windows::core::Result<IEnumWbemClassObject>;
    fn CreateClassEnumAsync(&mut self, strsuperclass: super::super::Foundation::BSTR, lflags: i32, pctx: ::core::option::Option<IWbemContext>, presponsehandler: ::core::option::Option<IWbemObjectSink>) -> ::windows::core::Result<()>;
    fn PutInstance(&mut self, pinst: ::core::option::Option<IWbemClassObject>, lflags: i32, pctx: ::core::option::Option<IWbemContext>, ppcallresult: *mut ::core::option::Option<IWbemCallResult>) -> ::windows::core::Result<()>;
    fn PutInstanceAsync(&mut self, pinst: ::core::option::Option<IWbemClassObject>, lflags: i32, pctx: ::core::option::Option<IWbemContext>, presponsehandler: ::core::option::Option<IWbemObjectSink>) -> ::windows::core::Result<()>;
    fn DeleteInstance(&mut self, strobjectpath: super::super::Foundation::BSTR, lflags: i32, pctx: ::core::option::Option<IWbemContext>, ppcallresult: *mut ::core::option::Option<IWbemCallResult>) -> ::windows::core::Result<()>;
    fn DeleteInstanceAsync(&mut self, strobjectpath: super::super::Foundation::BSTR, lflags: i32, pctx: ::core::option::Option<IWbemContext>, presponsehandler: ::core::option::Option<IWbemObjectSink>) -> ::windows::core::Result<()>;
    fn CreateInstanceEnum(&mut self, strfilter: super::super::Foundation::BSTR, lflags: i32, pctx: ::core::option::Option<IWbemContext>) -> ::windows::core::Result<IEnumWbemClassObject>;
    fn CreateInstanceEnumAsync(&mut self, strfilter: super::super::Foundation::BSTR, lflags: i32, pctx: ::core::option::Option<IWbemContext>, presponsehandler: ::core::option::Option<IWbemObjectSink>) -> ::windows::core::Result<()>;
    fn ExecQuery(&mut self, strquerylanguage: super::super::Foundation::BSTR, strquery: super::super::Foundation::BSTR, lflags: i32, pctx: ::core::option::Option<IWbemContext>) -> ::windows::core::Result<IEnumWbemClassObject>;
    fn ExecQueryAsync(&mut self, strquerylanguage: super::super::Foundation::BSTR, strquery: super::super::Foundation::BSTR, lflags: i32, pctx: ::core::option::Option<IWbemContext>, presponsehandler: ::core::option::Option<IWbemObjectSink>) -> ::windows::core::Result<()>;
    fn ExecNotificationQuery(&mut self, strquerylanguage: super::super::Foundation::BSTR, strquery: super::super::Foundation::BSTR, lflags: i32, pctx: ::core::option::Option<IWbemContext>) -> ::windows::core::Result<IEnumWbemClassObject>;
    fn ExecNotificationQueryAsync(&mut self, strquerylanguage: super::super::Foundation::BSTR, strquery: super::super::Foundation::BSTR, lflags: i32, pctx: ::core::option::Option<IWbemContext>, presponsehandler: ::core::option::Option<IWbemObjectSink>) -> ::windows::core::Result<()>;
    fn ExecMethod(&mut self, strobjectpath: super::super::Foundation::BSTR, strmethodname: super::super::Foundation::BSTR, lflags: i32, pctx: ::core::option::Option<IWbemContext>, pinparams: ::core::option::Option<IWbemClassObject>, ppoutparams: *mut ::core::option::Option<IWbemClassObject>, ppcallresult: *mut ::core::option::Option<IWbemCallResult>) -> ::windows::core::Result<()>;
    fn ExecMethodAsync(&mut self, strobjectpath: super::super::Foundation::BSTR, strmethodname: super::super::Foundation::BSTR, lflags: i32, pctx: ::core::option::Option<IWbemContext>, pinparams: ::core::option::Option<IWbemClassObject>, presponsehandler: ::core::option::Option<IWbemObjectSink>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWbemServicesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWbemServicesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWbemServicesVtbl {
        unsafe extern "system" fn OpenNamespace<Impl: IWbemServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strnamespace: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lflags: i32, pctx: ::windows::core::RawPtr, ppworkingnamespace: *mut ::windows::core::RawPtr, ppresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OpenNamespace(::core::mem::transmute_copy(&strnamespace), ::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&pctx), ::core::mem::transmute_copy(&ppworkingnamespace), ::core::mem::transmute_copy(&ppresult)).into()
        }
        unsafe extern "system" fn CancelAsyncCall<Impl: IWbemServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CancelAsyncCall(::core::mem::transmute(&psink)).into()
        }
        unsafe extern "system" fn QueryObjectSink<Impl: IWbemServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, ppresponsehandler: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryObjectSink(::core::mem::transmute_copy(&lflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppresponsehandler = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetObject<Impl: IWbemServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strobjectpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lflags: i32, pctx: ::windows::core::RawPtr, ppobject: *mut ::windows::core::RawPtr, ppcallresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetObject(::core::mem::transmute_copy(&strobjectpath), ::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&pctx), ::core::mem::transmute_copy(&ppobject), ::core::mem::transmute_copy(&ppcallresult)).into()
        }
        unsafe extern "system" fn GetObjectAsync<Impl: IWbemServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strobjectpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lflags: i32, pctx: ::windows::core::RawPtr, presponsehandler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetObjectAsync(::core::mem::transmute_copy(&strobjectpath), ::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&pctx), ::core::mem::transmute(&presponsehandler)).into()
        }
        unsafe extern "system" fn PutClass<Impl: IWbemServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pobject: ::windows::core::RawPtr, lflags: i32, pctx: ::windows::core::RawPtr, ppcallresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PutClass(::core::mem::transmute(&pobject), ::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&pctx), ::core::mem::transmute_copy(&ppcallresult)).into()
        }
        unsafe extern "system" fn PutClassAsync<Impl: IWbemServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pobject: ::windows::core::RawPtr, lflags: i32, pctx: ::windows::core::RawPtr, presponsehandler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PutClassAsync(::core::mem::transmute(&pobject), ::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&pctx), ::core::mem::transmute(&presponsehandler)).into()
        }
        unsafe extern "system" fn DeleteClass<Impl: IWbemServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lflags: i32, pctx: ::windows::core::RawPtr, ppcallresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteClass(::core::mem::transmute_copy(&strclass), ::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&pctx), ::core::mem::transmute_copy(&ppcallresult)).into()
        }
        unsafe extern "system" fn DeleteClassAsync<Impl: IWbemServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lflags: i32, pctx: ::windows::core::RawPtr, presponsehandler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteClassAsync(::core::mem::transmute_copy(&strclass), ::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&pctx), ::core::mem::transmute(&presponsehandler)).into()
        }
        unsafe extern "system" fn CreateClassEnum<Impl: IWbemServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strsuperclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lflags: i32, pctx: ::windows::core::RawPtr, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateClassEnum(::core::mem::transmute_copy(&strsuperclass), ::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&pctx)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateClassEnumAsync<Impl: IWbemServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strsuperclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lflags: i32, pctx: ::windows::core::RawPtr, presponsehandler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateClassEnumAsync(::core::mem::transmute_copy(&strsuperclass), ::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&pctx), ::core::mem::transmute(&presponsehandler)).into()
        }
        unsafe extern "system" fn PutInstance<Impl: IWbemServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinst: ::windows::core::RawPtr, lflags: i32, pctx: ::windows::core::RawPtr, ppcallresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PutInstance(::core::mem::transmute(&pinst), ::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&pctx), ::core::mem::transmute_copy(&ppcallresult)).into()
        }
        unsafe extern "system" fn PutInstanceAsync<Impl: IWbemServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinst: ::windows::core::RawPtr, lflags: i32, pctx: ::windows::core::RawPtr, presponsehandler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PutInstanceAsync(::core::mem::transmute(&pinst), ::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&pctx), ::core::mem::transmute(&presponsehandler)).into()
        }
        unsafe extern "system" fn DeleteInstance<Impl: IWbemServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strobjectpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lflags: i32, pctx: ::windows::core::RawPtr, ppcallresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteInstance(::core::mem::transmute_copy(&strobjectpath), ::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&pctx), ::core::mem::transmute_copy(&ppcallresult)).into()
        }
        unsafe extern "system" fn DeleteInstanceAsync<Impl: IWbemServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strobjectpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lflags: i32, pctx: ::windows::core::RawPtr, presponsehandler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteInstanceAsync(::core::mem::transmute_copy(&strobjectpath), ::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&pctx), ::core::mem::transmute(&presponsehandler)).into()
        }
        unsafe extern "system" fn CreateInstanceEnum<Impl: IWbemServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strfilter: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lflags: i32, pctx: ::windows::core::RawPtr, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstanceEnum(::core::mem::transmute_copy(&strfilter), ::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&pctx)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateInstanceEnumAsync<Impl: IWbemServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strfilter: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lflags: i32, pctx: ::windows::core::RawPtr, presponsehandler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateInstanceEnumAsync(::core::mem::transmute_copy(&strfilter), ::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&pctx), ::core::mem::transmute(&presponsehandler)).into()
        }
        unsafe extern "system" fn ExecQuery<Impl: IWbemServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strquerylanguage: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strquery: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lflags: i32, pctx: ::windows::core::RawPtr, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExecQuery(::core::mem::transmute_copy(&strquerylanguage), ::core::mem::transmute_copy(&strquery), ::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&pctx)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExecQueryAsync<Impl: IWbemServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strquerylanguage: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strquery: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lflags: i32, pctx: ::windows::core::RawPtr, presponsehandler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ExecQueryAsync(::core::mem::transmute_copy(&strquerylanguage), ::core::mem::transmute_copy(&strquery), ::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&pctx), ::core::mem::transmute(&presponsehandler)).into()
        }
        unsafe extern "system" fn ExecNotificationQuery<Impl: IWbemServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strquerylanguage: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strquery: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lflags: i32, pctx: ::windows::core::RawPtr, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExecNotificationQuery(::core::mem::transmute_copy(&strquerylanguage), ::core::mem::transmute_copy(&strquery), ::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&pctx)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExecNotificationQueryAsync<Impl: IWbemServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strquerylanguage: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strquery: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lflags: i32, pctx: ::windows::core::RawPtr, presponsehandler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ExecNotificationQueryAsync(::core::mem::transmute_copy(&strquerylanguage), ::core::mem::transmute_copy(&strquery), ::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&pctx), ::core::mem::transmute(&presponsehandler)).into()
        }
        unsafe extern "system" fn ExecMethod<Impl: IWbemServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strobjectpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strmethodname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lflags: i32, pctx: ::windows::core::RawPtr, pinparams: ::windows::core::RawPtr, ppoutparams: *mut ::windows::core::RawPtr, ppcallresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ExecMethod(::core::mem::transmute_copy(&strobjectpath), ::core::mem::transmute_copy(&strmethodname), ::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&pctx), ::core::mem::transmute(&pinparams), ::core::mem::transmute_copy(&ppoutparams), ::core::mem::transmute_copy(&ppcallresult)).into()
        }
        unsafe extern "system" fn ExecMethodAsync<Impl: IWbemServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strobjectpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strmethodname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lflags: i32, pctx: ::windows::core::RawPtr, pinparams: ::windows::core::RawPtr, presponsehandler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ExecMethodAsync(::core::mem::transmute_copy(&strobjectpath), ::core::mem::transmute_copy(&strmethodname), ::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&pctx), ::core::mem::transmute(&pinparams), ::core::mem::transmute(&presponsehandler)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            OpenNamespace: OpenNamespace::<Impl, IMPL_OFFSET>,
            CancelAsyncCall: CancelAsyncCall::<Impl, IMPL_OFFSET>,
            QueryObjectSink: QueryObjectSink::<Impl, IMPL_OFFSET>,
            GetObject: GetObject::<Impl, IMPL_OFFSET>,
            GetObjectAsync: GetObjectAsync::<Impl, IMPL_OFFSET>,
            PutClass: PutClass::<Impl, IMPL_OFFSET>,
            PutClassAsync: PutClassAsync::<Impl, IMPL_OFFSET>,
            DeleteClass: DeleteClass::<Impl, IMPL_OFFSET>,
            DeleteClassAsync: DeleteClassAsync::<Impl, IMPL_OFFSET>,
            CreateClassEnum: CreateClassEnum::<Impl, IMPL_OFFSET>,
            CreateClassEnumAsync: CreateClassEnumAsync::<Impl, IMPL_OFFSET>,
            PutInstance: PutInstance::<Impl, IMPL_OFFSET>,
            PutInstanceAsync: PutInstanceAsync::<Impl, IMPL_OFFSET>,
            DeleteInstance: DeleteInstance::<Impl, IMPL_OFFSET>,
            DeleteInstanceAsync: DeleteInstanceAsync::<Impl, IMPL_OFFSET>,
            CreateInstanceEnum: CreateInstanceEnum::<Impl, IMPL_OFFSET>,
            CreateInstanceEnumAsync: CreateInstanceEnumAsync::<Impl, IMPL_OFFSET>,
            ExecQuery: ExecQuery::<Impl, IMPL_OFFSET>,
            ExecQueryAsync: ExecQueryAsync::<Impl, IMPL_OFFSET>,
            ExecNotificationQuery: ExecNotificationQuery::<Impl, IMPL_OFFSET>,
            ExecNotificationQueryAsync: ExecNotificationQueryAsync::<Impl, IMPL_OFFSET>,
            ExecMethod: ExecMethod::<Impl, IMPL_OFFSET>,
            ExecMethodAsync: ExecMethodAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWbemServices as ::windows::core::Interface>::IID
    }
}
pub trait IWbemShutdownImpl: Sized {
    fn Shutdown(&mut self, ureason: i32, umaxmilliseconds: u32, pctx: ::core::option::Option<IWbemContext>) -> ::windows::core::Result<()>;
}
impl IWbemShutdownVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWbemShutdownImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWbemShutdownVtbl {
        unsafe extern "system" fn Shutdown<Impl: IWbemShutdownImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ureason: i32, umaxmilliseconds: u32, pctx: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Shutdown(::core::mem::transmute_copy(&ureason), ::core::mem::transmute_copy(&umaxmilliseconds), ::core::mem::transmute(&pctx)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), Shutdown: Shutdown::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWbemShutdown as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWbemStatusCodeTextImpl: Sized {
    fn GetErrorCodeText(&mut self, hres: ::windows::core::HRESULT, localeid: u32, lflags: i32) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetFacilityCodeText(&mut self, hres: ::windows::core::HRESULT, localeid: u32, lflags: i32) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWbemStatusCodeTextVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWbemStatusCodeTextImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWbemStatusCodeTextVtbl {
        unsafe extern "system" fn GetErrorCodeText<Impl: IWbemStatusCodeTextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hres: ::windows::core::HRESULT, localeid: u32, lflags: i32, messagetext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetErrorCodeText(::core::mem::transmute_copy(&hres), ::core::mem::transmute_copy(&localeid), ::core::mem::transmute_copy(&lflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *messagetext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFacilityCodeText<Impl: IWbemStatusCodeTextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hres: ::windows::core::HRESULT, localeid: u32, lflags: i32, messagetext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFacilityCodeText(::core::mem::transmute_copy(&hres), ::core::mem::transmute_copy(&localeid), ::core::mem::transmute_copy(&lflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *messagetext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetErrorCodeText: GetErrorCodeText::<Impl, IMPL_OFFSET>,
            GetFacilityCodeText: GetFacilityCodeText::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWbemStatusCodeText as ::windows::core::Interface>::IID
    }
}
pub trait IWbemTransportImpl: Sized {
    fn Initialize(&mut self) -> ::windows::core::Result<()>;
}
impl IWbemTransportVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWbemTransportImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWbemTransportVtbl {
        unsafe extern "system" fn Initialize<Impl: IWbemTransportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Initialize().into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), Initialize: Initialize::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWbemTransport as ::windows::core::Interface>::IID
    }
}
pub trait IWbemUnboundObjectSinkImpl: Sized {
    fn IndicateToConsumer(&mut self, plogicalconsumer: ::core::option::Option<IWbemClassObject>, lnumobjects: i32, apobjects: *const ::core::option::Option<IWbemClassObject>) -> ::windows::core::Result<()>;
}
impl IWbemUnboundObjectSinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWbemUnboundObjectSinkImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWbemUnboundObjectSinkVtbl {
        unsafe extern "system" fn IndicateToConsumer<Impl: IWbemUnboundObjectSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plogicalconsumer: ::windows::core::RawPtr, lnumobjects: i32, apobjects: *const ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IndicateToConsumer(::core::mem::transmute(&plogicalconsumer), ::core::mem::transmute_copy(&lnumobjects), ::core::mem::transmute_copy(&apobjects)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), IndicateToConsumer: IndicateToConsumer::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWbemUnboundObjectSink as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWbemUnsecuredApartmentImpl: Sized + IUnsecuredApartmentImpl {
    fn CreateSinkStub(&mut self, psink: ::core::option::Option<IWbemObjectSink>, dwflags: u32, wszreserved: super::super::Foundation::PWSTR) -> ::windows::core::Result<IWbemObjectSink>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWbemUnsecuredApartmentVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWbemUnsecuredApartmentImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWbemUnsecuredApartmentVtbl {
        unsafe extern "system" fn CreateSinkStub<Impl: IWbemUnsecuredApartmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psink: ::windows::core::RawPtr, dwflags: u32, wszreserved: super::super::Foundation::PWSTR, ppstub: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateSinkStub(::core::mem::transmute(&psink), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&wszreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppstub = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: IUnsecuredApartmentVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), CreateSinkStub: CreateSinkStub::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWbemUnsecuredApartment as ::windows::core::Interface>::IID
    }
}
