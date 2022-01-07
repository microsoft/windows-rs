pub trait IEnumWbemClassObjectImpl: Sized {
    fn Reset();
    fn Next();
    fn NextAsync();
    fn Clone();
    fn Skip();
}
impl ::windows::core::RuntimeName for IEnumWbemClassObject {
    const NAME: &'static str = "Windows.Win32.System.Wmi.IEnumWbemClassObject";
}
impl IEnumWbemClassObjectVtbl {
    pub const fn new<Impl: IEnumWbemClassObjectImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IEnumWbemClassObjectVtbl {
        unsafe extern "system" fn Reset<Impl: IEnumWbemClassObjectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Reset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Impl: IEnumWbemClassObjectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ltimeout: i32, ucount: u32, apobjects: *mut ::windows::core::RawPtr, pureturned: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Next(ltimeout, ucount, ::core::mem::transmute_copy(&apobjects), ::core::mem::transmute_copy(&pureturned)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NextAsync<Impl: IEnumWbemClassObjectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ucount: u32, psink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NextAsync(ucount, &*(&psink as *const <IWbemObjectSink as ::windows::core::Abi>::Abi as *const <IWbemObjectSink as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IEnumWbemClassObjectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Skip<Impl: IEnumWbemClassObjectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ltimeout: i32, ncount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Skip(ltimeout, ncount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IEnumWbemClassObject>, base.5, Reset::<Impl, OFFSET>, Next::<Impl, OFFSET>, NextAsync::<Impl, OFFSET>, Clone::<Impl, OFFSET>, Skip::<Impl, OFFSET>)
    }
}
pub trait IMofCompilerImpl: Sized {
    fn CompileFile();
    fn CompileBuffer();
    fn CreateBMOF();
}
impl ::windows::core::RuntimeName for IMofCompiler {
    const NAME: &'static str = "Windows.Win32.System.Wmi.IMofCompiler";
}
impl IMofCompilerVtbl {
    pub const fn new<Impl: IMofCompilerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMofCompilerVtbl {
        unsafe extern "system" fn CompileFile<Impl: IMofCompilerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, filename: super::super::Foundation::PWSTR, serverandnamespace: super::super::Foundation::PWSTR, user: super::super::Foundation::PWSTR, authority: super::super::Foundation::PWSTR, password: super::super::Foundation::PWSTR, loptionflags: i32, lclassflags: i32, linstanceflags: i32, pinfo: *mut WBEM_COMPILE_STATUS_INFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CompileFile(
                &*(&filename as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&serverandnamespace as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&user as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&authority as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&password as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                loptionflags,
                lclassflags,
                linstanceflags,
                &*(&pinfo as *const <WBEM_COMPILE_STATUS_INFO as ::windows::core::Abi>::Abi as *const <WBEM_COMPILE_STATUS_INFO as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CompileBuffer<Impl: IMofCompilerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, buffsize: i32, pbuffer: *const u8, serverandnamespace: super::super::Foundation::PWSTR, user: super::super::Foundation::PWSTR, authority: super::super::Foundation::PWSTR, password: super::super::Foundation::PWSTR, loptionflags: i32, lclassflags: i32, linstanceflags: i32, pinfo: *mut WBEM_COMPILE_STATUS_INFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CompileBuffer(
                buffsize,
                pbuffer,
                &*(&serverandnamespace as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&user as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&authority as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&password as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                loptionflags,
                lclassflags,
                linstanceflags,
                &*(&pinfo as *const <WBEM_COMPILE_STATUS_INFO as ::windows::core::Abi>::Abi as *const <WBEM_COMPILE_STATUS_INFO as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBMOF<Impl: IMofCompilerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, textfilename: super::super::Foundation::PWSTR, bmoffilename: super::super::Foundation::PWSTR, serverandnamespace: super::super::Foundation::PWSTR, loptionflags: i32, lclassflags: i32, linstanceflags: i32, pinfo: *mut WBEM_COMPILE_STATUS_INFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateBMOF(
                &*(&textfilename as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bmoffilename as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&serverandnamespace as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                loptionflags,
                lclassflags,
                linstanceflags,
                &*(&pinfo as *const <WBEM_COMPILE_STATUS_INFO as ::windows::core::Abi>::Abi as *const <WBEM_COMPILE_STATUS_INFO as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMofCompiler>, base.5, CompileFile::<Impl, OFFSET>, CompileBuffer::<Impl, OFFSET>, CreateBMOF::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISWbemDateTimeImpl: Sized + IDispatchImpl {
    fn Value();
    fn SetValue();
    fn Year();
    fn SetYear();
    fn YearSpecified();
    fn SetYearSpecified();
    fn Month();
    fn SetMonth();
    fn MonthSpecified();
    fn SetMonthSpecified();
    fn Day();
    fn SetDay();
    fn DaySpecified();
    fn SetDaySpecified();
    fn Hours();
    fn SetHours();
    fn HoursSpecified();
    fn SetHoursSpecified();
    fn Minutes();
    fn SetMinutes();
    fn MinutesSpecified();
    fn SetMinutesSpecified();
    fn Seconds();
    fn SetSeconds();
    fn SecondsSpecified();
    fn SetSecondsSpecified();
    fn Microseconds();
    fn SetMicroseconds();
    fn MicrosecondsSpecified();
    fn SetMicrosecondsSpecified();
    fn UTC();
    fn SetUTC();
    fn UTCSpecified();
    fn SetUTCSpecified();
    fn IsInterval();
    fn SetIsInterval();
    fn GetVarDate();
    fn SetVarDate();
    fn GetFileTime();
    fn SetFileTime();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISWbemDateTime {
    const NAME: &'static str = "Windows.Win32.System.Wmi.ISWbemDateTime";
}
#[cfg(feature = "Win32_System_Com")]
impl ISWbemDateTimeVtbl {
    pub const fn new<Impl: ISWbemDateTimeImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISWbemDateTimeVtbl {
        unsafe extern "system" fn Value<Impl: ISWbemDateTimeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strvalue: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Value(::core::mem::transmute_copy(&strvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValue<Impl: ISWbemDateTimeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strvalue: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetValue(&*(&strvalue as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Year<Impl: ISWbemDateTimeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iyear: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Year(::core::mem::transmute_copy(&iyear)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetYear<Impl: ISWbemDateTimeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iyear: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetYear(iyear) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn YearSpecified<Impl: ISWbemDateTimeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, byearspecified: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).YearSpecified(::core::mem::transmute_copy(&byearspecified)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetYearSpecified<Impl: ISWbemDateTimeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, byearspecified: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetYearSpecified(byearspecified) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Month<Impl: ISWbemDateTimeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, imonth: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Month(::core::mem::transmute_copy(&imonth)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMonth<Impl: ISWbemDateTimeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, imonth: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetMonth(imonth) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MonthSpecified<Impl: ISWbemDateTimeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bmonthspecified: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MonthSpecified(::core::mem::transmute_copy(&bmonthspecified)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMonthSpecified<Impl: ISWbemDateTimeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bmonthspecified: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetMonthSpecified(bmonthspecified) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Day<Impl: ISWbemDateTimeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iday: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Day(::core::mem::transmute_copy(&iday)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDay<Impl: ISWbemDateTimeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iday: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetDay(iday) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DaySpecified<Impl: ISWbemDateTimeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bdayspecified: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DaySpecified(::core::mem::transmute_copy(&bdayspecified)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDaySpecified<Impl: ISWbemDateTimeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bdayspecified: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetDaySpecified(bdayspecified) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Hours<Impl: ISWbemDateTimeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ihours: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Hours(::core::mem::transmute_copy(&ihours)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHours<Impl: ISWbemDateTimeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ihours: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetHours(ihours) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HoursSpecified<Impl: ISWbemDateTimeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bhoursspecified: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HoursSpecified(::core::mem::transmute_copy(&bhoursspecified)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHoursSpecified<Impl: ISWbemDateTimeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bhoursspecified: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetHoursSpecified(bhoursspecified) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Minutes<Impl: ISWbemDateTimeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iminutes: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Minutes(::core::mem::transmute_copy(&iminutes)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMinutes<Impl: ISWbemDateTimeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iminutes: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetMinutes(iminutes) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinutesSpecified<Impl: ISWbemDateTimeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bminutesspecified: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MinutesSpecified(::core::mem::transmute_copy(&bminutesspecified)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMinutesSpecified<Impl: ISWbemDateTimeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bminutesspecified: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetMinutesSpecified(bminutesspecified) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Seconds<Impl: ISWbemDateTimeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iseconds: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Seconds(::core::mem::transmute_copy(&iseconds)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSeconds<Impl: ISWbemDateTimeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iseconds: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetSeconds(iseconds) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SecondsSpecified<Impl: ISWbemDateTimeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bsecondsspecified: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SecondsSpecified(::core::mem::transmute_copy(&bsecondsspecified)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSecondsSpecified<Impl: ISWbemDateTimeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bsecondsspecified: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetSecondsSpecified(bsecondsspecified) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Microseconds<Impl: ISWbemDateTimeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, imicroseconds: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Microseconds(::core::mem::transmute_copy(&imicroseconds)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMicroseconds<Impl: ISWbemDateTimeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, imicroseconds: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetMicroseconds(imicroseconds) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MicrosecondsSpecified<Impl: ISWbemDateTimeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bmicrosecondsspecified: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MicrosecondsSpecified(::core::mem::transmute_copy(&bmicrosecondsspecified)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMicrosecondsSpecified<Impl: ISWbemDateTimeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bmicrosecondsspecified: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetMicrosecondsSpecified(bmicrosecondsspecified) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UTC<Impl: ISWbemDateTimeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iutc: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UTC(::core::mem::transmute_copy(&iutc)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUTC<Impl: ISWbemDateTimeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iutc: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetUTC(iutc) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UTCSpecified<Impl: ISWbemDateTimeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, butcspecified: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UTCSpecified(::core::mem::transmute_copy(&butcspecified)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUTCSpecified<Impl: ISWbemDateTimeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, butcspecified: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetUTCSpecified(butcspecified) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsInterval<Impl: ISWbemDateTimeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bisinterval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsInterval(::core::mem::transmute_copy(&bisinterval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsInterval<Impl: ISWbemDateTimeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bisinterval: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetIsInterval(bisinterval) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVarDate<Impl: ISWbemDateTimeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bislocal: i16, dvardate: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetVarDate(bislocal, ::core::mem::transmute_copy(&dvardate)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVarDate<Impl: ISWbemDateTimeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dvardate: f64, bislocal: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetVarDate(dvardate, bislocal) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFileTime<Impl: ISWbemDateTimeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bislocal: i16, strfiletime: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetFileTime(bislocal, ::core::mem::transmute_copy(&strfiletime)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFileTime<Impl: ISWbemDateTimeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strfiletime: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bislocal: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetFileTime(&*(&strfiletime as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), bislocal) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<ISWbemDateTime>,
            base.5,
            Value::<Impl, OFFSET>,
            SetValue::<Impl, OFFSET>,
            Year::<Impl, OFFSET>,
            SetYear::<Impl, OFFSET>,
            YearSpecified::<Impl, OFFSET>,
            SetYearSpecified::<Impl, OFFSET>,
            Month::<Impl, OFFSET>,
            SetMonth::<Impl, OFFSET>,
            MonthSpecified::<Impl, OFFSET>,
            SetMonthSpecified::<Impl, OFFSET>,
            Day::<Impl, OFFSET>,
            SetDay::<Impl, OFFSET>,
            DaySpecified::<Impl, OFFSET>,
            SetDaySpecified::<Impl, OFFSET>,
            Hours::<Impl, OFFSET>,
            SetHours::<Impl, OFFSET>,
            HoursSpecified::<Impl, OFFSET>,
            SetHoursSpecified::<Impl, OFFSET>,
            Minutes::<Impl, OFFSET>,
            SetMinutes::<Impl, OFFSET>,
            MinutesSpecified::<Impl, OFFSET>,
            SetMinutesSpecified::<Impl, OFFSET>,
            Seconds::<Impl, OFFSET>,
            SetSeconds::<Impl, OFFSET>,
            SecondsSpecified::<Impl, OFFSET>,
            SetSecondsSpecified::<Impl, OFFSET>,
            Microseconds::<Impl, OFFSET>,
            SetMicroseconds::<Impl, OFFSET>,
            MicrosecondsSpecified::<Impl, OFFSET>,
            SetMicrosecondsSpecified::<Impl, OFFSET>,
            UTC::<Impl, OFFSET>,
            SetUTC::<Impl, OFFSET>,
            UTCSpecified::<Impl, OFFSET>,
            SetUTCSpecified::<Impl, OFFSET>,
            IsInterval::<Impl, OFFSET>,
            SetIsInterval::<Impl, OFFSET>,
            GetVarDate::<Impl, OFFSET>,
            SetVarDate::<Impl, OFFSET>,
            GetFileTime::<Impl, OFFSET>,
            SetFileTime::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISWbemEventSourceImpl: Sized + IDispatchImpl {
    fn NextEvent();
    fn Security_();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISWbemEventSource {
    const NAME: &'static str = "Windows.Win32.System.Wmi.ISWbemEventSource";
}
#[cfg(feature = "Win32_System_Com")]
impl ISWbemEventSourceVtbl {
    pub const fn new<Impl: ISWbemEventSourceImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISWbemEventSourceVtbl {
        unsafe extern "system" fn NextEvent<Impl: ISWbemEventSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, itimeoutms: i32, objwbemobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NextEvent(itimeoutms, ::core::mem::transmute_copy(&objwbemobject)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Security_<Impl: ISWbemEventSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, objwbemsecurity: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Security_(::core::mem::transmute_copy(&objwbemsecurity)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISWbemEventSource>, base.5, NextEvent::<Impl, OFFSET>, Security_::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISWbemLastErrorImpl: Sized + ISWbemObjectImpl + IDispatchImpl {}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISWbemLastError {
    const NAME: &'static str = "Windows.Win32.System.Wmi.ISWbemLastError";
}
#[cfg(feature = "Win32_System_Com")]
impl ISWbemLastErrorVtbl {
    pub const fn new<Impl: ISWbemLastErrorImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISWbemLastErrorVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISWbemLastError>, base.5)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISWbemLocatorImpl: Sized + IDispatchImpl {
    fn ConnectServer();
    fn Security_();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISWbemLocator {
    const NAME: &'static str = "Windows.Win32.System.Wmi.ISWbemLocator";
}
#[cfg(feature = "Win32_System_Com")]
impl ISWbemLocatorVtbl {
    pub const fn new<Impl: ISWbemLocatorImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISWbemLocatorVtbl {
        unsafe extern "system" fn ConnectServer<Impl: ISWbemLocatorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strserver: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strnamespace: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, struser: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strlocale: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strauthority: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, isecurityflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemservices: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ConnectServer(
                &*(&strserver as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&strnamespace as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&struser as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&strpassword as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&strlocale as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&strauthority as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                isecurityflags,
                &*(&objwbemnamedvalueset as *const <super::Com::IDispatch as ::windows::core::Abi>::Abi as *const <super::Com::IDispatch as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&objwbemservices),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Security_<Impl: ISWbemLocatorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, objwbemsecurity: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Security_(::core::mem::transmute_copy(&objwbemsecurity)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISWbemLocator>, base.5, ConnectServer::<Impl, OFFSET>, Security_::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISWbemMethodImpl: Sized + IDispatchImpl {
    fn Name();
    fn Origin();
    fn InParameters();
    fn OutParameters();
    fn Qualifiers_();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISWbemMethod {
    const NAME: &'static str = "Windows.Win32.System.Wmi.ISWbemMethod";
}
#[cfg(feature = "Win32_System_Com")]
impl ISWbemMethodVtbl {
    pub const fn new<Impl: ISWbemMethodImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISWbemMethodVtbl {
        unsafe extern "system" fn Name<Impl: ISWbemMethodImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Name(::core::mem::transmute_copy(&strname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Origin<Impl: ISWbemMethodImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strorigin: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Origin(::core::mem::transmute_copy(&strorigin)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InParameters<Impl: ISWbemMethodImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, objwbeminparameters: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InParameters(::core::mem::transmute_copy(&objwbeminparameters)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OutParameters<Impl: ISWbemMethodImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, objwbemoutparameters: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OutParameters(::core::mem::transmute_copy(&objwbemoutparameters)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Qualifiers_<Impl: ISWbemMethodImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, objwbemqualifierset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Qualifiers_(::core::mem::transmute_copy(&objwbemqualifierset)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISWbemMethod>, base.5, Name::<Impl, OFFSET>, Origin::<Impl, OFFSET>, InParameters::<Impl, OFFSET>, OutParameters::<Impl, OFFSET>, Qualifiers_::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISWbemMethodSetImpl: Sized + IDispatchImpl {
    fn _NewEnum();
    fn Item();
    fn Count();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISWbemMethodSet {
    const NAME: &'static str = "Windows.Win32.System.Wmi.ISWbemMethodSet";
}
#[cfg(feature = "Win32_System_Com")]
impl ISWbemMethodSetVtbl {
    pub const fn new<Impl: ISWbemMethodSetImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISWbemMethodSetVtbl {
        unsafe extern "system" fn _NewEnum<Impl: ISWbemMethodSetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this)._NewEnum(::core::mem::transmute_copy(&punk)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: ISWbemMethodSetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemmethod: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Item(&*(&strname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), iflags, ::core::mem::transmute_copy(&objwbemmethod)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: ISWbemMethodSetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, icount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Count(::core::mem::transmute_copy(&icount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISWbemMethodSet>, base.5, _NewEnum::<Impl, OFFSET>, Item::<Impl, OFFSET>, Count::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISWbemNamedValueImpl: Sized + IDispatchImpl {
    fn Value();
    fn SetValue();
    fn Name();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISWbemNamedValue {
    const NAME: &'static str = "Windows.Win32.System.Wmi.ISWbemNamedValue";
}
#[cfg(feature = "Win32_System_Com")]
impl ISWbemNamedValueVtbl {
    pub const fn new<Impl: ISWbemNamedValueImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISWbemNamedValueVtbl {
        unsafe extern "system" fn Value<Impl: ISWbemNamedValueImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varvalue: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Value(::core::mem::transmute_copy(&varvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValue<Impl: ISWbemNamedValueImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varvalue: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetValue(&*(&varvalue as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Impl: ISWbemNamedValueImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Name(::core::mem::transmute_copy(&strname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISWbemNamedValue>, base.5, Value::<Impl, OFFSET>, SetValue::<Impl, OFFSET>, Name::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISWbemNamedValueSetImpl: Sized + IDispatchImpl {
    fn _NewEnum();
    fn Item();
    fn Count();
    fn Add();
    fn Remove();
    fn Clone();
    fn DeleteAll();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISWbemNamedValueSet {
    const NAME: &'static str = "Windows.Win32.System.Wmi.ISWbemNamedValueSet";
}
#[cfg(feature = "Win32_System_Com")]
impl ISWbemNamedValueSetVtbl {
    pub const fn new<Impl: ISWbemNamedValueSetImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISWbemNamedValueSetVtbl {
        unsafe extern "system" fn _NewEnum<Impl: ISWbemNamedValueSetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this)._NewEnum(::core::mem::transmute_copy(&punk)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: ISWbemNamedValueSetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemnamedvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Item(&*(&strname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), iflags, ::core::mem::transmute_copy(&objwbemnamedvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: ISWbemNamedValueSetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, icount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Count(::core::mem::transmute_copy(&icount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Impl: ISWbemNamedValueSetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varvalue: *const super::Com::VARIANT, iflags: i32, objwbemnamedvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Add(&*(&strname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&varvalue as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), iflags, ::core::mem::transmute_copy(&objwbemnamedvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Remove<Impl: ISWbemNamedValueSetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Remove(&*(&strname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), iflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: ISWbemNamedValueSetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, objwbemnamedvalueset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&objwbemnamedvalueset)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteAll<Impl: ISWbemNamedValueSetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DeleteAll() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISWbemNamedValueSet>, base.5, _NewEnum::<Impl, OFFSET>, Item::<Impl, OFFSET>, Count::<Impl, OFFSET>, Add::<Impl, OFFSET>, Remove::<Impl, OFFSET>, Clone::<Impl, OFFSET>, DeleteAll::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISWbemObjectImpl: Sized + IDispatchImpl {
    fn Put_();
    fn PutAsync_();
    fn Delete_();
    fn DeleteAsync_();
    fn Instances_();
    fn InstancesAsync_();
    fn Subclasses_();
    fn SubclassesAsync_();
    fn Associators_();
    fn AssociatorsAsync_();
    fn References_();
    fn ReferencesAsync_();
    fn ExecMethod_();
    fn ExecMethodAsync_();
    fn Clone_();
    fn GetObjectText_();
    fn SpawnDerivedClass_();
    fn SpawnInstance_();
    fn CompareTo_();
    fn Qualifiers_();
    fn Properties_();
    fn Methods_();
    fn Derivation_();
    fn Path_();
    fn Security_();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISWbemObject {
    const NAME: &'static str = "Windows.Win32.System.Wmi.ISWbemObject";
}
#[cfg(feature = "Win32_System_Com")]
impl ISWbemObjectVtbl {
    pub const fn new<Impl: ISWbemObjectImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISWbemObjectVtbl {
        unsafe extern "system" fn Put_<Impl: ISWbemObjectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemobjectpath: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Put_(iflags, &*(&objwbemnamedvalueset as *const <super::Com::IDispatch as ::windows::core::Abi>::Abi as *const <super::Com::IDispatch as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&objwbemobjectpath)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PutAsync_<Impl: ISWbemObjectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, objwbemsink: ::windows::core::RawPtr, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemasynccontext: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PutAsync_(
                &*(&objwbemsink as *const <super::Com::IDispatch as ::windows::core::Abi>::Abi as *const <super::Com::IDispatch as ::windows::core::DefaultType>::DefaultType),
                iflags,
                &*(&objwbemnamedvalueset as *const <super::Com::IDispatch as ::windows::core::Abi>::Abi as *const <super::Com::IDispatch as ::windows::core::DefaultType>::DefaultType),
                &*(&objwbemasynccontext as *const <super::Com::IDispatch as ::windows::core::Abi>::Abi as *const <super::Com::IDispatch as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete_<Impl: ISWbemObjectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Delete_(iflags, &*(&objwbemnamedvalueset as *const <super::Com::IDispatch as ::windows::core::Abi>::Abi as *const <super::Com::IDispatch as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteAsync_<Impl: ISWbemObjectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, objwbemsink: ::windows::core::RawPtr, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemasynccontext: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DeleteAsync_(
                &*(&objwbemsink as *const <super::Com::IDispatch as ::windows::core::Abi>::Abi as *const <super::Com::IDispatch as ::windows::core::DefaultType>::DefaultType),
                iflags,
                &*(&objwbemnamedvalueset as *const <super::Com::IDispatch as ::windows::core::Abi>::Abi as *const <super::Com::IDispatch as ::windows::core::DefaultType>::DefaultType),
                &*(&objwbemasynccontext as *const <super::Com::IDispatch as ::windows::core::Abi>::Abi as *const <super::Com::IDispatch as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Instances_<Impl: ISWbemObjectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemobjectset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Instances_(iflags, &*(&objwbemnamedvalueset as *const <super::Com::IDispatch as ::windows::core::Abi>::Abi as *const <super::Com::IDispatch as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&objwbemobjectset)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InstancesAsync_<Impl: ISWbemObjectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, objwbemsink: ::windows::core::RawPtr, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemasynccontext: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InstancesAsync_(
                &*(&objwbemsink as *const <super::Com::IDispatch as ::windows::core::Abi>::Abi as *const <super::Com::IDispatch as ::windows::core::DefaultType>::DefaultType),
                iflags,
                &*(&objwbemnamedvalueset as *const <super::Com::IDispatch as ::windows::core::Abi>::Abi as *const <super::Com::IDispatch as ::windows::core::DefaultType>::DefaultType),
                &*(&objwbemasynccontext as *const <super::Com::IDispatch as ::windows::core::Abi>::Abi as *const <super::Com::IDispatch as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Subclasses_<Impl: ISWbemObjectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemobjectset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Subclasses_(iflags, &*(&objwbemnamedvalueset as *const <super::Com::IDispatch as ::windows::core::Abi>::Abi as *const <super::Com::IDispatch as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&objwbemobjectset)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SubclassesAsync_<Impl: ISWbemObjectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, objwbemsink: ::windows::core::RawPtr, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemasynccontext: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SubclassesAsync_(
                &*(&objwbemsink as *const <super::Com::IDispatch as ::windows::core::Abi>::Abi as *const <super::Com::IDispatch as ::windows::core::DefaultType>::DefaultType),
                iflags,
                &*(&objwbemnamedvalueset as *const <super::Com::IDispatch as ::windows::core::Abi>::Abi as *const <super::Com::IDispatch as ::windows::core::DefaultType>::DefaultType),
                &*(&objwbemasynccontext as *const <super::Com::IDispatch as ::windows::core::Abi>::Abi as *const <super::Com::IDispatch as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Associators_<Impl: ISWbemObjectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strassocclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strresultclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strresultrole: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strrole: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bclassesonly: i16, bschemaonly: i16, strrequiredassocqualifier: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strrequiredqualifier: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemobjectset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Associators_(
                &*(&strassocclass as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&strresultclass as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&strresultrole as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&strrole as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                bclassesonly,
                bschemaonly,
                &*(&strrequiredassocqualifier as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&strrequiredqualifier as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                iflags,
                &*(&objwbemnamedvalueset as *const <super::Com::IDispatch as ::windows::core::Abi>::Abi as *const <super::Com::IDispatch as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&objwbemobjectset),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AssociatorsAsync_<Impl: ISWbemObjectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, objwbemsink: ::windows::core::RawPtr, strassocclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strresultclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strresultrole: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strrole: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bclassesonly: i16, bschemaonly: i16, strrequiredassocqualifier: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strrequiredqualifier: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemasynccontext: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AssociatorsAsync_(
                &*(&objwbemsink as *const <super::Com::IDispatch as ::windows::core::Abi>::Abi as *const <super::Com::IDispatch as ::windows::core::DefaultType>::DefaultType),
                &*(&strassocclass as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&strresultclass as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&strresultrole as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&strrole as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                bclassesonly,
                bschemaonly,
                &*(&strrequiredassocqualifier as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&strrequiredqualifier as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                iflags,
                &*(&objwbemnamedvalueset as *const <super::Com::IDispatch as ::windows::core::Abi>::Abi as *const <super::Com::IDispatch as ::windows::core::DefaultType>::DefaultType),
                &*(&objwbemasynccontext as *const <super::Com::IDispatch as ::windows::core::Abi>::Abi as *const <super::Com::IDispatch as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn References_<Impl: ISWbemObjectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strresultclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strrole: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bclassesonly: i16, bschemaonly: i16, strrequiredqualifier: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemobjectset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).References_(
                &*(&strresultclass as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&strrole as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                bclassesonly,
                bschemaonly,
                &*(&strrequiredqualifier as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                iflags,
                &*(&objwbemnamedvalueset as *const <super::Com::IDispatch as ::windows::core::Abi>::Abi as *const <super::Com::IDispatch as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&objwbemobjectset),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReferencesAsync_<Impl: ISWbemObjectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, objwbemsink: ::windows::core::RawPtr, strresultclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strrole: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bclassesonly: i16, bschemaonly: i16, strrequiredqualifier: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemasynccontext: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ReferencesAsync_(
                &*(&objwbemsink as *const <super::Com::IDispatch as ::windows::core::Abi>::Abi as *const <super::Com::IDispatch as ::windows::core::DefaultType>::DefaultType),
                &*(&strresultclass as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&strrole as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                bclassesonly,
                bschemaonly,
                &*(&strrequiredqualifier as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                iflags,
                &*(&objwbemnamedvalueset as *const <super::Com::IDispatch as ::windows::core::Abi>::Abi as *const <super::Com::IDispatch as ::windows::core::DefaultType>::DefaultType),
                &*(&objwbemasynccontext as *const <super::Com::IDispatch as ::windows::core::Abi>::Abi as *const <super::Com::IDispatch as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExecMethod_<Impl: ISWbemObjectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strmethodname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, objwbeminparameters: ::windows::core::RawPtr, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemoutparameters: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ExecMethod_(
                &*(&strmethodname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&objwbeminparameters as *const <super::Com::IDispatch as ::windows::core::Abi>::Abi as *const <super::Com::IDispatch as ::windows::core::DefaultType>::DefaultType),
                iflags,
                &*(&objwbemnamedvalueset as *const <super::Com::IDispatch as ::windows::core::Abi>::Abi as *const <super::Com::IDispatch as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&objwbemoutparameters),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExecMethodAsync_<Impl: ISWbemObjectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, objwbemsink: ::windows::core::RawPtr, strmethodname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, objwbeminparameters: ::windows::core::RawPtr, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemasynccontext: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ExecMethodAsync_(
                &*(&objwbemsink as *const <super::Com::IDispatch as ::windows::core::Abi>::Abi as *const <super::Com::IDispatch as ::windows::core::DefaultType>::DefaultType),
                &*(&strmethodname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&objwbeminparameters as *const <super::Com::IDispatch as ::windows::core::Abi>::Abi as *const <super::Com::IDispatch as ::windows::core::DefaultType>::DefaultType),
                iflags,
                &*(&objwbemnamedvalueset as *const <super::Com::IDispatch as ::windows::core::Abi>::Abi as *const <super::Com::IDispatch as ::windows::core::DefaultType>::DefaultType),
                &*(&objwbemasynccontext as *const <super::Com::IDispatch as ::windows::core::Abi>::Abi as *const <super::Com::IDispatch as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone_<Impl: ISWbemObjectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, objwbemobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Clone_(::core::mem::transmute_copy(&objwbemobject)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetObjectText_<Impl: ISWbemObjectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iflags: i32, strobjecttext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetObjectText_(iflags, ::core::mem::transmute_copy(&strobjecttext)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SpawnDerivedClass_<Impl: ISWbemObjectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iflags: i32, objwbemobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SpawnDerivedClass_(iflags, ::core::mem::transmute_copy(&objwbemobject)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SpawnInstance_<Impl: ISWbemObjectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iflags: i32, objwbemobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SpawnInstance_(iflags, ::core::mem::transmute_copy(&objwbemobject)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CompareTo_<Impl: ISWbemObjectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, objwbemobject: ::windows::core::RawPtr, iflags: i32, bresult: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CompareTo_(&*(&objwbemobject as *const <super::Com::IDispatch as ::windows::core::Abi>::Abi as *const <super::Com::IDispatch as ::windows::core::DefaultType>::DefaultType), iflags, ::core::mem::transmute_copy(&bresult)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Qualifiers_<Impl: ISWbemObjectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, objwbemqualifierset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Qualifiers_(::core::mem::transmute_copy(&objwbemqualifierset)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties_<Impl: ISWbemObjectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, objwbempropertyset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Properties_(::core::mem::transmute_copy(&objwbempropertyset)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Methods_<Impl: ISWbemObjectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, objwbemmethodset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Methods_(::core::mem::transmute_copy(&objwbemmethodset)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Derivation_<Impl: ISWbemObjectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strclassnamearray: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Derivation_(::core::mem::transmute_copy(&strclassnamearray)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Path_<Impl: ISWbemObjectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, objwbemobjectpath: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Path_(::core::mem::transmute_copy(&objwbemobjectpath)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Security_<Impl: ISWbemObjectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, objwbemsecurity: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Security_(::core::mem::transmute_copy(&objwbemsecurity)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<ISWbemObject>,
            base.5,
            Put_::<Impl, OFFSET>,
            PutAsync_::<Impl, OFFSET>,
            Delete_::<Impl, OFFSET>,
            DeleteAsync_::<Impl, OFFSET>,
            Instances_::<Impl, OFFSET>,
            InstancesAsync_::<Impl, OFFSET>,
            Subclasses_::<Impl, OFFSET>,
            SubclassesAsync_::<Impl, OFFSET>,
            Associators_::<Impl, OFFSET>,
            AssociatorsAsync_::<Impl, OFFSET>,
            References_::<Impl, OFFSET>,
            ReferencesAsync_::<Impl, OFFSET>,
            ExecMethod_::<Impl, OFFSET>,
            ExecMethodAsync_::<Impl, OFFSET>,
            Clone_::<Impl, OFFSET>,
            GetObjectText_::<Impl, OFFSET>,
            SpawnDerivedClass_::<Impl, OFFSET>,
            SpawnInstance_::<Impl, OFFSET>,
            CompareTo_::<Impl, OFFSET>,
            Qualifiers_::<Impl, OFFSET>,
            Properties_::<Impl, OFFSET>,
            Methods_::<Impl, OFFSET>,
            Derivation_::<Impl, OFFSET>,
            Path_::<Impl, OFFSET>,
            Security_::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISWbemObjectExImpl: Sized + ISWbemObjectImpl + IDispatchImpl {
    fn Refresh_();
    fn SystemProperties_();
    fn GetText_();
    fn SetFromText_();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISWbemObjectEx {
    const NAME: &'static str = "Windows.Win32.System.Wmi.ISWbemObjectEx";
}
#[cfg(feature = "Win32_System_Com")]
impl ISWbemObjectExVtbl {
    pub const fn new<Impl: ISWbemObjectExImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISWbemObjectExVtbl {
        unsafe extern "system" fn Refresh_<Impl: ISWbemObjectExImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Refresh_(iflags, &*(&objwbemnamedvalueset as *const <super::Com::IDispatch as ::windows::core::Abi>::Abi as *const <super::Com::IDispatch as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SystemProperties_<Impl: ISWbemObjectExImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, objwbempropertyset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SystemProperties_(::core::mem::transmute_copy(&objwbempropertyset)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetText_<Impl: ISWbemObjectExImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iobjecttextformat: WbemObjectTextFormatEnum, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, bstext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetText_(iobjecttextformat, iflags, &*(&objwbemnamedvalueset as *const <super::Com::IDispatch as ::windows::core::Abi>::Abi as *const <super::Com::IDispatch as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&bstext)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFromText_<Impl: ISWbemObjectExImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstext: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iobjecttextformat: WbemObjectTextFormatEnum, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetFromText_(&*(&bstext as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), iobjecttextformat, iflags, &*(&objwbemnamedvalueset as *const <super::Com::IDispatch as ::windows::core::Abi>::Abi as *const <super::Com::IDispatch as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISWbemObjectEx>, base.5, Refresh_::<Impl, OFFSET>, SystemProperties_::<Impl, OFFSET>, GetText_::<Impl, OFFSET>, SetFromText_::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISWbemObjectPathImpl: Sized + IDispatchImpl {
    fn Path();
    fn SetPath();
    fn RelPath();
    fn SetRelPath();
    fn Server();
    fn SetServer();
    fn Namespace();
    fn SetNamespace();
    fn ParentNamespace();
    fn DisplayName();
    fn SetDisplayName();
    fn Class();
    fn SetClass();
    fn IsClass();
    fn SetAsClass();
    fn IsSingleton();
    fn SetAsSingleton();
    fn Keys();
    fn Security_();
    fn Locale();
    fn SetLocale();
    fn Authority();
    fn SetAuthority();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISWbemObjectPath {
    const NAME: &'static str = "Windows.Win32.System.Wmi.ISWbemObjectPath";
}
#[cfg(feature = "Win32_System_Com")]
impl ISWbemObjectPathVtbl {
    pub const fn new<Impl: ISWbemObjectPathImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISWbemObjectPathVtbl {
        unsafe extern "system" fn Path<Impl: ISWbemObjectPathImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strpath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Path(::core::mem::transmute_copy(&strpath)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPath<Impl: ISWbemObjectPathImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetPath(&*(&strpath as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RelPath<Impl: ISWbemObjectPathImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strrelpath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RelPath(::core::mem::transmute_copy(&strrelpath)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRelPath<Impl: ISWbemObjectPathImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strrelpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetRelPath(&*(&strrelpath as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Server<Impl: ISWbemObjectPathImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strserver: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Server(::core::mem::transmute_copy(&strserver)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetServer<Impl: ISWbemObjectPathImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strserver: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetServer(&*(&strserver as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Namespace<Impl: ISWbemObjectPathImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strnamespace: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Namespace(::core::mem::transmute_copy(&strnamespace)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNamespace<Impl: ISWbemObjectPathImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strnamespace: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetNamespace(&*(&strnamespace as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ParentNamespace<Impl: ISWbemObjectPathImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strparentnamespace: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ParentNamespace(::core::mem::transmute_copy(&strparentnamespace)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplayName<Impl: ISWbemObjectPathImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strdisplayname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DisplayName(::core::mem::transmute_copy(&strdisplayname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisplayName<Impl: ISWbemObjectPathImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strdisplayname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetDisplayName(&*(&strdisplayname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Class<Impl: ISWbemObjectPathImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strclass: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Class(::core::mem::transmute_copy(&strclass)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClass<Impl: ISWbemObjectPathImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetClass(&*(&strclass as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsClass<Impl: ISWbemObjectPathImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bisclass: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsClass(::core::mem::transmute_copy(&bisclass)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAsClass<Impl: ISWbemObjectPathImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetAsClass() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsSingleton<Impl: ISWbemObjectPathImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bissingleton: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsSingleton(::core::mem::transmute_copy(&bissingleton)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAsSingleton<Impl: ISWbemObjectPathImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetAsSingleton() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Keys<Impl: ISWbemObjectPathImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, objwbemnamedvalueset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Keys(::core::mem::transmute_copy(&objwbemnamedvalueset)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Security_<Impl: ISWbemObjectPathImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, objwbemsecurity: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Security_(::core::mem::transmute_copy(&objwbemsecurity)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Locale<Impl: ISWbemObjectPathImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strlocale: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Locale(::core::mem::transmute_copy(&strlocale)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLocale<Impl: ISWbemObjectPathImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strlocale: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetLocale(&*(&strlocale as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Authority<Impl: ISWbemObjectPathImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strauthority: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Authority(::core::mem::transmute_copy(&strauthority)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAuthority<Impl: ISWbemObjectPathImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strauthority: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetAuthority(&*(&strauthority as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<ISWbemObjectPath>,
            base.5,
            Path::<Impl, OFFSET>,
            SetPath::<Impl, OFFSET>,
            RelPath::<Impl, OFFSET>,
            SetRelPath::<Impl, OFFSET>,
            Server::<Impl, OFFSET>,
            SetServer::<Impl, OFFSET>,
            Namespace::<Impl, OFFSET>,
            SetNamespace::<Impl, OFFSET>,
            ParentNamespace::<Impl, OFFSET>,
            DisplayName::<Impl, OFFSET>,
            SetDisplayName::<Impl, OFFSET>,
            Class::<Impl, OFFSET>,
            SetClass::<Impl, OFFSET>,
            IsClass::<Impl, OFFSET>,
            SetAsClass::<Impl, OFFSET>,
            IsSingleton::<Impl, OFFSET>,
            SetAsSingleton::<Impl, OFFSET>,
            Keys::<Impl, OFFSET>,
            Security_::<Impl, OFFSET>,
            Locale::<Impl, OFFSET>,
            SetLocale::<Impl, OFFSET>,
            Authority::<Impl, OFFSET>,
            SetAuthority::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISWbemObjectSetImpl: Sized + IDispatchImpl {
    fn _NewEnum();
    fn Item();
    fn Count();
    fn Security_();
    fn ItemIndex();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISWbemObjectSet {
    const NAME: &'static str = "Windows.Win32.System.Wmi.ISWbemObjectSet";
}
#[cfg(feature = "Win32_System_Com")]
impl ISWbemObjectSetVtbl {
    pub const fn new<Impl: ISWbemObjectSetImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISWbemObjectSetVtbl {
        unsafe extern "system" fn _NewEnum<Impl: ISWbemObjectSetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this)._NewEnum(::core::mem::transmute_copy(&punk)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: ISWbemObjectSetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strobjectpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Item(&*(&strobjectpath as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), iflags, ::core::mem::transmute_copy(&objwbemobject)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: ISWbemObjectSetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, icount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Count(::core::mem::transmute_copy(&icount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Security_<Impl: ISWbemObjectSetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, objwbemsecurity: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Security_(::core::mem::transmute_copy(&objwbemsecurity)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ItemIndex<Impl: ISWbemObjectSetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lindex: i32, objwbemobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ItemIndex(lindex, ::core::mem::transmute_copy(&objwbemobject)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISWbemObjectSet>, base.5, _NewEnum::<Impl, OFFSET>, Item::<Impl, OFFSET>, Count::<Impl, OFFSET>, Security_::<Impl, OFFSET>, ItemIndex::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISWbemPrivilegeImpl: Sized + IDispatchImpl {
    fn IsEnabled();
    fn SetIsEnabled();
    fn Name();
    fn DisplayName();
    fn Identifier();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISWbemPrivilege {
    const NAME: &'static str = "Windows.Win32.System.Wmi.ISWbemPrivilege";
}
#[cfg(feature = "Win32_System_Com")]
impl ISWbemPrivilegeVtbl {
    pub const fn new<Impl: ISWbemPrivilegeImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISWbemPrivilegeVtbl {
        unsafe extern "system" fn IsEnabled<Impl: ISWbemPrivilegeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bisenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsEnabled(::core::mem::transmute_copy(&bisenabled)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsEnabled<Impl: ISWbemPrivilegeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bisenabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetIsEnabled(bisenabled) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Impl: ISWbemPrivilegeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strdisplayname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Name(::core::mem::transmute_copy(&strdisplayname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplayName<Impl: ISWbemPrivilegeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strdisplayname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DisplayName(::core::mem::transmute_copy(&strdisplayname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Identifier<Impl: ISWbemPrivilegeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iprivilege: *mut WbemPrivilegeEnum) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Identifier(::core::mem::transmute_copy(&iprivilege)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISWbemPrivilege>, base.5, IsEnabled::<Impl, OFFSET>, SetIsEnabled::<Impl, OFFSET>, Name::<Impl, OFFSET>, DisplayName::<Impl, OFFSET>, Identifier::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISWbemPrivilegeSetImpl: Sized + IDispatchImpl {
    fn _NewEnum();
    fn Item();
    fn Count();
    fn Add();
    fn Remove();
    fn DeleteAll();
    fn AddAsString();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISWbemPrivilegeSet {
    const NAME: &'static str = "Windows.Win32.System.Wmi.ISWbemPrivilegeSet";
}
#[cfg(feature = "Win32_System_Com")]
impl ISWbemPrivilegeSetVtbl {
    pub const fn new<Impl: ISWbemPrivilegeSetImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISWbemPrivilegeSetVtbl {
        unsafe extern "system" fn _NewEnum<Impl: ISWbemPrivilegeSetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this)._NewEnum(::core::mem::transmute_copy(&punk)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: ISWbemPrivilegeSetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iprivilege: WbemPrivilegeEnum, objwbemprivilege: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Item(iprivilege, ::core::mem::transmute_copy(&objwbemprivilege)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: ISWbemPrivilegeSetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, icount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Count(::core::mem::transmute_copy(&icount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Impl: ISWbemPrivilegeSetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iprivilege: WbemPrivilegeEnum, bisenabled: i16, objwbemprivilege: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Add(iprivilege, bisenabled, ::core::mem::transmute_copy(&objwbemprivilege)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Remove<Impl: ISWbemPrivilegeSetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iprivilege: WbemPrivilegeEnum) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Remove(iprivilege) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteAll<Impl: ISWbemPrivilegeSetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DeleteAll() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddAsString<Impl: ISWbemPrivilegeSetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strprivilege: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bisenabled: i16, objwbemprivilege: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddAsString(&*(&strprivilege as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), bisenabled, ::core::mem::transmute_copy(&objwbemprivilege)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISWbemPrivilegeSet>, base.5, _NewEnum::<Impl, OFFSET>, Item::<Impl, OFFSET>, Count::<Impl, OFFSET>, Add::<Impl, OFFSET>, Remove::<Impl, OFFSET>, DeleteAll::<Impl, OFFSET>, AddAsString::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISWbemPropertyImpl: Sized + IDispatchImpl {
    fn Value();
    fn SetValue();
    fn Name();
    fn IsLocal();
    fn Origin();
    fn CIMType();
    fn Qualifiers_();
    fn IsArray();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISWbemProperty {
    const NAME: &'static str = "Windows.Win32.System.Wmi.ISWbemProperty";
}
#[cfg(feature = "Win32_System_Com")]
impl ISWbemPropertyVtbl {
    pub const fn new<Impl: ISWbemPropertyImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISWbemPropertyVtbl {
        unsafe extern "system" fn Value<Impl: ISWbemPropertyImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varvalue: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Value(::core::mem::transmute_copy(&varvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValue<Impl: ISWbemPropertyImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varvalue: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetValue(&*(&varvalue as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Impl: ISWbemPropertyImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Name(::core::mem::transmute_copy(&strname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsLocal<Impl: ISWbemPropertyImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bislocal: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsLocal(::core::mem::transmute_copy(&bislocal)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Origin<Impl: ISWbemPropertyImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strorigin: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Origin(::core::mem::transmute_copy(&strorigin)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CIMType<Impl: ISWbemPropertyImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, icimtype: *mut WbemCimtypeEnum) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CIMType(::core::mem::transmute_copy(&icimtype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Qualifiers_<Impl: ISWbemPropertyImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, objwbemqualifierset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Qualifiers_(::core::mem::transmute_copy(&objwbemqualifierset)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsArray<Impl: ISWbemPropertyImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bisarray: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsArray(::core::mem::transmute_copy(&bisarray)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISWbemProperty>, base.5, Value::<Impl, OFFSET>, SetValue::<Impl, OFFSET>, Name::<Impl, OFFSET>, IsLocal::<Impl, OFFSET>, Origin::<Impl, OFFSET>, CIMType::<Impl, OFFSET>, Qualifiers_::<Impl, OFFSET>, IsArray::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISWbemPropertySetImpl: Sized + IDispatchImpl {
    fn _NewEnum();
    fn Item();
    fn Count();
    fn Add();
    fn Remove();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISWbemPropertySet {
    const NAME: &'static str = "Windows.Win32.System.Wmi.ISWbemPropertySet";
}
#[cfg(feature = "Win32_System_Com")]
impl ISWbemPropertySetVtbl {
    pub const fn new<Impl: ISWbemPropertySetImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISWbemPropertySetVtbl {
        unsafe extern "system" fn _NewEnum<Impl: ISWbemPropertySetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this)._NewEnum(::core::mem::transmute_copy(&punk)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: ISWbemPropertySetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemproperty: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Item(&*(&strname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), iflags, ::core::mem::transmute_copy(&objwbemproperty)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: ISWbemPropertySetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, icount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Count(::core::mem::transmute_copy(&icount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Impl: ISWbemPropertySetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, icimtype: WbemCimtypeEnum, bisarray: i16, iflags: i32, objwbemproperty: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Add(&*(&strname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), icimtype, bisarray, iflags, ::core::mem::transmute_copy(&objwbemproperty)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Remove<Impl: ISWbemPropertySetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Remove(&*(&strname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), iflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISWbemPropertySet>, base.5, _NewEnum::<Impl, OFFSET>, Item::<Impl, OFFSET>, Count::<Impl, OFFSET>, Add::<Impl, OFFSET>, Remove::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISWbemQualifierImpl: Sized + IDispatchImpl {
    fn Value();
    fn SetValue();
    fn Name();
    fn IsLocal();
    fn PropagatesToSubclass();
    fn SetPropagatesToSubclass();
    fn PropagatesToInstance();
    fn SetPropagatesToInstance();
    fn IsOverridable();
    fn SetIsOverridable();
    fn IsAmended();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISWbemQualifier {
    const NAME: &'static str = "Windows.Win32.System.Wmi.ISWbemQualifier";
}
#[cfg(feature = "Win32_System_Com")]
impl ISWbemQualifierVtbl {
    pub const fn new<Impl: ISWbemQualifierImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISWbemQualifierVtbl {
        unsafe extern "system" fn Value<Impl: ISWbemQualifierImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varvalue: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Value(::core::mem::transmute_copy(&varvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValue<Impl: ISWbemQualifierImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varvalue: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetValue(&*(&varvalue as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Impl: ISWbemQualifierImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Name(::core::mem::transmute_copy(&strname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsLocal<Impl: ISWbemQualifierImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bislocal: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsLocal(::core::mem::transmute_copy(&bislocal)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PropagatesToSubclass<Impl: ISWbemQualifierImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bpropagatestosubclass: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PropagatesToSubclass(::core::mem::transmute_copy(&bpropagatestosubclass)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPropagatesToSubclass<Impl: ISWbemQualifierImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bpropagatestosubclass: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetPropagatesToSubclass(bpropagatestosubclass) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PropagatesToInstance<Impl: ISWbemQualifierImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bpropagatestoinstance: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PropagatesToInstance(::core::mem::transmute_copy(&bpropagatestoinstance)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPropagatesToInstance<Impl: ISWbemQualifierImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bpropagatestoinstance: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetPropagatesToInstance(bpropagatestoinstance) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsOverridable<Impl: ISWbemQualifierImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bisoverridable: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsOverridable(::core::mem::transmute_copy(&bisoverridable)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsOverridable<Impl: ISWbemQualifierImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bisoverridable: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetIsOverridable(bisoverridable) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsAmended<Impl: ISWbemQualifierImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bisamended: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsAmended(::core::mem::transmute_copy(&bisamended)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISWbemQualifier>, base.5, Value::<Impl, OFFSET>, SetValue::<Impl, OFFSET>, Name::<Impl, OFFSET>, IsLocal::<Impl, OFFSET>, PropagatesToSubclass::<Impl, OFFSET>, SetPropagatesToSubclass::<Impl, OFFSET>, PropagatesToInstance::<Impl, OFFSET>, SetPropagatesToInstance::<Impl, OFFSET>, IsOverridable::<Impl, OFFSET>, SetIsOverridable::<Impl, OFFSET>, IsAmended::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISWbemQualifierSetImpl: Sized + IDispatchImpl {
    fn _NewEnum();
    fn Item();
    fn Count();
    fn Add();
    fn Remove();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISWbemQualifierSet {
    const NAME: &'static str = "Windows.Win32.System.Wmi.ISWbemQualifierSet";
}
#[cfg(feature = "Win32_System_Com")]
impl ISWbemQualifierSetVtbl {
    pub const fn new<Impl: ISWbemQualifierSetImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISWbemQualifierSetVtbl {
        unsafe extern "system" fn _NewEnum<Impl: ISWbemQualifierSetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this)._NewEnum(::core::mem::transmute_copy(&punk)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: ISWbemQualifierSetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemqualifier: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Item(&*(&name as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), iflags, ::core::mem::transmute_copy(&objwbemqualifier)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: ISWbemQualifierSetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, icount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Count(::core::mem::transmute_copy(&icount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Impl: ISWbemQualifierSetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varval: *const super::Com::VARIANT, bpropagatestosubclass: i16, bpropagatestoinstance: i16, bisoverridable: i16, iflags: i32, objwbemqualifier: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Add(&*(&strname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&varval as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), bpropagatestosubclass, bpropagatestoinstance, bisoverridable, iflags, ::core::mem::transmute_copy(&objwbemqualifier)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Remove<Impl: ISWbemQualifierSetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Remove(&*(&strname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), iflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISWbemQualifierSet>, base.5, _NewEnum::<Impl, OFFSET>, Item::<Impl, OFFSET>, Count::<Impl, OFFSET>, Add::<Impl, OFFSET>, Remove::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISWbemRefreshableItemImpl: Sized + IDispatchImpl {
    fn Index();
    fn Refresher();
    fn IsSet();
    fn Object();
    fn ObjectSet();
    fn Remove();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISWbemRefreshableItem {
    const NAME: &'static str = "Windows.Win32.System.Wmi.ISWbemRefreshableItem";
}
#[cfg(feature = "Win32_System_Com")]
impl ISWbemRefreshableItemVtbl {
    pub const fn new<Impl: ISWbemRefreshableItemImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISWbemRefreshableItemVtbl {
        unsafe extern "system" fn Index<Impl: ISWbemRefreshableItemImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iindex: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Index(::core::mem::transmute_copy(&iindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresher<Impl: ISWbemRefreshableItemImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, objwbemrefresher: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Refresher(::core::mem::transmute_copy(&objwbemrefresher)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsSet<Impl: ISWbemRefreshableItemImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bisset: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsSet(::core::mem::transmute_copy(&bisset)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Object<Impl: ISWbemRefreshableItemImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, objwbemobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Object(::core::mem::transmute_copy(&objwbemobject)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ObjectSet<Impl: ISWbemRefreshableItemImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, objwbemobjectset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ObjectSet(::core::mem::transmute_copy(&objwbemobjectset)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Remove<Impl: ISWbemRefreshableItemImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Remove(iflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISWbemRefreshableItem>, base.5, Index::<Impl, OFFSET>, Refresher::<Impl, OFFSET>, IsSet::<Impl, OFFSET>, Object::<Impl, OFFSET>, ObjectSet::<Impl, OFFSET>, Remove::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISWbemRefresherImpl: Sized + IDispatchImpl {
    fn _NewEnum();
    fn Item();
    fn Count();
    fn Add();
    fn AddEnum();
    fn Remove();
    fn Refresh();
    fn AutoReconnect();
    fn SetAutoReconnect();
    fn DeleteAll();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISWbemRefresher {
    const NAME: &'static str = "Windows.Win32.System.Wmi.ISWbemRefresher";
}
#[cfg(feature = "Win32_System_Com")]
impl ISWbemRefresherVtbl {
    pub const fn new<Impl: ISWbemRefresherImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISWbemRefresherVtbl {
        unsafe extern "system" fn _NewEnum<Impl: ISWbemRefresherImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this)._NewEnum(::core::mem::transmute_copy(&punk)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: ISWbemRefresherImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iindex: i32, objwbemrefreshableitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Item(iindex, ::core::mem::transmute_copy(&objwbemrefreshableitem)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: ISWbemRefresherImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, icount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Count(::core::mem::transmute_copy(&icount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Impl: ISWbemRefresherImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, objwbemservices: ::windows::core::RawPtr, bsinstancepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemrefreshableitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Add(
                &*(&objwbemservices as *const <ISWbemServicesEx as ::windows::core::Abi>::Abi as *const <ISWbemServicesEx as ::windows::core::DefaultType>::DefaultType),
                &*(&bsinstancepath as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                iflags,
                &*(&objwbemnamedvalueset as *const <super::Com::IDispatch as ::windows::core::Abi>::Abi as *const <super::Com::IDispatch as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&objwbemrefreshableitem),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddEnum<Impl: ISWbemRefresherImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, objwbemservices: ::windows::core::RawPtr, bsclassname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemrefreshableitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddEnum(
                &*(&objwbemservices as *const <ISWbemServicesEx as ::windows::core::Abi>::Abi as *const <ISWbemServicesEx as ::windows::core::DefaultType>::DefaultType),
                &*(&bsclassname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                iflags,
                &*(&objwbemnamedvalueset as *const <super::Com::IDispatch as ::windows::core::Abi>::Abi as *const <super::Com::IDispatch as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&objwbemrefreshableitem),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Remove<Impl: ISWbemRefresherImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iindex: i32, iflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Remove(iindex, iflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Impl: ISWbemRefresherImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Refresh(iflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AutoReconnect<Impl: ISWbemRefresherImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bcount: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AutoReconnect(::core::mem::transmute_copy(&bcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoReconnect<Impl: ISWbemRefresherImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bcount: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetAutoReconnect(bcount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteAll<Impl: ISWbemRefresherImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DeleteAll() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISWbemRefresher>, base.5, _NewEnum::<Impl, OFFSET>, Item::<Impl, OFFSET>, Count::<Impl, OFFSET>, Add::<Impl, OFFSET>, AddEnum::<Impl, OFFSET>, Remove::<Impl, OFFSET>, Refresh::<Impl, OFFSET>, AutoReconnect::<Impl, OFFSET>, SetAutoReconnect::<Impl, OFFSET>, DeleteAll::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISWbemSecurityImpl: Sized + IDispatchImpl {
    fn ImpersonationLevel();
    fn SetImpersonationLevel();
    fn AuthenticationLevel();
    fn SetAuthenticationLevel();
    fn Privileges();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISWbemSecurity {
    const NAME: &'static str = "Windows.Win32.System.Wmi.ISWbemSecurity";
}
#[cfg(feature = "Win32_System_Com")]
impl ISWbemSecurityVtbl {
    pub const fn new<Impl: ISWbemSecurityImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISWbemSecurityVtbl {
        unsafe extern "system" fn ImpersonationLevel<Impl: ISWbemSecurityImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iimpersonationlevel: *mut WbemImpersonationLevelEnum) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ImpersonationLevel(::core::mem::transmute_copy(&iimpersonationlevel)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetImpersonationLevel<Impl: ISWbemSecurityImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iimpersonationlevel: WbemImpersonationLevelEnum) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetImpersonationLevel(iimpersonationlevel) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AuthenticationLevel<Impl: ISWbemSecurityImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iauthenticationlevel: *mut WbemAuthenticationLevelEnum) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AuthenticationLevel(::core::mem::transmute_copy(&iauthenticationlevel)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAuthenticationLevel<Impl: ISWbemSecurityImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iauthenticationlevel: WbemAuthenticationLevelEnum) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetAuthenticationLevel(iauthenticationlevel) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Privileges<Impl: ISWbemSecurityImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, objwbemprivilegeset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Privileges(::core::mem::transmute_copy(&objwbemprivilegeset)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISWbemSecurity>, base.5, ImpersonationLevel::<Impl, OFFSET>, SetImpersonationLevel::<Impl, OFFSET>, AuthenticationLevel::<Impl, OFFSET>, SetAuthenticationLevel::<Impl, OFFSET>, Privileges::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISWbemServicesImpl: Sized + IDispatchImpl {
    fn Get();
    fn GetAsync();
    fn Delete();
    fn DeleteAsync();
    fn InstancesOf();
    fn InstancesOfAsync();
    fn SubclassesOf();
    fn SubclassesOfAsync();
    fn ExecQuery();
    fn ExecQueryAsync();
    fn AssociatorsOf();
    fn AssociatorsOfAsync();
    fn ReferencesTo();
    fn ReferencesToAsync();
    fn ExecNotificationQuery();
    fn ExecNotificationQueryAsync();
    fn ExecMethod();
    fn ExecMethodAsync();
    fn Security_();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISWbemServices {
    const NAME: &'static str = "Windows.Win32.System.Wmi.ISWbemServices";
}
#[cfg(feature = "Win32_System_Com")]
impl ISWbemServicesVtbl {
    pub const fn new<Impl: ISWbemServicesImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISWbemServicesVtbl {
        unsafe extern "system" fn Get<Impl: ISWbemServicesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strobjectpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Get(&*(&strobjectpath as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), iflags, &*(&objwbemnamedvalueset as *const <super::Com::IDispatch as ::windows::core::Abi>::Abi as *const <super::Com::IDispatch as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&objwbemobject)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAsync<Impl: ISWbemServicesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, objwbemsink: ::windows::core::RawPtr, strobjectpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemasynccontext: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAsync(
                &*(&objwbemsink as *const <super::Com::IDispatch as ::windows::core::Abi>::Abi as *const <super::Com::IDispatch as ::windows::core::DefaultType>::DefaultType),
                &*(&strobjectpath as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                iflags,
                &*(&objwbemnamedvalueset as *const <super::Com::IDispatch as ::windows::core::Abi>::Abi as *const <super::Com::IDispatch as ::windows::core::DefaultType>::DefaultType),
                &*(&objwbemasynccontext as *const <super::Com::IDispatch as ::windows::core::Abi>::Abi as *const <super::Com::IDispatch as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Impl: ISWbemServicesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strobjectpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Delete(&*(&strobjectpath as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), iflags, &*(&objwbemnamedvalueset as *const <super::Com::IDispatch as ::windows::core::Abi>::Abi as *const <super::Com::IDispatch as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteAsync<Impl: ISWbemServicesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, objwbemsink: ::windows::core::RawPtr, strobjectpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemasynccontext: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DeleteAsync(
                &*(&objwbemsink as *const <super::Com::IDispatch as ::windows::core::Abi>::Abi as *const <super::Com::IDispatch as ::windows::core::DefaultType>::DefaultType),
                &*(&strobjectpath as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                iflags,
                &*(&objwbemnamedvalueset as *const <super::Com::IDispatch as ::windows::core::Abi>::Abi as *const <super::Com::IDispatch as ::windows::core::DefaultType>::DefaultType),
                &*(&objwbemasynccontext as *const <super::Com::IDispatch as ::windows::core::Abi>::Abi as *const <super::Com::IDispatch as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InstancesOf<Impl: ISWbemServicesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemobjectset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InstancesOf(&*(&strclass as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), iflags, &*(&objwbemnamedvalueset as *const <super::Com::IDispatch as ::windows::core::Abi>::Abi as *const <super::Com::IDispatch as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&objwbemobjectset)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InstancesOfAsync<Impl: ISWbemServicesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, objwbemsink: ::windows::core::RawPtr, strclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemasynccontext: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InstancesOfAsync(
                &*(&objwbemsink as *const <super::Com::IDispatch as ::windows::core::Abi>::Abi as *const <super::Com::IDispatch as ::windows::core::DefaultType>::DefaultType),
                &*(&strclass as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                iflags,
                &*(&objwbemnamedvalueset as *const <super::Com::IDispatch as ::windows::core::Abi>::Abi as *const <super::Com::IDispatch as ::windows::core::DefaultType>::DefaultType),
                &*(&objwbemasynccontext as *const <super::Com::IDispatch as ::windows::core::Abi>::Abi as *const <super::Com::IDispatch as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SubclassesOf<Impl: ISWbemServicesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strsuperclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemobjectset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SubclassesOf(&*(&strsuperclass as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), iflags, &*(&objwbemnamedvalueset as *const <super::Com::IDispatch as ::windows::core::Abi>::Abi as *const <super::Com::IDispatch as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&objwbemobjectset)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SubclassesOfAsync<Impl: ISWbemServicesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, objwbemsink: ::windows::core::RawPtr, strsuperclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemasynccontext: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SubclassesOfAsync(
                &*(&objwbemsink as *const <super::Com::IDispatch as ::windows::core::Abi>::Abi as *const <super::Com::IDispatch as ::windows::core::DefaultType>::DefaultType),
                &*(&strsuperclass as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                iflags,
                &*(&objwbemnamedvalueset as *const <super::Com::IDispatch as ::windows::core::Abi>::Abi as *const <super::Com::IDispatch as ::windows::core::DefaultType>::DefaultType),
                &*(&objwbemasynccontext as *const <super::Com::IDispatch as ::windows::core::Abi>::Abi as *const <super::Com::IDispatch as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExecQuery<Impl: ISWbemServicesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strquery: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strquerylanguage: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemobjectset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ExecQuery(
                &*(&strquery as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&strquerylanguage as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                iflags,
                &*(&objwbemnamedvalueset as *const <super::Com::IDispatch as ::windows::core::Abi>::Abi as *const <super::Com::IDispatch as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&objwbemobjectset),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExecQueryAsync<Impl: ISWbemServicesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, objwbemsink: ::windows::core::RawPtr, strquery: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strquerylanguage: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemasynccontext: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ExecQueryAsync(
                &*(&objwbemsink as *const <super::Com::IDispatch as ::windows::core::Abi>::Abi as *const <super::Com::IDispatch as ::windows::core::DefaultType>::DefaultType),
                &*(&strquery as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&strquerylanguage as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                lflags,
                &*(&objwbemnamedvalueset as *const <super::Com::IDispatch as ::windows::core::Abi>::Abi as *const <super::Com::IDispatch as ::windows::core::DefaultType>::DefaultType),
                &*(&objwbemasynccontext as *const <super::Com::IDispatch as ::windows::core::Abi>::Abi as *const <super::Com::IDispatch as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AssociatorsOf<Impl: ISWbemServicesImpl, const OFFSET: usize>(
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
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AssociatorsOf(
                &*(&strobjectpath as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&strassocclass as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&strresultclass as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&strresultrole as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&strrole as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                bclassesonly,
                bschemaonly,
                &*(&strrequiredassocqualifier as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&strrequiredqualifier as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                iflags,
                &*(&objwbemnamedvalueset as *const <super::Com::IDispatch as ::windows::core::Abi>::Abi as *const <super::Com::IDispatch as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&objwbemobjectset),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AssociatorsOfAsync<Impl: ISWbemServicesImpl, const OFFSET: usize>(
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
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AssociatorsOfAsync(
                &*(&objwbemsink as *const <super::Com::IDispatch as ::windows::core::Abi>::Abi as *const <super::Com::IDispatch as ::windows::core::DefaultType>::DefaultType),
                &*(&strobjectpath as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&strassocclass as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&strresultclass as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&strresultrole as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&strrole as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                bclassesonly,
                bschemaonly,
                &*(&strrequiredassocqualifier as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&strrequiredqualifier as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                iflags,
                &*(&objwbemnamedvalueset as *const <super::Com::IDispatch as ::windows::core::Abi>::Abi as *const <super::Com::IDispatch as ::windows::core::DefaultType>::DefaultType),
                &*(&objwbemasynccontext as *const <super::Com::IDispatch as ::windows::core::Abi>::Abi as *const <super::Com::IDispatch as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReferencesTo<Impl: ISWbemServicesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strobjectpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strresultclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strrole: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bclassesonly: i16, bschemaonly: i16, strrequiredqualifier: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemobjectset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ReferencesTo(
                &*(&strobjectpath as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&strresultclass as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&strrole as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                bclassesonly,
                bschemaonly,
                &*(&strrequiredqualifier as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                iflags,
                &*(&objwbemnamedvalueset as *const <super::Com::IDispatch as ::windows::core::Abi>::Abi as *const <super::Com::IDispatch as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&objwbemobjectset),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReferencesToAsync<Impl: ISWbemServicesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, objwbemsink: ::windows::core::RawPtr, strobjectpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strresultclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strrole: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bclassesonly: i16, bschemaonly: i16, strrequiredqualifier: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemasynccontext: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ReferencesToAsync(
                &*(&objwbemsink as *const <super::Com::IDispatch as ::windows::core::Abi>::Abi as *const <super::Com::IDispatch as ::windows::core::DefaultType>::DefaultType),
                &*(&strobjectpath as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&strresultclass as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&strrole as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                bclassesonly,
                bschemaonly,
                &*(&strrequiredqualifier as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                iflags,
                &*(&objwbemnamedvalueset as *const <super::Com::IDispatch as ::windows::core::Abi>::Abi as *const <super::Com::IDispatch as ::windows::core::DefaultType>::DefaultType),
                &*(&objwbemasynccontext as *const <super::Com::IDispatch as ::windows::core::Abi>::Abi as *const <super::Com::IDispatch as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExecNotificationQuery<Impl: ISWbemServicesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strquery: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strquerylanguage: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemeventsource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ExecNotificationQuery(
                &*(&strquery as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&strquerylanguage as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                iflags,
                &*(&objwbemnamedvalueset as *const <super::Com::IDispatch as ::windows::core::Abi>::Abi as *const <super::Com::IDispatch as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&objwbemeventsource),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExecNotificationQueryAsync<Impl: ISWbemServicesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, objwbemsink: ::windows::core::RawPtr, strquery: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strquerylanguage: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemasynccontext: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ExecNotificationQueryAsync(
                &*(&objwbemsink as *const <super::Com::IDispatch as ::windows::core::Abi>::Abi as *const <super::Com::IDispatch as ::windows::core::DefaultType>::DefaultType),
                &*(&strquery as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&strquerylanguage as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                iflags,
                &*(&objwbemnamedvalueset as *const <super::Com::IDispatch as ::windows::core::Abi>::Abi as *const <super::Com::IDispatch as ::windows::core::DefaultType>::DefaultType),
                &*(&objwbemasynccontext as *const <super::Com::IDispatch as ::windows::core::Abi>::Abi as *const <super::Com::IDispatch as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExecMethod<Impl: ISWbemServicesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strobjectpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strmethodname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, objwbeminparameters: ::windows::core::RawPtr, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemoutparameters: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ExecMethod(
                &*(&strobjectpath as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&strmethodname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&objwbeminparameters as *const <super::Com::IDispatch as ::windows::core::Abi>::Abi as *const <super::Com::IDispatch as ::windows::core::DefaultType>::DefaultType),
                iflags,
                &*(&objwbemnamedvalueset as *const <super::Com::IDispatch as ::windows::core::Abi>::Abi as *const <super::Com::IDispatch as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&objwbemoutparameters),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExecMethodAsync<Impl: ISWbemServicesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, objwbemsink: ::windows::core::RawPtr, strobjectpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strmethodname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, objwbeminparameters: ::windows::core::RawPtr, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemasynccontext: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ExecMethodAsync(
                &*(&objwbemsink as *const <super::Com::IDispatch as ::windows::core::Abi>::Abi as *const <super::Com::IDispatch as ::windows::core::DefaultType>::DefaultType),
                &*(&strobjectpath as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&strmethodname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&objwbeminparameters as *const <super::Com::IDispatch as ::windows::core::Abi>::Abi as *const <super::Com::IDispatch as ::windows::core::DefaultType>::DefaultType),
                iflags,
                &*(&objwbemnamedvalueset as *const <super::Com::IDispatch as ::windows::core::Abi>::Abi as *const <super::Com::IDispatch as ::windows::core::DefaultType>::DefaultType),
                &*(&objwbemasynccontext as *const <super::Com::IDispatch as ::windows::core::Abi>::Abi as *const <super::Com::IDispatch as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Security_<Impl: ISWbemServicesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, objwbemsecurity: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Security_(::core::mem::transmute_copy(&objwbemsecurity)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<ISWbemServices>,
            base.5,
            Get::<Impl, OFFSET>,
            GetAsync::<Impl, OFFSET>,
            Delete::<Impl, OFFSET>,
            DeleteAsync::<Impl, OFFSET>,
            InstancesOf::<Impl, OFFSET>,
            InstancesOfAsync::<Impl, OFFSET>,
            SubclassesOf::<Impl, OFFSET>,
            SubclassesOfAsync::<Impl, OFFSET>,
            ExecQuery::<Impl, OFFSET>,
            ExecQueryAsync::<Impl, OFFSET>,
            AssociatorsOf::<Impl, OFFSET>,
            AssociatorsOfAsync::<Impl, OFFSET>,
            ReferencesTo::<Impl, OFFSET>,
            ReferencesToAsync::<Impl, OFFSET>,
            ExecNotificationQuery::<Impl, OFFSET>,
            ExecNotificationQueryAsync::<Impl, OFFSET>,
            ExecMethod::<Impl, OFFSET>,
            ExecMethodAsync::<Impl, OFFSET>,
            Security_::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISWbemServicesExImpl: Sized + ISWbemServicesImpl + IDispatchImpl {
    fn Put();
    fn PutAsync();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISWbemServicesEx {
    const NAME: &'static str = "Windows.Win32.System.Wmi.ISWbemServicesEx";
}
#[cfg(feature = "Win32_System_Com")]
impl ISWbemServicesExVtbl {
    pub const fn new<Impl: ISWbemServicesExImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISWbemServicesExVtbl {
        unsafe extern "system" fn Put<Impl: ISWbemServicesExImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, objwbemobject: ::windows::core::RawPtr, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemobjectpath: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Put(&*(&objwbemobject as *const <ISWbemObjectEx as ::windows::core::Abi>::Abi as *const <ISWbemObjectEx as ::windows::core::DefaultType>::DefaultType), iflags, &*(&objwbemnamedvalueset as *const <super::Com::IDispatch as ::windows::core::Abi>::Abi as *const <super::Com::IDispatch as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&objwbemobjectpath)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PutAsync<Impl: ISWbemServicesExImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, objwbemsink: ::windows::core::RawPtr, objwbemobject: ::windows::core::RawPtr, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemasynccontext: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PutAsync(
                &*(&objwbemsink as *const <ISWbemSink as ::windows::core::Abi>::Abi as *const <ISWbemSink as ::windows::core::DefaultType>::DefaultType),
                &*(&objwbemobject as *const <ISWbemObjectEx as ::windows::core::Abi>::Abi as *const <ISWbemObjectEx as ::windows::core::DefaultType>::DefaultType),
                iflags,
                &*(&objwbemnamedvalueset as *const <super::Com::IDispatch as ::windows::core::Abi>::Abi as *const <super::Com::IDispatch as ::windows::core::DefaultType>::DefaultType),
                &*(&objwbemasynccontext as *const <super::Com::IDispatch as ::windows::core::Abi>::Abi as *const <super::Com::IDispatch as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISWbemServicesEx>, base.5, Put::<Impl, OFFSET>, PutAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISWbemSinkImpl: Sized + IDispatchImpl {
    fn Cancel();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISWbemSink {
    const NAME: &'static str = "Windows.Win32.System.Wmi.ISWbemSink";
}
#[cfg(feature = "Win32_System_Com")]
impl ISWbemSinkVtbl {
    pub const fn new<Impl: ISWbemSinkImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISWbemSinkVtbl {
        unsafe extern "system" fn Cancel<Impl: ISWbemSinkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Cancel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISWbemSink>, base.5, Cancel::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISWbemSinkEventsImpl: Sized + IDispatchImpl {}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISWbemSinkEvents {
    const NAME: &'static str = "Windows.Win32.System.Wmi.ISWbemSinkEvents";
}
#[cfg(feature = "Win32_System_Com")]
impl ISWbemSinkEventsVtbl {
    pub const fn new<Impl: ISWbemSinkEventsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISWbemSinkEventsVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISWbemSinkEvents>, base.5)
    }
}
pub trait IUnsecuredApartmentImpl: Sized {
    fn CreateObjectStub();
}
impl ::windows::core::RuntimeName for IUnsecuredApartment {
    const NAME: &'static str = "Windows.Win32.System.Wmi.IUnsecuredApartment";
}
impl IUnsecuredApartmentVtbl {
    pub const fn new<Impl: IUnsecuredApartmentImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUnsecuredApartmentVtbl {
        unsafe extern "system" fn CreateObjectStub<Impl: IUnsecuredApartmentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pobject: *mut ::core::ffi::c_void, ppstub: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateObjectStub(&*(&pobject as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppstub)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IUnsecuredApartment>, base.5, CreateObjectStub::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWMIExtensionImpl: Sized + IDispatchImpl {
    fn WMIObjectPath();
    fn GetWMIObject();
    fn GetWMIServices();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IWMIExtension {
    const NAME: &'static str = "Windows.Win32.System.Wmi.IWMIExtension";
}
#[cfg(feature = "Win32_System_Com")]
impl IWMIExtensionVtbl {
    pub const fn new<Impl: IWMIExtensionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWMIExtensionVtbl {
        unsafe extern "system" fn WMIObjectPath<Impl: IWMIExtensionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strwmiobjectpath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).WMIObjectPath(::core::mem::transmute_copy(&strwmiobjectpath)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWMIObject<Impl: IWMIExtensionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, objwmiobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetWMIObject(::core::mem::transmute_copy(&objwmiobject)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWMIServices<Impl: IWMIExtensionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, objwmiservices: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetWMIServices(::core::mem::transmute_copy(&objwmiservices)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWMIExtension>, base.5, WMIObjectPath::<Impl, OFFSET>, GetWMIObject::<Impl, OFFSET>, GetWMIServices::<Impl, OFFSET>)
    }
}
pub trait IWbemAddressResolutionImpl: Sized {
    fn Resolve();
}
impl ::windows::core::RuntimeName for IWbemAddressResolution {
    const NAME: &'static str = "Windows.Win32.System.Wmi.IWbemAddressResolution";
}
impl IWbemAddressResolutionVtbl {
    pub const fn new<Impl: IWbemAddressResolutionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWbemAddressResolutionVtbl {
        unsafe extern "system" fn Resolve<Impl: IWbemAddressResolutionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wsznamespacepath: super::super::Foundation::PWSTR, wszaddresstype: super::super::Foundation::PWSTR, pdwaddresslength: *mut u32, pabbinaryaddress: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Resolve(&*(&wsznamespacepath as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&wszaddresstype), ::core::mem::transmute_copy(&pdwaddresslength), ::core::mem::transmute_copy(&pabbinaryaddress)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWbemAddressResolution>, base.5, Resolve::<Impl, OFFSET>)
    }
}
pub trait IWbemBackupRestoreImpl: Sized {
    fn Backup();
    fn Restore();
}
impl ::windows::core::RuntimeName for IWbemBackupRestore {
    const NAME: &'static str = "Windows.Win32.System.Wmi.IWbemBackupRestore";
}
impl IWbemBackupRestoreVtbl {
    pub const fn new<Impl: IWbemBackupRestoreImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWbemBackupRestoreVtbl {
        unsafe extern "system" fn Backup<Impl: IWbemBackupRestoreImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strbackuptofile: super::super::Foundation::PWSTR, lflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Backup(&*(&strbackuptofile as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), lflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Restore<Impl: IWbemBackupRestoreImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strrestorefromfile: super::super::Foundation::PWSTR, lflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Restore(&*(&strrestorefromfile as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), lflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWbemBackupRestore>, base.5, Backup::<Impl, OFFSET>, Restore::<Impl, OFFSET>)
    }
}
pub trait IWbemBackupRestoreExImpl: Sized + IWbemBackupRestoreImpl {
    fn Pause();
    fn Resume();
}
impl ::windows::core::RuntimeName for IWbemBackupRestoreEx {
    const NAME: &'static str = "Windows.Win32.System.Wmi.IWbemBackupRestoreEx";
}
impl IWbemBackupRestoreExVtbl {
    pub const fn new<Impl: IWbemBackupRestoreExImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWbemBackupRestoreExVtbl {
        unsafe extern "system" fn Pause<Impl: IWbemBackupRestoreExImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Pause() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Resume<Impl: IWbemBackupRestoreExImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Resume() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWbemBackupRestoreEx>, base.5, Pause::<Impl, OFFSET>, Resume::<Impl, OFFSET>)
    }
}
pub trait IWbemCallResultImpl: Sized {
    fn GetResultObject();
    fn GetResultString();
    fn GetResultServices();
    fn GetCallStatus();
}
impl ::windows::core::RuntimeName for IWbemCallResult {
    const NAME: &'static str = "Windows.Win32.System.Wmi.IWbemCallResult";
}
impl IWbemCallResultVtbl {
    pub const fn new<Impl: IWbemCallResultImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWbemCallResultVtbl {
        unsafe extern "system" fn GetResultObject<Impl: IWbemCallResultImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ltimeout: i32, ppresultobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetResultObject(ltimeout, ::core::mem::transmute_copy(&ppresultobject)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetResultString<Impl: IWbemCallResultImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ltimeout: i32, pstrresultstring: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetResultString(ltimeout, ::core::mem::transmute_copy(&pstrresultstring)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetResultServices<Impl: IWbemCallResultImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ltimeout: i32, ppservices: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetResultServices(ltimeout, ::core::mem::transmute_copy(&ppservices)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCallStatus<Impl: IWbemCallResultImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ltimeout: i32, plstatus: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCallStatus(ltimeout, ::core::mem::transmute_copy(&plstatus)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWbemCallResult>, base.5, GetResultObject::<Impl, OFFSET>, GetResultString::<Impl, OFFSET>, GetResultServices::<Impl, OFFSET>, GetCallStatus::<Impl, OFFSET>)
    }
}
pub trait IWbemClassObjectImpl: Sized {
    fn GetQualifierSet();
    fn Get();
    fn Put();
    fn Delete();
    fn GetNames();
    fn BeginEnumeration();
    fn Next();
    fn EndEnumeration();
    fn GetPropertyQualifierSet();
    fn Clone();
    fn GetObjectText();
    fn SpawnDerivedClass();
    fn SpawnInstance();
    fn CompareTo();
    fn GetPropertyOrigin();
    fn InheritsFrom();
    fn GetMethod();
    fn PutMethod();
    fn DeleteMethod();
    fn BeginMethodEnumeration();
    fn NextMethod();
    fn EndMethodEnumeration();
    fn GetMethodQualifierSet();
    fn GetMethodOrigin();
}
impl ::windows::core::RuntimeName for IWbemClassObject {
    const NAME: &'static str = "Windows.Win32.System.Wmi.IWbemClassObject";
}
impl IWbemClassObjectVtbl {
    pub const fn new<Impl: IWbemClassObjectImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWbemClassObjectVtbl {
        unsafe extern "system" fn GetQualifierSet<Impl: IWbemClassObjectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppqualset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetQualifierSet(::core::mem::transmute_copy(&ppqualset)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Get<Impl: IWbemClassObjectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wszname: super::super::Foundation::PWSTR, lflags: i32, pval: *mut super::Com::VARIANT, ptype: *mut i32, plflavor: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Get(&*(&wszname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), lflags, &*(&pval as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), ptype, plflavor) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Put<Impl: IWbemClassObjectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wszname: super::super::Foundation::PWSTR, lflags: i32, pval: *const super::Com::VARIANT, r#type: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Put(&*(&wszname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), lflags, &*(&pval as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), r#type) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Impl: IWbemClassObjectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wszname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Delete(&*(&wszname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNames<Impl: IWbemClassObjectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wszqualifiername: super::super::Foundation::PWSTR, lflags: i32, pqualifierval: *const super::Com::VARIANT, pnames: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetNames(&*(&wszqualifiername as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), lflags, &*(&pqualifierval as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pnames)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginEnumeration<Impl: IWbemClassObjectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lenumflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BeginEnumeration(lenumflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Impl: IWbemClassObjectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32, strname: *mut super::super::Foundation::BSTR, pval: *mut super::Com::VARIANT, ptype: *mut i32, plflavor: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Next(lflags, &*(&strname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&pval as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), ptype, plflavor) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndEnumeration<Impl: IWbemClassObjectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EndEnumeration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPropertyQualifierSet<Impl: IWbemClassObjectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wszproperty: super::super::Foundation::PWSTR, ppqualset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPropertyQualifierSet(&*(&wszproperty as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppqualset)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IWbemClassObjectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcopy: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&ppcopy)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetObjectText<Impl: IWbemClassObjectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32, pstrobjecttext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetObjectText(lflags, ::core::mem::transmute_copy(&pstrobjecttext)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SpawnDerivedClass<Impl: IWbemClassObjectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32, ppnewclass: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SpawnDerivedClass(lflags, ::core::mem::transmute_copy(&ppnewclass)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SpawnInstance<Impl: IWbemClassObjectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32, ppnewinstance: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SpawnInstance(lflags, ::core::mem::transmute_copy(&ppnewinstance)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CompareTo<Impl: IWbemClassObjectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32, pcompareto: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CompareTo(lflags, &*(&pcompareto as *const <IWbemClassObject as ::windows::core::Abi>::Abi as *const <IWbemClassObject as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPropertyOrigin<Impl: IWbemClassObjectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wszname: super::super::Foundation::PWSTR, pstrclassname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPropertyOrigin(&*(&wszname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pstrclassname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InheritsFrom<Impl: IWbemClassObjectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strancestor: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InheritsFrom(&*(&strancestor as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMethod<Impl: IWbemClassObjectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wszname: super::super::Foundation::PWSTR, lflags: i32, ppinsignature: *mut ::windows::core::RawPtr, ppoutsignature: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetMethod(&*(&wszname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), lflags, ::core::mem::transmute_copy(&ppinsignature), ::core::mem::transmute_copy(&ppoutsignature)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PutMethod<Impl: IWbemClassObjectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wszname: super::super::Foundation::PWSTR, lflags: i32, pinsignature: ::windows::core::RawPtr, poutsignature: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PutMethod(
                &*(&wszname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                lflags,
                &*(&pinsignature as *const <IWbemClassObject as ::windows::core::Abi>::Abi as *const <IWbemClassObject as ::windows::core::DefaultType>::DefaultType),
                &*(&poutsignature as *const <IWbemClassObject as ::windows::core::Abi>::Abi as *const <IWbemClassObject as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteMethod<Impl: IWbemClassObjectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wszname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DeleteMethod(&*(&wszname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginMethodEnumeration<Impl: IWbemClassObjectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lenumflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BeginMethodEnumeration(lenumflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NextMethod<Impl: IWbemClassObjectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32, pstrname: *mut super::super::Foundation::BSTR, ppinsignature: *mut ::windows::core::RawPtr, ppoutsignature: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NextMethod(lflags, &*(&pstrname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppinsignature), ::core::mem::transmute_copy(&ppoutsignature)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndMethodEnumeration<Impl: IWbemClassObjectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EndMethodEnumeration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMethodQualifierSet<Impl: IWbemClassObjectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wszmethod: super::super::Foundation::PWSTR, ppqualset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetMethodQualifierSet(&*(&wszmethod as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppqualset)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMethodOrigin<Impl: IWbemClassObjectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wszmethodname: super::super::Foundation::PWSTR, pstrclassname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetMethodOrigin(&*(&wszmethodname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pstrclassname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IWbemClassObject>,
            base.5,
            GetQualifierSet::<Impl, OFFSET>,
            Get::<Impl, OFFSET>,
            Put::<Impl, OFFSET>,
            Delete::<Impl, OFFSET>,
            GetNames::<Impl, OFFSET>,
            BeginEnumeration::<Impl, OFFSET>,
            Next::<Impl, OFFSET>,
            EndEnumeration::<Impl, OFFSET>,
            GetPropertyQualifierSet::<Impl, OFFSET>,
            Clone::<Impl, OFFSET>,
            GetObjectText::<Impl, OFFSET>,
            SpawnDerivedClass::<Impl, OFFSET>,
            SpawnInstance::<Impl, OFFSET>,
            CompareTo::<Impl, OFFSET>,
            GetPropertyOrigin::<Impl, OFFSET>,
            InheritsFrom::<Impl, OFFSET>,
            GetMethod::<Impl, OFFSET>,
            PutMethod::<Impl, OFFSET>,
            DeleteMethod::<Impl, OFFSET>,
            BeginMethodEnumeration::<Impl, OFFSET>,
            NextMethod::<Impl, OFFSET>,
            EndMethodEnumeration::<Impl, OFFSET>,
            GetMethodQualifierSet::<Impl, OFFSET>,
            GetMethodOrigin::<Impl, OFFSET>,
        )
    }
}
pub trait IWbemClientConnectionTransportImpl: Sized {
    fn Open();
    fn OpenAsync();
    fn Cancel();
}
impl ::windows::core::RuntimeName for IWbemClientConnectionTransport {
    const NAME: &'static str = "Windows.Win32.System.Wmi.IWbemClientConnectionTransport";
}
impl IWbemClientConnectionTransportVtbl {
    pub const fn new<Impl: IWbemClientConnectionTransportImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWbemClientConnectionTransportVtbl {
        unsafe extern "system" fn Open<Impl: IWbemClientConnectionTransportImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, straddresstype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwbinaryaddresslength: u32, abbinaryaddress: *const u8, strobject: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, struser: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strlocale: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lflags: i32, pctx: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, pinterface: *mut *mut ::core::ffi::c_void, pcallres: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Open(
                &*(&straddresstype as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                dwbinaryaddresslength,
                abbinaryaddress,
                &*(&strobject as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&struser as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&strpassword as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&strlocale as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                lflags,
                &*(&pctx as *const <IWbemContext as ::windows::core::Abi>::Abi as *const <IWbemContext as ::windows::core::DefaultType>::DefaultType),
                &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&pinterface),
                ::core::mem::transmute_copy(&pcallres),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenAsync<Impl: IWbemClientConnectionTransportImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, straddresstype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwbinaryaddresslength: u32, abbinaryaddress: *const u8, strobject: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, struser: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strlocale: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lflags: i32, pctx: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, presponsehandler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OpenAsync(
                &*(&straddresstype as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                dwbinaryaddresslength,
                abbinaryaddress,
                &*(&strobject as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&struser as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&strpassword as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&strlocale as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                lflags,
                &*(&pctx as *const <IWbemContext as ::windows::core::Abi>::Abi as *const <IWbemContext as ::windows::core::DefaultType>::DefaultType),
                &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&presponsehandler as *const <IWbemObjectSink as ::windows::core::Abi>::Abi as *const <IWbemObjectSink as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cancel<Impl: IWbemClientConnectionTransportImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32, phandler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Cancel(lflags, &*(&phandler as *const <IWbemObjectSink as ::windows::core::Abi>::Abi as *const <IWbemObjectSink as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWbemClientConnectionTransport>, base.5, Open::<Impl, OFFSET>, OpenAsync::<Impl, OFFSET>, Cancel::<Impl, OFFSET>)
    }
}
pub trait IWbemClientTransportImpl: Sized {
    fn ConnectServer();
}
impl ::windows::core::RuntimeName for IWbemClientTransport {
    const NAME: &'static str = "Windows.Win32.System.Wmi.IWbemClientTransport";
}
impl IWbemClientTransportVtbl {
    pub const fn new<Impl: IWbemClientTransportImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWbemClientTransportVtbl {
        unsafe extern "system" fn ConnectServer<Impl: IWbemClientTransportImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, straddresstype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwbinaryaddresslength: u32, abbinaryaddress: *const u8, strnetworkresource: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, struser: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strlocale: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lsecurityflags: i32, strauthority: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pctx: ::windows::core::RawPtr, ppnamespace: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ConnectServer(
                &*(&straddresstype as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                dwbinaryaddresslength,
                abbinaryaddress,
                &*(&strnetworkresource as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&struser as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&strpassword as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&strlocale as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                lsecurityflags,
                &*(&strauthority as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pctx as *const <IWbemContext as ::windows::core::Abi>::Abi as *const <IWbemContext as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppnamespace),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWbemClientTransport>, base.5, ConnectServer::<Impl, OFFSET>)
    }
}
pub trait IWbemConfigureRefresherImpl: Sized {
    fn AddObjectByPath();
    fn AddObjectByTemplate();
    fn AddRefresher();
    fn Remove();
    fn AddEnum();
}
impl ::windows::core::RuntimeName for IWbemConfigureRefresher {
    const NAME: &'static str = "Windows.Win32.System.Wmi.IWbemConfigureRefresher";
}
impl IWbemConfigureRefresherVtbl {
    pub const fn new<Impl: IWbemConfigureRefresherImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWbemConfigureRefresherVtbl {
        unsafe extern "system" fn AddObjectByPath<Impl: IWbemConfigureRefresherImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnamespace: ::windows::core::RawPtr, wszpath: super::super::Foundation::PWSTR, lflags: i32, pcontext: ::windows::core::RawPtr, pprefreshable: *mut ::windows::core::RawPtr, plid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddObjectByPath(
                &*(&pnamespace as *const <IWbemServices as ::windows::core::Abi>::Abi as *const <IWbemServices as ::windows::core::DefaultType>::DefaultType),
                &*(&wszpath as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                lflags,
                &*(&pcontext as *const <IWbemContext as ::windows::core::Abi>::Abi as *const <IWbemContext as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&pprefreshable),
                plid,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddObjectByTemplate<Impl: IWbemConfigureRefresherImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnamespace: ::windows::core::RawPtr, ptemplate: ::windows::core::RawPtr, lflags: i32, pcontext: ::windows::core::RawPtr, pprefreshable: *mut ::windows::core::RawPtr, plid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddObjectByTemplate(
                &*(&pnamespace as *const <IWbemServices as ::windows::core::Abi>::Abi as *const <IWbemServices as ::windows::core::DefaultType>::DefaultType),
                &*(&ptemplate as *const <IWbemClassObject as ::windows::core::Abi>::Abi as *const <IWbemClassObject as ::windows::core::DefaultType>::DefaultType),
                lflags,
                &*(&pcontext as *const <IWbemContext as ::windows::core::Abi>::Abi as *const <IWbemContext as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&pprefreshable),
                plid,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddRefresher<Impl: IWbemConfigureRefresherImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prefresher: ::windows::core::RawPtr, lflags: i32, plid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddRefresher(&*(&prefresher as *const <IWbemRefresher as ::windows::core::Abi>::Abi as *const <IWbemRefresher as ::windows::core::DefaultType>::DefaultType), lflags, plid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Remove<Impl: IWbemConfigureRefresherImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lid: i32, lflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Remove(lid, lflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddEnum<Impl: IWbemConfigureRefresherImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnamespace: ::windows::core::RawPtr, wszclassname: super::super::Foundation::PWSTR, lflags: i32, pcontext: ::windows::core::RawPtr, ppenum: *mut ::windows::core::RawPtr, plid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddEnum(
                &*(&pnamespace as *const <IWbemServices as ::windows::core::Abi>::Abi as *const <IWbemServices as ::windows::core::DefaultType>::DefaultType),
                &*(&wszclassname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                lflags,
                &*(&pcontext as *const <IWbemContext as ::windows::core::Abi>::Abi as *const <IWbemContext as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppenum),
                plid,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWbemConfigureRefresher>, base.5, AddObjectByPath::<Impl, OFFSET>, AddObjectByTemplate::<Impl, OFFSET>, AddRefresher::<Impl, OFFSET>, Remove::<Impl, OFFSET>, AddEnum::<Impl, OFFSET>)
    }
}
pub trait IWbemConnectorLoginImpl: Sized {
    fn ConnectorLogin();
}
impl ::windows::core::RuntimeName for IWbemConnectorLogin {
    const NAME: &'static str = "Windows.Win32.System.Wmi.IWbemConnectorLogin";
}
impl IWbemConnectorLoginVtbl {
    pub const fn new<Impl: IWbemConnectorLoginImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWbemConnectorLoginVtbl {
        unsafe extern "system" fn ConnectorLogin<Impl: IWbemConnectorLoginImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wsznetworkresource: super::super::Foundation::PWSTR, wszpreferredlocale: super::super::Foundation::PWSTR, lflags: i32, pctx: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, pinterface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ConnectorLogin(
                &*(&wsznetworkresource as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&wszpreferredlocale as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                lflags,
                &*(&pctx as *const <IWbemContext as ::windows::core::Abi>::Abi as *const <IWbemContext as ::windows::core::DefaultType>::DefaultType),
                &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&pinterface),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWbemConnectorLogin>, base.5, ConnectorLogin::<Impl, OFFSET>)
    }
}
pub trait IWbemConstructClassObjectImpl: Sized {
    fn SetInheritanceChain();
    fn SetPropertyOrigin();
    fn SetMethodOrigin();
    fn SetServerNamespace();
}
impl ::windows::core::RuntimeName for IWbemConstructClassObject {
    const NAME: &'static str = "Windows.Win32.System.Wmi.IWbemConstructClassObject";
}
impl IWbemConstructClassObjectVtbl {
    pub const fn new<Impl: IWbemConstructClassObjectImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWbemConstructClassObjectVtbl {
        unsafe extern "system" fn SetInheritanceChain<Impl: IWbemConstructClassObjectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lnumantecedents: i32, awszantecedents: *const super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetInheritanceChain(lnumantecedents, &*(&awszantecedents as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPropertyOrigin<Impl: IWbemConstructClassObjectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wszpropertyname: super::super::Foundation::PWSTR, loriginindex: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetPropertyOrigin(&*(&wszpropertyname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), loriginindex) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMethodOrigin<Impl: IWbemConstructClassObjectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wszmethodname: super::super::Foundation::PWSTR, loriginindex: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetMethodOrigin(&*(&wszmethodname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), loriginindex) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetServerNamespace<Impl: IWbemConstructClassObjectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wszserver: super::super::Foundation::PWSTR, wsznamespace: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetServerNamespace(&*(&wszserver as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&wsznamespace as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWbemConstructClassObject>, base.5, SetInheritanceChain::<Impl, OFFSET>, SetPropertyOrigin::<Impl, OFFSET>, SetMethodOrigin::<Impl, OFFSET>, SetServerNamespace::<Impl, OFFSET>)
    }
}
pub trait IWbemContextImpl: Sized {
    fn Clone();
    fn GetNames();
    fn BeginEnumeration();
    fn Next();
    fn EndEnumeration();
    fn SetValue();
    fn GetValue();
    fn DeleteValue();
    fn DeleteAll();
}
impl ::windows::core::RuntimeName for IWbemContext {
    const NAME: &'static str = "Windows.Win32.System.Wmi.IWbemContext";
}
impl IWbemContextVtbl {
    pub const fn new<Impl: IWbemContextImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWbemContextVtbl {
        unsafe extern "system" fn Clone<Impl: IWbemContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppnewcopy: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&ppnewcopy)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNames<Impl: IWbemContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32, pnames: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetNames(lflags, ::core::mem::transmute_copy(&pnames)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginEnumeration<Impl: IWbemContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BeginEnumeration(lflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Impl: IWbemContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32, pstrname: *mut super::super::Foundation::BSTR, pvalue: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Next(lflags, ::core::mem::transmute_copy(&pstrname), ::core::mem::transmute_copy(&pvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndEnumeration<Impl: IWbemContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EndEnumeration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValue<Impl: IWbemContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wszname: super::super::Foundation::PWSTR, lflags: i32, pvalue: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetValue(&*(&wszname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), lflags, &*(&pvalue as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetValue<Impl: IWbemContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wszname: super::super::Foundation::PWSTR, lflags: i32, pvalue: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetValue(&*(&wszname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), lflags, ::core::mem::transmute_copy(&pvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteValue<Impl: IWbemContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wszname: super::super::Foundation::PWSTR, lflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DeleteValue(&*(&wszname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), lflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteAll<Impl: IWbemContextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DeleteAll() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWbemContext>, base.5, Clone::<Impl, OFFSET>, GetNames::<Impl, OFFSET>, BeginEnumeration::<Impl, OFFSET>, Next::<Impl, OFFSET>, EndEnumeration::<Impl, OFFSET>, SetValue::<Impl, OFFSET>, GetValue::<Impl, OFFSET>, DeleteValue::<Impl, OFFSET>, DeleteAll::<Impl, OFFSET>)
    }
}
pub trait IWbemDecoupledBasicEventProviderImpl: Sized + IWbemDecoupledRegistrarImpl {
    fn GetSink();
    fn GetService();
}
impl ::windows::core::RuntimeName for IWbemDecoupledBasicEventProvider {
    const NAME: &'static str = "Windows.Win32.System.Wmi.IWbemDecoupledBasicEventProvider";
}
impl IWbemDecoupledBasicEventProviderVtbl {
    pub const fn new<Impl: IWbemDecoupledBasicEventProviderImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWbemDecoupledBasicEventProviderVtbl {
        unsafe extern "system" fn GetSink<Impl: IWbemDecoupledBasicEventProviderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, a_flags: i32, a_context: ::windows::core::RawPtr, a_sink: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSink(a_flags, &*(&a_context as *const <IWbemContext as ::windows::core::Abi>::Abi as *const <IWbemContext as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&a_sink)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetService<Impl: IWbemDecoupledBasicEventProviderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, a_flags: i32, a_context: ::windows::core::RawPtr, a_service: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetService(a_flags, &*(&a_context as *const <IWbemContext as ::windows::core::Abi>::Abi as *const <IWbemContext as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&a_service)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWbemDecoupledBasicEventProvider>, base.5, GetSink::<Impl, OFFSET>, GetService::<Impl, OFFSET>)
    }
}
pub trait IWbemDecoupledRegistrarImpl: Sized {
    fn Register();
    fn UnRegister();
}
impl ::windows::core::RuntimeName for IWbemDecoupledRegistrar {
    const NAME: &'static str = "Windows.Win32.System.Wmi.IWbemDecoupledRegistrar";
}
impl IWbemDecoupledRegistrarVtbl {
    pub const fn new<Impl: IWbemDecoupledRegistrarImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWbemDecoupledRegistrarVtbl {
        unsafe extern "system" fn Register<Impl: IWbemDecoupledRegistrarImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, a_flags: i32, a_context: ::windows::core::RawPtr, a_user: super::super::Foundation::PWSTR, a_locale: super::super::Foundation::PWSTR, a_scope: super::super::Foundation::PWSTR, a_registration: super::super::Foundation::PWSTR, piunknown: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Register(
                a_flags,
                &*(&a_context as *const <IWbemContext as ::windows::core::Abi>::Abi as *const <IWbemContext as ::windows::core::DefaultType>::DefaultType),
                &*(&a_user as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&a_locale as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&a_scope as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&a_registration as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&piunknown as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnRegister<Impl: IWbemDecoupledRegistrarImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UnRegister() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWbemDecoupledRegistrar>, base.5, Register::<Impl, OFFSET>, UnRegister::<Impl, OFFSET>)
    }
}
pub trait IWbemEventConsumerProviderImpl: Sized {
    fn FindConsumer();
}
impl ::windows::core::RuntimeName for IWbemEventConsumerProvider {
    const NAME: &'static str = "Windows.Win32.System.Wmi.IWbemEventConsumerProvider";
}
impl IWbemEventConsumerProviderVtbl {
    pub const fn new<Impl: IWbemEventConsumerProviderImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWbemEventConsumerProviderVtbl {
        unsafe extern "system" fn FindConsumer<Impl: IWbemEventConsumerProviderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plogicalconsumer: ::windows::core::RawPtr, ppconsumer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FindConsumer(&*(&plogicalconsumer as *const <IWbemClassObject as ::windows::core::Abi>::Abi as *const <IWbemClassObject as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppconsumer)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWbemEventConsumerProvider>, base.5, FindConsumer::<Impl, OFFSET>)
    }
}
pub trait IWbemEventProviderImpl: Sized {
    fn ProvideEvents();
}
impl ::windows::core::RuntimeName for IWbemEventProvider {
    const NAME: &'static str = "Windows.Win32.System.Wmi.IWbemEventProvider";
}
impl IWbemEventProviderVtbl {
    pub const fn new<Impl: IWbemEventProviderImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWbemEventProviderVtbl {
        unsafe extern "system" fn ProvideEvents<Impl: IWbemEventProviderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psink: ::windows::core::RawPtr, lflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ProvideEvents(&*(&psink as *const <IWbemObjectSink as ::windows::core::Abi>::Abi as *const <IWbemObjectSink as ::windows::core::DefaultType>::DefaultType), lflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWbemEventProvider>, base.5, ProvideEvents::<Impl, OFFSET>)
    }
}
pub trait IWbemEventProviderQuerySinkImpl: Sized {
    fn NewQuery();
    fn CancelQuery();
}
impl ::windows::core::RuntimeName for IWbemEventProviderQuerySink {
    const NAME: &'static str = "Windows.Win32.System.Wmi.IWbemEventProviderQuerySink";
}
impl IWbemEventProviderQuerySinkVtbl {
    pub const fn new<Impl: IWbemEventProviderQuerySinkImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWbemEventProviderQuerySinkVtbl {
        unsafe extern "system" fn NewQuery<Impl: IWbemEventProviderQuerySinkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwid: u32, wszquerylanguage: *const u16, wszquery: *const u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NewQuery(dwid, wszquerylanguage, wszquery) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CancelQuery<Impl: IWbemEventProviderQuerySinkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CancelQuery(dwid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWbemEventProviderQuerySink>, base.5, NewQuery::<Impl, OFFSET>, CancelQuery::<Impl, OFFSET>)
    }
}
pub trait IWbemEventProviderSecurityImpl: Sized {
    fn AccessCheck();
}
impl ::windows::core::RuntimeName for IWbemEventProviderSecurity {
    const NAME: &'static str = "Windows.Win32.System.Wmi.IWbemEventProviderSecurity";
}
impl IWbemEventProviderSecurityVtbl {
    pub const fn new<Impl: IWbemEventProviderSecurityImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWbemEventProviderSecurityVtbl {
        unsafe extern "system" fn AccessCheck<Impl: IWbemEventProviderSecurityImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wszquerylanguage: *const u16, wszquery: *const u16, lsidlength: i32, psid: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AccessCheck(wszquerylanguage, wszquery, lsidlength, psid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWbemEventProviderSecurity>, base.5, AccessCheck::<Impl, OFFSET>)
    }
}
pub trait IWbemEventSinkImpl: Sized + IWbemObjectSinkImpl {
    fn SetSinkSecurity();
    fn IsActive();
    fn GetRestrictedSink();
    fn SetBatchingParameters();
}
impl ::windows::core::RuntimeName for IWbemEventSink {
    const NAME: &'static str = "Windows.Win32.System.Wmi.IWbemEventSink";
}
impl IWbemEventSinkVtbl {
    pub const fn new<Impl: IWbemEventSinkImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWbemEventSinkVtbl {
        unsafe extern "system" fn SetSinkSecurity<Impl: IWbemEventSinkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lsdlength: i32, psd: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetSinkSecurity(lsdlength, psd) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsActive<Impl: IWbemEventSinkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsActive() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRestrictedSink<Impl: IWbemEventSinkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lnumqueries: i32, awszqueries: *const super::super::Foundation::PWSTR, pcallback: *mut ::core::ffi::c_void, ppsink: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetRestrictedSink(lnumqueries, &*(&awszqueries as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&pcallback as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppsink)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBatchingParameters<Impl: IWbemEventSinkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32, dwmaxbuffersize: u32, dwmaxsendlatency: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetBatchingParameters(lflags, dwmaxbuffersize, dwmaxsendlatency) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWbemEventSink>, base.5, SetSinkSecurity::<Impl, OFFSET>, IsActive::<Impl, OFFSET>, GetRestrictedSink::<Impl, OFFSET>, SetBatchingParameters::<Impl, OFFSET>)
    }
}
pub trait IWbemHiPerfEnumImpl: Sized {
    fn AddObjects();
    fn RemoveObjects();
    fn GetObjects();
    fn RemoveAll();
}
impl ::windows::core::RuntimeName for IWbemHiPerfEnum {
    const NAME: &'static str = "Windows.Win32.System.Wmi.IWbemHiPerfEnum";
}
impl IWbemHiPerfEnumVtbl {
    pub const fn new<Impl: IWbemHiPerfEnumImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWbemHiPerfEnumVtbl {
        unsafe extern "system" fn AddObjects<Impl: IWbemHiPerfEnumImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32, unumobjects: u32, apids: *const i32, apobj: *const ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddObjects(lflags, unumobjects, apids, &*(&apobj as *const <IWbemObjectAccess as ::windows::core::Abi>::Abi as *const <IWbemObjectAccess as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveObjects<Impl: IWbemHiPerfEnumImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32, unumobjects: u32, apids: *const i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RemoveObjects(lflags, unumobjects, apids) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetObjects<Impl: IWbemHiPerfEnumImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32, unumobjects: u32, apobj: *mut ::windows::core::RawPtr, pureturned: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetObjects(lflags, unumobjects, ::core::mem::transmute_copy(&apobj), ::core::mem::transmute_copy(&pureturned)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAll<Impl: IWbemHiPerfEnumImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RemoveAll(lflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWbemHiPerfEnum>, base.5, AddObjects::<Impl, OFFSET>, RemoveObjects::<Impl, OFFSET>, GetObjects::<Impl, OFFSET>, RemoveAll::<Impl, OFFSET>)
    }
}
pub trait IWbemHiPerfProviderImpl: Sized {
    fn QueryInstances();
    fn CreateRefresher();
    fn CreateRefreshableObject();
    fn StopRefreshing();
    fn CreateRefreshableEnum();
    fn GetObjects();
}
impl ::windows::core::RuntimeName for IWbemHiPerfProvider {
    const NAME: &'static str = "Windows.Win32.System.Wmi.IWbemHiPerfProvider";
}
impl IWbemHiPerfProviderVtbl {
    pub const fn new<Impl: IWbemHiPerfProviderImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWbemHiPerfProviderVtbl {
        unsafe extern "system" fn QueryInstances<Impl: IWbemHiPerfProviderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnamespace: ::windows::core::RawPtr, wszclass: super::super::Foundation::PWSTR, lflags: i32, pctx: ::windows::core::RawPtr, psink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).QueryInstances(
                &*(&pnamespace as *const <IWbemServices as ::windows::core::Abi>::Abi as *const <IWbemServices as ::windows::core::DefaultType>::DefaultType),
                &*(&wszclass as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                lflags,
                &*(&pctx as *const <IWbemContext as ::windows::core::Abi>::Abi as *const <IWbemContext as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&psink),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRefresher<Impl: IWbemHiPerfProviderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnamespace: ::windows::core::RawPtr, lflags: i32, pprefresher: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateRefresher(&*(&pnamespace as *const <IWbemServices as ::windows::core::Abi>::Abi as *const <IWbemServices as ::windows::core::DefaultType>::DefaultType), lflags, ::core::mem::transmute_copy(&pprefresher)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRefreshableObject<Impl: IWbemHiPerfProviderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnamespace: ::windows::core::RawPtr, ptemplate: ::windows::core::RawPtr, prefresher: ::windows::core::RawPtr, lflags: i32, pcontext: ::windows::core::RawPtr, pprefreshable: *mut ::windows::core::RawPtr, plid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateRefreshableObject(
                &*(&pnamespace as *const <IWbemServices as ::windows::core::Abi>::Abi as *const <IWbemServices as ::windows::core::DefaultType>::DefaultType),
                &*(&ptemplate as *const <IWbemObjectAccess as ::windows::core::Abi>::Abi as *const <IWbemObjectAccess as ::windows::core::DefaultType>::DefaultType),
                &*(&prefresher as *const <IWbemRefresher as ::windows::core::Abi>::Abi as *const <IWbemRefresher as ::windows::core::DefaultType>::DefaultType),
                lflags,
                &*(&pcontext as *const <IWbemContext as ::windows::core::Abi>::Abi as *const <IWbemContext as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&pprefreshable),
                ::core::mem::transmute_copy(&plid),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StopRefreshing<Impl: IWbemHiPerfProviderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prefresher: ::windows::core::RawPtr, lid: i32, lflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StopRefreshing(&*(&prefresher as *const <IWbemRefresher as ::windows::core::Abi>::Abi as *const <IWbemRefresher as ::windows::core::DefaultType>::DefaultType), lid, lflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRefreshableEnum<Impl: IWbemHiPerfProviderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnamespace: ::windows::core::RawPtr, wszclass: super::super::Foundation::PWSTR, prefresher: ::windows::core::RawPtr, lflags: i32, pcontext: ::windows::core::RawPtr, phiperfenum: ::windows::core::RawPtr, plid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateRefreshableEnum(
                &*(&pnamespace as *const <IWbemServices as ::windows::core::Abi>::Abi as *const <IWbemServices as ::windows::core::DefaultType>::DefaultType),
                &*(&wszclass as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&prefresher as *const <IWbemRefresher as ::windows::core::Abi>::Abi as *const <IWbemRefresher as ::windows::core::DefaultType>::DefaultType),
                lflags,
                &*(&pcontext as *const <IWbemContext as ::windows::core::Abi>::Abi as *const <IWbemContext as ::windows::core::DefaultType>::DefaultType),
                &*(&phiperfenum as *const <IWbemHiPerfEnum as ::windows::core::Abi>::Abi as *const <IWbemHiPerfEnum as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&plid),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetObjects<Impl: IWbemHiPerfProviderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnamespace: ::windows::core::RawPtr, lnumobjects: i32, apobj: *mut ::windows::core::RawPtr, lflags: i32, pcontext: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetObjects(&*(&pnamespace as *const <IWbemServices as ::windows::core::Abi>::Abi as *const <IWbemServices as ::windows::core::DefaultType>::DefaultType), lnumobjects, ::core::mem::transmute_copy(&apobj), lflags, &*(&pcontext as *const <IWbemContext as ::windows::core::Abi>::Abi as *const <IWbemContext as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWbemHiPerfProvider>, base.5, QueryInstances::<Impl, OFFSET>, CreateRefresher::<Impl, OFFSET>, CreateRefreshableObject::<Impl, OFFSET>, StopRefreshing::<Impl, OFFSET>, CreateRefreshableEnum::<Impl, OFFSET>, GetObjects::<Impl, OFFSET>)
    }
}
pub trait IWbemLevel1LoginImpl: Sized {
    fn EstablishPosition();
    fn RequestChallenge();
    fn WBEMLogin();
    fn NTLMLogin();
}
impl ::windows::core::RuntimeName for IWbemLevel1Login {
    const NAME: &'static str = "Windows.Win32.System.Wmi.IWbemLevel1Login";
}
impl IWbemLevel1LoginVtbl {
    pub const fn new<Impl: IWbemLevel1LoginImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWbemLevel1LoginVtbl {
        unsafe extern "system" fn EstablishPosition<Impl: IWbemLevel1LoginImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wszlocalelist: super::super::Foundation::PWSTR, dwnumlocales: u32, reserved: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EstablishPosition(&*(&wszlocalelist as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), dwnumlocales, ::core::mem::transmute_copy(&reserved)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestChallenge<Impl: IWbemLevel1LoginImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wsznetworkresource: super::super::Foundation::PWSTR, wszuser: super::super::Foundation::PWSTR, nonce: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RequestChallenge(&*(&wsznetworkresource as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&wszuser as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&nonce)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WBEMLogin<Impl: IWbemLevel1LoginImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wszpreferredlocale: super::super::Foundation::PWSTR, accesstoken: *const u8, lflags: i32, pctx: ::windows::core::RawPtr, ppnamespace: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).WBEMLogin(&*(&wszpreferredlocale as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), accesstoken, lflags, &*(&pctx as *const <IWbemContext as ::windows::core::Abi>::Abi as *const <IWbemContext as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppnamespace)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NTLMLogin<Impl: IWbemLevel1LoginImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wsznetworkresource: super::super::Foundation::PWSTR, wszpreferredlocale: super::super::Foundation::PWSTR, lflags: i32, pctx: ::windows::core::RawPtr, ppnamespace: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NTLMLogin(
                &*(&wsznetworkresource as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&wszpreferredlocale as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                lflags,
                &*(&pctx as *const <IWbemContext as ::windows::core::Abi>::Abi as *const <IWbemContext as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppnamespace),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWbemLevel1Login>, base.5, EstablishPosition::<Impl, OFFSET>, RequestChallenge::<Impl, OFFSET>, WBEMLogin::<Impl, OFFSET>, NTLMLogin::<Impl, OFFSET>)
    }
}
pub trait IWbemLocatorImpl: Sized {
    fn ConnectServer();
}
impl ::windows::core::RuntimeName for IWbemLocator {
    const NAME: &'static str = "Windows.Win32.System.Wmi.IWbemLocator";
}
impl IWbemLocatorVtbl {
    pub const fn new<Impl: IWbemLocatorImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWbemLocatorVtbl {
        unsafe extern "system" fn ConnectServer<Impl: IWbemLocatorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strnetworkresource: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, struser: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strlocale: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lsecurityflags: i32, strauthority: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pctx: ::windows::core::RawPtr, ppnamespace: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ConnectServer(
                &*(&strnetworkresource as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&struser as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&strpassword as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&strlocale as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                lsecurityflags,
                &*(&strauthority as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pctx as *const <IWbemContext as ::windows::core::Abi>::Abi as *const <IWbemContext as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppnamespace),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWbemLocator>, base.5, ConnectServer::<Impl, OFFSET>)
    }
}
pub trait IWbemObjectAccessImpl: Sized + IWbemClassObjectImpl {
    fn GetPropertyHandle();
    fn WritePropertyValue();
    fn ReadPropertyValue();
    fn ReadDWORD();
    fn WriteDWORD();
    fn ReadQWORD();
    fn WriteQWORD();
    fn GetPropertyInfoByHandle();
    fn Lock();
    fn Unlock();
}
impl ::windows::core::RuntimeName for IWbemObjectAccess {
    const NAME: &'static str = "Windows.Win32.System.Wmi.IWbemObjectAccess";
}
impl IWbemObjectAccessVtbl {
    pub const fn new<Impl: IWbemObjectAccessImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWbemObjectAccessVtbl {
        unsafe extern "system" fn GetPropertyHandle<Impl: IWbemObjectAccessImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wszpropertyname: super::super::Foundation::PWSTR, ptype: *mut i32, plhandle: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPropertyHandle(&*(&wszpropertyname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ptype), ::core::mem::transmute_copy(&plhandle)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WritePropertyValue<Impl: IWbemObjectAccessImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lhandle: i32, lnumbytes: i32, adata: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).WritePropertyValue(lhandle, lnumbytes, adata) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadPropertyValue<Impl: IWbemObjectAccessImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lhandle: i32, lbuffersize: i32, plnumbytes: *mut i32, adata: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ReadPropertyValue(lhandle, lbuffersize, ::core::mem::transmute_copy(&plnumbytes), ::core::mem::transmute_copy(&adata)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadDWORD<Impl: IWbemObjectAccessImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lhandle: i32, pdw: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ReadDWORD(lhandle, ::core::mem::transmute_copy(&pdw)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WriteDWORD<Impl: IWbemObjectAccessImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lhandle: i32, dw: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).WriteDWORD(lhandle, dw) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadQWORD<Impl: IWbemObjectAccessImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lhandle: i32, pqw: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ReadQWORD(lhandle, ::core::mem::transmute_copy(&pqw)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WriteQWORD<Impl: IWbemObjectAccessImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lhandle: i32, pw: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).WriteQWORD(lhandle, pw) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPropertyInfoByHandle<Impl: IWbemObjectAccessImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lhandle: i32, pstrname: *mut super::super::Foundation::BSTR, ptype: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPropertyInfoByHandle(lhandle, ::core::mem::transmute_copy(&pstrname), ::core::mem::transmute_copy(&ptype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Lock<Impl: IWbemObjectAccessImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Lock(lflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Unlock<Impl: IWbemObjectAccessImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Unlock(lflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWbemObjectAccess>, base.5, GetPropertyHandle::<Impl, OFFSET>, WritePropertyValue::<Impl, OFFSET>, ReadPropertyValue::<Impl, OFFSET>, ReadDWORD::<Impl, OFFSET>, WriteDWORD::<Impl, OFFSET>, ReadQWORD::<Impl, OFFSET>, WriteQWORD::<Impl, OFFSET>, GetPropertyInfoByHandle::<Impl, OFFSET>, Lock::<Impl, OFFSET>, Unlock::<Impl, OFFSET>)
    }
}
pub trait IWbemObjectSinkImpl: Sized {
    fn Indicate();
    fn SetStatus();
}
impl ::windows::core::RuntimeName for IWbemObjectSink {
    const NAME: &'static str = "Windows.Win32.System.Wmi.IWbemObjectSink";
}
impl IWbemObjectSinkVtbl {
    pub const fn new<Impl: IWbemObjectSinkImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWbemObjectSinkVtbl {
        unsafe extern "system" fn Indicate<Impl: IWbemObjectSinkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lobjectcount: i32, apobjarray: *const ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Indicate(lobjectcount, &*(&apobjarray as *const <IWbemClassObject as ::windows::core::Abi>::Abi as *const <IWbemClassObject as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStatus<Impl: IWbemObjectSinkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32, hresult: ::windows::core::HRESULT, strparam: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pobjparam: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetStatus(lflags, hresult, &*(&strparam as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&pobjparam as *const <IWbemClassObject as ::windows::core::Abi>::Abi as *const <IWbemClassObject as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWbemObjectSink>, base.5, Indicate::<Impl, OFFSET>, SetStatus::<Impl, OFFSET>)
    }
}
pub trait IWbemObjectSinkExImpl: Sized + IWbemObjectSinkImpl {
    fn WriteMessage();
    fn WriteError();
    fn PromptUser();
    fn WriteProgress();
    fn WriteStreamParameter();
}
impl ::windows::core::RuntimeName for IWbemObjectSinkEx {
    const NAME: &'static str = "Windows.Win32.System.Wmi.IWbemObjectSinkEx";
}
impl IWbemObjectSinkExVtbl {
    pub const fn new<Impl: IWbemObjectSinkExImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWbemObjectSinkExVtbl {
        unsafe extern "system" fn WriteMessage<Impl: IWbemObjectSinkExImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uchannel: u32, strmessage: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).WriteMessage(uchannel, &*(&strmessage as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WriteError<Impl: IWbemObjectSinkExImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pobjerror: ::windows::core::RawPtr, pureturned: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).WriteError(&*(&pobjerror as *const <IWbemClassObject as ::windows::core::Abi>::Abi as *const <IWbemClassObject as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pureturned)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PromptUser<Impl: IWbemObjectSinkExImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strmessage: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, uprompttype: u8, pureturned: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PromptUser(&*(&strmessage as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), uprompttype, ::core::mem::transmute_copy(&pureturned)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WriteProgress<Impl: IWbemObjectSinkExImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, stractivity: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strcurrentoperation: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strstatusdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, upercentcomplete: u32, usecondsremaining: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).WriteProgress(
                &*(&stractivity as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&strcurrentoperation as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&strstatusdescription as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                upercentcomplete,
                usecondsremaining,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WriteStreamParameter<Impl: IWbemObjectSinkExImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, vtvalue: *const super::Com::VARIANT, ultype: u32, ulflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).WriteStreamParameter(&*(&strname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&vtvalue as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), ultype, ulflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWbemObjectSinkEx>, base.5, WriteMessage::<Impl, OFFSET>, WriteError::<Impl, OFFSET>, PromptUser::<Impl, OFFSET>, WriteProgress::<Impl, OFFSET>, WriteStreamParameter::<Impl, OFFSET>)
    }
}
pub trait IWbemObjectTextSrcImpl: Sized {
    fn GetText();
    fn CreateFromText();
}
impl ::windows::core::RuntimeName for IWbemObjectTextSrc {
    const NAME: &'static str = "Windows.Win32.System.Wmi.IWbemObjectTextSrc";
}
impl IWbemObjectTextSrcVtbl {
    pub const fn new<Impl: IWbemObjectTextSrcImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWbemObjectTextSrcVtbl {
        unsafe extern "system" fn GetText<Impl: IWbemObjectTextSrcImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32, pobj: ::windows::core::RawPtr, uobjtextformat: u32, pctx: ::windows::core::RawPtr, strtext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetText(lflags, &*(&pobj as *const <IWbemClassObject as ::windows::core::Abi>::Abi as *const <IWbemClassObject as ::windows::core::DefaultType>::DefaultType), uobjtextformat, &*(&pctx as *const <IWbemContext as ::windows::core::Abi>::Abi as *const <IWbemContext as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&strtext)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromText<Impl: IWbemObjectTextSrcImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32, strtext: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, uobjtextformat: u32, pctx: ::windows::core::RawPtr, pnewobj: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateFromText(lflags, &*(&strtext as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), uobjtextformat, &*(&pctx as *const <IWbemContext as ::windows::core::Abi>::Abi as *const <IWbemContext as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pnewobj)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWbemObjectTextSrc>, base.5, GetText::<Impl, OFFSET>, CreateFromText::<Impl, OFFSET>)
    }
}
pub trait IWbemPathImpl: Sized {
    fn SetText();
    fn GetText();
    fn GetInfo();
    fn SetServer();
    fn GetServer();
    fn GetNamespaceCount();
    fn SetNamespaceAt();
    fn GetNamespaceAt();
    fn RemoveNamespaceAt();
    fn RemoveAllNamespaces();
    fn GetScopeCount();
    fn SetScope();
    fn SetScopeFromText();
    fn GetScope();
    fn GetScopeAsText();
    fn RemoveScope();
    fn RemoveAllScopes();
    fn SetClassName();
    fn GetClassName();
    fn GetKeyList();
    fn CreateClassPart();
    fn DeleteClassPart();
    fn IsRelative();
    fn IsRelativeOrChild();
    fn IsLocal();
    fn IsSameClassName();
}
impl ::windows::core::RuntimeName for IWbemPath {
    const NAME: &'static str = "Windows.Win32.System.Wmi.IWbemPath";
}
impl IWbemPathVtbl {
    pub const fn new<Impl: IWbemPathImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWbemPathVtbl {
        unsafe extern "system" fn SetText<Impl: IWbemPathImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, umode: u32, pszpath: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetText(umode, &*(&pszpath as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetText<Impl: IWbemPathImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32, pubufflength: *mut u32, psztext: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetText(lflags, pubufflength, ::core::mem::transmute_copy(&psztext)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInfo<Impl: IWbemPathImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, urequestedinfo: u32, puresponse: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetInfo(urequestedinfo, ::core::mem::transmute_copy(&puresponse)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetServer<Impl: IWbemPathImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetServer(&*(&name as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetServer<Impl: IWbemPathImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punamebuflength: *mut u32, pname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetServer(punamebuflength, ::core::mem::transmute_copy(&pname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNamespaceCount<Impl: IWbemPathImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pucount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetNamespaceCount(::core::mem::transmute_copy(&pucount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNamespaceAt<Impl: IWbemPathImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uindex: u32, pszname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetNamespaceAt(uindex, &*(&pszname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNamespaceAt<Impl: IWbemPathImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uindex: u32, punamebuflength: *mut u32, pname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetNamespaceAt(uindex, punamebuflength, ::core::mem::transmute_copy(&pname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveNamespaceAt<Impl: IWbemPathImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RemoveNamespaceAt(uindex) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAllNamespaces<Impl: IWbemPathImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RemoveAllNamespaces() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetScopeCount<Impl: IWbemPathImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pucount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetScopeCount(::core::mem::transmute_copy(&pucount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetScope<Impl: IWbemPathImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uindex: u32, pszclass: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetScope(uindex, &*(&pszclass as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetScopeFromText<Impl: IWbemPathImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uindex: u32, psztext: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetScopeFromText(uindex, &*(&psztext as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetScope<Impl: IWbemPathImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uindex: u32, puclassnamebufsize: *mut u32, pszclass: super::super::Foundation::PWSTR, pkeylist: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetScope(uindex, puclassnamebufsize, ::core::mem::transmute_copy(&pszclass), ::core::mem::transmute_copy(&pkeylist)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetScopeAsText<Impl: IWbemPathImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uindex: u32, putextbufsize: *mut u32, psztext: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetScopeAsText(uindex, putextbufsize, ::core::mem::transmute_copy(&psztext)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveScope<Impl: IWbemPathImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RemoveScope(uindex) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAllScopes<Impl: IWbemPathImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RemoveAllScopes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClassName<Impl: IWbemPathImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetClassName(&*(&name as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetClassName<Impl: IWbemPathImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pubufflength: *mut u32, pszname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetClassName(pubufflength, ::core::mem::transmute_copy(&pszname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetKeyList<Impl: IWbemPathImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pout: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetKeyList(::core::mem::transmute_copy(&pout)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateClassPart<Impl: IWbemPathImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32, name: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateClassPart(lflags, &*(&name as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteClassPart<Impl: IWbemPathImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DeleteClassPart(lflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsRelative<Impl: IWbemPathImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wszmachine: super::super::Foundation::PWSTR, wsznamespace: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsRelative(&*(&wszmachine as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&wsznamespace as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsRelativeOrChild<Impl: IWbemPathImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wszmachine: super::super::Foundation::PWSTR, wsznamespace: super::super::Foundation::PWSTR, lflags: i32) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsRelativeOrChild(&*(&wszmachine as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&wsznamespace as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), lflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsLocal<Impl: IWbemPathImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wszmachine: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsLocal(&*(&wszmachine as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsSameClassName<Impl: IWbemPathImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wszclass: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsSameClassName(&*(&wszclass as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IWbemPath>,
            base.5,
            SetText::<Impl, OFFSET>,
            GetText::<Impl, OFFSET>,
            GetInfo::<Impl, OFFSET>,
            SetServer::<Impl, OFFSET>,
            GetServer::<Impl, OFFSET>,
            GetNamespaceCount::<Impl, OFFSET>,
            SetNamespaceAt::<Impl, OFFSET>,
            GetNamespaceAt::<Impl, OFFSET>,
            RemoveNamespaceAt::<Impl, OFFSET>,
            RemoveAllNamespaces::<Impl, OFFSET>,
            GetScopeCount::<Impl, OFFSET>,
            SetScope::<Impl, OFFSET>,
            SetScopeFromText::<Impl, OFFSET>,
            GetScope::<Impl, OFFSET>,
            GetScopeAsText::<Impl, OFFSET>,
            RemoveScope::<Impl, OFFSET>,
            RemoveAllScopes::<Impl, OFFSET>,
            SetClassName::<Impl, OFFSET>,
            GetClassName::<Impl, OFFSET>,
            GetKeyList::<Impl, OFFSET>,
            CreateClassPart::<Impl, OFFSET>,
            DeleteClassPart::<Impl, OFFSET>,
            IsRelative::<Impl, OFFSET>,
            IsRelativeOrChild::<Impl, OFFSET>,
            IsLocal::<Impl, OFFSET>,
            IsSameClassName::<Impl, OFFSET>,
        )
    }
}
pub trait IWbemPathKeyListImpl: Sized {
    fn GetCount();
    fn SetKey();
    fn SetKey2();
    fn GetKey();
    fn GetKey2();
    fn RemoveKey();
    fn RemoveAllKeys();
    fn MakeSingleton();
    fn GetInfo();
    fn GetText();
}
impl ::windows::core::RuntimeName for IWbemPathKeyList {
    const NAME: &'static str = "Windows.Win32.System.Wmi.IWbemPathKeyList";
}
impl IWbemPathKeyListVtbl {
    pub const fn new<Impl: IWbemPathKeyListImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWbemPathKeyListVtbl {
        unsafe extern "system" fn GetCount<Impl: IWbemPathKeyListImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pukeycount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCount(::core::mem::transmute_copy(&pukeycount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetKey<Impl: IWbemPathKeyListImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wszname: super::super::Foundation::PWSTR, uflags: u32, ucimtype: u32, pkeyval: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetKey(&*(&wszname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), uflags, ucimtype, &*(&pkeyval as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetKey2<Impl: IWbemPathKeyListImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wszname: super::super::Foundation::PWSTR, uflags: u32, ucimtype: u32, pkeyval: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetKey2(&*(&wszname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), uflags, ucimtype, &*(&pkeyval as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetKey<Impl: IWbemPathKeyListImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ukeyix: u32, uflags: u32, punamebufsize: *mut u32, pszkeyname: super::super::Foundation::PWSTR, pukeyvalbufsize: *mut u32, pkeyval: *mut ::core::ffi::c_void, puapparentcimtype: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetKey(ukeyix, uflags, punamebufsize, ::core::mem::transmute_copy(&pszkeyname), pukeyvalbufsize, &*(&pkeyval as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&puapparentcimtype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetKey2<Impl: IWbemPathKeyListImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ukeyix: u32, uflags: u32, punamebufsize: *mut u32, pszkeyname: super::super::Foundation::PWSTR, pkeyvalue: *mut super::Com::VARIANT, puapparentcimtype: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetKey2(ukeyix, uflags, punamebufsize, ::core::mem::transmute_copy(&pszkeyname), &*(&pkeyvalue as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&puapparentcimtype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveKey<Impl: IWbemPathKeyListImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wszname: super::super::Foundation::PWSTR, uflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RemoveKey(&*(&wszname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), uflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAllKeys<Impl: IWbemPathKeyListImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RemoveAllKeys(uflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MakeSingleton<Impl: IWbemPathKeyListImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bset: u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MakeSingleton(bset) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInfo<Impl: IWbemPathKeyListImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, urequestedinfo: u32, puresponse: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetInfo(urequestedinfo, ::core::mem::transmute_copy(&puresponse)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetText<Impl: IWbemPathKeyListImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32, pubufflength: *mut u32, psztext: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetText(lflags, pubufflength, ::core::mem::transmute_copy(&psztext)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWbemPathKeyList>, base.5, GetCount::<Impl, OFFSET>, SetKey::<Impl, OFFSET>, SetKey2::<Impl, OFFSET>, GetKey::<Impl, OFFSET>, GetKey2::<Impl, OFFSET>, RemoveKey::<Impl, OFFSET>, RemoveAllKeys::<Impl, OFFSET>, MakeSingleton::<Impl, OFFSET>, GetInfo::<Impl, OFFSET>, GetText::<Impl, OFFSET>)
    }
}
pub trait IWbemPropertyProviderImpl: Sized {
    fn GetProperty();
    fn PutProperty();
}
impl ::windows::core::RuntimeName for IWbemPropertyProvider {
    const NAME: &'static str = "Windows.Win32.System.Wmi.IWbemPropertyProvider";
}
impl IWbemPropertyProviderVtbl {
    pub const fn new<Impl: IWbemPropertyProviderImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWbemPropertyProviderVtbl {
        unsafe extern "system" fn GetProperty<Impl: IWbemPropertyProviderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32, strlocale: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strclassmapping: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strinstmapping: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strpropmapping: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvvalue: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetProperty(
                lflags,
                &*(&strlocale as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&strclassmapping as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&strinstmapping as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&strpropmapping as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&pvvalue),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PutProperty<Impl: IWbemPropertyProviderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32, strlocale: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strclassmapping: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strinstmapping: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strpropmapping: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvvalue: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PutProperty(
                lflags,
                &*(&strlocale as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&strclassmapping as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&strinstmapping as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&strpropmapping as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pvvalue as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWbemPropertyProvider>, base.5, GetProperty::<Impl, OFFSET>, PutProperty::<Impl, OFFSET>)
    }
}
pub trait IWbemProviderIdentityImpl: Sized {
    fn SetRegistrationObject();
}
impl ::windows::core::RuntimeName for IWbemProviderIdentity {
    const NAME: &'static str = "Windows.Win32.System.Wmi.IWbemProviderIdentity";
}
impl IWbemProviderIdentityVtbl {
    pub const fn new<Impl: IWbemProviderIdentityImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWbemProviderIdentityVtbl {
        unsafe extern "system" fn SetRegistrationObject<Impl: IWbemProviderIdentityImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32, pprovreg: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetRegistrationObject(lflags, &*(&pprovreg as *const <IWbemClassObject as ::windows::core::Abi>::Abi as *const <IWbemClassObject as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWbemProviderIdentity>, base.5, SetRegistrationObject::<Impl, OFFSET>)
    }
}
pub trait IWbemProviderInitImpl: Sized {
    fn Initialize();
}
impl ::windows::core::RuntimeName for IWbemProviderInit {
    const NAME: &'static str = "Windows.Win32.System.Wmi.IWbemProviderInit";
}
impl IWbemProviderInitVtbl {
    pub const fn new<Impl: IWbemProviderInitImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWbemProviderInitVtbl {
        unsafe extern "system" fn Initialize<Impl: IWbemProviderInitImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wszuser: super::super::Foundation::PWSTR, lflags: i32, wsznamespace: super::super::Foundation::PWSTR, wszlocale: super::super::Foundation::PWSTR, pnamespace: ::windows::core::RawPtr, pctx: ::windows::core::RawPtr, pinitsink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Initialize(
                &*(&wszuser as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                lflags,
                &*(&wsznamespace as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&wszlocale as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pnamespace as *const <IWbemServices as ::windows::core::Abi>::Abi as *const <IWbemServices as ::windows::core::DefaultType>::DefaultType),
                &*(&pctx as *const <IWbemContext as ::windows::core::Abi>::Abi as *const <IWbemContext as ::windows::core::DefaultType>::DefaultType),
                &*(&pinitsink as *const <IWbemProviderInitSink as ::windows::core::Abi>::Abi as *const <IWbemProviderInitSink as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWbemProviderInit>, base.5, Initialize::<Impl, OFFSET>)
    }
}
pub trait IWbemProviderInitSinkImpl: Sized {
    fn SetStatus();
}
impl ::windows::core::RuntimeName for IWbemProviderInitSink {
    const NAME: &'static str = "Windows.Win32.System.Wmi.IWbemProviderInitSink";
}
impl IWbemProviderInitSinkVtbl {
    pub const fn new<Impl: IWbemProviderInitSinkImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWbemProviderInitSinkVtbl {
        unsafe extern "system" fn SetStatus<Impl: IWbemProviderInitSinkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lstatus: i32, lflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetStatus(lstatus, lflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWbemProviderInitSink>, base.5, SetStatus::<Impl, OFFSET>)
    }
}
pub trait IWbemQualifierSetImpl: Sized {
    fn Get();
    fn Put();
    fn Delete();
    fn GetNames();
    fn BeginEnumeration();
    fn Next();
    fn EndEnumeration();
}
impl ::windows::core::RuntimeName for IWbemQualifierSet {
    const NAME: &'static str = "Windows.Win32.System.Wmi.IWbemQualifierSet";
}
impl IWbemQualifierSetVtbl {
    pub const fn new<Impl: IWbemQualifierSetImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWbemQualifierSetVtbl {
        unsafe extern "system" fn Get<Impl: IWbemQualifierSetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wszname: super::super::Foundation::PWSTR, lflags: i32, pval: *mut super::Com::VARIANT, plflavor: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Get(&*(&wszname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), lflags, &*(&pval as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), plflavor) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Put<Impl: IWbemQualifierSetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wszname: super::super::Foundation::PWSTR, pval: *const super::Com::VARIANT, lflavor: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Put(&*(&wszname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&pval as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), lflavor) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Impl: IWbemQualifierSetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wszname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Delete(&*(&wszname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNames<Impl: IWbemQualifierSetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32, pnames: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetNames(lflags, ::core::mem::transmute_copy(&pnames)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginEnumeration<Impl: IWbemQualifierSetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BeginEnumeration(lflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Impl: IWbemQualifierSetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32, pstrname: *mut super::super::Foundation::BSTR, pval: *mut super::Com::VARIANT, plflavor: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Next(lflags, &*(&pstrname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&pval as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), plflavor) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndEnumeration<Impl: IWbemQualifierSetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EndEnumeration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWbemQualifierSet>, base.5, Get::<Impl, OFFSET>, Put::<Impl, OFFSET>, Delete::<Impl, OFFSET>, GetNames::<Impl, OFFSET>, BeginEnumeration::<Impl, OFFSET>, Next::<Impl, OFFSET>, EndEnumeration::<Impl, OFFSET>)
    }
}
pub trait IWbemQueryImpl: Sized {
    fn Empty();
    fn SetLanguageFeatures();
    fn TestLanguageFeatures();
    fn Parse();
    fn GetAnalysis();
    fn FreeMemory();
    fn GetQueryInfo();
}
impl ::windows::core::RuntimeName for IWbemQuery {
    const NAME: &'static str = "Windows.Win32.System.Wmi.IWbemQuery";
}
impl IWbemQueryVtbl {
    pub const fn new<Impl: IWbemQueryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWbemQueryVtbl {
        unsafe extern "system" fn Empty<Impl: IWbemQueryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Empty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLanguageFeatures<Impl: IWbemQueryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uflags: u32, uarraysize: u32, pufeatures: *const u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetLanguageFeatures(uflags, uarraysize, pufeatures) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TestLanguageFeatures<Impl: IWbemQueryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uflags: u32, uarraysize: *mut u32, pufeatures: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TestLanguageFeatures(uflags, uarraysize, ::core::mem::transmute_copy(&pufeatures)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Parse<Impl: IWbemQueryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszlang: super::super::Foundation::PWSTR, pszquery: super::super::Foundation::PWSTR, uflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Parse(&*(&pszlang as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&pszquery as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), uflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAnalysis<Impl: IWbemQueryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uanalysistype: u32, uflags: u32, panalysis: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAnalysis(uanalysistype, uflags, ::core::mem::transmute_copy(&panalysis)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FreeMemory<Impl: IWbemQueryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmem: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FreeMemory(&*(&pmem as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetQueryInfo<Impl: IWbemQueryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uanalysistype: u32, uinfoid: u32, ubufsize: u32, pdestbuf: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetQueryInfo(uanalysistype, uinfoid, ubufsize, ::core::mem::transmute_copy(&pdestbuf)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWbemQuery>, base.5, Empty::<Impl, OFFSET>, SetLanguageFeatures::<Impl, OFFSET>, TestLanguageFeatures::<Impl, OFFSET>, Parse::<Impl, OFFSET>, GetAnalysis::<Impl, OFFSET>, FreeMemory::<Impl, OFFSET>, GetQueryInfo::<Impl, OFFSET>)
    }
}
pub trait IWbemRefresherImpl: Sized {
    fn Refresh();
}
impl ::windows::core::RuntimeName for IWbemRefresher {
    const NAME: &'static str = "Windows.Win32.System.Wmi.IWbemRefresher";
}
impl IWbemRefresherVtbl {
    pub const fn new<Impl: IWbemRefresherImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWbemRefresherVtbl {
        unsafe extern "system" fn Refresh<Impl: IWbemRefresherImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Refresh(lflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWbemRefresher>, base.5, Refresh::<Impl, OFFSET>)
    }
}
pub trait IWbemServicesImpl: Sized {
    fn OpenNamespace();
    fn CancelAsyncCall();
    fn QueryObjectSink();
    fn GetObject();
    fn GetObjectAsync();
    fn PutClass();
    fn PutClassAsync();
    fn DeleteClass();
    fn DeleteClassAsync();
    fn CreateClassEnum();
    fn CreateClassEnumAsync();
    fn PutInstance();
    fn PutInstanceAsync();
    fn DeleteInstance();
    fn DeleteInstanceAsync();
    fn CreateInstanceEnum();
    fn CreateInstanceEnumAsync();
    fn ExecQuery();
    fn ExecQueryAsync();
    fn ExecNotificationQuery();
    fn ExecNotificationQueryAsync();
    fn ExecMethod();
    fn ExecMethodAsync();
}
impl ::windows::core::RuntimeName for IWbemServices {
    const NAME: &'static str = "Windows.Win32.System.Wmi.IWbemServices";
}
impl IWbemServicesVtbl {
    pub const fn new<Impl: IWbemServicesImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWbemServicesVtbl {
        unsafe extern "system" fn OpenNamespace<Impl: IWbemServicesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strnamespace: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lflags: i32, pctx: ::windows::core::RawPtr, ppworkingnamespace: *mut ::windows::core::RawPtr, ppresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OpenNamespace(
                &*(&strnamespace as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                lflags,
                &*(&pctx as *const <IWbemContext as ::windows::core::Abi>::Abi as *const <IWbemContext as ::windows::core::DefaultType>::DefaultType),
                &*(&ppworkingnamespace as *const <IWbemServices as ::windows::core::Abi>::Abi as *const <IWbemServices as ::windows::core::DefaultType>::DefaultType),
                &*(&ppresult as *const <IWbemCallResult as ::windows::core::Abi>::Abi as *const <IWbemCallResult as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CancelAsyncCall<Impl: IWbemServicesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CancelAsyncCall(&*(&psink as *const <IWbemObjectSink as ::windows::core::Abi>::Abi as *const <IWbemObjectSink as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryObjectSink<Impl: IWbemServicesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32, ppresponsehandler: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).QueryObjectSink(lflags, ::core::mem::transmute_copy(&ppresponsehandler)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetObject<Impl: IWbemServicesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strobjectpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lflags: i32, pctx: ::windows::core::RawPtr, ppobject: *mut ::windows::core::RawPtr, ppcallresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetObject(
                &*(&strobjectpath as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                lflags,
                &*(&pctx as *const <IWbemContext as ::windows::core::Abi>::Abi as *const <IWbemContext as ::windows::core::DefaultType>::DefaultType),
                &*(&ppobject as *const <IWbemClassObject as ::windows::core::Abi>::Abi as *const <IWbemClassObject as ::windows::core::DefaultType>::DefaultType),
                &*(&ppcallresult as *const <IWbemCallResult as ::windows::core::Abi>::Abi as *const <IWbemCallResult as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetObjectAsync<Impl: IWbemServicesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strobjectpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lflags: i32, pctx: ::windows::core::RawPtr, presponsehandler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetObjectAsync(&*(&strobjectpath as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), lflags, &*(&pctx as *const <IWbemContext as ::windows::core::Abi>::Abi as *const <IWbemContext as ::windows::core::DefaultType>::DefaultType), &*(&presponsehandler as *const <IWbemObjectSink as ::windows::core::Abi>::Abi as *const <IWbemObjectSink as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PutClass<Impl: IWbemServicesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pobject: ::windows::core::RawPtr, lflags: i32, pctx: ::windows::core::RawPtr, ppcallresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PutClass(&*(&pobject as *const <IWbemClassObject as ::windows::core::Abi>::Abi as *const <IWbemClassObject as ::windows::core::DefaultType>::DefaultType), lflags, &*(&pctx as *const <IWbemContext as ::windows::core::Abi>::Abi as *const <IWbemContext as ::windows::core::DefaultType>::DefaultType), &*(&ppcallresult as *const <IWbemCallResult as ::windows::core::Abi>::Abi as *const <IWbemCallResult as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PutClassAsync<Impl: IWbemServicesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pobject: ::windows::core::RawPtr, lflags: i32, pctx: ::windows::core::RawPtr, presponsehandler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PutClassAsync(&*(&pobject as *const <IWbemClassObject as ::windows::core::Abi>::Abi as *const <IWbemClassObject as ::windows::core::DefaultType>::DefaultType), lflags, &*(&pctx as *const <IWbemContext as ::windows::core::Abi>::Abi as *const <IWbemContext as ::windows::core::DefaultType>::DefaultType), &*(&presponsehandler as *const <IWbemObjectSink as ::windows::core::Abi>::Abi as *const <IWbemObjectSink as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteClass<Impl: IWbemServicesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lflags: i32, pctx: ::windows::core::RawPtr, ppcallresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DeleteClass(&*(&strclass as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), lflags, &*(&pctx as *const <IWbemContext as ::windows::core::Abi>::Abi as *const <IWbemContext as ::windows::core::DefaultType>::DefaultType), &*(&ppcallresult as *const <IWbemCallResult as ::windows::core::Abi>::Abi as *const <IWbemCallResult as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteClassAsync<Impl: IWbemServicesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lflags: i32, pctx: ::windows::core::RawPtr, presponsehandler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DeleteClassAsync(&*(&strclass as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), lflags, &*(&pctx as *const <IWbemContext as ::windows::core::Abi>::Abi as *const <IWbemContext as ::windows::core::DefaultType>::DefaultType), &*(&presponsehandler as *const <IWbemObjectSink as ::windows::core::Abi>::Abi as *const <IWbemObjectSink as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateClassEnum<Impl: IWbemServicesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strsuperclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lflags: i32, pctx: ::windows::core::RawPtr, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateClassEnum(&*(&strsuperclass as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), lflags, &*(&pctx as *const <IWbemContext as ::windows::core::Abi>::Abi as *const <IWbemContext as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateClassEnumAsync<Impl: IWbemServicesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strsuperclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lflags: i32, pctx: ::windows::core::RawPtr, presponsehandler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateClassEnumAsync(&*(&strsuperclass as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), lflags, &*(&pctx as *const <IWbemContext as ::windows::core::Abi>::Abi as *const <IWbemContext as ::windows::core::DefaultType>::DefaultType), &*(&presponsehandler as *const <IWbemObjectSink as ::windows::core::Abi>::Abi as *const <IWbemObjectSink as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PutInstance<Impl: IWbemServicesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinst: ::windows::core::RawPtr, lflags: i32, pctx: ::windows::core::RawPtr, ppcallresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PutInstance(&*(&pinst as *const <IWbemClassObject as ::windows::core::Abi>::Abi as *const <IWbemClassObject as ::windows::core::DefaultType>::DefaultType), lflags, &*(&pctx as *const <IWbemContext as ::windows::core::Abi>::Abi as *const <IWbemContext as ::windows::core::DefaultType>::DefaultType), &*(&ppcallresult as *const <IWbemCallResult as ::windows::core::Abi>::Abi as *const <IWbemCallResult as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PutInstanceAsync<Impl: IWbemServicesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinst: ::windows::core::RawPtr, lflags: i32, pctx: ::windows::core::RawPtr, presponsehandler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PutInstanceAsync(&*(&pinst as *const <IWbemClassObject as ::windows::core::Abi>::Abi as *const <IWbemClassObject as ::windows::core::DefaultType>::DefaultType), lflags, &*(&pctx as *const <IWbemContext as ::windows::core::Abi>::Abi as *const <IWbemContext as ::windows::core::DefaultType>::DefaultType), &*(&presponsehandler as *const <IWbemObjectSink as ::windows::core::Abi>::Abi as *const <IWbemObjectSink as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteInstance<Impl: IWbemServicesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strobjectpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lflags: i32, pctx: ::windows::core::RawPtr, ppcallresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DeleteInstance(&*(&strobjectpath as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), lflags, &*(&pctx as *const <IWbemContext as ::windows::core::Abi>::Abi as *const <IWbemContext as ::windows::core::DefaultType>::DefaultType), &*(&ppcallresult as *const <IWbemCallResult as ::windows::core::Abi>::Abi as *const <IWbemCallResult as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteInstanceAsync<Impl: IWbemServicesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strobjectpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lflags: i32, pctx: ::windows::core::RawPtr, presponsehandler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DeleteInstanceAsync(&*(&strobjectpath as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), lflags, &*(&pctx as *const <IWbemContext as ::windows::core::Abi>::Abi as *const <IWbemContext as ::windows::core::DefaultType>::DefaultType), &*(&presponsehandler as *const <IWbemObjectSink as ::windows::core::Abi>::Abi as *const <IWbemObjectSink as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateInstanceEnum<Impl: IWbemServicesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strfilter: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lflags: i32, pctx: ::windows::core::RawPtr, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstanceEnum(&*(&strfilter as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), lflags, &*(&pctx as *const <IWbemContext as ::windows::core::Abi>::Abi as *const <IWbemContext as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateInstanceEnumAsync<Impl: IWbemServicesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strfilter: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lflags: i32, pctx: ::windows::core::RawPtr, presponsehandler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstanceEnumAsync(&*(&strfilter as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), lflags, &*(&pctx as *const <IWbemContext as ::windows::core::Abi>::Abi as *const <IWbemContext as ::windows::core::DefaultType>::DefaultType), &*(&presponsehandler as *const <IWbemObjectSink as ::windows::core::Abi>::Abi as *const <IWbemObjectSink as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExecQuery<Impl: IWbemServicesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strquerylanguage: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strquery: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lflags: i32, pctx: ::windows::core::RawPtr, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ExecQuery(
                &*(&strquerylanguage as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&strquery as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                lflags,
                &*(&pctx as *const <IWbemContext as ::windows::core::Abi>::Abi as *const <IWbemContext as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppenum),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExecQueryAsync<Impl: IWbemServicesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strquerylanguage: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strquery: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lflags: i32, pctx: ::windows::core::RawPtr, presponsehandler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ExecQueryAsync(
                &*(&strquerylanguage as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&strquery as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                lflags,
                &*(&pctx as *const <IWbemContext as ::windows::core::Abi>::Abi as *const <IWbemContext as ::windows::core::DefaultType>::DefaultType),
                &*(&presponsehandler as *const <IWbemObjectSink as ::windows::core::Abi>::Abi as *const <IWbemObjectSink as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExecNotificationQuery<Impl: IWbemServicesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strquerylanguage: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strquery: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lflags: i32, pctx: ::windows::core::RawPtr, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ExecNotificationQuery(
                &*(&strquerylanguage as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&strquery as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                lflags,
                &*(&pctx as *const <IWbemContext as ::windows::core::Abi>::Abi as *const <IWbemContext as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppenum),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExecNotificationQueryAsync<Impl: IWbemServicesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strquerylanguage: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strquery: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lflags: i32, pctx: ::windows::core::RawPtr, presponsehandler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ExecNotificationQueryAsync(
                &*(&strquerylanguage as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&strquery as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                lflags,
                &*(&pctx as *const <IWbemContext as ::windows::core::Abi>::Abi as *const <IWbemContext as ::windows::core::DefaultType>::DefaultType),
                &*(&presponsehandler as *const <IWbemObjectSink as ::windows::core::Abi>::Abi as *const <IWbemObjectSink as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExecMethod<Impl: IWbemServicesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strobjectpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strmethodname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lflags: i32, pctx: ::windows::core::RawPtr, pinparams: ::windows::core::RawPtr, ppoutparams: *mut ::windows::core::RawPtr, ppcallresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ExecMethod(
                &*(&strobjectpath as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&strmethodname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                lflags,
                &*(&pctx as *const <IWbemContext as ::windows::core::Abi>::Abi as *const <IWbemContext as ::windows::core::DefaultType>::DefaultType),
                &*(&pinparams as *const <IWbemClassObject as ::windows::core::Abi>::Abi as *const <IWbemClassObject as ::windows::core::DefaultType>::DefaultType),
                &*(&ppoutparams as *const <IWbemClassObject as ::windows::core::Abi>::Abi as *const <IWbemClassObject as ::windows::core::DefaultType>::DefaultType),
                &*(&ppcallresult as *const <IWbemCallResult as ::windows::core::Abi>::Abi as *const <IWbemCallResult as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExecMethodAsync<Impl: IWbemServicesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strobjectpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strmethodname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lflags: i32, pctx: ::windows::core::RawPtr, pinparams: ::windows::core::RawPtr, presponsehandler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ExecMethodAsync(
                &*(&strobjectpath as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&strmethodname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                lflags,
                &*(&pctx as *const <IWbemContext as ::windows::core::Abi>::Abi as *const <IWbemContext as ::windows::core::DefaultType>::DefaultType),
                &*(&pinparams as *const <IWbemClassObject as ::windows::core::Abi>::Abi as *const <IWbemClassObject as ::windows::core::DefaultType>::DefaultType),
                &*(&presponsehandler as *const <IWbemObjectSink as ::windows::core::Abi>::Abi as *const <IWbemObjectSink as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IWbemServices>,
            base.5,
            OpenNamespace::<Impl, OFFSET>,
            CancelAsyncCall::<Impl, OFFSET>,
            QueryObjectSink::<Impl, OFFSET>,
            GetObject::<Impl, OFFSET>,
            GetObjectAsync::<Impl, OFFSET>,
            PutClass::<Impl, OFFSET>,
            PutClassAsync::<Impl, OFFSET>,
            DeleteClass::<Impl, OFFSET>,
            DeleteClassAsync::<Impl, OFFSET>,
            CreateClassEnum::<Impl, OFFSET>,
            CreateClassEnumAsync::<Impl, OFFSET>,
            PutInstance::<Impl, OFFSET>,
            PutInstanceAsync::<Impl, OFFSET>,
            DeleteInstance::<Impl, OFFSET>,
            DeleteInstanceAsync::<Impl, OFFSET>,
            CreateInstanceEnum::<Impl, OFFSET>,
            CreateInstanceEnumAsync::<Impl, OFFSET>,
            ExecQuery::<Impl, OFFSET>,
            ExecQueryAsync::<Impl, OFFSET>,
            ExecNotificationQuery::<Impl, OFFSET>,
            ExecNotificationQueryAsync::<Impl, OFFSET>,
            ExecMethod::<Impl, OFFSET>,
            ExecMethodAsync::<Impl, OFFSET>,
        )
    }
}
pub trait IWbemShutdownImpl: Sized {
    fn Shutdown();
}
impl ::windows::core::RuntimeName for IWbemShutdown {
    const NAME: &'static str = "Windows.Win32.System.Wmi.IWbemShutdown";
}
impl IWbemShutdownVtbl {
    pub const fn new<Impl: IWbemShutdownImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWbemShutdownVtbl {
        unsafe extern "system" fn Shutdown<Impl: IWbemShutdownImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ureason: i32, umaxmilliseconds: u32, pctx: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Shutdown(ureason, umaxmilliseconds, &*(&pctx as *const <IWbemContext as ::windows::core::Abi>::Abi as *const <IWbemContext as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWbemShutdown>, base.5, Shutdown::<Impl, OFFSET>)
    }
}
pub trait IWbemStatusCodeTextImpl: Sized {
    fn GetErrorCodeText();
    fn GetFacilityCodeText();
}
impl ::windows::core::RuntimeName for IWbemStatusCodeText {
    const NAME: &'static str = "Windows.Win32.System.Wmi.IWbemStatusCodeText";
}
impl IWbemStatusCodeTextVtbl {
    pub const fn new<Impl: IWbemStatusCodeTextImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWbemStatusCodeTextVtbl {
        unsafe extern "system" fn GetErrorCodeText<Impl: IWbemStatusCodeTextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hres: ::windows::core::HRESULT, localeid: u32, lflags: i32, messagetext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetErrorCodeText(hres, localeid, lflags, ::core::mem::transmute_copy(&messagetext)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFacilityCodeText<Impl: IWbemStatusCodeTextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hres: ::windows::core::HRESULT, localeid: u32, lflags: i32, messagetext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetFacilityCodeText(hres, localeid, lflags, ::core::mem::transmute_copy(&messagetext)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWbemStatusCodeText>, base.5, GetErrorCodeText::<Impl, OFFSET>, GetFacilityCodeText::<Impl, OFFSET>)
    }
}
pub trait IWbemTransportImpl: Sized {
    fn Initialize();
}
impl ::windows::core::RuntimeName for IWbemTransport {
    const NAME: &'static str = "Windows.Win32.System.Wmi.IWbemTransport";
}
impl IWbemTransportVtbl {
    pub const fn new<Impl: IWbemTransportImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWbemTransportVtbl {
        unsafe extern "system" fn Initialize<Impl: IWbemTransportImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Initialize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWbemTransport>, base.5, Initialize::<Impl, OFFSET>)
    }
}
pub trait IWbemUnboundObjectSinkImpl: Sized {
    fn IndicateToConsumer();
}
impl ::windows::core::RuntimeName for IWbemUnboundObjectSink {
    const NAME: &'static str = "Windows.Win32.System.Wmi.IWbemUnboundObjectSink";
}
impl IWbemUnboundObjectSinkVtbl {
    pub const fn new<Impl: IWbemUnboundObjectSinkImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWbemUnboundObjectSinkVtbl {
        unsafe extern "system" fn IndicateToConsumer<Impl: IWbemUnboundObjectSinkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plogicalconsumer: ::windows::core::RawPtr, lnumobjects: i32, apobjects: *const ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IndicateToConsumer(&*(&plogicalconsumer as *const <IWbemClassObject as ::windows::core::Abi>::Abi as *const <IWbemClassObject as ::windows::core::DefaultType>::DefaultType), lnumobjects, &*(&apobjects as *const <IWbemClassObject as ::windows::core::Abi>::Abi as *const <IWbemClassObject as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWbemUnboundObjectSink>, base.5, IndicateToConsumer::<Impl, OFFSET>)
    }
}
pub trait IWbemUnsecuredApartmentImpl: Sized + IUnsecuredApartmentImpl {
    fn CreateSinkStub();
}
impl ::windows::core::RuntimeName for IWbemUnsecuredApartment {
    const NAME: &'static str = "Windows.Win32.System.Wmi.IWbemUnsecuredApartment";
}
impl IWbemUnsecuredApartmentVtbl {
    pub const fn new<Impl: IWbemUnsecuredApartmentImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWbemUnsecuredApartmentVtbl {
        unsafe extern "system" fn CreateSinkStub<Impl: IWbemUnsecuredApartmentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psink: ::windows::core::RawPtr, dwflags: u32, wszreserved: super::super::Foundation::PWSTR, ppstub: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateSinkStub(&*(&psink as *const <IWbemObjectSink as ::windows::core::Abi>::Abi as *const <IWbemObjectSink as ::windows::core::DefaultType>::DefaultType), dwflags, &*(&wszreserved as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppstub)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWbemUnsecuredApartment>, base.5, CreateSinkStub::<Impl, OFFSET>)
    }
}
