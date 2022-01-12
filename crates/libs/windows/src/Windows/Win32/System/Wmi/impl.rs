pub trait IEnumWbemClassObjectImpl: Sized {
    fn Reset();
    fn Next();
    fn NextAsync();
    fn Clone();
    fn Skip();
}
impl IEnumWbemClassObjectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumWbemClassObjectImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumWbemClassObjectVtbl {
        unsafe extern "system" fn Reset<Impl: IEnumWbemClassObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Next<Impl: IEnumWbemClassObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ltimeout: i32, ucount: u32, apobjects: *mut ::windows::core::RawPtr, pureturned: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn NextAsync<Impl: IEnumWbemClassObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ucount: u32, psink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clone<Impl: IEnumWbemClassObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Skip<Impl: IEnumWbemClassObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ltimeout: i32, ncount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn CompileFile();
    fn CompileBuffer();
    fn CreateBMOF();
}
#[cfg(feature = "Win32_Foundation")]
impl IMofCompilerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMofCompilerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMofCompilerVtbl {
        unsafe extern "system" fn CompileFile<Impl: IMofCompilerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: super::super::Foundation::PWSTR, serverandnamespace: super::super::Foundation::PWSTR, user: super::super::Foundation::PWSTR, authority: super::super::Foundation::PWSTR, password: super::super::Foundation::PWSTR, loptionflags: i32, lclassflags: i32, linstanceflags: i32, pinfo: *mut WBEM_COMPILE_STATUS_INFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CompileBuffer<Impl: IMofCompilerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buffsize: i32, pbuffer: *const u8, serverandnamespace: super::super::Foundation::PWSTR, user: super::super::Foundation::PWSTR, authority: super::super::Foundation::PWSTR, password: super::super::Foundation::PWSTR, loptionflags: i32, lclassflags: i32, linstanceflags: i32, pinfo: *mut WBEM_COMPILE_STATUS_INFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateBMOF<Impl: IMofCompilerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, textfilename: super::super::Foundation::PWSTR, bmoffilename: super::super::Foundation::PWSTR, serverandnamespace: super::super::Foundation::PWSTR, loptionflags: i32, lclassflags: i32, linstanceflags: i32, pinfo: *mut WBEM_COMPILE_STATUS_INFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISWbemDateTimeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISWbemDateTimeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISWbemDateTimeVtbl {
        unsafe extern "system" fn Value<Impl: ISWbemDateTimeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strvalue: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetValue<Impl: ISWbemDateTimeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strvalue: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Year<Impl: ISWbemDateTimeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iyear: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetYear<Impl: ISWbemDateTimeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iyear: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn YearSpecified<Impl: ISWbemDateTimeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, byearspecified: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetYearSpecified<Impl: ISWbemDateTimeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, byearspecified: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Month<Impl: ISWbemDateTimeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, imonth: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMonth<Impl: ISWbemDateTimeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, imonth: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MonthSpecified<Impl: ISWbemDateTimeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bmonthspecified: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMonthSpecified<Impl: ISWbemDateTimeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bmonthspecified: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Day<Impl: ISWbemDateTimeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iday: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDay<Impl: ISWbemDateTimeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iday: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DaySpecified<Impl: ISWbemDateTimeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bdayspecified: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDaySpecified<Impl: ISWbemDateTimeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bdayspecified: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Hours<Impl: ISWbemDateTimeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ihours: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetHours<Impl: ISWbemDateTimeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ihours: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn HoursSpecified<Impl: ISWbemDateTimeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bhoursspecified: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetHoursSpecified<Impl: ISWbemDateTimeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bhoursspecified: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Minutes<Impl: ISWbemDateTimeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iminutes: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMinutes<Impl: ISWbemDateTimeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iminutes: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MinutesSpecified<Impl: ISWbemDateTimeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bminutesspecified: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMinutesSpecified<Impl: ISWbemDateTimeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bminutesspecified: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Seconds<Impl: ISWbemDateTimeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iseconds: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSeconds<Impl: ISWbemDateTimeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iseconds: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SecondsSpecified<Impl: ISWbemDateTimeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bsecondsspecified: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSecondsSpecified<Impl: ISWbemDateTimeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bsecondsspecified: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Microseconds<Impl: ISWbemDateTimeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, imicroseconds: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMicroseconds<Impl: ISWbemDateTimeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, imicroseconds: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MicrosecondsSpecified<Impl: ISWbemDateTimeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bmicrosecondsspecified: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMicrosecondsSpecified<Impl: ISWbemDateTimeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bmicrosecondsspecified: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UTC<Impl: ISWbemDateTimeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iutc: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetUTC<Impl: ISWbemDateTimeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iutc: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UTCSpecified<Impl: ISWbemDateTimeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, butcspecified: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetUTCSpecified<Impl: ISWbemDateTimeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, butcspecified: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsInterval<Impl: ISWbemDateTimeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bisinterval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetIsInterval<Impl: ISWbemDateTimeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bisinterval: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetVarDate<Impl: ISWbemDateTimeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bislocal: i16, dvardate: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetVarDate<Impl: ISWbemDateTimeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dvardate: f64, bislocal: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFileTime<Impl: ISWbemDateTimeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bislocal: i16, strfiletime: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetFileTime<Impl: ISWbemDateTimeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strfiletime: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bislocal: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn NextEvent();
    fn Security_();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISWbemEventSourceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISWbemEventSourceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISWbemEventSourceVtbl {
        unsafe extern "system" fn NextEvent<Impl: ISWbemEventSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, itimeoutms: i32, objwbemobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Security_<Impl: ISWbemEventSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objwbemsecurity: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn ConnectServer();
    fn Security_();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISWbemLocatorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISWbemLocatorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISWbemLocatorVtbl {
        unsafe extern "system" fn ConnectServer<Impl: ISWbemLocatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strserver: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strnamespace: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, struser: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strlocale: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strauthority: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, isecurityflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemservices: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Security_<Impl: ISWbemLocatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objwbemsecurity: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn Name();
    fn Origin();
    fn InParameters();
    fn OutParameters();
    fn Qualifiers_();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISWbemMethodVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISWbemMethodImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISWbemMethodVtbl {
        unsafe extern "system" fn Name<Impl: ISWbemMethodImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Origin<Impl: ISWbemMethodImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strorigin: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InParameters<Impl: ISWbemMethodImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objwbeminparameters: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OutParameters<Impl: ISWbemMethodImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objwbemoutparameters: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Qualifiers_<Impl: ISWbemMethodImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objwbemqualifierset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn _NewEnum();
    fn Item();
    fn Count();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISWbemMethodSetVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISWbemMethodSetImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISWbemMethodSetVtbl {
        unsafe extern "system" fn _NewEnum<Impl: ISWbemMethodSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Item<Impl: ISWbemMethodSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemmethod: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Count<Impl: ISWbemMethodSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, icount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn Value();
    fn SetValue();
    fn Name();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISWbemNamedValueVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISWbemNamedValueImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISWbemNamedValueVtbl {
        unsafe extern "system" fn Value<Impl: ISWbemNamedValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varvalue: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetValue<Impl: ISWbemNamedValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varvalue: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Name<Impl: ISWbemNamedValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn _NewEnum();
    fn Item();
    fn Count();
    fn Add();
    fn Remove();
    fn Clone();
    fn DeleteAll();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISWbemNamedValueSetVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISWbemNamedValueSetImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISWbemNamedValueSetVtbl {
        unsafe extern "system" fn _NewEnum<Impl: ISWbemNamedValueSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Item<Impl: ISWbemNamedValueSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemnamedvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Count<Impl: ISWbemNamedValueSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, icount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Add<Impl: ISWbemNamedValueSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varvalue: *const super::Com::VARIANT, iflags: i32, objwbemnamedvalue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Remove<Impl: ISWbemNamedValueSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clone<Impl: ISWbemNamedValueSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objwbemnamedvalueset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DeleteAll<Impl: ISWbemNamedValueSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISWbemObjectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISWbemObjectImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISWbemObjectVtbl {
        unsafe extern "system" fn Put_<Impl: ISWbemObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemobjectpath: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PutAsync_<Impl: ISWbemObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objwbemsink: ::windows::core::RawPtr, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemasynccontext: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Delete_<Impl: ISWbemObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DeleteAsync_<Impl: ISWbemObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objwbemsink: ::windows::core::RawPtr, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemasynccontext: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Instances_<Impl: ISWbemObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemobjectset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InstancesAsync_<Impl: ISWbemObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objwbemsink: ::windows::core::RawPtr, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemasynccontext: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Subclasses_<Impl: ISWbemObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemobjectset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SubclassesAsync_<Impl: ISWbemObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objwbemsink: ::windows::core::RawPtr, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemasynccontext: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Associators_<Impl: ISWbemObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strassocclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strresultclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strresultrole: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strrole: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bclassesonly: i16, bschemaonly: i16, strrequiredassocqualifier: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strrequiredqualifier: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemobjectset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AssociatorsAsync_<Impl: ISWbemObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objwbemsink: ::windows::core::RawPtr, strassocclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strresultclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strresultrole: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strrole: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bclassesonly: i16, bschemaonly: i16, strrequiredassocqualifier: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strrequiredqualifier: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemasynccontext: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn References_<Impl: ISWbemObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strresultclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strrole: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bclassesonly: i16, bschemaonly: i16, strrequiredqualifier: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemobjectset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReferencesAsync_<Impl: ISWbemObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objwbemsink: ::windows::core::RawPtr, strresultclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strrole: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bclassesonly: i16, bschemaonly: i16, strrequiredqualifier: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemasynccontext: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ExecMethod_<Impl: ISWbemObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strmethodname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, objwbeminparameters: ::windows::core::RawPtr, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemoutparameters: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ExecMethodAsync_<Impl: ISWbemObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objwbemsink: ::windows::core::RawPtr, strmethodname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, objwbeminparameters: ::windows::core::RawPtr, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemasynccontext: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clone_<Impl: ISWbemObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objwbemobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetObjectText_<Impl: ISWbemObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iflags: i32, strobjecttext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SpawnDerivedClass_<Impl: ISWbemObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iflags: i32, objwbemobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SpawnInstance_<Impl: ISWbemObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iflags: i32, objwbemobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CompareTo_<Impl: ISWbemObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objwbemobject: ::windows::core::RawPtr, iflags: i32, bresult: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Qualifiers_<Impl: ISWbemObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objwbemqualifierset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Properties_<Impl: ISWbemObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objwbempropertyset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Methods_<Impl: ISWbemObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objwbemmethodset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Derivation_<Impl: ISWbemObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strclassnamearray: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Path_<Impl: ISWbemObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objwbemobjectpath: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Security_<Impl: ISWbemObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objwbemsecurity: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn Refresh_();
    fn SystemProperties_();
    fn GetText_();
    fn SetFromText_();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISWbemObjectExVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISWbemObjectExImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISWbemObjectExVtbl {
        unsafe extern "system" fn Refresh_<Impl: ISWbemObjectExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SystemProperties_<Impl: ISWbemObjectExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objwbempropertyset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetText_<Impl: ISWbemObjectExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iobjecttextformat: WbemObjectTextFormatEnum, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, bstext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetFromText_<Impl: ISWbemObjectExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstext: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iobjecttextformat: WbemObjectTextFormatEnum, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISWbemObjectPathVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISWbemObjectPathImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISWbemObjectPathVtbl {
        unsafe extern "system" fn Path<Impl: ISWbemObjectPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strpath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPath<Impl: ISWbemObjectPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RelPath<Impl: ISWbemObjectPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strrelpath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetRelPath<Impl: ISWbemObjectPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strrelpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Server<Impl: ISWbemObjectPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strserver: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetServer<Impl: ISWbemObjectPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strserver: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Namespace<Impl: ISWbemObjectPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strnamespace: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetNamespace<Impl: ISWbemObjectPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strnamespace: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ParentNamespace<Impl: ISWbemObjectPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strparentnamespace: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DisplayName<Impl: ISWbemObjectPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strdisplayname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDisplayName<Impl: ISWbemObjectPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strdisplayname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Class<Impl: ISWbemObjectPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strclass: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetClass<Impl: ISWbemObjectPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsClass<Impl: ISWbemObjectPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bisclass: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAsClass<Impl: ISWbemObjectPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsSingleton<Impl: ISWbemObjectPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bissingleton: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAsSingleton<Impl: ISWbemObjectPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Keys<Impl: ISWbemObjectPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objwbemnamedvalueset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Security_<Impl: ISWbemObjectPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objwbemsecurity: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Locale<Impl: ISWbemObjectPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strlocale: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetLocale<Impl: ISWbemObjectPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strlocale: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Authority<Impl: ISWbemObjectPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strauthority: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAuthority<Impl: ISWbemObjectPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strauthority: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn _NewEnum();
    fn Item();
    fn Count();
    fn Security_();
    fn ItemIndex();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISWbemObjectSetVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISWbemObjectSetImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISWbemObjectSetVtbl {
        unsafe extern "system" fn _NewEnum<Impl: ISWbemObjectSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Item<Impl: ISWbemObjectSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strobjectpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Count<Impl: ISWbemObjectSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, icount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Security_<Impl: ISWbemObjectSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objwbemsecurity: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ItemIndex<Impl: ISWbemObjectSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32, objwbemobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn IsEnabled();
    fn SetIsEnabled();
    fn Name();
    fn DisplayName();
    fn Identifier();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISWbemPrivilegeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISWbemPrivilegeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISWbemPrivilegeVtbl {
        unsafe extern "system" fn IsEnabled<Impl: ISWbemPrivilegeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bisenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetIsEnabled<Impl: ISWbemPrivilegeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bisenabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Name<Impl: ISWbemPrivilegeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strdisplayname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DisplayName<Impl: ISWbemPrivilegeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strdisplayname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Identifier<Impl: ISWbemPrivilegeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iprivilege: *mut WbemPrivilegeEnum) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn _NewEnum();
    fn Item();
    fn Count();
    fn Add();
    fn Remove();
    fn DeleteAll();
    fn AddAsString();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISWbemPrivilegeSetVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISWbemPrivilegeSetImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISWbemPrivilegeSetVtbl {
        unsafe extern "system" fn _NewEnum<Impl: ISWbemPrivilegeSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Item<Impl: ISWbemPrivilegeSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iprivilege: WbemPrivilegeEnum, objwbemprivilege: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Count<Impl: ISWbemPrivilegeSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, icount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Add<Impl: ISWbemPrivilegeSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iprivilege: WbemPrivilegeEnum, bisenabled: i16, objwbemprivilege: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Remove<Impl: ISWbemPrivilegeSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iprivilege: WbemPrivilegeEnum) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DeleteAll<Impl: ISWbemPrivilegeSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddAsString<Impl: ISWbemPrivilegeSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strprivilege: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bisenabled: i16, objwbemprivilege: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn Value();
    fn SetValue();
    fn Name();
    fn IsLocal();
    fn Origin();
    fn CIMType();
    fn Qualifiers_();
    fn IsArray();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISWbemPropertyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISWbemPropertyImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISWbemPropertyVtbl {
        unsafe extern "system" fn Value<Impl: ISWbemPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varvalue: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetValue<Impl: ISWbemPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varvalue: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Name<Impl: ISWbemPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsLocal<Impl: ISWbemPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bislocal: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Origin<Impl: ISWbemPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strorigin: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CIMType<Impl: ISWbemPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, icimtype: *mut WbemCimtypeEnum) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Qualifiers_<Impl: ISWbemPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objwbemqualifierset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsArray<Impl: ISWbemPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bisarray: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn _NewEnum();
    fn Item();
    fn Count();
    fn Add();
    fn Remove();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISWbemPropertySetVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISWbemPropertySetImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISWbemPropertySetVtbl {
        unsafe extern "system" fn _NewEnum<Impl: ISWbemPropertySetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Item<Impl: ISWbemPropertySetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemproperty: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Count<Impl: ISWbemPropertySetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, icount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Add<Impl: ISWbemPropertySetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, icimtype: WbemCimtypeEnum, bisarray: i16, iflags: i32, objwbemproperty: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Remove<Impl: ISWbemPropertySetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISWbemQualifierVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISWbemQualifierImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISWbemQualifierVtbl {
        unsafe extern "system" fn Value<Impl: ISWbemQualifierImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varvalue: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetValue<Impl: ISWbemQualifierImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varvalue: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Name<Impl: ISWbemQualifierImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsLocal<Impl: ISWbemQualifierImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bislocal: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PropagatesToSubclass<Impl: ISWbemQualifierImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bpropagatestosubclass: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPropagatesToSubclass<Impl: ISWbemQualifierImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bpropagatestosubclass: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PropagatesToInstance<Impl: ISWbemQualifierImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bpropagatestoinstance: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPropagatesToInstance<Impl: ISWbemQualifierImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bpropagatestoinstance: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsOverridable<Impl: ISWbemQualifierImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bisoverridable: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetIsOverridable<Impl: ISWbemQualifierImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bisoverridable: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsAmended<Impl: ISWbemQualifierImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bisamended: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn _NewEnum();
    fn Item();
    fn Count();
    fn Add();
    fn Remove();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISWbemQualifierSetVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISWbemQualifierSetImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISWbemQualifierSetVtbl {
        unsafe extern "system" fn _NewEnum<Impl: ISWbemQualifierSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Item<Impl: ISWbemQualifierSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemqualifier: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Count<Impl: ISWbemQualifierSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, icount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Add<Impl: ISWbemQualifierSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varval: *const super::Com::VARIANT, bpropagatestosubclass: i16, bpropagatestoinstance: i16, bisoverridable: i16, iflags: i32, objwbemqualifier: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Remove<Impl: ISWbemQualifierSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn Index();
    fn Refresher();
    fn IsSet();
    fn Object();
    fn ObjectSet();
    fn Remove();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISWbemRefreshableItemVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISWbemRefreshableItemImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISWbemRefreshableItemVtbl {
        unsafe extern "system" fn Index<Impl: ISWbemRefreshableItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iindex: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Refresher<Impl: ISWbemRefreshableItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objwbemrefresher: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsSet<Impl: ISWbemRefreshableItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bisset: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Object<Impl: ISWbemRefreshableItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objwbemobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ObjectSet<Impl: ISWbemRefreshableItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objwbemobjectset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Remove<Impl: ISWbemRefreshableItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISWbemRefresherVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISWbemRefresherImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISWbemRefresherVtbl {
        unsafe extern "system" fn _NewEnum<Impl: ISWbemRefresherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Item<Impl: ISWbemRefresherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iindex: i32, objwbemrefreshableitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Count<Impl: ISWbemRefresherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, icount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Add<Impl: ISWbemRefresherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objwbemservices: ::windows::core::RawPtr, bsinstancepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemrefreshableitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddEnum<Impl: ISWbemRefresherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objwbemservices: ::windows::core::RawPtr, bsclassname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemrefreshableitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Remove<Impl: ISWbemRefresherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iindex: i32, iflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Refresh<Impl: ISWbemRefresherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AutoReconnect<Impl: ISWbemRefresherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bcount: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAutoReconnect<Impl: ISWbemRefresherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bcount: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DeleteAll<Impl: ISWbemRefresherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn ImpersonationLevel();
    fn SetImpersonationLevel();
    fn AuthenticationLevel();
    fn SetAuthenticationLevel();
    fn Privileges();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISWbemSecurityVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISWbemSecurityImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISWbemSecurityVtbl {
        unsafe extern "system" fn ImpersonationLevel<Impl: ISWbemSecurityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iimpersonationlevel: *mut WbemImpersonationLevelEnum) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetImpersonationLevel<Impl: ISWbemSecurityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iimpersonationlevel: WbemImpersonationLevelEnum) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AuthenticationLevel<Impl: ISWbemSecurityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iauthenticationlevel: *mut WbemAuthenticationLevelEnum) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAuthenticationLevel<Impl: ISWbemSecurityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iauthenticationlevel: WbemAuthenticationLevelEnum) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Privileges<Impl: ISWbemSecurityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objwbemprivilegeset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISWbemServicesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISWbemServicesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISWbemServicesVtbl {
        unsafe extern "system" fn Get<Impl: ISWbemServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strobjectpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAsync<Impl: ISWbemServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objwbemsink: ::windows::core::RawPtr, strobjectpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemasynccontext: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Delete<Impl: ISWbemServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strobjectpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DeleteAsync<Impl: ISWbemServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objwbemsink: ::windows::core::RawPtr, strobjectpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemasynccontext: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InstancesOf<Impl: ISWbemServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemobjectset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InstancesOfAsync<Impl: ISWbemServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objwbemsink: ::windows::core::RawPtr, strclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemasynccontext: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SubclassesOf<Impl: ISWbemServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strsuperclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemobjectset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SubclassesOfAsync<Impl: ISWbemServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objwbemsink: ::windows::core::RawPtr, strsuperclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemasynccontext: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ExecQuery<Impl: ISWbemServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strquery: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strquerylanguage: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemobjectset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ExecQueryAsync<Impl: ISWbemServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objwbemsink: ::windows::core::RawPtr, strquery: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strquerylanguage: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemasynccontext: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
            panic!()
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
            panic!()
        }
        unsafe extern "system" fn ReferencesTo<Impl: ISWbemServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strobjectpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strresultclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strrole: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bclassesonly: i16, bschemaonly: i16, strrequiredqualifier: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemobjectset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReferencesToAsync<Impl: ISWbemServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objwbemsink: ::windows::core::RawPtr, strobjectpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strresultclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strrole: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bclassesonly: i16, bschemaonly: i16, strrequiredqualifier: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemasynccontext: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ExecNotificationQuery<Impl: ISWbemServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strquery: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strquerylanguage: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemeventsource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ExecNotificationQueryAsync<Impl: ISWbemServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objwbemsink: ::windows::core::RawPtr, strquery: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strquerylanguage: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemasynccontext: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ExecMethod<Impl: ISWbemServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strobjectpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strmethodname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, objwbeminparameters: ::windows::core::RawPtr, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemoutparameters: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ExecMethodAsync<Impl: ISWbemServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objwbemsink: ::windows::core::RawPtr, strobjectpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strmethodname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, objwbeminparameters: ::windows::core::RawPtr, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemasynccontext: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Security_<Impl: ISWbemServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objwbemsecurity: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn Put();
    fn PutAsync();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISWbemServicesExVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISWbemServicesExImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISWbemServicesExVtbl {
        unsafe extern "system" fn Put<Impl: ISWbemServicesExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objwbemobject: ::windows::core::RawPtr, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemobjectpath: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PutAsync<Impl: ISWbemServicesExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objwbemsink: ::windows::core::RawPtr, objwbemobject: ::windows::core::RawPtr, iflags: i32, objwbemnamedvalueset: ::windows::core::RawPtr, objwbemasynccontext: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn Cancel();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISWbemSinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISWbemSinkImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISWbemSinkVtbl {
        unsafe extern "system" fn Cancel<Impl: ISWbemSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn CreateObjectStub();
}
impl IUnsecuredApartmentVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUnsecuredApartmentImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUnsecuredApartmentVtbl {
        unsafe extern "system" fn CreateObjectStub<Impl: IUnsecuredApartmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pobject: *mut ::core::ffi::c_void, ppstub: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), CreateObjectStub: CreateObjectStub::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUnsecuredApartment as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWMIExtensionImpl: Sized + IDispatchImpl {
    fn WMIObjectPath();
    fn GetWMIObject();
    fn GetWMIServices();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWMIExtensionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMIExtensionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMIExtensionVtbl {
        unsafe extern "system" fn WMIObjectPath<Impl: IWMIExtensionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strwmiobjectpath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetWMIObject<Impl: IWMIExtensionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objwmiobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetWMIServices<Impl: IWMIExtensionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objwmiservices: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn Resolve();
}
#[cfg(feature = "Win32_Foundation")]
impl IWbemAddressResolutionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWbemAddressResolutionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWbemAddressResolutionVtbl {
        unsafe extern "system" fn Resolve<Impl: IWbemAddressResolutionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wsznamespacepath: super::super::Foundation::PWSTR, wszaddresstype: super::super::Foundation::PWSTR, pdwaddresslength: *mut u32, pabbinaryaddress: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), Resolve: Resolve::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWbemAddressResolution as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWbemBackupRestoreImpl: Sized {
    fn Backup();
    fn Restore();
}
#[cfg(feature = "Win32_Foundation")]
impl IWbemBackupRestoreVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWbemBackupRestoreImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWbemBackupRestoreVtbl {
        unsafe extern "system" fn Backup<Impl: IWbemBackupRestoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strbackuptofile: super::super::Foundation::PWSTR, lflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Restore<Impl: IWbemBackupRestoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strrestorefromfile: super::super::Foundation::PWSTR, lflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), Backup: Backup::<Impl, IMPL_OFFSET>, Restore: Restore::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWbemBackupRestore as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWbemBackupRestoreExImpl: Sized + IWbemBackupRestoreImpl {
    fn Pause();
    fn Resume();
}
#[cfg(feature = "Win32_Foundation")]
impl IWbemBackupRestoreExVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWbemBackupRestoreExImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWbemBackupRestoreExVtbl {
        unsafe extern "system" fn Pause<Impl: IWbemBackupRestoreExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Resume<Impl: IWbemBackupRestoreExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn GetResultObject();
    fn GetResultString();
    fn GetResultServices();
    fn GetCallStatus();
}
#[cfg(feature = "Win32_Foundation")]
impl IWbemCallResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWbemCallResultImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWbemCallResultVtbl {
        unsafe extern "system" fn GetResultObject<Impl: IWbemCallResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ltimeout: i32, ppresultobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetResultString<Impl: IWbemCallResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ltimeout: i32, pstrresultstring: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetResultServices<Impl: IWbemCallResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ltimeout: i32, ppservices: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCallStatus<Impl: IWbemCallResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ltimeout: i32, plstatus: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWbemClassObjectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWbemClassObjectImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWbemClassObjectVtbl {
        unsafe extern "system" fn GetQualifierSet<Impl: IWbemClassObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqualset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Get<Impl: IWbemClassObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszname: super::super::Foundation::PWSTR, lflags: i32, pval: *mut super::Com::VARIANT, ptype: *mut i32, plflavor: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Put<Impl: IWbemClassObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszname: super::super::Foundation::PWSTR, lflags: i32, pval: *const super::Com::VARIANT, r#type: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Delete<Impl: IWbemClassObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetNames<Impl: IWbemClassObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszqualifiername: super::super::Foundation::PWSTR, lflags: i32, pqualifierval: *const super::Com::VARIANT, pnames: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BeginEnumeration<Impl: IWbemClassObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lenumflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Next<Impl: IWbemClassObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, strname: *mut super::super::Foundation::BSTR, pval: *mut super::Com::VARIANT, ptype: *mut i32, plflavor: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EndEnumeration<Impl: IWbemClassObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPropertyQualifierSet<Impl: IWbemClassObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszproperty: super::super::Foundation::PWSTR, ppqualset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clone<Impl: IWbemClassObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcopy: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetObjectText<Impl: IWbemClassObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, pstrobjecttext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SpawnDerivedClass<Impl: IWbemClassObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, ppnewclass: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SpawnInstance<Impl: IWbemClassObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, ppnewinstance: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CompareTo<Impl: IWbemClassObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, pcompareto: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPropertyOrigin<Impl: IWbemClassObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszname: super::super::Foundation::PWSTR, pstrclassname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InheritsFrom<Impl: IWbemClassObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strancestor: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMethod<Impl: IWbemClassObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszname: super::super::Foundation::PWSTR, lflags: i32, ppinsignature: *mut ::windows::core::RawPtr, ppoutsignature: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PutMethod<Impl: IWbemClassObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszname: super::super::Foundation::PWSTR, lflags: i32, pinsignature: ::windows::core::RawPtr, poutsignature: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DeleteMethod<Impl: IWbemClassObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BeginMethodEnumeration<Impl: IWbemClassObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lenumflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn NextMethod<Impl: IWbemClassObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, pstrname: *mut super::super::Foundation::BSTR, ppinsignature: *mut ::windows::core::RawPtr, ppoutsignature: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EndMethodEnumeration<Impl: IWbemClassObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMethodQualifierSet<Impl: IWbemClassObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszmethod: super::super::Foundation::PWSTR, ppqualset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMethodOrigin<Impl: IWbemClassObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszmethodname: super::super::Foundation::PWSTR, pstrclassname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn Open();
    fn OpenAsync();
    fn Cancel();
}
#[cfg(feature = "Win32_Foundation")]
impl IWbemClientConnectionTransportVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWbemClientConnectionTransportImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWbemClientConnectionTransportVtbl {
        unsafe extern "system" fn Open<Impl: IWbemClientConnectionTransportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, straddresstype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwbinaryaddresslength: u32, abbinaryaddress: *const u8, strobject: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, struser: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strlocale: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lflags: i32, pctx: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, pinterface: *mut *mut ::core::ffi::c_void, pcallres: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OpenAsync<Impl: IWbemClientConnectionTransportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, straddresstype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwbinaryaddresslength: u32, abbinaryaddress: *const u8, strobject: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, struser: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strlocale: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lflags: i32, pctx: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, presponsehandler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Cancel<Impl: IWbemClientConnectionTransportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, phandler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn ConnectServer();
}
#[cfg(feature = "Win32_Foundation")]
impl IWbemClientTransportVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWbemClientTransportImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWbemClientTransportVtbl {
        unsafe extern "system" fn ConnectServer<Impl: IWbemClientTransportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, straddresstype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwbinaryaddresslength: u32, abbinaryaddress: *const u8, strnetworkresource: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, struser: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strlocale: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lsecurityflags: i32, strauthority: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pctx: ::windows::core::RawPtr, ppnamespace: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), ConnectServer: ConnectServer::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWbemClientTransport as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWbemConfigureRefresherImpl: Sized {
    fn AddObjectByPath();
    fn AddObjectByTemplate();
    fn AddRefresher();
    fn Remove();
    fn AddEnum();
}
#[cfg(feature = "Win32_Foundation")]
impl IWbemConfigureRefresherVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWbemConfigureRefresherImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWbemConfigureRefresherVtbl {
        unsafe extern "system" fn AddObjectByPath<Impl: IWbemConfigureRefresherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnamespace: ::windows::core::RawPtr, wszpath: super::super::Foundation::PWSTR, lflags: i32, pcontext: ::windows::core::RawPtr, pprefreshable: *mut ::windows::core::RawPtr, plid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddObjectByTemplate<Impl: IWbemConfigureRefresherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnamespace: ::windows::core::RawPtr, ptemplate: ::windows::core::RawPtr, lflags: i32, pcontext: ::windows::core::RawPtr, pprefreshable: *mut ::windows::core::RawPtr, plid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddRefresher<Impl: IWbemConfigureRefresherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prefresher: ::windows::core::RawPtr, lflags: i32, plid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Remove<Impl: IWbemConfigureRefresherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lid: i32, lflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddEnum<Impl: IWbemConfigureRefresherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnamespace: ::windows::core::RawPtr, wszclassname: super::super::Foundation::PWSTR, lflags: i32, pcontext: ::windows::core::RawPtr, ppenum: *mut ::windows::core::RawPtr, plid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn ConnectorLogin();
}
#[cfg(feature = "Win32_Foundation")]
impl IWbemConnectorLoginVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWbemConnectorLoginImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWbemConnectorLoginVtbl {
        unsafe extern "system" fn ConnectorLogin<Impl: IWbemConnectorLoginImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wsznetworkresource: super::super::Foundation::PWSTR, wszpreferredlocale: super::super::Foundation::PWSTR, lflags: i32, pctx: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, pinterface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), ConnectorLogin: ConnectorLogin::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWbemConnectorLogin as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWbemConstructClassObjectImpl: Sized {
    fn SetInheritanceChain();
    fn SetPropertyOrigin();
    fn SetMethodOrigin();
    fn SetServerNamespace();
}
#[cfg(feature = "Win32_Foundation")]
impl IWbemConstructClassObjectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWbemConstructClassObjectImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWbemConstructClassObjectVtbl {
        unsafe extern "system" fn SetInheritanceChain<Impl: IWbemConstructClassObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnumantecedents: i32, awszantecedents: *const super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPropertyOrigin<Impl: IWbemConstructClassObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszpropertyname: super::super::Foundation::PWSTR, loriginindex: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMethodOrigin<Impl: IWbemConstructClassObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszmethodname: super::super::Foundation::PWSTR, loriginindex: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetServerNamespace<Impl: IWbemConstructClassObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszserver: super::super::Foundation::PWSTR, wsznamespace: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWbemContextVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWbemContextImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWbemContextVtbl {
        unsafe extern "system" fn Clone<Impl: IWbemContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppnewcopy: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetNames<Impl: IWbemContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, pnames: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BeginEnumeration<Impl: IWbemContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Next<Impl: IWbemContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, pstrname: *mut super::super::Foundation::BSTR, pvalue: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EndEnumeration<Impl: IWbemContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetValue<Impl: IWbemContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszname: super::super::Foundation::PWSTR, lflags: i32, pvalue: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetValue<Impl: IWbemContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszname: super::super::Foundation::PWSTR, lflags: i32, pvalue: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DeleteValue<Impl: IWbemContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszname: super::super::Foundation::PWSTR, lflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DeleteAll<Impl: IWbemContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn GetSink();
    fn GetService();
}
#[cfg(feature = "Win32_Foundation")]
impl IWbemDecoupledBasicEventProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWbemDecoupledBasicEventProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWbemDecoupledBasicEventProviderVtbl {
        unsafe extern "system" fn GetSink<Impl: IWbemDecoupledBasicEventProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, a_flags: i32, a_context: ::windows::core::RawPtr, a_sink: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetService<Impl: IWbemDecoupledBasicEventProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, a_flags: i32, a_context: ::windows::core::RawPtr, a_service: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn Register();
    fn UnRegister();
}
#[cfg(feature = "Win32_Foundation")]
impl IWbemDecoupledRegistrarVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWbemDecoupledRegistrarImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWbemDecoupledRegistrarVtbl {
        unsafe extern "system" fn Register<Impl: IWbemDecoupledRegistrarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, a_flags: i32, a_context: ::windows::core::RawPtr, a_user: super::super::Foundation::PWSTR, a_locale: super::super::Foundation::PWSTR, a_scope: super::super::Foundation::PWSTR, a_registration: super::super::Foundation::PWSTR, piunknown: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UnRegister<Impl: IWbemDecoupledRegistrarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn FindConsumer();
}
impl IWbemEventConsumerProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWbemEventConsumerProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWbemEventConsumerProviderVtbl {
        unsafe extern "system" fn FindConsumer<Impl: IWbemEventConsumerProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plogicalconsumer: ::windows::core::RawPtr, ppconsumer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), FindConsumer: FindConsumer::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWbemEventConsumerProvider as ::windows::core::Interface>::IID
    }
}
pub trait IWbemEventProviderImpl: Sized {
    fn ProvideEvents();
}
impl IWbemEventProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWbemEventProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWbemEventProviderVtbl {
        unsafe extern "system" fn ProvideEvents<Impl: IWbemEventProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psink: ::windows::core::RawPtr, lflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), ProvideEvents: ProvideEvents::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWbemEventProvider as ::windows::core::Interface>::IID
    }
}
pub trait IWbemEventProviderQuerySinkImpl: Sized {
    fn NewQuery();
    fn CancelQuery();
}
impl IWbemEventProviderQuerySinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWbemEventProviderQuerySinkImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWbemEventProviderQuerySinkVtbl {
        unsafe extern "system" fn NewQuery<Impl: IWbemEventProviderQuerySinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwid: u32, wszquerylanguage: *const u16, wszquery: *const u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CancelQuery<Impl: IWbemEventProviderQuerySinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn AccessCheck();
}
impl IWbemEventProviderSecurityVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWbemEventProviderSecurityImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWbemEventProviderSecurityVtbl {
        unsafe extern "system" fn AccessCheck<Impl: IWbemEventProviderSecurityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszquerylanguage: *const u16, wszquery: *const u16, lsidlength: i32, psid: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), AccessCheck: AccessCheck::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWbemEventProviderSecurity as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWbemEventSinkImpl: Sized + IWbemObjectSinkImpl {
    fn SetSinkSecurity();
    fn IsActive();
    fn GetRestrictedSink();
    fn SetBatchingParameters();
}
#[cfg(feature = "Win32_Foundation")]
impl IWbemEventSinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWbemEventSinkImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWbemEventSinkVtbl {
        unsafe extern "system" fn SetSinkSecurity<Impl: IWbemEventSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lsdlength: i32, psd: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsActive<Impl: IWbemEventSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRestrictedSink<Impl: IWbemEventSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnumqueries: i32, awszqueries: *const super::super::Foundation::PWSTR, pcallback: *mut ::core::ffi::c_void, ppsink: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetBatchingParameters<Impl: IWbemEventSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, dwmaxbuffersize: u32, dwmaxsendlatency: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn AddObjects();
    fn RemoveObjects();
    fn GetObjects();
    fn RemoveAll();
}
impl IWbemHiPerfEnumVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWbemHiPerfEnumImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWbemHiPerfEnumVtbl {
        unsafe extern "system" fn AddObjects<Impl: IWbemHiPerfEnumImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, unumobjects: u32, apids: *const i32, apobj: *const ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveObjects<Impl: IWbemHiPerfEnumImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, unumobjects: u32, apids: *const i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetObjects<Impl: IWbemHiPerfEnumImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, unumobjects: u32, apobj: *mut ::windows::core::RawPtr, pureturned: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveAll<Impl: IWbemHiPerfEnumImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn QueryInstances();
    fn CreateRefresher();
    fn CreateRefreshableObject();
    fn StopRefreshing();
    fn CreateRefreshableEnum();
    fn GetObjects();
}
#[cfg(feature = "Win32_Foundation")]
impl IWbemHiPerfProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWbemHiPerfProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWbemHiPerfProviderVtbl {
        unsafe extern "system" fn QueryInstances<Impl: IWbemHiPerfProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnamespace: ::windows::core::RawPtr, wszclass: super::super::Foundation::PWSTR, lflags: i32, pctx: ::windows::core::RawPtr, psink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateRefresher<Impl: IWbemHiPerfProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnamespace: ::windows::core::RawPtr, lflags: i32, pprefresher: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateRefreshableObject<Impl: IWbemHiPerfProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnamespace: ::windows::core::RawPtr, ptemplate: ::windows::core::RawPtr, prefresher: ::windows::core::RawPtr, lflags: i32, pcontext: ::windows::core::RawPtr, pprefreshable: *mut ::windows::core::RawPtr, plid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn StopRefreshing<Impl: IWbemHiPerfProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prefresher: ::windows::core::RawPtr, lid: i32, lflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateRefreshableEnum<Impl: IWbemHiPerfProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnamespace: ::windows::core::RawPtr, wszclass: super::super::Foundation::PWSTR, prefresher: ::windows::core::RawPtr, lflags: i32, pcontext: ::windows::core::RawPtr, phiperfenum: ::windows::core::RawPtr, plid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetObjects<Impl: IWbemHiPerfProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnamespace: ::windows::core::RawPtr, lnumobjects: i32, apobj: *mut ::windows::core::RawPtr, lflags: i32, pcontext: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn EstablishPosition();
    fn RequestChallenge();
    fn WBEMLogin();
    fn NTLMLogin();
}
#[cfg(feature = "Win32_Foundation")]
impl IWbemLevel1LoginVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWbemLevel1LoginImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWbemLevel1LoginVtbl {
        unsafe extern "system" fn EstablishPosition<Impl: IWbemLevel1LoginImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszlocalelist: super::super::Foundation::PWSTR, dwnumlocales: u32, reserved: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RequestChallenge<Impl: IWbemLevel1LoginImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wsznetworkresource: super::super::Foundation::PWSTR, wszuser: super::super::Foundation::PWSTR, nonce: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WBEMLogin<Impl: IWbemLevel1LoginImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszpreferredlocale: super::super::Foundation::PWSTR, accesstoken: *const u8, lflags: i32, pctx: ::windows::core::RawPtr, ppnamespace: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn NTLMLogin<Impl: IWbemLevel1LoginImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wsznetworkresource: super::super::Foundation::PWSTR, wszpreferredlocale: super::super::Foundation::PWSTR, lflags: i32, pctx: ::windows::core::RawPtr, ppnamespace: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn ConnectServer();
}
#[cfg(feature = "Win32_Foundation")]
impl IWbemLocatorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWbemLocatorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWbemLocatorVtbl {
        unsafe extern "system" fn ConnectServer<Impl: IWbemLocatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strnetworkresource: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, struser: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strlocale: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lsecurityflags: i32, strauthority: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pctx: ::windows::core::RawPtr, ppnamespace: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), ConnectServer: ConnectServer::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWbemLocator as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWbemObjectAccessVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWbemObjectAccessImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWbemObjectAccessVtbl {
        unsafe extern "system" fn GetPropertyHandle<Impl: IWbemObjectAccessImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszpropertyname: super::super::Foundation::PWSTR, ptype: *mut i32, plhandle: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WritePropertyValue<Impl: IWbemObjectAccessImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lhandle: i32, lnumbytes: i32, adata: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReadPropertyValue<Impl: IWbemObjectAccessImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lhandle: i32, lbuffersize: i32, plnumbytes: *mut i32, adata: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReadDWORD<Impl: IWbemObjectAccessImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lhandle: i32, pdw: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WriteDWORD<Impl: IWbemObjectAccessImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lhandle: i32, dw: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReadQWORD<Impl: IWbemObjectAccessImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lhandle: i32, pqw: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WriteQWORD<Impl: IWbemObjectAccessImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lhandle: i32, pw: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPropertyInfoByHandle<Impl: IWbemObjectAccessImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lhandle: i32, pstrname: *mut super::super::Foundation::BSTR, ptype: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Lock<Impl: IWbemObjectAccessImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Unlock<Impl: IWbemObjectAccessImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn Indicate();
    fn SetStatus();
}
#[cfg(feature = "Win32_Foundation")]
impl IWbemObjectSinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWbemObjectSinkImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWbemObjectSinkVtbl {
        unsafe extern "system" fn Indicate<Impl: IWbemObjectSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lobjectcount: i32, apobjarray: *const ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetStatus<Impl: IWbemObjectSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, hresult: ::windows::core::HRESULT, strparam: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pobjparam: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn WriteMessage();
    fn WriteError();
    fn PromptUser();
    fn WriteProgress();
    fn WriteStreamParameter();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWbemObjectSinkExVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWbemObjectSinkExImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWbemObjectSinkExVtbl {
        unsafe extern "system" fn WriteMessage<Impl: IWbemObjectSinkExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uchannel: u32, strmessage: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WriteError<Impl: IWbemObjectSinkExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pobjerror: ::windows::core::RawPtr, pureturned: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PromptUser<Impl: IWbemObjectSinkExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strmessage: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, uprompttype: u8, pureturned: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WriteProgress<Impl: IWbemObjectSinkExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stractivity: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strcurrentoperation: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strstatusdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, upercentcomplete: u32, usecondsremaining: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WriteStreamParameter<Impl: IWbemObjectSinkExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, vtvalue: *const super::Com::VARIANT, ultype: u32, ulflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn GetText();
    fn CreateFromText();
}
#[cfg(feature = "Win32_Foundation")]
impl IWbemObjectTextSrcVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWbemObjectTextSrcImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWbemObjectTextSrcVtbl {
        unsafe extern "system" fn GetText<Impl: IWbemObjectTextSrcImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, pobj: ::windows::core::RawPtr, uobjtextformat: u32, pctx: ::windows::core::RawPtr, strtext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateFromText<Impl: IWbemObjectTextSrcImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, strtext: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, uobjtextformat: u32, pctx: ::windows::core::RawPtr, pnewobj: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
#[cfg(feature = "Win32_Foundation")]
impl IWbemPathVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWbemPathImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWbemPathVtbl {
        unsafe extern "system" fn SetText<Impl: IWbemPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, umode: u32, pszpath: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetText<Impl: IWbemPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, pubufflength: *mut u32, psztext: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetInfo<Impl: IWbemPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, urequestedinfo: u32, puresponse: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetServer<Impl: IWbemPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetServer<Impl: IWbemPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punamebuflength: *mut u32, pname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetNamespaceCount<Impl: IWbemPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pucount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetNamespaceAt<Impl: IWbemPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uindex: u32, pszname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetNamespaceAt<Impl: IWbemPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uindex: u32, punamebuflength: *mut u32, pname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveNamespaceAt<Impl: IWbemPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveAllNamespaces<Impl: IWbemPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetScopeCount<Impl: IWbemPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pucount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetScope<Impl: IWbemPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uindex: u32, pszclass: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetScopeFromText<Impl: IWbemPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uindex: u32, psztext: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetScope<Impl: IWbemPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uindex: u32, puclassnamebufsize: *mut u32, pszclass: super::super::Foundation::PWSTR, pkeylist: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetScopeAsText<Impl: IWbemPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uindex: u32, putextbufsize: *mut u32, psztext: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveScope<Impl: IWbemPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveAllScopes<Impl: IWbemPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetClassName<Impl: IWbemPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetClassName<Impl: IWbemPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pubufflength: *mut u32, pszname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetKeyList<Impl: IWbemPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pout: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateClassPart<Impl: IWbemPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, name: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DeleteClassPart<Impl: IWbemPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsRelative<Impl: IWbemPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszmachine: super::super::Foundation::PWSTR, wsznamespace: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsRelativeOrChild<Impl: IWbemPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszmachine: super::super::Foundation::PWSTR, wsznamespace: super::super::Foundation::PWSTR, lflags: i32) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsLocal<Impl: IWbemPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszmachine: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsSameClassName<Impl: IWbemPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszclass: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWbemPathKeyListVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWbemPathKeyListImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWbemPathKeyListVtbl {
        unsafe extern "system" fn GetCount<Impl: IWbemPathKeyListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pukeycount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetKey<Impl: IWbemPathKeyListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszname: super::super::Foundation::PWSTR, uflags: u32, ucimtype: u32, pkeyval: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetKey2<Impl: IWbemPathKeyListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszname: super::super::Foundation::PWSTR, uflags: u32, ucimtype: u32, pkeyval: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetKey<Impl: IWbemPathKeyListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ukeyix: u32, uflags: u32, punamebufsize: *mut u32, pszkeyname: super::super::Foundation::PWSTR, pukeyvalbufsize: *mut u32, pkeyval: *mut ::core::ffi::c_void, puapparentcimtype: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetKey2<Impl: IWbemPathKeyListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ukeyix: u32, uflags: u32, punamebufsize: *mut u32, pszkeyname: super::super::Foundation::PWSTR, pkeyvalue: *mut super::Com::VARIANT, puapparentcimtype: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveKey<Impl: IWbemPathKeyListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszname: super::super::Foundation::PWSTR, uflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveAllKeys<Impl: IWbemPathKeyListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MakeSingleton<Impl: IWbemPathKeyListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bset: u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetInfo<Impl: IWbemPathKeyListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, urequestedinfo: u32, puresponse: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetText<Impl: IWbemPathKeyListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, pubufflength: *mut u32, psztext: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn GetProperty();
    fn PutProperty();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWbemPropertyProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWbemPropertyProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWbemPropertyProviderVtbl {
        unsafe extern "system" fn GetProperty<Impl: IWbemPropertyProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, strlocale: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strclassmapping: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strinstmapping: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strpropmapping: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvvalue: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PutProperty<Impl: IWbemPropertyProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, strlocale: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strclassmapping: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strinstmapping: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strpropmapping: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvvalue: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn SetRegistrationObject();
}
impl IWbemProviderIdentityVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWbemProviderIdentityImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWbemProviderIdentityVtbl {
        unsafe extern "system" fn SetRegistrationObject<Impl: IWbemProviderIdentityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, pprovreg: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), SetRegistrationObject: SetRegistrationObject::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWbemProviderIdentity as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWbemProviderInitImpl: Sized {
    fn Initialize();
}
#[cfg(feature = "Win32_Foundation")]
impl IWbemProviderInitVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWbemProviderInitImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWbemProviderInitVtbl {
        unsafe extern "system" fn Initialize<Impl: IWbemProviderInitImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszuser: super::super::Foundation::PWSTR, lflags: i32, wsznamespace: super::super::Foundation::PWSTR, wszlocale: super::super::Foundation::PWSTR, pnamespace: ::windows::core::RawPtr, pctx: ::windows::core::RawPtr, pinitsink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), Initialize: Initialize::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWbemProviderInit as ::windows::core::Interface>::IID
    }
}
pub trait IWbemProviderInitSinkImpl: Sized {
    fn SetStatus();
}
impl IWbemProviderInitSinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWbemProviderInitSinkImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWbemProviderInitSinkVtbl {
        unsafe extern "system" fn SetStatus<Impl: IWbemProviderInitSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lstatus: i32, lflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), SetStatus: SetStatus::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWbemProviderInitSink as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWbemQualifierSetImpl: Sized {
    fn Get();
    fn Put();
    fn Delete();
    fn GetNames();
    fn BeginEnumeration();
    fn Next();
    fn EndEnumeration();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWbemQualifierSetVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWbemQualifierSetImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWbemQualifierSetVtbl {
        unsafe extern "system" fn Get<Impl: IWbemQualifierSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszname: super::super::Foundation::PWSTR, lflags: i32, pval: *mut super::Com::VARIANT, plflavor: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Put<Impl: IWbemQualifierSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszname: super::super::Foundation::PWSTR, pval: *const super::Com::VARIANT, lflavor: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Delete<Impl: IWbemQualifierSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetNames<Impl: IWbemQualifierSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, pnames: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BeginEnumeration<Impl: IWbemQualifierSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Next<Impl: IWbemQualifierSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, pstrname: *mut super::super::Foundation::BSTR, pval: *mut super::Com::VARIANT, plflavor: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EndEnumeration<Impl: IWbemQualifierSetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn Empty();
    fn SetLanguageFeatures();
    fn TestLanguageFeatures();
    fn Parse();
    fn GetAnalysis();
    fn FreeMemory();
    fn GetQueryInfo();
}
#[cfg(feature = "Win32_Foundation")]
impl IWbemQueryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWbemQueryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWbemQueryVtbl {
        unsafe extern "system" fn Empty<Impl: IWbemQueryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetLanguageFeatures<Impl: IWbemQueryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uflags: u32, uarraysize: u32, pufeatures: *const u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TestLanguageFeatures<Impl: IWbemQueryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uflags: u32, uarraysize: *mut u32, pufeatures: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Parse<Impl: IWbemQueryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszlang: super::super::Foundation::PWSTR, pszquery: super::super::Foundation::PWSTR, uflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAnalysis<Impl: IWbemQueryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uanalysistype: u32, uflags: u32, panalysis: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FreeMemory<Impl: IWbemQueryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmem: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetQueryInfo<Impl: IWbemQueryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uanalysistype: u32, uinfoid: u32, ubufsize: u32, pdestbuf: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn Refresh();
}
impl IWbemRefresherVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWbemRefresherImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWbemRefresherVtbl {
        unsafe extern "system" fn Refresh<Impl: IWbemRefresherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), Refresh: Refresh::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWbemRefresher as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
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
#[cfg(feature = "Win32_Foundation")]
impl IWbemServicesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWbemServicesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWbemServicesVtbl {
        unsafe extern "system" fn OpenNamespace<Impl: IWbemServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strnamespace: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lflags: i32, pctx: ::windows::core::RawPtr, ppworkingnamespace: *mut ::windows::core::RawPtr, ppresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CancelAsyncCall<Impl: IWbemServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn QueryObjectSink<Impl: IWbemServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, ppresponsehandler: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetObject<Impl: IWbemServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strobjectpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lflags: i32, pctx: ::windows::core::RawPtr, ppobject: *mut ::windows::core::RawPtr, ppcallresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetObjectAsync<Impl: IWbemServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strobjectpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lflags: i32, pctx: ::windows::core::RawPtr, presponsehandler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PutClass<Impl: IWbemServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pobject: ::windows::core::RawPtr, lflags: i32, pctx: ::windows::core::RawPtr, ppcallresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PutClassAsync<Impl: IWbemServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pobject: ::windows::core::RawPtr, lflags: i32, pctx: ::windows::core::RawPtr, presponsehandler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DeleteClass<Impl: IWbemServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lflags: i32, pctx: ::windows::core::RawPtr, ppcallresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DeleteClassAsync<Impl: IWbemServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lflags: i32, pctx: ::windows::core::RawPtr, presponsehandler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateClassEnum<Impl: IWbemServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strsuperclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lflags: i32, pctx: ::windows::core::RawPtr, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateClassEnumAsync<Impl: IWbemServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strsuperclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lflags: i32, pctx: ::windows::core::RawPtr, presponsehandler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PutInstance<Impl: IWbemServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinst: ::windows::core::RawPtr, lflags: i32, pctx: ::windows::core::RawPtr, ppcallresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PutInstanceAsync<Impl: IWbemServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinst: ::windows::core::RawPtr, lflags: i32, pctx: ::windows::core::RawPtr, presponsehandler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DeleteInstance<Impl: IWbemServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strobjectpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lflags: i32, pctx: ::windows::core::RawPtr, ppcallresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DeleteInstanceAsync<Impl: IWbemServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strobjectpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lflags: i32, pctx: ::windows::core::RawPtr, presponsehandler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateInstanceEnum<Impl: IWbemServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strfilter: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lflags: i32, pctx: ::windows::core::RawPtr, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateInstanceEnumAsync<Impl: IWbemServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strfilter: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lflags: i32, pctx: ::windows::core::RawPtr, presponsehandler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ExecQuery<Impl: IWbemServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strquerylanguage: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strquery: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lflags: i32, pctx: ::windows::core::RawPtr, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ExecQueryAsync<Impl: IWbemServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strquerylanguage: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strquery: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lflags: i32, pctx: ::windows::core::RawPtr, presponsehandler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ExecNotificationQuery<Impl: IWbemServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strquerylanguage: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strquery: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lflags: i32, pctx: ::windows::core::RawPtr, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ExecNotificationQueryAsync<Impl: IWbemServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strquerylanguage: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strquery: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lflags: i32, pctx: ::windows::core::RawPtr, presponsehandler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ExecMethod<Impl: IWbemServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strobjectpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strmethodname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lflags: i32, pctx: ::windows::core::RawPtr, pinparams: ::windows::core::RawPtr, ppoutparams: *mut ::windows::core::RawPtr, ppcallresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ExecMethodAsync<Impl: IWbemServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strobjectpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, strmethodname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lflags: i32, pctx: ::windows::core::RawPtr, pinparams: ::windows::core::RawPtr, presponsehandler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn Shutdown();
}
impl IWbemShutdownVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWbemShutdownImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWbemShutdownVtbl {
        unsafe extern "system" fn Shutdown<Impl: IWbemShutdownImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ureason: i32, umaxmilliseconds: u32, pctx: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), Shutdown: Shutdown::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWbemShutdown as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWbemStatusCodeTextImpl: Sized {
    fn GetErrorCodeText();
    fn GetFacilityCodeText();
}
#[cfg(feature = "Win32_Foundation")]
impl IWbemStatusCodeTextVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWbemStatusCodeTextImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWbemStatusCodeTextVtbl {
        unsafe extern "system" fn GetErrorCodeText<Impl: IWbemStatusCodeTextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hres: ::windows::core::HRESULT, localeid: u32, lflags: i32, messagetext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFacilityCodeText<Impl: IWbemStatusCodeTextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hres: ::windows::core::HRESULT, localeid: u32, lflags: i32, messagetext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn Initialize();
}
impl IWbemTransportVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWbemTransportImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWbemTransportVtbl {
        unsafe extern "system" fn Initialize<Impl: IWbemTransportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), Initialize: Initialize::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWbemTransport as ::windows::core::Interface>::IID
    }
}
pub trait IWbemUnboundObjectSinkImpl: Sized {
    fn IndicateToConsumer();
}
impl IWbemUnboundObjectSinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWbemUnboundObjectSinkImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWbemUnboundObjectSinkVtbl {
        unsafe extern "system" fn IndicateToConsumer<Impl: IWbemUnboundObjectSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plogicalconsumer: ::windows::core::RawPtr, lnumobjects: i32, apobjects: *const ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), IndicateToConsumer: IndicateToConsumer::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWbemUnboundObjectSink as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWbemUnsecuredApartmentImpl: Sized + IUnsecuredApartmentImpl {
    fn CreateSinkStub();
}
#[cfg(feature = "Win32_Foundation")]
impl IWbemUnsecuredApartmentVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWbemUnsecuredApartmentImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWbemUnsecuredApartmentVtbl {
        unsafe extern "system" fn CreateSinkStub<Impl: IWbemUnsecuredApartmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psink: ::windows::core::RawPtr, dwflags: u32, wszreserved: super::super::Foundation::PWSTR, ppstub: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: IUnsecuredApartmentVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), CreateSinkStub: CreateSinkStub::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWbemUnsecuredApartment as ::windows::core::Interface>::IID
    }
}
