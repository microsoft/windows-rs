#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn CompareStringA();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CompareStringEx();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CompareStringOrdinal();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CompareStringW();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ConvertDefaultLocale();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumCalendarInfoA();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumCalendarInfoExA();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumCalendarInfoExEx();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumCalendarInfoExW();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumCalendarInfoW();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumDateFormatsA();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumDateFormatsExA();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumDateFormatsExEx();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumDateFormatsExW();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumDateFormatsW();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumLanguageGroupLocalesA();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumLanguageGroupLocalesW();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumSystemCodePagesA();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumSystemCodePagesW();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumSystemGeoID();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumSystemGeoNames();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumSystemLanguageGroupsA();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumSystemLanguageGroupsW();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumSystemLocalesA();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumSystemLocalesEx();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumSystemLocalesW();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumTimeFormatsA();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumTimeFormatsEx();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumTimeFormatsW();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumUILanguagesA();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumUILanguagesW();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindNLSString();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindNLSStringEx();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindStringOrdinal();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FoldStringA();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FoldStringW();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn GetACP();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCPInfo();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCPInfoExA();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCPInfoExW();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCalendarInfoA();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCalendarInfoEx();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCalendarInfoW();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCurrencyFormatA();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCurrencyFormatEx();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCurrencyFormatW();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDateFormatA();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDateFormatEx();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDateFormatW();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDistanceOfClosestLanguageInList();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDurationFormat();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDurationFormatEx();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFileMUIInfo();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFileMUIPath();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetGeoInfoA();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetGeoInfoEx();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetGeoInfoW();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetLocaleInfoA();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetLocaleInfoEx();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetLocaleInfoW();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetNLSVersion();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetNLSVersionEx();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetNumberFormatA();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetNumberFormatEx();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetNumberFormatW();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn GetOEMCP();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetProcessPreferredUILanguages();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetStringScripts();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetStringTypeA();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetStringTypeExA();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetStringTypeExW();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetStringTypeW();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn GetSystemDefaultLCID();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn GetSystemDefaultLangID();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetSystemDefaultLocaleName();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn GetSystemDefaultUILanguage();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetSystemPreferredUILanguages();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn GetTextCharset();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn GetTextCharsetInfo();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn GetThreadLocale();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetThreadPreferredUILanguages();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn GetThreadUILanguage();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetTimeFormatA();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetTimeFormatEx();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetTimeFormatW();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetUILanguageInfo();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetUserDefaultGeoName();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn GetUserDefaultLCID();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn GetUserDefaultLangID();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetUserDefaultLocaleName();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn GetUserDefaultUILanguage();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn GetUserGeoID();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetUserPreferredUILanguages();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IdnToAscii();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IdnToNameprepUnicode();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IdnToUnicode();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsDBCSLeadByte();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsDBCSLeadByteEx();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsNLSDefinedString();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsNormalizedString();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsTextUnicode();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsValidCodePage();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsValidLanguageGroup();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsValidLocale();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsValidLocaleName();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsValidNLSVersion();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsWellFormedTag();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LCIDToLocaleName();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LCMapStringA();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LCMapStringEx();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LCMapStringW();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LocaleNameToLCID();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MappingDoAction();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MappingFreePropertyBag();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MappingFreeServices();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MappingGetServices();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MappingRecognizeText();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MultiByteToWideChar();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NormalizeString();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NotifyUILanguageChange();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResolveLocaleName();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn RestoreThreadPreferredUILanguages();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ScriptApplyDigitSubstitution();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn ScriptApplyLogicalWidth();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ScriptBreak();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ScriptCPtoX();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn ScriptCacheGetHeight();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ScriptFreeCache();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn ScriptGetCMap();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn ScriptGetFontAlternateGlyphs();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn ScriptGetFontFeatureTags();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn ScriptGetFontLanguageTags();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn ScriptGetFontProperties();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn ScriptGetFontScriptTags();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn ScriptGetGlyphABCWidth();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ScriptGetLogicalWidths();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ScriptGetProperties();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ScriptIsComplex();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ScriptItemize();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ScriptItemizeOpenType();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ScriptJustify();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ScriptLayout();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn ScriptPlace();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn ScriptPlaceOpenType();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn ScriptPositionSingleGlyph();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ScriptRecordDigitSubstitution();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn ScriptShape();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn ScriptShapeOpenType();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn ScriptStringAnalyse();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ScriptStringCPtoX();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ScriptStringFree();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ScriptStringGetLogicalWidths();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ScriptStringGetOrder();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn ScriptStringOut();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ScriptStringValidate();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ScriptStringXtoCP();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ScriptString_pLogAttr();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ScriptString_pSize();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ScriptString_pcOutChars();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn ScriptSubstituteSingleGlyph();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn ScriptTextOut();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ScriptXtoCP();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetCalendarInfoA();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetCalendarInfoW();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetLocaleInfoA();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetLocaleInfoW();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetProcessPreferredUILanguages();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetThreadLocale();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetThreadPreferredUILanguages();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetThreadPreferredUILanguages2();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn SetThreadUILanguage();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetUserGeoID();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetUserGeoName();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TranslateCharsetInfo();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UCNV_FROM_U_CALLBACK_ESCAPE();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UCNV_FROM_U_CALLBACK_SKIP();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UCNV_FROM_U_CALLBACK_STOP();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UCNV_FROM_U_CALLBACK_SUBSTITUTE();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UCNV_TO_U_CALLBACK_ESCAPE();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UCNV_TO_U_CALLBACK_SKIP();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UCNV_TO_U_CALLBACK_STOP();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UCNV_TO_U_CALLBACK_SUBSTITUTE();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VerifyScripts();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WideCharToMultiByte();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lstrcatA();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lstrcatW();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lstrcmpA();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lstrcmpW();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lstrcmpiA();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lstrcmpiW();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lstrcpyA();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lstrcpyW();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lstrcpynA();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lstrcpynW();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lstrlenA();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lstrlenW();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_UCharsToChars();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_austrcpy();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_austrncpy();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_catclose();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_catgets();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_catopen();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_charAge();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_charDigitValue();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_charDirection();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_charFromName();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_charMirror();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_charName();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_charType();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_charsToUChars();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_cleanup();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_countChar32();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_digit();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_enumCharNames();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_enumCharTypes();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_errorName();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_foldCase();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_forDigit();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_formatMessage();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_formatMessageWithError();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_getBidiPairedBracket();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_getBinaryPropertySet();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_getCombiningClass();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_getDataVersion();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_getFC_NFKC_Closure();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_getIntPropertyMap();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_getIntPropertyMaxValue();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_getIntPropertyMinValue();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_getIntPropertyValue();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_getNumericValue();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_getPropertyEnum();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_getPropertyName();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_getPropertyValueEnum();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_getPropertyValueName();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_getUnicodeVersion();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_getVersion();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_hasBinaryProperty();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_init();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_isIDIgnorable();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_isIDPart();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_isIDStart();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_isISOControl();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_isJavaIDPart();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_isJavaIDStart();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_isJavaSpaceChar();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_isMirrored();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_isUAlphabetic();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_isULowercase();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_isUUppercase();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_isUWhiteSpace();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_isWhitespace();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_isalnum();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_isalpha();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_isbase();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_isblank();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_iscntrl();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_isdefined();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_isdigit();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_isgraph();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_islower();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_isprint();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_ispunct();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_isspace();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_istitle();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_isupper();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_isxdigit();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_memcasecmp();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_memchr();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_memchr32();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_memcmp();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_memcmpCodePointOrder();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_memcpy();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_memmove();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_memrchr();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_memrchr32();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_memset();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_parseMessage();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_parseMessageWithError();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_setMemoryFunctions();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_shapeArabic();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_strCaseCompare();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_strCompare();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_strCompareIter();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_strFindFirst();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_strFindLast();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_strFoldCase();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_strFromJavaModifiedUTF8WithSub();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_strFromUTF32();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_strFromUTF32WithSub();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_strFromUTF8();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_strFromUTF8Lenient();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_strFromUTF8WithSub();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_strFromWCS();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_strHasMoreChar32Than();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_strToJavaModifiedUTF8();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_strToLower();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_strToTitle();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_strToUTF32();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_strToUTF32WithSub();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_strToUTF8();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_strToUTF8WithSub();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_strToUpper();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_strToWCS();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_strcasecmp();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_strcat();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_strchr();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_strchr32();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_strcmp();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_strcmpCodePointOrder();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_strcpy();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_strcspn();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_strlen();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_strncasecmp();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_strncat();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_strncmp();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_strncmpCodePointOrder();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_strncpy();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_strpbrk();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_strrchr();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_strrchr32();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_strrstr();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_strspn();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_strstr();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_strtok_r();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_tolower();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_totitle();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_toupper();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_uastrcpy();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_uastrncpy();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_unescape();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_unescapeAt();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_versionFromString();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn u_versionFromUString();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_versionToString();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_vformatMessage();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_vformatMessageWithError();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_vparseMessage();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn u_vparseMessageWithError();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ubidi_close();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ubidi_countParagraphs();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ubidi_countRuns();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ubidi_getBaseDirection();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ubidi_getClassCallback();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ubidi_getCustomizedClass();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ubidi_getDirection();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ubidi_getLength();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ubidi_getLevelAt();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ubidi_getLevels();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ubidi_getLogicalIndex();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ubidi_getLogicalMap();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ubidi_getLogicalRun();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ubidi_getParaLevel();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ubidi_getParagraph();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ubidi_getParagraphByIndex();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ubidi_getProcessedLength();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ubidi_getReorderingMode();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ubidi_getReorderingOptions();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ubidi_getResultLength();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ubidi_getText();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ubidi_getVisualIndex();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ubidi_getVisualMap();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ubidi_getVisualRun();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ubidi_invertMap();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ubidi_isInverse();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ubidi_isOrderParagraphsLTR();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ubidi_open();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ubidi_openSized();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ubidi_orderParagraphsLTR();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ubidi_reorderLogical();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ubidi_reorderVisual();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ubidi_setClassCallback();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ubidi_setContext();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ubidi_setInverse();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ubidi_setLine();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ubidi_setPara();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ubidi_setReorderingMode();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ubidi_setReorderingOptions();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ubidi_writeReordered();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ubidi_writeReverse();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ubiditransform_close();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ubiditransform_open();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ubiditransform_transform();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ublock_getCode();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ubrk_close();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ubrk_countAvailable();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ubrk_current();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ubrk_first();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ubrk_following();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ubrk_getAvailable();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ubrk_getBinaryRules();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ubrk_getLocaleByType();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ubrk_getRuleStatus();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ubrk_getRuleStatusVec();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ubrk_isBoundary();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ubrk_last();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ubrk_next();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ubrk_open();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ubrk_openBinaryRules();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ubrk_openRules();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ubrk_preceding();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ubrk_previous();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ubrk_refreshUText();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ubrk_safeClone();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ubrk_setText();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ubrk_setUText();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucal_add();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucal_clear();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucal_clearField();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucal_clone();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucal_close();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucal_countAvailable();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucal_equivalentTo();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucal_get();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucal_getAttribute();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucal_getAvailable();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucal_getCanonicalTimeZoneID();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucal_getDSTSavings();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucal_getDayOfWeekType();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucal_getDefaultTimeZone();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucal_getFieldDifference();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucal_getGregorianChange();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucal_getHostTimeZone();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucal_getKeywordValuesForLocale();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucal_getLimit();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucal_getLocaleByType();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucal_getMillis();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucal_getNow();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucal_getTZDataVersion();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucal_getTimeZoneDisplayName();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucal_getTimeZoneID();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucal_getTimeZoneIDForWindowsID();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucal_getTimeZoneTransitionDate();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucal_getType();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucal_getWeekendTransition();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucal_getWindowsTimeZoneID();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucal_inDaylightTime();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucal_isSet();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucal_isWeekend();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucal_open();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucal_openCountryTimeZones();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucal_openTimeZoneIDEnumeration();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucal_openTimeZones();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucal_roll();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucal_set();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucal_setAttribute();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucal_setDate();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucal_setDateTime();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucal_setDefaultTimeZone();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucal_setGregorianChange();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucal_setMillis();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucal_setTimeZone();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucasemap_close();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucasemap_getBreakIterator();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucasemap_getLocale();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucasemap_getOptions();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucasemap_open();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucasemap_setBreakIterator();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucasemap_setLocale();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucasemap_setOptions();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucasemap_toTitle();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucasemap_utf8FoldCase();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucasemap_utf8ToLower();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucasemap_utf8ToTitle();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucasemap_utf8ToUpper();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucfpos_close();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucfpos_constrainCategory();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucfpos_constrainField();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucfpos_getCategory();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucfpos_getField();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucfpos_getIndexes();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucfpos_getInt64IterationContext();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucfpos_matchesField();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucfpos_open();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucfpos_reset();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucfpos_setInt64IterationContext();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucfpos_setState();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_cbFromUWriteBytes();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_cbFromUWriteSub();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_cbFromUWriteUChars();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_cbToUWriteSub();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_cbToUWriteUChars();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucnv_close();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_compareNames();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_convert();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_convertEx();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_countAliases();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucnv_countAvailable();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucnv_countStandards();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_detectUnicodeSignature();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucnv_fixFileSeparator();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucnv_flushCache();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_fromAlgorithmic();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_fromUChars();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucnv_fromUCountPending();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_fromUnicode();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_getAlias();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_getAliases();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_getAvailableName();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucnv_getCCSID();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_getCanonicalName();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_getDefaultName();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_getDisplayName();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_getFromUCallBack();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_getInvalidChars();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucnv_getInvalidUChars();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucnv_getMaxCharSize();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucnv_getMinCharSize();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_getName();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_getNextUChar();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucnv_getPlatform();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_getStandard();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_getStandardName();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucnv_getStarters();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_getSubstChars();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_getToUCallBack();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucnv_getType();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucnv_getUnicodeSet();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucnv_isAmbiguous();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucnv_isFixedWidth();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_open();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucnv_openAllNames();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucnv_openCCSID();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_openPackage();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_openStandardNames();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucnv_openU();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucnv_reset();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucnv_resetFromUnicode();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucnv_resetToUnicode();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucnv_safeClone();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_setDefaultName();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucnv_setFallback();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_setFromUCallBack();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_setSubstChars();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucnv_setSubstString();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_setToUCallBack();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_toAlgorithmic();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_toUChars();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucnv_toUCountPending();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnv_toUnicode();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucnv_usesFallback();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucnvsel_close();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucnvsel_open();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucnvsel_openFromSerialized();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucnvsel_selectForString();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucnvsel_selectForUTF8();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucnvsel_serialize();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucol_cloneBinary();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucol_close();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucol_closeElements();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucol_countAvailable();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucol_equal();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucol_getAttribute();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucol_getAvailable();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucol_getBound();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucol_getContractionsAndExpansions();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucol_getDisplayName();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucol_getEquivalentReorderCodes();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucol_getFunctionalEquivalent();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucol_getKeywordValues();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucol_getKeywordValuesForLocale();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucol_getKeywords();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucol_getLocaleByType();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucol_getMaxExpansion();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucol_getMaxVariable();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucol_getOffset();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucol_getReorderCodes();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucol_getRules();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucol_getRulesEx();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucol_getSortKey();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucol_getStrength();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucol_getTailoredSet();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucol_getUCAVersion();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucol_getVariableTop();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucol_getVersion();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucol_greater();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucol_greaterOrEqual();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucol_keyHashCode();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucol_mergeSortkeys();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucol_next();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucol_nextSortKeyPart();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucol_open();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucol_openAvailableLocales();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucol_openBinary();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucol_openElements();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucol_openRules();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucol_previous();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucol_primaryOrder();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucol_reset();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucol_safeClone();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucol_secondaryOrder();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucol_setAttribute();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucol_setMaxVariable();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucol_setOffset();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucol_setReorderCodes();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucol_setStrength();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucol_setText();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucol_strcoll();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucol_strcollIter();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucol_strcollUTF8();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucol_tertiaryOrder();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucpmap_get();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucpmap_getRange();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucptrie_close();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucptrie_get();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucptrie_getRange();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucptrie_getType();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucptrie_getValueWidth();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucptrie_internalSmallIndex();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucptrie_internalSmallU8Index();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucptrie_internalU8PrevIndex();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucptrie_openFromBinary();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucptrie_toBinary();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucsdet_close();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucsdet_detect();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucsdet_detectAll();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucsdet_enableInputFilter();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucsdet_getAllDetectableCharsets();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucsdet_getConfidence();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucsdet_getLanguage();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucsdet_getName();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucsdet_getUChars();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucsdet_isInputFilterEnabled();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucsdet_open();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucsdet_setDeclaredEncoding();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucsdet_setText();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucurr_countCurrencies();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucurr_forLocale();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucurr_forLocaleAndDate();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucurr_getDefaultFractionDigits();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucurr_getDefaultFractionDigitsForUsage();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucurr_getKeywordValuesForLocale();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucurr_getName();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucurr_getNumericCode();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucurr_getPluralName();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucurr_getRoundingIncrement();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucurr_getRoundingIncrementForUsage();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucurr_isAvailable();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucurr_openISOCurrencies();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ucurr_register();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ucurr_unregister();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn udat_adoptNumberFormat();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn udat_adoptNumberFormatForFields();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn udat_applyPattern();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn udat_clone();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn udat_close();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn udat_countAvailable();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn udat_countSymbols();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn udat_format();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn udat_formatCalendar();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn udat_formatCalendarForFields();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn udat_formatForFields();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn udat_get2DigitYearStart();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn udat_getAvailable();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn udat_getBooleanAttribute();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn udat_getCalendar();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn udat_getContext();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn udat_getLocaleByType();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn udat_getNumberFormat();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn udat_getNumberFormatForField();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn udat_getSymbols();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn udat_isLenient();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn udat_open();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn udat_parse();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn udat_parseCalendar();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn udat_set2DigitYearStart();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn udat_setBooleanAttribute();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn udat_setCalendar();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn udat_setContext();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn udat_setLenient();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn udat_setNumberFormat();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn udat_setSymbols();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn udat_toCalendarDateField();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn udat_toPattern();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn udatpg_addPattern();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn udatpg_clone();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn udatpg_close();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn udatpg_getAppendItemFormat();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn udatpg_getAppendItemName();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn udatpg_getBaseSkeleton();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn udatpg_getBestPattern();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn udatpg_getBestPatternWithOptions();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn udatpg_getDateTimeFormat();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn udatpg_getDecimal();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn udatpg_getFieldDisplayName();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn udatpg_getPatternForSkeleton();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn udatpg_getSkeleton();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn udatpg_open();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn udatpg_openBaseSkeletons();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn udatpg_openEmpty();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn udatpg_openSkeletons();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn udatpg_replaceFieldTypes();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn udatpg_replaceFieldTypesWithOptions();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn udatpg_setAppendItemFormat();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn udatpg_setAppendItemName();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn udatpg_setDateTimeFormat();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn udatpg_setDecimal();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn udtitvfmt_close();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn udtitvfmt_closeResult();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn udtitvfmt_format();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn udtitvfmt_open();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn udtitvfmt_openResult();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn udtitvfmt_resultAsValue();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uenum_close();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uenum_count();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn uenum_next();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uenum_openCharStringsEnumeration();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uenum_openUCharStringsEnumeration();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uenum_reset();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uenum_unext();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ufieldpositer_close();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ufieldpositer_next();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ufieldpositer_open();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ufmt_close();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ufmt_getArrayItemByIndex();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ufmt_getArrayLength();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ufmt_getDate();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ufmt_getDecNumChars();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ufmt_getDouble();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ufmt_getInt64();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ufmt_getLong();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ufmt_getObject();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ufmt_getType();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ufmt_getUChars();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ufmt_isNumeric();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ufmt_open();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ufmtval_getString();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ufmtval_nextPosition();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ugender_getInstance();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ugender_getListGender();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uidna_close();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uidna_labelToASCII();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn uidna_labelToASCII_UTF8();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uidna_labelToUnicode();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn uidna_labelToUnicodeUTF8();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uidna_nameToASCII();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn uidna_nameToASCII_UTF8();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uidna_nameToUnicode();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn uidna_nameToUnicodeUTF8();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uidna_openUTS46();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uiter_current32();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uiter_getState();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uiter_next32();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uiter_previous32();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uiter_setState();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uiter_setString();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn uiter_setUTF16BE();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn uiter_setUTF8();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uldn_close();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uldn_getContext();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uldn_getDialectHandling();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn uldn_getLocale();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn uldn_keyDisplayName();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn uldn_keyValueDisplayName();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn uldn_languageDisplayName();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn uldn_localeDisplayName();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn uldn_open();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn uldn_openForContext();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn uldn_regionDisplayName();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uldn_scriptCodeDisplayName();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn uldn_scriptDisplayName();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn uldn_variantDisplayName();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ulistfmt_close();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ulistfmt_closeResult();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ulistfmt_format();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ulistfmt_formatStringsToResult();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ulistfmt_open();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ulistfmt_openForType();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ulistfmt_openResult();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ulistfmt_resultAsValue();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_acceptLanguage();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_acceptLanguageFromHTTP();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_addLikelySubtags();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_canonicalize();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uloc_countAvailable();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_forLanguageTag();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_getAvailable();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_getBaseName();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_getCharacterOrientation();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_getCountry();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_getDefault();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_getDisplayCountry();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_getDisplayKeyword();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_getDisplayKeywordValue();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_getDisplayLanguage();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_getDisplayName();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_getDisplayScript();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_getDisplayVariant();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_getISO3Country();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_getISO3Language();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uloc_getISOCountries();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uloc_getISOLanguages();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_getKeywordValue();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_getLCID();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_getLanguage();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_getLineOrientation();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_getLocaleForLCID();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_getName();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_getParent();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_getScript();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_getVariant();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_isRightToLeft();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_minimizeSubtags();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uloc_openAvailableByType();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_openKeywords();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_setDefault();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_setKeywordValue();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_toLanguageTag();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_toLegacyKey();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_toLegacyType();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_toUnicodeLocaleKey();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn uloc_toUnicodeLocaleType();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ulocdata_close();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ulocdata_getCLDRVersion();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ulocdata_getDelimiter();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ulocdata_getExemplarSet();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ulocdata_getLocaleDisplayPattern();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ulocdata_getLocaleSeparator();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ulocdata_getMeasurementSystem();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ulocdata_getNoSubstitute();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ulocdata_getPaperSize();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ulocdata_open();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ulocdata_setNoSubstitute();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn umsg_applyPattern();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn umsg_autoQuoteApostrophe();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn umsg_clone();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn umsg_close();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn umsg_format();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn umsg_getLocale();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn umsg_open();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn umsg_parse();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn umsg_setLocale();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn umsg_toPattern();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn umsg_vformat();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn umsg_vparse();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn umutablecptrie_buildImmutable();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn umutablecptrie_clone();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn umutablecptrie_close();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn umutablecptrie_fromUCPMap();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn umutablecptrie_fromUCPTrie();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn umutablecptrie_get();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn umutablecptrie_getRange();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn umutablecptrie_open();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn umutablecptrie_set();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn umutablecptrie_setRange();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn unorm2_append();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn unorm2_close();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn unorm2_composePair();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn unorm2_getCombiningClass();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn unorm2_getDecomposition();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn unorm2_getInstance();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn unorm2_getNFCInstance();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn unorm2_getNFDInstance();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn unorm2_getNFKCCasefoldInstance();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn unorm2_getNFKCInstance();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn unorm2_getNFKDInstance();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn unorm2_getRawDecomposition();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn unorm2_hasBoundaryAfter();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn unorm2_hasBoundaryBefore();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn unorm2_isInert();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn unorm2_isNormalized();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn unorm2_normalize();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn unorm2_normalizeSecondAndAppend();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn unorm2_openFiltered();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn unorm2_quickCheck();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn unorm2_spanQuickCheckYes();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn unorm_compare();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn unum_applyPattern();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn unum_clone();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn unum_close();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn unum_countAvailable();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn unum_format();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn unum_formatDecimal();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn unum_formatDouble();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn unum_formatDoubleCurrency();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn unum_formatDoubleForFields();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn unum_formatInt64();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn unum_formatUFormattable();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn unum_getAttribute();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn unum_getAvailable();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn unum_getContext();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn unum_getDoubleAttribute();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn unum_getLocaleByType();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn unum_getSymbol();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn unum_getTextAttribute();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn unum_open();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn unum_parse();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn unum_parseDecimal();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn unum_parseDouble();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn unum_parseDoubleCurrency();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn unum_parseInt64();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn unum_parseToUFormattable();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn unum_setAttribute();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn unum_setContext();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn unum_setDoubleAttribute();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn unum_setSymbol();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn unum_setTextAttribute();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn unum_toPattern();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn unumf_close();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn unumf_closeResult();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn unumf_formatDecimal();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn unumf_formatDouble();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn unumf_formatInt();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn unumf_openForSkeletonAndLocale();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn unumf_openForSkeletonAndLocaleWithError();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn unumf_openResult();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn unumf_resultAsValue();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn unumf_resultGetAllFieldPositions();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn unumf_resultNextFieldPosition();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn unumf_resultToString();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn unumsys_close();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn unumsys_getDescription();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn unumsys_getName();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn unumsys_getRadix();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn unumsys_isAlgorithmic();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn unumsys_open();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn unumsys_openAvailableNames();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn unumsys_openByName();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uplrules_close();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uplrules_getKeywords();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn uplrules_open();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn uplrules_openForType();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uplrules_select();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uplrules_selectFormatted();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uregex_appendReplacement();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uregex_appendReplacementUText();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uregex_appendTail();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uregex_appendTailUText();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uregex_clone();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uregex_close();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uregex_end();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uregex_end64();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uregex_find();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uregex_find64();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uregex_findNext();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uregex_flags();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uregex_getFindProgressCallback();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uregex_getMatchCallback();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uregex_getStackLimit();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uregex_getText();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uregex_getTimeLimit();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uregex_getUText();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uregex_group();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uregex_groupCount();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn uregex_groupNumberFromCName();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uregex_groupNumberFromName();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uregex_groupUText();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uregex_hasAnchoringBounds();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uregex_hasTransparentBounds();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uregex_hitEnd();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uregex_lookingAt();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uregex_lookingAt64();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uregex_matches();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uregex_matches64();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uregex_open();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn uregex_openC();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uregex_openUText();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uregex_pattern();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uregex_patternUText();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uregex_refreshUText();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uregex_regionEnd();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uregex_regionEnd64();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uregex_regionStart();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uregex_regionStart64();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uregex_replaceAll();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uregex_replaceAllUText();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uregex_replaceFirst();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uregex_replaceFirstUText();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uregex_requireEnd();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uregex_reset();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uregex_reset64();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uregex_setFindProgressCallback();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uregex_setMatchCallback();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uregex_setRegion();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uregex_setRegion64();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uregex_setRegionAndStart();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uregex_setStackLimit();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uregex_setText();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uregex_setTimeLimit();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uregex_setUText();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uregex_split();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uregex_splitUText();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uregex_start();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uregex_start64();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uregex_useAnchoringBounds();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uregex_useTransparentBounds();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uregion_areEqual();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uregion_contains();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uregion_getAvailable();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uregion_getContainedRegions();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uregion_getContainedRegionsOfType();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uregion_getContainingRegion();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uregion_getContainingRegionOfType();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uregion_getNumericCode();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uregion_getPreferredValues();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn uregion_getRegionCode();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn uregion_getRegionFromCode();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uregion_getRegionFromNumericCode();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uregion_getType();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ureldatefmt_close();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ureldatefmt_closeResult();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ureldatefmt_combineDateAndTime();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ureldatefmt_format();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ureldatefmt_formatNumeric();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ureldatefmt_formatNumericToResult();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ureldatefmt_formatToResult();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ureldatefmt_open();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ureldatefmt_openResult();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ureldatefmt_resultAsValue();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ures_close();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ures_getBinary();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ures_getByIndex();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ures_getByKey();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ures_getInt();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ures_getIntVector();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ures_getKey();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ures_getLocaleByType();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ures_getNextResource();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ures_getNextString();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ures_getSize();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ures_getString();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ures_getStringByIndex();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ures_getStringByKey();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ures_getType();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ures_getUInt();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ures_getUTF8String();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ures_getUTF8StringByIndex();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ures_getUTF8StringByKey();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ures_getVersion();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ures_hasNext();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ures_open();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ures_openAvailableLocales();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ures_openDirect();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ures_openU();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn ures_resetIterator();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uscript_breaksBetweenLetters();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn uscript_getCode();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn uscript_getName();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uscript_getSampleString();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uscript_getScript();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uscript_getScriptExtensions();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn uscript_getShortName();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uscript_getUsage();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uscript_hasScript();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uscript_isCased();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uscript_isRightToLeft();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn usearch_close();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn usearch_first();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn usearch_following();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn usearch_getAttribute();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn usearch_getBreakIterator();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn usearch_getCollator();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn usearch_getMatchedLength();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn usearch_getMatchedStart();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn usearch_getMatchedText();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn usearch_getOffset();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn usearch_getPattern();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn usearch_getText();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn usearch_last();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn usearch_next();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn usearch_open();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn usearch_openFromCollator();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn usearch_preceding();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn usearch_previous();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn usearch_reset();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn usearch_setAttribute();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn usearch_setBreakIterator();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn usearch_setCollator();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn usearch_setOffset();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn usearch_setPattern();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn usearch_setText();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uset_add();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uset_addAll();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uset_addAllCodePoints();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uset_addRange();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uset_addString();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uset_applyIntPropertyValue();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uset_applyPattern();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uset_applyPropertyAlias();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uset_charAt();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uset_clear();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uset_clone();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uset_cloneAsThawed();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uset_close();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uset_closeOver();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uset_compact();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uset_complement();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uset_complementAll();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uset_contains();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uset_containsAll();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uset_containsAllCodePoints();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uset_containsNone();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uset_containsRange();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uset_containsSome();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uset_containsString();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uset_equals();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uset_freeze();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uset_getItem();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uset_getItemCount();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uset_getSerializedRange();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uset_getSerializedRangeCount();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uset_getSerializedSet();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uset_indexOf();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uset_isEmpty();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uset_isFrozen();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uset_open();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uset_openEmpty();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uset_openPattern();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uset_openPatternOptions();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uset_remove();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uset_removeAll();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uset_removeAllStrings();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uset_removeRange();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uset_removeString();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uset_resemblesPattern();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uset_retain();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uset_retainAll();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uset_serialize();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uset_serializedContains();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uset_set();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uset_setSerializedToOne();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uset_size();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uset_span();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uset_spanBack();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn uset_spanBackUTF8();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn uset_spanUTF8();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uset_toPattern();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uspoof_areConfusable();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn uspoof_areConfusableUTF8();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uspoof_check();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uspoof_check2();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn uspoof_check2UTF8();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn uspoof_checkUTF8();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uspoof_clone();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uspoof_close();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uspoof_closeCheckResult();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uspoof_getAllowedChars();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn uspoof_getAllowedLocales();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uspoof_getCheckResultChecks();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uspoof_getCheckResultNumerics();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uspoof_getCheckResultRestrictionLevel();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uspoof_getChecks();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uspoof_getInclusionSet();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uspoof_getRecommendedSet();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uspoof_getRestrictionLevel();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uspoof_getSkeleton();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn uspoof_getSkeletonUTF8();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uspoof_open();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uspoof_openCheckResult();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uspoof_openFromSerialized();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn uspoof_openFromSource();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uspoof_serialize();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uspoof_setAllowedChars();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn uspoof_setAllowedLocales();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uspoof_setChecks();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn uspoof_setRestrictionLevel();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn usprep_close();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn usprep_open();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn usprep_openByType();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn usprep_prepare();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn utext_char32At();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn utext_clone();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn utext_close();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn utext_copy();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn utext_current32();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn utext_equals();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn utext_extract();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn utext_freeze();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn utext_getNativeIndex();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn utext_getPreviousNativeIndex();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn utext_hasMetaData();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn utext_isLengthExpensive();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn utext_isWritable();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn utext_moveIndex32();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn utext_nativeLength();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn utext_next32();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn utext_next32From();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn utext_openUChars();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn utext_openUTF8();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn utext_previous32();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn utext_previous32From();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn utext_replace();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn utext_setNativeIndex();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn utext_setup();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn utf8_appendCharSafeBody();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn utf8_back1SafeBody();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn utf8_nextCharSafeBody();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn utf8_prevCharSafeBody();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn utmscale_fromInt64();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn utmscale_getTimeScaleValue();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn utmscale_toInt64();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn utrace_format();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn utrace_functionName();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn utrace_getFunctions();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn utrace_getLevel();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn utrace_setFunctions();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn utrace_setLevel();
    #[doc = "*Required features: `Win32_Globalization`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn utrace_vformat();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn utrans_clone();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn utrans_close();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn utrans_countAvailableIDs();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn utrans_getSourceSet();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn utrans_getUnicodeID();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn utrans_openIDs();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn utrans_openInverse();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn utrans_openU();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn utrans_register();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn utrans_setFilter();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn utrans_toRules();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn utrans_trans();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn utrans_transIncremental();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn utrans_transIncrementalUChars();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn utrans_transUChars();
    #[doc = "*Required features: `Win32_Globalization`*"]
    pub fn utrans_unregisterID();
}
