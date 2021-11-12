#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    pub fn CompareStringA(locale: u32, dwcmpflags: u32, lpstring1: *const i8, cchcount1: i32, lpstring2: *const i8, cchcount2: i32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CompareStringEx(lplocalename: super::Foundation::PWSTR, dwcmpflags: COMPARE_STRING_FLAGS, lpstring1: super::Foundation::PWSTR, cchcount1: i32, lpstring2: super::Foundation::PWSTR, cchcount2: i32, lpversioninformation: *mut NLSVERSIONINFO, lpreserved: *mut ::core::ffi::c_void, lparam: super::Foundation::LPARAM) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CompareStringOrdinal(lpstring1: super::Foundation::PWSTR, cchcount1: i32, lpstring2: super::Foundation::PWSTR, cchcount2: i32, bignorecase: super::Foundation::BOOL) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CompareStringW(locale: u32, dwcmpflags: u32, lpstring1: super::Foundation::PWSTR, cchcount1: i32, lpstring2: super::Foundation::PWSTR, cchcount2: i32) -> i32;
    pub fn ConvertDefaultLocale(locale: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumCalendarInfoA(lpcalinfoenumproc: CALINFO_ENUMPROCA, locale: u32, calendar: u32, caltype: u32) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumCalendarInfoExA(lpcalinfoenumprocex: CALINFO_ENUMPROCEXA, locale: u32, calendar: u32, caltype: u32) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumCalendarInfoExEx(pcalinfoenumprocexex: CALINFO_ENUMPROCEXEX, lplocalename: super::Foundation::PWSTR, calendar: u32, lpreserved: super::Foundation::PWSTR, caltype: u32, lparam: super::Foundation::LPARAM) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumCalendarInfoExW(lpcalinfoenumprocex: CALINFO_ENUMPROCEXW, locale: u32, calendar: u32, caltype: u32) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumCalendarInfoW(lpcalinfoenumproc: CALINFO_ENUMPROCW, locale: u32, calendar: u32, caltype: u32) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumDateFormatsA(lpdatefmtenumproc: DATEFMT_ENUMPROCA, locale: u32, dwflags: u32) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumDateFormatsExA(lpdatefmtenumprocex: DATEFMT_ENUMPROCEXA, locale: u32, dwflags: u32) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumDateFormatsExEx(lpdatefmtenumprocexex: DATEFMT_ENUMPROCEXEX, lplocalename: super::Foundation::PWSTR, dwflags: ENUM_DATE_FORMATS_FLAGS, lparam: super::Foundation::LPARAM) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumDateFormatsExW(lpdatefmtenumprocex: DATEFMT_ENUMPROCEXW, locale: u32, dwflags: u32) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumDateFormatsW(lpdatefmtenumproc: DATEFMT_ENUMPROCW, locale: u32, dwflags: u32) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumLanguageGroupLocalesA(lplanggrouplocaleenumproc: LANGGROUPLOCALE_ENUMPROCA, languagegroup: u32, dwflags: u32, lparam: isize) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumLanguageGroupLocalesW(lplanggrouplocaleenumproc: LANGGROUPLOCALE_ENUMPROCW, languagegroup: u32, dwflags: u32, lparam: isize) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumSystemCodePagesA(lpcodepageenumproc: CODEPAGE_ENUMPROCA, dwflags: ENUM_SYSTEM_CODE_PAGES_FLAGS) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumSystemCodePagesW(lpcodepageenumproc: CODEPAGE_ENUMPROCW, dwflags: ENUM_SYSTEM_CODE_PAGES_FLAGS) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumSystemGeoID(geoclass: u32, parentgeoid: i32, lpgeoenumproc: GEO_ENUMPROC) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumSystemGeoNames(geoclass: u32, geoenumproc: GEO_ENUMNAMEPROC, data: super::Foundation::LPARAM) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumSystemLanguageGroupsA(lplanguagegroupenumproc: LANGUAGEGROUP_ENUMPROCA, dwflags: ENUM_SYSTEM_LANGUAGE_GROUPS_FLAGS, lparam: isize) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumSystemLanguageGroupsW(lplanguagegroupenumproc: LANGUAGEGROUP_ENUMPROCW, dwflags: ENUM_SYSTEM_LANGUAGE_GROUPS_FLAGS, lparam: isize) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumSystemLocalesA(lplocaleenumproc: LOCALE_ENUMPROCA, dwflags: u32) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumSystemLocalesEx(lplocaleenumprocex: LOCALE_ENUMPROCEX, dwflags: u32, lparam: super::Foundation::LPARAM, lpreserved: *const ::core::ffi::c_void) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumSystemLocalesW(lplocaleenumproc: LOCALE_ENUMPROCW, dwflags: u32) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumTimeFormatsA(lptimefmtenumproc: TIMEFMT_ENUMPROCA, locale: u32, dwflags: TIME_FORMAT_FLAGS) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumTimeFormatsEx(lptimefmtenumprocex: TIMEFMT_ENUMPROCEX, lplocalename: super::Foundation::PWSTR, dwflags: u32, lparam: super::Foundation::LPARAM) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumTimeFormatsW(lptimefmtenumproc: TIMEFMT_ENUMPROCW, locale: u32, dwflags: TIME_FORMAT_FLAGS) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumUILanguagesA(lpuilanguageenumproc: UILANGUAGE_ENUMPROCA, dwflags: u32, lparam: isize) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumUILanguagesW(lpuilanguageenumproc: UILANGUAGE_ENUMPROCW, dwflags: u32, lparam: isize) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindNLSString(locale: u32, dwfindnlsstringflags: u32, lpstringsource: super::Foundation::PWSTR, cchsource: i32, lpstringvalue: super::Foundation::PWSTR, cchvalue: i32, pcchfound: *mut i32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindNLSStringEx(lplocalename: super::Foundation::PWSTR, dwfindnlsstringflags: u32, lpstringsource: super::Foundation::PWSTR, cchsource: i32, lpstringvalue: super::Foundation::PWSTR, cchvalue: i32, pcchfound: *mut i32, lpversioninformation: *const NLSVERSIONINFO, lpreserved: *const ::core::ffi::c_void, sorthandle: super::Foundation::LPARAM) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindStringOrdinal(dwfindstringordinalflags: u32, lpstringsource: super::Foundation::PWSTR, cchsource: i32, lpstringvalue: super::Foundation::PWSTR, cchvalue: i32, bignorecase: super::Foundation::BOOL) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FoldStringA(dwmapflags: FOLD_STRING_MAP_FLAGS, lpsrcstr: super::Foundation::PSTR, cchsrc: i32, lpdeststr: super::Foundation::PSTR, cchdest: i32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FoldStringW(dwmapflags: FOLD_STRING_MAP_FLAGS, lpsrcstr: super::Foundation::PWSTR, cchsrc: i32, lpdeststr: super::Foundation::PWSTR, cchdest: i32) -> i32;
    pub fn GetACP() -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCPInfo(codepage: u32, lpcpinfo: *mut CPINFO) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCPInfoExA(codepage: u32, dwflags: u32, lpcpinfoex: *mut CPINFOEXA) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCPInfoExW(codepage: u32, dwflags: u32, lpcpinfoex: *mut CPINFOEXW) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCalendarInfoA(locale: u32, calendar: u32, caltype: u32, lpcaldata: super::Foundation::PSTR, cchdata: i32, lpvalue: *mut u32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCalendarInfoEx(lplocalename: super::Foundation::PWSTR, calendar: u32, lpreserved: super::Foundation::PWSTR, caltype: u32, lpcaldata: super::Foundation::PWSTR, cchdata: i32, lpvalue: *mut u32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCalendarInfoW(locale: u32, calendar: u32, caltype: u32, lpcaldata: super::Foundation::PWSTR, cchdata: i32, lpvalue: *mut u32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCurrencyFormatA(locale: u32, dwflags: u32, lpvalue: super::Foundation::PSTR, lpformat: *const CURRENCYFMTA, lpcurrencystr: super::Foundation::PSTR, cchcurrency: i32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCurrencyFormatEx(lplocalename: super::Foundation::PWSTR, dwflags: u32, lpvalue: super::Foundation::PWSTR, lpformat: *const CURRENCYFMTW, lpcurrencystr: super::Foundation::PWSTR, cchcurrency: i32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCurrencyFormatW(locale: u32, dwflags: u32, lpvalue: super::Foundation::PWSTR, lpformat: *const CURRENCYFMTW, lpcurrencystr: super::Foundation::PWSTR, cchcurrency: i32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDateFormatA(locale: u32, dwflags: u32, lpdate: *const super::Foundation::SYSTEMTIME, lpformat: super::Foundation::PSTR, lpdatestr: super::Foundation::PSTR, cchdate: i32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDateFormatEx(lplocalename: super::Foundation::PWSTR, dwflags: ENUM_DATE_FORMATS_FLAGS, lpdate: *const super::Foundation::SYSTEMTIME, lpformat: super::Foundation::PWSTR, lpdatestr: super::Foundation::PWSTR, cchdate: i32, lpcalendar: super::Foundation::PWSTR) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDateFormatW(locale: u32, dwflags: u32, lpdate: *const super::Foundation::SYSTEMTIME, lpformat: super::Foundation::PWSTR, lpdatestr: super::Foundation::PWSTR, cchdate: i32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDistanceOfClosestLanguageInList(pszlanguage: super::Foundation::PWSTR, pszlanguageslist: super::Foundation::PWSTR, wchlistdelimiter: u16, pclosestdistance: *mut f64) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDurationFormat(locale: u32, dwflags: u32, lpduration: *const super::Foundation::SYSTEMTIME, ullduration: u64, lpformat: super::Foundation::PWSTR, lpdurationstr: super::Foundation::PWSTR, cchduration: i32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDurationFormatEx(lplocalename: super::Foundation::PWSTR, dwflags: u32, lpduration: *const super::Foundation::SYSTEMTIME, ullduration: u64, lpformat: super::Foundation::PWSTR, lpdurationstr: super::Foundation::PWSTR, cchduration: i32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFileMUIInfo(dwflags: u32, pcwszfilepath: super::Foundation::PWSTR, pfilemuiinfo: *mut FILEMUIINFO, pcbfilemuiinfo: *mut u32) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFileMUIPath(dwflags: u32, pcwszfilepath: super::Foundation::PWSTR, pwszlanguage: super::Foundation::PWSTR, pcchlanguage: *mut u32, pwszfilemuipath: super::Foundation::PWSTR, pcchfilemuipath: *mut u32, pululenumerator: *mut u64) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetGeoInfoA(location: i32, geotype: u32, lpgeodata: super::Foundation::PSTR, cchdata: i32, langid: u16) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetGeoInfoEx(location: super::Foundation::PWSTR, geotype: u32, geodata: super::Foundation::PWSTR, geodatacount: i32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetGeoInfoW(location: i32, geotype: u32, lpgeodata: super::Foundation::PWSTR, cchdata: i32, langid: u16) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetLocaleInfoA(locale: u32, lctype: u32, lplcdata: super::Foundation::PSTR, cchdata: i32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetLocaleInfoEx(lplocalename: super::Foundation::PWSTR, lctype: u32, lplcdata: super::Foundation::PWSTR, cchdata: i32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetLocaleInfoW(locale: u32, lctype: u32, lplcdata: super::Foundation::PWSTR, cchdata: i32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetNLSVersion(function: u32, locale: u32, lpversioninformation: *mut NLSVERSIONINFO) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetNLSVersionEx(function: u32, lplocalename: super::Foundation::PWSTR, lpversioninformation: *mut NLSVERSIONINFOEX) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetNumberFormatA(locale: u32, dwflags: u32, lpvalue: super::Foundation::PSTR, lpformat: *const NUMBERFMTA, lpnumberstr: super::Foundation::PSTR, cchnumber: i32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetNumberFormatEx(lplocalename: super::Foundation::PWSTR, dwflags: u32, lpvalue: super::Foundation::PWSTR, lpformat: *const NUMBERFMTW, lpnumberstr: super::Foundation::PWSTR, cchnumber: i32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetNumberFormatW(locale: u32, dwflags: u32, lpvalue: super::Foundation::PWSTR, lpformat: *const NUMBERFMTW, lpnumberstr: super::Foundation::PWSTR, cchnumber: i32) -> i32;
    pub fn GetOEMCP() -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetProcessPreferredUILanguages(dwflags: u32, pulnumlanguages: *mut u32, pwszlanguagesbuffer: super::Foundation::PWSTR, pcchlanguagesbuffer: *mut u32) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetStringScripts(dwflags: u32, lpstring: super::Foundation::PWSTR, cchstring: i32, lpscripts: super::Foundation::PWSTR, cchscripts: i32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetStringTypeA(locale: u32, dwinfotype: u32, lpsrcstr: super::Foundation::PSTR, cchsrc: i32, lpchartype: *mut u16) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetStringTypeExA(locale: u32, dwinfotype: u32, lpsrcstr: super::Foundation::PSTR, cchsrc: i32, lpchartype: *mut u16) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetStringTypeExW(locale: u32, dwinfotype: u32, lpsrcstr: super::Foundation::PWSTR, cchsrc: i32, lpchartype: *mut u16) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetStringTypeW(dwinfotype: u32, lpsrcstr: super::Foundation::PWSTR, cchsrc: i32, lpchartype: *mut u16) -> super::Foundation::BOOL;
    pub fn GetSystemDefaultLCID() -> u32;
    pub fn GetSystemDefaultLangID() -> u16;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetSystemDefaultLocaleName(lplocalename: super::Foundation::PWSTR, cchlocalename: i32) -> i32;
    pub fn GetSystemDefaultUILanguage() -> u16;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetSystemPreferredUILanguages(dwflags: u32, pulnumlanguages: *mut u32, pwszlanguagesbuffer: super::Foundation::PWSTR, pcchlanguagesbuffer: *mut u32) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn GetTextCharset(hdc: super::Graphics::Gdi::HDC) -> i32;
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn GetTextCharsetInfo(hdc: super::Graphics::Gdi::HDC, lpsig: *mut FONTSIGNATURE, dwflags: u32) -> i32;
    pub fn GetThreadLocale() -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetThreadPreferredUILanguages(dwflags: u32, pulnumlanguages: *mut u32, pwszlanguagesbuffer: super::Foundation::PWSTR, pcchlanguagesbuffer: *mut u32) -> super::Foundation::BOOL;
    pub fn GetThreadUILanguage() -> u16;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetTimeFormatA(locale: u32, dwflags: u32, lptime: *const super::Foundation::SYSTEMTIME, lpformat: super::Foundation::PSTR, lptimestr: super::Foundation::PSTR, cchtime: i32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetTimeFormatEx(lplocalename: super::Foundation::PWSTR, dwflags: TIME_FORMAT_FLAGS, lptime: *const super::Foundation::SYSTEMTIME, lpformat: super::Foundation::PWSTR, lptimestr: super::Foundation::PWSTR, cchtime: i32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetTimeFormatW(locale: u32, dwflags: u32, lptime: *const super::Foundation::SYSTEMTIME, lpformat: super::Foundation::PWSTR, lptimestr: super::Foundation::PWSTR, cchtime: i32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetUILanguageInfo(dwflags: u32, pwmszlanguage: super::Foundation::PWSTR, pwszfallbacklanguages: super::Foundation::PWSTR, pcchfallbacklanguages: *mut u32, pattributes: *mut u32) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetUserDefaultGeoName(geoname: super::Foundation::PWSTR, geonamecount: i32) -> i32;
    pub fn GetUserDefaultLCID() -> u32;
    pub fn GetUserDefaultLangID() -> u16;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetUserDefaultLocaleName(lplocalename: super::Foundation::PWSTR, cchlocalename: i32) -> i32;
    pub fn GetUserDefaultUILanguage() -> u16;
    pub fn GetUserGeoID(geoclass: u32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetUserPreferredUILanguages(dwflags: u32, pulnumlanguages: *mut u32, pwszlanguagesbuffer: super::Foundation::PWSTR, pcchlanguagesbuffer: *mut u32) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn IdnToAscii(dwflags: u32, lpunicodecharstr: super::Foundation::PWSTR, cchunicodechar: i32, lpasciicharstr: super::Foundation::PWSTR, cchasciichar: i32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn IdnToNameprepUnicode(dwflags: u32, lpunicodecharstr: super::Foundation::PWSTR, cchunicodechar: i32, lpnameprepcharstr: super::Foundation::PWSTR, cchnameprepchar: i32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn IdnToUnicode(dwflags: u32, lpasciicharstr: super::Foundation::PWSTR, cchasciichar: i32, lpunicodecharstr: super::Foundation::PWSTR, cchunicodechar: i32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsDBCSLeadByte(testchar: u8) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsDBCSLeadByteEx(codepage: u32, testchar: u8) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsNLSDefinedString(function: u32, dwflags: u32, lpversioninformation: *const NLSVERSIONINFO, lpstring: super::Foundation::PWSTR, cchstr: i32) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsNormalizedString(normform: NORM_FORM, lpstring: super::Foundation::PWSTR, cwlength: i32) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsTextUnicode(lpv: *const ::core::ffi::c_void, isize: i32, lpiresult: *mut IS_TEXT_UNICODE_RESULT) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsValidCodePage(codepage: u32) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsValidLanguageGroup(languagegroup: u32, dwflags: ENUM_SYSTEM_LANGUAGE_GROUPS_FLAGS) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsValidLocale(locale: u32, dwflags: IS_VALID_LOCALE_FLAGS) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsValidLocaleName(lplocalename: super::Foundation::PWSTR) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsValidNLSVersion(function: u32, lplocalename: super::Foundation::PWSTR, lpversioninformation: *const NLSVERSIONINFOEX) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsWellFormedTag(psztag: super::Foundation::PWSTR) -> u8;
    #[cfg(feature = "Win32_Foundation")]
    pub fn LCIDToLocaleName(locale: u32, lpname: super::Foundation::PWSTR, cchname: i32, dwflags: u32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn LCMapStringA(locale: u32, dwmapflags: u32, lpsrcstr: super::Foundation::PSTR, cchsrc: i32, lpdeststr: super::Foundation::PSTR, cchdest: i32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn LCMapStringEx(lplocalename: super::Foundation::PWSTR, dwmapflags: u32, lpsrcstr: super::Foundation::PWSTR, cchsrc: i32, lpdeststr: super::Foundation::PWSTR, cchdest: i32, lpversioninformation: *const NLSVERSIONINFO, lpreserved: *const ::core::ffi::c_void, sorthandle: super::Foundation::LPARAM) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn LCMapStringW(locale: u32, dwmapflags: u32, lpsrcstr: super::Foundation::PWSTR, cchsrc: i32, lpdeststr: super::Foundation::PWSTR, cchdest: i32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn LocaleNameToLCID(lpname: super::Foundation::PWSTR, dwflags: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MappingDoAction(pbag: *mut MAPPING_PROPERTY_BAG, dwrangeindex: u32, pszactionid: super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MappingFreePropertyBag(pbag: *const MAPPING_PROPERTY_BAG) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MappingFreeServices(pserviceinfo: *const MAPPING_SERVICE_INFO) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MappingGetServices(poptions: *const MAPPING_ENUM_OPTIONS, prgservices: *mut *mut MAPPING_SERVICE_INFO, pdwservicescount: *mut u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MappingRecognizeText(pserviceinfo: *const MAPPING_SERVICE_INFO, psztext: super::Foundation::PWSTR, dwlength: u32, dwindex: u32, poptions: *const MAPPING_OPTIONS, pbag: *mut MAPPING_PROPERTY_BAG) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MultiByteToWideChar(codepage: u32, dwflags: MULTI_BYTE_TO_WIDE_CHAR_FLAGS, lpmultibytestr: super::Foundation::PSTR, cbmultibyte: i32, lpwidecharstr: super::Foundation::PWSTR, cchwidechar: i32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NormalizeString(normform: NORM_FORM, lpsrcstring: super::Foundation::PWSTR, cwsrclength: i32, lpdststring: super::Foundation::PWSTR, cwdstlength: i32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NotifyUILanguageChange(dwflags: u32, pcwstrnewlanguage: super::Foundation::PWSTR, pcwstrpreviouslanguage: super::Foundation::PWSTR, dwreserved: u32, pdwstatusrtrn: *mut u32) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResolveLocaleName(lpnametoresolve: super::Foundation::PWSTR, lplocalename: super::Foundation::PWSTR, cchlocalename: i32) -> i32;
    pub fn RestoreThreadPreferredUILanguages(snapshot: HSAVEDUILANGUAGES);
    pub fn ScriptApplyDigitSubstitution(psds: *const SCRIPT_DIGITSUBSTITUTE, psc: *mut SCRIPT_CONTROL, pss: *mut SCRIPT_STATE) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn ScriptApplyLogicalWidth(pidx: *const i32, cchars: i32, cglyphs: i32, pwlogclust: *const u16, psva: *const SCRIPT_VISATTR, piadvance: *const i32, psa: *const SCRIPT_ANALYSIS, pabc: *mut super::Graphics::Gdi::ABC, pijustify: *mut i32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ScriptBreak(pwcchars: super::Foundation::PWSTR, cchars: i32, psa: *const SCRIPT_ANALYSIS, psla: *mut SCRIPT_LOGATTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ScriptCPtoX(icp: i32, ftrailing: super::Foundation::BOOL, cchars: i32, cglyphs: i32, pwlogclust: *const u16, psva: *const SCRIPT_VISATTR, piadvance: *const i32, psa: *const SCRIPT_ANALYSIS, pix: *mut i32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn ScriptCacheGetHeight(hdc: super::Graphics::Gdi::HDC, psc: *mut *mut ::core::ffi::c_void, tmheight: *mut i32) -> ::windows_sys::core::HRESULT;
    pub fn ScriptFreeCache(psc: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn ScriptGetCMap(hdc: super::Graphics::Gdi::HDC, psc: *mut *mut ::core::ffi::c_void, pwcinchars: super::Foundation::PWSTR, cchars: i32, dwflags: u32, pwoutglyphs: *mut u16) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn ScriptGetFontAlternateGlyphs(hdc: super::Graphics::Gdi::HDC, psc: *mut *mut ::core::ffi::c_void, psa: *const SCRIPT_ANALYSIS, tagscript: u32, taglangsys: u32, tagfeature: u32, wglyphid: u16, cmaxalternates: i32, palternateglyphs: *mut u16, pcalternates: *mut i32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn ScriptGetFontFeatureTags(hdc: super::Graphics::Gdi::HDC, psc: *mut *mut ::core::ffi::c_void, psa: *const SCRIPT_ANALYSIS, tagscript: u32, taglangsys: u32, cmaxtags: i32, pfeaturetags: *mut u32, pctags: *mut i32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn ScriptGetFontLanguageTags(hdc: super::Graphics::Gdi::HDC, psc: *mut *mut ::core::ffi::c_void, psa: *const SCRIPT_ANALYSIS, tagscript: u32, cmaxtags: i32, plangsystags: *mut u32, pctags: *mut i32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn ScriptGetFontProperties(hdc: super::Graphics::Gdi::HDC, psc: *mut *mut ::core::ffi::c_void, sfp: *mut SCRIPT_FONTPROPERTIES) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn ScriptGetFontScriptTags(hdc: super::Graphics::Gdi::HDC, psc: *mut *mut ::core::ffi::c_void, psa: *const SCRIPT_ANALYSIS, cmaxtags: i32, pscripttags: *mut u32, pctags: *mut i32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn ScriptGetGlyphABCWidth(hdc: super::Graphics::Gdi::HDC, psc: *mut *mut ::core::ffi::c_void, wglyph: u16, pabc: *mut super::Graphics::Gdi::ABC) -> ::windows_sys::core::HRESULT;
    pub fn ScriptGetLogicalWidths(psa: *const SCRIPT_ANALYSIS, cchars: i32, cglyphs: i32, piglyphwidth: *const i32, pwlogclust: *const u16, psva: *const SCRIPT_VISATTR, pidx: *const i32) -> ::windows_sys::core::HRESULT;
    pub fn ScriptGetProperties(ppsp: *mut *mut *mut SCRIPT_PROPERTIES, pinumscripts: *mut i32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ScriptIsComplex(pwcinchars: super::Foundation::PWSTR, cinchars: i32, dwflags: SCRIPT_IS_COMPLEX_FLAGS) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ScriptItemize(pwcinchars: super::Foundation::PWSTR, cinchars: i32, cmaxitems: i32, pscontrol: *const SCRIPT_CONTROL, psstate: *const SCRIPT_STATE, pitems: *mut SCRIPT_ITEM, pcitems: *mut i32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ScriptItemizeOpenType(pwcinchars: super::Foundation::PWSTR, cinchars: i32, cmaxitems: i32, pscontrol: *const SCRIPT_CONTROL, psstate: *const SCRIPT_STATE, pitems: *mut SCRIPT_ITEM, pscripttags: *mut u32, pcitems: *mut i32) -> ::windows_sys::core::HRESULT;
    pub fn ScriptJustify(psva: *const SCRIPT_VISATTR, piadvance: *const i32, cglyphs: i32, idx: i32, iminkashida: i32, pijustify: *mut i32) -> ::windows_sys::core::HRESULT;
    pub fn ScriptLayout(cruns: i32, pblevel: *const u8, pivisualtological: *mut i32, pilogicaltovisual: *mut i32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn ScriptPlace(hdc: super::Graphics::Gdi::HDC, psc: *mut *mut ::core::ffi::c_void, pwglyphs: *const u16, cglyphs: i32, psva: *const SCRIPT_VISATTR, psa: *mut SCRIPT_ANALYSIS, piadvance: *mut i32, pgoffset: *mut GOFFSET, pabc: *mut super::Graphics::Gdi::ABC) -> ::windows_sys::core::HRESULT;
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
    ) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn ScriptPositionSingleGlyph(hdc: super::Graphics::Gdi::HDC, psc: *mut *mut ::core::ffi::c_void, psa: *const SCRIPT_ANALYSIS, tagscript: u32, taglangsys: u32, tagfeature: u32, lparameter: i32, wglyphid: u16, iadvance: i32, goffset: GOFFSET, pioutadvance: *mut i32, poutgoffset: *mut GOFFSET) -> ::windows_sys::core::HRESULT;
    pub fn ScriptRecordDigitSubstitution(locale: u32, psds: *mut SCRIPT_DIGITSUBSTITUTE) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn ScriptShape(hdc: super::Graphics::Gdi::HDC, psc: *mut *mut ::core::ffi::c_void, pwcchars: super::Foundation::PWSTR, cchars: i32, cmaxglyphs: i32, psa: *mut SCRIPT_ANALYSIS, pwoutglyphs: *mut u16, pwlogclust: *mut u16, psva: *mut SCRIPT_VISATTR, pcglyphs: *mut i32) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn ScriptShapeOpenType(hdc: super::Graphics::Gdi::HDC, psc: *mut *mut ::core::ffi::c_void, psa: *mut SCRIPT_ANALYSIS, tagscript: u32, taglangsys: u32, rcrangechars: *const i32, rprangeproperties: *const *const textrange_properties, cranges: i32, pwcchars: super::Foundation::PWSTR, cchars: i32, cmaxglyphs: i32, pwlogclust: *mut u16, pcharprops: *mut script_charprop, pwoutglyphs: *mut u16, poutglyphprops: *mut script_glyphprop, pcglyphs: *mut i32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn ScriptStringAnalyse(hdc: super::Graphics::Gdi::HDC, pstring: *const ::core::ffi::c_void, cstring: i32, cglyphs: i32, icharset: i32, dwflags: u32, ireqwidth: i32, pscontrol: *const SCRIPT_CONTROL, psstate: *const SCRIPT_STATE, pidx: *const i32, ptabdef: *const SCRIPT_TABDEF, pbinclass: *const u8, pssa: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ScriptStringCPtoX(ssa: *const ::core::ffi::c_void, icp: i32, ftrailing: super::Foundation::BOOL, px: *mut i32) -> ::windows_sys::core::HRESULT;
    pub fn ScriptStringFree(pssa: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    pub fn ScriptStringGetLogicalWidths(ssa: *const ::core::ffi::c_void, pidx: *mut i32) -> ::windows_sys::core::HRESULT;
    pub fn ScriptStringGetOrder(ssa: *const ::core::ffi::c_void, puorder: *mut u32) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn ScriptStringOut(ssa: *const ::core::ffi::c_void, ix: i32, iy: i32, uoptions: super::Graphics::Gdi::ETO_OPTIONS, prc: *const super::Foundation::RECT, iminsel: i32, imaxsel: i32, fdisabled: super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    pub fn ScriptStringValidate(ssa: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    pub fn ScriptStringXtoCP(ssa: *const ::core::ffi::c_void, ix: i32, pich: *mut i32, pitrailing: *mut i32) -> ::windows_sys::core::HRESULT;
    pub fn ScriptString_pLogAttr(ssa: *const ::core::ffi::c_void) -> *mut SCRIPT_LOGATTR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ScriptString_pSize(ssa: *const ::core::ffi::c_void) -> *mut super::Foundation::SIZE;
    pub fn ScriptString_pcOutChars(ssa: *const ::core::ffi::c_void) -> *mut i32;
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn ScriptSubstituteSingleGlyph(hdc: super::Graphics::Gdi::HDC, psc: *mut *mut ::core::ffi::c_void, psa: *const SCRIPT_ANALYSIS, tagscript: u32, taglangsys: u32, tagfeature: u32, lparameter: i32, wglyphid: u16, pwoutglyphid: *mut u16) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn ScriptTextOut(hdc: super::Graphics::Gdi::HDC, psc: *mut *mut ::core::ffi::c_void, x: i32, y: i32, fuoptions: u32, lprc: *const super::Foundation::RECT, psa: *const SCRIPT_ANALYSIS, pwcreserved: super::Foundation::PWSTR, ireserved: i32, pwglyphs: *const u16, cglyphs: i32, piadvance: *const i32, pijustify: *const i32, pgoffset: *const GOFFSET) -> ::windows_sys::core::HRESULT;
    pub fn ScriptXtoCP(ix: i32, cchars: i32, cglyphs: i32, pwlogclust: *const u16, psva: *const SCRIPT_VISATTR, piadvance: *const i32, psa: *const SCRIPT_ANALYSIS, picp: *mut i32, pitrailing: *mut i32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetCalendarInfoA(locale: u32, calendar: u32, caltype: u32, lpcaldata: super::Foundation::PSTR) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetCalendarInfoW(locale: u32, calendar: u32, caltype: u32, lpcaldata: super::Foundation::PWSTR) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetLocaleInfoA(locale: u32, lctype: u32, lplcdata: super::Foundation::PSTR) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetLocaleInfoW(locale: u32, lctype: u32, lplcdata: super::Foundation::PWSTR) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetProcessPreferredUILanguages(dwflags: u32, pwszlanguagesbuffer: super::Foundation::PWSTR, pulnumlanguages: *mut u32) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetThreadLocale(locale: u32) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetThreadPreferredUILanguages(dwflags: u32, pwszlanguagesbuffer: super::Foundation::PWSTR, pulnumlanguages: *mut u32) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetThreadPreferredUILanguages2(flags: u32, languages: super::Foundation::PWSTR, numlanguagesset: *mut u32, snapshot: *mut HSAVEDUILANGUAGES) -> super::Foundation::BOOL;
    pub fn SetThreadUILanguage(langid: u16) -> u16;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetUserGeoID(geoid: i32) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetUserGeoName(geoname: super::Foundation::PWSTR) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn TranslateCharsetInfo(lpsrc: *mut u32, lpcs: *mut CHARSETINFO, dwflags: TRANSLATE_CHARSET_INFO_FLAGS) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn UCNV_FROM_U_CALLBACK_ESCAPE(context: *const ::core::ffi::c_void, fromuargs: *mut UConverterFromUnicodeArgs, codeunits: *const u16, length: i32, codepoint: i32, reason: UConverterCallbackReason, err: *mut UErrorCode);
    #[cfg(feature = "Win32_Foundation")]
    pub fn UCNV_FROM_U_CALLBACK_SKIP(context: *const ::core::ffi::c_void, fromuargs: *mut UConverterFromUnicodeArgs, codeunits: *const u16, length: i32, codepoint: i32, reason: UConverterCallbackReason, err: *mut UErrorCode);
    #[cfg(feature = "Win32_Foundation")]
    pub fn UCNV_FROM_U_CALLBACK_STOP(context: *const ::core::ffi::c_void, fromuargs: *mut UConverterFromUnicodeArgs, codeunits: *const u16, length: i32, codepoint: i32, reason: UConverterCallbackReason, err: *mut UErrorCode);
    #[cfg(feature = "Win32_Foundation")]
    pub fn UCNV_FROM_U_CALLBACK_SUBSTITUTE(context: *const ::core::ffi::c_void, fromuargs: *mut UConverterFromUnicodeArgs, codeunits: *const u16, length: i32, codepoint: i32, reason: UConverterCallbackReason, err: *mut UErrorCode);
    #[cfg(feature = "Win32_Foundation")]
    pub fn UCNV_TO_U_CALLBACK_ESCAPE(context: *const ::core::ffi::c_void, touargs: *mut UConverterToUnicodeArgs, codeunits: super::Foundation::PSTR, length: i32, reason: UConverterCallbackReason, err: *mut UErrorCode);
    #[cfg(feature = "Win32_Foundation")]
    pub fn UCNV_TO_U_CALLBACK_SKIP(context: *const ::core::ffi::c_void, touargs: *mut UConverterToUnicodeArgs, codeunits: super::Foundation::PSTR, length: i32, reason: UConverterCallbackReason, err: *mut UErrorCode);
    #[cfg(feature = "Win32_Foundation")]
    pub fn UCNV_TO_U_CALLBACK_STOP(context: *const ::core::ffi::c_void, touargs: *mut UConverterToUnicodeArgs, codeunits: super::Foundation::PSTR, length: i32, reason: UConverterCallbackReason, err: *mut UErrorCode);
    #[cfg(feature = "Win32_Foundation")]
    pub fn UCNV_TO_U_CALLBACK_SUBSTITUTE(context: *const ::core::ffi::c_void, touargs: *mut UConverterToUnicodeArgs, codeunits: super::Foundation::PSTR, length: i32, reason: UConverterCallbackReason, err: *mut UErrorCode);
    #[cfg(feature = "Win32_Foundation")]
    pub fn VerifyScripts(dwflags: u32, lplocalescripts: super::Foundation::PWSTR, cchlocalescripts: i32, lptestscripts: super::Foundation::PWSTR, cchtestscripts: i32) -> super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WideCharToMultiByte(codepage: u32, dwflags: u32, lpwidecharstr: super::Foundation::PWSTR, cchwidechar: i32, lpmultibytestr: super::Foundation::PSTR, cbmultibyte: i32, lpdefaultchar: super::Foundation::PSTR, lpuseddefaultchar: *mut i32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn lstrcatA(lpstring1: super::Foundation::PSTR, lpstring2: super::Foundation::PSTR) -> super::Foundation::PSTR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn lstrcatW(lpstring1: super::Foundation::PWSTR, lpstring2: super::Foundation::PWSTR) -> super::Foundation::PWSTR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn lstrcmpA(lpstring1: super::Foundation::PSTR, lpstring2: super::Foundation::PSTR) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn lstrcmpW(lpstring1: super::Foundation::PWSTR, lpstring2: super::Foundation::PWSTR) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn lstrcmpiA(lpstring1: super::Foundation::PSTR, lpstring2: super::Foundation::PSTR) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn lstrcmpiW(lpstring1: super::Foundation::PWSTR, lpstring2: super::Foundation::PWSTR) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn lstrcpyA(lpstring1: super::Foundation::PSTR, lpstring2: super::Foundation::PSTR) -> super::Foundation::PSTR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn lstrcpyW(lpstring1: super::Foundation::PWSTR, lpstring2: super::Foundation::PWSTR) -> super::Foundation::PWSTR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn lstrcpynA(lpstring1: super::Foundation::PSTR, lpstring2: super::Foundation::PSTR, imaxlength: i32) -> super::Foundation::PSTR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn lstrcpynW(lpstring1: super::Foundation::PWSTR, lpstring2: super::Foundation::PWSTR, imaxlength: i32) -> super::Foundation::PWSTR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn lstrlenA(lpstring: super::Foundation::PSTR) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn lstrlenW(lpstring: super::Foundation::PWSTR) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_UCharsToChars(us: *const u16, cs: super::Foundation::PSTR, length: i32);
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_austrcpy(dst: super::Foundation::PSTR, src: *const u16) -> super::Foundation::PSTR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_austrncpy(dst: super::Foundation::PSTR, src: *const u16, n: i32) -> super::Foundation::PSTR;
    pub fn u_catclose(catd: *mut UResourceBundle);
    pub fn u_catgets(catd: *mut UResourceBundle, set_num: i32, msg_num: i32, s: *const u16, len: *mut i32, ec: *mut UErrorCode) -> *mut u16;
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_catopen(name: super::Foundation::PSTR, locale: super::Foundation::PSTR, ec: *mut UErrorCode) -> *mut UResourceBundle;
    pub fn u_charAge(c: i32, versionarray: *mut u8);
    pub fn u_charDigitValue(c: i32) -> i32;
    pub fn u_charDirection(c: i32) -> UCharDirection;
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_charFromName(namechoice: UCharNameChoice, name: super::Foundation::PSTR, perrorcode: *mut UErrorCode) -> i32;
    pub fn u_charMirror(c: i32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_charName(code: i32, namechoice: UCharNameChoice, buffer: super::Foundation::PSTR, bufferlength: i32, perrorcode: *mut UErrorCode) -> i32;
    pub fn u_charType(c: i32) -> i8;
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_charsToUChars(cs: super::Foundation::PSTR, us: *mut u16, length: i32);
    pub fn u_cleanup();
    pub fn u_countChar32(s: *const u16, length: i32) -> i32;
    pub fn u_digit(ch: i32, radix: i8) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_enumCharNames(start: i32, limit: i32, r#fn: *mut UEnumCharNamesFn, context: *mut ::core::ffi::c_void, namechoice: UCharNameChoice, perrorcode: *mut UErrorCode);
    pub fn u_enumCharTypes(enumrange: *mut UCharEnumTypeRange, context: *const ::core::ffi::c_void);
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_errorName(code: UErrorCode) -> super::Foundation::PSTR;
    pub fn u_foldCase(c: i32, options: u32) -> i32;
    pub fn u_forDigit(digit: i32, radix: i8) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_formatMessage(locale: super::Foundation::PSTR, pattern: *const u16, patternlength: i32, result: *mut u16, resultlength: i32, status: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_formatMessageWithError(locale: super::Foundation::PSTR, pattern: *const u16, patternlength: i32, result: *mut u16, resultlength: i32, parseerror: *mut UParseError, status: *mut UErrorCode) -> i32;
    pub fn u_getBidiPairedBracket(c: i32) -> i32;
    pub fn u_getBinaryPropertySet(property: UProperty, perrorcode: *mut UErrorCode) -> *mut USet;
    pub fn u_getCombiningClass(c: i32) -> u8;
    pub fn u_getDataVersion(dataversionfillin: *mut u8, status: *mut UErrorCode);
    pub fn u_getFC_NFKC_Closure(c: i32, dest: *mut u16, destcapacity: i32, perrorcode: *mut UErrorCode) -> i32;
    pub fn u_getIntPropertyMap(property: UProperty, perrorcode: *mut UErrorCode) -> *mut UCPMap;
    pub fn u_getIntPropertyMaxValue(which: UProperty) -> i32;
    pub fn u_getIntPropertyMinValue(which: UProperty) -> i32;
    pub fn u_getIntPropertyValue(c: i32, which: UProperty) -> i32;
    pub fn u_getNumericValue(c: i32) -> f64;
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_getPropertyEnum(alias: super::Foundation::PSTR) -> UProperty;
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_getPropertyName(property: UProperty, namechoice: UPropertyNameChoice) -> super::Foundation::PSTR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_getPropertyValueEnum(property: UProperty, alias: super::Foundation::PSTR) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_getPropertyValueName(property: UProperty, value: i32, namechoice: UPropertyNameChoice) -> super::Foundation::PSTR;
    pub fn u_getUnicodeVersion(versionarray: *mut u8);
    pub fn u_getVersion(versionarray: *mut u8);
    pub fn u_hasBinaryProperty(c: i32, which: UProperty) -> i8;
    pub fn u_init(status: *mut UErrorCode);
    pub fn u_isIDIgnorable(c: i32) -> i8;
    pub fn u_isIDPart(c: i32) -> i8;
    pub fn u_isIDStart(c: i32) -> i8;
    pub fn u_isISOControl(c: i32) -> i8;
    pub fn u_isJavaIDPart(c: i32) -> i8;
    pub fn u_isJavaIDStart(c: i32) -> i8;
    pub fn u_isJavaSpaceChar(c: i32) -> i8;
    pub fn u_isMirrored(c: i32) -> i8;
    pub fn u_isUAlphabetic(c: i32) -> i8;
    pub fn u_isULowercase(c: i32) -> i8;
    pub fn u_isUUppercase(c: i32) -> i8;
    pub fn u_isUWhiteSpace(c: i32) -> i8;
    pub fn u_isWhitespace(c: i32) -> i8;
    pub fn u_isalnum(c: i32) -> i8;
    pub fn u_isalpha(c: i32) -> i8;
    pub fn u_isbase(c: i32) -> i8;
    pub fn u_isblank(c: i32) -> i8;
    pub fn u_iscntrl(c: i32) -> i8;
    pub fn u_isdefined(c: i32) -> i8;
    pub fn u_isdigit(c: i32) -> i8;
    pub fn u_isgraph(c: i32) -> i8;
    pub fn u_islower(c: i32) -> i8;
    pub fn u_isprint(c: i32) -> i8;
    pub fn u_ispunct(c: i32) -> i8;
    pub fn u_isspace(c: i32) -> i8;
    pub fn u_istitle(c: i32) -> i8;
    pub fn u_isupper(c: i32) -> i8;
    pub fn u_isxdigit(c: i32) -> i8;
    pub fn u_memcasecmp(s1: *const u16, s2: *const u16, length: i32, options: u32) -> i32;
    pub fn u_memchr(s: *const u16, c: u16, count: i32) -> *mut u16;
    pub fn u_memchr32(s: *const u16, c: i32, count: i32) -> *mut u16;
    pub fn u_memcmp(buf1: *const u16, buf2: *const u16, count: i32) -> i32;
    pub fn u_memcmpCodePointOrder(s1: *const u16, s2: *const u16, count: i32) -> i32;
    pub fn u_memcpy(dest: *mut u16, src: *const u16, count: i32) -> *mut u16;
    pub fn u_memmove(dest: *mut u16, src: *const u16, count: i32) -> *mut u16;
    pub fn u_memrchr(s: *const u16, c: u16, count: i32) -> *mut u16;
    pub fn u_memrchr32(s: *const u16, c: i32, count: i32) -> *mut u16;
    pub fn u_memset(dest: *mut u16, c: u16, count: i32) -> *mut u16;
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_parseMessage(locale: super::Foundation::PSTR, pattern: *const u16, patternlength: i32, source: *const u16, sourcelength: i32, status: *mut UErrorCode);
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_parseMessageWithError(locale: super::Foundation::PSTR, pattern: *const u16, patternlength: i32, source: *const u16, sourcelength: i32, parseerror: *mut UParseError, status: *mut UErrorCode);
    pub fn u_setMemoryFunctions(context: *const ::core::ffi::c_void, a: *mut UMemAllocFn, r: *mut UMemReallocFn, f: *mut UMemFreeFn, status: *mut UErrorCode);
    pub fn u_shapeArabic(source: *const u16, sourcelength: i32, dest: *mut u16, destsize: i32, options: u32, perrorcode: *mut UErrorCode) -> i32;
    pub fn u_strCaseCompare(s1: *const u16, length1: i32, s2: *const u16, length2: i32, options: u32, perrorcode: *mut UErrorCode) -> i32;
    pub fn u_strCompare(s1: *const u16, length1: i32, s2: *const u16, length2: i32, codepointorder: i8) -> i32;
    pub fn u_strCompareIter(iter1: *mut UCharIterator, iter2: *mut UCharIterator, codepointorder: i8) -> i32;
    pub fn u_strFindFirst(s: *const u16, length: i32, substring: *const u16, sublength: i32) -> *mut u16;
    pub fn u_strFindLast(s: *const u16, length: i32, substring: *const u16, sublength: i32) -> *mut u16;
    pub fn u_strFoldCase(dest: *mut u16, destcapacity: i32, src: *const u16, srclength: i32, options: u32, perrorcode: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_strFromJavaModifiedUTF8WithSub(dest: *mut u16, destcapacity: i32, pdestlength: *mut i32, src: super::Foundation::PSTR, srclength: i32, subchar: i32, pnumsubstitutions: *mut i32, perrorcode: *mut UErrorCode) -> *mut u16;
    pub fn u_strFromUTF32(dest: *mut u16, destcapacity: i32, pdestlength: *mut i32, src: *const i32, srclength: i32, perrorcode: *mut UErrorCode) -> *mut u16;
    pub fn u_strFromUTF32WithSub(dest: *mut u16, destcapacity: i32, pdestlength: *mut i32, src: *const i32, srclength: i32, subchar: i32, pnumsubstitutions: *mut i32, perrorcode: *mut UErrorCode) -> *mut u16;
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_strFromUTF8(dest: *mut u16, destcapacity: i32, pdestlength: *mut i32, src: super::Foundation::PSTR, srclength: i32, perrorcode: *mut UErrorCode) -> *mut u16;
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_strFromUTF8Lenient(dest: *mut u16, destcapacity: i32, pdestlength: *mut i32, src: super::Foundation::PSTR, srclength: i32, perrorcode: *mut UErrorCode) -> *mut u16;
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_strFromUTF8WithSub(dest: *mut u16, destcapacity: i32, pdestlength: *mut i32, src: super::Foundation::PSTR, srclength: i32, subchar: i32, pnumsubstitutions: *mut i32, perrorcode: *mut UErrorCode) -> *mut u16;
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_strFromWCS(dest: *mut u16, destcapacity: i32, pdestlength: *mut i32, src: super::Foundation::PWSTR, srclength: i32, perrorcode: *mut UErrorCode) -> *mut u16;
    pub fn u_strHasMoreChar32Than(s: *const u16, length: i32, number: i32) -> i8;
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_strToJavaModifiedUTF8(dest: super::Foundation::PSTR, destcapacity: i32, pdestlength: *mut i32, src: *const u16, srclength: i32, perrorcode: *mut UErrorCode) -> super::Foundation::PSTR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_strToLower(dest: *mut u16, destcapacity: i32, src: *const u16, srclength: i32, locale: super::Foundation::PSTR, perrorcode: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_strToTitle(dest: *mut u16, destcapacity: i32, src: *const u16, srclength: i32, titleiter: *mut UBreakIterator, locale: super::Foundation::PSTR, perrorcode: *mut UErrorCode) -> i32;
    pub fn u_strToUTF32(dest: *mut i32, destcapacity: i32, pdestlength: *mut i32, src: *const u16, srclength: i32, perrorcode: *mut UErrorCode) -> *mut i32;
    pub fn u_strToUTF32WithSub(dest: *mut i32, destcapacity: i32, pdestlength: *mut i32, src: *const u16, srclength: i32, subchar: i32, pnumsubstitutions: *mut i32, perrorcode: *mut UErrorCode) -> *mut i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_strToUTF8(dest: super::Foundation::PSTR, destcapacity: i32, pdestlength: *mut i32, src: *const u16, srclength: i32, perrorcode: *mut UErrorCode) -> super::Foundation::PSTR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_strToUTF8WithSub(dest: super::Foundation::PSTR, destcapacity: i32, pdestlength: *mut i32, src: *const u16, srclength: i32, subchar: i32, pnumsubstitutions: *mut i32, perrorcode: *mut UErrorCode) -> super::Foundation::PSTR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_strToUpper(dest: *mut u16, destcapacity: i32, src: *const u16, srclength: i32, locale: super::Foundation::PSTR, perrorcode: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_strToWCS(dest: super::Foundation::PWSTR, destcapacity: i32, pdestlength: *mut i32, src: *const u16, srclength: i32, perrorcode: *mut UErrorCode) -> super::Foundation::PWSTR;
    pub fn u_strcasecmp(s1: *const u16, s2: *const u16, options: u32) -> i32;
    pub fn u_strcat(dst: *mut u16, src: *const u16) -> *mut u16;
    pub fn u_strchr(s: *const u16, c: u16) -> *mut u16;
    pub fn u_strchr32(s: *const u16, c: i32) -> *mut u16;
    pub fn u_strcmp(s1: *const u16, s2: *const u16) -> i32;
    pub fn u_strcmpCodePointOrder(s1: *const u16, s2: *const u16) -> i32;
    pub fn u_strcpy(dst: *mut u16, src: *const u16) -> *mut u16;
    pub fn u_strcspn(string: *const u16, matchset: *const u16) -> i32;
    pub fn u_strlen(s: *const u16) -> i32;
    pub fn u_strncasecmp(s1: *const u16, s2: *const u16, n: i32, options: u32) -> i32;
    pub fn u_strncat(dst: *mut u16, src: *const u16, n: i32) -> *mut u16;
    pub fn u_strncmp(ucs1: *const u16, ucs2: *const u16, n: i32) -> i32;
    pub fn u_strncmpCodePointOrder(s1: *const u16, s2: *const u16, n: i32) -> i32;
    pub fn u_strncpy(dst: *mut u16, src: *const u16, n: i32) -> *mut u16;
    pub fn u_strpbrk(string: *const u16, matchset: *const u16) -> *mut u16;
    pub fn u_strrchr(s: *const u16, c: u16) -> *mut u16;
    pub fn u_strrchr32(s: *const u16, c: i32) -> *mut u16;
    pub fn u_strrstr(s: *const u16, substring: *const u16) -> *mut u16;
    pub fn u_strspn(string: *const u16, matchset: *const u16) -> i32;
    pub fn u_strstr(s: *const u16, substring: *const u16) -> *mut u16;
    pub fn u_strtok_r(src: *mut u16, delim: *const u16, savestate: *mut *mut u16) -> *mut u16;
    pub fn u_tolower(c: i32) -> i32;
    pub fn u_totitle(c: i32) -> i32;
    pub fn u_toupper(c: i32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_uastrcpy(dst: *mut u16, src: super::Foundation::PSTR) -> *mut u16;
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_uastrncpy(dst: *mut u16, src: super::Foundation::PSTR, n: i32) -> *mut u16;
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_unescape(src: super::Foundation::PSTR, dest: *mut u16, destcapacity: i32) -> i32;
    pub fn u_unescapeAt(charat: UNESCAPE_CHAR_AT, offset: *mut i32, length: i32, context: *mut ::core::ffi::c_void) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_versionFromString(versionarray: *mut u8, versionstring: super::Foundation::PSTR);
    pub fn u_versionFromUString(versionarray: *mut u8, versionstring: *const u16);
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_versionToString(versionarray: *const u8, versionstring: super::Foundation::PSTR);
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_vformatMessage(locale: super::Foundation::PSTR, pattern: *const u16, patternlength: i32, result: *mut u16, resultlength: i32, ap: *mut i8, status: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_vformatMessageWithError(locale: super::Foundation::PSTR, pattern: *const u16, patternlength: i32, result: *mut u16, resultlength: i32, parseerror: *mut UParseError, ap: *mut i8, status: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_vparseMessage(locale: super::Foundation::PSTR, pattern: *const u16, patternlength: i32, source: *const u16, sourcelength: i32, ap: *mut i8, status: *mut UErrorCode);
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_vparseMessageWithError(locale: super::Foundation::PSTR, pattern: *const u16, patternlength: i32, source: *const u16, sourcelength: i32, ap: *mut i8, parseerror: *mut UParseError, status: *mut UErrorCode);
    pub fn ubidi_close(pbidi: *mut UBiDi);
    pub fn ubidi_countParagraphs(pbidi: *mut UBiDi) -> i32;
    pub fn ubidi_countRuns(pbidi: *mut UBiDi, perrorcode: *mut UErrorCode) -> i32;
    pub fn ubidi_getBaseDirection(text: *const u16, length: i32) -> UBiDiDirection;
    pub fn ubidi_getClassCallback(pbidi: *mut UBiDi, r#fn: *mut UBiDiClassCallback, context: *const *const ::core::ffi::c_void);
    pub fn ubidi_getCustomizedClass(pbidi: *mut UBiDi, c: i32) -> UCharDirection;
    pub fn ubidi_getDirection(pbidi: *const UBiDi) -> UBiDiDirection;
    pub fn ubidi_getLength(pbidi: *const UBiDi) -> i32;
    pub fn ubidi_getLevelAt(pbidi: *const UBiDi, charindex: i32) -> u8;
    pub fn ubidi_getLevels(pbidi: *mut UBiDi, perrorcode: *mut UErrorCode) -> *mut u8;
    pub fn ubidi_getLogicalIndex(pbidi: *mut UBiDi, visualindex: i32, perrorcode: *mut UErrorCode) -> i32;
    pub fn ubidi_getLogicalMap(pbidi: *mut UBiDi, indexmap: *mut i32, perrorcode: *mut UErrorCode);
    pub fn ubidi_getLogicalRun(pbidi: *const UBiDi, logicalposition: i32, plogicallimit: *mut i32, plevel: *mut u8);
    pub fn ubidi_getParaLevel(pbidi: *const UBiDi) -> u8;
    pub fn ubidi_getParagraph(pbidi: *const UBiDi, charindex: i32, pparastart: *mut i32, pparalimit: *mut i32, pparalevel: *mut u8, perrorcode: *mut UErrorCode) -> i32;
    pub fn ubidi_getParagraphByIndex(pbidi: *const UBiDi, paraindex: i32, pparastart: *mut i32, pparalimit: *mut i32, pparalevel: *mut u8, perrorcode: *mut UErrorCode);
    pub fn ubidi_getProcessedLength(pbidi: *const UBiDi) -> i32;
    pub fn ubidi_getReorderingMode(pbidi: *mut UBiDi) -> UBiDiReorderingMode;
    pub fn ubidi_getReorderingOptions(pbidi: *mut UBiDi) -> u32;
    pub fn ubidi_getResultLength(pbidi: *const UBiDi) -> i32;
    pub fn ubidi_getText(pbidi: *const UBiDi) -> *mut u16;
    pub fn ubidi_getVisualIndex(pbidi: *mut UBiDi, logicalindex: i32, perrorcode: *mut UErrorCode) -> i32;
    pub fn ubidi_getVisualMap(pbidi: *mut UBiDi, indexmap: *mut i32, perrorcode: *mut UErrorCode);
    pub fn ubidi_getVisualRun(pbidi: *mut UBiDi, runindex: i32, plogicalstart: *mut i32, plength: *mut i32) -> UBiDiDirection;
    pub fn ubidi_invertMap(srcmap: *const i32, destmap: *mut i32, length: i32);
    pub fn ubidi_isInverse(pbidi: *mut UBiDi) -> i8;
    pub fn ubidi_isOrderParagraphsLTR(pbidi: *mut UBiDi) -> i8;
    pub fn ubidi_open() -> *mut UBiDi;
    pub fn ubidi_openSized(maxlength: i32, maxruncount: i32, perrorcode: *mut UErrorCode) -> *mut UBiDi;
    pub fn ubidi_orderParagraphsLTR(pbidi: *mut UBiDi, orderparagraphsltr: i8);
    pub fn ubidi_reorderLogical(levels: *const u8, length: i32, indexmap: *mut i32);
    pub fn ubidi_reorderVisual(levels: *const u8, length: i32, indexmap: *mut i32);
    pub fn ubidi_setClassCallback(pbidi: *mut UBiDi, newfn: UBiDiClassCallback, newcontext: *const ::core::ffi::c_void, oldfn: *mut UBiDiClassCallback, oldcontext: *const *const ::core::ffi::c_void, perrorcode: *mut UErrorCode);
    pub fn ubidi_setContext(pbidi: *mut UBiDi, prologue: *const u16, prolength: i32, epilogue: *const u16, epilength: i32, perrorcode: *mut UErrorCode);
    pub fn ubidi_setInverse(pbidi: *mut UBiDi, isinverse: i8);
    pub fn ubidi_setLine(pparabidi: *const UBiDi, start: i32, limit: i32, plinebidi: *mut UBiDi, perrorcode: *mut UErrorCode);
    pub fn ubidi_setPara(pbidi: *mut UBiDi, text: *const u16, length: i32, paralevel: u8, embeddinglevels: *mut u8, perrorcode: *mut UErrorCode);
    pub fn ubidi_setReorderingMode(pbidi: *mut UBiDi, reorderingmode: UBiDiReorderingMode);
    pub fn ubidi_setReorderingOptions(pbidi: *mut UBiDi, reorderingoptions: u32);
    pub fn ubidi_writeReordered(pbidi: *mut UBiDi, dest: *mut u16, destsize: i32, options: u16, perrorcode: *mut UErrorCode) -> i32;
    pub fn ubidi_writeReverse(src: *const u16, srclength: i32, dest: *mut u16, destsize: i32, options: u16, perrorcode: *mut UErrorCode) -> i32;
    pub fn ubiditransform_close(pbiditransform: *mut UBiDiTransform);
    pub fn ubiditransform_open(perrorcode: *mut UErrorCode) -> *mut UBiDiTransform;
    pub fn ubiditransform_transform(pbiditransform: *mut UBiDiTransform, src: *const u16, srclength: i32, dest: *mut u16, destsize: i32, inparalevel: u8, inorder: UBiDiOrder, outparalevel: u8, outorder: UBiDiOrder, domirroring: UBiDiMirroring, shapingoptions: u32, perrorcode: *mut UErrorCode) -> u32;
    pub fn ublock_getCode(c: i32) -> UBlockCode;
    pub fn ubrk_close(bi: *mut UBreakIterator);
    pub fn ubrk_countAvailable() -> i32;
    pub fn ubrk_current(bi: *const UBreakIterator) -> i32;
    pub fn ubrk_first(bi: *mut UBreakIterator) -> i32;
    pub fn ubrk_following(bi: *mut UBreakIterator, offset: i32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ubrk_getAvailable(index: i32) -> super::Foundation::PSTR;
    pub fn ubrk_getBinaryRules(bi: *mut UBreakIterator, binaryrules: *mut u8, rulescapacity: i32, status: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ubrk_getLocaleByType(bi: *const UBreakIterator, r#type: ULocDataLocaleType, status: *mut UErrorCode) -> super::Foundation::PSTR;
    pub fn ubrk_getRuleStatus(bi: *mut UBreakIterator) -> i32;
    pub fn ubrk_getRuleStatusVec(bi: *mut UBreakIterator, fillinvec: *mut i32, capacity: i32, status: *mut UErrorCode) -> i32;
    pub fn ubrk_isBoundary(bi: *mut UBreakIterator, offset: i32) -> i8;
    pub fn ubrk_last(bi: *mut UBreakIterator) -> i32;
    pub fn ubrk_next(bi: *mut UBreakIterator) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ubrk_open(r#type: UBreakIteratorType, locale: super::Foundation::PSTR, text: *const u16, textlength: i32, status: *mut UErrorCode) -> *mut UBreakIterator;
    pub fn ubrk_openBinaryRules(binaryrules: *const u8, ruleslength: i32, text: *const u16, textlength: i32, status: *mut UErrorCode) -> *mut UBreakIterator;
    pub fn ubrk_openRules(rules: *const u16, ruleslength: i32, text: *const u16, textlength: i32, parseerr: *mut UParseError, status: *mut UErrorCode) -> *mut UBreakIterator;
    pub fn ubrk_preceding(bi: *mut UBreakIterator, offset: i32) -> i32;
    pub fn ubrk_previous(bi: *mut UBreakIterator) -> i32;
    pub fn ubrk_refreshUText(bi: *mut UBreakIterator, text: *mut UText, status: *mut UErrorCode);
    pub fn ubrk_safeClone(bi: *const UBreakIterator, stackbuffer: *mut ::core::ffi::c_void, pbuffersize: *mut i32, status: *mut UErrorCode) -> *mut UBreakIterator;
    pub fn ubrk_setText(bi: *mut UBreakIterator, text: *const u16, textlength: i32, status: *mut UErrorCode);
    pub fn ubrk_setUText(bi: *mut UBreakIterator, text: *mut UText, status: *mut UErrorCode);
    pub fn ucal_add(cal: *mut *mut ::core::ffi::c_void, field: UCalendarDateFields, amount: i32, status: *mut UErrorCode);
    pub fn ucal_clear(calendar: *mut *mut ::core::ffi::c_void);
    pub fn ucal_clearField(cal: *mut *mut ::core::ffi::c_void, field: UCalendarDateFields);
    pub fn ucal_clone(cal: *const *const ::core::ffi::c_void, status: *mut UErrorCode) -> *mut *mut ::core::ffi::c_void;
    pub fn ucal_close(cal: *mut *mut ::core::ffi::c_void);
    pub fn ucal_countAvailable() -> i32;
    pub fn ucal_equivalentTo(cal1: *const *const ::core::ffi::c_void, cal2: *const *const ::core::ffi::c_void) -> i8;
    pub fn ucal_get(cal: *const *const ::core::ffi::c_void, field: UCalendarDateFields, status: *mut UErrorCode) -> i32;
    pub fn ucal_getAttribute(cal: *const *const ::core::ffi::c_void, attr: UCalendarAttribute) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucal_getAvailable(localeindex: i32) -> super::Foundation::PSTR;
    pub fn ucal_getCanonicalTimeZoneID(id: *const u16, len: i32, result: *mut u16, resultcapacity: i32, issystemid: *mut i8, status: *mut UErrorCode) -> i32;
    pub fn ucal_getDSTSavings(zoneid: *const u16, ec: *mut UErrorCode) -> i32;
    pub fn ucal_getDayOfWeekType(cal: *const *const ::core::ffi::c_void, dayofweek: UCalendarDaysOfWeek, status: *mut UErrorCode) -> UCalendarWeekdayType;
    pub fn ucal_getDefaultTimeZone(result: *mut u16, resultcapacity: i32, ec: *mut UErrorCode) -> i32;
    pub fn ucal_getFieldDifference(cal: *mut *mut ::core::ffi::c_void, target: f64, field: UCalendarDateFields, status: *mut UErrorCode) -> i32;
    pub fn ucal_getGregorianChange(cal: *const *const ::core::ffi::c_void, perrorcode: *mut UErrorCode) -> f64;
    pub fn ucal_getHostTimeZone(result: *mut u16, resultcapacity: i32, ec: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucal_getKeywordValuesForLocale(key: super::Foundation::PSTR, locale: super::Foundation::PSTR, commonlyused: i8, status: *mut UErrorCode) -> *mut UEnumeration;
    pub fn ucal_getLimit(cal: *const *const ::core::ffi::c_void, field: UCalendarDateFields, r#type: UCalendarLimitType, status: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucal_getLocaleByType(cal: *const *const ::core::ffi::c_void, r#type: ULocDataLocaleType, status: *mut UErrorCode) -> super::Foundation::PSTR;
    pub fn ucal_getMillis(cal: *const *const ::core::ffi::c_void, status: *mut UErrorCode) -> f64;
    pub fn ucal_getNow() -> f64;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucal_getTZDataVersion(status: *mut UErrorCode) -> super::Foundation::PSTR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucal_getTimeZoneDisplayName(cal: *const *const ::core::ffi::c_void, r#type: UCalendarDisplayNameType, locale: super::Foundation::PSTR, result: *mut u16, resultlength: i32, status: *mut UErrorCode) -> i32;
    pub fn ucal_getTimeZoneID(cal: *const *const ::core::ffi::c_void, result: *mut u16, resultlength: i32, status: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucal_getTimeZoneIDForWindowsID(winid: *const u16, len: i32, region: super::Foundation::PSTR, id: *mut u16, idcapacity: i32, status: *mut UErrorCode) -> i32;
    pub fn ucal_getTimeZoneTransitionDate(cal: *const *const ::core::ffi::c_void, r#type: UTimeZoneTransitionType, transition: *mut f64, status: *mut UErrorCode) -> i8;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucal_getType(cal: *const *const ::core::ffi::c_void, status: *mut UErrorCode) -> super::Foundation::PSTR;
    pub fn ucal_getWeekendTransition(cal: *const *const ::core::ffi::c_void, dayofweek: UCalendarDaysOfWeek, status: *mut UErrorCode) -> i32;
    pub fn ucal_getWindowsTimeZoneID(id: *const u16, len: i32, winid: *mut u16, winidcapacity: i32, status: *mut UErrorCode) -> i32;
    pub fn ucal_inDaylightTime(cal: *const *const ::core::ffi::c_void, status: *mut UErrorCode) -> i8;
    pub fn ucal_isSet(cal: *const *const ::core::ffi::c_void, field: UCalendarDateFields) -> i8;
    pub fn ucal_isWeekend(cal: *const *const ::core::ffi::c_void, date: f64, status: *mut UErrorCode) -> i8;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucal_open(zoneid: *const u16, len: i32, locale: super::Foundation::PSTR, r#type: UCalendarType, status: *mut UErrorCode) -> *mut *mut ::core::ffi::c_void;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucal_openCountryTimeZones(country: super::Foundation::PSTR, ec: *mut UErrorCode) -> *mut UEnumeration;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucal_openTimeZoneIDEnumeration(zonetype: USystemTimeZoneType, region: super::Foundation::PSTR, rawoffset: *const i32, ec: *mut UErrorCode) -> *mut UEnumeration;
    pub fn ucal_openTimeZones(ec: *mut UErrorCode) -> *mut UEnumeration;
    pub fn ucal_roll(cal: *mut *mut ::core::ffi::c_void, field: UCalendarDateFields, amount: i32, status: *mut UErrorCode);
    pub fn ucal_set(cal: *mut *mut ::core::ffi::c_void, field: UCalendarDateFields, value: i32);
    pub fn ucal_setAttribute(cal: *mut *mut ::core::ffi::c_void, attr: UCalendarAttribute, newvalue: i32);
    pub fn ucal_setDate(cal: *mut *mut ::core::ffi::c_void, year: i32, month: i32, date: i32, status: *mut UErrorCode);
    pub fn ucal_setDateTime(cal: *mut *mut ::core::ffi::c_void, year: i32, month: i32, date: i32, hour: i32, minute: i32, second: i32, status: *mut UErrorCode);
    pub fn ucal_setDefaultTimeZone(zoneid: *const u16, ec: *mut UErrorCode);
    pub fn ucal_setGregorianChange(cal: *mut *mut ::core::ffi::c_void, date: f64, perrorcode: *mut UErrorCode);
    pub fn ucal_setMillis(cal: *mut *mut ::core::ffi::c_void, datetime: f64, status: *mut UErrorCode);
    pub fn ucal_setTimeZone(cal: *mut *mut ::core::ffi::c_void, zoneid: *const u16, len: i32, status: *mut UErrorCode);
    pub fn ucasemap_close(csm: *mut UCaseMap);
    pub fn ucasemap_getBreakIterator(csm: *const UCaseMap) -> *mut UBreakIterator;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucasemap_getLocale(csm: *const UCaseMap) -> super::Foundation::PSTR;
    pub fn ucasemap_getOptions(csm: *const UCaseMap) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucasemap_open(locale: super::Foundation::PSTR, options: u32, perrorcode: *mut UErrorCode) -> *mut UCaseMap;
    pub fn ucasemap_setBreakIterator(csm: *mut UCaseMap, itertoadopt: *mut UBreakIterator, perrorcode: *mut UErrorCode);
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucasemap_setLocale(csm: *mut UCaseMap, locale: super::Foundation::PSTR, perrorcode: *mut UErrorCode);
    pub fn ucasemap_setOptions(csm: *mut UCaseMap, options: u32, perrorcode: *mut UErrorCode);
    pub fn ucasemap_toTitle(csm: *mut UCaseMap, dest: *mut u16, destcapacity: i32, src: *const u16, srclength: i32, perrorcode: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucasemap_utf8FoldCase(csm: *const UCaseMap, dest: super::Foundation::PSTR, destcapacity: i32, src: super::Foundation::PSTR, srclength: i32, perrorcode: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucasemap_utf8ToLower(csm: *const UCaseMap, dest: super::Foundation::PSTR, destcapacity: i32, src: super::Foundation::PSTR, srclength: i32, perrorcode: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucasemap_utf8ToTitle(csm: *mut UCaseMap, dest: super::Foundation::PSTR, destcapacity: i32, src: super::Foundation::PSTR, srclength: i32, perrorcode: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucasemap_utf8ToUpper(csm: *const UCaseMap, dest: super::Foundation::PSTR, destcapacity: i32, src: super::Foundation::PSTR, srclength: i32, perrorcode: *mut UErrorCode) -> i32;
    pub fn ucfpos_close(ucfpos: *mut UConstrainedFieldPosition);
    pub fn ucfpos_constrainCategory(ucfpos: *mut UConstrainedFieldPosition, category: i32, ec: *mut UErrorCode);
    pub fn ucfpos_constrainField(ucfpos: *mut UConstrainedFieldPosition, category: i32, field: i32, ec: *mut UErrorCode);
    pub fn ucfpos_getCategory(ucfpos: *const UConstrainedFieldPosition, ec: *mut UErrorCode) -> i32;
    pub fn ucfpos_getField(ucfpos: *const UConstrainedFieldPosition, ec: *mut UErrorCode) -> i32;
    pub fn ucfpos_getIndexes(ucfpos: *const UConstrainedFieldPosition, pstart: *mut i32, plimit: *mut i32, ec: *mut UErrorCode);
    pub fn ucfpos_getInt64IterationContext(ucfpos: *const UConstrainedFieldPosition, ec: *mut UErrorCode) -> i64;
    pub fn ucfpos_matchesField(ucfpos: *const UConstrainedFieldPosition, category: i32, field: i32, ec: *mut UErrorCode) -> i8;
    pub fn ucfpos_open(ec: *mut UErrorCode) -> *mut UConstrainedFieldPosition;
    pub fn ucfpos_reset(ucfpos: *mut UConstrainedFieldPosition, ec: *mut UErrorCode);
    pub fn ucfpos_setInt64IterationContext(ucfpos: *mut UConstrainedFieldPosition, context: i64, ec: *mut UErrorCode);
    pub fn ucfpos_setState(ucfpos: *mut UConstrainedFieldPosition, category: i32, field: i32, start: i32, limit: i32, ec: *mut UErrorCode);
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_cbFromUWriteBytes(args: *mut UConverterFromUnicodeArgs, source: super::Foundation::PSTR, length: i32, offsetindex: i32, err: *mut UErrorCode);
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_cbFromUWriteSub(args: *mut UConverterFromUnicodeArgs, offsetindex: i32, err: *mut UErrorCode);
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_cbFromUWriteUChars(args: *mut UConverterFromUnicodeArgs, source: *const *const u16, sourcelimit: *const u16, offsetindex: i32, err: *mut UErrorCode);
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_cbToUWriteSub(args: *mut UConverterToUnicodeArgs, offsetindex: i32, err: *mut UErrorCode);
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_cbToUWriteUChars(args: *mut UConverterToUnicodeArgs, source: *const u16, length: i32, offsetindex: i32, err: *mut UErrorCode);
    pub fn ucnv_close(converter: *mut UConverter);
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_compareNames(name1: super::Foundation::PSTR, name2: super::Foundation::PSTR) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_convert(toconvertername: super::Foundation::PSTR, fromconvertername: super::Foundation::PSTR, target: super::Foundation::PSTR, targetcapacity: i32, source: super::Foundation::PSTR, sourcelength: i32, perrorcode: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_convertEx(targetcnv: *mut UConverter, sourcecnv: *mut UConverter, target: *mut *mut i8, targetlimit: super::Foundation::PSTR, source: *const *const i8, sourcelimit: super::Foundation::PSTR, pivotstart: *mut u16, pivotsource: *mut *mut u16, pivottarget: *mut *mut u16, pivotlimit: *const u16, reset: i8, flush: i8, perrorcode: *mut UErrorCode);
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_countAliases(alias: super::Foundation::PSTR, perrorcode: *mut UErrorCode) -> u16;
    pub fn ucnv_countAvailable() -> i32;
    pub fn ucnv_countStandards() -> u16;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_detectUnicodeSignature(source: super::Foundation::PSTR, sourcelength: i32, signaturelength: *mut i32, perrorcode: *mut UErrorCode) -> super::Foundation::PSTR;
    pub fn ucnv_fixFileSeparator(cnv: *const UConverter, source: *mut u16, sourcelen: i32);
    pub fn ucnv_flushCache() -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_fromAlgorithmic(cnv: *mut UConverter, algorithmictype: UConverterType, target: super::Foundation::PSTR, targetcapacity: i32, source: super::Foundation::PSTR, sourcelength: i32, perrorcode: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_fromUChars(cnv: *mut UConverter, dest: super::Foundation::PSTR, destcapacity: i32, src: *const u16, srclength: i32, perrorcode: *mut UErrorCode) -> i32;
    pub fn ucnv_fromUCountPending(cnv: *const UConverter, status: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_fromUnicode(converter: *mut UConverter, target: *mut *mut i8, targetlimit: super::Foundation::PSTR, source: *const *const u16, sourcelimit: *const u16, offsets: *mut i32, flush: i8, err: *mut UErrorCode);
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_getAlias(alias: super::Foundation::PSTR, n: u16, perrorcode: *mut UErrorCode) -> super::Foundation::PSTR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_getAliases(alias: super::Foundation::PSTR, aliases: *const *const i8, perrorcode: *mut UErrorCode);
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_getAvailableName(n: i32) -> super::Foundation::PSTR;
    pub fn ucnv_getCCSID(converter: *const UConverter, err: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_getCanonicalName(alias: super::Foundation::PSTR, standard: super::Foundation::PSTR, perrorcode: *mut UErrorCode) -> super::Foundation::PSTR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_getDefaultName() -> super::Foundation::PSTR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_getDisplayName(converter: *const UConverter, displaylocale: super::Foundation::PSTR, displayname: *mut u16, displaynamecapacity: i32, err: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_getFromUCallBack(converter: *const UConverter, action: *mut UConverterFromUCallback, context: *const *const ::core::ffi::c_void);
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_getInvalidChars(converter: *const UConverter, errbytes: super::Foundation::PSTR, len: *mut i8, err: *mut UErrorCode);
    pub fn ucnv_getInvalidUChars(converter: *const UConverter, erruchars: *mut u16, len: *mut i8, err: *mut UErrorCode);
    pub fn ucnv_getMaxCharSize(converter: *const UConverter) -> i8;
    pub fn ucnv_getMinCharSize(converter: *const UConverter) -> i8;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_getName(converter: *const UConverter, err: *mut UErrorCode) -> super::Foundation::PSTR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_getNextUChar(converter: *mut UConverter, source: *const *const i8, sourcelimit: super::Foundation::PSTR, err: *mut UErrorCode) -> i32;
    pub fn ucnv_getPlatform(converter: *const UConverter, err: *mut UErrorCode) -> UConverterPlatform;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_getStandard(n: u16, perrorcode: *mut UErrorCode) -> super::Foundation::PSTR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_getStandardName(name: super::Foundation::PSTR, standard: super::Foundation::PSTR, perrorcode: *mut UErrorCode) -> super::Foundation::PSTR;
    pub fn ucnv_getStarters(converter: *const UConverter, starters: *mut i8, err: *mut UErrorCode);
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_getSubstChars(converter: *const UConverter, subchars: super::Foundation::PSTR, len: *mut i8, err: *mut UErrorCode);
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_getToUCallBack(converter: *const UConverter, action: *mut UConverterToUCallback, context: *const *const ::core::ffi::c_void);
    pub fn ucnv_getType(converter: *const UConverter) -> UConverterType;
    pub fn ucnv_getUnicodeSet(cnv: *const UConverter, setfillin: *mut USet, whichset: UConverterUnicodeSet, perrorcode: *mut UErrorCode);
    pub fn ucnv_isAmbiguous(cnv: *const UConverter) -> i8;
    pub fn ucnv_isFixedWidth(cnv: *mut UConverter, status: *mut UErrorCode) -> i8;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_open(convertername: super::Foundation::PSTR, err: *mut UErrorCode) -> *mut UConverter;
    pub fn ucnv_openAllNames(perrorcode: *mut UErrorCode) -> *mut UEnumeration;
    pub fn ucnv_openCCSID(codepage: i32, platform: UConverterPlatform, err: *mut UErrorCode) -> *mut UConverter;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_openPackage(packagename: super::Foundation::PSTR, convertername: super::Foundation::PSTR, err: *mut UErrorCode) -> *mut UConverter;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_openStandardNames(convname: super::Foundation::PSTR, standard: super::Foundation::PSTR, perrorcode: *mut UErrorCode) -> *mut UEnumeration;
    pub fn ucnv_openU(name: *const u16, err: *mut UErrorCode) -> *mut UConverter;
    pub fn ucnv_reset(converter: *mut UConverter);
    pub fn ucnv_resetFromUnicode(converter: *mut UConverter);
    pub fn ucnv_resetToUnicode(converter: *mut UConverter);
    pub fn ucnv_safeClone(cnv: *const UConverter, stackbuffer: *mut ::core::ffi::c_void, pbuffersize: *mut i32, status: *mut UErrorCode) -> *mut UConverter;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_setDefaultName(name: super::Foundation::PSTR);
    pub fn ucnv_setFallback(cnv: *mut UConverter, usesfallback: i8);
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_setFromUCallBack(converter: *mut UConverter, newaction: UConverterFromUCallback, newcontext: *const ::core::ffi::c_void, oldaction: *mut UConverterFromUCallback, oldcontext: *const *const ::core::ffi::c_void, err: *mut UErrorCode);
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_setSubstChars(converter: *mut UConverter, subchars: super::Foundation::PSTR, len: i8, err: *mut UErrorCode);
    pub fn ucnv_setSubstString(cnv: *mut UConverter, s: *const u16, length: i32, err: *mut UErrorCode);
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_setToUCallBack(converter: *mut UConverter, newaction: UConverterToUCallback, newcontext: *const ::core::ffi::c_void, oldaction: *mut UConverterToUCallback, oldcontext: *const *const ::core::ffi::c_void, err: *mut UErrorCode);
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_toAlgorithmic(algorithmictype: UConverterType, cnv: *mut UConverter, target: super::Foundation::PSTR, targetcapacity: i32, source: super::Foundation::PSTR, sourcelength: i32, perrorcode: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_toUChars(cnv: *mut UConverter, dest: *mut u16, destcapacity: i32, src: super::Foundation::PSTR, srclength: i32, perrorcode: *mut UErrorCode) -> i32;
    pub fn ucnv_toUCountPending(cnv: *const UConverter, status: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_toUnicode(converter: *mut UConverter, target: *mut *mut u16, targetlimit: *const u16, source: *const *const i8, sourcelimit: super::Foundation::PSTR, offsets: *mut i32, flush: i8, err: *mut UErrorCode);
    pub fn ucnv_usesFallback(cnv: *const UConverter) -> i8;
    pub fn ucnvsel_close(sel: *mut UConverterSelector);
    pub fn ucnvsel_open(converterlist: *const *const i8, converterlistsize: i32, excludedcodepoints: *const USet, whichset: UConverterUnicodeSet, status: *mut UErrorCode) -> *mut UConverterSelector;
    pub fn ucnvsel_openFromSerialized(buffer: *const ::core::ffi::c_void, length: i32, status: *mut UErrorCode) -> *mut UConverterSelector;
    pub fn ucnvsel_selectForString(sel: *const UConverterSelector, s: *const u16, length: i32, status: *mut UErrorCode) -> *mut UEnumeration;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnvsel_selectForUTF8(sel: *const UConverterSelector, s: super::Foundation::PSTR, length: i32, status: *mut UErrorCode) -> *mut UEnumeration;
    pub fn ucnvsel_serialize(sel: *const UConverterSelector, buffer: *mut ::core::ffi::c_void, buffercapacity: i32, status: *mut UErrorCode) -> i32;
    pub fn ucol_cloneBinary(coll: *const UCollator, buffer: *mut u8, capacity: i32, status: *mut UErrorCode) -> i32;
    pub fn ucol_close(coll: *mut UCollator);
    pub fn ucol_closeElements(elems: *mut UCollationElements);
    pub fn ucol_countAvailable() -> i32;
    pub fn ucol_equal(coll: *const UCollator, source: *const u16, sourcelength: i32, target: *const u16, targetlength: i32) -> i8;
    pub fn ucol_getAttribute(coll: *const UCollator, attr: UColAttribute, status: *mut UErrorCode) -> UColAttributeValue;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucol_getAvailable(localeindex: i32) -> super::Foundation::PSTR;
    pub fn ucol_getBound(source: *const u8, sourcelength: i32, boundtype: UColBoundMode, nooflevels: u32, result: *mut u8, resultlength: i32, status: *mut UErrorCode) -> i32;
    pub fn ucol_getContractionsAndExpansions(coll: *const UCollator, contractions: *mut USet, expansions: *mut USet, addprefixes: i8, status: *mut UErrorCode);
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucol_getDisplayName(objloc: super::Foundation::PSTR, disploc: super::Foundation::PSTR, result: *mut u16, resultlength: i32, status: *mut UErrorCode) -> i32;
    pub fn ucol_getEquivalentReorderCodes(reordercode: i32, dest: *mut i32, destcapacity: i32, perrorcode: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucol_getFunctionalEquivalent(result: super::Foundation::PSTR, resultcapacity: i32, keyword: super::Foundation::PSTR, locale: super::Foundation::PSTR, isavailable: *mut i8, status: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucol_getKeywordValues(keyword: super::Foundation::PSTR, status: *mut UErrorCode) -> *mut UEnumeration;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucol_getKeywordValuesForLocale(key: super::Foundation::PSTR, locale: super::Foundation::PSTR, commonlyused: i8, status: *mut UErrorCode) -> *mut UEnumeration;
    pub fn ucol_getKeywords(status: *mut UErrorCode) -> *mut UEnumeration;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucol_getLocaleByType(coll: *const UCollator, r#type: ULocDataLocaleType, status: *mut UErrorCode) -> super::Foundation::PSTR;
    pub fn ucol_getMaxExpansion(elems: *const UCollationElements, order: i32) -> i32;
    pub fn ucol_getMaxVariable(coll: *const UCollator) -> UColReorderCode;
    pub fn ucol_getOffset(elems: *const UCollationElements) -> i32;
    pub fn ucol_getReorderCodes(coll: *const UCollator, dest: *mut i32, destcapacity: i32, perrorcode: *mut UErrorCode) -> i32;
    pub fn ucol_getRules(coll: *const UCollator, length: *mut i32) -> *mut u16;
    pub fn ucol_getRulesEx(coll: *const UCollator, delta: UColRuleOption, buffer: *mut u16, bufferlen: i32) -> i32;
    pub fn ucol_getSortKey(coll: *const UCollator, source: *const u16, sourcelength: i32, result: *mut u8, resultlength: i32) -> i32;
    pub fn ucol_getStrength(coll: *const UCollator) -> UColAttributeValue;
    pub fn ucol_getTailoredSet(coll: *const UCollator, status: *mut UErrorCode) -> *mut USet;
    pub fn ucol_getUCAVersion(coll: *const UCollator, info: *mut u8);
    pub fn ucol_getVariableTop(coll: *const UCollator, status: *mut UErrorCode) -> u32;
    pub fn ucol_getVersion(coll: *const UCollator, info: *mut u8);
    pub fn ucol_greater(coll: *const UCollator, source: *const u16, sourcelength: i32, target: *const u16, targetlength: i32) -> i8;
    pub fn ucol_greaterOrEqual(coll: *const UCollator, source: *const u16, sourcelength: i32, target: *const u16, targetlength: i32) -> i8;
    pub fn ucol_keyHashCode(key: *const u8, length: i32) -> i32;
    pub fn ucol_mergeSortkeys(src1: *const u8, src1length: i32, src2: *const u8, src2length: i32, dest: *mut u8, destcapacity: i32) -> i32;
    pub fn ucol_next(elems: *mut UCollationElements, status: *mut UErrorCode) -> i32;
    pub fn ucol_nextSortKeyPart(coll: *const UCollator, iter: *mut UCharIterator, state: *mut u32, dest: *mut u8, count: i32, status: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucol_open(loc: super::Foundation::PSTR, status: *mut UErrorCode) -> *mut UCollator;
    pub fn ucol_openAvailableLocales(status: *mut UErrorCode) -> *mut UEnumeration;
    pub fn ucol_openBinary(bin: *const u8, length: i32, base: *const UCollator, status: *mut UErrorCode) -> *mut UCollator;
    pub fn ucol_openElements(coll: *const UCollator, text: *const u16, textlength: i32, status: *mut UErrorCode) -> *mut UCollationElements;
    pub fn ucol_openRules(rules: *const u16, ruleslength: i32, normalizationmode: UColAttributeValue, strength: UColAttributeValue, parseerror: *mut UParseError, status: *mut UErrorCode) -> *mut UCollator;
    pub fn ucol_previous(elems: *mut UCollationElements, status: *mut UErrorCode) -> i32;
    pub fn ucol_primaryOrder(order: i32) -> i32;
    pub fn ucol_reset(elems: *mut UCollationElements);
    pub fn ucol_safeClone(coll: *const UCollator, stackbuffer: *mut ::core::ffi::c_void, pbuffersize: *mut i32, status: *mut UErrorCode) -> *mut UCollator;
    pub fn ucol_secondaryOrder(order: i32) -> i32;
    pub fn ucol_setAttribute(coll: *mut UCollator, attr: UColAttribute, value: UColAttributeValue, status: *mut UErrorCode);
    pub fn ucol_setMaxVariable(coll: *mut UCollator, group: UColReorderCode, perrorcode: *mut UErrorCode);
    pub fn ucol_setOffset(elems: *mut UCollationElements, offset: i32, status: *mut UErrorCode);
    pub fn ucol_setReorderCodes(coll: *mut UCollator, reordercodes: *const i32, reordercodeslength: i32, perrorcode: *mut UErrorCode);
    pub fn ucol_setStrength(coll: *mut UCollator, strength: UColAttributeValue);
    pub fn ucol_setText(elems: *mut UCollationElements, text: *const u16, textlength: i32, status: *mut UErrorCode);
    pub fn ucol_strcoll(coll: *const UCollator, source: *const u16, sourcelength: i32, target: *const u16, targetlength: i32) -> UCollationResult;
    pub fn ucol_strcollIter(coll: *const UCollator, siter: *mut UCharIterator, titer: *mut UCharIterator, status: *mut UErrorCode) -> UCollationResult;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucol_strcollUTF8(coll: *const UCollator, source: super::Foundation::PSTR, sourcelength: i32, target: super::Foundation::PSTR, targetlength: i32, status: *mut UErrorCode) -> UCollationResult;
    pub fn ucol_tertiaryOrder(order: i32) -> i32;
    pub fn ucpmap_get(map: *const UCPMap, c: i32) -> u32;
    pub fn ucpmap_getRange(map: *const UCPMap, start: i32, option: UCPMapRangeOption, surrogatevalue: u32, filter: *mut UCPMapValueFilter, context: *const ::core::ffi::c_void, pvalue: *mut u32) -> i32;
    pub fn ucptrie_close(trie: *mut UCPTrie);
    pub fn ucptrie_get(trie: *const UCPTrie, c: i32) -> u32;
    pub fn ucptrie_getRange(trie: *const UCPTrie, start: i32, option: UCPMapRangeOption, surrogatevalue: u32, filter: *mut UCPMapValueFilter, context: *const ::core::ffi::c_void, pvalue: *mut u32) -> i32;
    pub fn ucptrie_getType(trie: *const UCPTrie) -> UCPTrieType;
    pub fn ucptrie_getValueWidth(trie: *const UCPTrie) -> UCPTrieValueWidth;
    pub fn ucptrie_internalSmallIndex(trie: *const UCPTrie, c: i32) -> i32;
    pub fn ucptrie_internalSmallU8Index(trie: *const UCPTrie, lt1: i32, t2: u8, t3: u8) -> i32;
    pub fn ucptrie_internalU8PrevIndex(trie: *const UCPTrie, c: i32, start: *const u8, src: *const u8) -> i32;
    pub fn ucptrie_openFromBinary(r#type: UCPTrieType, valuewidth: UCPTrieValueWidth, data: *const ::core::ffi::c_void, length: i32, pactuallength: *mut i32, perrorcode: *mut UErrorCode) -> *mut UCPTrie;
    pub fn ucptrie_toBinary(trie: *const UCPTrie, data: *mut ::core::ffi::c_void, capacity: i32, perrorcode: *mut UErrorCode) -> i32;
    pub fn ucsdet_close(ucsd: *mut UCharsetDetector);
    pub fn ucsdet_detect(ucsd: *mut UCharsetDetector, status: *mut UErrorCode) -> *mut UCharsetMatch;
    pub fn ucsdet_detectAll(ucsd: *mut UCharsetDetector, matchesfound: *mut i32, status: *mut UErrorCode) -> *mut *mut UCharsetMatch;
    pub fn ucsdet_enableInputFilter(ucsd: *mut UCharsetDetector, filter: i8) -> i8;
    pub fn ucsdet_getAllDetectableCharsets(ucsd: *const UCharsetDetector, status: *mut UErrorCode) -> *mut UEnumeration;
    pub fn ucsdet_getConfidence(ucsm: *const UCharsetMatch, status: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucsdet_getLanguage(ucsm: *const UCharsetMatch, status: *mut UErrorCode) -> super::Foundation::PSTR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucsdet_getName(ucsm: *const UCharsetMatch, status: *mut UErrorCode) -> super::Foundation::PSTR;
    pub fn ucsdet_getUChars(ucsm: *const UCharsetMatch, buf: *mut u16, cap: i32, status: *mut UErrorCode) -> i32;
    pub fn ucsdet_isInputFilterEnabled(ucsd: *const UCharsetDetector) -> i8;
    pub fn ucsdet_open(status: *mut UErrorCode) -> *mut UCharsetDetector;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucsdet_setDeclaredEncoding(ucsd: *mut UCharsetDetector, encoding: super::Foundation::PSTR, length: i32, status: *mut UErrorCode);
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucsdet_setText(ucsd: *mut UCharsetDetector, textin: super::Foundation::PSTR, len: i32, status: *mut UErrorCode);
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucurr_countCurrencies(locale: super::Foundation::PSTR, date: f64, ec: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucurr_forLocale(locale: super::Foundation::PSTR, buff: *mut u16, buffcapacity: i32, ec: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucurr_forLocaleAndDate(locale: super::Foundation::PSTR, date: f64, index: i32, buff: *mut u16, buffcapacity: i32, ec: *mut UErrorCode) -> i32;
    pub fn ucurr_getDefaultFractionDigits(currency: *const u16, ec: *mut UErrorCode) -> i32;
    pub fn ucurr_getDefaultFractionDigitsForUsage(currency: *const u16, usage: UCurrencyUsage, ec: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucurr_getKeywordValuesForLocale(key: super::Foundation::PSTR, locale: super::Foundation::PSTR, commonlyused: i8, status: *mut UErrorCode) -> *mut UEnumeration;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucurr_getName(currency: *const u16, locale: super::Foundation::PSTR, namestyle: UCurrNameStyle, ischoiceformat: *mut i8, len: *mut i32, ec: *mut UErrorCode) -> *mut u16;
    pub fn ucurr_getNumericCode(currency: *const u16) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucurr_getPluralName(currency: *const u16, locale: super::Foundation::PSTR, ischoiceformat: *mut i8, pluralcount: super::Foundation::PSTR, len: *mut i32, ec: *mut UErrorCode) -> *mut u16;
    pub fn ucurr_getRoundingIncrement(currency: *const u16, ec: *mut UErrorCode) -> f64;
    pub fn ucurr_getRoundingIncrementForUsage(currency: *const u16, usage: UCurrencyUsage, ec: *mut UErrorCode) -> f64;
    pub fn ucurr_isAvailable(isocode: *const u16, from: f64, to: f64, errorcode: *mut UErrorCode) -> i8;
    pub fn ucurr_openISOCurrencies(currtype: u32, perrorcode: *mut UErrorCode) -> *mut UEnumeration;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucurr_register(isocode: *const u16, locale: super::Foundation::PSTR, status: *mut UErrorCode) -> *mut ::core::ffi::c_void;
    pub fn ucurr_unregister(key: *mut ::core::ffi::c_void, status: *mut UErrorCode) -> i8;
    pub fn udat_adoptNumberFormat(fmt: *mut *mut ::core::ffi::c_void, numberformattoadopt: *mut *mut ::core::ffi::c_void);
    pub fn udat_adoptNumberFormatForFields(fmt: *mut *mut ::core::ffi::c_void, fields: *const u16, numberformattoset: *mut *mut ::core::ffi::c_void, status: *mut UErrorCode);
    pub fn udat_applyPattern(format: *mut *mut ::core::ffi::c_void, localized: i8, pattern: *const u16, patternlength: i32);
    pub fn udat_clone(fmt: *const *const ::core::ffi::c_void, status: *mut UErrorCode) -> *mut *mut ::core::ffi::c_void;
    pub fn udat_close(format: *mut *mut ::core::ffi::c_void);
    pub fn udat_countAvailable() -> i32;
    pub fn udat_countSymbols(fmt: *const *const ::core::ffi::c_void, r#type: UDateFormatSymbolType) -> i32;
    pub fn udat_format(format: *const *const ::core::ffi::c_void, datetoformat: f64, result: *mut u16, resultlength: i32, position: *mut UFieldPosition, status: *mut UErrorCode) -> i32;
    pub fn udat_formatCalendar(format: *const *const ::core::ffi::c_void, calendar: *mut *mut ::core::ffi::c_void, result: *mut u16, capacity: i32, position: *mut UFieldPosition, status: *mut UErrorCode) -> i32;
    pub fn udat_formatCalendarForFields(format: *const *const ::core::ffi::c_void, calendar: *mut *mut ::core::ffi::c_void, result: *mut u16, capacity: i32, fpositer: *mut UFieldPositionIterator, status: *mut UErrorCode) -> i32;
    pub fn udat_formatForFields(format: *const *const ::core::ffi::c_void, datetoformat: f64, result: *mut u16, resultlength: i32, fpositer: *mut UFieldPositionIterator, status: *mut UErrorCode) -> i32;
    pub fn udat_get2DigitYearStart(fmt: *const *const ::core::ffi::c_void, status: *mut UErrorCode) -> f64;
    #[cfg(feature = "Win32_Foundation")]
    pub fn udat_getAvailable(localeindex: i32) -> super::Foundation::PSTR;
    pub fn udat_getBooleanAttribute(fmt: *const *const ::core::ffi::c_void, attr: UDateFormatBooleanAttribute, status: *mut UErrorCode) -> i8;
    pub fn udat_getCalendar(fmt: *const *const ::core::ffi::c_void) -> *mut *mut ::core::ffi::c_void;
    pub fn udat_getContext(fmt: *const *const ::core::ffi::c_void, r#type: UDisplayContextType, status: *mut UErrorCode) -> UDisplayContext;
    #[cfg(feature = "Win32_Foundation")]
    pub fn udat_getLocaleByType(fmt: *const *const ::core::ffi::c_void, r#type: ULocDataLocaleType, status: *mut UErrorCode) -> super::Foundation::PSTR;
    pub fn udat_getNumberFormat(fmt: *const *const ::core::ffi::c_void) -> *mut *mut ::core::ffi::c_void;
    pub fn udat_getNumberFormatForField(fmt: *const *const ::core::ffi::c_void, field: u16) -> *mut *mut ::core::ffi::c_void;
    pub fn udat_getSymbols(fmt: *const *const ::core::ffi::c_void, r#type: UDateFormatSymbolType, symbolindex: i32, result: *mut u16, resultlength: i32, status: *mut UErrorCode) -> i32;
    pub fn udat_isLenient(fmt: *const *const ::core::ffi::c_void) -> i8;
    #[cfg(feature = "Win32_Foundation")]
    pub fn udat_open(timestyle: UDateFormatStyle, datestyle: UDateFormatStyle, locale: super::Foundation::PSTR, tzid: *const u16, tzidlength: i32, pattern: *const u16, patternlength: i32, status: *mut UErrorCode) -> *mut *mut ::core::ffi::c_void;
    pub fn udat_parse(format: *const *const ::core::ffi::c_void, text: *const u16, textlength: i32, parsepos: *mut i32, status: *mut UErrorCode) -> f64;
    pub fn udat_parseCalendar(format: *const *const ::core::ffi::c_void, calendar: *mut *mut ::core::ffi::c_void, text: *const u16, textlength: i32, parsepos: *mut i32, status: *mut UErrorCode);
    pub fn udat_set2DigitYearStart(fmt: *mut *mut ::core::ffi::c_void, d: f64, status: *mut UErrorCode);
    pub fn udat_setBooleanAttribute(fmt: *mut *mut ::core::ffi::c_void, attr: UDateFormatBooleanAttribute, newvalue: i8, status: *mut UErrorCode);
    pub fn udat_setCalendar(fmt: *mut *mut ::core::ffi::c_void, calendartoset: *const *const ::core::ffi::c_void);
    pub fn udat_setContext(fmt: *mut *mut ::core::ffi::c_void, value: UDisplayContext, status: *mut UErrorCode);
    pub fn udat_setLenient(fmt: *mut *mut ::core::ffi::c_void, islenient: i8);
    pub fn udat_setNumberFormat(fmt: *mut *mut ::core::ffi::c_void, numberformattoset: *const *const ::core::ffi::c_void);
    pub fn udat_setSymbols(format: *mut *mut ::core::ffi::c_void, r#type: UDateFormatSymbolType, symbolindex: i32, value: *mut u16, valuelength: i32, status: *mut UErrorCode);
    pub fn udat_toCalendarDateField(field: UDateFormatField) -> UCalendarDateFields;
    pub fn udat_toPattern(fmt: *const *const ::core::ffi::c_void, localized: i8, result: *mut u16, resultlength: i32, status: *mut UErrorCode) -> i32;
    pub fn udatpg_addPattern(dtpg: *mut *mut ::core::ffi::c_void, pattern: *const u16, patternlength: i32, r#override: i8, conflictingpattern: *mut u16, capacity: i32, plength: *mut i32, perrorcode: *mut UErrorCode) -> UDateTimePatternConflict;
    pub fn udatpg_clone(dtpg: *const *const ::core::ffi::c_void, perrorcode: *mut UErrorCode) -> *mut *mut ::core::ffi::c_void;
    pub fn udatpg_close(dtpg: *mut *mut ::core::ffi::c_void);
    pub fn udatpg_getAppendItemFormat(dtpg: *const *const ::core::ffi::c_void, field: UDateTimePatternField, plength: *mut i32) -> *mut u16;
    pub fn udatpg_getAppendItemName(dtpg: *const *const ::core::ffi::c_void, field: UDateTimePatternField, plength: *mut i32) -> *mut u16;
    pub fn udatpg_getBaseSkeleton(unuseddtpg: *mut *mut ::core::ffi::c_void, pattern: *const u16, length: i32, baseskeleton: *mut u16, capacity: i32, perrorcode: *mut UErrorCode) -> i32;
    pub fn udatpg_getBestPattern(dtpg: *mut *mut ::core::ffi::c_void, skeleton: *const u16, length: i32, bestpattern: *mut u16, capacity: i32, perrorcode: *mut UErrorCode) -> i32;
    pub fn udatpg_getBestPatternWithOptions(dtpg: *mut *mut ::core::ffi::c_void, skeleton: *const u16, length: i32, options: UDateTimePatternMatchOptions, bestpattern: *mut u16, capacity: i32, perrorcode: *mut UErrorCode) -> i32;
    pub fn udatpg_getDateTimeFormat(dtpg: *const *const ::core::ffi::c_void, plength: *mut i32) -> *mut u16;
    pub fn udatpg_getDecimal(dtpg: *const *const ::core::ffi::c_void, plength: *mut i32) -> *mut u16;
    pub fn udatpg_getFieldDisplayName(dtpg: *const *const ::core::ffi::c_void, field: UDateTimePatternField, width: UDateTimePGDisplayWidth, fieldname: *mut u16, capacity: i32, perrorcode: *mut UErrorCode) -> i32;
    pub fn udatpg_getPatternForSkeleton(dtpg: *const *const ::core::ffi::c_void, skeleton: *const u16, skeletonlength: i32, plength: *mut i32) -> *mut u16;
    pub fn udatpg_getSkeleton(unuseddtpg: *mut *mut ::core::ffi::c_void, pattern: *const u16, length: i32, skeleton: *mut u16, capacity: i32, perrorcode: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn udatpg_open(locale: super::Foundation::PSTR, perrorcode: *mut UErrorCode) -> *mut *mut ::core::ffi::c_void;
    pub fn udatpg_openBaseSkeletons(dtpg: *const *const ::core::ffi::c_void, perrorcode: *mut UErrorCode) -> *mut UEnumeration;
    pub fn udatpg_openEmpty(perrorcode: *mut UErrorCode) -> *mut *mut ::core::ffi::c_void;
    pub fn udatpg_openSkeletons(dtpg: *const *const ::core::ffi::c_void, perrorcode: *mut UErrorCode) -> *mut UEnumeration;
    pub fn udatpg_replaceFieldTypes(dtpg: *mut *mut ::core::ffi::c_void, pattern: *const u16, patternlength: i32, skeleton: *const u16, skeletonlength: i32, dest: *mut u16, destcapacity: i32, perrorcode: *mut UErrorCode) -> i32;
    pub fn udatpg_replaceFieldTypesWithOptions(dtpg: *mut *mut ::core::ffi::c_void, pattern: *const u16, patternlength: i32, skeleton: *const u16, skeletonlength: i32, options: UDateTimePatternMatchOptions, dest: *mut u16, destcapacity: i32, perrorcode: *mut UErrorCode) -> i32;
    pub fn udatpg_setAppendItemFormat(dtpg: *mut *mut ::core::ffi::c_void, field: UDateTimePatternField, value: *const u16, length: i32);
    pub fn udatpg_setAppendItemName(dtpg: *mut *mut ::core::ffi::c_void, field: UDateTimePatternField, value: *const u16, length: i32);
    pub fn udatpg_setDateTimeFormat(dtpg: *const *const ::core::ffi::c_void, dtformat: *const u16, length: i32);
    pub fn udatpg_setDecimal(dtpg: *mut *mut ::core::ffi::c_void, decimal: *const u16, length: i32);
    pub fn udtitvfmt_close(formatter: *mut UDateIntervalFormat);
    pub fn udtitvfmt_closeResult(uresult: *mut UFormattedDateInterval);
    pub fn udtitvfmt_format(formatter: *const UDateIntervalFormat, fromdate: f64, todate: f64, result: *mut u16, resultcapacity: i32, position: *mut UFieldPosition, status: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn udtitvfmt_open(locale: super::Foundation::PSTR, skeleton: *const u16, skeletonlength: i32, tzid: *const u16, tzidlength: i32, status: *mut UErrorCode) -> *mut UDateIntervalFormat;
    pub fn udtitvfmt_openResult(ec: *mut UErrorCode) -> *mut UFormattedDateInterval;
    pub fn udtitvfmt_resultAsValue(uresult: *const UFormattedDateInterval, ec: *mut UErrorCode) -> *mut UFormattedValue;
    pub fn uenum_close(en: *mut UEnumeration);
    pub fn uenum_count(en: *mut UEnumeration, status: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn uenum_next(en: *mut UEnumeration, resultlength: *mut i32, status: *mut UErrorCode) -> super::Foundation::PSTR;
    pub fn uenum_openCharStringsEnumeration(strings: *const *const i8, count: i32, ec: *mut UErrorCode) -> *mut UEnumeration;
    pub fn uenum_openUCharStringsEnumeration(strings: *const *const u16, count: i32, ec: *mut UErrorCode) -> *mut UEnumeration;
    pub fn uenum_reset(en: *mut UEnumeration, status: *mut UErrorCode);
    pub fn uenum_unext(en: *mut UEnumeration, resultlength: *mut i32, status: *mut UErrorCode) -> *mut u16;
    pub fn ufieldpositer_close(fpositer: *mut UFieldPositionIterator);
    pub fn ufieldpositer_next(fpositer: *mut UFieldPositionIterator, beginindex: *mut i32, endindex: *mut i32) -> i32;
    pub fn ufieldpositer_open(status: *mut UErrorCode) -> *mut UFieldPositionIterator;
    pub fn ufmt_close(fmt: *mut *mut ::core::ffi::c_void);
    pub fn ufmt_getArrayItemByIndex(fmt: *mut *mut ::core::ffi::c_void, n: i32, status: *mut UErrorCode) -> *mut *mut ::core::ffi::c_void;
    pub fn ufmt_getArrayLength(fmt: *const *const ::core::ffi::c_void, status: *mut UErrorCode) -> i32;
    pub fn ufmt_getDate(fmt: *const *const ::core::ffi::c_void, status: *mut UErrorCode) -> f64;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ufmt_getDecNumChars(fmt: *mut *mut ::core::ffi::c_void, len: *mut i32, status: *mut UErrorCode) -> super::Foundation::PSTR;
    pub fn ufmt_getDouble(fmt: *mut *mut ::core::ffi::c_void, status: *mut UErrorCode) -> f64;
    pub fn ufmt_getInt64(fmt: *mut *mut ::core::ffi::c_void, status: *mut UErrorCode) -> i64;
    pub fn ufmt_getLong(fmt: *mut *mut ::core::ffi::c_void, status: *mut UErrorCode) -> i32;
    pub fn ufmt_getObject(fmt: *const *const ::core::ffi::c_void, status: *mut UErrorCode) -> *mut ::core::ffi::c_void;
    pub fn ufmt_getType(fmt: *const *const ::core::ffi::c_void, status: *mut UErrorCode) -> UFormattableType;
    pub fn ufmt_getUChars(fmt: *mut *mut ::core::ffi::c_void, len: *mut i32, status: *mut UErrorCode) -> *mut u16;
    pub fn ufmt_isNumeric(fmt: *const *const ::core::ffi::c_void) -> i8;
    pub fn ufmt_open(status: *mut UErrorCode) -> *mut *mut ::core::ffi::c_void;
    pub fn ufmtval_getString(ufmtval: *const UFormattedValue, plength: *mut i32, ec: *mut UErrorCode) -> *mut u16;
    pub fn ufmtval_nextPosition(ufmtval: *const UFormattedValue, ucfpos: *mut UConstrainedFieldPosition, ec: *mut UErrorCode) -> i8;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ugender_getInstance(locale: super::Foundation::PSTR, status: *mut UErrorCode) -> *mut UGenderInfo;
    pub fn ugender_getListGender(genderinfo: *const UGenderInfo, genders: *const UGender, size: i32, status: *mut UErrorCode) -> UGender;
    pub fn uidna_close(idna: *mut UIDNA);
    pub fn uidna_labelToASCII(idna: *const UIDNA, label: *const u16, length: i32, dest: *mut u16, capacity: i32, pinfo: *mut UIDNAInfo, perrorcode: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn uidna_labelToASCII_UTF8(idna: *const UIDNA, label: super::Foundation::PSTR, length: i32, dest: super::Foundation::PSTR, capacity: i32, pinfo: *mut UIDNAInfo, perrorcode: *mut UErrorCode) -> i32;
    pub fn uidna_labelToUnicode(idna: *const UIDNA, label: *const u16, length: i32, dest: *mut u16, capacity: i32, pinfo: *mut UIDNAInfo, perrorcode: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn uidna_labelToUnicodeUTF8(idna: *const UIDNA, label: super::Foundation::PSTR, length: i32, dest: super::Foundation::PSTR, capacity: i32, pinfo: *mut UIDNAInfo, perrorcode: *mut UErrorCode) -> i32;
    pub fn uidna_nameToASCII(idna: *const UIDNA, name: *const u16, length: i32, dest: *mut u16, capacity: i32, pinfo: *mut UIDNAInfo, perrorcode: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn uidna_nameToASCII_UTF8(idna: *const UIDNA, name: super::Foundation::PSTR, length: i32, dest: super::Foundation::PSTR, capacity: i32, pinfo: *mut UIDNAInfo, perrorcode: *mut UErrorCode) -> i32;
    pub fn uidna_nameToUnicode(idna: *const UIDNA, name: *const u16, length: i32, dest: *mut u16, capacity: i32, pinfo: *mut UIDNAInfo, perrorcode: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn uidna_nameToUnicodeUTF8(idna: *const UIDNA, name: super::Foundation::PSTR, length: i32, dest: super::Foundation::PSTR, capacity: i32, pinfo: *mut UIDNAInfo, perrorcode: *mut UErrorCode) -> i32;
    pub fn uidna_openUTS46(options: u32, perrorcode: *mut UErrorCode) -> *mut UIDNA;
    pub fn uiter_current32(iter: *mut UCharIterator) -> i32;
    pub fn uiter_getState(iter: *const UCharIterator) -> u32;
    pub fn uiter_next32(iter: *mut UCharIterator) -> i32;
    pub fn uiter_previous32(iter: *mut UCharIterator) -> i32;
    pub fn uiter_setState(iter: *mut UCharIterator, state: u32, perrorcode: *mut UErrorCode);
    pub fn uiter_setString(iter: *mut UCharIterator, s: *const u16, length: i32);
    #[cfg(feature = "Win32_Foundation")]
    pub fn uiter_setUTF16BE(iter: *mut UCharIterator, s: super::Foundation::PSTR, length: i32);
    #[cfg(feature = "Win32_Foundation")]
    pub fn uiter_setUTF8(iter: *mut UCharIterator, s: super::Foundation::PSTR, length: i32);
    pub fn uldn_close(ldn: *mut ULocaleDisplayNames);
    pub fn uldn_getContext(ldn: *const ULocaleDisplayNames, r#type: UDisplayContextType, perrorcode: *mut UErrorCode) -> UDisplayContext;
    pub fn uldn_getDialectHandling(ldn: *const ULocaleDisplayNames) -> UDialectHandling;
    #[cfg(feature = "Win32_Foundation")]
    pub fn uldn_getLocale(ldn: *const ULocaleDisplayNames) -> super::Foundation::PSTR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn uldn_keyDisplayName(ldn: *const ULocaleDisplayNames, key: super::Foundation::PSTR, result: *mut u16, maxresultsize: i32, perrorcode: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn uldn_keyValueDisplayName(ldn: *const ULocaleDisplayNames, key: super::Foundation::PSTR, value: super::Foundation::PSTR, result: *mut u16, maxresultsize: i32, perrorcode: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn uldn_languageDisplayName(ldn: *const ULocaleDisplayNames, lang: super::Foundation::PSTR, result: *mut u16, maxresultsize: i32, perrorcode: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn uldn_localeDisplayName(ldn: *const ULocaleDisplayNames, locale: super::Foundation::PSTR, result: *mut u16, maxresultsize: i32, perrorcode: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn uldn_open(locale: super::Foundation::PSTR, dialecthandling: UDialectHandling, perrorcode: *mut UErrorCode) -> *mut ULocaleDisplayNames;
    #[cfg(feature = "Win32_Foundation")]
    pub fn uldn_openForContext(locale: super::Foundation::PSTR, contexts: *mut UDisplayContext, length: i32, perrorcode: *mut UErrorCode) -> *mut ULocaleDisplayNames;
    #[cfg(feature = "Win32_Foundation")]
    pub fn uldn_regionDisplayName(ldn: *const ULocaleDisplayNames, region: super::Foundation::PSTR, result: *mut u16, maxresultsize: i32, perrorcode: *mut UErrorCode) -> i32;
    pub fn uldn_scriptCodeDisplayName(ldn: *const ULocaleDisplayNames, scriptcode: UScriptCode, result: *mut u16, maxresultsize: i32, perrorcode: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn uldn_scriptDisplayName(ldn: *const ULocaleDisplayNames, script: super::Foundation::PSTR, result: *mut u16, maxresultsize: i32, perrorcode: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn uldn_variantDisplayName(ldn: *const ULocaleDisplayNames, variant: super::Foundation::PSTR, result: *mut u16, maxresultsize: i32, perrorcode: *mut UErrorCode) -> i32;
    pub fn ulistfmt_close(listfmt: *mut UListFormatter);
    pub fn ulistfmt_closeResult(uresult: *mut UFormattedList);
    pub fn ulistfmt_format(listfmt: *const UListFormatter, strings: *const *const u16, stringlengths: *const i32, stringcount: i32, result: *mut u16, resultcapacity: i32, status: *mut UErrorCode) -> i32;
    pub fn ulistfmt_formatStringsToResult(listfmt: *const UListFormatter, strings: *const *const u16, stringlengths: *const i32, stringcount: i32, uresult: *mut UFormattedList, status: *mut UErrorCode);
    #[cfg(feature = "Win32_Foundation")]
    pub fn ulistfmt_open(locale: super::Foundation::PSTR, status: *mut UErrorCode) -> *mut UListFormatter;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ulistfmt_openForType(locale: super::Foundation::PSTR, r#type: UListFormatterType, width: UListFormatterWidth, status: *mut UErrorCode) -> *mut UListFormatter;
    pub fn ulistfmt_openResult(ec: *mut UErrorCode) -> *mut UFormattedList;
    pub fn ulistfmt_resultAsValue(uresult: *const UFormattedList, ec: *mut UErrorCode) -> *mut UFormattedValue;
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_acceptLanguage(result: super::Foundation::PSTR, resultavailable: i32, outresult: *mut UAcceptResult, acceptlist: *const *const i8, acceptlistcount: i32, availablelocales: *mut UEnumeration, status: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_acceptLanguageFromHTTP(result: super::Foundation::PSTR, resultavailable: i32, outresult: *mut UAcceptResult, httpacceptlanguage: super::Foundation::PSTR, availablelocales: *mut UEnumeration, status: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_addLikelySubtags(localeid: super::Foundation::PSTR, maximizedlocaleid: super::Foundation::PSTR, maximizedlocaleidcapacity: i32, err: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_canonicalize(localeid: super::Foundation::PSTR, name: super::Foundation::PSTR, namecapacity: i32, err: *mut UErrorCode) -> i32;
    pub fn uloc_countAvailable() -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_forLanguageTag(langtag: super::Foundation::PSTR, localeid: super::Foundation::PSTR, localeidcapacity: i32, parsedlength: *mut i32, err: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_getAvailable(n: i32) -> super::Foundation::PSTR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_getBaseName(localeid: super::Foundation::PSTR, name: super::Foundation::PSTR, namecapacity: i32, err: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_getCharacterOrientation(localeid: super::Foundation::PSTR, status: *mut UErrorCode) -> ULayoutType;
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_getCountry(localeid: super::Foundation::PSTR, country: super::Foundation::PSTR, countrycapacity: i32, err: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_getDefault() -> super::Foundation::PSTR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_getDisplayCountry(locale: super::Foundation::PSTR, displaylocale: super::Foundation::PSTR, country: *mut u16, countrycapacity: i32, status: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_getDisplayKeyword(keyword: super::Foundation::PSTR, displaylocale: super::Foundation::PSTR, dest: *mut u16, destcapacity: i32, status: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_getDisplayKeywordValue(locale: super::Foundation::PSTR, keyword: super::Foundation::PSTR, displaylocale: super::Foundation::PSTR, dest: *mut u16, destcapacity: i32, status: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_getDisplayLanguage(locale: super::Foundation::PSTR, displaylocale: super::Foundation::PSTR, language: *mut u16, languagecapacity: i32, status: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_getDisplayName(localeid: super::Foundation::PSTR, inlocaleid: super::Foundation::PSTR, result: *mut u16, maxresultsize: i32, err: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_getDisplayScript(locale: super::Foundation::PSTR, displaylocale: super::Foundation::PSTR, script: *mut u16, scriptcapacity: i32, status: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_getDisplayVariant(locale: super::Foundation::PSTR, displaylocale: super::Foundation::PSTR, variant: *mut u16, variantcapacity: i32, status: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_getISO3Country(localeid: super::Foundation::PSTR) -> super::Foundation::PSTR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_getISO3Language(localeid: super::Foundation::PSTR) -> super::Foundation::PSTR;
    pub fn uloc_getISOCountries() -> *mut *mut i8;
    pub fn uloc_getISOLanguages() -> *mut *mut i8;
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_getKeywordValue(localeid: super::Foundation::PSTR, keywordname: super::Foundation::PSTR, buffer: super::Foundation::PSTR, buffercapacity: i32, status: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_getLCID(localeid: super::Foundation::PSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_getLanguage(localeid: super::Foundation::PSTR, language: super::Foundation::PSTR, languagecapacity: i32, err: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_getLineOrientation(localeid: super::Foundation::PSTR, status: *mut UErrorCode) -> ULayoutType;
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_getLocaleForLCID(hostid: u32, locale: super::Foundation::PSTR, localecapacity: i32, status: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_getName(localeid: super::Foundation::PSTR, name: super::Foundation::PSTR, namecapacity: i32, err: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_getParent(localeid: super::Foundation::PSTR, parent: super::Foundation::PSTR, parentcapacity: i32, err: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_getScript(localeid: super::Foundation::PSTR, script: super::Foundation::PSTR, scriptcapacity: i32, err: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_getVariant(localeid: super::Foundation::PSTR, variant: super::Foundation::PSTR, variantcapacity: i32, err: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_isRightToLeft(locale: super::Foundation::PSTR) -> i8;
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_minimizeSubtags(localeid: super::Foundation::PSTR, minimizedlocaleid: super::Foundation::PSTR, minimizedlocaleidcapacity: i32, err: *mut UErrorCode) -> i32;
    pub fn uloc_openAvailableByType(r#type: ULocAvailableType, status: *mut UErrorCode) -> *mut UEnumeration;
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_openKeywords(localeid: super::Foundation::PSTR, status: *mut UErrorCode) -> *mut UEnumeration;
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_setDefault(localeid: super::Foundation::PSTR, status: *mut UErrorCode);
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_setKeywordValue(keywordname: super::Foundation::PSTR, keywordvalue: super::Foundation::PSTR, buffer: super::Foundation::PSTR, buffercapacity: i32, status: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_toLanguageTag(localeid: super::Foundation::PSTR, langtag: super::Foundation::PSTR, langtagcapacity: i32, strict: i8, err: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_toLegacyKey(keyword: super::Foundation::PSTR) -> super::Foundation::PSTR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_toLegacyType(keyword: super::Foundation::PSTR, value: super::Foundation::PSTR) -> super::Foundation::PSTR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_toUnicodeLocaleKey(keyword: super::Foundation::PSTR) -> super::Foundation::PSTR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_toUnicodeLocaleType(keyword: super::Foundation::PSTR, value: super::Foundation::PSTR) -> super::Foundation::PSTR;
    pub fn ulocdata_close(uld: *mut ULocaleData);
    pub fn ulocdata_getCLDRVersion(versionarray: *mut u8, status: *mut UErrorCode);
    pub fn ulocdata_getDelimiter(uld: *mut ULocaleData, r#type: ULocaleDataDelimiterType, result: *mut u16, resultlength: i32, status: *mut UErrorCode) -> i32;
    pub fn ulocdata_getExemplarSet(uld: *mut ULocaleData, fillin: *mut USet, options: u32, extype: ULocaleDataExemplarSetType, status: *mut UErrorCode) -> *mut USet;
    pub fn ulocdata_getLocaleDisplayPattern(uld: *mut ULocaleData, pattern: *mut u16, patterncapacity: i32, status: *mut UErrorCode) -> i32;
    pub fn ulocdata_getLocaleSeparator(uld: *mut ULocaleData, separator: *mut u16, separatorcapacity: i32, status: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ulocdata_getMeasurementSystem(localeid: super::Foundation::PSTR, status: *mut UErrorCode) -> UMeasurementSystem;
    pub fn ulocdata_getNoSubstitute(uld: *mut ULocaleData) -> i8;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ulocdata_getPaperSize(localeid: super::Foundation::PSTR, height: *mut i32, width: *mut i32, status: *mut UErrorCode);
    #[cfg(feature = "Win32_Foundation")]
    pub fn ulocdata_open(localeid: super::Foundation::PSTR, status: *mut UErrorCode) -> *mut ULocaleData;
    pub fn ulocdata_setNoSubstitute(uld: *mut ULocaleData, setting: i8);
    pub fn umsg_applyPattern(fmt: *mut *mut ::core::ffi::c_void, pattern: *const u16, patternlength: i32, parseerror: *mut UParseError, status: *mut UErrorCode);
    pub fn umsg_autoQuoteApostrophe(pattern: *const u16, patternlength: i32, dest: *mut u16, destcapacity: i32, ec: *mut UErrorCode) -> i32;
    pub fn umsg_clone(fmt: *const *const ::core::ffi::c_void, status: *mut UErrorCode) -> *mut ::core::ffi::c_void;
    pub fn umsg_close(format: *mut *mut ::core::ffi::c_void);
    pub fn umsg_format(fmt: *const *const ::core::ffi::c_void, result: *mut u16, resultlength: i32, status: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn umsg_getLocale(fmt: *const *const ::core::ffi::c_void) -> super::Foundation::PSTR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn umsg_open(pattern: *const u16, patternlength: i32, locale: super::Foundation::PSTR, parseerror: *mut UParseError, status: *mut UErrorCode) -> *mut *mut ::core::ffi::c_void;
    pub fn umsg_parse(fmt: *const *const ::core::ffi::c_void, source: *const u16, sourcelength: i32, count: *mut i32, status: *mut UErrorCode);
    #[cfg(feature = "Win32_Foundation")]
    pub fn umsg_setLocale(fmt: *mut *mut ::core::ffi::c_void, locale: super::Foundation::PSTR);
    pub fn umsg_toPattern(fmt: *const *const ::core::ffi::c_void, result: *mut u16, resultlength: i32, status: *mut UErrorCode) -> i32;
    pub fn umsg_vformat(fmt: *const *const ::core::ffi::c_void, result: *mut u16, resultlength: i32, ap: *mut i8, status: *mut UErrorCode) -> i32;
    pub fn umsg_vparse(fmt: *const *const ::core::ffi::c_void, source: *const u16, sourcelength: i32, count: *mut i32, ap: *mut i8, status: *mut UErrorCode);
    pub fn umutablecptrie_buildImmutable(trie: *mut UMutableCPTrie, r#type: UCPTrieType, valuewidth: UCPTrieValueWidth, perrorcode: *mut UErrorCode) -> *mut UCPTrie;
    pub fn umutablecptrie_clone(other: *const UMutableCPTrie, perrorcode: *mut UErrorCode) -> *mut UMutableCPTrie;
    pub fn umutablecptrie_close(trie: *mut UMutableCPTrie);
    pub fn umutablecptrie_fromUCPMap(map: *const UCPMap, perrorcode: *mut UErrorCode) -> *mut UMutableCPTrie;
    pub fn umutablecptrie_fromUCPTrie(trie: *const UCPTrie, perrorcode: *mut UErrorCode) -> *mut UMutableCPTrie;
    pub fn umutablecptrie_get(trie: *const UMutableCPTrie, c: i32) -> u32;
    pub fn umutablecptrie_getRange(trie: *const UMutableCPTrie, start: i32, option: UCPMapRangeOption, surrogatevalue: u32, filter: *mut UCPMapValueFilter, context: *const ::core::ffi::c_void, pvalue: *mut u32) -> i32;
    pub fn umutablecptrie_open(initialvalue: u32, errorvalue: u32, perrorcode: *mut UErrorCode) -> *mut UMutableCPTrie;
    pub fn umutablecptrie_set(trie: *mut UMutableCPTrie, c: i32, value: u32, perrorcode: *mut UErrorCode);
    pub fn umutablecptrie_setRange(trie: *mut UMutableCPTrie, start: i32, end: i32, value: u32, perrorcode: *mut UErrorCode);
    pub fn unorm2_append(norm2: *const UNormalizer2, first: *mut u16, firstlength: i32, firstcapacity: i32, second: *const u16, secondlength: i32, perrorcode: *mut UErrorCode) -> i32;
    pub fn unorm2_close(norm2: *mut UNormalizer2);
    pub fn unorm2_composePair(norm2: *const UNormalizer2, a: i32, b: i32) -> i32;
    pub fn unorm2_getCombiningClass(norm2: *const UNormalizer2, c: i32) -> u8;
    pub fn unorm2_getDecomposition(norm2: *const UNormalizer2, c: i32, decomposition: *mut u16, capacity: i32, perrorcode: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn unorm2_getInstance(packagename: super::Foundation::PSTR, name: super::Foundation::PSTR, mode: UNormalization2Mode, perrorcode: *mut UErrorCode) -> *mut UNormalizer2;
    pub fn unorm2_getNFCInstance(perrorcode: *mut UErrorCode) -> *mut UNormalizer2;
    pub fn unorm2_getNFDInstance(perrorcode: *mut UErrorCode) -> *mut UNormalizer2;
    pub fn unorm2_getNFKCCasefoldInstance(perrorcode: *mut UErrorCode) -> *mut UNormalizer2;
    pub fn unorm2_getNFKCInstance(perrorcode: *mut UErrorCode) -> *mut UNormalizer2;
    pub fn unorm2_getNFKDInstance(perrorcode: *mut UErrorCode) -> *mut UNormalizer2;
    pub fn unorm2_getRawDecomposition(norm2: *const UNormalizer2, c: i32, decomposition: *mut u16, capacity: i32, perrorcode: *mut UErrorCode) -> i32;
    pub fn unorm2_hasBoundaryAfter(norm2: *const UNormalizer2, c: i32) -> i8;
    pub fn unorm2_hasBoundaryBefore(norm2: *const UNormalizer2, c: i32) -> i8;
    pub fn unorm2_isInert(norm2: *const UNormalizer2, c: i32) -> i8;
    pub fn unorm2_isNormalized(norm2: *const UNormalizer2, s: *const u16, length: i32, perrorcode: *mut UErrorCode) -> i8;
    pub fn unorm2_normalize(norm2: *const UNormalizer2, src: *const u16, length: i32, dest: *mut u16, capacity: i32, perrorcode: *mut UErrorCode) -> i32;
    pub fn unorm2_normalizeSecondAndAppend(norm2: *const UNormalizer2, first: *mut u16, firstlength: i32, firstcapacity: i32, second: *const u16, secondlength: i32, perrorcode: *mut UErrorCode) -> i32;
    pub fn unorm2_openFiltered(norm2: *const UNormalizer2, filterset: *const USet, perrorcode: *mut UErrorCode) -> *mut UNormalizer2;
    pub fn unorm2_quickCheck(norm2: *const UNormalizer2, s: *const u16, length: i32, perrorcode: *mut UErrorCode) -> UNormalizationCheckResult;
    pub fn unorm2_spanQuickCheckYes(norm2: *const UNormalizer2, s: *const u16, length: i32, perrorcode: *mut UErrorCode) -> i32;
    pub fn unorm_compare(s1: *const u16, length1: i32, s2: *const u16, length2: i32, options: u32, perrorcode: *mut UErrorCode) -> i32;
    pub fn unum_applyPattern(format: *mut *mut ::core::ffi::c_void, localized: i8, pattern: *const u16, patternlength: i32, parseerror: *mut UParseError, status: *mut UErrorCode);
    pub fn unum_clone(fmt: *const *const ::core::ffi::c_void, status: *mut UErrorCode) -> *mut *mut ::core::ffi::c_void;
    pub fn unum_close(fmt: *mut *mut ::core::ffi::c_void);
    pub fn unum_countAvailable() -> i32;
    pub fn unum_format(fmt: *const *const ::core::ffi::c_void, number: i32, result: *mut u16, resultlength: i32, pos: *mut UFieldPosition, status: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn unum_formatDecimal(fmt: *const *const ::core::ffi::c_void, number: super::Foundation::PSTR, length: i32, result: *mut u16, resultlength: i32, pos: *mut UFieldPosition, status: *mut UErrorCode) -> i32;
    pub fn unum_formatDouble(fmt: *const *const ::core::ffi::c_void, number: f64, result: *mut u16, resultlength: i32, pos: *mut UFieldPosition, status: *mut UErrorCode) -> i32;
    pub fn unum_formatDoubleCurrency(fmt: *const *const ::core::ffi::c_void, number: f64, currency: *mut u16, result: *mut u16, resultlength: i32, pos: *mut UFieldPosition, status: *mut UErrorCode) -> i32;
    pub fn unum_formatDoubleForFields(format: *const *const ::core::ffi::c_void, number: f64, result: *mut u16, resultlength: i32, fpositer: *mut UFieldPositionIterator, status: *mut UErrorCode) -> i32;
    pub fn unum_formatInt64(fmt: *const *const ::core::ffi::c_void, number: i64, result: *mut u16, resultlength: i32, pos: *mut UFieldPosition, status: *mut UErrorCode) -> i32;
    pub fn unum_formatUFormattable(fmt: *const *const ::core::ffi::c_void, number: *const *const ::core::ffi::c_void, result: *mut u16, resultlength: i32, pos: *mut UFieldPosition, status: *mut UErrorCode) -> i32;
    pub fn unum_getAttribute(fmt: *const *const ::core::ffi::c_void, attr: UNumberFormatAttribute) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn unum_getAvailable(localeindex: i32) -> super::Foundation::PSTR;
    pub fn unum_getContext(fmt: *const *const ::core::ffi::c_void, r#type: UDisplayContextType, status: *mut UErrorCode) -> UDisplayContext;
    pub fn unum_getDoubleAttribute(fmt: *const *const ::core::ffi::c_void, attr: UNumberFormatAttribute) -> f64;
    #[cfg(feature = "Win32_Foundation")]
    pub fn unum_getLocaleByType(fmt: *const *const ::core::ffi::c_void, r#type: ULocDataLocaleType, status: *mut UErrorCode) -> super::Foundation::PSTR;
    pub fn unum_getSymbol(fmt: *const *const ::core::ffi::c_void, symbol: UNumberFormatSymbol, buffer: *mut u16, size: i32, status: *mut UErrorCode) -> i32;
    pub fn unum_getTextAttribute(fmt: *const *const ::core::ffi::c_void, tag: UNumberFormatTextAttribute, result: *mut u16, resultlength: i32, status: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn unum_open(style: UNumberFormatStyle, pattern: *const u16, patternlength: i32, locale: super::Foundation::PSTR, parseerr: *mut UParseError, status: *mut UErrorCode) -> *mut *mut ::core::ffi::c_void;
    pub fn unum_parse(fmt: *const *const ::core::ffi::c_void, text: *const u16, textlength: i32, parsepos: *mut i32, status: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn unum_parseDecimal(fmt: *const *const ::core::ffi::c_void, text: *const u16, textlength: i32, parsepos: *mut i32, outbuf: super::Foundation::PSTR, outbuflength: i32, status: *mut UErrorCode) -> i32;
    pub fn unum_parseDouble(fmt: *const *const ::core::ffi::c_void, text: *const u16, textlength: i32, parsepos: *mut i32, status: *mut UErrorCode) -> f64;
    pub fn unum_parseDoubleCurrency(fmt: *const *const ::core::ffi::c_void, text: *const u16, textlength: i32, parsepos: *mut i32, currency: *mut u16, status: *mut UErrorCode) -> f64;
    pub fn unum_parseInt64(fmt: *const *const ::core::ffi::c_void, text: *const u16, textlength: i32, parsepos: *mut i32, status: *mut UErrorCode) -> i64;
    pub fn unum_parseToUFormattable(fmt: *const *const ::core::ffi::c_void, result: *mut *mut ::core::ffi::c_void, text: *const u16, textlength: i32, parsepos: *mut i32, status: *mut UErrorCode) -> *mut *mut ::core::ffi::c_void;
    pub fn unum_setAttribute(fmt: *mut *mut ::core::ffi::c_void, attr: UNumberFormatAttribute, newvalue: i32);
    pub fn unum_setContext(fmt: *mut *mut ::core::ffi::c_void, value: UDisplayContext, status: *mut UErrorCode);
    pub fn unum_setDoubleAttribute(fmt: *mut *mut ::core::ffi::c_void, attr: UNumberFormatAttribute, newvalue: f64);
    pub fn unum_setSymbol(fmt: *mut *mut ::core::ffi::c_void, symbol: UNumberFormatSymbol, value: *const u16, length: i32, status: *mut UErrorCode);
    pub fn unum_setTextAttribute(fmt: *mut *mut ::core::ffi::c_void, tag: UNumberFormatTextAttribute, newvalue: *const u16, newvaluelength: i32, status: *mut UErrorCode);
    pub fn unum_toPattern(fmt: *const *const ::core::ffi::c_void, ispatternlocalized: i8, result: *mut u16, resultlength: i32, status: *mut UErrorCode) -> i32;
    pub fn unumf_close(uformatter: *mut UNumberFormatter);
    pub fn unumf_closeResult(uresult: *mut UFormattedNumber);
    #[cfg(feature = "Win32_Foundation")]
    pub fn unumf_formatDecimal(uformatter: *const UNumberFormatter, value: super::Foundation::PSTR, valuelen: i32, uresult: *mut UFormattedNumber, ec: *mut UErrorCode);
    pub fn unumf_formatDouble(uformatter: *const UNumberFormatter, value: f64, uresult: *mut UFormattedNumber, ec: *mut UErrorCode);
    pub fn unumf_formatInt(uformatter: *const UNumberFormatter, value: i64, uresult: *mut UFormattedNumber, ec: *mut UErrorCode);
    #[cfg(feature = "Win32_Foundation")]
    pub fn unumf_openForSkeletonAndLocale(skeleton: *const u16, skeletonlen: i32, locale: super::Foundation::PSTR, ec: *mut UErrorCode) -> *mut UNumberFormatter;
    #[cfg(feature = "Win32_Foundation")]
    pub fn unumf_openForSkeletonAndLocaleWithError(skeleton: *const u16, skeletonlen: i32, locale: super::Foundation::PSTR, perror: *mut UParseError, ec: *mut UErrorCode) -> *mut UNumberFormatter;
    pub fn unumf_openResult(ec: *mut UErrorCode) -> *mut UFormattedNumber;
    pub fn unumf_resultAsValue(uresult: *const UFormattedNumber, ec: *mut UErrorCode) -> *mut UFormattedValue;
    pub fn unumf_resultGetAllFieldPositions(uresult: *const UFormattedNumber, ufpositer: *mut UFieldPositionIterator, ec: *mut UErrorCode);
    pub fn unumf_resultNextFieldPosition(uresult: *const UFormattedNumber, ufpos: *mut UFieldPosition, ec: *mut UErrorCode) -> i8;
    pub fn unumf_resultToString(uresult: *const UFormattedNumber, buffer: *mut u16, buffercapacity: i32, ec: *mut UErrorCode) -> i32;
    pub fn unumsys_close(unumsys: *mut UNumberingSystem);
    pub fn unumsys_getDescription(unumsys: *const UNumberingSystem, result: *mut u16, resultlength: i32, status: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn unumsys_getName(unumsys: *const UNumberingSystem) -> super::Foundation::PSTR;
    pub fn unumsys_getRadix(unumsys: *const UNumberingSystem) -> i32;
    pub fn unumsys_isAlgorithmic(unumsys: *const UNumberingSystem) -> i8;
    #[cfg(feature = "Win32_Foundation")]
    pub fn unumsys_open(locale: super::Foundation::PSTR, status: *mut UErrorCode) -> *mut UNumberingSystem;
    pub fn unumsys_openAvailableNames(status: *mut UErrorCode) -> *mut UEnumeration;
    #[cfg(feature = "Win32_Foundation")]
    pub fn unumsys_openByName(name: super::Foundation::PSTR, status: *mut UErrorCode) -> *mut UNumberingSystem;
    pub fn uplrules_close(uplrules: *mut UPluralRules);
    pub fn uplrules_getKeywords(uplrules: *const UPluralRules, status: *mut UErrorCode) -> *mut UEnumeration;
    #[cfg(feature = "Win32_Foundation")]
    pub fn uplrules_open(locale: super::Foundation::PSTR, status: *mut UErrorCode) -> *mut UPluralRules;
    #[cfg(feature = "Win32_Foundation")]
    pub fn uplrules_openForType(locale: super::Foundation::PSTR, r#type: UPluralType, status: *mut UErrorCode) -> *mut UPluralRules;
    pub fn uplrules_select(uplrules: *const UPluralRules, number: f64, keyword: *mut u16, capacity: i32, status: *mut UErrorCode) -> i32;
    pub fn uplrules_selectFormatted(uplrules: *const UPluralRules, number: *const UFormattedNumber, keyword: *mut u16, capacity: i32, status: *mut UErrorCode) -> i32;
    pub fn uregex_appendReplacement(regexp: *mut URegularExpression, replacementtext: *const u16, replacementlength: i32, destbuf: *mut *mut u16, destcapacity: *mut i32, status: *mut UErrorCode) -> i32;
    pub fn uregex_appendReplacementUText(regexp: *mut URegularExpression, replacementtext: *mut UText, dest: *mut UText, status: *mut UErrorCode);
    pub fn uregex_appendTail(regexp: *mut URegularExpression, destbuf: *mut *mut u16, destcapacity: *mut i32, status: *mut UErrorCode) -> i32;
    pub fn uregex_appendTailUText(regexp: *mut URegularExpression, dest: *mut UText, status: *mut UErrorCode) -> *mut UText;
    pub fn uregex_clone(regexp: *const URegularExpression, status: *mut UErrorCode) -> *mut URegularExpression;
    pub fn uregex_close(regexp: *mut URegularExpression);
    pub fn uregex_end(regexp: *mut URegularExpression, groupnum: i32, status: *mut UErrorCode) -> i32;
    pub fn uregex_end64(regexp: *mut URegularExpression, groupnum: i32, status: *mut UErrorCode) -> i64;
    pub fn uregex_find(regexp: *mut URegularExpression, startindex: i32, status: *mut UErrorCode) -> i8;
    pub fn uregex_find64(regexp: *mut URegularExpression, startindex: i64, status: *mut UErrorCode) -> i8;
    pub fn uregex_findNext(regexp: *mut URegularExpression, status: *mut UErrorCode) -> i8;
    pub fn uregex_flags(regexp: *const URegularExpression, status: *mut UErrorCode) -> i32;
    pub fn uregex_getFindProgressCallback(regexp: *const URegularExpression, callback: *mut URegexFindProgressCallback, context: *const *const ::core::ffi::c_void, status: *mut UErrorCode);
    pub fn uregex_getMatchCallback(regexp: *const URegularExpression, callback: *mut URegexMatchCallback, context: *const *const ::core::ffi::c_void, status: *mut UErrorCode);
    pub fn uregex_getStackLimit(regexp: *const URegularExpression, status: *mut UErrorCode) -> i32;
    pub fn uregex_getText(regexp: *mut URegularExpression, textlength: *mut i32, status: *mut UErrorCode) -> *mut u16;
    pub fn uregex_getTimeLimit(regexp: *const URegularExpression, status: *mut UErrorCode) -> i32;
    pub fn uregex_getUText(regexp: *mut URegularExpression, dest: *mut UText, status: *mut UErrorCode) -> *mut UText;
    pub fn uregex_group(regexp: *mut URegularExpression, groupnum: i32, dest: *mut u16, destcapacity: i32, status: *mut UErrorCode) -> i32;
    pub fn uregex_groupCount(regexp: *mut URegularExpression, status: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn uregex_groupNumberFromCName(regexp: *mut URegularExpression, groupname: super::Foundation::PSTR, namelength: i32, status: *mut UErrorCode) -> i32;
    pub fn uregex_groupNumberFromName(regexp: *mut URegularExpression, groupname: *const u16, namelength: i32, status: *mut UErrorCode) -> i32;
    pub fn uregex_groupUText(regexp: *mut URegularExpression, groupnum: i32, dest: *mut UText, grouplength: *mut i64, status: *mut UErrorCode) -> *mut UText;
    pub fn uregex_hasAnchoringBounds(regexp: *const URegularExpression, status: *mut UErrorCode) -> i8;
    pub fn uregex_hasTransparentBounds(regexp: *const URegularExpression, status: *mut UErrorCode) -> i8;
    pub fn uregex_hitEnd(regexp: *const URegularExpression, status: *mut UErrorCode) -> i8;
    pub fn uregex_lookingAt(regexp: *mut URegularExpression, startindex: i32, status: *mut UErrorCode) -> i8;
    pub fn uregex_lookingAt64(regexp: *mut URegularExpression, startindex: i64, status: *mut UErrorCode) -> i8;
    pub fn uregex_matches(regexp: *mut URegularExpression, startindex: i32, status: *mut UErrorCode) -> i8;
    pub fn uregex_matches64(regexp: *mut URegularExpression, startindex: i64, status: *mut UErrorCode) -> i8;
    pub fn uregex_open(pattern: *const u16, patternlength: i32, flags: u32, pe: *mut UParseError, status: *mut UErrorCode) -> *mut URegularExpression;
    #[cfg(feature = "Win32_Foundation")]
    pub fn uregex_openC(pattern: super::Foundation::PSTR, flags: u32, pe: *mut UParseError, status: *mut UErrorCode) -> *mut URegularExpression;
    pub fn uregex_openUText(pattern: *mut UText, flags: u32, pe: *mut UParseError, status: *mut UErrorCode) -> *mut URegularExpression;
    pub fn uregex_pattern(regexp: *const URegularExpression, patlength: *mut i32, status: *mut UErrorCode) -> *mut u16;
    pub fn uregex_patternUText(regexp: *const URegularExpression, status: *mut UErrorCode) -> *mut UText;
    pub fn uregex_refreshUText(regexp: *mut URegularExpression, text: *mut UText, status: *mut UErrorCode);
    pub fn uregex_regionEnd(regexp: *const URegularExpression, status: *mut UErrorCode) -> i32;
    pub fn uregex_regionEnd64(regexp: *const URegularExpression, status: *mut UErrorCode) -> i64;
    pub fn uregex_regionStart(regexp: *const URegularExpression, status: *mut UErrorCode) -> i32;
    pub fn uregex_regionStart64(regexp: *const URegularExpression, status: *mut UErrorCode) -> i64;
    pub fn uregex_replaceAll(regexp: *mut URegularExpression, replacementtext: *const u16, replacementlength: i32, destbuf: *mut u16, destcapacity: i32, status: *mut UErrorCode) -> i32;
    pub fn uregex_replaceAllUText(regexp: *mut URegularExpression, replacement: *mut UText, dest: *mut UText, status: *mut UErrorCode) -> *mut UText;
    pub fn uregex_replaceFirst(regexp: *mut URegularExpression, replacementtext: *const u16, replacementlength: i32, destbuf: *mut u16, destcapacity: i32, status: *mut UErrorCode) -> i32;
    pub fn uregex_replaceFirstUText(regexp: *mut URegularExpression, replacement: *mut UText, dest: *mut UText, status: *mut UErrorCode) -> *mut UText;
    pub fn uregex_requireEnd(regexp: *const URegularExpression, status: *mut UErrorCode) -> i8;
    pub fn uregex_reset(regexp: *mut URegularExpression, index: i32, status: *mut UErrorCode);
    pub fn uregex_reset64(regexp: *mut URegularExpression, index: i64, status: *mut UErrorCode);
    pub fn uregex_setFindProgressCallback(regexp: *mut URegularExpression, callback: URegexFindProgressCallback, context: *const ::core::ffi::c_void, status: *mut UErrorCode);
    pub fn uregex_setMatchCallback(regexp: *mut URegularExpression, callback: URegexMatchCallback, context: *const ::core::ffi::c_void, status: *mut UErrorCode);
    pub fn uregex_setRegion(regexp: *mut URegularExpression, regionstart: i32, regionlimit: i32, status: *mut UErrorCode);
    pub fn uregex_setRegion64(regexp: *mut URegularExpression, regionstart: i64, regionlimit: i64, status: *mut UErrorCode);
    pub fn uregex_setRegionAndStart(regexp: *mut URegularExpression, regionstart: i64, regionlimit: i64, startindex: i64, status: *mut UErrorCode);
    pub fn uregex_setStackLimit(regexp: *mut URegularExpression, limit: i32, status: *mut UErrorCode);
    pub fn uregex_setText(regexp: *mut URegularExpression, text: *const u16, textlength: i32, status: *mut UErrorCode);
    pub fn uregex_setTimeLimit(regexp: *mut URegularExpression, limit: i32, status: *mut UErrorCode);
    pub fn uregex_setUText(regexp: *mut URegularExpression, text: *mut UText, status: *mut UErrorCode);
    pub fn uregex_split(regexp: *mut URegularExpression, destbuf: *mut u16, destcapacity: i32, requiredcapacity: *mut i32, destfields: *mut *mut u16, destfieldscapacity: i32, status: *mut UErrorCode) -> i32;
    pub fn uregex_splitUText(regexp: *mut URegularExpression, destfields: *mut *mut UText, destfieldscapacity: i32, status: *mut UErrorCode) -> i32;
    pub fn uregex_start(regexp: *mut URegularExpression, groupnum: i32, status: *mut UErrorCode) -> i32;
    pub fn uregex_start64(regexp: *mut URegularExpression, groupnum: i32, status: *mut UErrorCode) -> i64;
    pub fn uregex_useAnchoringBounds(regexp: *mut URegularExpression, b: i8, status: *mut UErrorCode);
    pub fn uregex_useTransparentBounds(regexp: *mut URegularExpression, b: i8, status: *mut UErrorCode);
    pub fn uregion_areEqual(uregion: *const URegion, otherregion: *const URegion) -> i8;
    pub fn uregion_contains(uregion: *const URegion, otherregion: *const URegion) -> i8;
    pub fn uregion_getAvailable(r#type: URegionType, status: *mut UErrorCode) -> *mut UEnumeration;
    pub fn uregion_getContainedRegions(uregion: *const URegion, status: *mut UErrorCode) -> *mut UEnumeration;
    pub fn uregion_getContainedRegionsOfType(uregion: *const URegion, r#type: URegionType, status: *mut UErrorCode) -> *mut UEnumeration;
    pub fn uregion_getContainingRegion(uregion: *const URegion) -> *mut URegion;
    pub fn uregion_getContainingRegionOfType(uregion: *const URegion, r#type: URegionType) -> *mut URegion;
    pub fn uregion_getNumericCode(uregion: *const URegion) -> i32;
    pub fn uregion_getPreferredValues(uregion: *const URegion, status: *mut UErrorCode) -> *mut UEnumeration;
    #[cfg(feature = "Win32_Foundation")]
    pub fn uregion_getRegionCode(uregion: *const URegion) -> super::Foundation::PSTR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn uregion_getRegionFromCode(regioncode: super::Foundation::PSTR, status: *mut UErrorCode) -> *mut URegion;
    pub fn uregion_getRegionFromNumericCode(code: i32, status: *mut UErrorCode) -> *mut URegion;
    pub fn uregion_getType(uregion: *const URegion) -> URegionType;
    pub fn ureldatefmt_close(reldatefmt: *mut URelativeDateTimeFormatter);
    pub fn ureldatefmt_closeResult(ufrdt: *mut UFormattedRelativeDateTime);
    pub fn ureldatefmt_combineDateAndTime(reldatefmt: *const URelativeDateTimeFormatter, relativedatestring: *const u16, relativedatestringlen: i32, timestring: *const u16, timestringlen: i32, result: *mut u16, resultcapacity: i32, status: *mut UErrorCode) -> i32;
    pub fn ureldatefmt_format(reldatefmt: *const URelativeDateTimeFormatter, offset: f64, unit: URelativeDateTimeUnit, result: *mut u16, resultcapacity: i32, status: *mut UErrorCode) -> i32;
    pub fn ureldatefmt_formatNumeric(reldatefmt: *const URelativeDateTimeFormatter, offset: f64, unit: URelativeDateTimeUnit, result: *mut u16, resultcapacity: i32, status: *mut UErrorCode) -> i32;
    pub fn ureldatefmt_formatNumericToResult(reldatefmt: *const URelativeDateTimeFormatter, offset: f64, unit: URelativeDateTimeUnit, result: *mut UFormattedRelativeDateTime, status: *mut UErrorCode);
    pub fn ureldatefmt_formatToResult(reldatefmt: *const URelativeDateTimeFormatter, offset: f64, unit: URelativeDateTimeUnit, result: *mut UFormattedRelativeDateTime, status: *mut UErrorCode);
    #[cfg(feature = "Win32_Foundation")]
    pub fn ureldatefmt_open(locale: super::Foundation::PSTR, nftoadopt: *mut *mut ::core::ffi::c_void, width: UDateRelativeDateTimeFormatterStyle, capitalizationcontext: UDisplayContext, status: *mut UErrorCode) -> *mut URelativeDateTimeFormatter;
    pub fn ureldatefmt_openResult(ec: *mut UErrorCode) -> *mut UFormattedRelativeDateTime;
    pub fn ureldatefmt_resultAsValue(ufrdt: *const UFormattedRelativeDateTime, ec: *mut UErrorCode) -> *mut UFormattedValue;
    pub fn ures_close(resourcebundle: *mut UResourceBundle);
    pub fn ures_getBinary(resourcebundle: *const UResourceBundle, len: *mut i32, status: *mut UErrorCode) -> *mut u8;
    pub fn ures_getByIndex(resourcebundle: *const UResourceBundle, indexr: i32, fillin: *mut UResourceBundle, status: *mut UErrorCode) -> *mut UResourceBundle;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ures_getByKey(resourcebundle: *const UResourceBundle, key: super::Foundation::PSTR, fillin: *mut UResourceBundle, status: *mut UErrorCode) -> *mut UResourceBundle;
    pub fn ures_getInt(resourcebundle: *const UResourceBundle, status: *mut UErrorCode) -> i32;
    pub fn ures_getIntVector(resourcebundle: *const UResourceBundle, len: *mut i32, status: *mut UErrorCode) -> *mut i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ures_getKey(resourcebundle: *const UResourceBundle) -> super::Foundation::PSTR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ures_getLocaleByType(resourcebundle: *const UResourceBundle, r#type: ULocDataLocaleType, status: *mut UErrorCode) -> super::Foundation::PSTR;
    pub fn ures_getNextResource(resourcebundle: *mut UResourceBundle, fillin: *mut UResourceBundle, status: *mut UErrorCode) -> *mut UResourceBundle;
    pub fn ures_getNextString(resourcebundle: *mut UResourceBundle, len: *mut i32, key: *const *const i8, status: *mut UErrorCode) -> *mut u16;
    pub fn ures_getSize(resourcebundle: *const UResourceBundle) -> i32;
    pub fn ures_getString(resourcebundle: *const UResourceBundle, len: *mut i32, status: *mut UErrorCode) -> *mut u16;
    pub fn ures_getStringByIndex(resourcebundle: *const UResourceBundle, indexs: i32, len: *mut i32, status: *mut UErrorCode) -> *mut u16;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ures_getStringByKey(resb: *const UResourceBundle, key: super::Foundation::PSTR, len: *mut i32, status: *mut UErrorCode) -> *mut u16;
    pub fn ures_getType(resourcebundle: *const UResourceBundle) -> UResType;
    pub fn ures_getUInt(resourcebundle: *const UResourceBundle, status: *mut UErrorCode) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ures_getUTF8String(resb: *const UResourceBundle, dest: super::Foundation::PSTR, length: *mut i32, forcecopy: i8, status: *mut UErrorCode) -> super::Foundation::PSTR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ures_getUTF8StringByIndex(resb: *const UResourceBundle, stringindex: i32, dest: super::Foundation::PSTR, plength: *mut i32, forcecopy: i8, status: *mut UErrorCode) -> super::Foundation::PSTR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ures_getUTF8StringByKey(resb: *const UResourceBundle, key: super::Foundation::PSTR, dest: super::Foundation::PSTR, plength: *mut i32, forcecopy: i8, status: *mut UErrorCode) -> super::Foundation::PSTR;
    pub fn ures_getVersion(resb: *const UResourceBundle, versioninfo: *mut u8);
    pub fn ures_hasNext(resourcebundle: *const UResourceBundle) -> i8;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ures_open(packagename: super::Foundation::PSTR, locale: super::Foundation::PSTR, status: *mut UErrorCode) -> *mut UResourceBundle;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ures_openAvailableLocales(packagename: super::Foundation::PSTR, status: *mut UErrorCode) -> *mut UEnumeration;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ures_openDirect(packagename: super::Foundation::PSTR, locale: super::Foundation::PSTR, status: *mut UErrorCode) -> *mut UResourceBundle;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ures_openU(packagename: *const u16, locale: super::Foundation::PSTR, status: *mut UErrorCode) -> *mut UResourceBundle;
    pub fn ures_resetIterator(resourcebundle: *mut UResourceBundle);
    pub fn uscript_breaksBetweenLetters(script: UScriptCode) -> i8;
    #[cfg(feature = "Win32_Foundation")]
    pub fn uscript_getCode(nameorabbrorlocale: super::Foundation::PSTR, fillin: *mut UScriptCode, capacity: i32, err: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn uscript_getName(scriptcode: UScriptCode) -> super::Foundation::PSTR;
    pub fn uscript_getSampleString(script: UScriptCode, dest: *mut u16, capacity: i32, perrorcode: *mut UErrorCode) -> i32;
    pub fn uscript_getScript(codepoint: i32, err: *mut UErrorCode) -> UScriptCode;
    pub fn uscript_getScriptExtensions(c: i32, scripts: *mut UScriptCode, capacity: i32, errorcode: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn uscript_getShortName(scriptcode: UScriptCode) -> super::Foundation::PSTR;
    pub fn uscript_getUsage(script: UScriptCode) -> UScriptUsage;
    pub fn uscript_hasScript(c: i32, sc: UScriptCode) -> i8;
    pub fn uscript_isCased(script: UScriptCode) -> i8;
    pub fn uscript_isRightToLeft(script: UScriptCode) -> i8;
    pub fn usearch_close(searchiter: *mut UStringSearch);
    pub fn usearch_first(strsrch: *mut UStringSearch, status: *mut UErrorCode) -> i32;
    pub fn usearch_following(strsrch: *mut UStringSearch, position: i32, status: *mut UErrorCode) -> i32;
    pub fn usearch_getAttribute(strsrch: *const UStringSearch, attribute: USearchAttribute) -> USearchAttributeValue;
    pub fn usearch_getBreakIterator(strsrch: *const UStringSearch) -> *mut UBreakIterator;
    pub fn usearch_getCollator(strsrch: *const UStringSearch) -> *mut UCollator;
    pub fn usearch_getMatchedLength(strsrch: *const UStringSearch) -> i32;
    pub fn usearch_getMatchedStart(strsrch: *const UStringSearch) -> i32;
    pub fn usearch_getMatchedText(strsrch: *const UStringSearch, result: *mut u16, resultcapacity: i32, status: *mut UErrorCode) -> i32;
    pub fn usearch_getOffset(strsrch: *const UStringSearch) -> i32;
    pub fn usearch_getPattern(strsrch: *const UStringSearch, length: *mut i32) -> *mut u16;
    pub fn usearch_getText(strsrch: *const UStringSearch, length: *mut i32) -> *mut u16;
    pub fn usearch_last(strsrch: *mut UStringSearch, status: *mut UErrorCode) -> i32;
    pub fn usearch_next(strsrch: *mut UStringSearch, status: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn usearch_open(pattern: *const u16, patternlength: i32, text: *const u16, textlength: i32, locale: super::Foundation::PSTR, breakiter: *mut UBreakIterator, status: *mut UErrorCode) -> *mut UStringSearch;
    pub fn usearch_openFromCollator(pattern: *const u16, patternlength: i32, text: *const u16, textlength: i32, collator: *const UCollator, breakiter: *mut UBreakIterator, status: *mut UErrorCode) -> *mut UStringSearch;
    pub fn usearch_preceding(strsrch: *mut UStringSearch, position: i32, status: *mut UErrorCode) -> i32;
    pub fn usearch_previous(strsrch: *mut UStringSearch, status: *mut UErrorCode) -> i32;
    pub fn usearch_reset(strsrch: *mut UStringSearch);
    pub fn usearch_setAttribute(strsrch: *mut UStringSearch, attribute: USearchAttribute, value: USearchAttributeValue, status: *mut UErrorCode);
    pub fn usearch_setBreakIterator(strsrch: *mut UStringSearch, breakiter: *mut UBreakIterator, status: *mut UErrorCode);
    pub fn usearch_setCollator(strsrch: *mut UStringSearch, collator: *const UCollator, status: *mut UErrorCode);
    pub fn usearch_setOffset(strsrch: *mut UStringSearch, position: i32, status: *mut UErrorCode);
    pub fn usearch_setPattern(strsrch: *mut UStringSearch, pattern: *const u16, patternlength: i32, status: *mut UErrorCode);
    pub fn usearch_setText(strsrch: *mut UStringSearch, text: *const u16, textlength: i32, status: *mut UErrorCode);
    pub fn uset_add(set: *mut USet, c: i32);
    pub fn uset_addAll(set: *mut USet, additionalset: *const USet);
    pub fn uset_addAllCodePoints(set: *mut USet, str: *const u16, strlen: i32);
    pub fn uset_addRange(set: *mut USet, start: i32, end: i32);
    pub fn uset_addString(set: *mut USet, str: *const u16, strlen: i32);
    pub fn uset_applyIntPropertyValue(set: *mut USet, prop: UProperty, value: i32, ec: *mut UErrorCode);
    pub fn uset_applyPattern(set: *mut USet, pattern: *const u16, patternlength: i32, options: u32, status: *mut UErrorCode) -> i32;
    pub fn uset_applyPropertyAlias(set: *mut USet, prop: *const u16, proplength: i32, value: *const u16, valuelength: i32, ec: *mut UErrorCode);
    pub fn uset_charAt(set: *const USet, charindex: i32) -> i32;
    pub fn uset_clear(set: *mut USet);
    pub fn uset_clone(set: *const USet) -> *mut USet;
    pub fn uset_cloneAsThawed(set: *const USet) -> *mut USet;
    pub fn uset_close(set: *mut USet);
    pub fn uset_closeOver(set: *mut USet, attributes: i32);
    pub fn uset_compact(set: *mut USet);
    pub fn uset_complement(set: *mut USet);
    pub fn uset_complementAll(set: *mut USet, complement: *const USet);
    pub fn uset_contains(set: *const USet, c: i32) -> i8;
    pub fn uset_containsAll(set1: *const USet, set2: *const USet) -> i8;
    pub fn uset_containsAllCodePoints(set: *const USet, str: *const u16, strlen: i32) -> i8;
    pub fn uset_containsNone(set1: *const USet, set2: *const USet) -> i8;
    pub fn uset_containsRange(set: *const USet, start: i32, end: i32) -> i8;
    pub fn uset_containsSome(set1: *const USet, set2: *const USet) -> i8;
    pub fn uset_containsString(set: *const USet, str: *const u16, strlen: i32) -> i8;
    pub fn uset_equals(set1: *const USet, set2: *const USet) -> i8;
    pub fn uset_freeze(set: *mut USet);
    pub fn uset_getItem(set: *const USet, itemindex: i32, start: *mut i32, end: *mut i32, str: *mut u16, strcapacity: i32, ec: *mut UErrorCode) -> i32;
    pub fn uset_getItemCount(set: *const USet) -> i32;
    pub fn uset_getSerializedRange(set: *const USerializedSet, rangeindex: i32, pstart: *mut i32, pend: *mut i32) -> i8;
    pub fn uset_getSerializedRangeCount(set: *const USerializedSet) -> i32;
    pub fn uset_getSerializedSet(fillset: *mut USerializedSet, src: *const u16, srclength: i32) -> i8;
    pub fn uset_indexOf(set: *const USet, c: i32) -> i32;
    pub fn uset_isEmpty(set: *const USet) -> i8;
    pub fn uset_isFrozen(set: *const USet) -> i8;
    pub fn uset_open(start: i32, end: i32) -> *mut USet;
    pub fn uset_openEmpty() -> *mut USet;
    pub fn uset_openPattern(pattern: *const u16, patternlength: i32, ec: *mut UErrorCode) -> *mut USet;
    pub fn uset_openPatternOptions(pattern: *const u16, patternlength: i32, options: u32, ec: *mut UErrorCode) -> *mut USet;
    pub fn uset_remove(set: *mut USet, c: i32);
    pub fn uset_removeAll(set: *mut USet, removeset: *const USet);
    pub fn uset_removeAllStrings(set: *mut USet);
    pub fn uset_removeRange(set: *mut USet, start: i32, end: i32);
    pub fn uset_removeString(set: *mut USet, str: *const u16, strlen: i32);
    pub fn uset_resemblesPattern(pattern: *const u16, patternlength: i32, pos: i32) -> i8;
    pub fn uset_retain(set: *mut USet, start: i32, end: i32);
    pub fn uset_retainAll(set: *mut USet, retain: *const USet);
    pub fn uset_serialize(set: *const USet, dest: *mut u16, destcapacity: i32, perrorcode: *mut UErrorCode) -> i32;
    pub fn uset_serializedContains(set: *const USerializedSet, c: i32) -> i8;
    pub fn uset_set(set: *mut USet, start: i32, end: i32);
    pub fn uset_setSerializedToOne(fillset: *mut USerializedSet, c: i32);
    pub fn uset_size(set: *const USet) -> i32;
    pub fn uset_span(set: *const USet, s: *const u16, length: i32, spancondition: USetSpanCondition) -> i32;
    pub fn uset_spanBack(set: *const USet, s: *const u16, length: i32, spancondition: USetSpanCondition) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn uset_spanBackUTF8(set: *const USet, s: super::Foundation::PSTR, length: i32, spancondition: USetSpanCondition) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn uset_spanUTF8(set: *const USet, s: super::Foundation::PSTR, length: i32, spancondition: USetSpanCondition) -> i32;
    pub fn uset_toPattern(set: *const USet, result: *mut u16, resultcapacity: i32, escapeunprintable: i8, ec: *mut UErrorCode) -> i32;
    pub fn uspoof_areConfusable(sc: *const USpoofChecker, id1: *const u16, length1: i32, id2: *const u16, length2: i32, status: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn uspoof_areConfusableUTF8(sc: *const USpoofChecker, id1: super::Foundation::PSTR, length1: i32, id2: super::Foundation::PSTR, length2: i32, status: *mut UErrorCode) -> i32;
    pub fn uspoof_check(sc: *const USpoofChecker, id: *const u16, length: i32, position: *mut i32, status: *mut UErrorCode) -> i32;
    pub fn uspoof_check2(sc: *const USpoofChecker, id: *const u16, length: i32, checkresult: *mut USpoofCheckResult, status: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn uspoof_check2UTF8(sc: *const USpoofChecker, id: super::Foundation::PSTR, length: i32, checkresult: *mut USpoofCheckResult, status: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn uspoof_checkUTF8(sc: *const USpoofChecker, id: super::Foundation::PSTR, length: i32, position: *mut i32, status: *mut UErrorCode) -> i32;
    pub fn uspoof_clone(sc: *const USpoofChecker, status: *mut UErrorCode) -> *mut USpoofChecker;
    pub fn uspoof_close(sc: *mut USpoofChecker);
    pub fn uspoof_closeCheckResult(checkresult: *mut USpoofCheckResult);
    pub fn uspoof_getAllowedChars(sc: *const USpoofChecker, status: *mut UErrorCode) -> *mut USet;
    #[cfg(feature = "Win32_Foundation")]
    pub fn uspoof_getAllowedLocales(sc: *mut USpoofChecker, status: *mut UErrorCode) -> super::Foundation::PSTR;
    pub fn uspoof_getCheckResultChecks(checkresult: *const USpoofCheckResult, status: *mut UErrorCode) -> i32;
    pub fn uspoof_getCheckResultNumerics(checkresult: *const USpoofCheckResult, status: *mut UErrorCode) -> *mut USet;
    pub fn uspoof_getCheckResultRestrictionLevel(checkresult: *const USpoofCheckResult, status: *mut UErrorCode) -> URestrictionLevel;
    pub fn uspoof_getChecks(sc: *const USpoofChecker, status: *mut UErrorCode) -> i32;
    pub fn uspoof_getInclusionSet(status: *mut UErrorCode) -> *mut USet;
    pub fn uspoof_getRecommendedSet(status: *mut UErrorCode) -> *mut USet;
    pub fn uspoof_getRestrictionLevel(sc: *const USpoofChecker) -> URestrictionLevel;
    pub fn uspoof_getSkeleton(sc: *const USpoofChecker, r#type: u32, id: *const u16, length: i32, dest: *mut u16, destcapacity: i32, status: *mut UErrorCode) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn uspoof_getSkeletonUTF8(sc: *const USpoofChecker, r#type: u32, id: super::Foundation::PSTR, length: i32, dest: super::Foundation::PSTR, destcapacity: i32, status: *mut UErrorCode) -> i32;
    pub fn uspoof_open(status: *mut UErrorCode) -> *mut USpoofChecker;
    pub fn uspoof_openCheckResult(status: *mut UErrorCode) -> *mut USpoofCheckResult;
    pub fn uspoof_openFromSerialized(data: *const ::core::ffi::c_void, length: i32, pactuallength: *mut i32, perrorcode: *mut UErrorCode) -> *mut USpoofChecker;
    #[cfg(feature = "Win32_Foundation")]
    pub fn uspoof_openFromSource(confusables: super::Foundation::PSTR, confusableslen: i32, confusableswholescript: super::Foundation::PSTR, confusableswholescriptlen: i32, errtype: *mut i32, pe: *mut UParseError, status: *mut UErrorCode) -> *mut USpoofChecker;
    pub fn uspoof_serialize(sc: *mut USpoofChecker, data: *mut ::core::ffi::c_void, capacity: i32, status: *mut UErrorCode) -> i32;
    pub fn uspoof_setAllowedChars(sc: *mut USpoofChecker, chars: *const USet, status: *mut UErrorCode);
    #[cfg(feature = "Win32_Foundation")]
    pub fn uspoof_setAllowedLocales(sc: *mut USpoofChecker, localeslist: super::Foundation::PSTR, status: *mut UErrorCode);
    pub fn uspoof_setChecks(sc: *mut USpoofChecker, checks: i32, status: *mut UErrorCode);
    pub fn uspoof_setRestrictionLevel(sc: *mut USpoofChecker, restrictionlevel: URestrictionLevel);
    pub fn usprep_close(profile: *mut UStringPrepProfile);
    #[cfg(feature = "Win32_Foundation")]
    pub fn usprep_open(path: super::Foundation::PSTR, filename: super::Foundation::PSTR, status: *mut UErrorCode) -> *mut UStringPrepProfile;
    pub fn usprep_openByType(r#type: UStringPrepProfileType, status: *mut UErrorCode) -> *mut UStringPrepProfile;
    pub fn usprep_prepare(prep: *const UStringPrepProfile, src: *const u16, srclength: i32, dest: *mut u16, destcapacity: i32, options: i32, parseerror: *mut UParseError, status: *mut UErrorCode) -> i32;
    pub fn utext_char32At(ut: *mut UText, nativeindex: i64) -> i32;
    pub fn utext_clone(dest: *mut UText, src: *const UText, deep: i8, readonly: i8, status: *mut UErrorCode) -> *mut UText;
    pub fn utext_close(ut: *mut UText) -> *mut UText;
    pub fn utext_copy(ut: *mut UText, nativestart: i64, nativelimit: i64, destindex: i64, r#move: i8, status: *mut UErrorCode);
    pub fn utext_current32(ut: *mut UText) -> i32;
    pub fn utext_equals(a: *const UText, b: *const UText) -> i8;
    pub fn utext_extract(ut: *mut UText, nativestart: i64, nativelimit: i64, dest: *mut u16, destcapacity: i32, status: *mut UErrorCode) -> i32;
    pub fn utext_freeze(ut: *mut UText);
    pub fn utext_getNativeIndex(ut: *const UText) -> i64;
    pub fn utext_getPreviousNativeIndex(ut: *mut UText) -> i64;
    pub fn utext_hasMetaData(ut: *const UText) -> i8;
    pub fn utext_isLengthExpensive(ut: *const UText) -> i8;
    pub fn utext_isWritable(ut: *const UText) -> i8;
    pub fn utext_moveIndex32(ut: *mut UText, delta: i32) -> i8;
    pub fn utext_nativeLength(ut: *mut UText) -> i64;
    pub fn utext_next32(ut: *mut UText) -> i32;
    pub fn utext_next32From(ut: *mut UText, nativeindex: i64) -> i32;
    pub fn utext_openUChars(ut: *mut UText, s: *const u16, length: i64, status: *mut UErrorCode) -> *mut UText;
    #[cfg(feature = "Win32_Foundation")]
    pub fn utext_openUTF8(ut: *mut UText, s: super::Foundation::PSTR, length: i64, status: *mut UErrorCode) -> *mut UText;
    pub fn utext_previous32(ut: *mut UText) -> i32;
    pub fn utext_previous32From(ut: *mut UText, nativeindex: i64) -> i32;
    pub fn utext_replace(ut: *mut UText, nativestart: i64, nativelimit: i64, replacementtext: *const u16, replacementlength: i32, status: *mut UErrorCode) -> i32;
    pub fn utext_setNativeIndex(ut: *mut UText, nativeindex: i64);
    pub fn utext_setup(ut: *mut UText, extraspace: i32, status: *mut UErrorCode) -> *mut UText;
    pub fn utf8_appendCharSafeBody(s: *mut u8, i: i32, length: i32, c: i32, piserror: *mut i8) -> i32;
    pub fn utf8_back1SafeBody(s: *const u8, start: i32, i: i32) -> i32;
    pub fn utf8_nextCharSafeBody(s: *const u8, pi: *mut i32, length: i32, c: i32, strict: i8) -> i32;
    pub fn utf8_prevCharSafeBody(s: *const u8, start: i32, pi: *mut i32, c: i32, strict: i8) -> i32;
    pub fn utmscale_fromInt64(othertime: i64, timescale: UDateTimeScale, status: *mut UErrorCode) -> i64;
    pub fn utmscale_getTimeScaleValue(timescale: UDateTimeScale, value: UTimeScaleValue, status: *mut UErrorCode) -> i64;
    pub fn utmscale_toInt64(universaltime: i64, timescale: UDateTimeScale, status: *mut UErrorCode) -> i64;
    #[cfg(feature = "Win32_Foundation")]
    pub fn utrace_format(outbuf: super::Foundation::PSTR, capacity: i32, indent: i32, fmt: super::Foundation::PSTR) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn utrace_functionName(fnnumber: i32) -> super::Foundation::PSTR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn utrace_getFunctions(context: *const *const ::core::ffi::c_void, e: *mut UTraceEntry, x: *mut UTraceExit, d: *mut UTraceData);
    pub fn utrace_getLevel() -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn utrace_setFunctions(context: *const ::core::ffi::c_void, e: UTraceEntry, x: UTraceExit, d: UTraceData);
    pub fn utrace_setLevel(tracelevel: i32);
    #[cfg(feature = "Win32_Foundation")]
    pub fn utrace_vformat(outbuf: super::Foundation::PSTR, capacity: i32, indent: i32, fmt: super::Foundation::PSTR, args: *mut i8) -> i32;
    pub fn utrans_clone(trans: *const *const ::core::ffi::c_void, status: *mut UErrorCode) -> *mut *mut ::core::ffi::c_void;
    pub fn utrans_close(trans: *mut *mut ::core::ffi::c_void);
    pub fn utrans_countAvailableIDs() -> i32;
    pub fn utrans_getSourceSet(trans: *const *const ::core::ffi::c_void, ignorefilter: i8, fillin: *mut USet, status: *mut UErrorCode) -> *mut USet;
    pub fn utrans_getUnicodeID(trans: *const *const ::core::ffi::c_void, resultlength: *mut i32) -> *mut u16;
    pub fn utrans_openIDs(perrorcode: *mut UErrorCode) -> *mut UEnumeration;
    pub fn utrans_openInverse(trans: *const *const ::core::ffi::c_void, status: *mut UErrorCode) -> *mut *mut ::core::ffi::c_void;
    pub fn utrans_openU(id: *const u16, idlength: i32, dir: UTransDirection, rules: *const u16, ruleslength: i32, parseerror: *mut UParseError, perrorcode: *mut UErrorCode) -> *mut *mut ::core::ffi::c_void;
    pub fn utrans_register(adoptedtrans: *mut *mut ::core::ffi::c_void, status: *mut UErrorCode);
    pub fn utrans_setFilter(trans: *mut *mut ::core::ffi::c_void, filterpattern: *const u16, filterpatternlen: i32, status: *mut UErrorCode);
    pub fn utrans_toRules(trans: *const *const ::core::ffi::c_void, escapeunprintable: i8, result: *mut u16, resultlength: i32, status: *mut UErrorCode) -> i32;
    pub fn utrans_trans(trans: *const *const ::core::ffi::c_void, rep: *mut *mut ::core::ffi::c_void, repfunc: *const UReplaceableCallbacks, start: i32, limit: *mut i32, status: *mut UErrorCode);
    pub fn utrans_transIncremental(trans: *const *const ::core::ffi::c_void, rep: *mut *mut ::core::ffi::c_void, repfunc: *const UReplaceableCallbacks, pos: *mut UTransPosition, status: *mut UErrorCode);
    pub fn utrans_transIncrementalUChars(trans: *const *const ::core::ffi::c_void, text: *mut u16, textlength: *mut i32, textcapacity: i32, pos: *mut UTransPosition, status: *mut UErrorCode);
    pub fn utrans_transUChars(trans: *const *const ::core::ffi::c_void, text: *mut u16, textlength: *mut i32, textcapacity: i32, start: i32, limit: *mut i32, status: *mut UErrorCode);
    pub fn utrans_unregisterID(id: *const u16, idlength: i32);
}
pub const ALL_SERVICES: u32 = 0u32;
pub const ALL_SERVICE_TYPES: u32 = 0u32;
pub const C1_ALPHA: u32 = 256u32;
pub const C1_BLANK: u32 = 64u32;
pub const C1_CNTRL: u32 = 32u32;
pub const C1_DEFINED: u32 = 512u32;
pub const C1_DIGIT: u32 = 4u32;
pub const C1_LOWER: u32 = 2u32;
pub const C1_PUNCT: u32 = 16u32;
pub const C1_SPACE: u32 = 8u32;
pub const C1_UPPER: u32 = 1u32;
pub const C1_XDIGIT: u32 = 128u32;
pub const C2_ARABICNUMBER: u32 = 6u32;
pub const C2_BLOCKSEPARATOR: u32 = 8u32;
pub const C2_COMMONSEPARATOR: u32 = 7u32;
pub const C2_EUROPENUMBER: u32 = 3u32;
pub const C2_EUROPESEPARATOR: u32 = 4u32;
pub const C2_EUROPETERMINATOR: u32 = 5u32;
pub const C2_LEFTTORIGHT: u32 = 1u32;
pub const C2_NOTAPPLICABLE: u32 = 0u32;
pub const C2_OTHERNEUTRAL: u32 = 11u32;
pub const C2_RIGHTTOLEFT: u32 = 2u32;
pub const C2_SEGMENTSEPARATOR: u32 = 9u32;
pub const C2_WHITESPACE: u32 = 10u32;
pub const C3_ALPHA: u32 = 32768u32;
pub const C3_DIACRITIC: u32 = 2u32;
pub const C3_FULLWIDTH: u32 = 128u32;
pub const C3_HALFWIDTH: u32 = 64u32;
pub const C3_HIGHSURROGATE: u32 = 2048u32;
pub const C3_HIRAGANA: u32 = 32u32;
pub const C3_IDEOGRAPH: u32 = 256u32;
pub const C3_KASHIDA: u32 = 512u32;
pub const C3_KATAKANA: u32 = 16u32;
pub const C3_LEXICAL: u32 = 1024u32;
pub const C3_LOWSURROGATE: u32 = 4096u32;
pub const C3_NONSPACING: u32 = 1u32;
pub const C3_NOTAPPLICABLE: u32 = 0u32;
pub const C3_SYMBOL: u32 = 8u32;
pub const C3_VOWELMARK: u32 = 4u32;
pub struct CALINFO_ENUMPROCA(i32);
pub struct CALINFO_ENUMPROCEXA(i32);
pub struct CALINFO_ENUMPROCEXEX(i32);
pub struct CALINFO_ENUMPROCEXW(i32);
pub struct CALINFO_ENUMPROCW(i32);
pub const CAL_GREGORIAN: u32 = 1u32;
pub const CAL_GREGORIAN_ARABIC: u32 = 10u32;
pub const CAL_GREGORIAN_ME_FRENCH: u32 = 9u32;
pub const CAL_GREGORIAN_US: u32 = 2u32;
pub const CAL_GREGORIAN_XLIT_ENGLISH: u32 = 11u32;
pub const CAL_GREGORIAN_XLIT_FRENCH: u32 = 12u32;
pub const CAL_HEBREW: u32 = 8u32;
pub const CAL_HIJRI: u32 = 6u32;
pub const CAL_ICALINTVALUE: u32 = 1u32;
pub const CAL_ITWODIGITYEARMAX: u32 = 48u32;
pub const CAL_IYEAROFFSETRANGE: u32 = 3u32;
pub const CAL_JAPAN: u32 = 3u32;
pub const CAL_KOREA: u32 = 5u32;
pub const CAL_NOUSEROVERRIDE: u32 = 2147483648u32;
pub const CAL_PERSIAN: u32 = 22u32;
pub const CAL_RETURN_GENITIVE_NAMES: u32 = 268435456u32;
pub const CAL_RETURN_NUMBER: u32 = 536870912u32;
pub const CAL_SABBREVDAYNAME1: u32 = 14u32;
pub const CAL_SABBREVDAYNAME2: u32 = 15u32;
pub const CAL_SABBREVDAYNAME3: u32 = 16u32;
pub const CAL_SABBREVDAYNAME4: u32 = 17u32;
pub const CAL_SABBREVDAYNAME5: u32 = 18u32;
pub const CAL_SABBREVDAYNAME6: u32 = 19u32;
pub const CAL_SABBREVDAYNAME7: u32 = 20u32;
pub const CAL_SABBREVERASTRING: u32 = 57u32;
pub const CAL_SABBREVMONTHNAME1: u32 = 34u32;
pub const CAL_SABBREVMONTHNAME10: u32 = 43u32;
pub const CAL_SABBREVMONTHNAME11: u32 = 44u32;
pub const CAL_SABBREVMONTHNAME12: u32 = 45u32;
pub const CAL_SABBREVMONTHNAME13: u32 = 46u32;
pub const CAL_SABBREVMONTHNAME2: u32 = 35u32;
pub const CAL_SABBREVMONTHNAME3: u32 = 36u32;
pub const CAL_SABBREVMONTHNAME4: u32 = 37u32;
pub const CAL_SABBREVMONTHNAME5: u32 = 38u32;
pub const CAL_SABBREVMONTHNAME6: u32 = 39u32;
pub const CAL_SABBREVMONTHNAME7: u32 = 40u32;
pub const CAL_SABBREVMONTHNAME8: u32 = 41u32;
pub const CAL_SABBREVMONTHNAME9: u32 = 42u32;
pub const CAL_SCALNAME: u32 = 2u32;
pub const CAL_SDAYNAME1: u32 = 7u32;
pub const CAL_SDAYNAME2: u32 = 8u32;
pub const CAL_SDAYNAME3: u32 = 9u32;
pub const CAL_SDAYNAME4: u32 = 10u32;
pub const CAL_SDAYNAME5: u32 = 11u32;
pub const CAL_SDAYNAME6: u32 = 12u32;
pub const CAL_SDAYNAME7: u32 = 13u32;
pub const CAL_SENGLISHABBREVERANAME: u32 = 60u32;
pub const CAL_SENGLISHERANAME: u32 = 59u32;
pub const CAL_SERASTRING: u32 = 4u32;
pub const CAL_SJAPANESEERAFIRSTYEAR: u32 = 61u32;
pub const CAL_SLONGDATE: u32 = 6u32;
pub const CAL_SMONTHDAY: u32 = 56u32;
pub const CAL_SMONTHNAME1: u32 = 21u32;
pub const CAL_SMONTHNAME10: u32 = 30u32;
pub const CAL_SMONTHNAME11: u32 = 31u32;
pub const CAL_SMONTHNAME12: u32 = 32u32;
pub const CAL_SMONTHNAME13: u32 = 33u32;
pub const CAL_SMONTHNAME2: u32 = 22u32;
pub const CAL_SMONTHNAME3: u32 = 23u32;
pub const CAL_SMONTHNAME4: u32 = 24u32;
pub const CAL_SMONTHNAME5: u32 = 25u32;
pub const CAL_SMONTHNAME6: u32 = 26u32;
pub const CAL_SMONTHNAME7: u32 = 27u32;
pub const CAL_SMONTHNAME8: u32 = 28u32;
pub const CAL_SMONTHNAME9: u32 = 29u32;
pub const CAL_SRELATIVELONGDATE: u32 = 58u32;
pub const CAL_SSHORTDATE: u32 = 5u32;
pub const CAL_SSHORTESTDAYNAME1: u32 = 49u32;
pub const CAL_SSHORTESTDAYNAME2: u32 = 50u32;
pub const CAL_SSHORTESTDAYNAME3: u32 = 51u32;
pub const CAL_SSHORTESTDAYNAME4: u32 = 52u32;
pub const CAL_SSHORTESTDAYNAME5: u32 = 53u32;
pub const CAL_SSHORTESTDAYNAME6: u32 = 54u32;
pub const CAL_SSHORTESTDAYNAME7: u32 = 55u32;
pub const CAL_SYEARMONTH: u32 = 47u32;
pub const CAL_TAIWAN: u32 = 4u32;
pub const CAL_THAI: u32 = 7u32;
pub const CAL_UMALQURA: u32 = 23u32;
pub const CAL_USE_CP_ACP: u32 = 1073741824u32;
pub const CANITER_SKIP_ZEROES: u32 = 1u32;
pub struct CHARSETINFO(i32);
pub struct CMLangConvertCharset(i32);
pub struct CMLangString(i32);
pub struct CMultiLanguage(i32);
pub struct CODEPAGE_ENUMPROCA(i32);
pub struct CODEPAGE_ENUMPROCW(i32);
pub struct COMPARE_STRING_FLAGS(i32);
pub struct CORRECTIVE_ACTION(i32);
pub struct CPINFO(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct CPINFOEXA(i32);
pub struct CPINFOEXW(i32);
pub const CPIOD_FORCE_PROMPT: i32 = -2147483648i32;
pub const CPIOD_PEEK: i32 = 1073741824i32;
pub const CP_ACP: u32 = 0u32;
pub const CP_MACCP: u32 = 2u32;
pub const CP_OEMCP: u32 = 1u32;
pub const CP_SYMBOL: u32 = 42u32;
pub const CP_THREAD_ACP: u32 = 3u32;
pub const CP_UTF7: u32 = 65000u32;
pub const CP_UTF8: u32 = 65001u32;
pub const CSTR_EQUAL: u32 = 2u32;
pub const CSTR_GREATER_THAN: u32 = 3u32;
pub const CSTR_LESS_THAN: u32 = 1u32;
pub const CTRY_ALBANIA: u32 = 355u32;
pub const CTRY_ALGERIA: u32 = 213u32;
pub const CTRY_ARGENTINA: u32 = 54u32;
pub const CTRY_ARMENIA: u32 = 374u32;
pub const CTRY_AUSTRALIA: u32 = 61u32;
pub const CTRY_AUSTRIA: u32 = 43u32;
pub const CTRY_AZERBAIJAN: u32 = 994u32;
pub const CTRY_BAHRAIN: u32 = 973u32;
pub const CTRY_BELARUS: u32 = 375u32;
pub const CTRY_BELGIUM: u32 = 32u32;
pub const CTRY_BELIZE: u32 = 501u32;
pub const CTRY_BOLIVIA: u32 = 591u32;
pub const CTRY_BRAZIL: u32 = 55u32;
pub const CTRY_BRUNEI_DARUSSALAM: u32 = 673u32;
pub const CTRY_BULGARIA: u32 = 359u32;
pub const CTRY_CANADA: u32 = 2u32;
pub const CTRY_CARIBBEAN: u32 = 1u32;
pub const CTRY_CHILE: u32 = 56u32;
pub const CTRY_COLOMBIA: u32 = 57u32;
pub const CTRY_COSTA_RICA: u32 = 506u32;
pub const CTRY_CROATIA: u32 = 385u32;
pub const CTRY_CZECH: u32 = 420u32;
pub const CTRY_DEFAULT: u32 = 0u32;
pub const CTRY_DENMARK: u32 = 45u32;
pub const CTRY_DOMINICAN_REPUBLIC: u32 = 1u32;
pub const CTRY_ECUADOR: u32 = 593u32;
pub const CTRY_EGYPT: u32 = 20u32;
pub const CTRY_EL_SALVADOR: u32 = 503u32;
pub const CTRY_ESTONIA: u32 = 372u32;
pub const CTRY_FAEROE_ISLANDS: u32 = 298u32;
pub const CTRY_FINLAND: u32 = 358u32;
pub const CTRY_FRANCE: u32 = 33u32;
pub const CTRY_GEORGIA: u32 = 995u32;
pub const CTRY_GERMANY: u32 = 49u32;
pub const CTRY_GREECE: u32 = 30u32;
pub const CTRY_GUATEMALA: u32 = 502u32;
pub const CTRY_HONDURAS: u32 = 504u32;
pub const CTRY_HONG_KONG: u32 = 852u32;
pub const CTRY_HUNGARY: u32 = 36u32;
pub const CTRY_ICELAND: u32 = 354u32;
pub const CTRY_INDIA: u32 = 91u32;
pub const CTRY_INDONESIA: u32 = 62u32;
pub const CTRY_IRAN: u32 = 981u32;
pub const CTRY_IRAQ: u32 = 964u32;
pub const CTRY_IRELAND: u32 = 353u32;
pub const CTRY_ISRAEL: u32 = 972u32;
pub const CTRY_ITALY: u32 = 39u32;
pub const CTRY_JAMAICA: u32 = 1u32;
pub const CTRY_JAPAN: u32 = 81u32;
pub const CTRY_JORDAN: u32 = 962u32;
pub const CTRY_KAZAKSTAN: u32 = 7u32;
pub const CTRY_KENYA: u32 = 254u32;
pub const CTRY_KUWAIT: u32 = 965u32;
pub const CTRY_KYRGYZSTAN: u32 = 996u32;
pub const CTRY_LATVIA: u32 = 371u32;
pub const CTRY_LEBANON: u32 = 961u32;
pub const CTRY_LIBYA: u32 = 218u32;
pub const CTRY_LIECHTENSTEIN: u32 = 41u32;
pub const CTRY_LITHUANIA: u32 = 370u32;
pub const CTRY_LUXEMBOURG: u32 = 352u32;
pub const CTRY_MACAU: u32 = 853u32;
pub const CTRY_MACEDONIA: u32 = 389u32;
pub const CTRY_MALAYSIA: u32 = 60u32;
pub const CTRY_MALDIVES: u32 = 960u32;
pub const CTRY_MEXICO: u32 = 52u32;
pub const CTRY_MONACO: u32 = 33u32;
pub const CTRY_MONGOLIA: u32 = 976u32;
pub const CTRY_MOROCCO: u32 = 212u32;
pub const CTRY_NETHERLANDS: u32 = 31u32;
pub const CTRY_NEW_ZEALAND: u32 = 64u32;
pub const CTRY_NICARAGUA: u32 = 505u32;
pub const CTRY_NORWAY: u32 = 47u32;
pub const CTRY_OMAN: u32 = 968u32;
pub const CTRY_PAKISTAN: u32 = 92u32;
pub const CTRY_PANAMA: u32 = 507u32;
pub const CTRY_PARAGUAY: u32 = 595u32;
pub const CTRY_PERU: u32 = 51u32;
pub const CTRY_PHILIPPINES: u32 = 63u32;
pub const CTRY_POLAND: u32 = 48u32;
pub const CTRY_PORTUGAL: u32 = 351u32;
pub const CTRY_PRCHINA: u32 = 86u32;
pub const CTRY_PUERTO_RICO: u32 = 1u32;
pub const CTRY_QATAR: u32 = 974u32;
pub const CTRY_ROMANIA: u32 = 40u32;
pub const CTRY_RUSSIA: u32 = 7u32;
pub const CTRY_SAUDI_ARABIA: u32 = 966u32;
pub const CTRY_SERBIA: u32 = 381u32;
pub const CTRY_SINGAPORE: u32 = 65u32;
pub const CTRY_SLOVAK: u32 = 421u32;
pub const CTRY_SLOVENIA: u32 = 386u32;
pub const CTRY_SOUTH_AFRICA: u32 = 27u32;
pub const CTRY_SOUTH_KOREA: u32 = 82u32;
pub const CTRY_SPAIN: u32 = 34u32;
pub const CTRY_SWEDEN: u32 = 46u32;
pub const CTRY_SWITZERLAND: u32 = 41u32;
pub const CTRY_SYRIA: u32 = 963u32;
pub const CTRY_TAIWAN: u32 = 886u32;
pub const CTRY_TATARSTAN: u32 = 7u32;
pub const CTRY_THAILAND: u32 = 66u32;
pub const CTRY_TRINIDAD_Y_TOBAGO: u32 = 1u32;
pub const CTRY_TUNISIA: u32 = 216u32;
pub const CTRY_TURKEY: u32 = 90u32;
pub const CTRY_UAE: u32 = 971u32;
pub const CTRY_UKRAINE: u32 = 380u32;
pub const CTRY_UNITED_KINGDOM: u32 = 44u32;
pub const CTRY_UNITED_STATES: u32 = 1u32;
pub const CTRY_URUGUAY: u32 = 598u32;
pub const CTRY_UZBEKISTAN: u32 = 7u32;
pub const CTRY_VENEZUELA: u32 = 58u32;
pub const CTRY_VIET_NAM: u32 = 84u32;
pub const CTRY_YEMEN: u32 = 967u32;
pub const CTRY_ZIMBABWE: u32 = 263u32;
pub const CT_CTYPE1: u32 = 1u32;
pub const CT_CTYPE2: u32 = 2u32;
pub const CT_CTYPE3: u32 = 4u32;
#[cfg(feature = "Win32_Foundation")]
pub struct CURRENCYFMTA(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct CURRENCYFMTW(i32);
pub struct DATEFMT_ENUMPROCA(i32);
pub struct DATEFMT_ENUMPROCEXA(i32);
pub struct DATEFMT_ENUMPROCEXEX(i32);
pub struct DATEFMT_ENUMPROCEXW(i32);
pub struct DATEFMT_ENUMPROCW(i32);
pub struct DetectEncodingInfo(i32);
pub const ELS_GUID_LANGUAGE_DETECTION: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3481141425, data2: 37019, data3: 19861, data4: [168, 244, 97, 31, 124, 55, 119, 2] };
pub const ELS_GUID_SCRIPT_DETECTION: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 761574457,
    data2: 27823,
    data3: 20331,
    data4: [182, 136, 229, 208, 244, 250, 167, 215],
};
pub const ELS_GUID_TRANSLITERATION_BENGALI_TO_LATIN: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 4108310565,
    data2: 37284,
    data3: 18591,
    data4: [133, 94, 154, 217, 190, 229, 87, 39],
};
pub const ELS_GUID_TRANSLITERATION_CYRILLIC_TO_LATIN: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1037118104, data2: 23293, data3: 18691, data4: [161, 63, 225, 126, 108, 11, 254, 1] };
pub const ELS_GUID_TRANSLITERATION_DEVANAGARI_TO_LATIN: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3299138814, data2: 9825, data3: 19714, data4: [152, 53, 244, 129, 135, 16, 152, 3] };
pub const ELS_GUID_TRANSLITERATION_HANGUL_DECOMPOSITION: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 1268950817,
    data2: 58429,
    data3: 16823,
    data4: [179, 48, 83, 106, 225, 228, 136, 99],
};
pub const ELS_GUID_TRANSLITERATION_HANS_TO_HANT: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 1017957832,
    data2: 21904,
    data3: 17116,
    data4: [154, 123, 181, 166, 181, 179, 182, 59],
};
pub const ELS_GUID_TRANSLITERATION_HANT_TO_HANS: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2745709371, data2: 62716, data3: 17142, data4: [160, 196, 4, 98, 254, 115, 23, 203] };
pub const ELS_GUID_TRANSLITERATION_MALAYALAM_TO_LATIN: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3636036529, data2: 63679, data3: 18987, data4: [188, 213, 91, 94, 162, 6, 19, 225] };
#[cfg(feature = "Win32_Graphics_Gdi")]
pub struct ENUMTEXTMETRICA(i32);
#[cfg(feature = "Win32_Graphics_Gdi")]
pub struct ENUMTEXTMETRICW(i32);
pub const ENUM_ALL_CALENDARS: u32 = 4294967295u32;
pub struct ENUM_DATE_FORMATS_FLAGS(i32);
pub struct ENUM_SYSTEM_CODE_PAGES_FLAGS(i32);
pub struct ENUM_SYSTEM_LANGUAGE_GROUPS_FLAGS(i32);
pub struct FILEMUIINFO(i32);
pub const FIND_ENDSWITH: u32 = 2097152u32;
pub const FIND_FROMEND: u32 = 8388608u32;
pub const FIND_FROMSTART: u32 = 4194304u32;
pub const FIND_STARTSWITH: u32 = 1048576u32;
pub struct FOLD_STRING_MAP_FLAGS(i32);
pub struct FONTSIGNATURE(i32);
pub const GEOID_NOT_AVAILABLE: i32 = -1i32;
pub struct GEO_ENUMNAMEPROC(i32);
pub struct GEO_ENUMPROC(i32);
pub struct GOFFSET(i32);
pub const GSS_ALLOW_INHERITED_COMMON: u32 = 1u32;
pub const HIGHLEVEL_SERVICE_TYPES: u32 = 1u32;
pub const HIGH_SURROGATE_END: u32 = 56319u32;
pub const HIGH_SURROGATE_START: u32 = 55296u32;
pub struct HIMC(i32);
pub struct HIMCC(i32);
pub struct HSAVEDUILANGUAGES(i32);
#[repr(transparent)]
pub struct IComprehensiveSpellCheckProvider(pub *mut ::core::ffi::c_void);
pub const IDN_ALLOW_UNASSIGNED: u32 = 1u32;
pub const IDN_EMAIL_ADDRESS: u32 = 4u32;
pub const IDN_RAW_PUNYCODE: u32 = 8u32;
pub const IDN_USE_STD3_ASCII_RULES: u32 = 2u32;
#[repr(transparent)]
pub struct IEnumCodePage(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEnumRfc1766(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEnumScript(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEnumSpellingError(pub *mut ::core::ffi::c_void);
pub const IME_CMODE_ALPHANUMERIC: u32 = 0u32;
pub const IME_CMODE_CHARCODE: u32 = 32u32;
pub const IME_CMODE_CHINESE: u32 = 1u32;
pub const IME_CMODE_FULLSHAPE: u32 = 8u32;
pub const IME_CMODE_HANGUL: u32 = 1u32;
pub const IME_CMODE_HANJACONVERT: u32 = 64u32;
pub const IME_CMODE_JAPANESE: u32 = 1u32;
pub const IME_CMODE_KATAKANA: u32 = 2u32;
pub const IME_CMODE_LANGUAGE: u32 = 3u32;
pub const IME_CMODE_NATIVE: u32 = 1u32;
pub const IME_CMODE_NATIVESYMBOL: u32 = 128u32;
pub const IME_CMODE_ROMAN: u32 = 16u32;
#[repr(transparent)]
pub struct IMLangCodePages(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMLangConvertCharset(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMLangFontLink(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMLangFontLink2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMLangLineBreakConsole(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMLangString(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMLangStringAStr(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMLangStringBufA(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMLangStringBufW(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMLangStringWStr(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMultiLanguage(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMultiLanguage2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMultiLanguage3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IOptionDescription(pub *mut ::core::ffi::c_void);
pub struct IS_TEXT_UNICODE_RESULT(i32);
pub struct IS_VALID_LOCALE_FLAGS(i32);
#[repr(transparent)]
pub struct ISpellCheckProvider(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpellCheckProviderFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpellChecker(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpellChecker2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpellCheckerChangedEventHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpellCheckerFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpellingError(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUserDictionariesRegistrar(pub *mut ::core::ffi::c_void);
pub struct LANGGROUPLOCALE_ENUMPROCA(i32);
pub struct LANGGROUPLOCALE_ENUMPROCW(i32);
pub struct LANGUAGEGROUP_ENUMPROCA(i32);
pub struct LANGUAGEGROUP_ENUMPROCW(i32);
pub const LCID_ALTERNATE_SORTS: u32 = 4u32;
pub const LCMAP_BYTEREV: u32 = 2048u32;
pub const LCMAP_FULLWIDTH: u32 = 8388608u32;
pub const LCMAP_HALFWIDTH: u32 = 4194304u32;
pub const LCMAP_HASH: u32 = 262144u32;
pub const LCMAP_HIRAGANA: u32 = 1048576u32;
pub const LCMAP_KATAKANA: u32 = 2097152u32;
pub const LCMAP_LINGUISTIC_CASING: u32 = 16777216u32;
pub const LCMAP_LOWERCASE: u32 = 256u32;
pub const LCMAP_SIMPLIFIED_CHINESE: u32 = 33554432u32;
pub const LCMAP_SORTHANDLE: u32 = 536870912u32;
pub const LCMAP_SORTKEY: u32 = 1024u32;
pub const LCMAP_TITLECASE: u32 = 768u32;
pub const LCMAP_TRADITIONAL_CHINESE: u32 = 67108864u32;
pub const LCMAP_UPPERCASE: u32 = 512u32;
pub const LGRPID_ARABIC: u32 = 13u32;
pub const LGRPID_ARMENIAN: u32 = 17u32;
pub const LGRPID_BALTIC: u32 = 3u32;
pub const LGRPID_CENTRAL_EUROPE: u32 = 2u32;
pub const LGRPID_CYRILLIC: u32 = 5u32;
pub const LGRPID_GEORGIAN: u32 = 16u32;
pub const LGRPID_GREEK: u32 = 4u32;
pub const LGRPID_HEBREW: u32 = 12u32;
pub const LGRPID_INDIC: u32 = 15u32;
pub const LGRPID_JAPANESE: u32 = 7u32;
pub const LGRPID_KOREAN: u32 = 8u32;
pub const LGRPID_SIMPLIFIED_CHINESE: u32 = 10u32;
pub const LGRPID_THAI: u32 = 11u32;
pub const LGRPID_TRADITIONAL_CHINESE: u32 = 9u32;
pub const LGRPID_TURKIC: u32 = 6u32;
pub const LGRPID_TURKISH: u32 = 6u32;
pub const LGRPID_VIETNAMESE: u32 = 14u32;
pub const LGRPID_WESTERN_EUROPE: u32 = 1u32;
pub struct LOCALESIGNATURE(i32);
pub const LOCALE_ALL: u32 = 0u32;
pub const LOCALE_ALLOW_NEUTRAL_NAMES: u32 = 134217728u32;
pub const LOCALE_ALTERNATE_SORTS: u32 = 4u32;
pub struct LOCALE_ENUMPROCA(i32);
pub struct LOCALE_ENUMPROCEX(i32);
pub struct LOCALE_ENUMPROCW(i32);
pub const LOCALE_FONTSIGNATURE: u32 = 88u32;
pub const LOCALE_ICALENDARTYPE: u32 = 4105u32;
pub const LOCALE_ICENTURY: u32 = 36u32;
pub const LOCALE_ICONSTRUCTEDLOCALE: u32 = 125u32;
pub const LOCALE_ICOUNTRY: u32 = 5u32;
pub const LOCALE_ICURRDIGITS: u32 = 25u32;
pub const LOCALE_ICURRENCY: u32 = 27u32;
pub const LOCALE_IDATE: u32 = 33u32;
pub const LOCALE_IDAYLZERO: u32 = 38u32;
pub const LOCALE_IDEFAULTANSICODEPAGE: u32 = 4100u32;
pub const LOCALE_IDEFAULTCODEPAGE: u32 = 11u32;
pub const LOCALE_IDEFAULTCOUNTRY: u32 = 10u32;
pub const LOCALE_IDEFAULTEBCDICCODEPAGE: u32 = 4114u32;
pub const LOCALE_IDEFAULTLANGUAGE: u32 = 9u32;
pub const LOCALE_IDEFAULTMACCODEPAGE: u32 = 4113u32;
pub const LOCALE_IDIALINGCODE: u32 = 5u32;
pub const LOCALE_IDIGITS: u32 = 17u32;
pub const LOCALE_IDIGITSUBSTITUTION: u32 = 4116u32;
pub const LOCALE_IFIRSTDAYOFWEEK: u32 = 4108u32;
pub const LOCALE_IFIRSTWEEKOFYEAR: u32 = 4109u32;
pub const LOCALE_IGEOID: u32 = 91u32;
pub const LOCALE_IINTLCURRDIGITS: u32 = 26u32;
pub const LOCALE_ILANGUAGE: u32 = 1u32;
pub const LOCALE_ILDATE: u32 = 34u32;
pub const LOCALE_ILZERO: u32 = 18u32;
pub const LOCALE_IMEASURE: u32 = 13u32;
pub const LOCALE_IMONLZERO: u32 = 39u32;
pub const LOCALE_INEGATIVEPERCENT: u32 = 116u32;
pub const LOCALE_INEGCURR: u32 = 28u32;
pub const LOCALE_INEGNUMBER: u32 = 4112u32;
pub const LOCALE_INEGSEPBYSPACE: u32 = 87u32;
pub const LOCALE_INEGSIGNPOSN: u32 = 83u32;
pub const LOCALE_INEGSYMPRECEDES: u32 = 86u32;
pub const LOCALE_INEUTRAL: u32 = 113u32;
pub const LOCALE_IOPTIONALCALENDAR: u32 = 4107u32;
pub const LOCALE_IPAPERSIZE: u32 = 4106u32;
pub const LOCALE_IPOSITIVEPERCENT: u32 = 117u32;
pub const LOCALE_IPOSSEPBYSPACE: u32 = 85u32;
pub const LOCALE_IPOSSIGNPOSN: u32 = 82u32;
pub const LOCALE_IPOSSYMPRECEDES: u32 = 84u32;
pub const LOCALE_IREADINGLAYOUT: u32 = 112u32;
pub const LOCALE_ITIME: u32 = 35u32;
pub const LOCALE_ITIMEMARKPOSN: u32 = 4101u32;
pub const LOCALE_ITLZERO: u32 = 37u32;
pub const LOCALE_IUSEUTF8LEGACYACP: u32 = 1638u32;
pub const LOCALE_IUSEUTF8LEGACYOEMCP: u32 = 2457u32;
pub const LOCALE_NEUTRALDATA: u32 = 16u32;
pub const LOCALE_NOUSEROVERRIDE: u32 = 2147483648u32;
pub const LOCALE_REPLACEMENT: u32 = 8u32;
pub const LOCALE_RETURN_GENITIVE_NAMES: u32 = 268435456u32;
pub const LOCALE_RETURN_NUMBER: u32 = 536870912u32;
pub const LOCALE_S1159: u32 = 40u32;
pub const LOCALE_S2359: u32 = 41u32;
pub const LOCALE_SABBREVCTRYNAME: u32 = 7u32;
pub const LOCALE_SABBREVDAYNAME1: u32 = 49u32;
pub const LOCALE_SABBREVDAYNAME2: u32 = 50u32;
pub const LOCALE_SABBREVDAYNAME3: u32 = 51u32;
pub const LOCALE_SABBREVDAYNAME4: u32 = 52u32;
pub const LOCALE_SABBREVDAYNAME5: u32 = 53u32;
pub const LOCALE_SABBREVDAYNAME6: u32 = 54u32;
pub const LOCALE_SABBREVDAYNAME7: u32 = 55u32;
pub const LOCALE_SABBREVLANGNAME: u32 = 3u32;
pub const LOCALE_SABBREVMONTHNAME1: u32 = 68u32;
pub const LOCALE_SABBREVMONTHNAME10: u32 = 77u32;
pub const LOCALE_SABBREVMONTHNAME11: u32 = 78u32;
pub const LOCALE_SABBREVMONTHNAME12: u32 = 79u32;
pub const LOCALE_SABBREVMONTHNAME13: u32 = 4111u32;
pub const LOCALE_SABBREVMONTHNAME2: u32 = 69u32;
pub const LOCALE_SABBREVMONTHNAME3: u32 = 70u32;
pub const LOCALE_SABBREVMONTHNAME4: u32 = 71u32;
pub const LOCALE_SABBREVMONTHNAME5: u32 = 72u32;
pub const LOCALE_SABBREVMONTHNAME6: u32 = 73u32;
pub const LOCALE_SABBREVMONTHNAME7: u32 = 74u32;
pub const LOCALE_SABBREVMONTHNAME8: u32 = 75u32;
pub const LOCALE_SABBREVMONTHNAME9: u32 = 76u32;
pub const LOCALE_SAM: u32 = 40u32;
pub const LOCALE_SCONSOLEFALLBACKNAME: u32 = 110u32;
pub const LOCALE_SCOUNTRY: u32 = 6u32;
pub const LOCALE_SCURRENCY: u32 = 20u32;
pub const LOCALE_SDATE: u32 = 29u32;
pub const LOCALE_SDAYNAME1: u32 = 42u32;
pub const LOCALE_SDAYNAME2: u32 = 43u32;
pub const LOCALE_SDAYNAME3: u32 = 44u32;
pub const LOCALE_SDAYNAME4: u32 = 45u32;
pub const LOCALE_SDAYNAME5: u32 = 46u32;
pub const LOCALE_SDAYNAME6: u32 = 47u32;
pub const LOCALE_SDAYNAME7: u32 = 48u32;
pub const LOCALE_SDECIMAL: u32 = 14u32;
pub const LOCALE_SDURATION: u32 = 93u32;
pub const LOCALE_SENGCOUNTRY: u32 = 4098u32;
pub const LOCALE_SENGCURRNAME: u32 = 4103u32;
pub const LOCALE_SENGLANGUAGE: u32 = 4097u32;
pub const LOCALE_SENGLISHCOUNTRYNAME: u32 = 4098u32;
pub const LOCALE_SENGLISHDISPLAYNAME: u32 = 114u32;
pub const LOCALE_SENGLISHLANGUAGENAME: u32 = 4097u32;
pub const LOCALE_SGROUPING: u32 = 16u32;
pub const LOCALE_SINTLSYMBOL: u32 = 21u32;
pub const LOCALE_SISO3166CTRYNAME: u32 = 90u32;
pub const LOCALE_SISO3166CTRYNAME2: u32 = 104u32;
pub const LOCALE_SISO639LANGNAME: u32 = 89u32;
pub const LOCALE_SISO639LANGNAME2: u32 = 103u32;
pub const LOCALE_SKEYBOARDSTOINSTALL: u32 = 94u32;
pub const LOCALE_SLANGDISPLAYNAME: u32 = 111u32;
pub const LOCALE_SLANGUAGE: u32 = 2u32;
pub const LOCALE_SLIST: u32 = 12u32;
pub const LOCALE_SLOCALIZEDCOUNTRYNAME: u32 = 6u32;
pub const LOCALE_SLOCALIZEDDISPLAYNAME: u32 = 2u32;
pub const LOCALE_SLOCALIZEDLANGUAGENAME: u32 = 111u32;
pub const LOCALE_SLONGDATE: u32 = 32u32;
pub const LOCALE_SMONDECIMALSEP: u32 = 22u32;
pub const LOCALE_SMONGROUPING: u32 = 24u32;
pub const LOCALE_SMONTHDAY: u32 = 120u32;
pub const LOCALE_SMONTHNAME1: u32 = 56u32;
pub const LOCALE_SMONTHNAME10: u32 = 65u32;
pub const LOCALE_SMONTHNAME11: u32 = 66u32;
pub const LOCALE_SMONTHNAME12: u32 = 67u32;
pub const LOCALE_SMONTHNAME13: u32 = 4110u32;
pub const LOCALE_SMONTHNAME2: u32 = 57u32;
pub const LOCALE_SMONTHNAME3: u32 = 58u32;
pub const LOCALE_SMONTHNAME4: u32 = 59u32;
pub const LOCALE_SMONTHNAME5: u32 = 60u32;
pub const LOCALE_SMONTHNAME6: u32 = 61u32;
pub const LOCALE_SMONTHNAME7: u32 = 62u32;
pub const LOCALE_SMONTHNAME8: u32 = 63u32;
pub const LOCALE_SMONTHNAME9: u32 = 64u32;
pub const LOCALE_SMONTHOUSANDSEP: u32 = 23u32;
pub const LOCALE_SNAME: u32 = 92u32;
pub const LOCALE_SNAN: u32 = 105u32;
pub const LOCALE_SNATIVECOUNTRYNAME: u32 = 8u32;
pub const LOCALE_SNATIVECTRYNAME: u32 = 8u32;
pub const LOCALE_SNATIVECURRNAME: u32 = 4104u32;
pub const LOCALE_SNATIVEDIGITS: u32 = 19u32;
pub const LOCALE_SNATIVEDISPLAYNAME: u32 = 115u32;
pub const LOCALE_SNATIVELANGNAME: u32 = 4u32;
pub const LOCALE_SNATIVELANGUAGENAME: u32 = 4u32;
pub const LOCALE_SNEGATIVESIGN: u32 = 81u32;
pub const LOCALE_SNEGINFINITY: u32 = 107u32;
pub const LOCALE_SOPENTYPELANGUAGETAG: u32 = 122u32;
pub const LOCALE_SPARENT: u32 = 109u32;
pub const LOCALE_SPECIFICDATA: u32 = 32u32;
pub const LOCALE_SPERCENT: u32 = 118u32;
pub const LOCALE_SPERMILLE: u32 = 119u32;
pub const LOCALE_SPM: u32 = 41u32;
pub const LOCALE_SPOSINFINITY: u32 = 106u32;
pub const LOCALE_SPOSITIVESIGN: u32 = 80u32;
pub const LOCALE_SRELATIVELONGDATE: u32 = 124u32;
pub const LOCALE_SSCRIPTS: u32 = 108u32;
pub const LOCALE_SSHORTDATE: u32 = 31u32;
pub const LOCALE_SSHORTESTAM: u32 = 126u32;
pub const LOCALE_SSHORTESTDAYNAME1: u32 = 96u32;
pub const LOCALE_SSHORTESTDAYNAME2: u32 = 97u32;
pub const LOCALE_SSHORTESTDAYNAME3: u32 = 98u32;
pub const LOCALE_SSHORTESTDAYNAME4: u32 = 99u32;
pub const LOCALE_SSHORTESTDAYNAME5: u32 = 100u32;
pub const LOCALE_SSHORTESTDAYNAME6: u32 = 101u32;
pub const LOCALE_SSHORTESTDAYNAME7: u32 = 102u32;
pub const LOCALE_SSHORTESTPM: u32 = 127u32;
pub const LOCALE_SSHORTTIME: u32 = 121u32;
pub const LOCALE_SSORTLOCALE: u32 = 123u32;
pub const LOCALE_SSORTNAME: u32 = 4115u32;
pub const LOCALE_STHOUSAND: u32 = 15u32;
pub const LOCALE_STIME: u32 = 30u32;
pub const LOCALE_STIMEFORMAT: u32 = 4099u32;
pub const LOCALE_SUPPLEMENTAL: u32 = 2u32;
pub const LOCALE_SYEARMONTH: u32 = 4102u32;
pub const LOCALE_USE_CP_ACP: u32 = 1073741824u32;
pub const LOCALE_WINDOWS: u32 = 1u32;
pub const LOWLEVEL_SERVICE_TYPES: u32 = 2u32;
pub const LOW_SURROGATE_END: u32 = 57343u32;
pub const LOW_SURROGATE_START: u32 = 56320u32;
#[cfg(feature = "Win32_Foundation")]
pub struct MAPPING_DATA_RANGE(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct MAPPING_ENUM_OPTIONS(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct MAPPING_OPTIONS(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct MAPPING_PROPERTY_BAG(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct MAPPING_SERVICE_INFO(i32);
pub const MAX_DEFAULTCHAR: u32 = 2u32;
pub const MAX_LEADBYTES: u32 = 12u32;
pub const MAX_LOCALE_NAME: u32 = 32u32;
pub const MAX_MIMECP_NAME: u32 = 64u32;
pub const MAX_MIMECSET_NAME: u32 = 50u32;
pub const MAX_MIMEFACE_NAME: u32 = 32u32;
pub const MAX_RFC1766_NAME: u32 = 6u32;
pub const MAX_SCRIPT_NAME: u32 = 48u32;
pub struct MIMECONTF(i32);
pub struct MIMECPINFO(i32);
pub struct MIMECSETINFO(i32);
pub const MIN_SPELLING_NTDDI: u32 = 100794368u32;
pub struct MLDETECTCP(i32);
pub struct MLSTR_FLAGS(i32);
pub const MUI_COMPLEX_SCRIPT_FILTER: u32 = 512u32;
pub const MUI_CONSOLE_FILTER: u32 = 256u32;
pub const MUI_FILEINFO_VERSION: u32 = 1u32;
pub const MUI_FILETYPE_LANGUAGE_NEUTRAL_MAIN: u32 = 2u32;
pub const MUI_FILETYPE_LANGUAGE_NEUTRAL_MUI: u32 = 4u32;
pub const MUI_FILETYPE_NOT_LANGUAGE_NEUTRAL: u32 = 1u32;
pub const MUI_FORMAT_INF_COMPAT: u32 = 2u32;
pub const MUI_FORMAT_REG_COMPAT: u32 = 1u32;
pub const MUI_FULL_LANGUAGE: u32 = 1u32;
pub const MUI_IMMUTABLE_LOOKUP: u32 = 16u32;
pub const MUI_LANGUAGE_EXACT: u32 = 16u32;
pub const MUI_LANGUAGE_ID: u32 = 4u32;
pub const MUI_LANGUAGE_INSTALLED: u32 = 32u32;
pub const MUI_LANGUAGE_LICENSED: u32 = 64u32;
pub const MUI_LANGUAGE_NAME: u32 = 8u32;
pub const MUI_LANG_NEUTRAL_PE_FILE: u32 = 256u32;
pub const MUI_LIP_LANGUAGE: u32 = 4u32;
pub const MUI_MACHINE_LANGUAGE_SETTINGS: u32 = 1024u32;
pub const MUI_MERGE_SYSTEM_FALLBACK: u32 = 16u32;
pub const MUI_MERGE_USER_FALLBACK: u32 = 32u32;
pub const MUI_NON_LANG_NEUTRAL_FILE: u32 = 512u32;
pub const MUI_PARTIAL_LANGUAGE: u32 = 2u32;
pub const MUI_QUERY_CHECKSUM: u32 = 2u32;
pub const MUI_QUERY_LANGUAGE_NAME: u32 = 4u32;
pub const MUI_QUERY_RESOURCE_TYPES: u32 = 8u32;
pub const MUI_QUERY_TYPE: u32 = 1u32;
pub const MUI_RESET_FILTERS: u32 = 1u32;
pub const MUI_SKIP_STRING_CACHE: u32 = 8u32;
pub const MUI_THREAD_LANGUAGES: u32 = 64u32;
pub const MUI_USER_PREFERRED_UI_LANGUAGES: u32 = 16u32;
pub const MUI_USE_INSTALLED_LANGUAGES: u32 = 32u32;
pub const MUI_USE_SEARCH_ALL_LANGUAGES: u32 = 64u32;
pub const MUI_VERIFY_FILE_EXISTS: u32 = 4u32;
pub struct MULTI_BYTE_TO_WIDE_CHAR_FLAGS(i32);
#[cfg(feature = "Win32_Graphics_Gdi")]
pub struct NEWTEXTMETRICEXA(i32);
#[cfg(feature = "Win32_Graphics_Gdi")]
pub struct NEWTEXTMETRICEXW(i32);
pub struct NLSVERSIONINFO(i32);
pub struct NLSVERSIONINFOEX(i32);
pub const NLS_CP_CPINFO: u32 = 268435456u32;
pub const NLS_CP_MBTOWC: u32 = 1073741824u32;
pub const NLS_CP_WCTOMB: u32 = 2147483648u32;
pub struct NORM_FORM(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct NUMBERFMTA(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct NUMBERFMTW(i32);
pub const NUMSYS_NAME_CAPACITY: u32 = 8u32;
pub const OFFLINE_SERVICES: u32 = 2u32;
pub const ONLINE_SERVICES: u32 = 1u32;
pub struct PFN_MAPPINGCALLBACKPROC(i32);
pub struct RFC1766INFO(i32);
pub struct SCRIPTCONTF(i32);
pub struct SCRIPTFONTCONTF(i32);
pub struct SCRIPTINFO(i32);
pub struct SCRIPT_ANALYSIS(i32);
pub struct SCRIPT_CONTROL(i32);
pub struct SCRIPT_DIGITSUBSTITUTE(i32);
pub const SCRIPT_DIGITSUBSTITUTE_CONTEXT: u32 = 0u32;
pub const SCRIPT_DIGITSUBSTITUTE_NATIONAL: u32 = 2u32;
pub const SCRIPT_DIGITSUBSTITUTE_NONE: u32 = 1u32;
pub const SCRIPT_DIGITSUBSTITUTE_TRADITIONAL: u32 = 3u32;
pub struct SCRIPT_FONTPROPERTIES(i32);
pub struct SCRIPT_IS_COMPLEX_FLAGS(i32);
pub struct SCRIPT_ITEM(i32);
pub struct SCRIPT_JUSTIFY(i32);
pub struct SCRIPT_LOGATTR(i32);
pub struct SCRIPT_PROPERTIES(i32);
pub struct SCRIPT_STATE(i32);
pub struct SCRIPT_TABDEF(i32);
pub const SCRIPT_TAG_UNKNOWN: u32 = 0u32;
pub const SCRIPT_UNDEFINED: u32 = 0u32;
pub struct SCRIPT_VISATTR(i32);
pub const SGCM_RTL: u32 = 1u32;
pub const SORTING_PARADIGM_ICU: u32 = 16777216u32;
pub const SORTING_PARADIGM_NLS: u32 = 0u32;
pub const SSA_BREAK: u32 = 64u32;
pub const SSA_CLIP: u32 = 4u32;
pub const SSA_DONTGLYPH: u32 = 1073741824u32;
pub const SSA_DZWG: u32 = 16u32;
pub const SSA_FALLBACK: u32 = 32u32;
pub const SSA_FIT: u32 = 8u32;
pub const SSA_FULLMEASURE: u32 = 67108864u32;
pub const SSA_GCP: u32 = 512u32;
pub const SSA_GLYPHS: u32 = 128u32;
pub const SSA_HIDEHOTKEY: u32 = 8192u32;
pub const SSA_HOTKEY: u32 = 1024u32;
pub const SSA_HOTKEYONLY: u32 = 9216u32;
pub const SSA_LAYOUTRTL: u32 = 536870912u32;
pub const SSA_LINK: u32 = 4096u32;
pub const SSA_LPKANSIFALLBACK: u32 = 134217728u32;
pub const SSA_METAFILE: u32 = 2048u32;
pub const SSA_NOKASHIDA: u32 = 2147483648u32;
pub const SSA_PASSWORD: u32 = 1u32;
pub const SSA_PIDX: u32 = 268435456u32;
pub const SSA_RTL: u32 = 256u32;
pub const SSA_TAB: u32 = 2u32;
pub struct SYSGEOCLASS(i32);
pub struct SYSGEOTYPE(i32);
pub struct SYSNLS_FUNCTION(i32);
pub struct SpellCheckerFactory(i32);
pub struct TIMEFMT_ENUMPROCA(i32);
pub struct TIMEFMT_ENUMPROCEX(i32);
pub struct TIMEFMT_ENUMPROCW(i32);
pub struct TIME_FORMAT_FLAGS(i32);
pub struct TRANSLATE_CHARSET_INFO_FLAGS(i32);
pub const U16_MAX_LENGTH: u32 = 2u32;
pub const U8_MAX_LENGTH: u32 = 4u32;
pub struct UAcceptResult(i32);
pub struct UAlphabeticIndexLabelType(i32);
pub const UBIDI_DEFAULT_LTR: u32 = 254u32;
pub const UBIDI_DEFAULT_RTL: u32 = 255u32;
pub const UBIDI_DO_MIRRORING: u32 = 2u32;
pub const UBIDI_INSERT_LRM_FOR_NUMERIC: u32 = 4u32;
pub const UBIDI_KEEP_BASE_COMBINING: u32 = 1u32;
pub const UBIDI_LEVEL_OVERRIDE: u32 = 128u32;
pub const UBIDI_MAP_NOWHERE: i32 = -1i32;
pub const UBIDI_MAX_EXPLICIT_LEVEL: u32 = 125u32;
pub const UBIDI_OUTPUT_REVERSE: u32 = 16u32;
pub const UBIDI_REMOVE_BIDI_CONTROLS: u32 = 8u32;
pub struct UBiDi(i32);
pub struct UBiDiClassCallback(i32);
pub struct UBiDiDirection(i32);
pub struct UBiDiMirroring(i32);
pub struct UBiDiOrder(i32);
pub struct UBiDiReorderingMode(i32);
pub struct UBiDiReorderingOption(i32);
pub struct UBiDiTransform(i32);
pub struct UBidiPairedBracketType(i32);
pub struct UBlockCode(i32);
pub struct UBreakIterator(i32);
pub struct UBreakIteratorType(i32);
pub const UCHAR_MAX_VALUE: u32 = 1114111u32;
pub const UCHAR_MIN_VALUE: u32 = 0u32;
pub const UCLN_NO_AUTO_CLEANUP: u32 = 1u32;
pub const UCNV_MAX_CONVERTER_NAME_LENGTH: u32 = 60u32;
pub const UCNV_SI: u32 = 15u32;
pub const UCNV_SO: u32 = 14u32;
pub const UCONFIG_ENABLE_PLUGINS: u32 = 0u32;
pub struct UCPMap(i32);
pub struct UCPMapRangeOption(i32);
pub struct UCPMapValueFilter(i32);
pub const UCPTRIE_ERROR_VALUE_NEG_DATA_OFFSET: i32 = 1i32;
pub const UCPTRIE_FAST_DATA_BLOCK_LENGTH: i32 = 64i32;
pub const UCPTRIE_FAST_DATA_MASK: i32 = 63i32;
pub const UCPTRIE_FAST_SHIFT: i32 = 6i32;
pub const UCPTRIE_HIGH_VALUE_NEG_DATA_OFFSET: i32 = 2i32;
pub const UCPTRIE_SMALL_MAX: i32 = 4095i32;
pub struct UCPTrie(i32);
pub struct UCPTrieData(i32);
pub struct UCPTrieType(i32);
pub struct UCPTrieValueWidth(i32);
pub struct UCalendarAMPMs(i32);
pub struct UCalendarAttribute(i32);
pub struct UCalendarDateFields(i32);
pub struct UCalendarDaysOfWeek(i32);
pub struct UCalendarDisplayNameType(i32);
pub struct UCalendarLimitType(i32);
pub struct UCalendarMonths(i32);
pub struct UCalendarType(i32);
pub struct UCalendarWallTimeOption(i32);
pub struct UCalendarWeekdayType(i32);
pub struct UCaseMap(i32);
pub struct UCharCategory(i32);
pub struct UCharDirection(i32);
pub struct UCharEnumTypeRange(i32);
pub struct UCharIterator(i32);
pub struct UCharIteratorCurrent(i32);
pub struct UCharIteratorGetIndex(i32);
pub struct UCharIteratorGetState(i32);
pub struct UCharIteratorHasNext(i32);
pub struct UCharIteratorHasPrevious(i32);
pub struct UCharIteratorMove(i32);
pub struct UCharIteratorNext(i32);
pub struct UCharIteratorOrigin(i32);
pub struct UCharIteratorPrevious(i32);
pub struct UCharIteratorReserved(i32);
pub struct UCharIteratorSetState(i32);
pub struct UCharNameChoice(i32);
pub struct UCharsetDetector(i32);
pub struct UCharsetMatch(i32);
pub struct UColAttribute(i32);
pub struct UColAttributeValue(i32);
pub struct UColBoundMode(i32);
pub struct UColReorderCode(i32);
pub struct UColRuleOption(i32);
pub struct UCollationElements(i32);
pub struct UCollationResult(i32);
pub struct UCollator(i32);
pub struct UConstrainedFieldPosition(i32);
pub struct UConverter(i32);
pub struct UConverterCallbackReason(i32);
pub struct UConverterFromUCallback(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct UConverterFromUnicodeArgs(i32);
pub struct UConverterPlatform(i32);
pub struct UConverterSelector(i32);
pub struct UConverterToUCallback(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct UConverterToUnicodeArgs(i32);
pub struct UConverterType(i32);
pub struct UConverterUnicodeSet(i32);
pub struct UCurrCurrencyType(i32);
pub struct UCurrNameStyle(i32);
pub struct UCurrencySpacing(i32);
pub struct UCurrencyUsage(i32);
pub struct UDateAbsoluteUnit(i32);
pub struct UDateDirection(i32);
pub struct UDateFormatBooleanAttribute(i32);
pub struct UDateFormatField(i32);
pub struct UDateFormatStyle(i32);
pub struct UDateFormatSymbolType(i32);
pub struct UDateFormatSymbols(i32);
pub struct UDateIntervalFormat(i32);
pub struct UDateRelativeDateTimeFormatterStyle(i32);
pub struct UDateRelativeUnit(i32);
pub struct UDateTimePGDisplayWidth(i32);
pub struct UDateTimePatternConflict(i32);
pub struct UDateTimePatternField(i32);
pub struct UDateTimePatternMatchOptions(i32);
pub struct UDateTimeScale(i32);
pub struct UDecompositionType(i32);
pub struct UDialectHandling(i32);
pub struct UDisplayContext(i32);
pub struct UDisplayContextType(i32);
pub struct UEastAsianWidth(i32);
pub struct UEnumCharNamesFn(i32);
pub struct UEnumeration(i32);
pub struct UErrorCode(i32);
pub struct UFieldCategory(i32);
pub struct UFieldPosition(i32);
pub struct UFieldPositionIterator(i32);
pub struct UFormattableType(i32);
pub struct UFormattedDateInterval(i32);
pub struct UFormattedList(i32);
pub struct UFormattedNumber(i32);
pub struct UFormattedNumberRange(i32);
pub struct UFormattedRelativeDateTime(i32);
pub struct UFormattedValue(i32);
pub struct UGender(i32);
pub struct UGenderInfo(i32);
pub struct UGraphemeClusterBreak(i32);
pub struct UHangulSyllableType(i32);
pub struct UHashtable(i32);
pub struct UIDNA(i32);
pub struct UIDNAInfo(i32);
pub const UIDNA_CHECK_BIDI: i32 = 4i32;
pub const UIDNA_CHECK_CONTEXTJ: i32 = 8i32;
pub const UIDNA_CHECK_CONTEXTO: i32 = 64i32;
pub const UIDNA_DEFAULT: i32 = 0i32;
pub const UIDNA_ERROR_BIDI: i32 = 2048i32;
pub const UIDNA_ERROR_CONTEXTJ: i32 = 4096i32;
pub const UIDNA_ERROR_CONTEXTO_DIGITS: i32 = 16384i32;
pub const UIDNA_ERROR_CONTEXTO_PUNCTUATION: i32 = 8192i32;
pub const UIDNA_ERROR_DISALLOWED: i32 = 128i32;
pub const UIDNA_ERROR_DOMAIN_NAME_TOO_LONG: i32 = 4i32;
pub const UIDNA_ERROR_EMPTY_LABEL: i32 = 1i32;
pub const UIDNA_ERROR_HYPHEN_3_4: i32 = 32i32;
pub const UIDNA_ERROR_INVALID_ACE_LABEL: i32 = 1024i32;
pub const UIDNA_ERROR_LABEL_HAS_DOT: i32 = 512i32;
pub const UIDNA_ERROR_LABEL_TOO_LONG: i32 = 2i32;
pub const UIDNA_ERROR_LEADING_COMBINING_MARK: i32 = 64i32;
pub const UIDNA_ERROR_LEADING_HYPHEN: i32 = 8i32;
pub const UIDNA_ERROR_PUNYCODE: i32 = 256i32;
pub const UIDNA_ERROR_TRAILING_HYPHEN: i32 = 16i32;
pub const UIDNA_NONTRANSITIONAL_TO_ASCII: i32 = 16i32;
pub const UIDNA_NONTRANSITIONAL_TO_UNICODE: i32 = 32i32;
pub const UIDNA_USE_STD3_RULES: i32 = 2i32;
pub struct UILANGUAGE_ENUMPROCA(i32);
pub struct UILANGUAGE_ENUMPROCW(i32);
pub const UITER_UNKNOWN_INDEX: i32 = -2i32;
pub struct UIndicPositionalCategory(i32);
pub struct UIndicSyllabicCategory(i32);
pub struct UJoiningGroup(i32);
pub struct UJoiningType(i32);
pub const ULOC_COUNTRY_CAPACITY: u32 = 4u32;
pub const ULOC_FULLNAME_CAPACITY: u32 = 157u32;
pub const ULOC_KEYWORDS_CAPACITY: u32 = 96u32;
pub const ULOC_KEYWORD_AND_VALUES_CAPACITY: u32 = 100u32;
pub const ULOC_KEYWORD_ASSIGN_UNICODE: u32 = 61u32;
pub const ULOC_KEYWORD_ITEM_SEPARATOR_UNICODE: u32 = 59u32;
pub const ULOC_KEYWORD_SEPARATOR_UNICODE: u32 = 64u32;
pub const ULOC_LANG_CAPACITY: u32 = 12u32;
pub const ULOC_SCRIPT_CAPACITY: u32 = 6u32;
pub struct ULayoutType(i32);
pub struct ULineBreak(i32);
pub struct ULineBreakTag(i32);
pub struct UListFormatter(i32);
pub struct UListFormatterField(i32);
pub struct UListFormatterType(i32);
pub struct UListFormatterWidth(i32);
pub struct ULocAvailableType(i32);
pub struct ULocDataLocaleType(i32);
pub struct ULocaleData(i32);
pub struct ULocaleDataDelimiterType(i32);
pub struct ULocaleDataExemplarSetType(i32);
pub struct ULocaleDisplayNames(i32);
pub const UMSGPAT_ARG_NAME_NOT_NUMBER: i32 = -1i32;
pub const UMSGPAT_ARG_NAME_NOT_VALID: i32 = -2i32;
pub struct UMeasureFormatWidth(i32);
pub struct UMeasurementSystem(i32);
pub struct UMemAllocFn(i32);
pub struct UMemFreeFn(i32);
pub struct UMemReallocFn(i32);
pub struct UMessagePatternApostropheMode(i32);
pub struct UMessagePatternArgType(i32);
pub struct UMessagePatternPartType(i32);
pub struct UMutableCPTrie(i32);
pub struct UNESCAPE_CHAR_AT(i32);
pub struct UNICODERANGE(i32);
pub const UNISCRIBE_OPENTYPE: u32 = 256u32;
pub const UNORM_INPUT_IS_FCD: u32 = 131072u32;
pub struct UNormalization2Mode(i32);
pub struct UNormalizationCheckResult(i32);
pub struct UNormalizationMode(i32);
pub struct UNormalizer2(i32);
pub struct UNumberCompactStyle(i32);
pub struct UNumberDecimalSeparatorDisplay(i32);
pub struct UNumberFormatAttribute(i32);
pub struct UNumberFormatAttributeValue(i32);
pub struct UNumberFormatFields(i32);
pub struct UNumberFormatPadPosition(i32);
pub struct UNumberFormatRoundingMode(i32);
pub struct UNumberFormatStyle(i32);
pub struct UNumberFormatSymbol(i32);
pub struct UNumberFormatTextAttribute(i32);
pub struct UNumberFormatter(i32);
pub struct UNumberGroupingStrategy(i32);
pub struct UNumberRangeCollapse(i32);
pub struct UNumberRangeIdentityFallback(i32);
pub struct UNumberRangeIdentityResult(i32);
pub struct UNumberSignDisplay(i32);
pub struct UNumberUnitWidth(i32);
pub struct UNumberingSystem(i32);
pub struct UNumericType(i32);
pub struct UParseError(i32);
pub struct UPluralRules(i32);
pub struct UPluralType(i32);
pub struct UProperty(i32);
pub struct UPropertyNameChoice(i32);
pub struct URegexFindProgressCallback(i32);
pub struct URegexMatchCallback(i32);
pub struct URegexpFlag(i32);
pub struct URegion(i32);
pub struct URegionType(i32);
pub struct URegularExpression(i32);
pub struct URelativeDateTimeFormatter(i32);
pub struct URelativeDateTimeFormatterField(i32);
pub struct URelativeDateTimeUnit(i32);
pub struct UReplaceableCallbacks(i32);
pub struct UResType(i32);
pub struct UResourceBundle(i32);
pub struct URestrictionLevel(i32);
pub const USEARCH_DONE: i32 = -1i32;
pub const USET_ADD_CASE_MAPPINGS: i32 = 4i32;
pub const USET_CASE_INSENSITIVE: i32 = 2i32;
pub const USET_IGNORE_SPACE: i32 = 1i32;
pub const USET_SERIALIZED_STATIC_ARRAY_CAPACITY: i32 = 8i32;
pub const USPREP_ALLOW_UNASSIGNED: u32 = 1u32;
pub const USPREP_DEFAULT: u32 = 0u32;
pub const USP_E_SCRIPT_NOT_IN_FONT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220992i32 as _);
pub struct UScriptCode(i32);
pub struct UScriptUsage(i32);
pub struct USearch(i32);
pub struct USearchAttribute(i32);
pub struct USearchAttributeValue(i32);
pub struct USentenceBreak(i32);
pub struct USentenceBreakTag(i32);
pub struct USerializedSet(i32);
pub struct USet(i32);
pub struct USetSpanCondition(i32);
pub struct USpoofCheckResult(i32);
pub struct USpoofChecker(i32);
pub struct USpoofChecks(i32);
pub struct UStringCaseMapper(i32);
pub struct UStringPrepProfile(i32);
pub struct UStringPrepProfileType(i32);
pub struct UStringSearch(i32);
pub struct UStringTrieBuildOption(i32);
pub struct UStringTrieResult(i32);
pub struct USystemTimeZoneType(i32);
pub const UTEXT_MAGIC: i32 = 878368812i32;
pub const UTEXT_PROVIDER_HAS_META_DATA: i32 = 4i32;
pub const UTEXT_PROVIDER_LENGTH_IS_EXPENSIVE: i32 = 1i32;
pub const UTEXT_PROVIDER_OWNS_TEXT: i32 = 5i32;
pub const UTEXT_PROVIDER_STABLE_CHUNKS: i32 = 2i32;
pub const UTEXT_PROVIDER_WRITABLE: i32 = 3i32;
pub const UTF16_MAX_CHAR_LENGTH: u32 = 2u32;
pub const UTF32_MAX_CHAR_LENGTH: u32 = 1u32;
pub const UTF8_ERROR_VALUE_1: u32 = 21u32;
pub const UTF8_ERROR_VALUE_2: u32 = 159u32;
pub const UTF8_MAX_CHAR_LENGTH: u32 = 4u32;
pub const UTF_ERROR_VALUE: u32 = 65535u32;
pub const UTF_MAX_CHAR_LENGTH: u32 = 2u32;
pub const UTF_SIZE: u32 = 16u32;
pub struct UText(i32);
pub struct UTextAccess(i32);
pub struct UTextClone(i32);
pub struct UTextClose(i32);
pub struct UTextCopy(i32);
pub struct UTextExtract(i32);
pub struct UTextFuncs(i32);
pub struct UTextMapNativeIndexToUTF16(i32);
pub struct UTextMapOffsetToNative(i32);
pub struct UTextNativeLength(i32);
pub struct UTextReplace(i32);
pub struct UTimeScaleValue(i32);
pub struct UTimeZoneFormatGMTOffsetPatternType(i32);
pub struct UTimeZoneFormatParseOption(i32);
pub struct UTimeZoneFormatStyle(i32);
pub struct UTimeZoneFormatTimeType(i32);
pub struct UTimeZoneNameType(i32);
pub struct UTimeZoneTransitionType(i32);
pub struct UTraceData(i32);
pub struct UTraceEntry(i32);
pub struct UTraceExit(i32);
pub struct UTraceFunctionNumber(i32);
pub struct UTraceLevel(i32);
pub struct UTransDirection(i32);
pub struct UTransPosition(i32);
pub struct UVerticalOrientation(i32);
pub struct UWordBreak(i32);
pub struct UWordBreakValues(i32);
pub const U_ASCII_FAMILY: u32 = 0u32;
pub const U_CHECK_DYLOAD: u32 = 1u32;
pub const U_COMBINED_IMPLEMENTATION: u32 = 1u32;
pub const U_COMPARE_CODE_POINT_ORDER: u32 = 32768u32;
pub const U_COMPARE_IGNORE_CASE: u32 = 65536u32;
pub const U_COPYRIGHT_STRING_LENGTH: u32 = 128u32;
pub const U_DEFAULT_SHOW_DRAFT: u32 = 0u32;
pub const U_DEFINE_FALSE_AND_TRUE: u32 = 1u32;
pub const U_DISABLE_RENAMING: u32 = 1u32;
pub const U_EBCDIC_FAMILY: u32 = 1u32;
pub const U_EDITS_NO_RESET: u32 = 8192u32;
pub const U_ENABLE_DYLOAD: u32 = 1u32;
pub const U_ENABLE_TRACING: u32 = 0u32;
pub const U_FOLD_CASE_DEFAULT: u32 = 0u32;
pub const U_FOLD_CASE_EXCLUDE_SPECIAL_I: u32 = 1u32;
pub const U_HAVE_RBNF: u32 = 0u32;
pub const U_HAVE_STD_STRING: u32 = 0u32;
pub const U_HIDE_DEPRECATED_API: u32 = 1u32;
pub const U_HIDE_DRAFT_API: u32 = 1u32;
pub const U_HIDE_INTERNAL_API: u32 = 1u32;
pub const U_HIDE_OBSOLETE_API: u32 = 1u32;
pub const U_IOSTREAM_SOURCE: u32 = 199711u32;
pub const U_MAX_VERSION_LENGTH: u32 = 4u32;
pub const U_MAX_VERSION_STRING_LENGTH: u32 = 20u32;
pub const U_MILLIS_PER_DAY: u32 = 86400000u32;
pub const U_MILLIS_PER_HOUR: u32 = 3600000u32;
pub const U_MILLIS_PER_MINUTE: u32 = 60000u32;
pub const U_MILLIS_PER_SECOND: u32 = 1000u32;
pub const U_NO_DEFAULT_INCLUDE_UTF_HEADERS: u32 = 1u32;
pub const U_OMIT_UNCHANGED_TEXT: u32 = 16384u32;
pub const U_OVERRIDE_CXX_ALLOCATION: u32 = 1u32;
pub const U_PARSE_CONTEXT_LEN: i32 = 16i32;
pub const U_PF_AIX: u32 = 3100u32;
pub const U_PF_ANDROID: u32 = 4050u32;
pub const U_PF_BROWSER_NATIVE_CLIENT: u32 = 4020u32;
pub const U_PF_BSD: u32 = 3000u32;
pub const U_PF_CYGWIN: u32 = 1900u32;
pub const U_PF_DARWIN: u32 = 3500u32;
pub const U_PF_EMSCRIPTEN: u32 = 5010u32;
pub const U_PF_FUCHSIA: u32 = 4100u32;
pub const U_PF_HPUX: u32 = 2100u32;
pub const U_PF_IPHONE: u32 = 3550u32;
pub const U_PF_IRIX: u32 = 3200u32;
pub const U_PF_LINUX: u32 = 4000u32;
pub const U_PF_MINGW: u32 = 1800u32;
pub const U_PF_OS390: u32 = 9000u32;
pub const U_PF_OS400: u32 = 9400u32;
pub const U_PF_QNX: u32 = 3700u32;
pub const U_PF_SOLARIS: u32 = 2600u32;
pub const U_PF_UNKNOWN: u32 = 0u32;
pub const U_PF_WINDOWS: u32 = 1000u32;
pub const U_SENTINEL: i32 = -1i32;
pub const U_SHAPE_AGGREGATE_TASHKEEL: u32 = 16384u32;
pub const U_SHAPE_AGGREGATE_TASHKEEL_MASK: u32 = 16384u32;
pub const U_SHAPE_AGGREGATE_TASHKEEL_NOOP: u32 = 0u32;
pub const U_SHAPE_DIGITS_ALEN2AN_INIT_AL: u32 = 128u32;
pub const U_SHAPE_DIGITS_ALEN2AN_INIT_LR: u32 = 96u32;
pub const U_SHAPE_DIGITS_AN2EN: u32 = 64u32;
pub const U_SHAPE_DIGITS_EN2AN: u32 = 32u32;
pub const U_SHAPE_DIGITS_MASK: u32 = 224u32;
pub const U_SHAPE_DIGITS_NOOP: u32 = 0u32;
pub const U_SHAPE_DIGITS_RESERVED: u32 = 160u32;
pub const U_SHAPE_DIGIT_TYPE_AN: u32 = 0u32;
pub const U_SHAPE_DIGIT_TYPE_AN_EXTENDED: u32 = 256u32;
pub const U_SHAPE_DIGIT_TYPE_MASK: u32 = 768u32;
pub const U_SHAPE_DIGIT_TYPE_RESERVED: u32 = 512u32;
pub const U_SHAPE_LAMALEF_AUTO: u32 = 65536u32;
pub const U_SHAPE_LAMALEF_BEGIN: u32 = 3u32;
pub const U_SHAPE_LAMALEF_END: u32 = 2u32;
pub const U_SHAPE_LAMALEF_MASK: u32 = 65539u32;
pub const U_SHAPE_LAMALEF_NEAR: u32 = 1u32;
pub const U_SHAPE_LAMALEF_RESIZE: u32 = 0u32;
pub const U_SHAPE_LENGTH_FIXED_SPACES_AT_BEGINNING: u32 = 3u32;
pub const U_SHAPE_LENGTH_FIXED_SPACES_AT_END: u32 = 2u32;
pub const U_SHAPE_LENGTH_FIXED_SPACES_NEAR: u32 = 1u32;
pub const U_SHAPE_LENGTH_GROW_SHRINK: u32 = 0u32;
pub const U_SHAPE_LENGTH_MASK: u32 = 65539u32;
pub const U_SHAPE_LETTERS_MASK: u32 = 24u32;
pub const U_SHAPE_LETTERS_NOOP: u32 = 0u32;
pub const U_SHAPE_LETTERS_SHAPE: u32 = 8u32;
pub const U_SHAPE_LETTERS_SHAPE_TASHKEEL_ISOLATED: u32 = 24u32;
pub const U_SHAPE_LETTERS_UNSHAPE: u32 = 16u32;
pub const U_SHAPE_PRESERVE_PRESENTATION: u32 = 32768u32;
pub const U_SHAPE_PRESERVE_PRESENTATION_MASK: u32 = 32768u32;
pub const U_SHAPE_PRESERVE_PRESENTATION_NOOP: u32 = 0u32;
pub const U_SHAPE_SEEN_MASK: u32 = 7340032u32;
pub const U_SHAPE_SEEN_TWOCELL_NEAR: u32 = 2097152u32;
pub const U_SHAPE_SPACES_RELATIVE_TO_TEXT_BEGIN_END: u32 = 67108864u32;
pub const U_SHAPE_SPACES_RELATIVE_TO_TEXT_MASK: u32 = 67108864u32;
pub const U_SHAPE_TAIL_NEW_UNICODE: u32 = 134217728u32;
pub const U_SHAPE_TAIL_TYPE_MASK: u32 = 134217728u32;
pub const U_SHAPE_TASHKEEL_BEGIN: u32 = 262144u32;
pub const U_SHAPE_TASHKEEL_END: u32 = 393216u32;
pub const U_SHAPE_TASHKEEL_MASK: u32 = 917504u32;
pub const U_SHAPE_TASHKEEL_REPLACE_BY_TATWEEL: u32 = 786432u32;
pub const U_SHAPE_TASHKEEL_RESIZE: u32 = 524288u32;
pub const U_SHAPE_TEXT_DIRECTION_LOGICAL: u32 = 0u32;
pub const U_SHAPE_TEXT_DIRECTION_MASK: u32 = 4u32;
pub const U_SHAPE_TEXT_DIRECTION_VISUAL_LTR: u32 = 4u32;
pub const U_SHAPE_TEXT_DIRECTION_VISUAL_RTL: u32 = 0u32;
pub const U_SHAPE_YEHHAMZA_MASK: u32 = 58720256u32;
pub const U_SHAPE_YEHHAMZA_TWOCELL_NEAR: u32 = 16777216u32;
pub const U_SHOW_CPLUSPLUS_API: u32 = 0u32;
pub const U_SIZEOF_UCHAR: u32 = 2u32;
pub const U_TITLECASE_ADJUST_TO_CASED: u32 = 1024u32;
pub const U_TITLECASE_NO_BREAK_ADJUSTMENT: u32 = 512u32;
pub const U_TITLECASE_NO_LOWERCASE: u32 = 256u32;
pub const U_TITLECASE_SENTENCES: u32 = 64u32;
pub const U_TITLECASE_WHOLE_STRING: u32 = 32u32;
pub const VS_ALLOW_LATIN: u32 = 1u32;
pub const WC_COMPOSITECHECK: u32 = 512u32;
pub const WC_DEFAULTCHAR: u32 = 64u32;
pub const WC_DISCARDNS: u32 = 16u32;
pub const WC_ERR_INVALID_CHARS: u32 = 128u32;
pub const WC_NO_BEST_FIT_CHARS: u32 = 1024u32;
pub const WC_SEPCHARS: u32 = 32u32;
pub struct WORDLIST_TYPE(i32);
pub struct opentype_feature_record(i32);
pub struct script_charprop(i32);
pub struct script_glyphprop(i32);
pub struct tagMLCONVCHARF(i32);
pub struct tagMLCPF(i32);
pub struct tagSCRIPFONTINFO(i32);
pub struct textrange_properties(i32);
