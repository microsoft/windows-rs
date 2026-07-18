#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn CompareStringA(locale : super::LCID, dwcmpflags : u32, lpstring1 : *const i8, cchcount1 : i32, lpstring2 : *const i8, cchcount2 : i32) -> i32);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn ConvertDefaultLocale(locale : super::LCID) -> super::LCID);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn EnumCalendarInfoA(lpcalinfoenumproc : CALINFO_ENUMPROCA, locale : super::LCID, calendar : CALID, caltype : CALTYPE) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn EnumCalendarInfoExA(lpcalinfoenumprocex : CALINFO_ENUMPROCEXA, locale : super::LCID, calendar : CALID, caltype : CALTYPE) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("kernel32.dll" "system" fn EnumCalendarInfoExEx(pcalinfoenumprocexex : CALINFO_ENUMPROCEXEX, lplocalename : windows_sys::core::PCWSTR, calendar : CALID, lpreserved : windows_sys::core::PCWSTR, caltype : CALTYPE, lparam : super::LPARAM) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn EnumCalendarInfoExW(lpcalinfoenumprocex : CALINFO_ENUMPROCEXW, locale : super::LCID, calendar : CALID, caltype : CALTYPE) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn EnumCalendarInfoW(lpcalinfoenumproc : CALINFO_ENUMPROCW, locale : super::LCID, calendar : CALID, caltype : CALTYPE) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn EnumDateFormatsA(lpdatefmtenumproc : DATEFMT_ENUMPROCA, locale : super::LCID, dwflags : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn EnumDateFormatsExA(lpdatefmtenumprocex : DATEFMT_ENUMPROCEXA, locale : super::LCID, dwflags : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("kernel32.dll" "system" fn EnumDateFormatsExEx(lpdatefmtenumprocexex : DATEFMT_ENUMPROCEXEX, lplocalename : windows_sys::core::PCWSTR, dwflags : u32, lparam : super::LPARAM) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn EnumDateFormatsExW(lpdatefmtenumprocex : DATEFMT_ENUMPROCEXW, locale : super::LCID, dwflags : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn EnumDateFormatsW(lpdatefmtenumproc : DATEFMT_ENUMPROCW, locale : super::LCID, dwflags : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn EnumLanguageGroupLocalesA(lplanggrouplocaleenumproc : LANGGROUPLOCALE_ENUMPROCA, languagegroup : LGRPID, dwflags : u32, lparam : isize) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn EnumLanguageGroupLocalesW(lplanggrouplocaleenumproc : LANGGROUPLOCALE_ENUMPROCW, languagegroup : LGRPID, dwflags : u32, lparam : isize) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn EnumSystemCodePagesA(lpcodepageenumproc : CODEPAGE_ENUMPROCA, dwflags : u32) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn EnumSystemCodePagesW(lpcodepageenumproc : CODEPAGE_ENUMPROCW, dwflags : u32) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn EnumSystemGeoID(geoclass : GEOCLASS, parentgeoid : GEOID, lpgeoenumproc : GEO_ENUMPROC) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("kernel32.dll" "system" fn EnumSystemGeoNames(geoclass : GEOCLASS, geoenumproc : GEO_ENUMNAMEPROC, data : super::LPARAM) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn EnumSystemLanguageGroupsA(lplanguagegroupenumproc : LANGUAGEGROUP_ENUMPROCA, dwflags : u32, lparam : isize) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn EnumSystemLanguageGroupsW(lplanguagegroupenumproc : LANGUAGEGROUP_ENUMPROCW, dwflags : u32, lparam : isize) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn EnumSystemLocalesA(lplocaleenumproc : LOCALE_ENUMPROCA, dwflags : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("kernel32.dll" "system" fn EnumSystemLocalesEx(lplocaleenumprocex : LOCALE_ENUMPROCEX, dwflags : u32, lparam : super::LPARAM, lpreserved : *const core::ffi::c_void) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn EnumSystemLocalesW(lplocaleenumproc : LOCALE_ENUMPROCW, dwflags : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn EnumTimeFormatsA(lptimefmtenumproc : TIMEFMT_ENUMPROCA, locale : super::LCID, dwflags : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("kernel32.dll" "system" fn EnumTimeFormatsEx(lptimefmtenumprocex : TIMEFMT_ENUMPROCEX, lplocalename : windows_sys::core::PCWSTR, dwflags : u32, lparam : super::LPARAM) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn EnumTimeFormatsW(lptimefmtenumproc : TIMEFMT_ENUMPROCW, locale : super::LCID, dwflags : u32) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn EnumUILanguagesA(lpuilanguageenumproc : UILANGUAGE_ENUMPROCA, dwflags : u32, lparam : isize) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn EnumUILanguagesW(lpuilanguageenumproc : UILANGUAGE_ENUMPROCW, dwflags : u32, lparam : isize) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn FindNLSString(locale : super::LCID, dwfindnlsstringflags : u32, lpstringsource : windows_sys::core::PCWSTR, cchsource : i32, lpstringvalue : windows_sys::core::PCWSTR, cchvalue : i32, pcchfound : *mut i32) -> i32);
#[cfg(feature = "minwindef")]
windows_link::link!("kernel32.dll" "system" fn FindNLSStringEx(lplocalename : windows_sys::core::PCWSTR, dwfindnlsstringflags : u32, lpstringsource : windows_sys::core::PCWSTR, cchsource : i32, lpstringvalue : windows_sys::core::PCWSTR, cchvalue : i32, pcchfound : *mut i32, lpversioninformation : *const NLSVERSIONINFO, lpreserved : *const core::ffi::c_void, sorthandle : super::LPARAM) -> i32);
windows_link::link!("kernel32.dll" "system" fn FoldStringA(dwmapflags : u32, lpsrcstr : windows_sys::core::PCSTR, cchsrc : i32, lpdeststr : windows_sys::core::PSTR, cchdest : i32) -> i32);
windows_link::link!("kernel32.dll" "system" fn GetACP() -> u32);
windows_link::link!("kernel32.dll" "system" fn GetCPInfo(codepage : u32, lpcpinfo : *mut CPINFO) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn GetCPInfoExA(codepage : u32, dwflags : u32, lpcpinfoex : *mut CPINFOEXA) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn GetCPInfoExW(codepage : u32, dwflags : u32, lpcpinfoex : *mut CPINFOEXW) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn GetCalendarInfoA(locale : super::LCID, calendar : CALID, caltype : CALTYPE, lpcaldata : windows_sys::core::PSTR, cchdata : i32, lpvalue : *mut u32) -> i32);
windows_link::link!("kernel32.dll" "system" fn GetCalendarInfoEx(lplocalename : windows_sys::core::PCWSTR, calendar : CALID, lpreserved : windows_sys::core::PCWSTR, caltype : CALTYPE, lpcaldata : windows_sys::core::PWSTR, cchdata : i32, lpvalue : *mut u32) -> i32);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn GetCalendarInfoW(locale : super::LCID, calendar : CALID, caltype : CALTYPE, lpcaldata : windows_sys::core::PWSTR, cchdata : i32, lpvalue : *mut u32) -> i32);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn GetCurrencyFormatA(locale : super::LCID, dwflags : u32, lpvalue : windows_sys::core::PCSTR, lpformat : *const CURRENCYFMTA, lpcurrencystr : windows_sys::core::PSTR, cchcurrency : i32) -> i32);
windows_link::link!("kernel32.dll" "system" fn GetCurrencyFormatEx(lplocalename : windows_sys::core::PCWSTR, dwflags : u32, lpvalue : windows_sys::core::PCWSTR, lpformat : *const CURRENCYFMTW, lpcurrencystr : windows_sys::core::PWSTR, cchcurrency : i32) -> i32);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn GetCurrencyFormatW(locale : super::LCID, dwflags : u32, lpvalue : windows_sys::core::PCWSTR, lpformat : *const CURRENCYFMTW, lpcurrencystr : windows_sys::core::PWSTR, cchcurrency : i32) -> i32);
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn GetDurationFormat(locale : super::LCID, dwflags : u32, lpduration : *const super::SYSTEMTIME, ullduration : u64, lpformat : windows_sys::core::PCWSTR, lpdurationstr : windows_sys::core::PWSTR, cchduration : i32) -> i32);
windows_link::link!("kernel32.dll" "system" fn GetFileMUIInfo(dwflags : u32, pcwszfilepath : windows_sys::core::PCWSTR, pfilemuiinfo : *mut FILEMUIINFO, pcbfilemuiinfo : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn GetFileMUIPath(dwflags : u32, pcwszfilepath : windows_sys::core::PCWSTR, pwszlanguage : windows_sys::core::PWSTR, pcchlanguage : *mut u32, pwszfilemuipath : windows_sys::core::PWSTR, pcchfilemuipath : *mut u32, pululenumerator : *mut u64) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn GetGeoInfoA(location : GEOID, geotype : GEOTYPE, lpgeodata : windows_sys::core::PSTR, cchdata : i32, langid : super::LANGID) -> i32);
windows_link::link!("kernel32.dll" "system" fn GetGeoInfoEx(location : windows_sys::core::PCWSTR, geotype : GEOTYPE, geodata : windows_sys::core::PWSTR, geodatacount : i32) -> i32);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn GetGeoInfoW(location : GEOID, geotype : GEOTYPE, lpgeodata : windows_sys::core::PWSTR, cchdata : i32, langid : super::LANGID) -> i32);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn GetLocaleInfoA(locale : super::LCID, lctype : LCTYPE, lplcdata : windows_sys::core::PSTR, cchdata : i32) -> i32);
windows_link::link!("kernel32.dll" "system" fn GetLocaleInfoEx(lplocalename : windows_sys::core::PCWSTR, lctype : LCTYPE, lplcdata : windows_sys::core::PWSTR, cchdata : i32) -> i32);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn GetLocaleInfoW(locale : super::LCID, lctype : LCTYPE, lplcdata : windows_sys::core::PWSTR, cchdata : i32) -> i32);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn GetNLSVersion(function : NLS_FUNCTION, locale : super::LCID, lpversioninformation : *mut NLSVERSIONINFO) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn GetNLSVersionEx(function : NLS_FUNCTION, lplocalename : windows_sys::core::PCWSTR, lpversioninformation : *mut NLSVERSIONINFOEX) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn GetNumberFormatA(locale : super::LCID, dwflags : u32, lpvalue : windows_sys::core::PCSTR, lpformat : *const NUMBERFMTA, lpnumberstr : windows_sys::core::PSTR, cchnumber : i32) -> i32);
windows_link::link!("kernel32.dll" "system" fn GetNumberFormatEx(lplocalename : windows_sys::core::PCWSTR, dwflags : u32, lpvalue : windows_sys::core::PCWSTR, lpformat : *const NUMBERFMTW, lpnumberstr : windows_sys::core::PWSTR, cchnumber : i32) -> i32);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn GetNumberFormatW(locale : super::LCID, dwflags : u32, lpvalue : windows_sys::core::PCWSTR, lpformat : *const NUMBERFMTW, lpnumberstr : windows_sys::core::PWSTR, cchnumber : i32) -> i32);
windows_link::link!("kernel32.dll" "system" fn GetOEMCP() -> u32);
windows_link::link!("kernel32.dll" "system" fn GetProcessPreferredUILanguages(dwflags : u32, pulnumlanguages : *mut u32, pwszlanguagesbuffer : *mut u16, pcchlanguagesbuffer : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn GetStringScripts(dwflags : u32, lpstring : windows_sys::core::PCWSTR, cchstring : i32, lpscripts : windows_sys::core::PWSTR, cchscripts : i32) -> i32);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn GetStringTypeA(locale : super::LCID, dwinfotype : u32, lpsrcstr : windows_sys::core::PCSTR, cchsrc : i32, lpchartype : *mut u16) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn GetStringTypeExA(locale : super::LCID, dwinfotype : u32, lpsrcstr : windows_sys::core::PCSTR, cchsrc : i32, lpchartype : *mut u16) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn GetSystemDefaultLCID() -> super::LCID);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn GetSystemDefaultLangID() -> super::LANGID);
windows_link::link!("kernel32.dll" "system" fn GetSystemDefaultLocaleName(lplocalename : windows_sys::core::PWSTR, cchlocalename : i32) -> i32);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn GetSystemDefaultUILanguage() -> super::LANGID);
windows_link::link!("kernel32.dll" "system" fn GetSystemPreferredUILanguages(dwflags : u32, pulnumlanguages : *mut u32, pwszlanguagesbuffer : *mut u16, pcchlanguagesbuffer : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn GetThreadLocale() -> super::LCID);
windows_link::link!("kernel32.dll" "system" fn GetThreadPreferredUILanguages(dwflags : u32, pulnumlanguages : *mut u32, pwszlanguagesbuffer : *mut u16, pcchlanguagesbuffer : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn GetThreadUILanguage() -> super::LANGID);
windows_link::link!("kernel32.dll" "system" fn GetUILanguageInfo(dwflags : u32, pwmszlanguage : *const u16, pwszfallbacklanguages : *mut u16, pcchfallbacklanguages : *mut u32, pattributes : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn GetUserDefaultGeoName(geoname : windows_sys::core::PWSTR, geonamecount : i32) -> i32);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn GetUserDefaultLCID() -> super::LCID);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn GetUserDefaultLangID() -> super::LANGID);
windows_link::link!("kernel32.dll" "system" fn GetUserDefaultLocaleName(lplocalename : windows_sys::core::PWSTR, cchlocalename : i32) -> i32);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn GetUserDefaultUILanguage() -> super::LANGID);
windows_link::link!("kernel32.dll" "system" fn GetUserGeoID(geoclass : GEOCLASS) -> GEOID);
windows_link::link!("kernel32.dll" "system" fn GetUserPreferredUILanguages(dwflags : u32, pulnumlanguages : *mut u32, pwszlanguagesbuffer : *mut u16, pcchlanguagesbuffer : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("normaliz.dll" "system" fn IdnToAscii(dwflags : u32, lpunicodecharstr : windows_sys::core::PCWSTR, cchunicodechar : i32, lpasciicharstr : windows_sys::core::PWSTR, cchasciichar : i32) -> i32);
windows_link::link!("normaliz.dll" "system" fn IdnToNameprepUnicode(dwflags : u32, lpunicodecharstr : windows_sys::core::PCWSTR, cchunicodechar : i32, lpnameprepcharstr : windows_sys::core::PWSTR, cchnameprepchar : i32) -> i32);
windows_link::link!("normaliz.dll" "system" fn IdnToUnicode(dwflags : u32, lpasciicharstr : windows_sys::core::PCWSTR, cchasciichar : i32, lpunicodecharstr : windows_sys::core::PWSTR, cchunicodechar : i32) -> i32);
windows_link::link!("kernel32.dll" "system" fn IsDBCSLeadByte(testchar : u8) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn IsDBCSLeadByteEx(codepage : u32, testchar : u8) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn IsNLSDefinedString(function : NLS_FUNCTION, dwflags : u32, lpversioninformation : *const NLSVERSIONINFO, lpstring : windows_sys::core::PCWSTR, cchstr : i32) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn IsNormalizedString(normform : NORM_FORM, lpstring : windows_sys::core::PCWSTR, cwlength : i32) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn IsValidCodePage(codepage : u32) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn IsValidLanguageGroup(languagegroup : LGRPID, dwflags : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn IsValidLocale(locale : super::LCID, dwflags : u32) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn IsValidLocaleName(lplocalename : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn IsValidNLSVersion(function : NLS_FUNCTION, lplocalename : windows_sys::core::PCWSTR, lpversioninformation : *const NLSVERSIONINFOEX) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn LCIDToLocaleName(locale : super::LCID, lpname : windows_sys::core::PWSTR, cchname : i32, dwflags : u32) -> i32);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn LCMapStringA(locale : super::LCID, dwmapflags : u32, lpsrcstr : windows_sys::core::PCSTR, cchsrc : i32, lpdeststr : windows_sys::core::PSTR, cchdest : i32) -> i32);
#[cfg(feature = "minwindef")]
windows_link::link!("kernel32.dll" "system" fn LCMapStringEx(lplocalename : windows_sys::core::PCWSTR, dwmapflags : u32, lpsrcstr : windows_sys::core::PCWSTR, cchsrc : i32, lpdeststr : windows_sys::core::PWSTR, cchdest : i32, lpversioninformation : *const NLSVERSIONINFO, lpreserved : *const core::ffi::c_void, sorthandle : super::LPARAM) -> i32);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn LCMapStringW(locale : super::LCID, dwmapflags : u32, lpsrcstr : windows_sys::core::PCWSTR, cchsrc : i32, lpdeststr : windows_sys::core::PWSTR, cchdest : i32) -> i32);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn LocaleNameToLCID(lpname : windows_sys::core::PCWSTR, dwflags : u32) -> super::LCID);
windows_link::link!("kernel32.dll" "system" fn NormalizeString(normform : NORM_FORM, lpsrcstring : windows_sys::core::PCWSTR, cwsrclength : i32, lpdststring : windows_sys::core::PWSTR, cwdstlength : i32) -> i32);
windows_link::link!("kernel32.dll" "system" fn NotifyUILanguageChange(dwflags : u32, pcwstrnewlanguage : windows_sys::core::PCWSTR, pcwstrpreviouslanguage : windows_sys::core::PCWSTR, dwreserved : u32, pdwstatusrtrn : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn ResolveLocaleName(lpnametoresolve : windows_sys::core::PCWSTR, lplocalename : windows_sys::core::PWSTR, cchlocalename : i32) -> i32);
windows_link::link!("kernel32.dll" "system" fn RestoreThreadPreferredUILanguages(snapshot : HSAVEDUILANGUAGES));
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn SetCalendarInfoA(locale : super::LCID, calendar : CALID, caltype : CALTYPE, lpcaldata : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn SetCalendarInfoW(locale : super::LCID, calendar : CALID, caltype : CALTYPE, lpcaldata : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn SetLocaleInfoA(locale : super::LCID, lctype : LCTYPE, lplcdata : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn SetLocaleInfoW(locale : super::LCID, lctype : LCTYPE, lplcdata : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn SetProcessPreferredUILanguages(dwflags : u32, pwszlanguagesbuffer : *const u16, pulnumlanguages : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn SetThreadLocale(locale : super::LCID) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn SetThreadPreferredUILanguages(dwflags : u32, pwszlanguagesbuffer : *const u16, pulnumlanguages : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn SetThreadPreferredUILanguages2(flags : u32, languages : *const u16, numlanguagesset : *mut u32, snapshot : *mut HSAVEDUILANGUAGES) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn SetThreadUILanguage(langid : super::LANGID) -> super::LANGID);
windows_link::link!("kernel32.dll" "system" fn SetUserGeoID(geoid : GEOID) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn SetUserGeoName(geoname : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn VerifyScripts(dwflags : u32, lplocalescripts : windows_sys::core::PCWSTR, cchlocalescripts : i32, lptestscripts : windows_sys::core::PCWSTR, cchtestscripts : i32) -> windows_sys::core::BOOL);
pub const C1_ALPHA: u32 = 256;
pub const C1_BLANK: u32 = 64;
pub const C1_CNTRL: u32 = 32;
pub const C1_DEFINED: u32 = 512;
pub const C1_DIGIT: u32 = 4;
pub const C1_LOWER: u32 = 2;
pub const C1_PUNCT: u32 = 16;
pub const C1_SPACE: u32 = 8;
pub const C1_UPPER: u32 = 1;
pub const C1_XDIGIT: u32 = 128;
pub const C2_ARABICNUMBER: u32 = 6;
pub const C2_BLOCKSEPARATOR: u32 = 8;
pub const C2_COMMONSEPARATOR: u32 = 7;
pub const C2_EUROPENUMBER: u32 = 3;
pub const C2_EUROPESEPARATOR: u32 = 4;
pub const C2_EUROPETERMINATOR: u32 = 5;
pub const C2_LEFTTORIGHT: u32 = 1;
pub const C2_NOTAPPLICABLE: u32 = 0;
pub const C2_OTHERNEUTRAL: u32 = 11;
pub const C2_RIGHTTOLEFT: u32 = 2;
pub const C2_SEGMENTSEPARATOR: u32 = 9;
pub const C2_WHITESPACE: u32 = 10;
pub const C3_ALPHA: u32 = 32768;
pub const C3_DIACRITIC: u32 = 2;
pub const C3_FULLWIDTH: u32 = 128;
pub const C3_HALFWIDTH: u32 = 64;
pub const C3_HIGHSURROGATE: u32 = 2048;
pub const C3_HIRAGANA: u32 = 32;
pub const C3_IDEOGRAPH: u32 = 256;
pub const C3_KASHIDA: u32 = 512;
pub const C3_KATAKANA: u32 = 16;
pub const C3_LEXICAL: u32 = 1024;
pub const C3_LOWSURROGATE: u32 = 4096;
pub const C3_NONSPACING: u32 = 1;
pub const C3_NOTAPPLICABLE: u32 = 0;
pub const C3_SYMBOL: u32 = 8;
pub const C3_VOWELMARK: u32 = 4;
pub type CALID = u32;
pub type CALINFO_ENUMPROCA = Option<unsafe extern "system" fn(param0: windows_sys::core::PCSTR) -> windows_sys::core::BOOL>;
pub type CALINFO_ENUMPROCEXA = Option<unsafe extern "system" fn(param0: windows_sys::core::PCSTR, param1: CALID) -> windows_sys::core::BOOL>;
#[cfg(feature = "minwindef")]
pub type CALINFO_ENUMPROCEXEX = Option<unsafe extern "system" fn(param0: windows_sys::core::PCWSTR, param1: CALID, param2: windows_sys::core::PCWSTR, param3: super::LPARAM) -> windows_sys::core::BOOL>;
pub type CALINFO_ENUMPROCEXW = Option<unsafe extern "system" fn(param0: windows_sys::core::PCWSTR, param1: CALID) -> windows_sys::core::BOOL>;
pub type CALINFO_ENUMPROCW = Option<unsafe extern "system" fn(param0: windows_sys::core::PCWSTR) -> windows_sys::core::BOOL>;
pub type CALTYPE = u32;
pub const CAL_GREGORIAN: u32 = 1;
pub const CAL_GREGORIAN_ARABIC: u32 = 10;
pub const CAL_GREGORIAN_ME_FRENCH: u32 = 9;
pub const CAL_GREGORIAN_US: u32 = 2;
pub const CAL_GREGORIAN_XLIT_ENGLISH: u32 = 11;
pub const CAL_GREGORIAN_XLIT_FRENCH: u32 = 12;
pub const CAL_HEBREW: u32 = 8;
pub const CAL_HIJRI: u32 = 6;
pub const CAL_ICALINTVALUE: u32 = 1;
pub const CAL_ITWODIGITYEARMAX: u32 = 48;
pub const CAL_IYEAROFFSETRANGE: u32 = 3;
pub const CAL_JAPAN: u32 = 3;
pub const CAL_KOREA: u32 = 5;
pub const CAL_NOUSEROVERRIDE: i32 = -2147483648;
pub const CAL_PERSIAN: u32 = 22;
pub const CAL_RETURN_GENITIVE_NAMES: u32 = 268435456;
pub const CAL_RETURN_NUMBER: u32 = 536870912;
pub const CAL_SABBREVDAYNAME1: u32 = 14;
pub const CAL_SABBREVDAYNAME2: u32 = 15;
pub const CAL_SABBREVDAYNAME3: u32 = 16;
pub const CAL_SABBREVDAYNAME4: u32 = 17;
pub const CAL_SABBREVDAYNAME5: u32 = 18;
pub const CAL_SABBREVDAYNAME6: u32 = 19;
pub const CAL_SABBREVDAYNAME7: u32 = 20;
pub const CAL_SABBREVERASTRING: u32 = 57;
pub const CAL_SABBREVMONTHNAME1: u32 = 34;
pub const CAL_SABBREVMONTHNAME10: u32 = 43;
pub const CAL_SABBREVMONTHNAME11: u32 = 44;
pub const CAL_SABBREVMONTHNAME12: u32 = 45;
pub const CAL_SABBREVMONTHNAME13: u32 = 46;
pub const CAL_SABBREVMONTHNAME2: u32 = 35;
pub const CAL_SABBREVMONTHNAME3: u32 = 36;
pub const CAL_SABBREVMONTHNAME4: u32 = 37;
pub const CAL_SABBREVMONTHNAME5: u32 = 38;
pub const CAL_SABBREVMONTHNAME6: u32 = 39;
pub const CAL_SABBREVMONTHNAME7: u32 = 40;
pub const CAL_SABBREVMONTHNAME8: u32 = 41;
pub const CAL_SABBREVMONTHNAME9: u32 = 42;
pub const CAL_SCALNAME: u32 = 2;
pub const CAL_SDAYNAME1: u32 = 7;
pub const CAL_SDAYNAME2: u32 = 8;
pub const CAL_SDAYNAME3: u32 = 9;
pub const CAL_SDAYNAME4: u32 = 10;
pub const CAL_SDAYNAME5: u32 = 11;
pub const CAL_SDAYNAME6: u32 = 12;
pub const CAL_SDAYNAME7: u32 = 13;
pub const CAL_SENGLISHABBREVERANAME: u32 = 60;
pub const CAL_SENGLISHERANAME: u32 = 59;
pub const CAL_SERASTRING: u32 = 4;
pub const CAL_SJAPANESEERAFIRSTYEAR: u32 = 61;
pub const CAL_SLONGDATE: u32 = 6;
pub const CAL_SMONTHDAY: u32 = 56;
pub const CAL_SMONTHNAME1: u32 = 21;
pub const CAL_SMONTHNAME10: u32 = 30;
pub const CAL_SMONTHNAME11: u32 = 31;
pub const CAL_SMONTHNAME12: u32 = 32;
pub const CAL_SMONTHNAME13: u32 = 33;
pub const CAL_SMONTHNAME2: u32 = 22;
pub const CAL_SMONTHNAME3: u32 = 23;
pub const CAL_SMONTHNAME4: u32 = 24;
pub const CAL_SMONTHNAME5: u32 = 25;
pub const CAL_SMONTHNAME6: u32 = 26;
pub const CAL_SMONTHNAME7: u32 = 27;
pub const CAL_SMONTHNAME8: u32 = 28;
pub const CAL_SMONTHNAME9: u32 = 29;
pub const CAL_SRELATIVELONGDATE: u32 = 58;
pub const CAL_SSHORTDATE: u32 = 5;
pub const CAL_SSHORTESTDAYNAME1: u32 = 49;
pub const CAL_SSHORTESTDAYNAME2: u32 = 50;
pub const CAL_SSHORTESTDAYNAME3: u32 = 51;
pub const CAL_SSHORTESTDAYNAME4: u32 = 52;
pub const CAL_SSHORTESTDAYNAME5: u32 = 53;
pub const CAL_SSHORTESTDAYNAME6: u32 = 54;
pub const CAL_SSHORTESTDAYNAME7: u32 = 55;
pub const CAL_SYEARMONTH: u32 = 47;
pub const CAL_TAIWAN: u32 = 4;
pub const CAL_THAI: u32 = 7;
pub const CAL_UMALQURA: u32 = 23;
pub const CAL_USE_CP_ACP: u32 = 1073741824;
pub type CODEPAGE_ENUMPROCA = Option<unsafe extern "system" fn(param0: windows_sys::core::PCSTR) -> windows_sys::core::BOOL>;
pub type CODEPAGE_ENUMPROCW = Option<unsafe extern "system" fn(param0: windows_sys::core::PCWSTR) -> windows_sys::core::BOOL>;
pub const COMPARE_STRING: SYSNLS_FUNCTION = 1;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CPINFO {
    pub MaxCharSize: u32,
    pub DefaultChar: [u8; 2],
    pub LeadByte: [u8; 12],
}
impl Default for CPINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type CPINFOEX = CPINFOEXA;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CPINFOEXA {
    pub MaxCharSize: u32,
    pub DefaultChar: [u8; 2],
    pub LeadByte: [u8; 12],
    pub UnicodeDefaultChar: u16,
    pub CodePage: u32,
    pub CodePageName: [i8; 260],
}
impl Default for CPINFOEXA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CPINFOEXW {
    pub MaxCharSize: u32,
    pub DefaultChar: [u8; 2],
    pub LeadByte: [u8; 12],
    pub UnicodeDefaultChar: u16,
    pub CodePage: u32,
    pub CodePageName: [u16; 260],
}
impl Default for CPINFOEXW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CP_ACP: u32 = 0;
pub const CP_INSTALLED: u32 = 1;
pub const CP_MACCP: u32 = 2;
pub const CP_OEMCP: u32 = 1;
pub const CP_SUPPORTED: u32 = 2;
pub const CP_SYMBOL: u32 = 42;
pub const CP_THREAD_ACP: u32 = 3;
pub const CP_UTF7: u32 = 65000;
pub const CP_UTF8: u32 = 65001;
pub const CSTR_EQUAL: u32 = 2;
pub const CSTR_GREATER_THAN: u32 = 3;
pub const CSTR_LESS_THAN: u32 = 1;
pub const CTRY_ALBANIA: u32 = 355;
pub const CTRY_ALGERIA: u32 = 213;
pub const CTRY_ARGENTINA: u32 = 54;
pub const CTRY_ARMENIA: u32 = 374;
pub const CTRY_AUSTRALIA: u32 = 61;
pub const CTRY_AUSTRIA: u32 = 43;
pub const CTRY_AZERBAIJAN: u32 = 994;
pub const CTRY_BAHRAIN: u32 = 973;
pub const CTRY_BELARUS: u32 = 375;
pub const CTRY_BELGIUM: u32 = 32;
pub const CTRY_BELIZE: u32 = 501;
pub const CTRY_BOLIVIA: u32 = 591;
pub const CTRY_BRAZIL: u32 = 55;
pub const CTRY_BRUNEI_DARUSSALAM: u32 = 673;
pub const CTRY_BULGARIA: u32 = 359;
pub const CTRY_CANADA: u32 = 2;
pub const CTRY_CARIBBEAN: u32 = 1;
pub const CTRY_CHILE: u32 = 56;
pub const CTRY_COLOMBIA: u32 = 57;
pub const CTRY_COSTA_RICA: u32 = 506;
pub const CTRY_CROATIA: u32 = 385;
pub const CTRY_CZECH: u32 = 420;
pub const CTRY_DEFAULT: u32 = 0;
pub const CTRY_DENMARK: u32 = 45;
pub const CTRY_DOMINICAN_REPUBLIC: u32 = 1;
pub const CTRY_ECUADOR: u32 = 593;
pub const CTRY_EGYPT: u32 = 20;
pub const CTRY_EL_SALVADOR: u32 = 503;
pub const CTRY_ESTONIA: u32 = 372;
pub const CTRY_FAEROE_ISLANDS: u32 = 298;
pub const CTRY_FINLAND: u32 = 358;
pub const CTRY_FRANCE: u32 = 33;
pub const CTRY_GEORGIA: u32 = 995;
pub const CTRY_GERMANY: u32 = 49;
pub const CTRY_GREECE: u32 = 30;
pub const CTRY_GUATEMALA: u32 = 502;
pub const CTRY_HONDURAS: u32 = 504;
pub const CTRY_HONG_KONG: u32 = 852;
pub const CTRY_HUNGARY: u32 = 36;
pub const CTRY_ICELAND: u32 = 354;
pub const CTRY_INDIA: u32 = 91;
pub const CTRY_INDONESIA: u32 = 62;
pub const CTRY_IRAN: u32 = 981;
pub const CTRY_IRAQ: u32 = 964;
pub const CTRY_IRELAND: u32 = 353;
pub const CTRY_ISRAEL: u32 = 972;
pub const CTRY_ITALY: u32 = 39;
pub const CTRY_JAMAICA: u32 = 1;
pub const CTRY_JAPAN: u32 = 81;
pub const CTRY_JORDAN: u32 = 962;
pub const CTRY_KAZAKSTAN: u32 = 7;
pub const CTRY_KENYA: u32 = 254;
pub const CTRY_KUWAIT: u32 = 965;
pub const CTRY_KYRGYZSTAN: u32 = 996;
pub const CTRY_LATVIA: u32 = 371;
pub const CTRY_LEBANON: u32 = 961;
pub const CTRY_LIBYA: u32 = 218;
pub const CTRY_LIECHTENSTEIN: u32 = 41;
pub const CTRY_LITHUANIA: u32 = 370;
pub const CTRY_LUXEMBOURG: u32 = 352;
pub const CTRY_MACAU: u32 = 853;
pub const CTRY_MACEDONIA: u32 = 389;
pub const CTRY_MALAYSIA: u32 = 60;
pub const CTRY_MALDIVES: u32 = 960;
pub const CTRY_MEXICO: u32 = 52;
pub const CTRY_MONACO: u32 = 33;
pub const CTRY_MONGOLIA: u32 = 976;
pub const CTRY_MOROCCO: u32 = 212;
pub const CTRY_NETHERLANDS: u32 = 31;
pub const CTRY_NEW_ZEALAND: u32 = 64;
pub const CTRY_NICARAGUA: u32 = 505;
pub const CTRY_NORWAY: u32 = 47;
pub const CTRY_OMAN: u32 = 968;
pub const CTRY_PAKISTAN: u32 = 92;
pub const CTRY_PANAMA: u32 = 507;
pub const CTRY_PARAGUAY: u32 = 595;
pub const CTRY_PERU: u32 = 51;
pub const CTRY_PHILIPPINES: u32 = 63;
pub const CTRY_POLAND: u32 = 48;
pub const CTRY_PORTUGAL: u32 = 351;
pub const CTRY_PRCHINA: u32 = 86;
pub const CTRY_PUERTO_RICO: u32 = 1;
pub const CTRY_QATAR: u32 = 974;
pub const CTRY_ROMANIA: u32 = 40;
pub const CTRY_RUSSIA: u32 = 7;
pub const CTRY_SAUDI_ARABIA: u32 = 966;
pub const CTRY_SERBIA: u32 = 381;
pub const CTRY_SINGAPORE: u32 = 65;
pub const CTRY_SLOVAK: u32 = 421;
pub const CTRY_SLOVENIA: u32 = 386;
pub const CTRY_SOUTH_AFRICA: u32 = 27;
pub const CTRY_SOUTH_KOREA: u32 = 82;
pub const CTRY_SPAIN: u32 = 34;
pub const CTRY_SWEDEN: u32 = 46;
pub const CTRY_SWITZERLAND: u32 = 41;
pub const CTRY_SYRIA: u32 = 963;
pub const CTRY_TAIWAN: u32 = 886;
pub const CTRY_TATARSTAN: u32 = 7;
pub const CTRY_THAILAND: u32 = 66;
pub const CTRY_TRINIDAD_Y_TOBAGO: u32 = 1;
pub const CTRY_TUNISIA: u32 = 216;
pub const CTRY_TURKEY: u32 = 90;
pub const CTRY_UAE: u32 = 971;
pub const CTRY_UKRAINE: u32 = 380;
pub const CTRY_UNITED_KINGDOM: u32 = 44;
pub const CTRY_UNITED_STATES: u32 = 1;
pub const CTRY_URUGUAY: u32 = 598;
pub const CTRY_UZBEKISTAN: u32 = 7;
pub const CTRY_VENEZUELA: u32 = 58;
pub const CTRY_VIET_NAM: u32 = 84;
pub const CTRY_YEMEN: u32 = 967;
pub const CTRY_ZIMBABWE: u32 = 263;
pub const CT_CTYPE1: u32 = 1;
pub const CT_CTYPE2: u32 = 2;
pub const CT_CTYPE3: u32 = 4;
pub type CURRENCYFMT = CURRENCYFMTA;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CURRENCYFMTA {
    pub NumDigits: u32,
    pub LeadingZero: u32,
    pub Grouping: u32,
    pub lpDecimalSep: windows_sys::core::PSTR,
    pub lpThousandSep: windows_sys::core::PSTR,
    pub NegativeOrder: u32,
    pub PositiveOrder: u32,
    pub lpCurrencySymbol: windows_sys::core::PSTR,
}
impl Default for CURRENCYFMTA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CURRENCYFMTW {
    pub NumDigits: u32,
    pub LeadingZero: u32,
    pub Grouping: u32,
    pub lpDecimalSep: windows_sys::core::PWSTR,
    pub lpThousandSep: windows_sys::core::PWSTR,
    pub NegativeOrder: u32,
    pub PositiveOrder: u32,
    pub lpCurrencySymbol: windows_sys::core::PWSTR,
}
impl Default for CURRENCYFMTW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type DATEFMT_ENUMPROCA = Option<unsafe extern "system" fn(param0: windows_sys::core::PCSTR) -> windows_sys::core::BOOL>;
pub type DATEFMT_ENUMPROCEXA = Option<unsafe extern "system" fn(param0: windows_sys::core::PCSTR, param1: CALID) -> windows_sys::core::BOOL>;
#[cfg(feature = "minwindef")]
pub type DATEFMT_ENUMPROCEXEX = Option<unsafe extern "system" fn(param0: windows_sys::core::PCWSTR, param1: CALID, param2: super::LPARAM) -> windows_sys::core::BOOL>;
pub type DATEFMT_ENUMPROCEXW = Option<unsafe extern "system" fn(param0: windows_sys::core::PCWSTR, param1: CALID) -> windows_sys::core::BOOL>;
pub type DATEFMT_ENUMPROCW = Option<unsafe extern "system" fn(param0: windows_sys::core::PCWSTR) -> windows_sys::core::BOOL>;
pub const DATE_AUTOLAYOUT: u32 = 64;
pub const DATE_LONGDATE: u32 = 2;
pub const DATE_LTRREADING: u32 = 16;
pub const DATE_MONTHDAY: u32 = 128;
pub const DATE_RTLREADING: u32 = 32;
pub const DATE_SHORTDATE: u32 = 1;
pub const DATE_USE_ALT_CALENDAR: u32 = 4;
pub const DATE_YEARMONTH: u32 = 8;
pub const ENUM_ALL_CALENDARS: u32 = 4294967295;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FILEMUIINFO {
    pub dwSize: u32,
    pub dwVersion: u32,
    pub dwFileType: u32,
    pub pChecksum: [u8; 16],
    pub pServiceChecksum: [u8; 16],
    pub dwLanguageNameOffset: u32,
    pub dwTypeIDMainSize: u32,
    pub dwTypeIDMainOffset: u32,
    pub dwTypeNameMainOffset: u32,
    pub dwTypeIDMUISize: u32,
    pub dwTypeIDMUIOffset: u32,
    pub dwTypeNameMUIOffset: u32,
    pub abBuffer: [u8; 8],
}
impl Default for FILEMUIINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FIND_ENDSWITH: u32 = 2097152;
pub const FIND_FROMEND: u32 = 8388608;
pub const FIND_FROMSTART: u32 = 4194304;
pub const FIND_STARTSWITH: u32 = 1048576;
pub type GEOCLASS = u32;
pub const GEOCLASS_ALL: SYSGEOCLASS = 0;
pub const GEOCLASS_NATION: SYSGEOCLASS = 16;
pub const GEOCLASS_REGION: SYSGEOCLASS = 14;
pub type GEOID = i32;
pub const GEOID_NOT_AVAILABLE: i32 = -1;
pub type GEOTYPE = u32;
pub const GEO_CURRENCYCODE: SYSGEOTYPE = 15;
pub const GEO_CURRENCYSYMBOL: SYSGEOTYPE = 16;
pub const GEO_DIALINGCODE: SYSGEOTYPE = 14;
#[cfg(feature = "minwindef")]
pub type GEO_ENUMNAMEPROC = Option<unsafe extern "system" fn(param0: windows_sys::core::PCWSTR, param1: super::LPARAM) -> windows_sys::core::BOOL>;
pub type GEO_ENUMPROC = Option<unsafe extern "system" fn(param0: GEOID) -> windows_sys::core::BOOL>;
pub const GEO_FRIENDLYNAME: SYSGEOTYPE = 8;
pub const GEO_ID: SYSGEOTYPE = 18;
pub const GEO_ISO2: SYSGEOTYPE = 4;
pub const GEO_ISO3: SYSGEOTYPE = 5;
pub const GEO_ISO_UN_NUMBER: SYSGEOTYPE = 12;
pub const GEO_LATITUDE: SYSGEOTYPE = 2;
pub const GEO_LCID: SYSGEOTYPE = 7;
pub const GEO_LONGITUDE: SYSGEOTYPE = 3;
pub const GEO_NAME: SYSGEOTYPE = 17;
pub const GEO_NAME_USER_DEFAULT: u32 = 0;
pub const GEO_NATION: SYSGEOTYPE = 1;
pub const GEO_OFFICIALLANGUAGES: SYSGEOTYPE = 11;
pub const GEO_OFFICIALNAME: SYSGEOTYPE = 9;
pub const GEO_PARENT: SYSGEOTYPE = 13;
pub const GEO_RFC1766: SYSGEOTYPE = 6;
pub const GEO_TIMEZONES: SYSGEOTYPE = 10;
pub const GSS_ALLOW_INHERITED_COMMON: u32 = 1;
pub const HIGH_SURROGATE_END: u32 = 56319;
pub const HIGH_SURROGATE_START: u32 = 55296;
pub type HSAVEDUILANGUAGES = *mut core::ffi::c_void;
pub const IDN_ALLOW_UNASSIGNED: u32 = 1;
pub const IDN_EMAIL_ADDRESS: u32 = 4;
pub const IDN_RAW_PUNYCODE: u32 = 8;
pub const IDN_USE_STD3_ASCII_RULES: u32 = 2;
#[cfg(feature = "winnt")]
pub type LANGGROUPLOCALE_ENUMPROCA = Option<unsafe extern "system" fn(param0: LGRPID, param1: super::LCID, param2: windows_sys::core::PCSTR, param3: isize) -> windows_sys::core::BOOL>;
#[cfg(feature = "winnt")]
pub type LANGGROUPLOCALE_ENUMPROCW = Option<unsafe extern "system" fn(param0: LGRPID, param1: super::LCID, param2: windows_sys::core::PCWSTR, param3: isize) -> windows_sys::core::BOOL>;
pub type LANGUAGEGROUP_ENUMPROCA = Option<unsafe extern "system" fn(param0: LGRPID, param1: windows_sys::core::PCSTR, param2: windows_sys::core::PCSTR, param3: u32, param4: isize) -> windows_sys::core::BOOL>;
pub type LANGUAGEGROUP_ENUMPROCW = Option<unsafe extern "system" fn(param0: LGRPID, param1: windows_sys::core::PCWSTR, param2: windows_sys::core::PCWSTR, param3: u32, param4: isize) -> windows_sys::core::BOOL>;
pub const LCID_ALTERNATE_SORTS: u32 = 4;
pub const LCID_INSTALLED: u32 = 1;
pub const LCID_SUPPORTED: u32 = 2;
pub const LCMAP_BYTEREV: u32 = 2048;
pub const LCMAP_FULLWIDTH: u32 = 8388608;
pub const LCMAP_HALFWIDTH: u32 = 4194304;
pub const LCMAP_HASH: u32 = 262144;
pub const LCMAP_HIRAGANA: u32 = 1048576;
pub const LCMAP_KATAKANA: u32 = 2097152;
pub const LCMAP_LINGUISTIC_CASING: u32 = 16777216;
pub const LCMAP_LOWERCASE: u32 = 256;
pub const LCMAP_SIMPLIFIED_CHINESE: u32 = 33554432;
pub const LCMAP_SORTHANDLE: u32 = 536870912;
pub const LCMAP_SORTKEY: u32 = 1024;
pub const LCMAP_TITLECASE: u32 = 768;
pub const LCMAP_TRADITIONAL_CHINESE: u32 = 67108864;
pub const LCMAP_UPPERCASE: u32 = 512;
pub type LCTYPE = u32;
pub type LGRPID = u32;
pub const LGRPID_ARABIC: u32 = 13;
pub const LGRPID_ARMENIAN: u32 = 17;
pub const LGRPID_BALTIC: u32 = 3;
pub const LGRPID_CENTRAL_EUROPE: u32 = 2;
pub const LGRPID_CYRILLIC: u32 = 5;
pub const LGRPID_GEORGIAN: u32 = 16;
pub const LGRPID_GREEK: u32 = 4;
pub const LGRPID_HEBREW: u32 = 12;
pub const LGRPID_INDIC: u32 = 15;
pub const LGRPID_INSTALLED: u32 = 1;
pub const LGRPID_JAPANESE: u32 = 7;
pub const LGRPID_KOREAN: u32 = 8;
pub const LGRPID_SIMPLIFIED_CHINESE: u32 = 10;
pub const LGRPID_SUPPORTED: u32 = 2;
pub const LGRPID_THAI: u32 = 11;
pub const LGRPID_TRADITIONAL_CHINESE: u32 = 9;
pub const LGRPID_TURKIC: u32 = 6;
pub const LGRPID_TURKISH: u32 = 6;
pub const LGRPID_VIETNAMESE: u32 = 14;
pub const LGRPID_WESTERN_EUROPE: u32 = 1;
pub const LINGUISTIC_IGNORECASE: u32 = 16;
pub const LINGUISTIC_IGNOREDIACRITIC: u32 = 32;
pub const LOCALE_ALL: u32 = 0;
pub const LOCALE_ALLOW_NEUTRAL_NAMES: u32 = 134217728;
pub const LOCALE_ALTERNATE_SORTS: u32 = 4;
pub type LOCALE_ENUMPROCA = Option<unsafe extern "system" fn(param0: windows_sys::core::PCSTR) -> windows_sys::core::BOOL>;
#[cfg(feature = "minwindef")]
pub type LOCALE_ENUMPROCEX = Option<unsafe extern "system" fn(param0: windows_sys::core::PCWSTR, param1: u32, param2: super::LPARAM) -> windows_sys::core::BOOL>;
pub type LOCALE_ENUMPROCW = Option<unsafe extern "system" fn(param0: windows_sys::core::PCWSTR) -> windows_sys::core::BOOL>;
pub const LOCALE_FONTSIGNATURE: u32 = 88;
pub const LOCALE_ICALENDARTYPE: u32 = 4105;
pub const LOCALE_ICENTURY: u32 = 36;
pub const LOCALE_ICONSTRUCTEDLOCALE: u32 = 125;
pub const LOCALE_ICOUNTRY: u32 = 5;
pub const LOCALE_ICURRDIGITS: u32 = 25;
pub const LOCALE_ICURRENCY: u32 = 27;
pub const LOCALE_IDATE: u32 = 33;
pub const LOCALE_IDAYLZERO: u32 = 38;
pub const LOCALE_IDEFAULTANSICODEPAGE: u32 = 4100;
pub const LOCALE_IDEFAULTCODEPAGE: u32 = 11;
pub const LOCALE_IDEFAULTCOUNTRY: u32 = 10;
pub const LOCALE_IDEFAULTEBCDICCODEPAGE: u32 = 4114;
pub const LOCALE_IDEFAULTLANGUAGE: u32 = 9;
pub const LOCALE_IDEFAULTMACCODEPAGE: u32 = 4113;
pub const LOCALE_IDIALINGCODE: u32 = 5;
pub const LOCALE_IDIGITS: u32 = 17;
pub const LOCALE_IDIGITSUBSTITUTION: u32 = 4116;
pub const LOCALE_IFIRSTDAYOFWEEK: u32 = 4108;
pub const LOCALE_IFIRSTWEEKOFYEAR: u32 = 4109;
pub const LOCALE_IGEOID: u32 = 91;
pub const LOCALE_IINTLCURRDIGITS: u32 = 26;
pub const LOCALE_ILANGUAGE: u32 = 1;
pub const LOCALE_ILDATE: u32 = 34;
pub const LOCALE_ILZERO: u32 = 18;
pub const LOCALE_IMEASURE: u32 = 13;
pub const LOCALE_IMONLZERO: u32 = 39;
pub const LOCALE_INEGATIVEPERCENT: u32 = 116;
pub const LOCALE_INEGCURR: u32 = 28;
pub const LOCALE_INEGNUMBER: u32 = 4112;
pub const LOCALE_INEGSEPBYSPACE: u32 = 87;
pub const LOCALE_INEGSIGNPOSN: u32 = 83;
pub const LOCALE_INEGSYMPRECEDES: u32 = 86;
pub const LOCALE_INEUTRAL: u32 = 113;
pub const LOCALE_IOPTIONALCALENDAR: u32 = 4107;
pub const LOCALE_IPAPERSIZE: u32 = 4106;
pub const LOCALE_IPOSITIVEPERCENT: u32 = 117;
pub const LOCALE_IPOSSEPBYSPACE: u32 = 85;
pub const LOCALE_IPOSSIGNPOSN: u32 = 82;
pub const LOCALE_IPOSSYMPRECEDES: u32 = 84;
pub const LOCALE_IREADINGLAYOUT: u32 = 112;
pub const LOCALE_ITIME: u32 = 35;
pub const LOCALE_ITIMEMARKPOSN: u32 = 4101;
pub const LOCALE_ITLZERO: u32 = 37;
pub const LOCALE_IUSEUTF8LEGACYACP: u32 = 1638;
pub const LOCALE_IUSEUTF8LEGACYOEMCP: u32 = 2457;
pub const LOCALE_NAME_INVARIANT: windows_sys::core::PCWSTR = windows_sys::core::w!("");
pub const LOCALE_NAME_SYSTEM_DEFAULT: windows_sys::core::PCWSTR = windows_sys::core::w!("!x-sys-default-locale");
pub const LOCALE_NAME_USER_DEFAULT: u32 = 0;
pub const LOCALE_NEUTRALDATA: u32 = 16;
pub const LOCALE_NOUSEROVERRIDE: u32 = 2147483648;
pub const LOCALE_REPLACEMENT: u32 = 8;
pub const LOCALE_RETURN_GENITIVE_NAMES: u32 = 268435456;
pub const LOCALE_RETURN_NUMBER: u32 = 536870912;
pub const LOCALE_S1159: u32 = 40;
pub const LOCALE_S2359: u32 = 41;
pub const LOCALE_SABBREVCTRYNAME: u32 = 7;
pub const LOCALE_SABBREVDAYNAME1: u32 = 49;
pub const LOCALE_SABBREVDAYNAME2: u32 = 50;
pub const LOCALE_SABBREVDAYNAME3: u32 = 51;
pub const LOCALE_SABBREVDAYNAME4: u32 = 52;
pub const LOCALE_SABBREVDAYNAME5: u32 = 53;
pub const LOCALE_SABBREVDAYNAME6: u32 = 54;
pub const LOCALE_SABBREVDAYNAME7: u32 = 55;
pub const LOCALE_SABBREVLANGNAME: u32 = 3;
pub const LOCALE_SABBREVMONTHNAME1: u32 = 68;
pub const LOCALE_SABBREVMONTHNAME10: u32 = 77;
pub const LOCALE_SABBREVMONTHNAME11: u32 = 78;
pub const LOCALE_SABBREVMONTHNAME12: u32 = 79;
pub const LOCALE_SABBREVMONTHNAME13: u32 = 4111;
pub const LOCALE_SABBREVMONTHNAME2: u32 = 69;
pub const LOCALE_SABBREVMONTHNAME3: u32 = 70;
pub const LOCALE_SABBREVMONTHNAME4: u32 = 71;
pub const LOCALE_SABBREVMONTHNAME5: u32 = 72;
pub const LOCALE_SABBREVMONTHNAME6: u32 = 73;
pub const LOCALE_SABBREVMONTHNAME7: u32 = 74;
pub const LOCALE_SABBREVMONTHNAME8: u32 = 75;
pub const LOCALE_SABBREVMONTHNAME9: u32 = 76;
pub const LOCALE_SAM: u32 = 40;
pub const LOCALE_SCONSOLEFALLBACKNAME: u32 = 110;
pub const LOCALE_SCOUNTRY: u32 = 6;
pub const LOCALE_SCURRENCY: u32 = 20;
pub const LOCALE_SDATE: u32 = 29;
pub const LOCALE_SDAYNAME1: u32 = 42;
pub const LOCALE_SDAYNAME2: u32 = 43;
pub const LOCALE_SDAYNAME3: u32 = 44;
pub const LOCALE_SDAYNAME4: u32 = 45;
pub const LOCALE_SDAYNAME5: u32 = 46;
pub const LOCALE_SDAYNAME6: u32 = 47;
pub const LOCALE_SDAYNAME7: u32 = 48;
pub const LOCALE_SDECIMAL: u32 = 14;
pub const LOCALE_SDURATION: u32 = 93;
pub const LOCALE_SENGCOUNTRY: u32 = 4098;
pub const LOCALE_SENGCURRNAME: u32 = 4103;
pub const LOCALE_SENGLANGUAGE: u32 = 4097;
pub const LOCALE_SENGLISHCOUNTRYNAME: u32 = 4098;
pub const LOCALE_SENGLISHDISPLAYNAME: u32 = 114;
pub const LOCALE_SENGLISHLANGUAGENAME: u32 = 4097;
pub const LOCALE_SGROUPING: u32 = 16;
pub const LOCALE_SINTLSYMBOL: u32 = 21;
pub const LOCALE_SISO3166CTRYNAME: u32 = 90;
pub const LOCALE_SISO3166CTRYNAME2: u32 = 104;
pub const LOCALE_SISO639LANGNAME: u32 = 89;
pub const LOCALE_SISO639LANGNAME2: u32 = 103;
pub const LOCALE_SKEYBOARDSTOINSTALL: u32 = 94;
pub const LOCALE_SLANGDISPLAYNAME: u32 = 111;
pub const LOCALE_SLANGUAGE: u32 = 2;
pub const LOCALE_SLIST: u32 = 12;
pub const LOCALE_SLOCALIZEDCOUNTRYNAME: u32 = 6;
pub const LOCALE_SLOCALIZEDDISPLAYNAME: u32 = 2;
pub const LOCALE_SLOCALIZEDLANGUAGENAME: u32 = 111;
pub const LOCALE_SLONGDATE: u32 = 32;
pub const LOCALE_SMONDECIMALSEP: u32 = 22;
pub const LOCALE_SMONGROUPING: u32 = 24;
pub const LOCALE_SMONTHDAY: u32 = 120;
pub const LOCALE_SMONTHNAME1: u32 = 56;
pub const LOCALE_SMONTHNAME10: u32 = 65;
pub const LOCALE_SMONTHNAME11: u32 = 66;
pub const LOCALE_SMONTHNAME12: u32 = 67;
pub const LOCALE_SMONTHNAME13: u32 = 4110;
pub const LOCALE_SMONTHNAME2: u32 = 57;
pub const LOCALE_SMONTHNAME3: u32 = 58;
pub const LOCALE_SMONTHNAME4: u32 = 59;
pub const LOCALE_SMONTHNAME5: u32 = 60;
pub const LOCALE_SMONTHNAME6: u32 = 61;
pub const LOCALE_SMONTHNAME7: u32 = 62;
pub const LOCALE_SMONTHNAME8: u32 = 63;
pub const LOCALE_SMONTHNAME9: u32 = 64;
pub const LOCALE_SMONTHOUSANDSEP: u32 = 23;
pub const LOCALE_SNAME: u32 = 92;
pub const LOCALE_SNAN: u32 = 105;
pub const LOCALE_SNATIVECOUNTRYNAME: u32 = 8;
pub const LOCALE_SNATIVECTRYNAME: u32 = 8;
pub const LOCALE_SNATIVECURRNAME: u32 = 4104;
pub const LOCALE_SNATIVEDIGITS: u32 = 19;
pub const LOCALE_SNATIVEDISPLAYNAME: u32 = 115;
pub const LOCALE_SNATIVELANGNAME: u32 = 4;
pub const LOCALE_SNATIVELANGUAGENAME: u32 = 4;
pub const LOCALE_SNEGATIVESIGN: u32 = 81;
pub const LOCALE_SNEGINFINITY: u32 = 107;
pub const LOCALE_SOPENTYPELANGUAGETAG: u32 = 122;
pub const LOCALE_SPARENT: u32 = 109;
pub const LOCALE_SPECIFICDATA: u32 = 32;
pub const LOCALE_SPERCENT: u32 = 118;
pub const LOCALE_SPERMILLE: u32 = 119;
pub const LOCALE_SPM: u32 = 41;
pub const LOCALE_SPOSINFINITY: u32 = 106;
pub const LOCALE_SPOSITIVESIGN: u32 = 80;
pub const LOCALE_SRELATIVELONGDATE: u32 = 124;
pub const LOCALE_SSCRIPTS: u32 = 108;
pub const LOCALE_SSHORTDATE: u32 = 31;
pub const LOCALE_SSHORTESTAM: u32 = 126;
pub const LOCALE_SSHORTESTDAYNAME1: u32 = 96;
pub const LOCALE_SSHORTESTDAYNAME2: u32 = 97;
pub const LOCALE_SSHORTESTDAYNAME3: u32 = 98;
pub const LOCALE_SSHORTESTDAYNAME4: u32 = 99;
pub const LOCALE_SSHORTESTDAYNAME5: u32 = 100;
pub const LOCALE_SSHORTESTDAYNAME6: u32 = 101;
pub const LOCALE_SSHORTESTDAYNAME7: u32 = 102;
pub const LOCALE_SSHORTESTPM: u32 = 127;
pub const LOCALE_SSHORTTIME: u32 = 121;
pub const LOCALE_SSORTLOCALE: u32 = 123;
pub const LOCALE_SSORTNAME: u32 = 4115;
pub const LOCALE_STHOUSAND: u32 = 15;
pub const LOCALE_STIME: u32 = 30;
pub const LOCALE_STIMEFORMAT: u32 = 4099;
pub const LOCALE_SUPPLEMENTAL: u32 = 2;
pub const LOCALE_SYEARMONTH: u32 = 4102;
pub const LOCALE_USE_CP_ACP: u32 = 1073741824;
pub const LOCALE_WINDOWS: u32 = 1;
pub const LOW_SURROGATE_END: u32 = 57343;
pub const LOW_SURROGATE_START: u32 = 56320;
pub type LPCPINFO = *mut CPINFO;
pub type LPCPINFOEX = LPCPINFOEXA;
pub type LPCPINFOEXA = *mut CPINFOEXA;
pub type LPCPINFOEXW = *mut CPINFOEXW;
pub type LPCURRENCYFMT = LPCURRENCYFMTA;
pub type LPCURRENCYFMTA = *mut CURRENCYFMTA;
pub type LPCURRENCYFMTW = *mut CURRENCYFMTW;
pub type LPNLSVERSIONINFO = *mut NLSVERSIONINFO;
pub type LPNLSVERSIONINFOEX = *mut NLSVERSIONINFOEX;
pub type LPNUMBERFMT = LPNUMBERFMTA;
pub type LPNUMBERFMTA = *mut NUMBERFMTA;
pub type LPNUMBERFMTW = *mut NUMBERFMTW;
pub const MAP_COMPOSITE: u32 = 64;
pub const MAP_EXPAND_LIGATURES: u32 = 8192;
pub const MAP_FOLDCZONE: u32 = 16;
pub const MAP_FOLDDIGITS: u32 = 128;
pub const MAP_PRECOMPOSED: u32 = 32;
pub const MAX_DEFAULTCHAR: u32 = 2;
pub const MAX_LEADBYTES: u32 = 12;
pub const MB_COMPOSITE: u32 = 2;
pub const MB_ERR_INVALID_CHARS: u32 = 8;
pub const MB_PRECOMPOSED: u32 = 1;
pub const MB_USEGLYPHCHARS: u32 = 4;
pub const MUI_COMPLEX_SCRIPT_FILTER: u32 = 512;
pub const MUI_CONSOLE_FILTER: u32 = 256;
pub const MUI_FILEINFO_VERSION: u32 = 1;
pub const MUI_FILETYPE_LANGUAGE_NEUTRAL_MAIN: u32 = 2;
pub const MUI_FILETYPE_LANGUAGE_NEUTRAL_MUI: u32 = 4;
pub const MUI_FILETYPE_NOT_LANGUAGE_NEUTRAL: u32 = 1;
pub const MUI_FORMAT_INF_COMPAT: u32 = 2;
pub const MUI_FORMAT_REG_COMPAT: u32 = 1;
pub const MUI_FULL_LANGUAGE: u32 = 1;
pub const MUI_IMMUTABLE_LOOKUP: u32 = 16;
pub const MUI_LANGUAGE_ID: u32 = 4;
pub const MUI_LANGUAGE_INSTALLED: u32 = 32;
pub const MUI_LANGUAGE_LICENSED: u32 = 64;
pub const MUI_LANGUAGE_NAME: u32 = 8;
pub const MUI_LANG_NEUTRAL_PE_FILE: u32 = 256;
pub const MUI_LIP_LANGUAGE: u32 = 4;
pub const MUI_MACHINE_LANGUAGE_SETTINGS: u32 = 1024;
pub const MUI_MERGE_SYSTEM_FALLBACK: u32 = 16;
pub const MUI_MERGE_USER_FALLBACK: u32 = 32;
pub const MUI_NON_LANG_NEUTRAL_FILE: u32 = 512;
pub const MUI_PARTIAL_LANGUAGE: u32 = 2;
pub const MUI_QUERY_CHECKSUM: u32 = 2;
pub const MUI_QUERY_LANGUAGE_NAME: u32 = 4;
pub const MUI_QUERY_RESOURCE_TYPES: u32 = 8;
pub const MUI_QUERY_TYPE: u32 = 1;
pub const MUI_RESET_FILTERS: u32 = 1;
pub const MUI_SKIP_STRING_CACHE: u32 = 8;
pub const MUI_THREAD_LANGUAGES: u32 = 64;
pub const MUI_UI_FALLBACK: u32 = 48;
pub const MUI_USER_PREFERRED_UI_LANGUAGES: u32 = 16;
pub const MUI_USE_INSTALLED_LANGUAGES: u32 = 32;
pub const MUI_USE_SEARCH_ALL_LANGUAGES: u32 = 64;
pub const MUI_VERIFY_FILE_EXISTS: u32 = 4;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NLSVERSIONINFO {
    pub dwNLSVersionInfoSize: u32,
    pub dwNLSVersion: u32,
    pub dwDefinedVersion: u32,
    pub dwEffectiveId: u32,
    pub guidCustomVersion: windows_sys::core::GUID,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NLSVERSIONINFOEX {
    pub dwNLSVersionInfoSize: u32,
    pub dwNLSVersion: u32,
    pub dwDefinedVersion: u32,
    pub dwEffectiveId: u32,
    pub guidCustomVersion: windows_sys::core::GUID,
}
pub type NLS_FUNCTION = u32;
pub type NORM_FORM = i32;
pub const NORM_IGNORECASE: u32 = 1;
pub const NORM_IGNOREKANATYPE: u32 = 65536;
pub const NORM_IGNORENONSPACE: u32 = 2;
pub const NORM_IGNORESYMBOLS: u32 = 4;
pub const NORM_IGNOREWIDTH: u32 = 131072;
pub const NORM_LINGUISTIC_CASING: u32 = 134217728;
pub type NUMBERFMT = NUMBERFMTA;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NUMBERFMTA {
    pub NumDigits: u32,
    pub LeadingZero: u32,
    pub Grouping: u32,
    pub lpDecimalSep: windows_sys::core::PSTR,
    pub lpThousandSep: windows_sys::core::PSTR,
    pub NegativeOrder: u32,
}
impl Default for NUMBERFMTA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NUMBERFMTW {
    pub NumDigits: u32,
    pub LeadingZero: u32,
    pub Grouping: u32,
    pub lpDecimalSep: windows_sys::core::PWSTR,
    pub lpThousandSep: windows_sys::core::PWSTR,
    pub NegativeOrder: u32,
}
impl Default for NUMBERFMTW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const NormalizationC: NORM_FORM = 1;
pub const NormalizationD: NORM_FORM = 2;
pub const NormalizationKC: NORM_FORM = 5;
pub const NormalizationKD: NORM_FORM = 6;
pub const NormalizationOther: NORM_FORM = 0;
pub type PFILEMUIINFO = *mut FILEMUIINFO;
pub const SORTING_PARADIGM_ICU: u32 = 16777216;
pub const SORTING_PARADIGM_NLS: u32 = 0;
pub const SORT_DIGITSASNUMBERS: u32 = 8;
pub const SORT_STRINGSORT: u32 = 4096;
pub type SYSGEOCLASS = i32;
pub type SYSGEOTYPE = i32;
pub type SYSNLS_FUNCTION = i32;
pub type TIMEFMT_ENUMPROCA = Option<unsafe extern "system" fn(param0: windows_sys::core::PCSTR) -> windows_sys::core::BOOL>;
#[cfg(feature = "minwindef")]
pub type TIMEFMT_ENUMPROCEX = Option<unsafe extern "system" fn(param0: windows_sys::core::PCWSTR, param1: super::LPARAM) -> windows_sys::core::BOOL>;
pub type TIMEFMT_ENUMPROCW = Option<unsafe extern "system" fn(param0: windows_sys::core::PCWSTR) -> windows_sys::core::BOOL>;
pub const TIME_FORCE24HOURFORMAT: u32 = 8;
pub const TIME_NOMINUTESORSECONDS: u32 = 1;
pub const TIME_NOSECONDS: u32 = 2;
pub const TIME_NOTIMEMARKER: u32 = 4;
pub type UILANGUAGE_ENUMPROCA = Option<unsafe extern "system" fn(param0: windows_sys::core::PCSTR, param1: isize) -> windows_sys::core::BOOL>;
pub type UILANGUAGE_ENUMPROCW = Option<unsafe extern "system" fn(param0: windows_sys::core::PCWSTR, param1: isize) -> windows_sys::core::BOOL>;
pub const VS_ALLOW_LATIN: u32 = 1;
pub const WC_COMPOSITECHECK: u32 = 512;
pub const WC_DEFAULTCHAR: u32 = 64;
pub const WC_DISCARDNS: u32 = 16;
pub const WC_ERR_INVALID_CHARS: u32 = 128;
pub const WC_NO_BEST_FIT_CHARS: u32 = 1024;
pub const WC_SEPCHARS: u32 = 32;
