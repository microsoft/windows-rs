#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn CompareStringA(locale: u32, dwcmpflags: u32, lpstring1: *const i8, cchcount1: i32, lpstring2: *const i8, cchcount2: i32) -> i32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CompareStringEx(lplocalename: super::Foundation::PWSTR, dwcmpflags: COMPARE_STRING_FLAGS, lpstring1: super::Foundation::PWSTR, cchcount1: i32, lpstring2: super::Foundation::PWSTR, cchcount2: i32, lpversioninformation: *mut NLSVERSIONINFO, lpreserved: *mut ::core::ffi::c_void, lparam: super::Foundation::LPARAM) -> i32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CompareStringOrdinal(lpstring1: super::Foundation::PWSTR, cchcount1: i32, lpstring2: super::Foundation::PWSTR, cchcount2: i32, bignorecase: super::Foundation::BOOL) -> i32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CompareStringW(locale: u32, dwcmpflags: u32, lpstring1: super::Foundation::PWSTR, cchcount1: i32, lpstring2: super::Foundation::PWSTR, cchcount2: i32) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ConvertDefaultLocale(locale: u32) -> u32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumCalendarInfoA(lpcalinfoenumproc: ::windows::runtime::RawPtr, locale: u32, calendar: u32, caltype: u32) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumCalendarInfoExA(lpcalinfoenumprocex: ::windows::runtime::RawPtr, locale: u32, calendar: u32, caltype: u32) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumCalendarInfoExEx(pcalinfoenumprocexex: ::windows::runtime::RawPtr, lplocalename: super::Foundation::PWSTR, calendar: u32, lpreserved: super::Foundation::PWSTR, caltype: u32, lparam: super::Foundation::LPARAM) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumCalendarInfoExW(lpcalinfoenumprocex: ::windows::runtime::RawPtr, locale: u32, calendar: u32, caltype: u32) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumCalendarInfoW(lpcalinfoenumproc: ::windows::runtime::RawPtr, locale: u32, calendar: u32, caltype: u32) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumDateFormatsA(lpdatefmtenumproc: ::windows::runtime::RawPtr, locale: u32, dwflags: u32) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumDateFormatsExA(lpdatefmtenumprocex: ::windows::runtime::RawPtr, locale: u32, dwflags: u32) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumDateFormatsExEx(lpdatefmtenumprocexex: ::windows::runtime::RawPtr, lplocalename: super::Foundation::PWSTR, dwflags: ENUM_DATE_FORMATS_FLAGS, lparam: super::Foundation::LPARAM) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumDateFormatsExW(lpdatefmtenumprocex: ::windows::runtime::RawPtr, locale: u32, dwflags: u32) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumDateFormatsW(lpdatefmtenumproc: ::windows::runtime::RawPtr, locale: u32, dwflags: u32) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumLanguageGroupLocalesA(lplanggrouplocaleenumproc: ::windows::runtime::RawPtr, languagegroup: u32, dwflags: u32, lparam: isize) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumLanguageGroupLocalesW(lplanggrouplocaleenumproc: ::windows::runtime::RawPtr, languagegroup: u32, dwflags: u32, lparam: isize) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumSystemCodePagesA(lpcodepageenumproc: ::windows::runtime::RawPtr, dwflags: ENUM_SYSTEM_CODE_PAGES_FLAGS) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumSystemCodePagesW(lpcodepageenumproc: ::windows::runtime::RawPtr, dwflags: ENUM_SYSTEM_CODE_PAGES_FLAGS) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumSystemGeoID(geoclass: u32, parentgeoid: i32, lpgeoenumproc: ::windows::runtime::RawPtr) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumSystemGeoNames(geoclass: u32, geoenumproc: ::windows::runtime::RawPtr, data: super::Foundation::LPARAM) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumSystemLanguageGroupsA(lplanguagegroupenumproc: ::windows::runtime::RawPtr, dwflags: ENUM_SYSTEM_LANGUAGE_GROUPS_FLAGS, lparam: isize) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumSystemLanguageGroupsW(lplanguagegroupenumproc: ::windows::runtime::RawPtr, dwflags: ENUM_SYSTEM_LANGUAGE_GROUPS_FLAGS, lparam: isize) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumSystemLocalesA(lplocaleenumproc: ::windows::runtime::RawPtr, dwflags: u32) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumSystemLocalesEx(lplocaleenumprocex: ::windows::runtime::RawPtr, dwflags: u32, lparam: super::Foundation::LPARAM, lpreserved: *const ::core::ffi::c_void) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumSystemLocalesW(lplocaleenumproc: ::windows::runtime::RawPtr, dwflags: u32) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumTimeFormatsA(lptimefmtenumproc: ::windows::runtime::RawPtr, locale: u32, dwflags: TIME_FORMAT_FLAGS) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumTimeFormatsEx(lptimefmtenumprocex: ::windows::runtime::RawPtr, lplocalename: super::Foundation::PWSTR, dwflags: u32, lparam: super::Foundation::LPARAM) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumTimeFormatsW(lptimefmtenumproc: ::windows::runtime::RawPtr, locale: u32, dwflags: TIME_FORMAT_FLAGS) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumUILanguagesA(lpuilanguageenumproc: ::windows::runtime::RawPtr, dwflags: u32, lparam: isize) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumUILanguagesW(lpuilanguageenumproc: ::windows::runtime::RawPtr, dwflags: u32, lparam: isize) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindNLSString(locale: u32, dwfindnlsstringflags: u32, lpstringsource: super::Foundation::PWSTR, cchsource: i32, lpstringvalue: super::Foundation::PWSTR, cchvalue: i32, pcchfound: *mut i32) -> i32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindNLSStringEx(lplocalename: super::Foundation::PWSTR, dwfindnlsstringflags: u32, lpstringsource: super::Foundation::PWSTR, cchsource: i32, lpstringvalue: super::Foundation::PWSTR, cchvalue: i32, pcchfound: *mut i32, lpversioninformation: *const NLSVERSIONINFO, lpreserved: *const ::core::ffi::c_void, sorthandle: super::Foundation::LPARAM) -> i32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindStringOrdinal(dwfindstringordinalflags: u32, lpstringsource: super::Foundation::PWSTR, cchsource: i32, lpstringvalue: super::Foundation::PWSTR, cchvalue: i32, bignorecase: super::Foundation::BOOL) -> i32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FoldStringA(dwmapflags: FOLD_STRING_MAP_FLAGS, lpsrcstr: super::Foundation::PSTR, cchsrc: i32, lpdeststr: super::Foundation::PSTR, cchdest: i32) -> i32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FoldStringW(dwmapflags: FOLD_STRING_MAP_FLAGS, lpsrcstr: super::Foundation::PWSTR, cchsrc: i32, lpdeststr: super::Foundation::PWSTR, cchdest: i32) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn GetACP() -> u32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCPInfo(codepage: u32, lpcpinfo: *mut CPINFO) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCPInfoExA(codepage: u32, dwflags: u32, lpcpinfoex: *mut CPINFOEXA) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCPInfoExW(codepage: u32, dwflags: u32, lpcpinfoex: *mut CPINFOEXW) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCalendarInfoA(locale: u32, calendar: u32, caltype: u32, lpcaldata: super::Foundation::PSTR, cchdata: i32, lpvalue: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCalendarInfoEx(lplocalename: super::Foundation::PWSTR, calendar: u32, lpreserved: super::Foundation::PWSTR, caltype: u32, lpcaldata: super::Foundation::PWSTR, cchdata: i32, lpvalue: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCalendarInfoW(locale: u32, calendar: u32, caltype: u32, lpcaldata: super::Foundation::PWSTR, cchdata: i32, lpvalue: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCurrencyFormatA(locale: u32, dwflags: u32, lpvalue: super::Foundation::PSTR, lpformat: *const CURRENCYFMTA, lpcurrencystr: super::Foundation::PSTR, cchcurrency: i32) -> i32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCurrencyFormatEx(lplocalename: super::Foundation::PWSTR, dwflags: u32, lpvalue: super::Foundation::PWSTR, lpformat: *const CURRENCYFMTW, lpcurrencystr: super::Foundation::PWSTR, cchcurrency: i32) -> i32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCurrencyFormatW(locale: u32, dwflags: u32, lpvalue: super::Foundation::PWSTR, lpformat: *const CURRENCYFMTW, lpcurrencystr: super::Foundation::PWSTR, cchcurrency: i32) -> i32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDateFormatA(locale: u32, dwflags: u32, lpdate: *const super::Foundation::SYSTEMTIME, lpformat: super::Foundation::PSTR, lpdatestr: super::Foundation::PSTR, cchdate: i32) -> i32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDateFormatEx(lplocalename: super::Foundation::PWSTR, dwflags: ENUM_DATE_FORMATS_FLAGS, lpdate: *const super::Foundation::SYSTEMTIME, lpformat: super::Foundation::PWSTR, lpdatestr: super::Foundation::PWSTR, cchdate: i32, lpcalendar: super::Foundation::PWSTR) -> i32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDateFormatW(locale: u32, dwflags: u32, lpdate: *const super::Foundation::SYSTEMTIME, lpformat: super::Foundation::PWSTR, lpdatestr: super::Foundation::PWSTR, cchdate: i32) -> i32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDistanceOfClosestLanguageInList(pszlanguage: super::Foundation::PWSTR, pszlanguageslist: super::Foundation::PWSTR, wchlistdelimiter: u16, pclosestdistance: *mut f64) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDurationFormat(locale: u32, dwflags: u32, lpduration: *const super::Foundation::SYSTEMTIME, ullduration: u64, lpformat: super::Foundation::PWSTR, lpdurationstr: super::Foundation::PWSTR, cchduration: i32) -> i32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDurationFormatEx(lplocalename: super::Foundation::PWSTR, dwflags: u32, lpduration: *const super::Foundation::SYSTEMTIME, ullduration: u64, lpformat: super::Foundation::PWSTR, lpdurationstr: super::Foundation::PWSTR, cchduration: i32) -> i32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFileMUIInfo(dwflags: u32, pcwszfilepath: super::Foundation::PWSTR, pfilemuiinfo: *mut FILEMUIINFO, pcbfilemuiinfo: *mut u32) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFileMUIPath(dwflags: u32, pcwszfilepath: super::Foundation::PWSTR, pwszlanguage: super::Foundation::PWSTR, pcchlanguage: *mut u32, pwszfilemuipath: super::Foundation::PWSTR, pcchfilemuipath: *mut u32, pululenumerator: *mut u64) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetGeoInfoA(location: i32, geotype: u32, lpgeodata: super::Foundation::PSTR, cchdata: i32, langid: u16) -> i32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetGeoInfoEx(location: super::Foundation::PWSTR, geotype: u32, geodata: super::Foundation::PWSTR, geodatacount: i32) -> i32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetGeoInfoW(location: i32, geotype: u32, lpgeodata: super::Foundation::PWSTR, cchdata: i32, langid: u16) -> i32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetLocaleInfoA(locale: u32, lctype: u32, lplcdata: super::Foundation::PSTR, cchdata: i32) -> i32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetLocaleInfoEx(lplocalename: super::Foundation::PWSTR, lctype: u32, lplcdata: super::Foundation::PWSTR, cchdata: i32) -> i32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetLocaleInfoW(locale: u32, lctype: u32, lplcdata: super::Foundation::PWSTR, cchdata: i32) -> i32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetNLSVersion(function: u32, locale: u32, lpversioninformation: *mut NLSVERSIONINFO) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetNLSVersionEx(function: u32, lplocalename: super::Foundation::PWSTR, lpversioninformation: *mut NLSVERSIONINFOEX) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetNumberFormatA(locale: u32, dwflags: u32, lpvalue: super::Foundation::PSTR, lpformat: *const NUMBERFMTA, lpnumberstr: super::Foundation::PSTR, cchnumber: i32) -> i32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetNumberFormatEx(lplocalename: super::Foundation::PWSTR, dwflags: u32, lpvalue: super::Foundation::PWSTR, lpformat: *const NUMBERFMTW, lpnumberstr: super::Foundation::PWSTR, cchnumber: i32) -> i32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetNumberFormatW(locale: u32, dwflags: u32, lpvalue: super::Foundation::PWSTR, lpformat: *const NUMBERFMTW, lpnumberstr: super::Foundation::PWSTR, cchnumber: i32) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn GetOEMCP() -> u32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetProcessPreferredUILanguages(dwflags: u32, pulnumlanguages: *mut u32, pwszlanguagesbuffer: super::Foundation::PWSTR, pcchlanguagesbuffer: *mut u32) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetStringScripts(dwflags: u32, lpstring: super::Foundation::PWSTR, cchstring: i32, lpscripts: super::Foundation::PWSTR, cchscripts: i32) -> i32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetStringTypeA(locale: u32, dwinfotype: u32, lpsrcstr: super::Foundation::PSTR, cchsrc: i32, lpchartype: *mut u16) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetStringTypeExA(locale: u32, dwinfotype: u32, lpsrcstr: super::Foundation::PSTR, cchsrc: i32, lpchartype: *mut u16) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetStringTypeExW(locale: u32, dwinfotype: u32, lpsrcstr: super::Foundation::PWSTR, cchsrc: i32, lpchartype: *mut u16) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetStringTypeW(dwinfotype: u32, lpsrcstr: super::Foundation::PWSTR, cchsrc: i32, lpchartype: *mut u16) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn GetSystemDefaultLCID() -> u32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn GetSystemDefaultLangID() -> u16;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetSystemDefaultLocaleName(lplocalename: super::Foundation::PWSTR, cchlocalename: i32) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn GetSystemDefaultUILanguage() -> u16;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetSystemPreferredUILanguages(dwflags: u32, pulnumlanguages: *mut u32, pwszlanguagesbuffer: super::Foundation::PWSTR, pcchlanguagesbuffer: *mut u32) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn GetTextCharset(hdc: super::Graphics::Gdi::HDC) -> i32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn GetTextCharsetInfo(hdc: super::Graphics::Gdi::HDC, lpsig: *mut FONTSIGNATURE, dwflags: u32) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn GetThreadLocale() -> u32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetThreadPreferredUILanguages(dwflags: u32, pulnumlanguages: *mut u32, pwszlanguagesbuffer: super::Foundation::PWSTR, pcchlanguagesbuffer: *mut u32) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn GetThreadUILanguage() -> u16;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetTimeFormatA(locale: u32, dwflags: u32, lptime: *const super::Foundation::SYSTEMTIME, lpformat: super::Foundation::PSTR, lptimestr: super::Foundation::PSTR, cchtime: i32) -> i32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetTimeFormatEx(lplocalename: super::Foundation::PWSTR, dwflags: TIME_FORMAT_FLAGS, lptime: *const super::Foundation::SYSTEMTIME, lpformat: super::Foundation::PWSTR, lptimestr: super::Foundation::PWSTR, cchtime: i32) -> i32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetTimeFormatW(locale: u32, dwflags: u32, lptime: *const super::Foundation::SYSTEMTIME, lpformat: super::Foundation::PWSTR, lptimestr: super::Foundation::PWSTR, cchtime: i32) -> i32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetUILanguageInfo(dwflags: u32, pwmszlanguage: super::Foundation::PWSTR, pwszfallbacklanguages: super::Foundation::PWSTR, pcchfallbacklanguages: *mut u32, pattributes: *mut u32) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetUserDefaultGeoName(geoname: super::Foundation::PWSTR, geonamecount: i32) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn GetUserDefaultLCID() -> u32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn GetUserDefaultLangID() -> u16;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetUserDefaultLocaleName(lplocalename: super::Foundation::PWSTR, cchlocalename: i32) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn GetUserDefaultUILanguage() -> u16;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn GetUserGeoID(geoclass: u32) -> i32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetUserPreferredUILanguages(dwflags: u32, pulnumlanguages: *mut u32, pwszlanguagesbuffer: super::Foundation::PWSTR, pcchlanguagesbuffer: *mut u32) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IdnToAscii(dwflags: u32, lpunicodecharstr: super::Foundation::PWSTR, cchunicodechar: i32, lpasciicharstr: super::Foundation::PWSTR, cchasciichar: i32) -> i32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IdnToNameprepUnicode(dwflags: u32, lpunicodecharstr: super::Foundation::PWSTR, cchunicodechar: i32, lpnameprepcharstr: super::Foundation::PWSTR, cchnameprepchar: i32) -> i32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IdnToUnicode(dwflags: u32, lpasciicharstr: super::Foundation::PWSTR, cchasciichar: i32, lpunicodecharstr: super::Foundation::PWSTR, cchunicodechar: i32) -> i32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsDBCSLeadByte(testchar: u8) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsDBCSLeadByteEx(codepage: u32, testchar: u8) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsNLSDefinedString(function: u32, dwflags: u32, lpversioninformation: *const NLSVERSIONINFO, lpstring: super::Foundation::PWSTR, cchstr: i32) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsNormalizedString(normform: NORM_FORM, lpstring: super::Foundation::PWSTR, cwlength: i32) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsTextUnicode(lpv: *const ::core::ffi::c_void, isize: i32, lpiresult: *mut IS_TEXT_UNICODE_RESULT) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsValidCodePage(codepage: u32) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsValidLanguageGroup(languagegroup: u32, dwflags: ENUM_SYSTEM_LANGUAGE_GROUPS_FLAGS) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsValidLocale(locale: u32, dwflags: IS_VALID_LOCALE_FLAGS) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsValidLocaleName(lplocalename: super::Foundation::PWSTR) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsValidNLSVersion(function: u32, lplocalename: super::Foundation::PWSTR, lpversioninformation: *const NLSVERSIONINFOEX) -> u32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsWellFormedTag(psztag: super::Foundation::PWSTR) -> u8;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LCIDToLocaleName(locale: u32, lpname: super::Foundation::PWSTR, cchname: i32, dwflags: u32) -> i32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LCMapStringA(locale: u32, dwmapflags: u32, lpsrcstr: super::Foundation::PSTR, cchsrc: i32, lpdeststr: super::Foundation::PSTR, cchdest: i32) -> i32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LCMapStringEx(lplocalename: super::Foundation::PWSTR, dwmapflags: u32, lpsrcstr: super::Foundation::PWSTR, cchsrc: i32, lpdeststr: super::Foundation::PWSTR, cchdest: i32, lpversioninformation: *const NLSVERSIONINFO, lpreserved: *const ::core::ffi::c_void, sorthandle: super::Foundation::LPARAM) -> i32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LCMapStringW(locale: u32, dwmapflags: u32, lpsrcstr: super::Foundation::PWSTR, cchsrc: i32, lpdeststr: super::Foundation::PWSTR, cchdest: i32) -> i32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LocaleNameToLCID(lpname: super::Foundation::PWSTR, dwflags: u32) -> u32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MappingDoAction(pbag: *mut MAPPING_PROPERTY_BAG, dwrangeindex: u32, pszactionid: super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MappingFreePropertyBag(pbag: *const MAPPING_PROPERTY_BAG) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MappingFreeServices(pserviceinfo: *const MAPPING_SERVICE_INFO) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MappingGetServices(poptions: *const MAPPING_ENUM_OPTIONS, prgservices: *mut *mut MAPPING_SERVICE_INFO, pdwservicescount: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MappingRecognizeText(pserviceinfo: *const MAPPING_SERVICE_INFO, psztext: super::Foundation::PWSTR, dwlength: u32, dwindex: u32, poptions: *const ::core::mem::ManuallyDrop<MAPPING_OPTIONS>, pbag: *mut MAPPING_PROPERTY_BAG) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MultiByteToWideChar(codepage: u32, dwflags: MULTI_BYTE_TO_WIDE_CHAR_FLAGS, lpmultibytestr: super::Foundation::PSTR, cbmultibyte: i32, lpwidecharstr: super::Foundation::PWSTR, cchwidechar: i32) -> i32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NormalizeString(normform: NORM_FORM, lpsrcstring: super::Foundation::PWSTR, cwsrclength: i32, lpdststring: super::Foundation::PWSTR, cwdstlength: i32) -> i32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NotifyUILanguageChange(dwflags: u32, pcwstrnewlanguage: super::Foundation::PWSTR, pcwstrpreviouslanguage: super::Foundation::PWSTR, dwreserved: u32, pdwstatusrtrn: *mut u32) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResolveLocaleName(lpnametoresolve: super::Foundation::PWSTR, lplocalename: super::Foundation::PWSTR, cchlocalename: i32) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn RestoreThreadPreferredUILanguages(snapshot: HSAVEDUILANGUAGES);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ScriptApplyDigitSubstitution(psds: *const SCRIPT_DIGITSUBSTITUTE, psc: *mut SCRIPT_CONTROL, pss: *mut SCRIPT_STATE) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn ScriptApplyLogicalWidth(pidx: *const i32, cchars: i32, cglyphs: i32, pwlogclust: *const u16, psva: *const SCRIPT_VISATTR, piadvance: *const i32, psa: *const SCRIPT_ANALYSIS, pabc: *mut super::Graphics::Gdi::ABC, pijustify: *mut i32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ScriptBreak(pwcchars: super::Foundation::PWSTR, cchars: i32, psa: *const SCRIPT_ANALYSIS, psla: *mut SCRIPT_LOGATTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ScriptCPtoX(icp: i32, ftrailing: super::Foundation::BOOL, cchars: i32, cglyphs: i32, pwlogclust: *const u16, psva: *const SCRIPT_VISATTR, piadvance: *const i32, psa: *const SCRIPT_ANALYSIS, pix: *mut i32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn ScriptCacheGetHeight(hdc: super::Graphics::Gdi::HDC, psc: *mut *mut ::core::ffi::c_void, tmheight: *mut i32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ScriptFreeCache(psc: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn ScriptGetCMap(hdc: super::Graphics::Gdi::HDC, psc: *mut *mut ::core::ffi::c_void, pwcinchars: super::Foundation::PWSTR, cchars: i32, dwflags: u32, pwoutglyphs: *mut u16) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn ScriptGetFontAlternateGlyphs(hdc: super::Graphics::Gdi::HDC, psc: *mut *mut ::core::ffi::c_void, psa: *const SCRIPT_ANALYSIS, tagscript: u32, taglangsys: u32, tagfeature: u32, wglyphid: u16, cmaxalternates: i32, palternateglyphs: *mut u16, pcalternates: *mut i32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn ScriptGetFontFeatureTags(hdc: super::Graphics::Gdi::HDC, psc: *mut *mut ::core::ffi::c_void, psa: *const SCRIPT_ANALYSIS, tagscript: u32, taglangsys: u32, cmaxtags: i32, pfeaturetags: *mut u32, pctags: *mut i32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn ScriptGetFontLanguageTags(hdc: super::Graphics::Gdi::HDC, psc: *mut *mut ::core::ffi::c_void, psa: *const SCRIPT_ANALYSIS, tagscript: u32, cmaxtags: i32, plangsystags: *mut u32, pctags: *mut i32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn ScriptGetFontProperties(hdc: super::Graphics::Gdi::HDC, psc: *mut *mut ::core::ffi::c_void, sfp: *mut SCRIPT_FONTPROPERTIES) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn ScriptGetFontScriptTags(hdc: super::Graphics::Gdi::HDC, psc: *mut *mut ::core::ffi::c_void, psa: *const SCRIPT_ANALYSIS, cmaxtags: i32, pscripttags: *mut u32, pctags: *mut i32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn ScriptGetGlyphABCWidth(hdc: super::Graphics::Gdi::HDC, psc: *mut *mut ::core::ffi::c_void, wglyph: u16, pabc: *mut super::Graphics::Gdi::ABC) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ScriptGetLogicalWidths(psa: *const SCRIPT_ANALYSIS, cchars: i32, cglyphs: i32, piglyphwidth: *const i32, pwlogclust: *const u16, psva: *const SCRIPT_VISATTR, pidx: *const i32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ScriptGetProperties(ppsp: *mut *mut *mut SCRIPT_PROPERTIES, pinumscripts: *mut i32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ScriptIsComplex(pwcinchars: super::Foundation::PWSTR, cinchars: i32, dwflags: SCRIPT_IS_COMPLEX_FLAGS) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ScriptItemize(pwcinchars: super::Foundation::PWSTR, cinchars: i32, cmaxitems: i32, pscontrol: *const SCRIPT_CONTROL, psstate: *const SCRIPT_STATE, pitems: *mut SCRIPT_ITEM, pcitems: *mut i32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ScriptItemizeOpenType(pwcinchars: super::Foundation::PWSTR, cinchars: i32, cmaxitems: i32, pscontrol: *const SCRIPT_CONTROL, psstate: *const SCRIPT_STATE, pitems: *mut SCRIPT_ITEM, pscripttags: *mut u32, pcitems: *mut i32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ScriptJustify(psva: *const SCRIPT_VISATTR, piadvance: *const i32, cglyphs: i32, idx: i32, iminkashida: i32, pijustify: *mut i32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ScriptLayout(cruns: i32, pblevel: *const u8, pivisualtological: *mut i32, pilogicaltovisual: *mut i32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn ScriptPlace(hdc: super::Graphics::Gdi::HDC, psc: *mut *mut ::core::ffi::c_void, pwglyphs: *const u16, cglyphs: i32, psva: *const SCRIPT_VISATTR, psa: *mut SCRIPT_ANALYSIS, piadvance: *mut i32, pgoffset: *mut GOFFSET, pabc: *mut super::Graphics::Gdi::ABC) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn ScriptPlaceOpenType(
        hdc: super::Graphics::Gdi::HDC,
        psc: *mut *mut ::core::ffi::c_void,
        psa: *mut SCRIPT_ANALYSIS,
        tagscript: u32,
        taglangsys: u32,
        rcrangechars: *const i32,
        rprangeproperties: *const *const textrange_properties,
        cranges: i32,
        pwcchars: super::Foundation::PWSTR,
        pwlogclust: *const u16,
        pcharprops: *const script_charprop,
        cchars: i32,
        pwglyphs: *const u16,
        pglyphprops: *const script_glyphprop,
        cglyphs: i32,
        piadvance: *mut i32,
        pgoffset: *mut GOFFSET,
        pabc: *mut super::Graphics::Gdi::ABC,
    ) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn ScriptPositionSingleGlyph(hdc: super::Graphics::Gdi::HDC, psc: *mut *mut ::core::ffi::c_void, psa: *const SCRIPT_ANALYSIS, tagscript: u32, taglangsys: u32, tagfeature: u32, lparameter: i32, wglyphid: u16, iadvance: i32, goffset: GOFFSET, pioutadvance: *mut i32, poutgoffset: *mut GOFFSET) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ScriptRecordDigitSubstitution(locale: u32, psds: *mut SCRIPT_DIGITSUBSTITUTE) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn ScriptShape(hdc: super::Graphics::Gdi::HDC, psc: *mut *mut ::core::ffi::c_void, pwcchars: super::Foundation::PWSTR, cchars: i32, cmaxglyphs: i32, psa: *mut SCRIPT_ANALYSIS, pwoutglyphs: *mut u16, pwlogclust: *mut u16, psva: *mut SCRIPT_VISATTR, pcglyphs: *mut i32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn ScriptShapeOpenType(hdc: super::Graphics::Gdi::HDC, psc: *mut *mut ::core::ffi::c_void, psa: *mut SCRIPT_ANALYSIS, tagscript: u32, taglangsys: u32, rcrangechars: *const i32, rprangeproperties: *const *const textrange_properties, cranges: i32, pwcchars: super::Foundation::PWSTR, cchars: i32, cmaxglyphs: i32, pwlogclust: *mut u16, pcharprops: *mut script_charprop, pwoutglyphs: *mut u16, poutglyphprops: *mut script_glyphprop, pcglyphs: *mut i32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn ScriptStringAnalyse(hdc: super::Graphics::Gdi::HDC, pstring: *const ::core::ffi::c_void, cstring: i32, cglyphs: i32, icharset: i32, dwflags: u32, ireqwidth: i32, pscontrol: *const SCRIPT_CONTROL, psstate: *const SCRIPT_STATE, pidx: *const i32, ptabdef: *const SCRIPT_TABDEF, pbinclass: *const u8, pssa: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ScriptStringCPtoX(ssa: *const ::core::ffi::c_void, icp: i32, ftrailing: super::Foundation::BOOL, px: *mut i32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ScriptStringFree(pssa: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ScriptStringGetLogicalWidths(ssa: *const ::core::ffi::c_void, pidx: *mut i32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ScriptStringGetOrder(ssa: *const ::core::ffi::c_void, puorder: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn ScriptStringOut(ssa: *const ::core::ffi::c_void, ix: i32, iy: i32, uoptions: super::Graphics::Gdi::ETO_OPTIONS, prc: *const super::Foundation::RECT, iminsel: i32, imaxsel: i32, fdisabled: super::Foundation::BOOL) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ScriptStringValidate(ssa: *const ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ScriptStringXtoCP(ssa: *const ::core::ffi::c_void, ix: i32, pich: *mut i32, pitrailing: *mut i32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ScriptString_pLogAttr(ssa: *const ::core::ffi::c_void) -> *mut SCRIPT_LOGATTR;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ScriptString_pSize(ssa: *const ::core::ffi::c_void) -> *mut super::Foundation::SIZE;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ScriptString_pcOutChars(ssa: *const ::core::ffi::c_void) -> *mut i32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn ScriptSubstituteSingleGlyph(hdc: super::Graphics::Gdi::HDC, psc: *mut *mut ::core::ffi::c_void, psa: *const SCRIPT_ANALYSIS, tagscript: u32, taglangsys: u32, tagfeature: u32, lparameter: i32, wglyphid: u16, pwoutglyphid: *mut u16) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn ScriptTextOut(hdc: super::Graphics::Gdi::HDC, psc: *mut *mut ::core::ffi::c_void, x: i32, y: i32, fuoptions: u32, lprc: *const super::Foundation::RECT, psa: *const SCRIPT_ANALYSIS, pwcreserved: super::Foundation::PWSTR, ireserved: i32, pwglyphs: *const u16, cglyphs: i32, piadvance: *const i32, pijustify: *const i32, pgoffset: *const GOFFSET) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ScriptXtoCP(ix: i32, cchars: i32, cglyphs: i32, pwlogclust: *const u16, psva: *const SCRIPT_VISATTR, piadvance: *const i32, psa: *const SCRIPT_ANALYSIS, picp: *mut i32, pitrailing: *mut i32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetCalendarInfoA(locale: u32, calendar: u32, caltype: u32, lpcaldata: super::Foundation::PSTR) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetCalendarInfoW(locale: u32, calendar: u32, caltype: u32, lpcaldata: super::Foundation::PWSTR) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetLocaleInfoA(locale: u32, lctype: u32, lplcdata: super::Foundation::PSTR) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetLocaleInfoW(locale: u32, lctype: u32, lplcdata: super::Foundation::PWSTR) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetProcessPreferredUILanguages(dwflags: u32, pwszlanguagesbuffer: super::Foundation::PWSTR, pulnumlanguages: *mut u32) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetThreadLocale(locale: u32) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetThreadPreferredUILanguages(dwflags: u32, pwszlanguagesbuffer: super::Foundation::PWSTR, pulnumlanguages: *mut u32) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetThreadPreferredUILanguages2(flags: u32, languages: super::Foundation::PWSTR, numlanguagesset: *mut u32, snapshot: *mut HSAVEDUILANGUAGES) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn SetThreadUILanguage(langid: u16) -> u16;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetUserGeoID(geoid: i32) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetUserGeoName(geoname: super::Foundation::PWSTR) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TranslateCharsetInfo(lpsrc: *mut u32, lpcs: *mut CHARSETINFO, dwflags: TRANSLATE_CHARSET_INFO_FLAGS) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UCNV_FROM_U_CALLBACK_ESCAPE(context: *const ::core::ffi::c_void, fromuargs: *mut UConverterFromUnicodeArgs, codeunits: *const u16, length: i32, codepoint: i32, reason: UConverterCallbackReason, err: *mut UErrorCode);
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UCNV_FROM_U_CALLBACK_SKIP(context: *const ::core::ffi::c_void, fromuargs: *mut UConverterFromUnicodeArgs, codeunits: *const u16, length: i32, codepoint: i32, reason: UConverterCallbackReason, err: *mut UErrorCode);
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UCNV_FROM_U_CALLBACK_STOP(context: *const ::core::ffi::c_void, fromuargs: *mut UConverterFromUnicodeArgs, codeunits: *const u16, length: i32, codepoint: i32, reason: UConverterCallbackReason, err: *mut UErrorCode);
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UCNV_FROM_U_CALLBACK_SUBSTITUTE(context: *const ::core::ffi::c_void, fromuargs: *mut UConverterFromUnicodeArgs, codeunits: *const u16, length: i32, codepoint: i32, reason: UConverterCallbackReason, err: *mut UErrorCode);
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UCNV_TO_U_CALLBACK_ESCAPE(context: *const ::core::ffi::c_void, touargs: *mut UConverterToUnicodeArgs, codeunits: super::Foundation::PSTR, length: i32, reason: UConverterCallbackReason, err: *mut UErrorCode);
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UCNV_TO_U_CALLBACK_SKIP(context: *const ::core::ffi::c_void, touargs: *mut UConverterToUnicodeArgs, codeunits: super::Foundation::PSTR, length: i32, reason: UConverterCallbackReason, err: *mut UErrorCode);
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UCNV_TO_U_CALLBACK_STOP(context: *const ::core::ffi::c_void, touargs: *mut UConverterToUnicodeArgs, codeunits: super::Foundation::PSTR, length: i32, reason: UConverterCallbackReason, err: *mut UErrorCode);
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UCNV_TO_U_CALLBACK_SUBSTITUTE(context: *const ::core::ffi::c_void, touargs: *mut UConverterToUnicodeArgs, codeunits: super::Foundation::PSTR, length: i32, reason: UConverterCallbackReason, err: *mut UErrorCode);
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VerifyScripts(dwflags: u32, lplocalescripts: super::Foundation::PWSTR, cchlocalescripts: i32, lptestscripts: super::Foundation::PWSTR, cchtestscripts: i32) -> super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WideCharToMultiByte(codepage: u32, dwflags: u32, lpwidecharstr: super::Foundation::PWSTR, cchwidechar: i32, lpmultibytestr: super::Foundation::PSTR, cbmultibyte: i32, lpdefaultchar: super::Foundation::PSTR, lpuseddefaultchar: *mut i32) -> i32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lstrcatA(lpstring1: super::Foundation::PSTR, lpstring2: super::Foundation::PSTR) -> super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lstrcatW(lpstring1: super::Foundation::PWSTR, lpstring2: super::Foundation::PWSTR) -> super::Foundation::PWSTR;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lstrcmpA(lpstring1: super::Foundation::PSTR, lpstring2: super::Foundation::PSTR) -> i32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lstrcmpW(lpstring1: super::Foundation::PWSTR, lpstring2: super::Foundation::PWSTR) -> i32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lstrcmpiA(lpstring1: super::Foundation::PSTR, lpstring2: super::Foundation::PSTR) -> i32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lstrcmpiW(lpstring1: super::Foundation::PWSTR, lpstring2: super::Foundation::PWSTR) -> i32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lstrcpyA(lpstring1: super::Foundation::PSTR, lpstring2: super::Foundation::PSTR) -> super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lstrcpyW(lpstring1: super::Foundation::PWSTR, lpstring2: super::Foundation::PWSTR) -> super::Foundation::PWSTR;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lstrcpynA(lpstring1: super::Foundation::PSTR, lpstring2: super::Foundation::PSTR, imaxlength: i32) -> super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lstrcpynW(lpstring1: super::Foundation::PWSTR, lpstring2: super::Foundation::PWSTR, imaxlength: i32) -> super::Foundation::PWSTR;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lstrlenA(lpstring: super::Foundation::PSTR) -> i32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lstrlenW(lpstring: super::Foundation::PWSTR) -> i32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_UCharsToChars(us: *const u16, cs: super::Foundation::PSTR, length: i32);
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_austrcpy(dst: super::Foundation::PSTR, src: *const u16) -> super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_austrncpy(dst: super::Foundation::PSTR, src: *const u16, n: i32) -> super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_catclose(catd: *mut UResourceBundle);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_catgets(catd: *mut UResourceBundle, set_num: i32, msg_num: i32, s: *const u16, len: *mut i32, ec: *mut UErrorCode) -> *mut u16;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_catopen(name: super::Foundation::PSTR, locale: super::Foundation::PSTR, ec: *mut UErrorCode) -> *mut UResourceBundle;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_charAge(c: i32, versionarray: *mut u8);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_charDigitValue(c: i32) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_charDirection(c: i32) -> UCharDirection;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_charFromName(namechoice: UCharNameChoice, name: super::Foundation::PSTR, perrorcode: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_charMirror(c: i32) -> i32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_charName(code: i32, namechoice: UCharNameChoice, buffer: super::Foundation::PSTR, bufferlength: i32, perrorcode: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_charType(c: i32) -> i8;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_charsToUChars(cs: super::Foundation::PSTR, us: *mut u16, length: i32);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_cleanup();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_countChar32(s: *const u16, length: i32) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_digit(ch: i32, radix: i8) -> i32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_enumCharNames(start: i32, limit: i32, r#fn: *mut ::windows::runtime::RawPtr, context: *mut ::core::ffi::c_void, namechoice: UCharNameChoice, perrorcode: *mut UErrorCode);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_enumCharTypes(enumrange: *mut ::windows::runtime::RawPtr, context: *const ::core::ffi::c_void);
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_errorName(code: UErrorCode) -> super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_foldCase(c: i32, options: u32) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_forDigit(digit: i32, radix: i8) -> i32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_formatMessage(locale: super::Foundation::PSTR, pattern: *const u16, patternlength: i32, result: *mut u16, resultlength: i32, status: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_formatMessageWithError(locale: super::Foundation::PSTR, pattern: *const u16, patternlength: i32, result: *mut u16, resultlength: i32, parseerror: *mut UParseError, status: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_getBidiPairedBracket(c: i32) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_getBinaryPropertySet(property: UProperty, perrorcode: *mut UErrorCode) -> *mut USet;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_getCombiningClass(c: i32) -> u8;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_getDataVersion(dataversionfillin: *mut u8, status: *mut UErrorCode);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_getFC_NFKC_Closure(c: i32, dest: *mut u16, destcapacity: i32, perrorcode: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_getIntPropertyMap(property: UProperty, perrorcode: *mut UErrorCode) -> *mut UCPMap;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_getIntPropertyMaxValue(which: UProperty) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_getIntPropertyMinValue(which: UProperty) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_getIntPropertyValue(c: i32, which: UProperty) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_getNumericValue(c: i32) -> f64;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_getPropertyEnum(alias: super::Foundation::PSTR) -> UProperty;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_getPropertyName(property: UProperty, namechoice: UPropertyNameChoice) -> super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_getPropertyValueEnum(property: UProperty, alias: super::Foundation::PSTR) -> i32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_getPropertyValueName(property: UProperty, value: i32, namechoice: UPropertyNameChoice) -> super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_getUnicodeVersion(versionarray: *mut u8);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_getVersion(versionarray: *mut u8);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_hasBinaryProperty(c: i32, which: UProperty) -> i8;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_init(status: *mut UErrorCode);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_isIDIgnorable(c: i32) -> i8;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_isIDPart(c: i32) -> i8;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_isIDStart(c: i32) -> i8;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_isISOControl(c: i32) -> i8;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_isJavaIDPart(c: i32) -> i8;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_isJavaIDStart(c: i32) -> i8;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_isJavaSpaceChar(c: i32) -> i8;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_isMirrored(c: i32) -> i8;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_isUAlphabetic(c: i32) -> i8;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_isULowercase(c: i32) -> i8;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_isUUppercase(c: i32) -> i8;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_isUWhiteSpace(c: i32) -> i8;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_isWhitespace(c: i32) -> i8;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_isalnum(c: i32) -> i8;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_isalpha(c: i32) -> i8;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_isbase(c: i32) -> i8;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_isblank(c: i32) -> i8;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_iscntrl(c: i32) -> i8;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_isdefined(c: i32) -> i8;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_isdigit(c: i32) -> i8;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_isgraph(c: i32) -> i8;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_islower(c: i32) -> i8;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_isprint(c: i32) -> i8;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_ispunct(c: i32) -> i8;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_isspace(c: i32) -> i8;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_istitle(c: i32) -> i8;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_isupper(c: i32) -> i8;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_isxdigit(c: i32) -> i8;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_memcasecmp(s1: *const u16, s2: *const u16, length: i32, options: u32) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_memchr(s: *const u16, c: u16, count: i32) -> *mut u16;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_memchr32(s: *const u16, c: i32, count: i32) -> *mut u16;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_memcmp(buf1: *const u16, buf2: *const u16, count: i32) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_memcmpCodePointOrder(s1: *const u16, s2: *const u16, count: i32) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_memcpy(dest: *mut u16, src: *const u16, count: i32) -> *mut u16;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_memmove(dest: *mut u16, src: *const u16, count: i32) -> *mut u16;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_memrchr(s: *const u16, c: u16, count: i32) -> *mut u16;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_memrchr32(s: *const u16, c: i32, count: i32) -> *mut u16;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_memset(dest: *mut u16, c: u16, count: i32) -> *mut u16;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_parseMessage(locale: super::Foundation::PSTR, pattern: *const u16, patternlength: i32, source: *const u16, sourcelength: i32, status: *mut UErrorCode);
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_parseMessageWithError(locale: super::Foundation::PSTR, pattern: *const u16, patternlength: i32, source: *const u16, sourcelength: i32, parseerror: *mut UParseError, status: *mut UErrorCode);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_setMemoryFunctions(context: *const ::core::ffi::c_void, a: *mut ::windows::runtime::RawPtr, r: *mut ::windows::runtime::RawPtr, f: *mut ::windows::runtime::RawPtr, status: *mut UErrorCode);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_shapeArabic(source: *const u16, sourcelength: i32, dest: *mut u16, destsize: i32, options: u32, perrorcode: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_strCaseCompare(s1: *const u16, length1: i32, s2: *const u16, length2: i32, options: u32, perrorcode: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_strCompare(s1: *const u16, length1: i32, s2: *const u16, length2: i32, codepointorder: i8) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_strCompareIter(iter1: *mut ::core::mem::ManuallyDrop<UCharIterator>, iter2: *mut ::core::mem::ManuallyDrop<UCharIterator>, codepointorder: i8) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_strFindFirst(s: *const u16, length: i32, substring: *const u16, sublength: i32) -> *mut u16;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_strFindLast(s: *const u16, length: i32, substring: *const u16, sublength: i32) -> *mut u16;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_strFoldCase(dest: *mut u16, destcapacity: i32, src: *const u16, srclength: i32, options: u32, perrorcode: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_strFromJavaModifiedUTF8WithSub(dest: *mut u16, destcapacity: i32, pdestlength: *mut i32, src: super::Foundation::PSTR, srclength: i32, subchar: i32, pnumsubstitutions: *mut i32, perrorcode: *mut UErrorCode) -> *mut u16;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_strFromUTF32(dest: *mut u16, destcapacity: i32, pdestlength: *mut i32, src: *const i32, srclength: i32, perrorcode: *mut UErrorCode) -> *mut u16;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_strFromUTF32WithSub(dest: *mut u16, destcapacity: i32, pdestlength: *mut i32, src: *const i32, srclength: i32, subchar: i32, pnumsubstitutions: *mut i32, perrorcode: *mut UErrorCode) -> *mut u16;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_strFromUTF8(dest: *mut u16, destcapacity: i32, pdestlength: *mut i32, src: super::Foundation::PSTR, srclength: i32, perrorcode: *mut UErrorCode) -> *mut u16;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_strFromUTF8Lenient(dest: *mut u16, destcapacity: i32, pdestlength: *mut i32, src: super::Foundation::PSTR, srclength: i32, perrorcode: *mut UErrorCode) -> *mut u16;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_strFromUTF8WithSub(dest: *mut u16, destcapacity: i32, pdestlength: *mut i32, src: super::Foundation::PSTR, srclength: i32, subchar: i32, pnumsubstitutions: *mut i32, perrorcode: *mut UErrorCode) -> *mut u16;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_strFromWCS(dest: *mut u16, destcapacity: i32, pdestlength: *mut i32, src: super::Foundation::PWSTR, srclength: i32, perrorcode: *mut UErrorCode) -> *mut u16;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_strHasMoreChar32Than(s: *const u16, length: i32, number: i32) -> i8;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_strToJavaModifiedUTF8(dest: super::Foundation::PSTR, destcapacity: i32, pdestlength: *mut i32, src: *const u16, srclength: i32, perrorcode: *mut UErrorCode) -> super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_strToLower(dest: *mut u16, destcapacity: i32, src: *const u16, srclength: i32, locale: super::Foundation::PSTR, perrorcode: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_strToTitle(dest: *mut u16, destcapacity: i32, src: *const u16, srclength: i32, titleiter: *mut UBreakIterator, locale: super::Foundation::PSTR, perrorcode: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_strToUTF32(dest: *mut i32, destcapacity: i32, pdestlength: *mut i32, src: *const u16, srclength: i32, perrorcode: *mut UErrorCode) -> *mut i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_strToUTF32WithSub(dest: *mut i32, destcapacity: i32, pdestlength: *mut i32, src: *const u16, srclength: i32, subchar: i32, pnumsubstitutions: *mut i32, perrorcode: *mut UErrorCode) -> *mut i32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_strToUTF8(dest: super::Foundation::PSTR, destcapacity: i32, pdestlength: *mut i32, src: *const u16, srclength: i32, perrorcode: *mut UErrorCode) -> super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_strToUTF8WithSub(dest: super::Foundation::PSTR, destcapacity: i32, pdestlength: *mut i32, src: *const u16, srclength: i32, subchar: i32, pnumsubstitutions: *mut i32, perrorcode: *mut UErrorCode) -> super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_strToUpper(dest: *mut u16, destcapacity: i32, src: *const u16, srclength: i32, locale: super::Foundation::PSTR, perrorcode: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_strToWCS(dest: super::Foundation::PWSTR, destcapacity: i32, pdestlength: *mut i32, src: *const u16, srclength: i32, perrorcode: *mut UErrorCode) -> super::Foundation::PWSTR;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_strcasecmp(s1: *const u16, s2: *const u16, options: u32) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_strcat(dst: *mut u16, src: *const u16) -> *mut u16;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_strchr(s: *const u16, c: u16) -> *mut u16;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_strchr32(s: *const u16, c: i32) -> *mut u16;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_strcmp(s1: *const u16, s2: *const u16) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_strcmpCodePointOrder(s1: *const u16, s2: *const u16) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_strcpy(dst: *mut u16, src: *const u16) -> *mut u16;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_strcspn(string: *const u16, matchset: *const u16) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_strlen(s: *const u16) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_strncasecmp(s1: *const u16, s2: *const u16, n: i32, options: u32) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_strncat(dst: *mut u16, src: *const u16, n: i32) -> *mut u16;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_strncmp(ucs1: *const u16, ucs2: *const u16, n: i32) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_strncmpCodePointOrder(s1: *const u16, s2: *const u16, n: i32) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_strncpy(dst: *mut u16, src: *const u16, n: i32) -> *mut u16;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_strpbrk(string: *const u16, matchset: *const u16) -> *mut u16;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_strrchr(s: *const u16, c: u16) -> *mut u16;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_strrchr32(s: *const u16, c: i32) -> *mut u16;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_strrstr(s: *const u16, substring: *const u16) -> *mut u16;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_strspn(string: *const u16, matchset: *const u16) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_strstr(s: *const u16, substring: *const u16) -> *mut u16;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_strtok_r(src: *mut u16, delim: *const u16, savestate: *mut *mut u16) -> *mut u16;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_tolower(c: i32) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_totitle(c: i32) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_toupper(c: i32) -> i32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_uastrcpy(dst: *mut u16, src: super::Foundation::PSTR) -> *mut u16;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_uastrncpy(dst: *mut u16, src: super::Foundation::PSTR, n: i32) -> *mut u16;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_unescape(src: super::Foundation::PSTR, dest: *mut u16, destcapacity: i32) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_unescapeAt(charat: ::windows::runtime::RawPtr, offset: *mut i32, length: i32, context: *mut ::core::ffi::c_void) -> i32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_versionFromString(versionarray: *mut u8, versionstring: super::Foundation::PSTR);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_versionFromUString(versionarray: *mut u8, versionstring: *const u16);
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_versionToString(versionarray: *const u8, versionstring: super::Foundation::PSTR);
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_vformatMessage(locale: super::Foundation::PSTR, pattern: *const u16, patternlength: i32, result: *mut u16, resultlength: i32, ap: *mut i8, status: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_vformatMessageWithError(locale: super::Foundation::PSTR, pattern: *const u16, patternlength: i32, result: *mut u16, resultlength: i32, parseerror: *mut UParseError, ap: *mut i8, status: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_vparseMessage(locale: super::Foundation::PSTR, pattern: *const u16, patternlength: i32, source: *const u16, sourcelength: i32, ap: *mut i8, status: *mut UErrorCode);
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_vparseMessageWithError(locale: super::Foundation::PSTR, pattern: *const u16, patternlength: i32, source: *const u16, sourcelength: i32, ap: *mut i8, parseerror: *mut UParseError, status: *mut UErrorCode);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ubidi_close(pbidi: *mut UBiDi);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ubidi_countParagraphs(pbidi: *mut UBiDi) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ubidi_countRuns(pbidi: *mut UBiDi, perrorcode: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ubidi_getBaseDirection(text: *const u16, length: i32) -> UBiDiDirection;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ubidi_getClassCallback(pbidi: *mut UBiDi, r#fn: *mut ::windows::runtime::RawPtr, context: *const *const ::core::ffi::c_void);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ubidi_getCustomizedClass(pbidi: *mut UBiDi, c: i32) -> UCharDirection;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ubidi_getDirection(pbidi: *const UBiDi) -> UBiDiDirection;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ubidi_getLength(pbidi: *const UBiDi) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ubidi_getLevelAt(pbidi: *const UBiDi, charindex: i32) -> u8;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ubidi_getLevels(pbidi: *mut UBiDi, perrorcode: *mut UErrorCode) -> *mut u8;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ubidi_getLogicalIndex(pbidi: *mut UBiDi, visualindex: i32, perrorcode: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ubidi_getLogicalMap(pbidi: *mut UBiDi, indexmap: *mut i32, perrorcode: *mut UErrorCode);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ubidi_getLogicalRun(pbidi: *const UBiDi, logicalposition: i32, plogicallimit: *mut i32, plevel: *mut u8);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ubidi_getParaLevel(pbidi: *const UBiDi) -> u8;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ubidi_getParagraph(pbidi: *const UBiDi, charindex: i32, pparastart: *mut i32, pparalimit: *mut i32, pparalevel: *mut u8, perrorcode: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ubidi_getParagraphByIndex(pbidi: *const UBiDi, paraindex: i32, pparastart: *mut i32, pparalimit: *mut i32, pparalevel: *mut u8, perrorcode: *mut UErrorCode);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ubidi_getProcessedLength(pbidi: *const UBiDi) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ubidi_getReorderingMode(pbidi: *mut UBiDi) -> UBiDiReorderingMode;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ubidi_getReorderingOptions(pbidi: *mut UBiDi) -> u32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ubidi_getResultLength(pbidi: *const UBiDi) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ubidi_getText(pbidi: *const UBiDi) -> *mut u16;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ubidi_getVisualIndex(pbidi: *mut UBiDi, logicalindex: i32, perrorcode: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ubidi_getVisualMap(pbidi: *mut UBiDi, indexmap: *mut i32, perrorcode: *mut UErrorCode);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ubidi_getVisualRun(pbidi: *mut UBiDi, runindex: i32, plogicalstart: *mut i32, plength: *mut i32) -> UBiDiDirection;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ubidi_invertMap(srcmap: *const i32, destmap: *mut i32, length: i32);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ubidi_isInverse(pbidi: *mut UBiDi) -> i8;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ubidi_isOrderParagraphsLTR(pbidi: *mut UBiDi) -> i8;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ubidi_open() -> *mut UBiDi;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ubidi_openSized(maxlength: i32, maxruncount: i32, perrorcode: *mut UErrorCode) -> *mut UBiDi;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ubidi_orderParagraphsLTR(pbidi: *mut UBiDi, orderparagraphsltr: i8);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ubidi_reorderLogical(levels: *const u8, length: i32, indexmap: *mut i32);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ubidi_reorderVisual(levels: *const u8, length: i32, indexmap: *mut i32);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ubidi_setClassCallback(pbidi: *mut UBiDi, newfn: ::windows::runtime::RawPtr, newcontext: *const ::core::ffi::c_void, oldfn: *mut ::windows::runtime::RawPtr, oldcontext: *const *const ::core::ffi::c_void, perrorcode: *mut UErrorCode);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ubidi_setContext(pbidi: *mut UBiDi, prologue: *const u16, prolength: i32, epilogue: *const u16, epilength: i32, perrorcode: *mut UErrorCode);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ubidi_setInverse(pbidi: *mut UBiDi, isinverse: i8);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ubidi_setLine(pparabidi: *const UBiDi, start: i32, limit: i32, plinebidi: *mut UBiDi, perrorcode: *mut UErrorCode);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ubidi_setPara(pbidi: *mut UBiDi, text: *const u16, length: i32, paralevel: u8, embeddinglevels: *mut u8, perrorcode: *mut UErrorCode);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ubidi_setReorderingMode(pbidi: *mut UBiDi, reorderingmode: UBiDiReorderingMode);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ubidi_setReorderingOptions(pbidi: *mut UBiDi, reorderingoptions: u32);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ubidi_writeReordered(pbidi: *mut UBiDi, dest: *mut u16, destsize: i32, options: u16, perrorcode: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ubidi_writeReverse(src: *const u16, srclength: i32, dest: *mut u16, destsize: i32, options: u16, perrorcode: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ubiditransform_close(pbiditransform: *mut UBiDiTransform);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ubiditransform_open(perrorcode: *mut UErrorCode) -> *mut UBiDiTransform;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ubiditransform_transform(pbiditransform: *mut UBiDiTransform, src: *const u16, srclength: i32, dest: *mut u16, destsize: i32, inparalevel: u8, inorder: UBiDiOrder, outparalevel: u8, outorder: UBiDiOrder, domirroring: UBiDiMirroring, shapingoptions: u32, perrorcode: *mut UErrorCode) -> u32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ublock_getCode(c: i32) -> UBlockCode;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ubrk_close(bi: *mut UBreakIterator);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ubrk_countAvailable() -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ubrk_current(bi: *const UBreakIterator) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ubrk_first(bi: *mut UBreakIterator) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ubrk_following(bi: *mut UBreakIterator, offset: i32) -> i32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ubrk_getAvailable(index: i32) -> super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ubrk_getBinaryRules(bi: *mut UBreakIterator, binaryrules: *mut u8, rulescapacity: i32, status: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ubrk_getLocaleByType(bi: *const UBreakIterator, r#type: ULocDataLocaleType, status: *mut UErrorCode) -> super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ubrk_getRuleStatus(bi: *mut UBreakIterator) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ubrk_getRuleStatusVec(bi: *mut UBreakIterator, fillinvec: *mut i32, capacity: i32, status: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ubrk_isBoundary(bi: *mut UBreakIterator, offset: i32) -> i8;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ubrk_last(bi: *mut UBreakIterator) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ubrk_next(bi: *mut UBreakIterator) -> i32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ubrk_open(r#type: UBreakIteratorType, locale: super::Foundation::PSTR, text: *const u16, textlength: i32, status: *mut UErrorCode) -> *mut UBreakIterator;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ubrk_openBinaryRules(binaryrules: *const u8, ruleslength: i32, text: *const u16, textlength: i32, status: *mut UErrorCode) -> *mut UBreakIterator;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ubrk_openRules(rules: *const u16, ruleslength: i32, text: *const u16, textlength: i32, parseerr: *mut UParseError, status: *mut UErrorCode) -> *mut UBreakIterator;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ubrk_preceding(bi: *mut UBreakIterator, offset: i32) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ubrk_previous(bi: *mut UBreakIterator) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ubrk_refreshUText(bi: *mut UBreakIterator, text: *mut UText, status: *mut UErrorCode);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ubrk_safeClone(bi: *const UBreakIterator, stackbuffer: *mut ::core::ffi::c_void, pbuffersize: *mut i32, status: *mut UErrorCode) -> *mut UBreakIterator;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ubrk_setText(bi: *mut UBreakIterator, text: *const u16, textlength: i32, status: *mut UErrorCode);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ubrk_setUText(bi: *mut UBreakIterator, text: *mut UText, status: *mut UErrorCode);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucal_add(cal: *mut *mut ::core::ffi::c_void, field: UCalendarDateFields, amount: i32, status: *mut UErrorCode);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucal_clear(calendar: *mut *mut ::core::ffi::c_void);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucal_clearField(cal: *mut *mut ::core::ffi::c_void, field: UCalendarDateFields);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucal_clone(cal: *const *const ::core::ffi::c_void, status: *mut UErrorCode) -> *mut *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucal_close(cal: *mut *mut ::core::ffi::c_void);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucal_countAvailable() -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucal_equivalentTo(cal1: *const *const ::core::ffi::c_void, cal2: *const *const ::core::ffi::c_void) -> i8;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucal_get(cal: *const *const ::core::ffi::c_void, field: UCalendarDateFields, status: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucal_getAttribute(cal: *const *const ::core::ffi::c_void, attr: UCalendarAttribute) -> i32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucal_getAvailable(localeindex: i32) -> super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucal_getCanonicalTimeZoneID(id: *const u16, len: i32, result: *mut u16, resultcapacity: i32, issystemid: *mut i8, status: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucal_getDSTSavings(zoneid: *const u16, ec: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucal_getDayOfWeekType(cal: *const *const ::core::ffi::c_void, dayofweek: UCalendarDaysOfWeek, status: *mut UErrorCode) -> UCalendarWeekdayType;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucal_getDefaultTimeZone(result: *mut u16, resultcapacity: i32, ec: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucal_getFieldDifference(cal: *mut *mut ::core::ffi::c_void, target: f64, field: UCalendarDateFields, status: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucal_getGregorianChange(cal: *const *const ::core::ffi::c_void, perrorcode: *mut UErrorCode) -> f64;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucal_getHostTimeZone(result: *mut u16, resultcapacity: i32, ec: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucal_getKeywordValuesForLocale(key: super::Foundation::PSTR, locale: super::Foundation::PSTR, commonlyused: i8, status: *mut UErrorCode) -> *mut UEnumeration;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucal_getLimit(cal: *const *const ::core::ffi::c_void, field: UCalendarDateFields, r#type: UCalendarLimitType, status: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucal_getLocaleByType(cal: *const *const ::core::ffi::c_void, r#type: ULocDataLocaleType, status: *mut UErrorCode) -> super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucal_getMillis(cal: *const *const ::core::ffi::c_void, status: *mut UErrorCode) -> f64;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucal_getNow() -> f64;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucal_getTZDataVersion(status: *mut UErrorCode) -> super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucal_getTimeZoneDisplayName(cal: *const *const ::core::ffi::c_void, r#type: UCalendarDisplayNameType, locale: super::Foundation::PSTR, result: *mut u16, resultlength: i32, status: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucal_getTimeZoneID(cal: *const *const ::core::ffi::c_void, result: *mut u16, resultlength: i32, status: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucal_getTimeZoneIDForWindowsID(winid: *const u16, len: i32, region: super::Foundation::PSTR, id: *mut u16, idcapacity: i32, status: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucal_getTimeZoneTransitionDate(cal: *const *const ::core::ffi::c_void, r#type: UTimeZoneTransitionType, transition: *mut f64, status: *mut UErrorCode) -> i8;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucal_getType(cal: *const *const ::core::ffi::c_void, status: *mut UErrorCode) -> super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucal_getWeekendTransition(cal: *const *const ::core::ffi::c_void, dayofweek: UCalendarDaysOfWeek, status: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucal_getWindowsTimeZoneID(id: *const u16, len: i32, winid: *mut u16, winidcapacity: i32, status: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucal_inDaylightTime(cal: *const *const ::core::ffi::c_void, status: *mut UErrorCode) -> i8;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucal_isSet(cal: *const *const ::core::ffi::c_void, field: UCalendarDateFields) -> i8;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucal_isWeekend(cal: *const *const ::core::ffi::c_void, date: f64, status: *mut UErrorCode) -> i8;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucal_open(zoneid: *const u16, len: i32, locale: super::Foundation::PSTR, r#type: UCalendarType, status: *mut UErrorCode) -> *mut *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucal_openCountryTimeZones(country: super::Foundation::PSTR, ec: *mut UErrorCode) -> *mut UEnumeration;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucal_openTimeZoneIDEnumeration(zonetype: USystemTimeZoneType, region: super::Foundation::PSTR, rawoffset: *const i32, ec: *mut UErrorCode) -> *mut UEnumeration;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucal_openTimeZones(ec: *mut UErrorCode) -> *mut UEnumeration;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucal_roll(cal: *mut *mut ::core::ffi::c_void, field: UCalendarDateFields, amount: i32, status: *mut UErrorCode);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucal_set(cal: *mut *mut ::core::ffi::c_void, field: UCalendarDateFields, value: i32);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucal_setAttribute(cal: *mut *mut ::core::ffi::c_void, attr: UCalendarAttribute, newvalue: i32);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucal_setDate(cal: *mut *mut ::core::ffi::c_void, year: i32, month: i32, date: i32, status: *mut UErrorCode);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucal_setDateTime(cal: *mut *mut ::core::ffi::c_void, year: i32, month: i32, date: i32, hour: i32, minute: i32, second: i32, status: *mut UErrorCode);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucal_setDefaultTimeZone(zoneid: *const u16, ec: *mut UErrorCode);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucal_setGregorianChange(cal: *mut *mut ::core::ffi::c_void, date: f64, perrorcode: *mut UErrorCode);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucal_setMillis(cal: *mut *mut ::core::ffi::c_void, datetime: f64, status: *mut UErrorCode);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucal_setTimeZone(cal: *mut *mut ::core::ffi::c_void, zoneid: *const u16, len: i32, status: *mut UErrorCode);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucasemap_close(csm: *mut UCaseMap);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucasemap_getBreakIterator(csm: *const UCaseMap) -> *mut UBreakIterator;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucasemap_getLocale(csm: *const UCaseMap) -> super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucasemap_getOptions(csm: *const UCaseMap) -> u32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucasemap_open(locale: super::Foundation::PSTR, options: u32, perrorcode: *mut UErrorCode) -> *mut UCaseMap;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucasemap_setBreakIterator(csm: *mut UCaseMap, itertoadopt: *mut UBreakIterator, perrorcode: *mut UErrorCode);
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucasemap_setLocale(csm: *mut UCaseMap, locale: super::Foundation::PSTR, perrorcode: *mut UErrorCode);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucasemap_setOptions(csm: *mut UCaseMap, options: u32, perrorcode: *mut UErrorCode);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucasemap_toTitle(csm: *mut UCaseMap, dest: *mut u16, destcapacity: i32, src: *const u16, srclength: i32, perrorcode: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucasemap_utf8FoldCase(csm: *const UCaseMap, dest: super::Foundation::PSTR, destcapacity: i32, src: super::Foundation::PSTR, srclength: i32, perrorcode: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucasemap_utf8ToLower(csm: *const UCaseMap, dest: super::Foundation::PSTR, destcapacity: i32, src: super::Foundation::PSTR, srclength: i32, perrorcode: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucasemap_utf8ToTitle(csm: *mut UCaseMap, dest: super::Foundation::PSTR, destcapacity: i32, src: super::Foundation::PSTR, srclength: i32, perrorcode: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucasemap_utf8ToUpper(csm: *const UCaseMap, dest: super::Foundation::PSTR, destcapacity: i32, src: super::Foundation::PSTR, srclength: i32, perrorcode: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucfpos_close(ucfpos: *mut UConstrainedFieldPosition);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucfpos_constrainCategory(ucfpos: *mut UConstrainedFieldPosition, category: i32, ec: *mut UErrorCode);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucfpos_constrainField(ucfpos: *mut UConstrainedFieldPosition, category: i32, field: i32, ec: *mut UErrorCode);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucfpos_getCategory(ucfpos: *const UConstrainedFieldPosition, ec: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucfpos_getField(ucfpos: *const UConstrainedFieldPosition, ec: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucfpos_getIndexes(ucfpos: *const UConstrainedFieldPosition, pstart: *mut i32, plimit: *mut i32, ec: *mut UErrorCode);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucfpos_getInt64IterationContext(ucfpos: *const UConstrainedFieldPosition, ec: *mut UErrorCode) -> i64;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucfpos_matchesField(ucfpos: *const UConstrainedFieldPosition, category: i32, field: i32, ec: *mut UErrorCode) -> i8;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucfpos_open(ec: *mut UErrorCode) -> *mut UConstrainedFieldPosition;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucfpos_reset(ucfpos: *mut UConstrainedFieldPosition, ec: *mut UErrorCode);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucfpos_setInt64IterationContext(ucfpos: *mut UConstrainedFieldPosition, context: i64, ec: *mut UErrorCode);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucfpos_setState(ucfpos: *mut UConstrainedFieldPosition, category: i32, field: i32, start: i32, limit: i32, ec: *mut UErrorCode);
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_cbFromUWriteBytes(args: *mut UConverterFromUnicodeArgs, source: super::Foundation::PSTR, length: i32, offsetindex: i32, err: *mut UErrorCode);
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_cbFromUWriteSub(args: *mut UConverterFromUnicodeArgs, offsetindex: i32, err: *mut UErrorCode);
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_cbFromUWriteUChars(args: *mut UConverterFromUnicodeArgs, source: *const *const u16, sourcelimit: *const u16, offsetindex: i32, err: *mut UErrorCode);
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_cbToUWriteSub(args: *mut UConverterToUnicodeArgs, offsetindex: i32, err: *mut UErrorCode);
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_cbToUWriteUChars(args: *mut UConverterToUnicodeArgs, source: *const u16, length: i32, offsetindex: i32, err: *mut UErrorCode);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucnv_close(converter: *mut UConverter);
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_compareNames(name1: super::Foundation::PSTR, name2: super::Foundation::PSTR) -> i32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_convert(toconvertername: super::Foundation::PSTR, fromconvertername: super::Foundation::PSTR, target: super::Foundation::PSTR, targetcapacity: i32, source: super::Foundation::PSTR, sourcelength: i32, perrorcode: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_convertEx(targetcnv: *mut UConverter, sourcecnv: *mut UConverter, target: *mut *mut i8, targetlimit: super::Foundation::PSTR, source: *const *const i8, sourcelimit: super::Foundation::PSTR, pivotstart: *mut u16, pivotsource: *mut *mut u16, pivottarget: *mut *mut u16, pivotlimit: *const u16, reset: i8, flush: i8, perrorcode: *mut UErrorCode);
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_countAliases(alias: super::Foundation::PSTR, perrorcode: *mut UErrorCode) -> u16;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucnv_countAvailable() -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucnv_countStandards() -> u16;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_detectUnicodeSignature(source: super::Foundation::PSTR, sourcelength: i32, signaturelength: *mut i32, perrorcode: *mut UErrorCode) -> super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucnv_fixFileSeparator(cnv: *const UConverter, source: *mut u16, sourcelen: i32);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucnv_flushCache() -> i32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_fromAlgorithmic(cnv: *mut UConverter, algorithmictype: UConverterType, target: super::Foundation::PSTR, targetcapacity: i32, source: super::Foundation::PSTR, sourcelength: i32, perrorcode: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_fromUChars(cnv: *mut UConverter, dest: super::Foundation::PSTR, destcapacity: i32, src: *const u16, srclength: i32, perrorcode: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucnv_fromUCountPending(cnv: *const UConverter, status: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_fromUnicode(converter: *mut UConverter, target: *mut *mut i8, targetlimit: super::Foundation::PSTR, source: *const *const u16, sourcelimit: *const u16, offsets: *mut i32, flush: i8, err: *mut UErrorCode);
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_getAlias(alias: super::Foundation::PSTR, n: u16, perrorcode: *mut UErrorCode) -> super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_getAliases(alias: super::Foundation::PSTR, aliases: *const *const i8, perrorcode: *mut UErrorCode);
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_getAvailableName(n: i32) -> super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucnv_getCCSID(converter: *const UConverter, err: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_getCanonicalName(alias: super::Foundation::PSTR, standard: super::Foundation::PSTR, perrorcode: *mut UErrorCode) -> super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_getDefaultName() -> super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_getDisplayName(converter: *const UConverter, displaylocale: super::Foundation::PSTR, displayname: *mut u16, displaynamecapacity: i32, err: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_getFromUCallBack(converter: *const UConverter, action: *mut ::windows::runtime::RawPtr, context: *const *const ::core::ffi::c_void);
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_getInvalidChars(converter: *const UConverter, errbytes: super::Foundation::PSTR, len: *mut i8, err: *mut UErrorCode);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucnv_getInvalidUChars(converter: *const UConverter, erruchars: *mut u16, len: *mut i8, err: *mut UErrorCode);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucnv_getMaxCharSize(converter: *const UConverter) -> i8;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucnv_getMinCharSize(converter: *const UConverter) -> i8;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_getName(converter: *const UConverter, err: *mut UErrorCode) -> super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_getNextUChar(converter: *mut UConverter, source: *const *const i8, sourcelimit: super::Foundation::PSTR, err: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucnv_getPlatform(converter: *const UConverter, err: *mut UErrorCode) -> UConverterPlatform;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_getStandard(n: u16, perrorcode: *mut UErrorCode) -> super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_getStandardName(name: super::Foundation::PSTR, standard: super::Foundation::PSTR, perrorcode: *mut UErrorCode) -> super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucnv_getStarters(converter: *const UConverter, starters: *mut i8, err: *mut UErrorCode);
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_getSubstChars(converter: *const UConverter, subchars: super::Foundation::PSTR, len: *mut i8, err: *mut UErrorCode);
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_getToUCallBack(converter: *const UConverter, action: *mut ::windows::runtime::RawPtr, context: *const *const ::core::ffi::c_void);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucnv_getType(converter: *const UConverter) -> UConverterType;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucnv_getUnicodeSet(cnv: *const UConverter, setfillin: *mut USet, whichset: UConverterUnicodeSet, perrorcode: *mut UErrorCode);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucnv_isAmbiguous(cnv: *const UConverter) -> i8;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucnv_isFixedWidth(cnv: *mut UConverter, status: *mut UErrorCode) -> i8;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_open(convertername: super::Foundation::PSTR, err: *mut UErrorCode) -> *mut UConverter;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucnv_openAllNames(perrorcode: *mut UErrorCode) -> *mut UEnumeration;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucnv_openCCSID(codepage: i32, platform: UConverterPlatform, err: *mut UErrorCode) -> *mut UConverter;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_openPackage(packagename: super::Foundation::PSTR, convertername: super::Foundation::PSTR, err: *mut UErrorCode) -> *mut UConverter;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_openStandardNames(convname: super::Foundation::PSTR, standard: super::Foundation::PSTR, perrorcode: *mut UErrorCode) -> *mut UEnumeration;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucnv_openU(name: *const u16, err: *mut UErrorCode) -> *mut UConverter;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucnv_reset(converter: *mut UConverter);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucnv_resetFromUnicode(converter: *mut UConverter);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucnv_resetToUnicode(converter: *mut UConverter);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucnv_safeClone(cnv: *const UConverter, stackbuffer: *mut ::core::ffi::c_void, pbuffersize: *mut i32, status: *mut UErrorCode) -> *mut UConverter;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_setDefaultName(name: super::Foundation::PSTR);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucnv_setFallback(cnv: *mut UConverter, usesfallback: i8);
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_setFromUCallBack(converter: *mut UConverter, newaction: ::windows::runtime::RawPtr, newcontext: *const ::core::ffi::c_void, oldaction: *mut ::windows::runtime::RawPtr, oldcontext: *const *const ::core::ffi::c_void, err: *mut UErrorCode);
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_setSubstChars(converter: *mut UConverter, subchars: super::Foundation::PSTR, len: i8, err: *mut UErrorCode);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucnv_setSubstString(cnv: *mut UConverter, s: *const u16, length: i32, err: *mut UErrorCode);
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_setToUCallBack(converter: *mut UConverter, newaction: ::windows::runtime::RawPtr, newcontext: *const ::core::ffi::c_void, oldaction: *mut ::windows::runtime::RawPtr, oldcontext: *const *const ::core::ffi::c_void, err: *mut UErrorCode);
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_toAlgorithmic(algorithmictype: UConverterType, cnv: *mut UConverter, target: super::Foundation::PSTR, targetcapacity: i32, source: super::Foundation::PSTR, sourcelength: i32, perrorcode: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_toUChars(cnv: *mut UConverter, dest: *mut u16, destcapacity: i32, src: super::Foundation::PSTR, srclength: i32, perrorcode: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucnv_toUCountPending(cnv: *const UConverter, status: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_toUnicode(converter: *mut UConverter, target: *mut *mut u16, targetlimit: *const u16, source: *const *const i8, sourcelimit: super::Foundation::PSTR, offsets: *mut i32, flush: i8, err: *mut UErrorCode);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucnv_usesFallback(cnv: *const UConverter) -> i8;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucnvsel_close(sel: *mut UConverterSelector);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucnvsel_open(converterlist: *const *const i8, converterlistsize: i32, excludedcodepoints: *const USet, whichset: UConverterUnicodeSet, status: *mut UErrorCode) -> *mut UConverterSelector;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucnvsel_openFromSerialized(buffer: *const ::core::ffi::c_void, length: i32, status: *mut UErrorCode) -> *mut UConverterSelector;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucnvsel_selectForString(sel: *const UConverterSelector, s: *const u16, length: i32, status: *mut UErrorCode) -> *mut UEnumeration;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnvsel_selectForUTF8(sel: *const UConverterSelector, s: super::Foundation::PSTR, length: i32, status: *mut UErrorCode) -> *mut UEnumeration;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucnvsel_serialize(sel: *const UConverterSelector, buffer: *mut ::core::ffi::c_void, buffercapacity: i32, status: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucol_cloneBinary(coll: *const UCollator, buffer: *mut u8, capacity: i32, status: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucol_close(coll: *mut UCollator);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucol_closeElements(elems: *mut UCollationElements);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucol_countAvailable() -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucol_equal(coll: *const UCollator, source: *const u16, sourcelength: i32, target: *const u16, targetlength: i32) -> i8;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucol_getAttribute(coll: *const UCollator, attr: UColAttribute, status: *mut UErrorCode) -> UColAttributeValue;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucol_getAvailable(localeindex: i32) -> super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucol_getBound(source: *const u8, sourcelength: i32, boundtype: UColBoundMode, nooflevels: u32, result: *mut u8, resultlength: i32, status: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucol_getContractionsAndExpansions(coll: *const UCollator, contractions: *mut USet, expansions: *mut USet, addprefixes: i8, status: *mut UErrorCode);
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucol_getDisplayName(objloc: super::Foundation::PSTR, disploc: super::Foundation::PSTR, result: *mut u16, resultlength: i32, status: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucol_getEquivalentReorderCodes(reordercode: i32, dest: *mut i32, destcapacity: i32, perrorcode: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucol_getFunctionalEquivalent(result: super::Foundation::PSTR, resultcapacity: i32, keyword: super::Foundation::PSTR, locale: super::Foundation::PSTR, isavailable: *mut i8, status: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucol_getKeywordValues(keyword: super::Foundation::PSTR, status: *mut UErrorCode) -> *mut UEnumeration;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucol_getKeywordValuesForLocale(key: super::Foundation::PSTR, locale: super::Foundation::PSTR, commonlyused: i8, status: *mut UErrorCode) -> *mut UEnumeration;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucol_getKeywords(status: *mut UErrorCode) -> *mut UEnumeration;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucol_getLocaleByType(coll: *const UCollator, r#type: ULocDataLocaleType, status: *mut UErrorCode) -> super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucol_getMaxExpansion(elems: *const UCollationElements, order: i32) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucol_getMaxVariable(coll: *const UCollator) -> UColReorderCode;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucol_getOffset(elems: *const UCollationElements) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucol_getReorderCodes(coll: *const UCollator, dest: *mut i32, destcapacity: i32, perrorcode: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucol_getRules(coll: *const UCollator, length: *mut i32) -> *mut u16;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucol_getRulesEx(coll: *const UCollator, delta: UColRuleOption, buffer: *mut u16, bufferlen: i32) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucol_getSortKey(coll: *const UCollator, source: *const u16, sourcelength: i32, result: *mut u8, resultlength: i32) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucol_getStrength(coll: *const UCollator) -> UColAttributeValue;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucol_getTailoredSet(coll: *const UCollator, status: *mut UErrorCode) -> *mut USet;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucol_getUCAVersion(coll: *const UCollator, info: *mut u8);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucol_getVariableTop(coll: *const UCollator, status: *mut UErrorCode) -> u32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucol_getVersion(coll: *const UCollator, info: *mut u8);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucol_greater(coll: *const UCollator, source: *const u16, sourcelength: i32, target: *const u16, targetlength: i32) -> i8;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucol_greaterOrEqual(coll: *const UCollator, source: *const u16, sourcelength: i32, target: *const u16, targetlength: i32) -> i8;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucol_keyHashCode(key: *const u8, length: i32) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucol_mergeSortkeys(src1: *const u8, src1length: i32, src2: *const u8, src2length: i32, dest: *mut u8, destcapacity: i32) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucol_next(elems: *mut UCollationElements, status: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucol_nextSortKeyPart(coll: *const UCollator, iter: *mut ::core::mem::ManuallyDrop<UCharIterator>, state: *mut u32, dest: *mut u8, count: i32, status: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucol_open(loc: super::Foundation::PSTR, status: *mut UErrorCode) -> *mut UCollator;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucol_openAvailableLocales(status: *mut UErrorCode) -> *mut UEnumeration;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucol_openBinary(bin: *const u8, length: i32, base: *const UCollator, status: *mut UErrorCode) -> *mut UCollator;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucol_openElements(coll: *const UCollator, text: *const u16, textlength: i32, status: *mut UErrorCode) -> *mut UCollationElements;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucol_openRules(rules: *const u16, ruleslength: i32, normalizationmode: UColAttributeValue, strength: UColAttributeValue, parseerror: *mut UParseError, status: *mut UErrorCode) -> *mut UCollator;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucol_previous(elems: *mut UCollationElements, status: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucol_primaryOrder(order: i32) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucol_reset(elems: *mut UCollationElements);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucol_safeClone(coll: *const UCollator, stackbuffer: *mut ::core::ffi::c_void, pbuffersize: *mut i32, status: *mut UErrorCode) -> *mut UCollator;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucol_secondaryOrder(order: i32) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucol_setAttribute(coll: *mut UCollator, attr: UColAttribute, value: UColAttributeValue, status: *mut UErrorCode);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucol_setMaxVariable(coll: *mut UCollator, group: UColReorderCode, perrorcode: *mut UErrorCode);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucol_setOffset(elems: *mut UCollationElements, offset: i32, status: *mut UErrorCode);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucol_setReorderCodes(coll: *mut UCollator, reordercodes: *const i32, reordercodeslength: i32, perrorcode: *mut UErrorCode);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucol_setStrength(coll: *mut UCollator, strength: UColAttributeValue);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucol_setText(elems: *mut UCollationElements, text: *const u16, textlength: i32, status: *mut UErrorCode);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucol_strcoll(coll: *const UCollator, source: *const u16, sourcelength: i32, target: *const u16, targetlength: i32) -> UCollationResult;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucol_strcollIter(coll: *const UCollator, siter: *mut ::core::mem::ManuallyDrop<UCharIterator>, titer: *mut ::core::mem::ManuallyDrop<UCharIterator>, status: *mut UErrorCode) -> UCollationResult;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucol_strcollUTF8(coll: *const UCollator, source: super::Foundation::PSTR, sourcelength: i32, target: super::Foundation::PSTR, targetlength: i32, status: *mut UErrorCode) -> UCollationResult;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucol_tertiaryOrder(order: i32) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucpmap_get(map: *const UCPMap, c: i32) -> u32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucpmap_getRange(map: *const UCPMap, start: i32, option: UCPMapRangeOption, surrogatevalue: u32, filter: *mut ::windows::runtime::RawPtr, context: *const ::core::ffi::c_void, pvalue: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucptrie_close(trie: *mut UCPTrie);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucptrie_get(trie: *const UCPTrie, c: i32) -> u32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucptrie_getRange(trie: *const UCPTrie, start: i32, option: UCPMapRangeOption, surrogatevalue: u32, filter: *mut ::windows::runtime::RawPtr, context: *const ::core::ffi::c_void, pvalue: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucptrie_getType(trie: *const UCPTrie) -> UCPTrieType;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucptrie_getValueWidth(trie: *const UCPTrie) -> UCPTrieValueWidth;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucptrie_internalSmallIndex(trie: *const UCPTrie, c: i32) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucptrie_internalSmallU8Index(trie: *const UCPTrie, lt1: i32, t2: u8, t3: u8) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucptrie_internalU8PrevIndex(trie: *const UCPTrie, c: i32, start: *const u8, src: *const u8) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucptrie_openFromBinary(r#type: UCPTrieType, valuewidth: UCPTrieValueWidth, data: *const ::core::ffi::c_void, length: i32, pactuallength: *mut i32, perrorcode: *mut UErrorCode) -> *mut UCPTrie;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucptrie_toBinary(trie: *const UCPTrie, data: *mut ::core::ffi::c_void, capacity: i32, perrorcode: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucsdet_close(ucsd: *mut UCharsetDetector);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucsdet_detect(ucsd: *mut UCharsetDetector, status: *mut UErrorCode) -> *mut UCharsetMatch;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucsdet_detectAll(ucsd: *mut UCharsetDetector, matchesfound: *mut i32, status: *mut UErrorCode) -> *mut *mut UCharsetMatch;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucsdet_enableInputFilter(ucsd: *mut UCharsetDetector, filter: i8) -> i8;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucsdet_getAllDetectableCharsets(ucsd: *const UCharsetDetector, status: *mut UErrorCode) -> *mut UEnumeration;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucsdet_getConfidence(ucsm: *const UCharsetMatch, status: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucsdet_getLanguage(ucsm: *const UCharsetMatch, status: *mut UErrorCode) -> super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucsdet_getName(ucsm: *const UCharsetMatch, status: *mut UErrorCode) -> super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucsdet_getUChars(ucsm: *const UCharsetMatch, buf: *mut u16, cap: i32, status: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucsdet_isInputFilterEnabled(ucsd: *const UCharsetDetector) -> i8;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucsdet_open(status: *mut UErrorCode) -> *mut UCharsetDetector;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucsdet_setDeclaredEncoding(ucsd: *mut UCharsetDetector, encoding: super::Foundation::PSTR, length: i32, status: *mut UErrorCode);
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucsdet_setText(ucsd: *mut UCharsetDetector, textin: super::Foundation::PSTR, len: i32, status: *mut UErrorCode);
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucurr_countCurrencies(locale: super::Foundation::PSTR, date: f64, ec: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucurr_forLocale(locale: super::Foundation::PSTR, buff: *mut u16, buffcapacity: i32, ec: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucurr_forLocaleAndDate(locale: super::Foundation::PSTR, date: f64, index: i32, buff: *mut u16, buffcapacity: i32, ec: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucurr_getDefaultFractionDigits(currency: *const u16, ec: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucurr_getDefaultFractionDigitsForUsage(currency: *const u16, usage: UCurrencyUsage, ec: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucurr_getKeywordValuesForLocale(key: super::Foundation::PSTR, locale: super::Foundation::PSTR, commonlyused: i8, status: *mut UErrorCode) -> *mut UEnumeration;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucurr_getName(currency: *const u16, locale: super::Foundation::PSTR, namestyle: UCurrNameStyle, ischoiceformat: *mut i8, len: *mut i32, ec: *mut UErrorCode) -> *mut u16;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucurr_getNumericCode(currency: *const u16) -> i32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucurr_getPluralName(currency: *const u16, locale: super::Foundation::PSTR, ischoiceformat: *mut i8, pluralcount: super::Foundation::PSTR, len: *mut i32, ec: *mut UErrorCode) -> *mut u16;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucurr_getRoundingIncrement(currency: *const u16, ec: *mut UErrorCode) -> f64;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucurr_getRoundingIncrementForUsage(currency: *const u16, usage: UCurrencyUsage, ec: *mut UErrorCode) -> f64;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucurr_isAvailable(isocode: *const u16, from: f64, to: f64, errorcode: *mut UErrorCode) -> i8;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucurr_openISOCurrencies(currtype: u32, perrorcode: *mut UErrorCode) -> *mut UEnumeration;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucurr_register(isocode: *const u16, locale: super::Foundation::PSTR, status: *mut UErrorCode) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucurr_unregister(key: *mut ::core::ffi::c_void, status: *mut UErrorCode) -> i8;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn udat_adoptNumberFormat(fmt: *mut *mut ::core::ffi::c_void, numberformattoadopt: *mut *mut ::core::ffi::c_void);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn udat_adoptNumberFormatForFields(fmt: *mut *mut ::core::ffi::c_void, fields: *const u16, numberformattoset: *mut *mut ::core::ffi::c_void, status: *mut UErrorCode);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn udat_applyPattern(format: *mut *mut ::core::ffi::c_void, localized: i8, pattern: *const u16, patternlength: i32);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn udat_clone(fmt: *const *const ::core::ffi::c_void, status: *mut UErrorCode) -> *mut *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn udat_close(format: *mut *mut ::core::ffi::c_void);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn udat_countAvailable() -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn udat_countSymbols(fmt: *const *const ::core::ffi::c_void, r#type: UDateFormatSymbolType) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn udat_format(format: *const *const ::core::ffi::c_void, datetoformat: f64, result: *mut u16, resultlength: i32, position: *mut UFieldPosition, status: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn udat_formatCalendar(format: *const *const ::core::ffi::c_void, calendar: *mut *mut ::core::ffi::c_void, result: *mut u16, capacity: i32, position: *mut UFieldPosition, status: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn udat_formatCalendarForFields(format: *const *const ::core::ffi::c_void, calendar: *mut *mut ::core::ffi::c_void, result: *mut u16, capacity: i32, fpositer: *mut UFieldPositionIterator, status: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn udat_formatForFields(format: *const *const ::core::ffi::c_void, datetoformat: f64, result: *mut u16, resultlength: i32, fpositer: *mut UFieldPositionIterator, status: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn udat_get2DigitYearStart(fmt: *const *const ::core::ffi::c_void, status: *mut UErrorCode) -> f64;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn udat_getAvailable(localeindex: i32) -> super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn udat_getBooleanAttribute(fmt: *const *const ::core::ffi::c_void, attr: UDateFormatBooleanAttribute, status: *mut UErrorCode) -> i8;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn udat_getCalendar(fmt: *const *const ::core::ffi::c_void) -> *mut *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn udat_getContext(fmt: *const *const ::core::ffi::c_void, r#type: UDisplayContextType, status: *mut UErrorCode) -> UDisplayContext;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn udat_getLocaleByType(fmt: *const *const ::core::ffi::c_void, r#type: ULocDataLocaleType, status: *mut UErrorCode) -> super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn udat_getNumberFormat(fmt: *const *const ::core::ffi::c_void) -> *mut *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn udat_getNumberFormatForField(fmt: *const *const ::core::ffi::c_void, field: u16) -> *mut *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn udat_getSymbols(fmt: *const *const ::core::ffi::c_void, r#type: UDateFormatSymbolType, symbolindex: i32, result: *mut u16, resultlength: i32, status: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn udat_isLenient(fmt: *const *const ::core::ffi::c_void) -> i8;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn udat_open(timestyle: UDateFormatStyle, datestyle: UDateFormatStyle, locale: super::Foundation::PSTR, tzid: *const u16, tzidlength: i32, pattern: *const u16, patternlength: i32, status: *mut UErrorCode) -> *mut *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn udat_parse(format: *const *const ::core::ffi::c_void, text: *const u16, textlength: i32, parsepos: *mut i32, status: *mut UErrorCode) -> f64;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn udat_parseCalendar(format: *const *const ::core::ffi::c_void, calendar: *mut *mut ::core::ffi::c_void, text: *const u16, textlength: i32, parsepos: *mut i32, status: *mut UErrorCode);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn udat_set2DigitYearStart(fmt: *mut *mut ::core::ffi::c_void, d: f64, status: *mut UErrorCode);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn udat_setBooleanAttribute(fmt: *mut *mut ::core::ffi::c_void, attr: UDateFormatBooleanAttribute, newvalue: i8, status: *mut UErrorCode);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn udat_setCalendar(fmt: *mut *mut ::core::ffi::c_void, calendartoset: *const *const ::core::ffi::c_void);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn udat_setContext(fmt: *mut *mut ::core::ffi::c_void, value: UDisplayContext, status: *mut UErrorCode);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn udat_setLenient(fmt: *mut *mut ::core::ffi::c_void, islenient: i8);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn udat_setNumberFormat(fmt: *mut *mut ::core::ffi::c_void, numberformattoset: *const *const ::core::ffi::c_void);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn udat_setSymbols(format: *mut *mut ::core::ffi::c_void, r#type: UDateFormatSymbolType, symbolindex: i32, value: *mut u16, valuelength: i32, status: *mut UErrorCode);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn udat_toCalendarDateField(field: UDateFormatField) -> UCalendarDateFields;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn udat_toPattern(fmt: *const *const ::core::ffi::c_void, localized: i8, result: *mut u16, resultlength: i32, status: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn udatpg_addPattern(dtpg: *mut *mut ::core::ffi::c_void, pattern: *const u16, patternlength: i32, r#override: i8, conflictingpattern: *mut u16, capacity: i32, plength: *mut i32, perrorcode: *mut UErrorCode) -> UDateTimePatternConflict;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn udatpg_clone(dtpg: *const *const ::core::ffi::c_void, perrorcode: *mut UErrorCode) -> *mut *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn udatpg_close(dtpg: *mut *mut ::core::ffi::c_void);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn udatpg_getAppendItemFormat(dtpg: *const *const ::core::ffi::c_void, field: UDateTimePatternField, plength: *mut i32) -> *mut u16;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn udatpg_getAppendItemName(dtpg: *const *const ::core::ffi::c_void, field: UDateTimePatternField, plength: *mut i32) -> *mut u16;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn udatpg_getBaseSkeleton(unuseddtpg: *mut *mut ::core::ffi::c_void, pattern: *const u16, length: i32, baseskeleton: *mut u16, capacity: i32, perrorcode: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn udatpg_getBestPattern(dtpg: *mut *mut ::core::ffi::c_void, skeleton: *const u16, length: i32, bestpattern: *mut u16, capacity: i32, perrorcode: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn udatpg_getBestPatternWithOptions(dtpg: *mut *mut ::core::ffi::c_void, skeleton: *const u16, length: i32, options: UDateTimePatternMatchOptions, bestpattern: *mut u16, capacity: i32, perrorcode: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn udatpg_getDateTimeFormat(dtpg: *const *const ::core::ffi::c_void, plength: *mut i32) -> *mut u16;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn udatpg_getDecimal(dtpg: *const *const ::core::ffi::c_void, plength: *mut i32) -> *mut u16;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn udatpg_getFieldDisplayName(dtpg: *const *const ::core::ffi::c_void, field: UDateTimePatternField, width: UDateTimePGDisplayWidth, fieldname: *mut u16, capacity: i32, perrorcode: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn udatpg_getPatternForSkeleton(dtpg: *const *const ::core::ffi::c_void, skeleton: *const u16, skeletonlength: i32, plength: *mut i32) -> *mut u16;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn udatpg_getSkeleton(unuseddtpg: *mut *mut ::core::ffi::c_void, pattern: *const u16, length: i32, skeleton: *mut u16, capacity: i32, perrorcode: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn udatpg_open(locale: super::Foundation::PSTR, perrorcode: *mut UErrorCode) -> *mut *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn udatpg_openBaseSkeletons(dtpg: *const *const ::core::ffi::c_void, perrorcode: *mut UErrorCode) -> *mut UEnumeration;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn udatpg_openEmpty(perrorcode: *mut UErrorCode) -> *mut *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn udatpg_openSkeletons(dtpg: *const *const ::core::ffi::c_void, perrorcode: *mut UErrorCode) -> *mut UEnumeration;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn udatpg_replaceFieldTypes(dtpg: *mut *mut ::core::ffi::c_void, pattern: *const u16, patternlength: i32, skeleton: *const u16, skeletonlength: i32, dest: *mut u16, destcapacity: i32, perrorcode: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn udatpg_replaceFieldTypesWithOptions(dtpg: *mut *mut ::core::ffi::c_void, pattern: *const u16, patternlength: i32, skeleton: *const u16, skeletonlength: i32, options: UDateTimePatternMatchOptions, dest: *mut u16, destcapacity: i32, perrorcode: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn udatpg_setAppendItemFormat(dtpg: *mut *mut ::core::ffi::c_void, field: UDateTimePatternField, value: *const u16, length: i32);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn udatpg_setAppendItemName(dtpg: *mut *mut ::core::ffi::c_void, field: UDateTimePatternField, value: *const u16, length: i32);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn udatpg_setDateTimeFormat(dtpg: *const *const ::core::ffi::c_void, dtformat: *const u16, length: i32);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn udatpg_setDecimal(dtpg: *mut *mut ::core::ffi::c_void, decimal: *const u16, length: i32);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn udtitvfmt_close(formatter: *mut UDateIntervalFormat);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn udtitvfmt_closeResult(uresult: *mut UFormattedDateInterval);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn udtitvfmt_format(formatter: *const UDateIntervalFormat, fromdate: f64, todate: f64, result: *mut u16, resultcapacity: i32, position: *mut UFieldPosition, status: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn udtitvfmt_open(locale: super::Foundation::PSTR, skeleton: *const u16, skeletonlength: i32, tzid: *const u16, tzidlength: i32, status: *mut UErrorCode) -> *mut UDateIntervalFormat;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn udtitvfmt_openResult(ec: *mut UErrorCode) -> *mut UFormattedDateInterval;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn udtitvfmt_resultAsValue(uresult: *const UFormattedDateInterval, ec: *mut UErrorCode) -> *mut UFormattedValue;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uenum_close(en: *mut UEnumeration);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uenum_count(en: *mut UEnumeration, status: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn uenum_next(en: *mut UEnumeration, resultlength: *mut i32, status: *mut UErrorCode) -> super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uenum_openCharStringsEnumeration(strings: *const *const i8, count: i32, ec: *mut UErrorCode) -> *mut UEnumeration;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uenum_openUCharStringsEnumeration(strings: *const *const u16, count: i32, ec: *mut UErrorCode) -> *mut UEnumeration;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uenum_reset(en: *mut UEnumeration, status: *mut UErrorCode);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uenum_unext(en: *mut UEnumeration, resultlength: *mut i32, status: *mut UErrorCode) -> *mut u16;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ufieldpositer_close(fpositer: *mut UFieldPositionIterator);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ufieldpositer_next(fpositer: *mut UFieldPositionIterator, beginindex: *mut i32, endindex: *mut i32) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ufieldpositer_open(status: *mut UErrorCode) -> *mut UFieldPositionIterator;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ufmt_close(fmt: *mut *mut ::core::ffi::c_void);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ufmt_getArrayItemByIndex(fmt: *mut *mut ::core::ffi::c_void, n: i32, status: *mut UErrorCode) -> *mut *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ufmt_getArrayLength(fmt: *const *const ::core::ffi::c_void, status: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ufmt_getDate(fmt: *const *const ::core::ffi::c_void, status: *mut UErrorCode) -> f64;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ufmt_getDecNumChars(fmt: *mut *mut ::core::ffi::c_void, len: *mut i32, status: *mut UErrorCode) -> super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ufmt_getDouble(fmt: *mut *mut ::core::ffi::c_void, status: *mut UErrorCode) -> f64;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ufmt_getInt64(fmt: *mut *mut ::core::ffi::c_void, status: *mut UErrorCode) -> i64;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ufmt_getLong(fmt: *mut *mut ::core::ffi::c_void, status: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ufmt_getObject(fmt: *const *const ::core::ffi::c_void, status: *mut UErrorCode) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ufmt_getType(fmt: *const *const ::core::ffi::c_void, status: *mut UErrorCode) -> UFormattableType;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ufmt_getUChars(fmt: *mut *mut ::core::ffi::c_void, len: *mut i32, status: *mut UErrorCode) -> *mut u16;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ufmt_isNumeric(fmt: *const *const ::core::ffi::c_void) -> i8;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ufmt_open(status: *mut UErrorCode) -> *mut *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ufmtval_getString(ufmtval: *const UFormattedValue, plength: *mut i32, ec: *mut UErrorCode) -> *mut u16;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ufmtval_nextPosition(ufmtval: *const UFormattedValue, ucfpos: *mut UConstrainedFieldPosition, ec: *mut UErrorCode) -> i8;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ugender_getInstance(locale: super::Foundation::PSTR, status: *mut UErrorCode) -> *mut UGenderInfo;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ugender_getListGender(genderinfo: *const UGenderInfo, genders: *const UGender, size: i32, status: *mut UErrorCode) -> UGender;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uidna_close(idna: *mut UIDNA);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uidna_labelToASCII(idna: *const UIDNA, label: *const u16, length: i32, dest: *mut u16, capacity: i32, pinfo: *mut UIDNAInfo, perrorcode: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn uidna_labelToASCII_UTF8(idna: *const UIDNA, label: super::Foundation::PSTR, length: i32, dest: super::Foundation::PSTR, capacity: i32, pinfo: *mut UIDNAInfo, perrorcode: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uidna_labelToUnicode(idna: *const UIDNA, label: *const u16, length: i32, dest: *mut u16, capacity: i32, pinfo: *mut UIDNAInfo, perrorcode: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn uidna_labelToUnicodeUTF8(idna: *const UIDNA, label: super::Foundation::PSTR, length: i32, dest: super::Foundation::PSTR, capacity: i32, pinfo: *mut UIDNAInfo, perrorcode: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uidna_nameToASCII(idna: *const UIDNA, name: *const u16, length: i32, dest: *mut u16, capacity: i32, pinfo: *mut UIDNAInfo, perrorcode: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn uidna_nameToASCII_UTF8(idna: *const UIDNA, name: super::Foundation::PSTR, length: i32, dest: super::Foundation::PSTR, capacity: i32, pinfo: *mut UIDNAInfo, perrorcode: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uidna_nameToUnicode(idna: *const UIDNA, name: *const u16, length: i32, dest: *mut u16, capacity: i32, pinfo: *mut UIDNAInfo, perrorcode: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn uidna_nameToUnicodeUTF8(idna: *const UIDNA, name: super::Foundation::PSTR, length: i32, dest: super::Foundation::PSTR, capacity: i32, pinfo: *mut UIDNAInfo, perrorcode: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uidna_openUTS46(options: u32, perrorcode: *mut UErrorCode) -> *mut UIDNA;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uiter_current32(iter: *mut ::core::mem::ManuallyDrop<UCharIterator>) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uiter_getState(iter: *const ::core::mem::ManuallyDrop<UCharIterator>) -> u32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uiter_next32(iter: *mut ::core::mem::ManuallyDrop<UCharIterator>) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uiter_previous32(iter: *mut ::core::mem::ManuallyDrop<UCharIterator>) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uiter_setState(iter: *mut ::core::mem::ManuallyDrop<UCharIterator>, state: u32, perrorcode: *mut UErrorCode);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uiter_setString(iter: *mut ::core::mem::ManuallyDrop<UCharIterator>, s: *const u16, length: i32);
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn uiter_setUTF16BE(iter: *mut ::core::mem::ManuallyDrop<UCharIterator>, s: super::Foundation::PSTR, length: i32);
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn uiter_setUTF8(iter: *mut ::core::mem::ManuallyDrop<UCharIterator>, s: super::Foundation::PSTR, length: i32);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uldn_close(ldn: *mut ULocaleDisplayNames);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uldn_getContext(ldn: *const ULocaleDisplayNames, r#type: UDisplayContextType, perrorcode: *mut UErrorCode) -> UDisplayContext;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uldn_getDialectHandling(ldn: *const ULocaleDisplayNames) -> UDialectHandling;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn uldn_getLocale(ldn: *const ULocaleDisplayNames) -> super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn uldn_keyDisplayName(ldn: *const ULocaleDisplayNames, key: super::Foundation::PSTR, result: *mut u16, maxresultsize: i32, perrorcode: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn uldn_keyValueDisplayName(ldn: *const ULocaleDisplayNames, key: super::Foundation::PSTR, value: super::Foundation::PSTR, result: *mut u16, maxresultsize: i32, perrorcode: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn uldn_languageDisplayName(ldn: *const ULocaleDisplayNames, lang: super::Foundation::PSTR, result: *mut u16, maxresultsize: i32, perrorcode: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn uldn_localeDisplayName(ldn: *const ULocaleDisplayNames, locale: super::Foundation::PSTR, result: *mut u16, maxresultsize: i32, perrorcode: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn uldn_open(locale: super::Foundation::PSTR, dialecthandling: UDialectHandling, perrorcode: *mut UErrorCode) -> *mut ULocaleDisplayNames;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn uldn_openForContext(locale: super::Foundation::PSTR, contexts: *mut UDisplayContext, length: i32, perrorcode: *mut UErrorCode) -> *mut ULocaleDisplayNames;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn uldn_regionDisplayName(ldn: *const ULocaleDisplayNames, region: super::Foundation::PSTR, result: *mut u16, maxresultsize: i32, perrorcode: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uldn_scriptCodeDisplayName(ldn: *const ULocaleDisplayNames, scriptcode: UScriptCode, result: *mut u16, maxresultsize: i32, perrorcode: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn uldn_scriptDisplayName(ldn: *const ULocaleDisplayNames, script: super::Foundation::PSTR, result: *mut u16, maxresultsize: i32, perrorcode: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn uldn_variantDisplayName(ldn: *const ULocaleDisplayNames, variant: super::Foundation::PSTR, result: *mut u16, maxresultsize: i32, perrorcode: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ulistfmt_close(listfmt: *mut UListFormatter);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ulistfmt_closeResult(uresult: *mut UFormattedList);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ulistfmt_format(listfmt: *const UListFormatter, strings: *const *const u16, stringlengths: *const i32, stringcount: i32, result: *mut u16, resultcapacity: i32, status: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ulistfmt_formatStringsToResult(listfmt: *const UListFormatter, strings: *const *const u16, stringlengths: *const i32, stringcount: i32, uresult: *mut UFormattedList, status: *mut UErrorCode);
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ulistfmt_open(locale: super::Foundation::PSTR, status: *mut UErrorCode) -> *mut UListFormatter;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ulistfmt_openForType(locale: super::Foundation::PSTR, r#type: UListFormatterType, width: UListFormatterWidth, status: *mut UErrorCode) -> *mut UListFormatter;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ulistfmt_openResult(ec: *mut UErrorCode) -> *mut UFormattedList;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ulistfmt_resultAsValue(uresult: *const UFormattedList, ec: *mut UErrorCode) -> *mut UFormattedValue;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_acceptLanguage(result: super::Foundation::PSTR, resultavailable: i32, outresult: *mut UAcceptResult, acceptlist: *const *const i8, acceptlistcount: i32, availablelocales: *mut UEnumeration, status: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_acceptLanguageFromHTTP(result: super::Foundation::PSTR, resultavailable: i32, outresult: *mut UAcceptResult, httpacceptlanguage: super::Foundation::PSTR, availablelocales: *mut UEnumeration, status: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_addLikelySubtags(localeid: super::Foundation::PSTR, maximizedlocaleid: super::Foundation::PSTR, maximizedlocaleidcapacity: i32, err: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_canonicalize(localeid: super::Foundation::PSTR, name: super::Foundation::PSTR, namecapacity: i32, err: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uloc_countAvailable() -> i32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_forLanguageTag(langtag: super::Foundation::PSTR, localeid: super::Foundation::PSTR, localeidcapacity: i32, parsedlength: *mut i32, err: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_getAvailable(n: i32) -> super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_getBaseName(localeid: super::Foundation::PSTR, name: super::Foundation::PSTR, namecapacity: i32, err: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_getCharacterOrientation(localeid: super::Foundation::PSTR, status: *mut UErrorCode) -> ULayoutType;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_getCountry(localeid: super::Foundation::PSTR, country: super::Foundation::PSTR, countrycapacity: i32, err: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_getDefault() -> super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_getDisplayCountry(locale: super::Foundation::PSTR, displaylocale: super::Foundation::PSTR, country: *mut u16, countrycapacity: i32, status: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_getDisplayKeyword(keyword: super::Foundation::PSTR, displaylocale: super::Foundation::PSTR, dest: *mut u16, destcapacity: i32, status: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_getDisplayKeywordValue(locale: super::Foundation::PSTR, keyword: super::Foundation::PSTR, displaylocale: super::Foundation::PSTR, dest: *mut u16, destcapacity: i32, status: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_getDisplayLanguage(locale: super::Foundation::PSTR, displaylocale: super::Foundation::PSTR, language: *mut u16, languagecapacity: i32, status: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_getDisplayName(localeid: super::Foundation::PSTR, inlocaleid: super::Foundation::PSTR, result: *mut u16, maxresultsize: i32, err: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_getDisplayScript(locale: super::Foundation::PSTR, displaylocale: super::Foundation::PSTR, script: *mut u16, scriptcapacity: i32, status: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_getDisplayVariant(locale: super::Foundation::PSTR, displaylocale: super::Foundation::PSTR, variant: *mut u16, variantcapacity: i32, status: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_getISO3Country(localeid: super::Foundation::PSTR) -> super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_getISO3Language(localeid: super::Foundation::PSTR) -> super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uloc_getISOCountries() -> *mut *mut i8;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uloc_getISOLanguages() -> *mut *mut i8;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_getKeywordValue(localeid: super::Foundation::PSTR, keywordname: super::Foundation::PSTR, buffer: super::Foundation::PSTR, buffercapacity: i32, status: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_getLCID(localeid: super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_getLanguage(localeid: super::Foundation::PSTR, language: super::Foundation::PSTR, languagecapacity: i32, err: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_getLineOrientation(localeid: super::Foundation::PSTR, status: *mut UErrorCode) -> ULayoutType;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_getLocaleForLCID(hostid: u32, locale: super::Foundation::PSTR, localecapacity: i32, status: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_getName(localeid: super::Foundation::PSTR, name: super::Foundation::PSTR, namecapacity: i32, err: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_getParent(localeid: super::Foundation::PSTR, parent: super::Foundation::PSTR, parentcapacity: i32, err: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_getScript(localeid: super::Foundation::PSTR, script: super::Foundation::PSTR, scriptcapacity: i32, err: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_getVariant(localeid: super::Foundation::PSTR, variant: super::Foundation::PSTR, variantcapacity: i32, err: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_isRightToLeft(locale: super::Foundation::PSTR) -> i8;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_minimizeSubtags(localeid: super::Foundation::PSTR, minimizedlocaleid: super::Foundation::PSTR, minimizedlocaleidcapacity: i32, err: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uloc_openAvailableByType(r#type: ULocAvailableType, status: *mut UErrorCode) -> *mut UEnumeration;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_openKeywords(localeid: super::Foundation::PSTR, status: *mut UErrorCode) -> *mut UEnumeration;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_setDefault(localeid: super::Foundation::PSTR, status: *mut UErrorCode);
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_setKeywordValue(keywordname: super::Foundation::PSTR, keywordvalue: super::Foundation::PSTR, buffer: super::Foundation::PSTR, buffercapacity: i32, status: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_toLanguageTag(localeid: super::Foundation::PSTR, langtag: super::Foundation::PSTR, langtagcapacity: i32, strict: i8, err: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_toLegacyKey(keyword: super::Foundation::PSTR) -> super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_toLegacyType(keyword: super::Foundation::PSTR, value: super::Foundation::PSTR) -> super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_toUnicodeLocaleKey(keyword: super::Foundation::PSTR) -> super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_toUnicodeLocaleType(keyword: super::Foundation::PSTR, value: super::Foundation::PSTR) -> super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ulocdata_close(uld: *mut ULocaleData);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ulocdata_getCLDRVersion(versionarray: *mut u8, status: *mut UErrorCode);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ulocdata_getDelimiter(uld: *mut ULocaleData, r#type: ULocaleDataDelimiterType, result: *mut u16, resultlength: i32, status: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ulocdata_getExemplarSet(uld: *mut ULocaleData, fillin: *mut USet, options: u32, extype: ULocaleDataExemplarSetType, status: *mut UErrorCode) -> *mut USet;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ulocdata_getLocaleDisplayPattern(uld: *mut ULocaleData, pattern: *mut u16, patterncapacity: i32, status: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ulocdata_getLocaleSeparator(uld: *mut ULocaleData, separator: *mut u16, separatorcapacity: i32, status: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ulocdata_getMeasurementSystem(localeid: super::Foundation::PSTR, status: *mut UErrorCode) -> UMeasurementSystem;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ulocdata_getNoSubstitute(uld: *mut ULocaleData) -> i8;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ulocdata_getPaperSize(localeid: super::Foundation::PSTR, height: *mut i32, width: *mut i32, status: *mut UErrorCode);
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ulocdata_open(localeid: super::Foundation::PSTR, status: *mut UErrorCode) -> *mut ULocaleData;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ulocdata_setNoSubstitute(uld: *mut ULocaleData, setting: i8);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn umsg_applyPattern(fmt: *mut *mut ::core::ffi::c_void, pattern: *const u16, patternlength: i32, parseerror: *mut UParseError, status: *mut UErrorCode);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn umsg_autoQuoteApostrophe(pattern: *const u16, patternlength: i32, dest: *mut u16, destcapacity: i32, ec: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn umsg_clone(fmt: *const *const ::core::ffi::c_void, status: *mut UErrorCode) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn umsg_close(format: *mut *mut ::core::ffi::c_void);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn umsg_format(fmt: *const *const ::core::ffi::c_void, result: *mut u16, resultlength: i32, status: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn umsg_getLocale(fmt: *const *const ::core::ffi::c_void) -> super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn umsg_open(pattern: *const u16, patternlength: i32, locale: super::Foundation::PSTR, parseerror: *mut UParseError, status: *mut UErrorCode) -> *mut *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn umsg_parse(fmt: *const *const ::core::ffi::c_void, source: *const u16, sourcelength: i32, count: *mut i32, status: *mut UErrorCode);
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn umsg_setLocale(fmt: *mut *mut ::core::ffi::c_void, locale: super::Foundation::PSTR);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn umsg_toPattern(fmt: *const *const ::core::ffi::c_void, result: *mut u16, resultlength: i32, status: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn umsg_vformat(fmt: *const *const ::core::ffi::c_void, result: *mut u16, resultlength: i32, ap: *mut i8, status: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn umsg_vparse(fmt: *const *const ::core::ffi::c_void, source: *const u16, sourcelength: i32, count: *mut i32, ap: *mut i8, status: *mut UErrorCode);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn umutablecptrie_buildImmutable(trie: *mut UMutableCPTrie, r#type: UCPTrieType, valuewidth: UCPTrieValueWidth, perrorcode: *mut UErrorCode) -> *mut UCPTrie;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn umutablecptrie_clone(other: *const UMutableCPTrie, perrorcode: *mut UErrorCode) -> *mut UMutableCPTrie;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn umutablecptrie_close(trie: *mut UMutableCPTrie);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn umutablecptrie_fromUCPMap(map: *const UCPMap, perrorcode: *mut UErrorCode) -> *mut UMutableCPTrie;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn umutablecptrie_fromUCPTrie(trie: *const UCPTrie, perrorcode: *mut UErrorCode) -> *mut UMutableCPTrie;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn umutablecptrie_get(trie: *const UMutableCPTrie, c: i32) -> u32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn umutablecptrie_getRange(trie: *const UMutableCPTrie, start: i32, option: UCPMapRangeOption, surrogatevalue: u32, filter: *mut ::windows::runtime::RawPtr, context: *const ::core::ffi::c_void, pvalue: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn umutablecptrie_open(initialvalue: u32, errorvalue: u32, perrorcode: *mut UErrorCode) -> *mut UMutableCPTrie;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn umutablecptrie_set(trie: *mut UMutableCPTrie, c: i32, value: u32, perrorcode: *mut UErrorCode);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn umutablecptrie_setRange(trie: *mut UMutableCPTrie, start: i32, end: i32, value: u32, perrorcode: *mut UErrorCode);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn unorm2_append(norm2: *const UNormalizer2, first: *mut u16, firstlength: i32, firstcapacity: i32, second: *const u16, secondlength: i32, perrorcode: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn unorm2_close(norm2: *mut UNormalizer2);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn unorm2_composePair(norm2: *const UNormalizer2, a: i32, b: i32) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn unorm2_getCombiningClass(norm2: *const UNormalizer2, c: i32) -> u8;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn unorm2_getDecomposition(norm2: *const UNormalizer2, c: i32, decomposition: *mut u16, capacity: i32, perrorcode: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn unorm2_getInstance(packagename: super::Foundation::PSTR, name: super::Foundation::PSTR, mode: UNormalization2Mode, perrorcode: *mut UErrorCode) -> *mut UNormalizer2;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn unorm2_getNFCInstance(perrorcode: *mut UErrorCode) -> *mut UNormalizer2;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn unorm2_getNFDInstance(perrorcode: *mut UErrorCode) -> *mut UNormalizer2;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn unorm2_getNFKCCasefoldInstance(perrorcode: *mut UErrorCode) -> *mut UNormalizer2;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn unorm2_getNFKCInstance(perrorcode: *mut UErrorCode) -> *mut UNormalizer2;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn unorm2_getNFKDInstance(perrorcode: *mut UErrorCode) -> *mut UNormalizer2;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn unorm2_getRawDecomposition(norm2: *const UNormalizer2, c: i32, decomposition: *mut u16, capacity: i32, perrorcode: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn unorm2_hasBoundaryAfter(norm2: *const UNormalizer2, c: i32) -> i8;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn unorm2_hasBoundaryBefore(norm2: *const UNormalizer2, c: i32) -> i8;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn unorm2_isInert(norm2: *const UNormalizer2, c: i32) -> i8;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn unorm2_isNormalized(norm2: *const UNormalizer2, s: *const u16, length: i32, perrorcode: *mut UErrorCode) -> i8;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn unorm2_normalize(norm2: *const UNormalizer2, src: *const u16, length: i32, dest: *mut u16, capacity: i32, perrorcode: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn unorm2_normalizeSecondAndAppend(norm2: *const UNormalizer2, first: *mut u16, firstlength: i32, firstcapacity: i32, second: *const u16, secondlength: i32, perrorcode: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn unorm2_openFiltered(norm2: *const UNormalizer2, filterset: *const USet, perrorcode: *mut UErrorCode) -> *mut UNormalizer2;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn unorm2_quickCheck(norm2: *const UNormalizer2, s: *const u16, length: i32, perrorcode: *mut UErrorCode) -> UNormalizationCheckResult;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn unorm2_spanQuickCheckYes(norm2: *const UNormalizer2, s: *const u16, length: i32, perrorcode: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn unorm_compare(s1: *const u16, length1: i32, s2: *const u16, length2: i32, options: u32, perrorcode: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn unum_applyPattern(format: *mut *mut ::core::ffi::c_void, localized: i8, pattern: *const u16, patternlength: i32, parseerror: *mut UParseError, status: *mut UErrorCode);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn unum_clone(fmt: *const *const ::core::ffi::c_void, status: *mut UErrorCode) -> *mut *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn unum_close(fmt: *mut *mut ::core::ffi::c_void);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn unum_countAvailable() -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn unum_format(fmt: *const *const ::core::ffi::c_void, number: i32, result: *mut u16, resultlength: i32, pos: *mut UFieldPosition, status: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn unum_formatDecimal(fmt: *const *const ::core::ffi::c_void, number: super::Foundation::PSTR, length: i32, result: *mut u16, resultlength: i32, pos: *mut UFieldPosition, status: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn unum_formatDouble(fmt: *const *const ::core::ffi::c_void, number: f64, result: *mut u16, resultlength: i32, pos: *mut UFieldPosition, status: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn unum_formatDoubleCurrency(fmt: *const *const ::core::ffi::c_void, number: f64, currency: *mut u16, result: *mut u16, resultlength: i32, pos: *mut UFieldPosition, status: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn unum_formatDoubleForFields(format: *const *const ::core::ffi::c_void, number: f64, result: *mut u16, resultlength: i32, fpositer: *mut UFieldPositionIterator, status: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn unum_formatInt64(fmt: *const *const ::core::ffi::c_void, number: i64, result: *mut u16, resultlength: i32, pos: *mut UFieldPosition, status: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn unum_formatUFormattable(fmt: *const *const ::core::ffi::c_void, number: *const *const ::core::ffi::c_void, result: *mut u16, resultlength: i32, pos: *mut UFieldPosition, status: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn unum_getAttribute(fmt: *const *const ::core::ffi::c_void, attr: UNumberFormatAttribute) -> i32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn unum_getAvailable(localeindex: i32) -> super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn unum_getContext(fmt: *const *const ::core::ffi::c_void, r#type: UDisplayContextType, status: *mut UErrorCode) -> UDisplayContext;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn unum_getDoubleAttribute(fmt: *const *const ::core::ffi::c_void, attr: UNumberFormatAttribute) -> f64;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn unum_getLocaleByType(fmt: *const *const ::core::ffi::c_void, r#type: ULocDataLocaleType, status: *mut UErrorCode) -> super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn unum_getSymbol(fmt: *const *const ::core::ffi::c_void, symbol: UNumberFormatSymbol, buffer: *mut u16, size: i32, status: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn unum_getTextAttribute(fmt: *const *const ::core::ffi::c_void, tag: UNumberFormatTextAttribute, result: *mut u16, resultlength: i32, status: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn unum_open(style: UNumberFormatStyle, pattern: *const u16, patternlength: i32, locale: super::Foundation::PSTR, parseerr: *mut UParseError, status: *mut UErrorCode) -> *mut *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn unum_parse(fmt: *const *const ::core::ffi::c_void, text: *const u16, textlength: i32, parsepos: *mut i32, status: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn unum_parseDecimal(fmt: *const *const ::core::ffi::c_void, text: *const u16, textlength: i32, parsepos: *mut i32, outbuf: super::Foundation::PSTR, outbuflength: i32, status: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn unum_parseDouble(fmt: *const *const ::core::ffi::c_void, text: *const u16, textlength: i32, parsepos: *mut i32, status: *mut UErrorCode) -> f64;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn unum_parseDoubleCurrency(fmt: *const *const ::core::ffi::c_void, text: *const u16, textlength: i32, parsepos: *mut i32, currency: *mut u16, status: *mut UErrorCode) -> f64;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn unum_parseInt64(fmt: *const *const ::core::ffi::c_void, text: *const u16, textlength: i32, parsepos: *mut i32, status: *mut UErrorCode) -> i64;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn unum_parseToUFormattable(fmt: *const *const ::core::ffi::c_void, result: *mut *mut ::core::ffi::c_void, text: *const u16, textlength: i32, parsepos: *mut i32, status: *mut UErrorCode) -> *mut *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn unum_setAttribute(fmt: *mut *mut ::core::ffi::c_void, attr: UNumberFormatAttribute, newvalue: i32);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn unum_setContext(fmt: *mut *mut ::core::ffi::c_void, value: UDisplayContext, status: *mut UErrorCode);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn unum_setDoubleAttribute(fmt: *mut *mut ::core::ffi::c_void, attr: UNumberFormatAttribute, newvalue: f64);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn unum_setSymbol(fmt: *mut *mut ::core::ffi::c_void, symbol: UNumberFormatSymbol, value: *const u16, length: i32, status: *mut UErrorCode);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn unum_setTextAttribute(fmt: *mut *mut ::core::ffi::c_void, tag: UNumberFormatTextAttribute, newvalue: *const u16, newvaluelength: i32, status: *mut UErrorCode);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn unum_toPattern(fmt: *const *const ::core::ffi::c_void, ispatternlocalized: i8, result: *mut u16, resultlength: i32, status: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn unumf_close(uformatter: *mut UNumberFormatter);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn unumf_closeResult(uresult: *mut UFormattedNumber);
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn unumf_formatDecimal(uformatter: *const UNumberFormatter, value: super::Foundation::PSTR, valuelen: i32, uresult: *mut UFormattedNumber, ec: *mut UErrorCode);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn unumf_formatDouble(uformatter: *const UNumberFormatter, value: f64, uresult: *mut UFormattedNumber, ec: *mut UErrorCode);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn unumf_formatInt(uformatter: *const UNumberFormatter, value: i64, uresult: *mut UFormattedNumber, ec: *mut UErrorCode);
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn unumf_openForSkeletonAndLocale(skeleton: *const u16, skeletonlen: i32, locale: super::Foundation::PSTR, ec: *mut UErrorCode) -> *mut UNumberFormatter;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn unumf_openForSkeletonAndLocaleWithError(skeleton: *const u16, skeletonlen: i32, locale: super::Foundation::PSTR, perror: *mut UParseError, ec: *mut UErrorCode) -> *mut UNumberFormatter;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn unumf_openResult(ec: *mut UErrorCode) -> *mut UFormattedNumber;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn unumf_resultAsValue(uresult: *const UFormattedNumber, ec: *mut UErrorCode) -> *mut UFormattedValue;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn unumf_resultGetAllFieldPositions(uresult: *const UFormattedNumber, ufpositer: *mut UFieldPositionIterator, ec: *mut UErrorCode);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn unumf_resultNextFieldPosition(uresult: *const UFormattedNumber, ufpos: *mut UFieldPosition, ec: *mut UErrorCode) -> i8;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn unumf_resultToString(uresult: *const UFormattedNumber, buffer: *mut u16, buffercapacity: i32, ec: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn unumsys_close(unumsys: *mut UNumberingSystem);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn unumsys_getDescription(unumsys: *const UNumberingSystem, result: *mut u16, resultlength: i32, status: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn unumsys_getName(unumsys: *const UNumberingSystem) -> super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn unumsys_getRadix(unumsys: *const UNumberingSystem) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn unumsys_isAlgorithmic(unumsys: *const UNumberingSystem) -> i8;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn unumsys_open(locale: super::Foundation::PSTR, status: *mut UErrorCode) -> *mut UNumberingSystem;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn unumsys_openAvailableNames(status: *mut UErrorCode) -> *mut UEnumeration;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn unumsys_openByName(name: super::Foundation::PSTR, status: *mut UErrorCode) -> *mut UNumberingSystem;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uplrules_close(uplrules: *mut UPluralRules);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uplrules_getKeywords(uplrules: *const UPluralRules, status: *mut UErrorCode) -> *mut UEnumeration;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn uplrules_open(locale: super::Foundation::PSTR, status: *mut UErrorCode) -> *mut UPluralRules;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn uplrules_openForType(locale: super::Foundation::PSTR, r#type: UPluralType, status: *mut UErrorCode) -> *mut UPluralRules;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uplrules_select(uplrules: *const UPluralRules, number: f64, keyword: *mut u16, capacity: i32, status: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uplrules_selectFormatted(uplrules: *const UPluralRules, number: *const UFormattedNumber, keyword: *mut u16, capacity: i32, status: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uregex_appendReplacement(regexp: *mut URegularExpression, replacementtext: *const u16, replacementlength: i32, destbuf: *mut *mut u16, destcapacity: *mut i32, status: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uregex_appendReplacementUText(regexp: *mut URegularExpression, replacementtext: *mut UText, dest: *mut UText, status: *mut UErrorCode);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uregex_appendTail(regexp: *mut URegularExpression, destbuf: *mut *mut u16, destcapacity: *mut i32, status: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uregex_appendTailUText(regexp: *mut URegularExpression, dest: *mut UText, status: *mut UErrorCode) -> *mut UText;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uregex_clone(regexp: *const URegularExpression, status: *mut UErrorCode) -> *mut URegularExpression;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uregex_close(regexp: *mut URegularExpression);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uregex_end(regexp: *mut URegularExpression, groupnum: i32, status: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uregex_end64(regexp: *mut URegularExpression, groupnum: i32, status: *mut UErrorCode) -> i64;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uregex_find(regexp: *mut URegularExpression, startindex: i32, status: *mut UErrorCode) -> i8;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uregex_find64(regexp: *mut URegularExpression, startindex: i64, status: *mut UErrorCode) -> i8;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uregex_findNext(regexp: *mut URegularExpression, status: *mut UErrorCode) -> i8;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uregex_flags(regexp: *const URegularExpression, status: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uregex_getFindProgressCallback(regexp: *const URegularExpression, callback: *mut ::windows::runtime::RawPtr, context: *const *const ::core::ffi::c_void, status: *mut UErrorCode);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uregex_getMatchCallback(regexp: *const URegularExpression, callback: *mut ::windows::runtime::RawPtr, context: *const *const ::core::ffi::c_void, status: *mut UErrorCode);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uregex_getStackLimit(regexp: *const URegularExpression, status: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uregex_getText(regexp: *mut URegularExpression, textlength: *mut i32, status: *mut UErrorCode) -> *mut u16;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uregex_getTimeLimit(regexp: *const URegularExpression, status: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uregex_getUText(regexp: *mut URegularExpression, dest: *mut UText, status: *mut UErrorCode) -> *mut UText;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uregex_group(regexp: *mut URegularExpression, groupnum: i32, dest: *mut u16, destcapacity: i32, status: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uregex_groupCount(regexp: *mut URegularExpression, status: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn uregex_groupNumberFromCName(regexp: *mut URegularExpression, groupname: super::Foundation::PSTR, namelength: i32, status: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uregex_groupNumberFromName(regexp: *mut URegularExpression, groupname: *const u16, namelength: i32, status: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uregex_groupUText(regexp: *mut URegularExpression, groupnum: i32, dest: *mut UText, grouplength: *mut i64, status: *mut UErrorCode) -> *mut UText;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uregex_hasAnchoringBounds(regexp: *const URegularExpression, status: *mut UErrorCode) -> i8;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uregex_hasTransparentBounds(regexp: *const URegularExpression, status: *mut UErrorCode) -> i8;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uregex_hitEnd(regexp: *const URegularExpression, status: *mut UErrorCode) -> i8;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uregex_lookingAt(regexp: *mut URegularExpression, startindex: i32, status: *mut UErrorCode) -> i8;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uregex_lookingAt64(regexp: *mut URegularExpression, startindex: i64, status: *mut UErrorCode) -> i8;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uregex_matches(regexp: *mut URegularExpression, startindex: i32, status: *mut UErrorCode) -> i8;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uregex_matches64(regexp: *mut URegularExpression, startindex: i64, status: *mut UErrorCode) -> i8;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uregex_open(pattern: *const u16, patternlength: i32, flags: u32, pe: *mut UParseError, status: *mut UErrorCode) -> *mut URegularExpression;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn uregex_openC(pattern: super::Foundation::PSTR, flags: u32, pe: *mut UParseError, status: *mut UErrorCode) -> *mut URegularExpression;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uregex_openUText(pattern: *mut UText, flags: u32, pe: *mut UParseError, status: *mut UErrorCode) -> *mut URegularExpression;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uregex_pattern(regexp: *const URegularExpression, patlength: *mut i32, status: *mut UErrorCode) -> *mut u16;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uregex_patternUText(regexp: *const URegularExpression, status: *mut UErrorCode) -> *mut UText;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uregex_refreshUText(regexp: *mut URegularExpression, text: *mut UText, status: *mut UErrorCode);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uregex_regionEnd(regexp: *const URegularExpression, status: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uregex_regionEnd64(regexp: *const URegularExpression, status: *mut UErrorCode) -> i64;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uregex_regionStart(regexp: *const URegularExpression, status: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uregex_regionStart64(regexp: *const URegularExpression, status: *mut UErrorCode) -> i64;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uregex_replaceAll(regexp: *mut URegularExpression, replacementtext: *const u16, replacementlength: i32, destbuf: *mut u16, destcapacity: i32, status: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uregex_replaceAllUText(regexp: *mut URegularExpression, replacement: *mut UText, dest: *mut UText, status: *mut UErrorCode) -> *mut UText;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uregex_replaceFirst(regexp: *mut URegularExpression, replacementtext: *const u16, replacementlength: i32, destbuf: *mut u16, destcapacity: i32, status: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uregex_replaceFirstUText(regexp: *mut URegularExpression, replacement: *mut UText, dest: *mut UText, status: *mut UErrorCode) -> *mut UText;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uregex_requireEnd(regexp: *const URegularExpression, status: *mut UErrorCode) -> i8;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uregex_reset(regexp: *mut URegularExpression, index: i32, status: *mut UErrorCode);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uregex_reset64(regexp: *mut URegularExpression, index: i64, status: *mut UErrorCode);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uregex_setFindProgressCallback(regexp: *mut URegularExpression, callback: ::windows::runtime::RawPtr, context: *const ::core::ffi::c_void, status: *mut UErrorCode);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uregex_setMatchCallback(regexp: *mut URegularExpression, callback: ::windows::runtime::RawPtr, context: *const ::core::ffi::c_void, status: *mut UErrorCode);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uregex_setRegion(regexp: *mut URegularExpression, regionstart: i32, regionlimit: i32, status: *mut UErrorCode);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uregex_setRegion64(regexp: *mut URegularExpression, regionstart: i64, regionlimit: i64, status: *mut UErrorCode);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uregex_setRegionAndStart(regexp: *mut URegularExpression, regionstart: i64, regionlimit: i64, startindex: i64, status: *mut UErrorCode);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uregex_setStackLimit(regexp: *mut URegularExpression, limit: i32, status: *mut UErrorCode);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uregex_setText(regexp: *mut URegularExpression, text: *const u16, textlength: i32, status: *mut UErrorCode);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uregex_setTimeLimit(regexp: *mut URegularExpression, limit: i32, status: *mut UErrorCode);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uregex_setUText(regexp: *mut URegularExpression, text: *mut UText, status: *mut UErrorCode);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uregex_split(regexp: *mut URegularExpression, destbuf: *mut u16, destcapacity: i32, requiredcapacity: *mut i32, destfields: *mut *mut u16, destfieldscapacity: i32, status: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uregex_splitUText(regexp: *mut URegularExpression, destfields: *mut *mut UText, destfieldscapacity: i32, status: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uregex_start(regexp: *mut URegularExpression, groupnum: i32, status: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uregex_start64(regexp: *mut URegularExpression, groupnum: i32, status: *mut UErrorCode) -> i64;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uregex_useAnchoringBounds(regexp: *mut URegularExpression, b: i8, status: *mut UErrorCode);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uregex_useTransparentBounds(regexp: *mut URegularExpression, b: i8, status: *mut UErrorCode);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uregion_areEqual(uregion: *const URegion, otherregion: *const URegion) -> i8;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uregion_contains(uregion: *const URegion, otherregion: *const URegion) -> i8;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uregion_getAvailable(r#type: URegionType, status: *mut UErrorCode) -> *mut UEnumeration;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uregion_getContainedRegions(uregion: *const URegion, status: *mut UErrorCode) -> *mut UEnumeration;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uregion_getContainedRegionsOfType(uregion: *const URegion, r#type: URegionType, status: *mut UErrorCode) -> *mut UEnumeration;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uregion_getContainingRegion(uregion: *const URegion) -> *mut URegion;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uregion_getContainingRegionOfType(uregion: *const URegion, r#type: URegionType) -> *mut URegion;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uregion_getNumericCode(uregion: *const URegion) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uregion_getPreferredValues(uregion: *const URegion, status: *mut UErrorCode) -> *mut UEnumeration;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn uregion_getRegionCode(uregion: *const URegion) -> super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn uregion_getRegionFromCode(regioncode: super::Foundation::PSTR, status: *mut UErrorCode) -> *mut URegion;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uregion_getRegionFromNumericCode(code: i32, status: *mut UErrorCode) -> *mut URegion;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uregion_getType(uregion: *const URegion) -> URegionType;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ureldatefmt_close(reldatefmt: *mut URelativeDateTimeFormatter);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ureldatefmt_closeResult(ufrdt: *mut UFormattedRelativeDateTime);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ureldatefmt_combineDateAndTime(reldatefmt: *const URelativeDateTimeFormatter, relativedatestring: *const u16, relativedatestringlen: i32, timestring: *const u16, timestringlen: i32, result: *mut u16, resultcapacity: i32, status: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ureldatefmt_format(reldatefmt: *const URelativeDateTimeFormatter, offset: f64, unit: URelativeDateTimeUnit, result: *mut u16, resultcapacity: i32, status: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ureldatefmt_formatNumeric(reldatefmt: *const URelativeDateTimeFormatter, offset: f64, unit: URelativeDateTimeUnit, result: *mut u16, resultcapacity: i32, status: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ureldatefmt_formatNumericToResult(reldatefmt: *const URelativeDateTimeFormatter, offset: f64, unit: URelativeDateTimeUnit, result: *mut UFormattedRelativeDateTime, status: *mut UErrorCode);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ureldatefmt_formatToResult(reldatefmt: *const URelativeDateTimeFormatter, offset: f64, unit: URelativeDateTimeUnit, result: *mut UFormattedRelativeDateTime, status: *mut UErrorCode);
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ureldatefmt_open(locale: super::Foundation::PSTR, nftoadopt: *mut *mut ::core::ffi::c_void, width: UDateRelativeDateTimeFormatterStyle, capitalizationcontext: UDisplayContext, status: *mut UErrorCode) -> *mut URelativeDateTimeFormatter;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ureldatefmt_openResult(ec: *mut UErrorCode) -> *mut UFormattedRelativeDateTime;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ureldatefmt_resultAsValue(ufrdt: *const UFormattedRelativeDateTime, ec: *mut UErrorCode) -> *mut UFormattedValue;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ures_close(resourcebundle: *mut UResourceBundle);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ures_getBinary(resourcebundle: *const UResourceBundle, len: *mut i32, status: *mut UErrorCode) -> *mut u8;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ures_getByIndex(resourcebundle: *const UResourceBundle, indexr: i32, fillin: *mut UResourceBundle, status: *mut UErrorCode) -> *mut UResourceBundle;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ures_getByKey(resourcebundle: *const UResourceBundle, key: super::Foundation::PSTR, fillin: *mut UResourceBundle, status: *mut UErrorCode) -> *mut UResourceBundle;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ures_getInt(resourcebundle: *const UResourceBundle, status: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ures_getIntVector(resourcebundle: *const UResourceBundle, len: *mut i32, status: *mut UErrorCode) -> *mut i32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ures_getKey(resourcebundle: *const UResourceBundle) -> super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ures_getLocaleByType(resourcebundle: *const UResourceBundle, r#type: ULocDataLocaleType, status: *mut UErrorCode) -> super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ures_getNextResource(resourcebundle: *mut UResourceBundle, fillin: *mut UResourceBundle, status: *mut UErrorCode) -> *mut UResourceBundle;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ures_getNextString(resourcebundle: *mut UResourceBundle, len: *mut i32, key: *const *const i8, status: *mut UErrorCode) -> *mut u16;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ures_getSize(resourcebundle: *const UResourceBundle) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ures_getString(resourcebundle: *const UResourceBundle, len: *mut i32, status: *mut UErrorCode) -> *mut u16;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ures_getStringByIndex(resourcebundle: *const UResourceBundle, indexs: i32, len: *mut i32, status: *mut UErrorCode) -> *mut u16;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ures_getStringByKey(resb: *const UResourceBundle, key: super::Foundation::PSTR, len: *mut i32, status: *mut UErrorCode) -> *mut u16;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ures_getType(resourcebundle: *const UResourceBundle) -> UResType;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ures_getUInt(resourcebundle: *const UResourceBundle, status: *mut UErrorCode) -> u32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ures_getUTF8String(resb: *const UResourceBundle, dest: super::Foundation::PSTR, length: *mut i32, forcecopy: i8, status: *mut UErrorCode) -> super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ures_getUTF8StringByIndex(resb: *const UResourceBundle, stringindex: i32, dest: super::Foundation::PSTR, plength: *mut i32, forcecopy: i8, status: *mut UErrorCode) -> super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ures_getUTF8StringByKey(resb: *const UResourceBundle, key: super::Foundation::PSTR, dest: super::Foundation::PSTR, plength: *mut i32, forcecopy: i8, status: *mut UErrorCode) -> super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ures_getVersion(resb: *const UResourceBundle, versioninfo: *mut u8);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ures_hasNext(resourcebundle: *const UResourceBundle) -> i8;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ures_open(packagename: super::Foundation::PSTR, locale: super::Foundation::PSTR, status: *mut UErrorCode) -> *mut UResourceBundle;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ures_openAvailableLocales(packagename: super::Foundation::PSTR, status: *mut UErrorCode) -> *mut UEnumeration;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ures_openDirect(packagename: super::Foundation::PSTR, locale: super::Foundation::PSTR, status: *mut UErrorCode) -> *mut UResourceBundle;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ures_openU(packagename: *const u16, locale: super::Foundation::PSTR, status: *mut UErrorCode) -> *mut UResourceBundle;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ures_resetIterator(resourcebundle: *mut UResourceBundle);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uscript_breaksBetweenLetters(script: UScriptCode) -> i8;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn uscript_getCode(nameorabbrorlocale: super::Foundation::PSTR, fillin: *mut UScriptCode, capacity: i32, err: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn uscript_getName(scriptcode: UScriptCode) -> super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uscript_getSampleString(script: UScriptCode, dest: *mut u16, capacity: i32, perrorcode: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uscript_getScript(codepoint: i32, err: *mut UErrorCode) -> UScriptCode;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uscript_getScriptExtensions(c: i32, scripts: *mut UScriptCode, capacity: i32, errorcode: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn uscript_getShortName(scriptcode: UScriptCode) -> super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uscript_getUsage(script: UScriptCode) -> UScriptUsage;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uscript_hasScript(c: i32, sc: UScriptCode) -> i8;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uscript_isCased(script: UScriptCode) -> i8;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uscript_isRightToLeft(script: UScriptCode) -> i8;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn usearch_close(searchiter: *mut UStringSearch);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn usearch_first(strsrch: *mut UStringSearch, status: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn usearch_following(strsrch: *mut UStringSearch, position: i32, status: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn usearch_getAttribute(strsrch: *const UStringSearch, attribute: USearchAttribute) -> USearchAttributeValue;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn usearch_getBreakIterator(strsrch: *const UStringSearch) -> *mut UBreakIterator;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn usearch_getCollator(strsrch: *const UStringSearch) -> *mut UCollator;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn usearch_getMatchedLength(strsrch: *const UStringSearch) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn usearch_getMatchedStart(strsrch: *const UStringSearch) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn usearch_getMatchedText(strsrch: *const UStringSearch, result: *mut u16, resultcapacity: i32, status: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn usearch_getOffset(strsrch: *const UStringSearch) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn usearch_getPattern(strsrch: *const UStringSearch, length: *mut i32) -> *mut u16;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn usearch_getText(strsrch: *const UStringSearch, length: *mut i32) -> *mut u16;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn usearch_last(strsrch: *mut UStringSearch, status: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn usearch_next(strsrch: *mut UStringSearch, status: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn usearch_open(pattern: *const u16, patternlength: i32, text: *const u16, textlength: i32, locale: super::Foundation::PSTR, breakiter: *mut UBreakIterator, status: *mut UErrorCode) -> *mut UStringSearch;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn usearch_openFromCollator(pattern: *const u16, patternlength: i32, text: *const u16, textlength: i32, collator: *const UCollator, breakiter: *mut UBreakIterator, status: *mut UErrorCode) -> *mut UStringSearch;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn usearch_preceding(strsrch: *mut UStringSearch, position: i32, status: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn usearch_previous(strsrch: *mut UStringSearch, status: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn usearch_reset(strsrch: *mut UStringSearch);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn usearch_setAttribute(strsrch: *mut UStringSearch, attribute: USearchAttribute, value: USearchAttributeValue, status: *mut UErrorCode);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn usearch_setBreakIterator(strsrch: *mut UStringSearch, breakiter: *mut UBreakIterator, status: *mut UErrorCode);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn usearch_setCollator(strsrch: *mut UStringSearch, collator: *const UCollator, status: *mut UErrorCode);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn usearch_setOffset(strsrch: *mut UStringSearch, position: i32, status: *mut UErrorCode);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn usearch_setPattern(strsrch: *mut UStringSearch, pattern: *const u16, patternlength: i32, status: *mut UErrorCode);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn usearch_setText(strsrch: *mut UStringSearch, text: *const u16, textlength: i32, status: *mut UErrorCode);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uset_add(set: *mut USet, c: i32);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uset_addAll(set: *mut USet, additionalset: *const USet);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uset_addAllCodePoints(set: *mut USet, str: *const u16, strlen: i32);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uset_addRange(set: *mut USet, start: i32, end: i32);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uset_addString(set: *mut USet, str: *const u16, strlen: i32);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uset_applyIntPropertyValue(set: *mut USet, prop: UProperty, value: i32, ec: *mut UErrorCode);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uset_applyPattern(set: *mut USet, pattern: *const u16, patternlength: i32, options: u32, status: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uset_applyPropertyAlias(set: *mut USet, prop: *const u16, proplength: i32, value: *const u16, valuelength: i32, ec: *mut UErrorCode);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uset_charAt(set: *const USet, charindex: i32) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uset_clear(set: *mut USet);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uset_clone(set: *const USet) -> *mut USet;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uset_cloneAsThawed(set: *const USet) -> *mut USet;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uset_close(set: *mut USet);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uset_closeOver(set: *mut USet, attributes: i32);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uset_compact(set: *mut USet);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uset_complement(set: *mut USet);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uset_complementAll(set: *mut USet, complement: *const USet);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uset_contains(set: *const USet, c: i32) -> i8;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uset_containsAll(set1: *const USet, set2: *const USet) -> i8;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uset_containsAllCodePoints(set: *const USet, str: *const u16, strlen: i32) -> i8;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uset_containsNone(set1: *const USet, set2: *const USet) -> i8;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uset_containsRange(set: *const USet, start: i32, end: i32) -> i8;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uset_containsSome(set1: *const USet, set2: *const USet) -> i8;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uset_containsString(set: *const USet, str: *const u16, strlen: i32) -> i8;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uset_equals(set1: *const USet, set2: *const USet) -> i8;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uset_freeze(set: *mut USet);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uset_getItem(set: *const USet, itemindex: i32, start: *mut i32, end: *mut i32, str: *mut u16, strcapacity: i32, ec: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uset_getItemCount(set: *const USet) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uset_getSerializedRange(set: *const USerializedSet, rangeindex: i32, pstart: *mut i32, pend: *mut i32) -> i8;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uset_getSerializedRangeCount(set: *const USerializedSet) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uset_getSerializedSet(fillset: *mut USerializedSet, src: *const u16, srclength: i32) -> i8;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uset_indexOf(set: *const USet, c: i32) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uset_isEmpty(set: *const USet) -> i8;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uset_isFrozen(set: *const USet) -> i8;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uset_open(start: i32, end: i32) -> *mut USet;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uset_openEmpty() -> *mut USet;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uset_openPattern(pattern: *const u16, patternlength: i32, ec: *mut UErrorCode) -> *mut USet;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uset_openPatternOptions(pattern: *const u16, patternlength: i32, options: u32, ec: *mut UErrorCode) -> *mut USet;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uset_remove(set: *mut USet, c: i32);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uset_removeAll(set: *mut USet, removeset: *const USet);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uset_removeAllStrings(set: *mut USet);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uset_removeRange(set: *mut USet, start: i32, end: i32);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uset_removeString(set: *mut USet, str: *const u16, strlen: i32);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uset_resemblesPattern(pattern: *const u16, patternlength: i32, pos: i32) -> i8;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uset_retain(set: *mut USet, start: i32, end: i32);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uset_retainAll(set: *mut USet, retain: *const USet);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uset_serialize(set: *const USet, dest: *mut u16, destcapacity: i32, perrorcode: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uset_serializedContains(set: *const USerializedSet, c: i32) -> i8;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uset_set(set: *mut USet, start: i32, end: i32);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uset_setSerializedToOne(fillset: *mut USerializedSet, c: i32);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uset_size(set: *const USet) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uset_span(set: *const USet, s: *const u16, length: i32, spancondition: USetSpanCondition) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uset_spanBack(set: *const USet, s: *const u16, length: i32, spancondition: USetSpanCondition) -> i32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn uset_spanBackUTF8(set: *const USet, s: super::Foundation::PSTR, length: i32, spancondition: USetSpanCondition) -> i32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn uset_spanUTF8(set: *const USet, s: super::Foundation::PSTR, length: i32, spancondition: USetSpanCondition) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uset_toPattern(set: *const USet, result: *mut u16, resultcapacity: i32, escapeunprintable: i8, ec: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uspoof_areConfusable(sc: *const USpoofChecker, id1: *const u16, length1: i32, id2: *const u16, length2: i32, status: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn uspoof_areConfusableUTF8(sc: *const USpoofChecker, id1: super::Foundation::PSTR, length1: i32, id2: super::Foundation::PSTR, length2: i32, status: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uspoof_check(sc: *const USpoofChecker, id: *const u16, length: i32, position: *mut i32, status: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uspoof_check2(sc: *const USpoofChecker, id: *const u16, length: i32, checkresult: *mut USpoofCheckResult, status: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn uspoof_check2UTF8(sc: *const USpoofChecker, id: super::Foundation::PSTR, length: i32, checkresult: *mut USpoofCheckResult, status: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn uspoof_checkUTF8(sc: *const USpoofChecker, id: super::Foundation::PSTR, length: i32, position: *mut i32, status: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uspoof_clone(sc: *const USpoofChecker, status: *mut UErrorCode) -> *mut USpoofChecker;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uspoof_close(sc: *mut USpoofChecker);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uspoof_closeCheckResult(checkresult: *mut USpoofCheckResult);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uspoof_getAllowedChars(sc: *const USpoofChecker, status: *mut UErrorCode) -> *mut USet;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn uspoof_getAllowedLocales(sc: *mut USpoofChecker, status: *mut UErrorCode) -> super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uspoof_getCheckResultChecks(checkresult: *const USpoofCheckResult, status: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uspoof_getCheckResultNumerics(checkresult: *const USpoofCheckResult, status: *mut UErrorCode) -> *mut USet;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uspoof_getCheckResultRestrictionLevel(checkresult: *const USpoofCheckResult, status: *mut UErrorCode) -> URestrictionLevel;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uspoof_getChecks(sc: *const USpoofChecker, status: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uspoof_getInclusionSet(status: *mut UErrorCode) -> *mut USet;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uspoof_getRecommendedSet(status: *mut UErrorCode) -> *mut USet;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uspoof_getRestrictionLevel(sc: *const USpoofChecker) -> URestrictionLevel;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uspoof_getSkeleton(sc: *const USpoofChecker, r#type: u32, id: *const u16, length: i32, dest: *mut u16, destcapacity: i32, status: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn uspoof_getSkeletonUTF8(sc: *const USpoofChecker, r#type: u32, id: super::Foundation::PSTR, length: i32, dest: super::Foundation::PSTR, destcapacity: i32, status: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uspoof_open(status: *mut UErrorCode) -> *mut USpoofChecker;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uspoof_openCheckResult(status: *mut UErrorCode) -> *mut USpoofCheckResult;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uspoof_openFromSerialized(data: *const ::core::ffi::c_void, length: i32, pactuallength: *mut i32, perrorcode: *mut UErrorCode) -> *mut USpoofChecker;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn uspoof_openFromSource(confusables: super::Foundation::PSTR, confusableslen: i32, confusableswholescript: super::Foundation::PSTR, confusableswholescriptlen: i32, errtype: *mut i32, pe: *mut UParseError, status: *mut UErrorCode) -> *mut USpoofChecker;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uspoof_serialize(sc: *mut USpoofChecker, data: *mut ::core::ffi::c_void, capacity: i32, status: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uspoof_setAllowedChars(sc: *mut USpoofChecker, chars: *const USet, status: *mut UErrorCode);
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn uspoof_setAllowedLocales(sc: *mut USpoofChecker, localeslist: super::Foundation::PSTR, status: *mut UErrorCode);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uspoof_setChecks(sc: *mut USpoofChecker, checks: i32, status: *mut UErrorCode);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uspoof_setRestrictionLevel(sc: *mut USpoofChecker, restrictionlevel: URestrictionLevel);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn usprep_close(profile: *mut UStringPrepProfile);
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn usprep_open(path: super::Foundation::PSTR, filename: super::Foundation::PSTR, status: *mut UErrorCode) -> *mut UStringPrepProfile;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn usprep_openByType(r#type: UStringPrepProfileType, status: *mut UErrorCode) -> *mut UStringPrepProfile;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn usprep_prepare(prep: *const UStringPrepProfile, src: *const u16, srclength: i32, dest: *mut u16, destcapacity: i32, options: i32, parseerror: *mut UParseError, status: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn utext_char32At(ut: *mut UText, nativeindex: i64) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn utext_clone(dest: *mut UText, src: *const UText, deep: i8, readonly: i8, status: *mut UErrorCode) -> *mut UText;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn utext_close(ut: *mut UText) -> *mut UText;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn utext_copy(ut: *mut UText, nativestart: i64, nativelimit: i64, destindex: i64, r#move: i8, status: *mut UErrorCode);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn utext_current32(ut: *mut UText) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn utext_equals(a: *const UText, b: *const UText) -> i8;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn utext_extract(ut: *mut UText, nativestart: i64, nativelimit: i64, dest: *mut u16, destcapacity: i32, status: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn utext_freeze(ut: *mut UText);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn utext_getNativeIndex(ut: *const UText) -> i64;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn utext_getPreviousNativeIndex(ut: *mut UText) -> i64;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn utext_hasMetaData(ut: *const UText) -> i8;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn utext_isLengthExpensive(ut: *const UText) -> i8;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn utext_isWritable(ut: *const UText) -> i8;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn utext_moveIndex32(ut: *mut UText, delta: i32) -> i8;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn utext_nativeLength(ut: *mut UText) -> i64;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn utext_next32(ut: *mut UText) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn utext_next32From(ut: *mut UText, nativeindex: i64) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn utext_openUChars(ut: *mut UText, s: *const u16, length: i64, status: *mut UErrorCode) -> *mut UText;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn utext_openUTF8(ut: *mut UText, s: super::Foundation::PSTR, length: i64, status: *mut UErrorCode) -> *mut UText;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn utext_previous32(ut: *mut UText) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn utext_previous32From(ut: *mut UText, nativeindex: i64) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn utext_replace(ut: *mut UText, nativestart: i64, nativelimit: i64, replacementtext: *const u16, replacementlength: i32, status: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn utext_setNativeIndex(ut: *mut UText, nativeindex: i64);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn utext_setup(ut: *mut UText, extraspace: i32, status: *mut UErrorCode) -> *mut UText;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn utf8_appendCharSafeBody(s: *mut u8, i: i32, length: i32, c: i32, piserror: *mut i8) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn utf8_back1SafeBody(s: *const u8, start: i32, i: i32) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn utf8_nextCharSafeBody(s: *const u8, pi: *mut i32, length: i32, c: i32, strict: i8) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn utf8_prevCharSafeBody(s: *const u8, start: i32, pi: *mut i32, c: i32, strict: i8) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn utmscale_fromInt64(othertime: i64, timescale: UDateTimeScale, status: *mut UErrorCode) -> i64;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn utmscale_getTimeScaleValue(timescale: UDateTimeScale, value: UTimeScaleValue, status: *mut UErrorCode) -> i64;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn utmscale_toInt64(universaltime: i64, timescale: UDateTimeScale, status: *mut UErrorCode) -> i64;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn utrace_format(outbuf: super::Foundation::PSTR, capacity: i32, indent: i32, fmt: super::Foundation::PSTR) -> i32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn utrace_functionName(fnnumber: i32) -> super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn utrace_getFunctions(context: *const *const ::core::ffi::c_void, e: *mut ::windows::runtime::RawPtr, x: *mut ::windows::runtime::RawPtr, d: *mut ::windows::runtime::RawPtr);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn utrace_getLevel() -> i32;
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn utrace_setFunctions(context: *const ::core::ffi::c_void, e: ::windows::runtime::RawPtr, x: ::windows::runtime::RawPtr, d: ::windows::runtime::RawPtr);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn utrace_setLevel(tracelevel: i32);
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn utrace_vformat(outbuf: super::Foundation::PSTR, capacity: i32, indent: i32, fmt: super::Foundation::PSTR, args: *mut i8) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn utrans_clone(trans: *const *const ::core::ffi::c_void, status: *mut UErrorCode) -> *mut *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn utrans_close(trans: *mut *mut ::core::ffi::c_void);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn utrans_countAvailableIDs() -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn utrans_getSourceSet(trans: *const *const ::core::ffi::c_void, ignorefilter: i8, fillin: *mut USet, status: *mut UErrorCode) -> *mut USet;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn utrans_getUnicodeID(trans: *const *const ::core::ffi::c_void, resultlength: *mut i32) -> *mut u16;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn utrans_openIDs(perrorcode: *mut UErrorCode) -> *mut UEnumeration;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn utrans_openInverse(trans: *const *const ::core::ffi::c_void, status: *mut UErrorCode) -> *mut *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn utrans_openU(id: *const u16, idlength: i32, dir: UTransDirection, rules: *const u16, ruleslength: i32, parseerror: *mut UParseError, perrorcode: *mut UErrorCode) -> *mut *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn utrans_register(adoptedtrans: *mut *mut ::core::ffi::c_void, status: *mut UErrorCode);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn utrans_setFilter(trans: *mut *mut ::core::ffi::c_void, filterpattern: *const u16, filterpatternlen: i32, status: *mut UErrorCode);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn utrans_toRules(trans: *const *const ::core::ffi::c_void, escapeunprintable: i8, result: *mut u16, resultlength: i32, status: *mut UErrorCode) -> i32;
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn utrans_trans(trans: *const *const ::core::ffi::c_void, rep: *mut *mut ::core::ffi::c_void, repfunc: *const UReplaceableCallbacks, start: i32, limit: *mut i32, status: *mut UErrorCode);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn utrans_transIncremental(trans: *const *const ::core::ffi::c_void, rep: *mut *mut ::core::ffi::c_void, repfunc: *const UReplaceableCallbacks, pos: *mut UTransPosition, status: *mut UErrorCode);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn utrans_transIncrementalUChars(trans: *const *const ::core::ffi::c_void, text: *mut u16, textlength: *mut i32, textcapacity: i32, pos: *mut UTransPosition, status: *mut UErrorCode);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn utrans_transUChars(trans: *const *const ::core::ffi::c_void, text: *mut u16, textlength: *mut i32, textcapacity: i32, start: i32, limit: *mut i32, status: *mut UErrorCode);
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn utrans_unregisterID(id: *const u16, idlength: i32);
}
